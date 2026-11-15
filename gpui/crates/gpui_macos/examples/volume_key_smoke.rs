//! Native smoke test: send AppKit volume events through a real GPUI window.
//! Run with `cargo run -p sotf-gpui-toolkit-gpui-macos --example volume_key_smoke`.
#![expect(
    deprecated,
    reason = "Exercise the same cocoa ABI as the native backend"
)]
use cocoa::{
    appkit::NSEventType,
    base::{id, nil},
    foundation::NSPoint,
};
use gpui::{prelude::*, *};
use objc::{class, msg_send, sel, sel_impl};
use std::{cell::RefCell, rc::Rc, time::Duration};

actions!(volume_key_smoke, [VolumeUp, VolumeDown, ToggleMute]);

struct Probe;
impl Render for Probe {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().child("Checking native volume-key delivery…")
    }
}

fn main() {
    let hits = Rc::new(RefCell::new(Vec::new()));
    let result = hits.clone();
    Application::with_platform(Rc::new(gpui_macos::MacPlatform::new(false))).run(move |cx| {
        cx.bind_keys([
            KeyBinding::new("audiovolumeup", VolumeUp, None),
            KeyBinding::new("audiovolumedown", VolumeDown, None),
            KeyBinding::new("audiovolumemute", ToggleMute, None),
        ]);
        let up = hits.clone();
        cx.on_action(move |_: &VolumeUp, _| up.borrow_mut().push("up"));
        let down = hits.clone();
        cx.on_action(move |_: &VolumeDown, _| down.borrow_mut().push("down"));
        let mute = hits.clone();
        cx.on_action(move |_: &ToggleMute, _| mute.borrow_mut().push("mute"));
        cx.open_window(WindowOptions::default(), |_, cx| cx.new(|_| Probe))
            .expect("open smoke-test window");
        cx.activate(true);
        cx.spawn(async move |cx| {
            // Wait for AppKit activation and the first dispatch-tree paint.
            cx.background_executor()
                .timer(Duration::from_millis(300))
                .await;
            // SAFETY: this foreground task runs on AppKit's main thread. The
            // factory events remain live in their autorelease pool during sendEvent:.
            // Every volume key has a consuming test action, with no audio engine.
            unsafe {
                let app: id = msg_send![class!(NSApplication), sharedApplication];
                let window: id = msg_send![app, keyWindow];
                assert!(!window.is_null(), "smoke-test window must be key");
                for (code, flags) in [(0isize, 0x0a00), (0, 0x0a01), (1, 0x0a00), (7, 0x0a00)] {
                    objc::rc::autoreleasepool(|| {
                        let event: id = msg_send![class!(NSEvent),
                            otherEventWithType: NSEventType::NSSystemDefined
                            location: NSPoint::new(0.0, 0.0)
                            modifierFlags: 0u64 timestamp: 0f64 windowNumber: 0isize
                            context: nil subtype: 8i16 data1: (code << 16) | flags data2: 0isize];
                        let _: () = msg_send![app, sendEvent: event];
                    });
                }
            }
            assert_eq!(*result.borrow(), ["up", "up", "down", "mute"]);
            println!("Native AppKit → GPUI volume-key bindings passed");
            cx.update(|cx| cx.quit());
        })
        .detach();
    });
}
