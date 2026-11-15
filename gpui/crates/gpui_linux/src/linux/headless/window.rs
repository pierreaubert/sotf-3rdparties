//! Windows for the headless platform client.
//!
//! A headless window has no compositor surface and no GPU: layout, text
//! shaping, and entity plumbing run normally, `draw` discards the scene, and
//! the sprite atlas hands out tiles without uploading pixels (the same
//! `HeadlessAtlas` GPUI's `TestWindow` uses). This lets command-line tools
//! drive real `Window`-based code paths without a display server.

use calloop::LoopSignal;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::sync::Arc;
use std::time::{Duration, Instant};

use gpui::{
    Bounds, Capslock, DispatchEventResult, GpuSpecs, HeadlessAtlas, Modifiers, Pixels,
    PlatformAtlas, PlatformDisplay, PlatformInput, PlatformInputHandler, PlatformWindow, Point,
    PromptButton, PromptLevel, RequestFrameOptions, Scene, Size, WindowAppearance,
    WindowBackgroundAppearance, WindowBounds, WindowControlArea, WindowParams, WindowVisibility,
};

pub(crate) struct HeadlessWindowState {
    bounds: Bounds<Pixels>,
    /// Held while this window exists, so the platform can tell that headless windows are open.
    _lease: Rc<()>,
    input_handler: Option<PlatformInputHandler>,
    title: Option<String>,
    is_fullscreen: bool,
    /// Registered redraw callback, driven by the platform loop at a
    /// vsync-like cadence (there is no compositor to pace frames).
    request_frame: Option<Box<dyn FnMut(RequestFrameOptions)>>,
    /// Registered resize callback; `resize` is the one headless event
    /// source for it (programmatic resizes and the dev API).
    resize_callback: Option<Box<dyn FnMut(Size<Pixels>, f32)>>,
    /// Size awaiting delivery to `resize_callback` by the platform
    /// loop. Delivery cannot happen inside `resize` itself: resize is
    /// usually called from within an app update, and the callback's
    /// own window update would be re-entrant and dropped.
    pending_resize: Option<Size<Pixels>>,
    /// Wakes the platform event loop so a pending frame is driven.
    signal: LoopSignal,
    last_frame: Option<Instant>,
    wakeup_scheduled: bool,
    /// Fallback atlas when no renderer is available (stable across
    /// calls, unlike a fresh atlas per `sprite_atlas()`).
    fallback_atlas: Arc<dyn PlatformAtlas>,
}

#[cfg(any(feature = "wayland", feature = "x11"))]
type SharedHeadlessRenderer = Rc<RefCell<gpui_wgpu::WgpuHeadlessRenderer>>;

#[cfg(any(feature = "wayland", feature = "x11"))]
thread_local! {
    /// One WGPU renderer shared by every headless window in the
    /// process. A renderer per window churns Vulkan instances;
    /// after a few dozen windows llvmpipe starts refusing new
    /// adapters, and screenshots fail mid-session. The headless
    /// renderer has no surface and sizes its target per render, so
    /// sharing is safe. The platform is single-threaded, hence the
    /// thread-local slot.
    static SHARED_HEADLESS_RENDERER: RefCell<Option<SharedHeadlessRenderer>> =
        const { RefCell::new(None) };
}

/// The shared headless WGPU renderer, created on first use when a
/// usable adapter exists.
#[cfg(any(feature = "wayland", feature = "x11"))]
fn shared_headless_renderer(
    size: gpui::Size<gpui::DevicePixels>,
) -> Option<SharedHeadlessRenderer> {
    // try_with: window/atlas destructors can run during thread
    // teardown, after this slot is already destroyed.
    SHARED_HEADLESS_RENDERER
        .try_with(|slot| {
            let mut slot = slot.borrow_mut();
            if slot.is_none() {
                *slot = gpui_wgpu::WgpuHeadlessRenderer::try_new(size, false).map(|renderer| {
                    let renderer = Rc::new(RefCell::new(renderer));
                    // Leak one strong reference on purpose:
                    // dropping WGPU resources from a TLS
                    // destructor panics inside wgpu-core (its
                    // own TLS is already gone). The shared
                    // renderer is process-lifetime by design.
                    std::mem::forget(renderer.clone());
                    renderer
                });
            }
            slot.clone()
        })
        .unwrap_or(None)
}

/// Registry of live headless windows, shared between the connection
/// (which drives frames) and the windows (which register on open).
pub(crate) type HeadlessWindows = Rc<RefCell<Vec<Weak<RefCell<HeadlessWindowState>>>>>;

/// Headless frame cadence: ~60 fps, matching a typical compositor.
const FRAME_INTERVAL: Duration = Duration::from_millis(16);

impl HeadlessWindowState {
    /// Whether a frame is pending and due, and if a delayed wakeup
    /// should be scheduled for one that is not due yet.
    pub(crate) fn frame_due(&mut self) -> (bool, bool) {
        if self.request_frame.is_none() {
            return (false, false);
        }
        let now = Instant::now();
        if let Some(last) = self.last_frame
            && now.duration_since(last) < FRAME_INTERVAL
        {
            let schedule = !std::mem::replace(&mut self.wakeup_scheduled, true);
            return (false, schedule);
        }
        self.last_frame = Some(now);
        self.wakeup_scheduled = false;
        (true, false)
    }

    pub(crate) fn take_frame_callback(&mut self) -> Option<Box<dyn FnMut(RequestFrameOptions)>> {
        self.request_frame.take()
    }

    pub(crate) fn restore_frame_callback(&mut self, callback: Box<dyn FnMut(RequestFrameOptions)>) {
        if self.request_frame.is_none() {
            self.request_frame = Some(callback);
        }
    }

    /// Take the resize callback and the size it should be called
    /// with, when a resize is awaiting delivery.
    pub(crate) fn take_resize_notification(
        &mut self,
    ) -> Option<(Box<dyn FnMut(Size<Pixels>, f32)>, Size<Pixels>)> {
        if self.pending_resize.is_none() || self.resize_callback.is_none() {
            return None;
        }
        let size = self.pending_resize.take()?;
        let callback = self.resize_callback.take()?;
        Some((callback, size))
    }

    pub(crate) fn restore_resize_callback(&mut self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        if self.resize_callback.is_none() {
            self.resize_callback = Some(callback);
        }
    }
}

pub(crate) struct HeadlessWindow(Rc<RefCell<HeadlessWindowState>>);

impl raw_window_handle::HasWindowHandle for HeadlessWindow {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        // Headless windows are not backed by a native window.
        Err(raw_window_handle::HandleError::NotSupported)
    }
}

impl raw_window_handle::HasDisplayHandle for HeadlessWindow {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        Err(raw_window_handle::HandleError::NotSupported)
    }
}

impl HeadlessWindow {
    pub(crate) fn new(
        params: WindowParams,
        lease: Rc<()>,
        signal: LoopSignal,
        windows: &HeadlessWindows,
    ) -> Self {
        let window = Self(Rc::new(RefCell::new(HeadlessWindowState {
            bounds: params.bounds,
            _lease: lease,
            input_handler: None,
            title: None,
            is_fullscreen: false,
            request_frame: None,
            resize_callback: None,
            pending_resize: None,
            signal,
            last_frame: None,
            wakeup_scheduled: false,
            fallback_atlas: Arc::new(HeadlessAtlas::default()),
        })));
        windows.borrow_mut().push(Rc::downgrade(&window.0));
        window
    }

    /// The shared state, for the connection's frame driver.
    pub(crate) fn state(&self) -> Rc<RefCell<HeadlessWindowState>> {
        self.0.clone()
    }
}

impl PlatformWindow for HeadlessWindow {
    fn bounds(&self) -> Bounds<Pixels> {
        self.0.borrow().bounds
    }

    fn is_maximized(&self) -> bool {
        false
    }

    fn window_bounds(&self) -> WindowBounds {
        WindowBounds::Windowed(self.bounds())
    }

    fn content_size(&self) -> Size<Pixels> {
        self.bounds().size
    }

    fn resize(&mut self, size: Size<Pixels>) {
        // Record the new size and queue the compositor-style
        // notification for the platform loop (see pending_resize).
        let signal = {
            let mut state = self.0.borrow_mut();
            state.bounds.size = size;
            state.pending_resize = Some(size);
            state.signal.clone()
        };
        signal.wakeup();
    }

    fn scale_factor(&self) -> f32 {
        1.0
    }

    fn appearance(&self) -> WindowAppearance {
        WindowAppearance::Dark
    }

    fn display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        None
    }

    fn mouse_position(&self) -> Point<Pixels> {
        Point::default()
    }

    fn modifiers(&self) -> Modifiers {
        Modifiers::default()
    }

    fn capslock(&self) -> Capslock {
        Capslock::default()
    }

    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {
        self.0.borrow_mut().input_handler = Some(input_handler);
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.0.borrow_mut().input_handler.take()
    }

    fn prompt(
        &self,
        _level: PromptLevel,
        _msg: &str,
        _detail: Option<&str>,
        _answers: &[PromptButton],
    ) -> Option<futures::channel::oneshot::Receiver<usize>> {
        // Fall back to GPUI's rendered prompts.
        None
    }

    fn activate(&self) {}

    fn is_active(&self) -> bool {
        false
    }

    fn visibility(&self) -> WindowVisibility {
        // There is no display server: `draw` discards every scene.
        WindowVisibility::Hidden
    }

    fn is_hovered(&self) -> bool {
        false
    }

    fn background_appearance(&self) -> WindowBackgroundAppearance {
        WindowBackgroundAppearance::Opaque
    }

    fn set_title(&mut self, title: &str) {
        self.0.borrow_mut().title = Some(title.to_owned());
    }

    fn get_title(&self) -> String {
        self.0.borrow().title.clone().unwrap_or_default()
    }

    fn set_background_appearance(&self, _background: WindowBackgroundAppearance) {}

    fn minimize(&self) {}

    fn zoom(&self) {}

    fn toggle_fullscreen(&self) {
        let mut state = self.0.borrow_mut();
        state.is_fullscreen = !state.is_fullscreen;
    }

    fn is_fullscreen(&self) -> bool {
        self.0.borrow().is_fullscreen
    }

    // No compositor drives a frame loop, so the platform loop drives
    // stored frame callbacks itself at a vsync-like cadence (see
    // HeadlessConnection::drive_frames). Status callbacks have no
    // source of events headlessly and are dropped.
    fn on_request_frame(&self, callback: Box<dyn FnMut(RequestFrameOptions)>) {
        let mut state = self.0.borrow_mut();
        state.request_frame = Some(callback);
        state.signal.wakeup();
    }

    fn on_input(&self, _callback: Box<dyn FnMut(PlatformInput) -> DispatchEventResult>) {}

    fn on_active_status_change(&self, _callback: Box<dyn FnMut(bool)>) {}

    fn on_visibility_change(&self, _callback: Box<dyn FnMut(WindowVisibility)>) {}

    fn on_hover_status_change(&self, _callback: Box<dyn FnMut(bool)>) {}

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.0.borrow_mut().resize_callback = Some(callback);
    }

    fn on_moved(&self, _callback: Box<dyn FnMut()>) {}

    fn on_display_changed(&self, _callback: Box<dyn FnMut()>) {}

    fn on_should_close(&self, _callback: Box<dyn FnMut() -> bool>) {}

    fn on_close(&self, _callback: Box<dyn FnOnce()>) {}

    fn on_hit_test_window_control(&self, _callback: Box<dyn FnMut() -> Option<WindowControlArea>>) {
    }

    fn on_appearance_changed(&self, _callback: Box<dyn FnMut()>) {}

    fn draw(&self, _scene: &Scene) {
        // Drawing is discarded. Screenshots re-render on demand in
        // render_to_image, so feeding the renderer every frame would
        // only burn CPU/allocations that performance budgets measure.
    }

    fn render_to_image(&self, scene: &Scene) -> anyhow::Result<image::RgbaImage> {
        #[cfg(any(feature = "wayland", feature = "x11"))]
        {
            let size = self.0.borrow().bounds.size.to_device_pixels(1.0);
            // Adapter probing can fail transiently under load; retry
            // a few times with brief pauses before giving up.
            let mut renderer = shared_headless_renderer(size);
            for attempt in 0..5 {
                if renderer.is_some() {
                    break;
                }
                log::debug!("headless renderer probe attempt {attempt} found no adapter");
                std::thread::sleep(std::time::Duration::from_millis(100));
                renderer = shared_headless_renderer(size);
            }
            if let Some(renderer) = renderer {
                let mut renderer = renderer.borrow_mut();
                return gpui::PlatformHeadlessRenderer::render_scene_to_image(
                    &mut *renderer,
                    scene,
                    size,
                );
            }
        }
        let _ = scene;
        anyhow::bail!("render_to_image not available: no headless WGPU renderer")
    }

    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        #[cfg(any(feature = "wayland", feature = "x11"))]
        {
            let size = self.0.borrow().bounds.size.to_device_pixels(1.0);
            if let Some(renderer) = shared_headless_renderer(size) {
                let renderer = renderer.borrow();
                return gpui::PlatformHeadlessRenderer::sprite_atlas(&*renderer);
            }
        }
        self.0.borrow().fallback_atlas.clone()
    }

    fn is_subpixel_rendering_supported(&self) -> bool {
        false
    }

    fn update_ime_position(&self, _bounds: Bounds<Pixels>) {}

    fn gpu_specs(&self) -> Option<GpuSpecs> {
        None
    }
}
