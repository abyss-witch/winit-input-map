fn main() {
    #[derive(ToUsize)]
    enum Actions {
        Debug,
        Left,
        Right,
        Click,
    }
    use winit_input_map::*;
    use Actions::*;

    let mut input = input_map!(
        (Debug, KeyCode::Space),
        (Left,  KeyCode::ArrowLeft, KeyCode::KeyA),
        (Right, KeyCode::ArrowRight, KeyCode::KeyD),
        (Click, MouseButton::Left)
    );

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
}
