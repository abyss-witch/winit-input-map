use winit_input_map::*;
use gilrs::Gilrs;
use winit::{event::*, application::*, window::*, event_loop::*, keyboard::KeyCode};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Action { Return }

fn main() {
    let input = input_map!((Action::Return, KeyCode::Enter));
    
    let gilrs = Gilrs::new().unwrap();
    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut App { window: None, input, gilrs, text: String::new() }).unwrap();
}

struct App { window: Option<Window>, input: InputMap<Action>, gilrs: Gilrs, text: String }
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        self.input.update_with_window_event(&event);
        if let WindowEvent::CloseRequested = &event { event_loop.exit() }
    }
    fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, event: DeviceEvent) {
        self.input.update_with_device_event(&event);
    }
    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        self.input.update_with_gilrs(&mut self.gilrs);

        if self.input.pressed(Action::Return) {
            println!("{}", self.text);
            self.text = String::new();
        } else if let Some(new) = &self.input.text_typed {
            self.text.push_str(new);
        }

        self.input.init();
    }
}
