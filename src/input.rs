#[cfg(feature = "glium-types")]
use glium_types::vectors::{vec2, Vec2};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{DeviceEvent, ElementState, Event, KeyEvent, MouseButton, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};
/// input system. define actions and their key binds and then see if their pressing, pressed or released. get mouse position and how much its moved. you can use anythin that implements the `Into<usize>` trait as an action, but it's recommended to use an action enum.
/// ```
/// enum Actions{
///     Debug,
///     Left,
///     Right,
///     Click
/// }
/// impl Into<usize> for Actions{
///     fn into(self) -> usize {
///         self as usize
///     }
/// }
/// use input::{InputMap, Input};
/// use Actions::*;
///
/// let mut input = Input::new([
///     (vec![Input::keycode(KeyCode::Space)], Debug),
///     (vec![Input::keycode(KeyCode::ArrowLeft), InputCode::keycode(KeyCode::KeyA)], Left),
///     (vec![Input::keycode(KeyCode::ArrowRight), InputCode::keycode(KeyCode::KeyD)], Right),
///     (vec![Input::Mouse(MouseButton::Left)], Click)
/// ]);
///     
/// use winit::{event::*, keyboard::KeyCode, window::WindowAttribures};
/// let event_loop = winit::event_loop::EventLoop::new().unwrap();
/// event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
/// let _window = event_loop.create_window(WindowAttribures::default()).unwrap();
///
/// event_loop.run(|event, target|{
///     input.update(&event);
///     match &event {
///         Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => { target.exit() },
///         Event::AboutToWait => {
///             if input.pressed(Debug) { println!("pressed {:?}", input.binds(Debug)) }
///             if input.pressing(Right) || input.pressing(Left) { println!("axis: {}", input.axis(Right, Left)) }
///             if input.mouse_move != (0.0, 0.0) { println!("mouse moved: {:?} and is now at {:?}", input.mouse_move, input.mouse_pos) }
///             if input.released(Click) { println!("released {:?}", input.binds(Click)) }
///             
///             std::thread::sleep(std::time::Duration::from_millis(100));
///             //put at end of loop because were done with inputs this frame.
///             input.init(;
///         }
///         _ => ()
///     }
/// }).unwrap();
/// ```
pub struct InputMap<const BINDS: usize> {
    pub binds: [Vec<Input>; BINDS],
    pub pressing: [bool; BINDS],
    pub pressed: [bool; BINDS],
    pub released: [bool; BINDS],
    #[cfg(feature = "glium-types")]
    pub mouse_move: Vec2,
    #[cfg(not(feature = "glium-types"))]
    pub mouse_move: (f32, f32),
    #[cfg(feature = "glium-types")]
    pub mouse_pos: Vec2,
    #[cfg(not(feature = "glium-types"))]
    pub mouse_pos: (f32, f32),
}
impl<const BINDS: usize> InputMap<BINDS> {
    ///create new input system. recommended to use an action enum which implements the `Into<usize>` trait for the second value.
    /// ```
    /// enum Action{
    ///     Forward,
    ///     Back,
    ///     Left,
    ///     Right
    /// }
    /// impl Into<usize> for Action{
    ///     fn into(self) -> usize{
    ///         self as usize
    ///     }
    /// }
    ///
    /// use Action::*;
    /// use input::{Input, InputCode};
    /// use winit::keyboard::KeyCode;
    /// //doesnt have to be the same ordered as the enum.
    /// let mut input = Input::new([
    ///     (vec![Input::keycode(KeyCode::KeyW)], Forward),
    ///     (vec![Input::keycode(KeyCode::KeyA)], Left),
    ///     (vec![Input::keycode(KeyCode::KeyS)], Back),
    ///     (vec![Input::keycode(KeyCode::KeyD)], Right)
    /// ]);
    /// ```
    pub fn new(binds: [(Vec<Input>, impl Into<usize>); BINDS]) -> Self {
        const NONE: Vec<Input> = Vec::new();
        let mut temp_binds = [NONE; BINDS];
        for (key, i) in binds {
            let i = i.into();
            if key.is_empty() {
                println!("no binds for {i:?}")
            }
            if i >= BINDS {
                panic!("input action is larger than bounds of array.")
            }
            temp_binds[i] = key;
        }
        Self {
            binds: temp_binds,
            pressing: [false; BINDS],
            pressed: [false; BINDS],
            released: [false; BINDS],
            mouse_move: v(0.0, 0.0),
            mouse_pos: v(0.0, 0.0),
        }
    }
    /// updates the input using a winit event. requires `input.init()` to be used before being updated.
    /// ```no_run
    /// use winit::event::*;
    /// use input::Input;
    ///
    /// let mut event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
    /// event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    ///
    /// let mut input = Input::new([]);
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
                _ => (),
            },
            _ => (),
        }
    }
    /// initialise input. required to be called for `mouse_move`, `pressed()` and `released()` to work.
    /// required to put `input.init()` before `input.update()`
    /// ```no_run
    /// use winit::event::*;
    /// use input::InputMap;
    ///
    /// let mut event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
    /// event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    ///
    /// let mut input = InputMap::new([]);
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
    pub fn init(&mut self) {
        self.mouse_move = v(0.0, 0.0);
        self.pressed = [false; BINDS];
        self.released = [false; BINDS];
    }
    ///you should use `self.update()` instead
    fn update_mouse(&mut self, position: PhysicalPosition<f64>) {
        self.mouse_pos = v(position.x as f32, position.y as f32);
    }

    fn update_mouse_move(&mut self, delta: (f64, f64)) {
        self.mouse_move = v(delta.0 as f32, delta.1 as f32);
    }
    ///you should use `self.update()` instead
    fn update_keys(&mut self, event: &KeyEvent) {
        for (i, key) in self.binds.iter().enumerate() {
            if key.contains(&Input::Key(event.physical_key)) {
                self.pressed[i] = event.state.is_pressed() && !self.pressing[i];
                self.released[i] = !event.state.is_pressed() && self.pressing[i];
                self.pressing[i] = event.state.is_pressed();
            }
        }
    }
    ///you should use `self.update()` instead
    fn update_buttons(&mut self, state: &ElementState, button: MouseButton) {
        for (i, key) in self.binds.iter().enumerate() {
            if key.contains(&Input::Mouse(button)) {
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
    /// returns 1.0 if pos is pressed, -1.0 if neg is pressed or 0.0 if either pos and neg or nothing is pressed. usefull for movement controls. same as `input::axis(input.pressing(pos), input.pressing(neg))`
    /// ```no_run
    /// let move_dir = (input.axis(Right, Left), input.axis(Up, Down));
    /// ```
    pub fn axis(&self, pos: impl Into<usize>, neg: impl Into<usize>) -> f32 {
        crate::input::axis(self.pressing(pos), self.pressing(neg))
    }
}
///converts two bools into a -1.0 to 1.0 float value. useful for movement controls
/// ```no_run
/// use input::axis;
/// let x = axis(input.pressing(Right), input.pressing(Left));
/// let z = axis(input.pressing(Forward), input.pressing(Back));
///
/// let move_dir = (x, z)
/// ```
pub fn axis(pos: bool, neg: bool) -> f32 {
    let dir = pos as i8 - neg as i8;
    dir as f32
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
#[cfg(feature = "glium-types")]
fn v(a: f32, b: f32) -> Vec2 {
    vec2(a, b)
}
#[cfg(not(feature = "glium-types"))]
fn v(a: f32, b: f32) -> (f32, f32) {
    (a, b)
}
