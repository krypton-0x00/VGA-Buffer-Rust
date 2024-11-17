# Bare Metal VGA Buffer Implementation in Rust

A minimal, no-std Rust implementation of VGA buffer text mode for bare metal systems. This project demonstrates low-level system programming by directly interfacing with VGA hardware to display text on screen.

## Features

- Direct VGA memory manipulation (0xb8000)
- 16-color support with foreground and background colors
- ASCII character display
- Basic text output functionality
- No standard library dependencies

## Technical Details

### VGA Colors

The implementation supports all 16 standard VGA colors:
- Basic Colors: Black, Blue, Green, Cyan, Red, Magenta, Brown, Light Gray
- Light Colors: Dark Gray, Light Blue, Light Green, Light Cyan, Light Red, Pink, Yellow, White

### Memory Layout

- Uses VGA text buffer at memory address 0xb8000
- Buffer dimensions: 80x25 characters
- Each character cell consists of:
  - ASCII character (1 byte)
  - Color code (1 byte)
    - Lower 4 bits: Foreground color
    - Upper 4 bits: Background color

## Project Structure

```
.
├── src
│   ├── main.rs           # Entry point and panic handler
│   └── vga_buffer.rs     # VGA buffer implementation
```

### Core Components

- `Color`: Enum representing VGA colors
- `ColorCode`: Wrapper for color attributes
- `ScreenChar`: Character cell representation
- `Buffer`: VGA buffer representation
- `Writer`: Text output handling

## Usage

To write text to the screen:

```rust
let mut writer = Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::Yellow, Color::Black),
    buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
};

writer.write_string("Hello World");
```

## Building

This is a no-std project intended for bare metal environments. To build:

1. Install Rust nightly:
```bash
rustup toolchain install nightly
```

2. Add target for bare metal development:
```bash
rustup target add x86_64-unknown-none
```

3. Build the project:
```bash
cargo build --target x86_64-unknown-none
```

## Safety Considerations

This implementation includes unsafe code for direct memory access. The VGA buffer address (0xb8000) is accessed through raw pointer manipulation. This is necessary for bare metal programming but should be used with caution.

## Future Improvements

- Implement scrolling functionality
- Add support for special characters
- Implement proper new line handling
- Add cursor support
- Add support for text attributes (bold, blink, etc.)

## License

This project is open source and available under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
