//! Define actions and their input binds and then see if its `pressing`, `pressed` or
//! `released`. This library handles variable pressure through the `action_val` function aswellas
//! multiple variable pressure inputs for things like 1d axis (through `axis`) and 2d vectors 
//! (through `dir` and `dir_max_len_1`). This makes it easy to have things like 3d camera controls
//! applied for both mouse movement and the gamepads right stick. The input map also can get the
//! `mouse_pos`, what was `recently_pressed` (used for rebinding things) and the `text_typed`
//! (useful for typing) to make sure it is fully featured. For better user control there are the
//! `mouse_scale` and `scroll_scale` variables to make sensitivity align with eveything else
//! 0-1 range and a `press_sensitivity` to control when a action counts as being pressed. Finaly,
//! theres an input_map! macro to reduce boilerplate and increase readability.
//! ```
//! use winit::{
//!     window::*, application::*, keyboard::*,
//!     event_loop::*, event::*
//! };
//! use gilrs::Gilrs;
//! use winit_input_map::*;
//!
//! #[derive(Hash, PartialEq, Eq, Clone, Copy)]
//! enum Actions{ Foo }
//! use Actions::*;
//!
//! let input = input_map!(
//!     (Foo, KeyCode::Space, GamepadInput::South)
//! );
//! let ev = EventLoop::new().unwrap();
//! let gilrs = Gilrs::new().unwrap();
//! ev.run_app(&mut App { window: None, input, gilrs}).unwrap();
//!
//! struct App {
//!     window: Option<Window>,
//!     input: InputMap<Actions>,
//!     gilrs: Gilrs
//! }
//! impl ApplicationHandler for App {
//!     fn resumed(&mut self, event_loop: &ActiveEventLoop) {
//!         let window_settings = Window::default_attributes();
//!         let window = event_loop.create_window(window_settings);
//!         self.window = Some(window.unwrap());
//!     }
//!     fn window_event(
//!         &mut self, event_loop: &ActiveEventLoop, _: WindowId,
//!         event: WindowEvent
//!     ) {
//!         self.input.update_with_window_event(&event);
//!         if let WindowEvent::CloseRequested = &event
//!         { event_loop.exit() }
//!     }
//!     fn device_event(
//!         &mut self, _: &ActiveEventLoop, _: DeviceId,
//!         event: DeviceEvent
//!     ) {
//!         self.input.update_with_device_event(&event);
//!     }
//!     fn about_to_wait(&mut self, _: &ActiveEventLoop) {
//!         self.input.update_with_gilrs(&mut self.gilrs);
//!         
//!         // your code here!
//!         if self.input.pressed(Foo) { println!("bar") }
//!
//!         self.input.init();
//!     }
//! }
//! ```
mod input;
mod input_code;
pub use crate::input::*;
pub use crate::input_code::*;
/// Creates new input map with inputed input codes bound to the acompaning action.
/// Anything that impliments `into<InputCode>` can be bound to an action
/// ```
/// use Action::*;
/// use winit_input_map::*;
/// use winit::{keyboard::KeyCode, event::MouseButton};
/// #[derive(Hash, PartialEq, Eq, Clone, Copy)]
/// enum Action {
///     Jump,
///     Left,
///     Right,
///     Interact
/// }
/// let mut input = input_map!(
///     (Jump,     KeyCode::Space                    ),
///     (Left,     KeyCode::KeyA, KeyCode::ArrowLeft ),
///     (Right,    KeyCode::KeyD, KeyCode::ArrowRight),
///     (Interact, MouseButton::Left                 )
/// );
/// ```
#[macro_export]
macro_rules! input_map {
    () => { InputMap::<()>::empty() };
    ( $( ( $x:expr, $( $k:expr ),* ) ),* ) => {
        InputMap::new(&[ $(
            ($x, vec![ $( $k.into(), )* ]),
        )*])
    };
}
