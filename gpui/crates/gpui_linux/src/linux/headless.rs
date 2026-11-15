mod window;

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::time::{Duration, Instant};

use calloop::LoopSignal;
use gpui::{AnyWindowHandle, PlatformDisplay, PlatformWindow, WindowParams};

pub(crate) use window::{HeadlessWindow, HeadlessWindows};

/// The state of a `LinuxPlatform` with no display server.
///
/// It reports no displays and opens [`HeadlessWindow`]s, which lay out and handle input but draw
/// nothing.
pub(crate) struct HeadlessConnection {
    /// Cloned into every open window, so its count tells whether windows are open.
    window_lease: Rc<()>,
    /// Event-loop signal shared with every window (frame wakeups).
    signal: LoopSignal,
    /// Live windows, for driving their frame callbacks.
    windows: HeadlessWindows,
}

impl HeadlessConnection {
    pub(crate) fn new(signal: LoopSignal) -> Self {
        Self {
            window_lease: Rc::new(()),
            signal,
            windows: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// The live window registry, for the platform's vsync timer.
    pub(crate) fn windows(&self) -> HeadlessWindows {
        self.windows.clone()
    }

    /// Deliver pending frames to headless windows. Called on every
    /// platform event-loop iteration while headless and from the
    /// vsync timer; throttled per window inside the window state.
    pub(crate) fn drive_frames(&self) {
        drive_headless_windows(&self.windows);
    }

    pub(crate) fn has_windows(&self) -> bool {
        Rc::strong_count(&self.window_lease) > 1
    }

    pub(crate) fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        Vec::new()
    }

    pub(crate) fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        None
    }

    pub(crate) fn open_window(
        &self,
        _handle: AnyWindowHandle,
        params: WindowParams,
    ) -> anyhow::Result<Box<dyn PlatformWindow>> {
        Ok(Box::new(HeadlessWindow::new(
            params,
            self.window_lease.clone(),
            self.signal.clone(),
            &self.windows,
        )))
    }
}

/// Deliver one due frame to each live headless window in `windows`.
pub(crate) fn drive_headless_windows(windows: &HeadlessWindows) {
    let states: Vec<_> = {
        let mut windows = windows.borrow_mut();
        windows.retain(|weak| weak.strong_count() > 0);
        windows.iter().filter_map(Weak::upgrade).collect()
    };
    for state in states {
        // Deliver a pending resize before the frame so the frame
        // already lays out at the new size.
        let resize = state.borrow_mut().take_resize_notification();
        if let Some((mut callback, size)) = resize {
            callback(size, 1.0);
            state.borrow_mut().restore_resize_callback(callback);
        }
        let (due, _schedule_wakeup) = state.borrow_mut().frame_due();
        if !due {
            continue;
        }
        let callback = state.borrow_mut().take_frame_callback();
        if let Some(mut callback) = callback {
            callback(gpui::RequestFrameOptions {
                require_presentation: false,
                force_render: false,
                signal_at: Some(Instant::now()),
                signal_source: gpui::FrameRequestSource::LocalSchedule,
            });
            state.borrow_mut().restore_frame_callback(callback);
        }
    }
}
