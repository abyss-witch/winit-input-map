#[derive(ToUsize)]
enum Actions {
    Debug,
    Left,
    Right,
    Click,
}
use winit_input_map::*;
use Actions::*;
use gilrs::{Gilrs, Button, ev::Axis};
use winit::{event::*, keyboard::KeyCode, application::*, window::*, event_loop::*};
fn main() {
    let input = input_map!(
        (Debug, KeyCode::Space, Button::South),
        (Left,  KeyCode::ArrowLeft, KeyCode::KeyA, GamepadInput::Axis(Axis::LeftStickX, Direction::Left)),
        (Right, KeyCode::ArrowRight, KeyCode::KeyD, GamepadInput::Axis(Axis::LeftStickX, Direction::Right)),
        (Click, MouseButton::Left)
    );
    
    let gilrs = Gilrs::new().unwrap();
    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut App { window: None, input, gilrs }).unwrap();
}
struct App<const BINDS: usize> { window: Option<Window>, input: InputMap<BINDS>, gilrs: Gilrs }
impl<const BINDS: usize> ApplicationHandler for App<BINDS> {
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
        let input = &mut self.input;
        input.update_with_gilrs(&mut self.gilrs);
        if input.pressed(Debug) {
            println!("pressed {:?}", input.binds(Debug))
        }
        if input.pressing(Right) || input.pressing(Left) {
            println!("axis: {}", input.axis(Right, Left))
        }
        if input.mouse_move != (0.0, 0.0) {
            println!(
                "mouse moved: {:?} and is now at {:?}",
                input.mouse_move, input.mouse_pos
            )
        }
        if input.released(Click) {
            println!("released {:?}", input.binds(Click))
        }
        if input.mouse_scroll != 0.0 {
            println!("scrolling {}", input.mouse_scroll);
        }
        if let Some(other) = input.other_pressed {
            println!("{other:?}");
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
        //reset input. use after your done with the input
        input.init();
    }
}

