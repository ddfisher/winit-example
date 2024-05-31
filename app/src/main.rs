use gloo::console;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy};
use winit::platform::web::WindowExtWebSys;
use winit::window::{Window, WindowId};

#[derive(Debug)]
enum CustomEvent {
    CustomEvent,
}

struct App {
    window: Option<Window>,
    proxy: EventLoopProxy<CustomEvent>,
}

impl ApplicationHandler<CustomEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .unwrap();

        gloo::utils::body()
            .append_child(&window.canvas().unwrap())
            .expect("Failed to append canvas element to <body>");

        self.window = Some(window)
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::MouseInput { state, .. } => {
                if state.is_pressed() {
                    let proxy_copy = self.proxy.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        console::log!("Sending event.");
                        proxy_copy
                            .send_event(CustomEvent::CustomEvent)
                            .expect("Send event failed");
                        console::log!("Returned from event send.");
                    })
                }
            }
            _ => (),
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: CustomEvent) {
        match event {
            CustomEvent::CustomEvent => {
                console::log!("Event handled.");
            }
        }
    }
}

fn main() {
    let event_loop = EventLoop::<CustomEvent>::with_user_event().build().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App {
        window: None,
        proxy: event_loop.create_proxy(),
    };
    let _ = event_loop.run_app(&mut app);
}
