//! define actions and their key binds and then see if its pressing, pressed or released.
//! You can get the `mouse_pos`, how much its moved (`mouse_move`), the scroll wheels movement (`mouse_scroll`),
//! last other input (`other_pressed` used for rebinding things) and the `text_typed` (useful for typing). you
//! can use anything that implements the `Into<usize>` trait as an action, but it's recommended 
//! to use an action enum which derives `ToUsize`.
//! ```
//! use winit::{window::*, application::*, keyboard::*, event_loop::*, event::*};
//! use gilrs::Gilrs;
//! use winit_input_map::*;
//! #[derive(ToUsize)]
//! enum Actions{ Foo }
//! use Actions::*;
//!
//! let input = input_map!((Foo, KeyCode::Space));
//! let ev = EventLoop::new().unwrap();
//! let gilrs = Gilrs::new().unwrap();
//! ev.run_app(&mut App { window: None, input, gilrs}).unwrap();
//!
//! struct App<const BINDS: usize> { window: Option<Window>, input: InputMap<BINDS>, gilrs: Gilrs }
//! impl<const BINDS: usize> ApplicationHandler for App<BINDS> {
//!     fn resumed(&mut self, event_loop: &ActiveEventLoop) {
//!         self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
//!     }
//!     fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
//!         self.input.update_with_window_event(&event);
//!         if let WindowEvent::CloseRequested = &event { event_loop.exit() }
//!     }
//!     fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, event: DeviceEvent) {
//!         self.input.update_with_device_event(&event);
//!     }
//!     fn about_to_wait(&mut self, _: &ActiveEventLoop) {
//!         self.input.update_with_gilrs(&mut self.gilrs);
//!
//!         if self.input.pressed(Foo) { println!("bar") }
//!
//!         self.input.init();
//!     }
//! }
//! ```
mod input;
pub use derive_to_usize::ToUsize;
pub use crate::input::*;
/// creates new input map with binds and actions. anything that impliments `into<Input>` can be
/// used as bind
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
/// use winit::{keyboard::KeyCode, event::MouseButton}
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
