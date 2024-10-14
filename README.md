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
let mut input_map!(
    (MoveLeft,
        KeyCode::KeyA, KeyCode::ArrowLeft, GamepadButton::DPadLeft,
        InputCode::gamepad_axis_neg(GamepadAxis::LeftStickX)
    ),
    (MoveRight,
        KeyCode::KeyD, KeyCode::ArrowRight, GamepadButton::DPadRight,
        InputCode::gamepad_axis_pos(GamepadAxis::LeftStickX)
    ),
    (CameraLeft,
        InputCode::MOUSE_MOVE_X_NEG,
        InputCode::gamepad_axis_neg(GamepadAxis::RightStickX)
    ),
    (CameraRight,
        InputCode::MOUSE_MOVE_X_POS,
        InputCode::gamepad_axis_pos(GamepadAxis::RightStickX)
    ),
    (CameraUp,
        InputCode::MOUSE_MOVE_Y_POS,
        InputCode::gamepad_axis_pos(GamepadAxis::RightStickY)
    ),
    (CameraDown,
        InputCode::MOUSE_MOVE_Y_NEG,
        InputCode::gamepad_axis_neg(GamepadAxis::RightStickY)
    )
);
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
```rust
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Actions {
    Debug, Click,
    Left, Right 
}
use winit_input_map::*;
use Actions::*;

let mut input = input_map!(
    (Debug, KeyCode::Space),
    (Left,  KeyCode::ArrowLeft, KeyCode::KeyA),
    (Right, KeyCode::ArrowRight, KeyCode::KeyD),
    (Click, MouseButton::Left)
);

// winit event handler
input.update(&event);

// use the input map!

// end of using event handler
input.init();
```

A more complete example:
```rust
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
    (MouseXP,      InputCode::MOUSE_MOVE_X_POS),
    (MouseXN,      InputCode::MOUSE_MOVE_X_NEG),
    (MouseYP,      InputCode::MOUSE_MOVE_Y_POS),
    (MouseYN,      InputCode::MOUSE_MOVE_Y_NEG),
    (MouseScrollP, InputCode::MOUSE_SCROLL_POS),
    (MouseScrollN, InputCode::MOUSE_SCROLL_NEG)
);
input.mouse_scale = 1.0;
input.scroll_scale = 1.0;

let gilrs = Gilrs::new().unwrap();
let event_loop = EventLoop::new().unwrap();
event_loop.run_app(&mut App { window: None, input, gilrs }).unwrap();

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
```
