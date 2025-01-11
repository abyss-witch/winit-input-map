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
