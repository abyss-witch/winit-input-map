#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
/// Enum that specifies an input
pub enum InputCode {
    #[cfg(feature = "mice-keyboard")]
    Device { id: SpecifyDevice, input: DeviceInput },
    #[cfg(feature = "gamepad")]
    Gamepad { id: SpecifyGamepad, input: GamepadInput }
}
impl InputCode {
    /// sets `SpecifyGamepad` or `SpecifyDevice` to any
    pub fn set_any(self) -> Self {
        match self {
            #[cfg(feature = "gamepad")]
            Self::Gamepad { input, .. } => input.into(),
            #[cfg(feature = "mice-keyboard")]
            Self::Device  { input, .. } => input.into(),
        }
    }
    pub fn is_any(self) -> bool {
        match self {
            #[cfg(feature = "gamepad")]
            Self::Gamepad { id, .. } => id == SpecifyGamepad::Any,
            #[cfg(feature = "mice-keyboard")]
            Self::Device  { id, .. } => id == SpecifyDevice::Any,
        }
    }
    #[cfg(feature = "mice-keyboard")]
    pub fn has_device_id(&self, id: winit::event::DeviceId) -> bool {
        match self {
            Self::Device { id: SpecifyDevice::Id(cid), .. } => *cid == id,
            Self::Device { id: SpecifyDevice::Any,     .. } => true,
            _ => false
        }
    }
    #[cfg(feature = "gamepad")]
    pub fn has_gamepad_id(&self, id: gilrs::GamepadId) -> bool {
        match self {
            Self::Gamepad { id: SpecifyGamepad::Id(cid), .. } => *cid == id,
            Self::Gamepad { id: SpecifyGamepad::Any,     .. } => true,
            _ => false
        }
    }
    #[cfg(feature = "gamepad")]
    #[allow(irrefutable_let_patterns)]
    /// sets the gamepad id. if its a device it does nothing.
    pub fn set_gamepad_id(self, id: gilrs::GamepadId) -> Self {
        if let Self::Gamepad { input, .. } = self { input.with_id(id) }
        else { self }
    }
    #[cfg(feature = "mice-keyboard")]
    #[allow(irrefutable_let_patterns)]
    /// sets the device id. if its a gamepad it does nothing.
    pub fn set_device_id(self, id: winit::event::DeviceId) -> Self {
        if let Self::Device { input, .. } = self { input.with_id(id) }
        else { self }
    }
}
/// imports everything needed to reduce boilerplate when creating an input_map
pub mod base_input_codes {
    #![allow(ambiguous_glob_reexports)]
    use crate::input_code::*;

    #[cfg(feature = "gamepad")]
    pub use gamepad::GamepadInput::{*, self};

    #[cfg(feature = "mice-keyboard")]
    pub use mice_keyboard::DeviceInput::{*, self};

    #[cfg(feature = "mice-keyboard")]
    pub use winit::{
        keyboard::{KeyCode::{*, self}, PhysicalKey::{*, self}},
        event::MouseButton
    };
}

#[cfg(feature = "mice-keyboard")]
pub use mice_keyboard::*;
#[cfg(feature = "mice-keyboard")]
mod mice_keyboard {
    use winit::keyboard::{ KeyCode, PhysicalKey };
    use winit::event::*;
    use crate::InputCode;
    #[cfg(feature = "mice-keyboard")]
    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub enum DeviceInput {
        Button(MouseButton),
        Key(PhysicalKey),
        MouseMoveLeft,
        MouseMoveRight,
        MouseMoveUp,
        MouseMoveDown,
        MouseScrollUp,
        MouseScrollDown,
        MouseScrollLeft,
        MouseScrollRight,
    }
    #[cfg(feature = "mice-keyboard")]
    impl DeviceInput {
        pub fn with_id(self, id: DeviceId) -> InputCode {
            InputCode::Device { id: SpecifyDevice::Id(id), input: self }
        }
        pub fn with_sid(self, id: SpecifyDevice) -> InputCode {
            InputCode::Device { id, input: self }
        }
    }
    #[cfg(feature = "mice-keyboard")]
    impl From<MouseButton> for DeviceInput {
        fn from(value: MouseButton) -> Self {
            Self::Button(value) 
        }
    }
    #[cfg(feature = "mice-keyboard")]
    impl From<KeyCode> for DeviceInput {
        fn from(value: KeyCode) -> Self {
            Self::Key(value.into()) 
        }
    }
    #[cfg(feature = "mice-keyboard")]
    impl From<PhysicalKey> for DeviceInput {
        fn from(value: PhysicalKey) -> Self {
            Self::Key(value)
        }
    }
    /// specify device to listen to. defaults to any and can be specified later on at runtime
    #[cfg(feature = "mice-keyboard")]
    #[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub enum SpecifyDevice {
        /// cant be set at compile time. use `Any` as default and then let the user select a specific
        /// gamepad at runtime
        Id(DeviceId),
        /// use as default
        #[default]
        Any
    }
    impl From<DeviceInput> for InputCode {
        fn from(value: DeviceInput) -> Self {
            Self::Device { id: SpecifyDevice::Any, input: value }
        }
    }
    impl From<MouseButton> for InputCode {
        fn from(value: MouseButton) -> Self {
            Self::Device { id: SpecifyDevice::Any, input: value.into() }
        }
    }
    impl From<PhysicalKey> for InputCode {
        fn from(value: PhysicalKey) -> Self {
            Self::Device { id: SpecifyDevice::Any, input: value.into() }
        }
    }
    impl From<KeyCode> for InputCode {
        fn from(value: KeyCode) -> Self {
            Self::Device { id: SpecifyDevice::Any, input: value.into() }
        }
    }
}


#[cfg(feature = "gamepad")]
pub use gamepad::*;
#[cfg(feature = "gamepad")]
mod gamepad {
    use crate::InputCode;
    use gilrs::{Axis, Button};
    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub enum GamepadInput {
        LeftStickLeft,
        /// the left stick, moved to the right
        LeftStickRight,
        LeftStickUp,
        LeftStickDown,
        LeftStickPress,

        /// the right stick, moved to the left
        RightStickLeft,
        RightStickRight,
        RightStickUp,
        RightStickDown,
        RightStickPress,

        DPadLeft,
        DPadRight,
        DPadUp,
        DPadDown,

        LeftZ,
        RightZ,

        South,
        East,
        North,
        West,

        LeftBumper,
        LeftTrigger,
        RightBumper,
        RightTrigger,

        Select,
        Start,
        Mode,
        /// unfortunately gilrs doesnt give enough infomation to have multiple 'Other' input binds
        Other
    }
    impl GamepadInput {
        pub fn with_id(self, id: gilrs::GamepadId) -> InputCode {
            InputCode::Gamepad { id: SpecifyGamepad::Id(id), input: self }
        }
        pub fn with_sid(self, id: SpecifyGamepad) -> InputCode {
            InputCode::Gamepad { id, input: self }
        }
    }
    pub fn axis_neg(axis: Axis) -> GamepadInput {
        match axis {
            Axis::LeftStickX => GamepadInput::LeftStickLeft,
            Axis::LeftStickY => GamepadInput::LeftStickDown,
            Axis::RightStickX => GamepadInput::RightStickLeft,
            Axis::RightStickY => GamepadInput::RightStickDown,
            Axis::LeftZ => GamepadInput::LeftZ,
            Axis::RightZ => GamepadInput::RightZ,
            Axis::DPadX => GamepadInput::DPadLeft,
            Axis::DPadY => GamepadInput::DPadDown,
            Axis::Unknown => GamepadInput::Other
        }
    }
    pub fn axis_pos(axis: Axis) -> GamepadInput {
        match axis {
            Axis::LeftStickX => GamepadInput::LeftStickRight,
            Axis::LeftStickY => GamepadInput::LeftStickUp,
            Axis::RightStickX => GamepadInput::RightStickRight,
            Axis::RightStickY => GamepadInput::RightStickUp,
            Axis::LeftZ => GamepadInput::LeftZ,
            Axis::RightZ => GamepadInput::RightZ,
            Axis::DPadX => GamepadInput::DPadRight,
            Axis::DPadY => GamepadInput::DPadUp,
            Axis::Unknown => GamepadInput::Other,
        }
    }
    impl From<Button> for GamepadInput {
        fn from(value: Button) -> Self {
            match value {
                Button::South => GamepadInput::South,
                Button::East => GamepadInput::East,
                Button::North => GamepadInput::North,
                Button::West => GamepadInput::West,
                Button::LeftTrigger => GamepadInput::LeftBumper,
                Button::LeftTrigger2 => GamepadInput::LeftTrigger,
                Button::RightTrigger2 => GamepadInput::RightTrigger,
                Button::RightTrigger => GamepadInput::RightBumper,
                Button::DPadUp => GamepadInput::DPadUp,
                Button::DPadDown => GamepadInput::DPadDown,
                Button::DPadLeft => GamepadInput::DPadLeft,
                Button::DPadRight => GamepadInput::DPadRight,
                Button::Z => GamepadInput::RightZ,
                Button::C => GamepadInput::LeftZ,
                Button::Select => GamepadInput::Select,
                Button::Start => GamepadInput::Start,
                Button::Mode => GamepadInput::Mode,
                Button::RightThumb => GamepadInput::RightStickPress,
                Button::LeftThumb  => GamepadInput::LeftStickPress,
                Button::Unknown => GamepadInput::Other 
            }
        }
    }
    impl From<GamepadInput> for InputCode {
        fn from(value: GamepadInput) -> InputCode {
            Self::Gamepad { input: value, id: Default::default() }
        }
    }
    /// Specify gamepad to listen to. defaults to any and can be specified later on at runtime
    #[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Hash)]
    pub enum SpecifyGamepad {
        /// cant be set at compile time. use `Any` as default and then let the user select a specific
        /// gamepad at runtime
        Id(gilrs::GamepadId),
        /// use as default
        #[default]
        Any
    }
}
