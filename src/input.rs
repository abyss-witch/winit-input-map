#[cfg(feature = "mice-keyboard")]
use winit::{
    dpi::PhysicalPosition,
    event::*,
};
use crate::input_code::*;
use std::collections::HashMap;
use std::{cmp::Eq, hash::Hash};
#[cfg(not(feature = "glium-types"))]
type Vec2 = (f32, f32);
#[cfg(feature = "glium-types")]
type Vec2 = glium_types::vectors::Vec2;
fn v(a: f32, b: f32) -> Vec2 {
    #[cfg(not(feature = "glium-types"))]
    { (a, b) }
    #[cfg(feature = "glium-types")]
    { Vec2::new(a, b) }
}
/// Values are the current value of the action, if its pressed, if its released and the sub values
/// that make up the current value.
type ActionValue = (f32, bool, bool, Vec<(f32, Vec<f32>)>);
/// Binds are the list of connected actions and its sub indices
type BindHash<F> = Vec<(F, usize, usize)>;
/// Binds are a list of actions and their bindings
pub type Binds<F> = Vec<(F, Vec<Vec<InputCode>>)>;
/// A struct that handles all your input needs once you've hooked it up to winit and gilrs.
/// ```
/// use gilrs::Gilrs;
/// use winit::{event::*, application::*, window::*, event_loop::*};
/// use winit_input_map::*;
/// struct App {
///     window: Option<Window>,
///     input: InputMap<()>,
///     gilrs: Gilrs
/// }
/// impl ApplicationHandler for App {
///     fn resumed(&mut self, event_loop: &ActiveEventLoop) {
///         self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
///     }
///     fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
///         self.input.update_with_window_event(&event);
///         if let WindowEvent::CloseRequested = &event { event_loop.exit() }
///     }
///     fn device_event(&mut self, _: &ActiveEventLoop, id: DeviceId, event: DeviceEvent) {
///         self.input.update_with_device_event(id, &event);
///     }
///     fn about_to_wait(&mut self, _: &ActiveEventLoop) {
///         self.input.update_with_gilrs(&mut self.gilrs);
/// 
///         // put your code here
///
///         self.input.init();
///     }
/// }
/// ```
pub struct InputMap<F: Hash + Copy> {
    /// Stores what each input code previous press value and what action its bound to and its sub
    /// indices 
    bind_hash: HashMap<InputCode, BindHash<F>>,
    /// f32s and each bound current val, 1st bool is pressed and 2nd bool is released.
    action_val: HashMap<F, ActionValue>,
    /// weather the window has focus and therefor, if it should recieve inputs
    #[cfg(all(feature = "mice-keyboard", feature = "gamepad"))]
    focus: bool,
    /// The mouse position
    #[cfg(feature = "mice-keyboard")]
    pub mouse_pos: Vec2,
    /// The last input event, even if it isn't in the binds. Useful for handling rebinding
    pub recently_pressed: Option<InputCode>,
    /// The text typed this loop
    pub text_typed: Option<String>,
    /// Since most values are from 0-1 reducing the mouse sensitivity will result in better
    /// consistancy
    #[cfg(feature = "mice-keyboard")]
    pub mouse_scale: f32,
    /// Since most values are from 0-1 reducing the scroll sensitivity will result in better
    /// consistancy
    #[cfg(feature = "mice-keyboard")]
    pub scroll_scale: f32,
    /// The minimum value something has to be at to count as being pressed. Values over 1 will
    /// result in most buttons being unusable
    pub press_sensitivity: f32
}
impl InputMap<()> { 
    /// Use if you dont want to have any actions and binds. Will still have access to everything else.
    pub fn empty() -> Self { Self::default() }
}
impl<F: Hash + Copy> Default for InputMap<F> {
    fn default() -> Self {
        Self {
            press_sensitivity:  0.5,
            #[cfg(feature = "mice-keyboard")]
            mouse_scale:        0.01,
            #[cfg(feature = "mice-keyboard")]
            scroll_scale:       1.0,
            #[cfg(feature = "mice-keyboard")]
            mouse_pos:  v(0.0, 0.0),
            recently_pressed:  None,
            text_typed:        None,
            bind_hash:  HashMap::<InputCode, BindHash<F>>::new(),
            action_val: HashMap::<F, ActionValue>::new(),
            #[cfg(all(feature = "mice-keyboard", feature = "gamepad"))]
            focus: true,
        }
    }
}
impl<F: Hash + Copy + Eq> InputMap<F> {
    /// Creates a new input system. Takes the action and a list of its associated binds. An action
    /// will count as being pressed if any of the binds are pressed. A bind is a list of
    /// `InputCode`s that need to all be pressed for the bind to count as being pressed.
    ///
    /// It's recommended to use the `input_map!` macro to reduce boilerplate
    /// and increase readability.
    /// ```
    /// use Action::*;
    /// use winit_input_map::*;
    /// use winit::keyboard::KeyCode;
    /// #[derive(Hash, PartialEq, Eq, Clone, Copy)]
    /// enum Action {
    ///     Forward,
    ///     Back,
    ///     Pos,
    ///     Neg
    /// }
    /// // doesnt have to be the same ordered as the enum.
    /// let input = InputMap::new(&vec![
    ///     (Forward, vec![ vec![KeyCode::KeyW.into()] ]),
    ///     (Pos,     vec![ vec![KeyCode::KeyA.into()] ]),
    ///     (Back,    vec![ vec![KeyCode::KeyS.into()] ]),
    ///     (Neg,     vec![ vec![KeyCode::KeyD.into()] ])
    /// ]);
    /// ```
    pub fn new(binds: &Binds<F>) -> Self {
        let mut result = Self::default();
        result.add_binds(binds);
        result
    }
    /// Takes binds and adds them to the currently existing map. The `binds!()` macro will help
    /// reduce the boiler_plate of this function.
    pub fn add_binds(&mut self, binds: &Binds<F>) {
        for (action, binds) in binds {
            for (bind_i, bind) in binds.iter().enumerate() {
                self.action_val.entry(*action).or_default().3.push((0.0, vec![]));
                for (code_i, code) in bind.iter().enumerate() {
                    self.action_val.entry(*action).or_default().3[bind_i].1.push(0.0);
                    self.bind_hash.entry(*code).or_default().push((*action, bind_i, code_i));
                }
            }
        }
        self.action_val.shrink_to_fit();
        self.bind_hash.shrink_to_fit();
    }
    /// Removes all binds and then adds the inputed binds. The `binds!()` macro will help
    /// reduce the boiler_plate of this function.
    pub fn set_binds(&mut self, binds: &Binds<F>) {
        self.bind_hash.clear();
        self.add_binds(binds);
    }
    /// Returns the current binds of the InputMap, may not be in the same order as the inputed
    /// binds.
    pub fn get_binds(&self) -> Binds<F> {
        let mut results = Vec::new();

        for (input_code,  binds) in self.bind_hash.iter() {
            for (action, binds_i, bind_i) in binds.iter() {
                let action_i = if let Some(result) = results.iter().position(|(a, _)| a == action) { result }
                else { results.push((*action, vec![])); results.len() - 1 };

                let len = results[action_i].1.len();
                if *binds_i >= len { results[action_i].1.append(&mut vec![vec![]; binds_i - len + 1]); }

                let vec = &mut results[action_i].1[*binds_i];

                let len = vec.len();
                if *bind_i >= len { vec.append(&mut vec![*input_code; bind_i - len + 1]); }
            }
        }
        results
    }
    /// Updates the input map using a winit event. Make sure to call `input.init()` when your done with
    /// the input this loop.
    /// ```no_run
    /// use winit::{event::*, window::WindowAttributes, event_loop::EventLoop};
    /// use winit_input_map::*;
    ///
    /// let mut event_loop = EventLoop::new().unwrap();
    /// event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    /// let _window = event_loop.create_window(WindowAttributes::default()).unwrap();
    ///
    /// let mut input = input_map!();
    ///
    /// event_loop.run(|event, target|{
    ///     input.update_with_winit(&event);
    ///     match &event{
    ///         Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => target.exit(),
    ///         Event::AboutToWait => input.init(),
    ///         _ => ()
    ///     }
    /// });
    /// ```
    #[cfg(feature = "mice-keyboard")]
    #[deprecated = "use `update_with_window_event` and `update_with_device_event`"]
    pub fn update_with_winit(&mut self, event: &Event<()>) {
        match event {
            Event::WindowEvent { event, .. } => self.update_with_window_event(event),
            Event::DeviceEvent { event, device_id, .. } => self.update_with_device_event(*device_id, event),
            _ => ()
        }
    }
    #[cfg(feature = "mice-keyboard")]
    pub fn update_with_device_event(&mut self, id: DeviceId, event: &DeviceEvent) {
        use base_input_codes::*;
        match event {
            DeviceEvent::MouseMotion { delta } => {
                let x = delta.0 as f32 * self.mouse_scale;
                let y = delta.1 as f32 * self.mouse_scale;
                self.modify_val(MouseMoveRight.with_id(id), |v| v + x.max(0.0));
                self.modify_val(MouseMoveLeft .with_id(id), |v| v - x.min(0.0));
                self.modify_val(MouseMoveDown .with_id(id), |v| v + y.max(0.0));
                self.modify_val(MouseMoveUp   .with_id(id), |v| v - y.min(0.0));
            },
            DeviceEvent::MouseWheel { delta } => self.update_scroll(*delta, id),
             _ => (),
        }
    }
    #[cfg(feature = "mice-keyboard")]
    pub fn update_with_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::CursorMoved { position, .. } => self.update_mouse(*position),
            WindowEvent::MouseWheel { delta, device_id, .. } => self.update_scroll(*delta, *device_id),
            WindowEvent::MouseInput { state, button, device_id } => self.update_buttons(state, *device_id, *button),
            WindowEvent::KeyboardInput { event, device_id, .. } => self.update_keys(*device_id, event),
            WindowEvent::Focused(false) => {
                for val in self.action_val.values_mut() {
                    val.3.iter_mut().for_each(|i| { i.0 = 0.0; i.1.iter_mut().for_each(|i| *i = 0.0) });
                    val.0 = 0.0;
                }
                #[cfg(feature = "gamepad")] { self.focus = false; }
            },
            #[cfg(feature = "gamepad")]
            WindowEvent::Focused(true) => self.focus = true,
            _ => ()
        }
    }
    #[cfg(feature = "gamepad")]
    pub fn update_with_gilrs(&mut self, gilrs: &mut gilrs::Gilrs) {
        while let Some(ev) = gilrs.next_event() {
            if self.focus { self.update_gamepad(ev); }
        }
    }
    /// Makes the input map ready to recieve new events.
    pub fn init(&mut self) {
        #[cfg(feature = "mice-keyboard")]
        {
            use base_input_codes::*;
            for i in [MouseMoveLeft, MouseMoveRight,   
            MouseMoveUp, MouseMoveDown, MouseScrollUp,
            MouseScrollDown, MouseScrollLeft, 
            MouseScrollRight] {
                self.update_val(i.into(), 0.0);
            }
        }
        self.action_val.values_mut().for_each(|(_, p, r, _)| (*p, *r) = (false, false));
        self.recently_pressed = None;
        self.text_typed = None;
    }
    #[cfg(feature = "mice-keyboard")]
    fn update_scroll(&mut self, delta: MouseScrollDelta, id: DeviceId) {
        use base_input_codes::*;
        let (x, y) = match delta {
        MouseScrollDelta::LineDelta(x, y) => (x, y),
            MouseScrollDelta::PixelDelta(PhysicalPosition { x, y }) => (x as f32, y as f32)
        };
        let (x, y) = (x * self.scroll_scale, y * self.scroll_scale);
        
        self.modify_val(MouseScrollUp.with_id(id),    |v| v + y.max(0.0));
        self.modify_val(MouseScrollDown.with_id(id),  |v| v - y.min(0.0));
        self.modify_val(MouseScrollLeft.with_id(id),  |v| v + x.max(0.0));
        self.modify_val(MouseScrollRight.with_id(id), |v| v - x.min(0.0));
    }
    #[cfg(feature = "mice-keyboard")]
    fn update_mouse(&mut self, position: PhysicalPosition<f64>) {
        self.mouse_pos = v(position.x as f32, position.y as f32);
    }
    #[cfg(feature = "mice-keyboard")]
    fn update_keys(&mut self, id: DeviceId, event: &KeyEvent) {
        let input_code: DeviceInput = event.physical_key.into();

        if let (Some(string), Some(new)) = (&mut self.text_typed, &event.text) {
            string.push_str(new);
        } else { self.text_typed = event.text.as_ref().map(|i| i.to_string()) }

        self.update_val(input_code.with_id(id), event.state.is_pressed().into());
    }
    #[cfg(feature = "mice-keyboard")]
    fn update_buttons(&mut self, state: &ElementState, id: DeviceId, button: MouseButton) {
        let input_code: DeviceInput = button.into();
        self.update_val(input_code.with_id(id), state.is_pressed().into());
    }
    /// updates provided input code
    fn update_val(&mut self, input_code: InputCode, val: f32) {
        self.modify_val(input_code, |_| val);
    }
    fn modify_val<FN: Fn(f32) -> f32>(&mut self, input_code: InputCode, f: FN) {
        self.modify_single_val(input_code, &f);
        self.modify_single_val(input_code.set_any(), f);
    }
    /// doesnt update both generic ids and specified ids, use `update_val` or `modify_val` for that
    fn modify_single_val<FN: Fn(f32) -> f32>(&mut self, input_code: InputCode, f: FN) {
        let Some(binds) = self.bind_hash.get(&input_code) else {
            if f(0.0) >= self.press_sensitivity && !input_code.is_any() { self.recently_pressed = Some(input_code) }
            return;
        };

        for &(action, index, sub_index) in binds {
            let (curr_val, pressing, releasing, sub_values) = &mut self.action_val.get_mut(&action).unwrap();
            
            let old_sub_sub_val = sub_values[index].1[sub_index];
            let new_sub_sub_val = f(old_sub_sub_val);
            let change = new_sub_sub_val / old_sub_sub_val;
            sub_values[index].1[sub_index] = new_sub_sub_val;

            let sub_value = sub_values[index].0;
            let new_sub_val = if change.is_finite() { sub_value * change }
                else { sub_values[index].1.iter().fold(1.0, |a, b| a * b) };
            sub_values[index].0 = new_sub_val;

            *curr_val += new_sub_val - sub_value;

            let now_pressing = *curr_val >= self.press_sensitivity;
            if now_pressing && !input_code.is_any() { self.recently_pressed = Some(input_code) }

            let jpressed = now_pressing && !*pressing;
            let released = !now_pressing && *pressing;
            *pressing = jpressed;
            *releasing = released;
        }
    }
    #[cfg(feature = "gamepad")]
    fn update_gamepad(&mut self, event: gilrs::Event) {
        let gilrs::Event { id, event, .. } = event;
        use crate::input_code::{axis_pos, axis_neg};
        use gilrs::ev::EventType;
        match event {
            EventType::ButtonChanged(b, v, _) => {
                let a: GamepadInput = b.into();
                self.update_val(a.with_id(id), v);
            },
            EventType::AxisChanged(b, v, _) => {
                let dir_pos = v.max(0.0);
                let dir_neg = (-v).max(0.0);
                let input_pos = axis_pos(b);
                let input_neg = axis_neg(b);

                self.update_val(input_pos.with_id(id), dir_pos);
                self.update_val(input_neg.with_id(id), dir_neg);
            },
            EventType::Disconnected => {
                // reset input

                use GamepadInput::*;
                for i in [LeftStickLeft, LeftStickRight, LeftStickUp, LeftStickDown, LeftStickPress,
                 RightStickLeft, RightStickRight, RightStickUp, RightStickDown,
                 RightStickPress, DPadLeft, DPadRight, DPadUp, DPadDown, LeftZ, RightZ,
                 South, East, North, West, LeftBumper, LeftTrigger, RightBumper,
                 RightTrigger,  Select, Start, Mode, Other].iter() {
                    self.update_val(i.with_id(id), 0.0);
                 }
            }
            _ => ()
        }
    }
    /// Checks if action is being pressed currently based on the `press_sensitivity`.
    /// same as `self.value(action) >= self.press_sensitivty`.
    pub fn pressing(&self, action: F) -> bool {
        self.value(action) >= self.press_sensitivity
    }
    /// Checks how much an action is being pressed. May be higher than 1 in the case of scroll
    /// wheels, mouse movement or when multiple binds are bound to an action.
    pub fn value(&self, action: F) -> f32 {
        if let Some(&(v, _, _, _)) = self.action_val.get(&action) { v } else {  0.0  }
    }
    /// Checks if action was just pressed.
    pub fn pressed(&self, action: F) -> bool {
        if let Some(&(_, v, _, _)) = self.action_val.get(&action) { v } else { false }
    }
    /// Checks if action was just released.
    pub fn released(&self, action: F) -> bool {
        if let Some(&(_, _, v, _)) = self.action_val.get(&action) { v } else { false }
    }
    /// Returns f32 based on how much pos and neg are pressed. may return values higher than 1.0 in
    /// the case of mouse movement and scrolling. usefull for movement controls. for 2d values see
    /// `dir` and `dir_max_len_1`
    /// ```no_test
    /// let move_dir = input.axis(Neg, Pos);
    /// ```
    /// same as `input.value(pos) - input.value(neg)`
    pub fn axis(&self, pos: F, neg: F) -> f32 {
        self.value(pos) - self.value(neg)
    }
    /// Returns a vector based off of the x and y axis. Can return values with a length higher than
    /// 1, if this is undesirable see `dir_max_len_1`.
    pub fn dir(&self, pos_x: F, neg_x: F, pos_y: F, neg_y: F) -> Vec2 {
        v(self.axis(pos_x, neg_x), self.axis(pos_y, neg_y))
    }
    /// Returns a vector based off of x and y axis with a maximum length of 1 (the same as a normalised
    /// vector). If this undesirable see `dir`.
    pub fn dir_max_len_1(&self, pos_x: F, neg_x: F, pos_y: F, neg_y: F) -> Vec2 {
        let (x, y) = (self.axis(pos_x, neg_x), self.axis(pos_y, neg_y));
        // if lower than 1, set to 1. since x/1 = x, that means anything lower than 1 is left unchanged
        let length = (x*x + y*y).sqrt().max(1.0);
        v(x/length, y/length)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    extern crate test;
    #[bench]
    fn bench_input(b: &mut Bencher) {
        use Action::*;
        #[derive(PartialEq, Eq, Clone, Copy, Hash)]
        enum Action {
            Test1,
            Test2,
            Test3,
            Test4,
        }
        let mut input = { use base_input_codes::*; crate::input_map!(
            (Test1, [ControlLeft, KeyZ], [ShiftLeft, KeyZ]),
            (Test2, [ControlLeft, KeyS], KeyZ, KeyU),
            (Test3, [KeyX, KeyI], [KeyS, KeyV, ControlLeft, KeyZ]),
            (Test4, KeyZ)
        ) };
        
        b.iter(|| {
            input.update_val(base_input_codes::KeyZ.into(), 1.0);
            input.update_val(base_input_codes::ControlLeft.into(), 1.0);
        });
    }
}
