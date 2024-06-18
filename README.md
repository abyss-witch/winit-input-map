Input map for winit.

Supports Keys, Mouse Buttons, Cursor, Mouse Scroll
```rust
enum Actions {
    Debug,
    Left,
    Right,
    Click,
}
impl Into<usize> for Actions {
    fn into(self) -> usize {
        self as usize
    }
}
use winit_input_map::*;
use Actions::*;

let mut input = InputMap::new([
    (vec![Input::keycode(KeyCode::Space)], Debug),
    (vec![
        Input::keycode(KeyCode::ArrowLeft),
        Input::keycode(KeyCode::KeyA),
    ], Left),
    (vec![
        Input::keycode(KeyCode::ArrowRight),
        Input::keycode(KeyCode::KeyD),
    ], Right),
    (vec![Input::Mouse(MouseButton::Left)], Click),
]);

use winit::{event::*, keyboard::KeyCode, window::Window};
let event_loop = winit::event_loop::EventLoop::new().unwrap();
event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
let _window = Window::new(&event_loop).unwrap();
event_loop.run(|event, target| {
    input.update(&event);
    match &event {
        Event::WindowEvent { 
            event: WindowEvent::CloseRequested, ..
        } => target.exit(),
        Event::AboutToWait => {
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
            std::thread::sleep(std::time::Duration::from_millis(100));
            //reset input. use after your done with the input
            input.init();
        }
        _ => (),
    }
}).unwrap();
```
Above example doesnt work on some platforms due to a lack of rendered graphics.
