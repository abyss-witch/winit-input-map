fn main(){
    enum Actions{
        Debug,
        Left,
        Right,
        Click
    }
    impl Into<usize> for Actions{
        fn into(self) -> usize {
            self as usize
        }
    }
    use input::{Input, InputCode};
    use Actions::*;
    
    let mut input = Input::new([
        (vec![InputCode::keycode(KeyCode::Space)], Debug),
        (vec![InputCode::keycode(KeyCode::ArrowLeft), InputCode::keycode(KeyCode::KeyA)], Left),
        (vec![InputCode::keycode(KeyCode::ArrowRight), InputCode::keycode(KeyCode::KeyD)], Right),
        (vec![InputCode::Mouse(MouseButton::Left)], Click)
    ]);
    
    use winit::{event::*, keyboard::KeyCode};
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let _window = winit::window::Window::new(&event_loop).unwrap();
    
    event_loop.run(|event, target|{
        input.update(&event);
        match &event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => { target.exit() },
            Event::AboutToWait => {
                if input.pressed(Debug) { println!("pressed {:?}", input.binds(Debug)) }
                if input.pressing(Right) || input.pressing(Left) { println!("axis: {}", input.axis(Right, Left)) }
                if input.mouse_move != (0.0, 0.0) { println!("mouse moved: {:?} and is now at {:?}", input.mouse_move, input.mouse_pos) }
                if input.released(Click) { println!("released {:?}", input.binds(Click)) }
                
                std::thread::sleep(std::time::Duration::from_millis(100));
                //since init is required to be called once before update, we put it at the end before it loops.
                input.init();
            }
            _ => ()
        }
    }).unwrap();
}
