use winit::{
    dpi::PhysicalPosition,
    event::*,
    keyboard::{KeyCode, PhysicalKey}
};
/// you can use anything that implements the `Into<usize>` trait as an action, but it's recommended 
/// to use an action enum which derives `ToUsize`. `input_map!` macro reduces the boilerplate of
/// this function
/// ```
/// #[derive(ToUsize)] // could also manualy implement Into<usize>
/// enum Actions{
///     Debug,
///     Left,
///     Right,
///     Click
/// }
/// use winit::{event::*, keyboard::KeyCode, event_loop::*, window::Window};
/// use winit_input_map::*;
/// use Actions::*;
///
/// let mut input = InputMap::new([ // doesnt have to be in the same order as the enum
///     (Debug, vec![Input::keycode(KeyCode::Space)]),
///     (Click, vec![Input::Mouse(MouseButton::Left)]),
///     (Left,  vec![Input::keycode(KeyCode::ArrowLeft), Input::keycode(KeyCode::KeyA)]),
///     (Right, vec![Input::keycode(KeyCode::ArrowRight), Input::keycode(KeyCode::KeyD)]),
/// ]);
/// 
/// let event_loop = EventLoop::new().unwrap();
/// event_loop.set_control_flow(ControlFlow::Poll);
/// let _window = Window::new(&event_loop).unwrap();
///
/// event_loop.run(|event, target|{
///     input.update(&event);
///     match &event {
///         Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => { target.exit() },
///         Event::AboutToWait => {
///             if input.pressed(Debug) { println!("pressed {:?}", input.binds(Debug)) }
///             if input.pressing(Right) || input.pressing(Left) { 
///                 println!("axis: {}", input.axis(Right, Left)) 
///             }
///             if input.mouse_move != (0.0, 0.0) {
///                 println!("mouse moved: {:?} and is now at {:?}", input.mouse_move, input.mouse_pos)
///             }
///             if input.released(Click) { println!("released {:?}", input.binds(Click)) }
///             
///             std::thread::sleep(std::time::Duration::from_millis(100));
///             //put at end of loop because were done with inputs this loop.
///             input.init();
///         }
///         _ => ()
///     }
/// }).unwrap();
/// ```
pub struct InputMap<const BINDS: usize> {
    pub binds: [Vec<Input>; BINDS],
    pub pressing: [bool; BINDS],
    pub pressed:  [bool; BINDS],
    pub released: [bool; BINDS],
    /// the amount the scroll wheel has changed
    pub mouse_scroll: f32,
    pub mouse_move: Vec2,
    pub mouse_pos: Vec2,
    /// last input even if it isnt in the binds. useful for rebinding.
    pub other_pressed: Option<Input>,
    /// the text typed this loop. useful for typing
    pub text_typed: Option<String>
}
impl<const BINDS: usize> InputMap<BINDS> {
    /// create new input system. recommended to use an action enum which implements the 
    /// `Into<usize>` trait. the `input_map!` macro reduces boilerplate.
    /// ```
    /// use Action::*;
    /// use input::*;
    /// use winit::keyboard::KeyCode;
    /// #[derive(ToUsize)]
    /// enum Action {
    ///     Forward,
    ///     Back,
    ///     Left,
    ///     Right
    /// }
    /// //doesnt have to be the same ordered as the enum.
    /// let mut input = Input::new([
    ///     (vec![Input::keycode(KeyCode::KeyW)], Forward),
    ///     (vec![Input::keycode(KeyCode::KeyA)], Left),
    ///     (vec![Input::keycode(KeyCode::KeyS)], Back),
    ///     (vec![Input::keycode(KeyCode::KeyD)], Right)
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
            binds: temp_binds,
            pressing: [false; BINDS],
            pressed:  [false; BINDS],
            released: [false; BINDS],
            mouse_scroll: 0.0,
            mouse_move: v(0.0, 0.0),
            mouse_pos:  v(0.0, 0.0),
            other_pressed: None,
            text_typed:    None
        }
    }
    /// use if you dont want to have any action. will still have access to everythin else
    pub fn empty() -> InputMap<0> {
        InputMap {
            mouse_scroll: 0.0,
            mouse_move: v(0.0, 0.0),
            mouse_pos:  v(0.0, 0.0),
            other_pressed: None,
            text_typed:    None,
            binds:    [],
            pressing: [],
            pressed:  [],
            released: []
        }
    }
    /// updates the input using a winit event. make sure to call `input.init()` when your done with
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
    pub fn update(&mut self, event: &Event<()>) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CursorMoved { position, .. } => {
                    self.update_mouse(*position);
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    self.update_buttons(state, *button)
                }
                WindowEvent::KeyboardInput { event, .. } => self.update_keys(event),
                _ => (),
            },
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => self.update_mouse_move(*delta),
                DeviceEvent::MouseWheel { delta } => self.mouse_scroll += match delta {
                    MouseScrollDelta::LineDelta(_, change) => *change,
                    MouseScrollDelta::PixelDelta(PhysicalPosition { y: change, .. }) => *change as f32
                },
                _ => (),
            },
            _ => (),
        }
    }
    /// initialise input. use when your done with input this loop. required to be called for 
    /// everything except `pressing` and `mouse_pos` to work properly.
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
    /// event_loop.run(|event, target| {
    ///     input.update(&event);
    ///     match &event{
    ///         Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => target.exit(),
    ///         Event::AboutToWait => input.init(),
    ///         _ => ()
    ///     }
    /// });
    /// ```
    pub fn init(&mut self) {
        self.mouse_move = v(0.0, 0.0);
        self.pressed = [false; BINDS];
        self.released = [false; BINDS];
        self.mouse_scroll  = 0.0;
        self.text_typed    = None;
        self.other_pressed = None;
    }
    /// you should use `self.update()` instead
    fn update_mouse(&mut self, position: PhysicalPosition<f64>) {
        self.mouse_pos = v(position.x as f32, position.y as f32);
    }

    fn update_mouse_move(&mut self, delta: (f64, f64)) {
        self.mouse_move = v(delta.0 as f32, delta.1 as f32);
    }
    /// you should use `self.update()` instead
    fn update_keys(&mut self, event: &KeyEvent) {
        let input_code = Input::Key(event.physical_key);
        self.other_pressed = Some(input_code);

        if let (Some(string), Some(new)) = (&mut self.text_typed, &event.text) {
            string.push_str(new);
        } else { self.text_typed = event.text.as_ref().map(|i| i.to_string()) }

        for (i, key) in self.binds.iter().enumerate() {
            if key.contains(&input_code) {
                self.pressed[i] = event.state.is_pressed() && !self.pressing[i];
                self.released[i] = !event.state.is_pressed() && self.pressing[i];
                self.pressing[i] = event.state.is_pressed();
            }
        }
    }
    /// you should use `self.update()` instead
    fn update_buttons(&mut self, state: &ElementState, button: MouseButton) {
        let input_code = Input::Mouse(button);
        self.other_pressed = Some(input_code);
        for (i, key) in self.binds.iter().enumerate() {
            if key.contains(&input_code) {
                self.pressed[i] = state.is_pressed() && !self.pressing[i];
                self.released[i] = !state.is_pressed() && self.pressing[i];
                self.pressing[i] = state.is_pressed();
            }
        }
    }
    /// get binds of action. same as `self.binds[action.into()]`
    pub fn binds(&mut self, action: impl Into<usize>) -> &mut Vec<Input> {
        &mut self.binds[action.into()]
    }
    /// checks if action is being pressed currently. same as `self.pressing[action.into()]`
    pub fn pressing(&self, action: impl Into<usize>) -> bool {
        self.pressing[action.into()]
    }
    /// checks if action was just pressed. same as `self.pressed[action.into()]`
    pub fn pressed(&self, action: impl Into<usize>) -> bool {
        self.pressed[action.into()]
    }
    /// checks if action was just released. same as `self.released[action.into()]`
    pub fn released(&self, action: impl Into<usize>) -> bool {
        self.released[action.into()]
    }
    /// returns 1.0 if pos is pressed, -1.0 if neg is pressed or 0.0 if either pos and neg
    /// or nothing is pressed. usefull for movement controls.
    /// ```
    /// let move_dir = (input.axis(Right, Left), input.axis(Up, Down));
    /// ```
    pub fn axis(&self, pos: impl Into<usize>, neg: impl Into<usize>) -> f32 {
        (self.pressing(pos) as i8 - self.pressing(neg) as i8) as f32
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Input {
    Key(PhysicalKey),
    Mouse(MouseButton),
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
