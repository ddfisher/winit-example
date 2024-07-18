use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::platform::web::WindowAttributesExtWebSys;
use winit::platform::web::WindowExtWebSys;
use winit::window::{Window, WindowId};

struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = event_loop
                .create_window(Window::default_attributes().with_append(true))
                .unwrap();

            window.set_prevent_default(false);
            add_prevent_default_listeners(window.canvas().unwrap());

            self.window = Some(window)
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn add_prevent_default_listeners(canvas: HtmlCanvasElement) {
    let events_types_to_fully_prevent_default = ["touchstart", "wheel", "contextmenu"];
    let pointer_events_to_focus_and_prevent_default = ["pointerdown"];
    let pointer_events_to_focus_and_prevent_default_on_chord = ["pointermove"];
    let key_events_to_partially_prevent_default = ["keyup", "keydown"];

    for event_type in events_types_to_fully_prevent_default.into_iter() {
        let prevent_default_listener =
            Closure::<dyn FnMut(_)>::new(move |event: web_sys::Event| {
                event.prevent_default();
            });

        let _ = canvas.add_event_listener_with_callback(
            event_type,
            prevent_default_listener.as_ref().unchecked_ref(),
        );
        prevent_default_listener.into_js_value();
    }

    for event_type in pointer_events_to_focus_and_prevent_default.into_iter() {
        let stored_canvas = canvas.clone();
        let prevent_default_listener =
            Closure::<dyn FnMut(_)>::new(move |event: web_sys::PointerEvent| {
                event.prevent_default();
                let _ = stored_canvas.focus();
            });

        let _ = canvas.add_event_listener_with_callback(
            event_type,
            prevent_default_listener.as_ref().unchecked_ref(),
        );
        prevent_default_listener.forget();
    }

    for event_type in pointer_events_to_focus_and_prevent_default_on_chord.into_iter() {
        let stored_canvas = canvas.clone();
        let prevent_default_listener =
            Closure::<dyn FnMut(_)>::new(move |event: web_sys::PointerEvent| {
                if event.button() != -1 {
                    // chorded button interaction
                    event.prevent_default();
                    let _ = stored_canvas.focus();
                }
            });

        let _ = canvas.add_event_listener_with_callback(
            event_type,
            prevent_default_listener.as_ref().unchecked_ref(),
        );
        prevent_default_listener.forget();
    }

    for event_type in key_events_to_partially_prevent_default.into_iter() {
        let prevent_default_listener =
            Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
                let only_ctrl_key =
                    event.ctrl_key() && !event.meta_key() && !event.shift_key() && !event.alt_key();
                let allow_default = only_ctrl_key && matches!(event.key().as_ref(), "p");
                if !allow_default {
                    event.prevent_default();
                }
            });

        let _ = canvas.add_event_listener_with_callback(
            event_type,
            prevent_default_listener.as_ref().unchecked_ref(),
        );
        prevent_default_listener.forget();
    }
}

fn main() {
    let mut app = App { window: None };
    let event_loop = EventLoop::new().unwrap();
    let _ = event_loop.run_app(&mut app);
}
