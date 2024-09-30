//! define actions and their key binds and then see if its pressing, pressed or released.
//! You can get the `mouse_pos`, how much its moved (`mouse_move`), the scroll wheels movement (`mouse_scroll`),
//! last other input (`other_pressed` used for rebinding things) and the `text_typed` (useful for typing). you
//! can use anything that implements the `Into<usize>` trait as an action, but it's recommended 
//! to use an action enum which derives `ToUsize`.
//! ```
//! #[derive(ToUsize)]
//! enum Actions{
//!     Debug,
//! }
//! use winit::{event::*, keyboard::KeyCode, event_loop::*, window::Window};
//! use winit_input_map::*;
//! use Actions::*;
//!
//! let mut input = input_map!(
//!     (Debug, KeyCode::Space),
//! );
//! 
//! let event_loop = EventLoop::new().unwrap();
//! event_loop.set_control_flow(ControlFlow::Poll);
//! let _window = Window::new(&event_loop).unwrap();
//!
//! event_loop.run(|event, target|{
//!     input.update(&event);
//!     match &event {
//!         Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => { target.exit() },
//!         Event::AboutToWait => {
//!             if input.pressed(Debug) { 
//!                 println!("pressed {:?}", input.binds(Debug)) 
//!             }
//!             std::thread::sleep(std::time::Duration::from_millis(100));
//!             //put at end of loop because were done with inputs this frame.
//!             input.init();
//!         }
//!         _ => ()
//!     }
//! }).unwrap();
//! ```
mod input;
pub use derive_to_usize::ToUsize;
pub use crate::input::*;
/// creates new input map with binds and actions.
/// ```
/// #[derive(ToUsize)]
/// enum Action {
///     Jump,
///     Left,
///     Right,
///     Interact
/// }
/// use Action::*;
/// use winit_input_map::*;
/// let mut input = input_map!(
///     (Jump, KeyCode::Space),
///     (Left, KeyCode::KeyA, KeyCode::LeftArrow),
///     (Right, KeyCode::KeyD, KeyCode::RightArrow),
///     (Interact, MouseButton::Left)
/// );
/// ```
#[macro_export]
macro_rules! input_map {
    () => { InputMap::<0>::empty() };
    ( $( ( $x:expr, $( $k:expr ),* ) ),* ) => {
        InputMap::new([ $(
            ($x, vec![ $( $k.into(), )* ]),
        )*])
    };
}
#[cfg(test)]
mod test;
