0.4.2
Added changelog
Made mice and keyboard input (and thus winit) optional
Updated to latest winit version
Added base_input_codes module to reduce boilerplate when setting up an input map
Made input bound to the same action not intefere with eachother
removed consts and axis to simplify enums
abstracted gilrs axis and buttons because of overlap
Made disconecting a gamepad stop its input
Updated documentation and readme

0.5.0
fixed mouse scroll sometimes not working
changed default mouse scale
fixed multiple devices/gamepads intefearing with any devices/gamepads binds
fixed specified device input

0.6.0
fixed Joystick X and MouseMove being inverted
changed Trigger to bumpers and Trigger2 to Trigger
changed action_val to value
updated to latest winit version
added multiple bind actions, allowing for shortcuts. works with variable input
removed unused AxisSign enum
added `set_binds`, `add_binds` and `get_binds`
removed `get_mut_binds`
fixed gamepad input from being detected while the window is unfoccused so its consistant with keyboard input
