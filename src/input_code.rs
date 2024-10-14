use winit::keyboard::{ KeyCode, PhysicalKey };
use winit::event::*;
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
/// Enum that specifies an input
pub enum InputCode {
    Device { id: SpecifyDevice, input: DeviceInput },
    #[cfg(feature = "gamepad")]
    Gamepad { id: SpecifyGamepad, input: GamepadInput }
}
impl InputCode {
    pub const MOUSE_MOVE_X_POS: Self = Self::Device {
        input: DeviceInput::MouseMoveX(AxisSign::Pos),
        id: SpecifyDevice::Any
    };
    pub const MOUSE_MOVE_X_NEG: Self = Self::Device {
        input: DeviceInput::MouseMoveX(AxisSign::Neg),
        id: SpecifyDevice::Any
    };
    pub const MOUSE_MOVE_Y_POS: Self = Self::Device {
        input: DeviceInput::MouseMoveY(AxisSign::Pos),
        id: SpecifyDevice::Any
    };
    pub const MOUSE_MOVE_Y_NEG: Self = Self::Device {
        input: DeviceInput::MouseMoveY(AxisSign::Neg),
        id: SpecifyDevice::Any
    };
    pub const MOUSE_SCROLL_X_POS: Self = Self::Device {
        input: DeviceInput::MouseScrollX(AxisSign::Pos),
        id: SpecifyDevice::Any
    };
    pub const MOUSE_SCROLL_X_NEG: Self = Self::Device {
        input: DeviceInput::MouseScrollX(AxisSign::Neg),
        id: SpecifyDevice::Any
    };
    pub const MOUSE_SCROLL_POS: Self = Self::Device {
        input: DeviceInput::MouseScroll(AxisSign::Pos),
        id: SpecifyDevice::Any
    };
    pub const MOUSE_SCROLL_NEG: Self = Self::Device {
        input: DeviceInput::MouseScroll(AxisSign::Neg),
        id: SpecifyDevice::Any
    };
    #[cfg(feature = "gamepad")]
    pub fn gamepad_axis_pos(axis: gilrs::Axis) -> Self {
        GamepadInput::Axis(axis, AxisSign::Pos).into()
    }
    #[cfg(feature = "gamepad")]
    pub fn gamepad_axis_neg(axis: gilrs::Axis) -> Self {
        GamepadInput::Axis(axis, AxisSign::Neg).into()
    }
    /// sets `SpecifyGamepad` or `SpecifyDevice` to any
    pub fn set_any(self) -> Self {
        match self {
            #[cfg(feature = "gamepad")]
            Self::Gamepad { input, .. } => input.into(),
            Self::Device  { input, .. } => input.into(),
        }
    }
    #[cfg(feature = "gamepad")]
    /// sets the gamepad id. if its a device it does nothing.
    pub fn set_gamepad_id(self, id: gilrs::GamepadId) -> Self {
        if let Self::Gamepad { input, .. } = self { input.with_id(id) }
        else { self }
    }
    #[allow(irrefutable_let_patterns)]
    /// sets the device id. if its a gamepad it does nothing.
    pub fn set_device_id(self, id: DeviceId) -> Self {
        if let Self::Device { input, .. } = self { input.with_id(id) }
        else { self }
    }
    pub fn set_axis_sign(self, sign: AxisSign) -> Self {
        match self {
            Self::Device { id, input } => match input {
                DeviceInput::MouseMoveX(_)   => DeviceInput::MouseMoveX(sign)  .with_sid(id),
                DeviceInput::MouseMoveY(_)   => DeviceInput::MouseMoveY(sign)  .with_sid(id),
                DeviceInput::MouseScroll(_)  => DeviceInput::MouseScroll(sign) .with_sid(id),
                DeviceInput::MouseScrollX(_) => DeviceInput::MouseScrollX(sign).with_sid(id),
                result =>                       result                         .with_sid(id)
            },
            #[cfg(feature = "gamepad")]
            Self::Gamepad { id, input } => match input {
                GamepadInput::Axis(axis, _) => GamepadInput::Axis(axis, sign).with_sid(id),
                result =>                      result                        .with_sid(id)
            }
        }
    }

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
/*impl<F> From<F> for InputCode
where F: Into<DeviceInput> + Sized {
    fn from(value: F) -> Self {
        std::convert::Into::<DeviceInput>::into(value).into()
    }
}*/
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DeviceInput {
    Button(MouseButton),
    Key(PhysicalKey),
    MouseMoveX(AxisSign),
    MouseMoveY(AxisSign),
    MouseScroll(AxisSign),
    /// axis for left and right mouse scroll. most mice cant do this but it is common with
    /// touchpads
    MouseScrollX(AxisSign),
}
impl DeviceInput {
    pub fn with_id(self, id: DeviceId) -> InputCode {
        InputCode::Device { id: SpecifyDevice::Id(id), input: self }
    }
    pub fn with_sid(self, id: SpecifyDevice) -> InputCode {
        InputCode::Device { id, input: self }
    }
}
impl From<MouseButton> for DeviceInput {
    fn from(value: MouseButton) -> Self {
        Self::Button(value) 
    }
}
impl From<KeyCode> for DeviceInput {
    fn from(value: KeyCode) -> Self {
        Self::Key(value.into()) 
    }
}
impl From<PhysicalKey> for DeviceInput {
    fn from(value: PhysicalKey) -> Self {
        Self::Key(value)
    }
}
/// specify device to listen to. defaults to any and can be specified later on at runtime
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum SpecifyDevice {
    /// cant be set at compile time. use `Any` as default and then let the user select a specific
    /// gamepad at runtime
    Id(DeviceId),
    /// use as default
    #[default]
    Any
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum AxisSign { Pos, Neg }
#[cfg(feature = "gamepad")]
pub use gamepad::*;
#[cfg(feature = "gamepad")]
mod gamepad {
    use crate::InputCode;
    pub type GamepadAxis = gilrs::Axis;
    pub type GamepadButton = gilrs::Button;
    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub enum GamepadInput {
        Button(GamepadButton),
        Axis(GamepadAxis, crate::AxisSign)
    }
    impl GamepadInput {
        pub fn with_id(self, id: gilrs::GamepadId) -> InputCode {
            InputCode::Gamepad { id: SpecifyGamepad::Id(id), input: self }
        }
        pub fn with_sid(self, id: SpecifyGamepad) -> InputCode {
            InputCode::Gamepad { id, input: self }
        }
    }
    impl From<GamepadButton> for GamepadInput {
        fn from(value: gilrs::Button) -> GamepadInput {
            GamepadInput::Button(value)
        }
    }
    impl From<GamepadInput> for InputCode {
        fn from(value: GamepadInput) -> InputCode {
            Self::Gamepad { input: value, id: Default::default() }
        }
    }
    impl From<GamepadButton> for InputCode {
        fn from(value: gilrs::Button) -> InputCode {
            Self::Gamepad { input: GamepadInput::Button(value), id: Default::default() }
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
