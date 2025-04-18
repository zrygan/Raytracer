# Raytracer

By: Zhean Ganituen (`zrygan`)  
Started: February 2025  
Last updated: April 2025  
Version: 1.0

## About

A simple two-dimensional ray tracing simulation application that visualizes light ray behavior through various optical elements.

## Getting Started

### Building the Rust Version

1. Clone the repository:
   ```bash
   git clone https://github.com/zrygan/Raytracer.git
   ```

2. Navigate to the project directory:
   ```bash
   cd Raytracer
   ```

3. Build and run the project:
   ```bash
   cargo run
   ```

   Alternatively, you can build without running:
   ```bash
   cargo build
   ```

### Using the Legacy Python Version

1. Clone the repository if you haven't already:
   ```bash
   git clone https://github.com/zrygan/Raytracer.git
   ```

2. Navigate to the Python directory:
   ```bash
   cd Raytracer/python
   ```

3. (Optional) Create a Python virtual environment:
   ```bash
   python -m venv venv
   venv/Scripts/activate
   ```

4. Install the required dependencies:
   ```bash
   pip install -r requirements.txt
   ```

5. Run the application:
   ```bash
   python main.py
   ```

## Controls

### Keybinds

The complete list of keybinds is available in [`src/globals.rs`](src/globals.rs). All constants prefixed with `KEYB_` are keybinds.

All objects are created at the cursor position with default parameters. Default parameters are defined in `globals.rs` as constants prefixed with `OBJC_`.

| Key | Action |
|-----|--------|
| `o` | Create a simple circle object |
| `i` | Create an isotropic emitter |
| `c` | Create a collimated emitter |
| `s` | Create a spotlight emitter |
| `p` | Create a perfect absorber |
| `backspace` | Delete object at cursor position |
| `\` | Debug tool: show all objects in scene |

## Features

### Light Emitters
- **Isotropic**: Emits light in all directions
- **Collimated**: Emits parallel light rays
- **Spotlight**: Emits a focused beam of light

### Objects
- **Circle**: Basic circular object
- **Perfect Absorber**: Fully opaque object that absorbs all light

## Requirements

This project requires:
- [Rust](https://www.rust-lang.org/tools/install) (for the main version)
- [Python 3.x](https://www.python.org/downloads/) (for the legacy version)

### Dependencies
- [Macroquad](https://macroquad.rs/) - Fast and simple game library for Rust
- [once_cell](https://docs.rs/once_cell/latest/once_cell/) - Single-assignment cells for Rust

All Rust dependencies will be automatically downloaded when building with Cargo.

## Documentation

### Online Documentation

You can access the documentation at:
- [GitHub Pages](https://zrygan.github.io/Raytracer/)
- [`docs/` directory](https://github.com/zrygan/Raytracer/tree/rust-rewrite/docs)

### Building the Documentation Locally

Generate the code documentation:

```bash
git clone https://github.com/zrygan/Raytracer.git
cd Raytracer
cargo doc --no-deps
```

Then open `target/doc/raytracer/index.html` in your browser.

## License

This project is licensed under the [MIT License](LICENSE).