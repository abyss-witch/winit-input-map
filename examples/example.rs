#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Actions {
    Debug,
    Left,
    Right,
    Click,
    MouseXP, MouseXN, MouseYP, MouseYN,
    MouseScrollP, MouseScrollN
}
use winit_input_map::*;
use Actions::*;
use gilrs::{Gilrs};
use winit::{event::*, keyboard::KeyCode, application::*, window::*, event_loop::*};
fn main() {
    let mut input = input_map!(
        (Debug, KeyCode::Space, GamepadButton::South),
        (Left,
            KeyCode::ArrowLeft,  KeyCode::KeyA,
            GamepadInput::Axis(Axis::LeftStickX, AxisSign::Neg)
        ),
        (Right,
            KeyCode::ArrowRight, KeyCode::KeyD,
            GamepadInput::Axis(Axis::LeftStickX, AxisSign::Pos)
        ),
        (Click, MouseButton::Left),
        (MouseXP, InputCode::MOUSE_MOVE_X_POS),
        (MouseXN, InputCode::MOUSE_MOVE_X_NEG),
        (MouseYP, InputCode::MOUSE_MOVE_Y_POS),
        (MouseYN, InputCode::MOUSE_MOVE_Y_NEG),
        (MouseScrollP, InputCode::MOUSE_SCROLL_POS),
        (MouseScrollN, InputCode::MOUSE_SCROLL_NEG)
    );
    input.mouse_scale = 1.0;
    input.scroll_scale = 1.0;
    
    let gilrs = Gilrs::new().unwrap();
    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut App { window: None, input, gilrs }).unwrap();
}
struct App { window: Option<Window>, input: InputMap<Actions>, gilrs: Gilrs }
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_settings = Window::default_attributes();
        let window = event_loop.create_window();
        self.window = Some(window.unwrap());
    }
    fn window_event(
        &mut self, event_loop: &ActiveEventLoop,
        _: WindowId, event: WindowEvent
    ) {
        self.input.update_with_window_event(&event);
        if let WindowEvent::CloseRequested = &event { event_loop.exit() }
    }
    fn device_event(
        &mut self, _: &ActiveEventLoop, _: DeviceId, event: DeviceEvent
    ) {
        self.input.update_with_device_event(&event);
    }
    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        let input = &mut self.input;
        let scroll = input.axis(MouseScrollP, MouseScrollN);
                input.update_with_gilrs(&mut self.gilrs);
        if input.pressed(Debug) {
            println!("pressed {:?}", input.binds.iter().filter_map(|(a, s)| {
                if s.contains(&Debug) { Some(*a) } else { None }
            }).collect::<Vec<InputCode>>())
        }
        if input.pressing(Right) || input.pressing(Left) {
            println!("axis: {}", input.axis(Right, Left))
        }

        let mouse_move = input.dir(MouseXP, MouseXN, MouseYP, MouseYN);
        if mouse_move != (0.0, 0.0) {
            println!(
                "mouse moved: {:?} and is now at {:?}",
                mouse_move, input.mouse_pos
            )
        }
        if input.released(Click) {
            println!("released")
        }
        if scroll != 0.0 {
            println!("scrolling {}", scroll);
        }
        if let Some(other) = input.other_pressed {
            println!("{other:?}");
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
        //reset input. use after your done with the input
        input.init();
    }
}

