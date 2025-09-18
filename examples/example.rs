#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Actions {
    Debug,
    Left,
    Right,
    Click,
    MouseR, MouseL, MouseU, MouseD,
    ScrollU, ScrollD
}
use winit_input_map::*;
use Actions::*;
use gilrs::Gilrs;
use winit::{event::*, application::*, window::*, event_loop::*};
fn main() {
    let mut input = { use base_input_codes::*; input_map!(
        (Debug, Space, South),
        (Left,  ArrowLeft,  KeyA, LeftStickLeft ),
        (Right, ArrowRight, KeyD, LeftStickRight),
        (Click, MouseButton::Left),
        (MouseR, MouseMoveRight, RightStickRight),
        (MouseL, MouseMoveLeft,  RightStickLeft ),
        (MouseU, MouseMoveUp,    RightStickUp   ),
        (MouseD, MouseMoveDown,  RightStickDown ),
        (ScrollU, Equal, MouseScrollUp  ),
        (ScrollD, Minus, MouseScrollDown)
    ) };
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
        let window = event_loop.create_window(window_settings);
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
        &mut self, _: &ActiveEventLoop, id: DeviceId, event: DeviceEvent
    ) {
        self.input.update_with_device_event(id, &event);
    }
    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        self.input.update_with_gilrs(&mut self.gilrs);

        let input = &mut self.input;
        let scroll = input.axis(ScrollU, ScrollD);
    
        if input.pressed(Debug) {
            println!("pressed {:?}", input.get_binds().into_iter().find(|&(a, _)| a == Debug).map(|(_, v)| v))
        }
        if input.pressing(Right) || input.pressing(Left) {
            println!("axis: {}", input.axis(Right, Left))
        }

        let mouse_move = input.dir(MouseL, MouseR, MouseU, MouseD);
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
        if let Some(other) = input.recently_pressed {
            println!("{other:?}");
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
        // reset input. use after your done with the input
        input.init();
    }
}

