#![cfg_attr(test, feature(test))]
//! Define actions and their list of input binds. and then see if its `pressing`, `pressed` or
//! `released`. This library handles variable pressure through the `value` function as well as
//! multiple variable pressure inputs for things like 1d axis (through `axis`) and 2d vectors 
//! (through `dir` and `dir_max_len_1`). This makes it easy to have things like 3d camera controls
//! applied for both mouse movement and the gamepads right stick.
//!
//! The binds are structured by a list of `(Action, bind, bind, ..)`. An action is pressed if any
//! of its binds are pressed. Binds are described as either `[InputCode, InputCode, ..]` or
//! InputCode. A bind is pressed if all its InputCodes are pressed.
//!
//! The input map also can get the `mouse_pos`, what was `recently_pressed` (used for rebinding
//! things) and the `text_typed` (useful for typing) to make sure it is fully featured.
//! For better user control there are the `mouse_scale` and `scroll_scale` variables to make
//! sensitivity align with eveything else 0-1 range and a `press_sensitivity` to control when an
//! action counts as being pressed. Finaly, theres an `input_map!` macro to reduce boilerplate and
//! increase readability.
//! ```no_run
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
//! let input = { use base_input_codes::*; input_map!(
//!     (Foo, KeyCode::Space, [GamepadInput::South, RightTrigger])
//! ) };
//! let ev = EventLoop::new().unwrap();
//! let gilrs = Gilrs::new().unwrap();
//! ev.run_app(&mut App { window: None, input, gilrs }).unwrap();
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
//!         &mut self, _: &ActiveEventLoop, id: DeviceId,
//!         event: DeviceEvent
//!     ) {
//!         self.input.update_with_device_event(id, &event);
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

/// Outputs an input with the inputed binds.
///
/// The input is structured by a list of `(Action, bind, bind, ..)`. An action is pressed if any of
/// its binds are pressed.
///
/// Binds are described as either `[InputCode, InputCode, ..]` or InputCode. A bind is pressed if all
/// its InputCodes are pressed.
/// ```
/// use winit_input_map::*;
/// use Action::*;
/// #[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
/// enum Action {
///     Select, Undo, Redo, Confirm
/// }
/// let input = { use base_input_codes::*; input_map!(
///     (Select, MouseButton::Left, ShiftLeft, ShiftRight),
///     (Action::Undo,  [KeyZ, ControlLeft], [KeyZ, ControlRight]),
///     (Redo,  [KeyR, ControlLeft], [KeyR, ControlRight]),
///     (Confirm, MouseButton::Left, Enter)
/// ) };
/// ```
#[macro_export]
macro_rules! input_map {
    () => { $crate::InputMap::empty() };
    ( $( ( $x: expr, $( $k: expr ),* ) ),* ) => {
        $crate::InputMap::new(
            &$crate::binds!($(
                ($x, $( $k ),* )
            ),*)
        )
    };
}
/// Outputs binds in the expected format for `add_binds`, `set_binds` and `InputMap::new` though in
/// the latter case `input_map!` should be used to skip the middle man.
///
/// The input is structured by a list of `(Action, bind, bind, ..)`. An action is pressed if any of
/// its binds are pressed.
///
/// Binds are described as either `[InputCode, InputCode, ..]` or InputCode. A bind is pressed if all
/// its InputCodes are pressed.
/// ```
/// use winit_input_map::*;
/// use Action::*;
/// #[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
/// enum Action {
///     Select, Undo, Redo, Confirm
/// }
/// let mut input = input_map!((Select, base_input_codes::ShiftLeft));
///
/// let binds = { use base_input_codes::*; binds!(
///     (Select, MouseButton::Left, ShiftRight),
///     (Action::Undo,  [KeyZ, ControlLeft], [KeyZ, ControlRight]),
///     (Redo,  [KeyR, ControlLeft], [KeyR, ControlRight]),
///     (Confirm, MouseButton::Left, Enter)
/// ) };
///
/// input.add_binds(&binds);
/// ```
#[macro_export]
macro_rules! binds {
    ( $( ( $x: expr, $( $k: expr ),* ) ),* ) => { {
        $crate::as_binds(&[ $(
                ($x, &[ $( (&$k as &dyn $crate::AsBind) ),* ])
        ),* ])
    } };
}
/// Takes in more flexible deffinition of binds, the `binds!` macro is however, still prefered.
pub fn as_binds<'a, F: std::hash::Hash + Copy>(binds: &'a [(F, &'a [&'a dyn AsBind])]) -> crate::Binds<F> {
    let mut result = Vec::new();
    for (action, binds) in binds {
        let mut sub_result = Vec::new();
        for bind in *binds {
            sub_result.push(bind.as_bind());
        }
        result.push((*action, sub_result));
    }
    result
}

trait AsInputCode { fn as_code(&self) -> InputCode; }
impl<T: Into<InputCode> + Copy> AsInputCode for T { fn as_code(&self) -> InputCode { (*self).into() } }

/// Dictates what can be turned into binds for the purposes of the `as_binds()` method and the
/// `binds!()` and `input_map!()` macros
pub trait AsBind { fn as_bind(&self) -> Vec<InputCode>; }
impl<T: AsInputCode> AsBind for T { fn as_bind(&self) -> Vec<InputCode> { vec![self.as_code()] } }
impl<T: AsInputCode, const X: usize> AsBind for [T; X] { 
    fn as_bind(&self) -> Vec<InputCode> {
        let mut result = Vec::new();
        for i in self {
            let bind: InputCode = i.as_code();
            result.push(bind);
        }
        result
    }
}
impl<T: AsInputCode> AsBind for [T] { 
    fn as_bind(&self) -> Vec<InputCode> {
        let mut result = Vec::new();
        for i in self {
            let bind: InputCode = i.as_code();
            result.push(bind);
        }
        result
    }
}
impl<T: AsInputCode> AsBind for &[T] { 
fn as_bind(&self) -> Vec<InputCode> {
    let mut result = Vec::new();
    for i in *self {
        let bind: InputCode = i.as_code();
        result.push(bind);
    }
    result
}
}
impl AsBind for Vec<&dyn AsInputCode> {
    fn as_bind(&self) -> Vec<InputCode> {
        let mut result = Vec::new();
        for i in self {
            let bind: InputCode = i.as_code();
            result.push(bind);
        }
        result
    }
}
impl AsBind for &[&dyn AsInputCode] {
    fn as_bind(&self) -> Vec<InputCode> {
        let mut result = Vec::new();
        for i in *self {
            let bind: InputCode = i.as_code();
            result.push(bind);
        }
        result
    }
}
impl AsBind for [&dyn AsInputCode] {
    fn as_bind(&self) -> Vec<InputCode> {
        let mut result = Vec::new();
        for i in self {
            let bind: InputCode = i.as_code();
            result.push(bind);
        }
        result
    }
}
impl<const X: usize> AsBind for &[&dyn AsInputCode; X] {
    fn as_bind(&self) -> Vec<InputCode> {
        let mut result = Vec::new();
        for i in *self {
            let bind: InputCode = i.as_code();
            result.push(bind);
        }
        result
    }
}
impl<const X: usize> AsBind for [&dyn AsInputCode; X] {
    fn as_bind(&self) -> Vec<InputCode> {
        let mut result = Vec::new();
        for i in self {
            let bind: InputCode = i.as_code();
            result.push(bind);
        }
        result
    }
}
