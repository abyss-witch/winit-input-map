use winit_input_map::*;
#[derive(ToUsize)]
enum Action {
    Return
}
fn main() {
    use Action::*;
    use winit::{window::Window, keyboard::KeyCode, event_loop::EventLoop, event::*};
    let mut input = input_map!(
        (Return, KeyCode::Enter)
    );

    let ev = EventLoop::new().unwrap(); // notice that we didnt set control flow poll
    let _window = Window::new(&ev).unwrap();
    
    let mut text = String::new();
    ev.run(|event, target|{
        input.update(&event);
        match &event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => { target.exit() },
            Event::AboutToWait => {
                if input.pressed(Return) {
                    println!("{text}");
                    text = String::new();
                } else if let Some(new) = &input.text_typed {
                    text.push_str(new);
                }
                input.init();
            }
            _ => ()
        }
    }).unwrap()
}
