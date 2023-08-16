# Basic Control System for the Open EMS prototype

[Link to project](https://github.com/cfoge/OPEN_SPECTRE-
)

# Crates
This project uses [nannou](https://crates.io/crates/nannou) for the graphics, [serial2](https://crates.io/crates/serial2) for the serial connection and [hecs](https://crates.io/crates/hecs) for the entity component system.
# Running
To run the project, clone the repo and use:

`cargo run --release`

you can also specify what serial port to try and connect to by passing it as an argument:

`cargo run --release <serial_port_path>`

On Unix systems, the name parameter must be a path to a TTY device. On Windows, it must be the name of a COM device, such as COM1, COM2, etc.

On Windows, for COM ports above COM9, you need to use the win32 device namespace for the name parameter. For example “\.\COM10” (or “\\.\COM10” with string escaping). For more details, [see the documentation from Microsoft.](https://learn.microsoft.com/en-us/windows/win32/fileio/naming-a-file?redirectedfrom=MSDN#win32-device-namespaces)





for example:

`cargo run --release /dev/tty`

or

`cargo run --release \.\COM10`



