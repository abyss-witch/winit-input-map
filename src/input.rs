use winit::{
    dpi::PhysicalPosition,
    event::*,
    keyboard::{KeyCode, PhysicalKey}
};
/// A struct that handles all your input needs once you've hooked it up to winit and gilrs.
/// ```
/// struct App<const BINDS: usize> {
///     window: Option<Window>,
///     input: InputMap<BINDS>,
///     gilrs: Gilrs
/// }
/// impl<const BINDS: usize> ApplicationHandler for App<BINDS> {
///     fn resumed(&mut self, event_loop: &ActiveEventLoop) {
///         self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
///     }
///     fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
///         self.input.update_with_window_event(&event);
///         if let WindowEvent::CloseRequested = &event { event_loop.exit() }
///     }
///     fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, event: DeviceEvent) {
///         self.input.update_with_device_event(&event);
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
pub struct InputMap<const BINDS: usize> {
    pub binds: [Vec<Input>; BINDS],
    pub press_val: [f32; BINDS],
    pub pressed:  [bool; BINDS],
    pub released: [bool; BINDS],
    /// the amount the scroll wheel has changed
    pub mouse_pos: Vec2,
    /// last input even if it isnt in the binds. useful for rebinding.
    pub other_pressed: Option<Input>,
    /// the text typed this loop. useful for typing
    pub text_typed: Option<String>,
    pub mouse_scale: f32,
    pub scroll_scale: f32
}
impl<const BINDS: usize> InputMap<BINDS> {
    /// create new input system. recommended to use an action enum which implements the 
    /// `Into<usize>` trait. using the `input_map!` macro to reduce boilerplate is recommended.
    /// ```
    /// use Action::*;
    /// use winit_input_map::*;
    /// use winit::keyboard::KeyCode;
    /// #[derive(ToUsize)]
    /// enum Action {
    ///     Forward,
    ///     Back,
    ///     Pos,
    ///     Neg
    /// }
    /// //doesnt have to be the same ordered as the enum.
    /// let mut input = Input::new([
    ///     (vec![Input::keycode(KeyCode::KeyW)], Forward),
    ///     (vec![Input::keycode(KeyCode::KeyA)], Pos),
    ///     (vec![Input::keycode(KeyCode::KeyS)], Back),
    ///     (vec![Input::keycode(KeyCode::KeyD)], Neg)
    /// ]);
    /// ```
    pub fn new(binds: [(impl Into<usize>, Vec<Input>); BINDS]) -> Self {
        const NONE: Vec<Input> = Vec::new();
        let mut temp_binds = [NONE; BINDS];
        for (i, binds) in binds {
            let i = i.into();
            if binds.is_empty() {
                println!("no binds for {i:?}")
            }
            if i >= BINDS {
                panic!("input action is larger than bounds of array.")
            }
            temp_binds[i] = binds;
        }
        Self {
            mouse_scale: 0.1,
            scroll_scale: 0.1,
            binds: temp_binds,
            press_val: [0.0; BINDS],
            pressed:  [false; BINDS],
            released: [false; BINDS],
            mouse_pos:  v(0.0, 0.0),
            other_pressed: None,
            text_typed:    None
        }
    }
    /// use if you dont want to have any actions and binds. will still have access to everything else.
    pub fn empty() -> InputMap<0> {
        InputMap {
            mouse_scale: 0.1,
            scroll_scale: 0.1,
            mouse_pos:  v(0.0, 0.0),
            other_pressed: None,
            text_typed:    None,
            binds:    [],
            press_val: [],
            pressed:  [],
            released: []
        }
    }
    /// updates the input map using a winit event. make sure to call `input.init()` when your done with
    /// the input this loop.
    /// ```no_run
    /// use winit::{event::*, window::Window, event_loop::EventLoop};
    /// use winit_input_map::InputMap;
    ///
    /// let mut event_loop = EventLoop::new().unwrap();
    /// event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    /// let _window = Window::new(&event_loop).unwrap();
    ///
    /// let mut input = input_map!();
    ///
    /// event_loop.run(|event, target|{
    ///     input.update(&event);
    ///     match &event{
    ///         Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => target.exit(),
    ///         Event::AboutToWait => input.init(),
    ///         _ => ()
    ///     }
    /// });
    /// ```
    #[deprecated = "use `update_with_window_event` and `update_with_device_event`"]
    pub fn update_with_winit(&mut self, event: &Event<()>) {
        match event {
            Event::WindowEvent { event, .. } => self.update_with_window_event(event),
            Event::DeviceEvent { event, .. } => self.update_with_device_event(event),
            _ => ()
        }
    }
    pub fn update_with_device_event(&mut self, event: &DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                let x = delta.0 as f32 * self.mouse_scale;
                let y = delta.1 as f32 * self.mouse_scale;
                self.update_val(Input::MouseMoveX(AxisSign::Pos), x.max(0.0));
                self.update_val(Input::MouseMoveX(AxisSign::Neg), (-x).max(0.0));
                self.update_val(Input::MouseMoveY(AxisSign::Pos), y.max(0.0));
                self.update_val(Input::MouseMoveY(AxisSign::Neg), (-y).max(0.0));
            },
            DeviceEvent::MouseWheel { delta } => {
                let change = match delta {
                    MouseScrollDelta::LineDelta(_, change) => *change,
                    MouseScrollDelta::PixelDelta(PhysicalPosition { y, .. }) => *y as f32
                } * self.scroll_scale;
                self.update_val(Input::MouseScroll(AxisSign::Pos), change.max(0.0));
                self.update_val(Input::MouseScroll(AxisSign::Neg), (-change).max(0.0));
            },
             _ => (),
        }
    }
    pub fn update_with_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::CursorMoved { position, .. } => self.update_mouse(*position),
            WindowEvent::MouseInput { state, button, .. } => self.update_buttons(state, *button),
            WindowEvent::KeyboardInput { event, .. } => self.update_keys(event),
            _ => ()
        }
    }
    #[cfg(feature = "gamepad")]
    pub fn update_with_gilrs(&mut self, gilrs: &mut gilrs::Gilrs) {
        while let Some(ev) = gilrs.next_event() {
            self.update_gamepad(ev);
        }
    }
    /// makes the input map ready to recieve new events.
    pub fn init(&mut self) {
        self.update_val(Input::MouseMoveX(AxisSign::Pos), 0.0);
        self.update_val(Input::MouseMoveX(AxisSign::Neg), 0.0);
        self.update_val(Input::MouseMoveY(AxisSign::Pos), 0.0);
        self.update_val(Input::MouseMoveY(AxisSign::Neg), 0.0);
        self.update_val(Input::MouseScroll(AxisSign::Pos), 0.0);
        self.update_val(Input::MouseScroll(AxisSign::Neg), 0.0);
        self.pressed  = [false; BINDS];
        self.released = [false; BINDS];
        self.text_typed    = None;
        self.other_pressed = None;
    }
    fn update_mouse(&mut self, position: PhysicalPosition<f64>) {
        self.mouse_pos = v(position.x as f32, position.y as f32);
    }
    fn update_keys(&mut self, event: &KeyEvent) {
        let input_code = Input::Key(event.physical_key);

        if let (Some(string), Some(new)) = (&mut self.text_typed, &event.text) {
            string.push_str(new);
        } else { self.text_typed = event.text.as_ref().map(|i| i.to_string()) }

        self.update_val(input_code, event.state.is_pressed() as u8 as f32);
    }
    fn update_buttons(&mut self, state: &ElementState, button: MouseButton) {
        let input_code = Input::Mouse(button);
        self.update_val(input_code, state.is_pressed() as u8 as f32);
    }
    /// updates provided input code
    fn update_val(&mut self, input_code: Input, val: f32) {
        let pressed = val != 0.0;
        if pressed { self.other_pressed = Some(input_code) }
        for (i, key) in self.binds.iter().enumerate() {
            if key.contains(&input_code) {
                self.pressed[i] = pressed && !self.pressing(i);
                self.released[i] = !pressed && self.pressing(i);
                self.press_val[i] = val;
            }
        }
    }
    #[cfg(feature = "gamepad")]
    fn update_gamepad(&mut self, event: gilrs::Event) {
        let gilrs::Event { id, event, .. } = event;
        let id = SpecifyGamepad::Id(id);

        use gilrs::ev::EventType;
        match event {
            EventType::ButtonPressed(b, _) => {
                let input = GamepadInput::Button(b);
                self.update_val(Input::Gamepad { id, input }, 1.0);
                self.update_val(Input::Gamepad { id: SpecifyGamepad::Any, input }, 1.0);
            },
            EventType::ButtonReleased(b, _) => {
                let input = GamepadInput::Button(b);
                self.update_val(Input::Gamepad { id, input }, 0.0);
                self.update_val(Input::Gamepad { id: SpecifyGamepad::Any, input }, 0.0);
            },
            EventType::ButtonChanged(b, v, _) => {
                self.update_val(Input::Gamepad { id, input: b.into() }, v);
            },
            EventType::AxisChanged(b, v, _) => {
                use GamepadInput::Axis;
                use AxisSign::*;
                let dir_pos = v.max(0.0);
                let dir_neg = (-v).max(0.0);
                self.update_val(Input::Gamepad { id, input: Axis(b, Pos) }, dir_neg);
                self.update_val(Input::Gamepad { id: SpecifyGamepad::Any, input: Axis(b, Pos) }, dir_neg);

                self.update_val(Input::Gamepad { id, input: Axis(b, Neg) }, dir_pos);
                self.update_val(Input::Gamepad { id: SpecifyGamepad::Any, input: Axis(b, Neg) }, dir_pos);
            }
            _ => ()
        }
    }
    /// get binds of action. same as `self.binds[action.into()]`
    pub fn binds(&mut self, action: impl Into<usize>) -> &mut Vec<Input> {
        &mut self.binds[action.into()]
    }
    /// checks if action is being pressed currently. same as `self.press_val[action.into()] != 0.0`
    pub fn pressing(&self, action: impl Into<usize>) -> bool {
        self.press_val[action.into()] != 0.0
    }
    /// checks how much action is being pressed. same as `self.press_val[action.into()]`
    pub fn press_val(&self, action: impl Into<usize>) -> f32 {
        self.press_val[action.into()]
    }
    /// checks if action was just pressed. same as `self.pressed[action.into()]`
    pub fn pressed(&self, action: impl Into<usize>) -> bool {
        self.pressed[action.into()]
    }
    /// checks if action was just released. same as `self.released[action.into()]`
    pub fn released(&self, action: impl Into<usize>) -> bool {
        self.released[action.into()]
    }
    /// returns f32 based on how much pos and neg are pressed. may return values higher than 1.0 in
    /// the case of mouse movement and scrolling. usefull for movement controls.
    /// ```
    /// let move_dir = (input.axis(Neg, Pos), input.axis(Up, Down));
    /// ```
    pub fn axis(&self, pos: impl Into<usize>, neg: impl Into<usize>) -> f32 {
        self.press_val(pos) - self.press_val(neg)
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Input {
    Key(PhysicalKey),
    Mouse(MouseButton),
    MouseMoveX(AxisSign),
    MouseMoveY(AxisSign),
    MouseScroll(AxisSign),
    #[cfg(feature = "gamepad")]
    Gamepad { id: SpecifyGamepad, input: GamepadInput  }
}
impl Input {
    pub const fn keycode(key: KeyCode) -> Self {
        Self::Key(PhysicalKey::Code(key))
    }
}
impl From<KeyCode> for Input {
    fn from(value: KeyCode) -> Input {
        Self::keycode(value)
    }
}
impl From<PhysicalKey> for Input {
    fn from(value: PhysicalKey) -> Input {
        Self::Key(value)
    }
}
impl From<MouseButton> for Input {
    fn from(value: MouseButton) -> Input {
        Self::Mouse(value)
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AxisSign {
    Pos,
    Neg
}

#[cfg(feature = "gamepad")]
pub use gamepad::*;
#[cfg(feature = "gamepad")]
mod gamepad {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum GamepadInput {
        Button(gilrs::ev::Button),
        Axis(gilrs::ev::Axis, crate::AxisSign)
    }
    impl From<gilrs::Button> for GamepadInput {
        fn from(value: gilrs::Button) -> GamepadInput {
            GamepadInput::Button(value)
        }
    }
    impl From<GamepadInput> for crate::Input {
        fn from(value: GamepadInput) -> crate::Input {
            Self::Gamepad { input: value, id: Default::default() }
        }
    }
    impl From<gilrs::Button> for crate::Input {
        fn from(value: gilrs::Button) -> crate::Input {
            Self::Gamepad { input: GamepadInput::Button(value), id: Default::default() }
        }
    }
    /// specify gamepad to use
    #[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
    pub enum SpecifyGamepad {
        /// cant be set at compile time. use `Any` as default and then let the user select a specific
        /// gamepad
        Id(gilrs::GamepadId),
        /// use as default
        #[default]
        Any
    }
}
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
