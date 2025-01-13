An input map for winit!

Winit input map aims to be fully featured, simple to use and easy to read.

It works by assigning each input code to an action, these action will have a variable press weight and functions
like axis and dir can turn them into .
e.g
```rust
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Actions {
    MoveLeft, MoveRight,
    CameraUp, CameraDown,
    CameraLeft, CameraRight
}
// set up
// ...
let mut input = { use base_input_codes::*; input_map!(
    (MoveLeft,  KeyA, ArrowLeft,  DPadLeft,  LeftStickLeft ),
    (MoveRight, KeyD, ArrowRight, DPadRight, LeftStickRight),
    (CameraLeft,  MouseMoveLeft,  RightStickLeft ),
    (CameraRight, MouseMoveRight, RightStickRight),
    (CameraUp,    MouseMoveUp,    RightStickUp   ),
    (CameraDown,  MouseMoveDown,  RightStickDown )
) };
// ...

// gameplay loop

// clamp is there incase it gets bound to scroll or mouse move as they can go above 1
player.pos += input.axis(MoveRight, MoveLeft).clamp(-1.0, 1.0);
camera.pos += input.dir(CameraRight, CameraLeft, CameraUp, CameraDown);
```

## Features:
    - Gamepad Support (through Gilrs)
    - Variable Pressure Support
    - Easy axis and vector handling
    - Easy rebinding
    - Mouse movement and scrolling
A more complete example:
```rust

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
            println!("pressed {:?}", input.binds.iter().filter_map(|(a, (_, s))| {
                if s.contains(&Debug) { Some(*a) } else { None }
            }).collect::<Vec<InputCode>>())
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
        //reset input. use after your done with the input
        input.init();
    }
}
```
