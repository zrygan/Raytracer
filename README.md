# Raytracer

By: Zhean Ganituen (zrygan). <br>
Started: February 2025 <br>
Last updated: April 2025 <br>

>❗**Notice** <br> <br>
> This project is currently being rewritten from Python to Rust using [Macroquad](https://macroquad.rs/). <br>
> The Python version is unmaintained as of April 2025 (however, working properly). <br>
> The Rust version is in progress (see the [Rust Rewrite tree](https://github.com/zrygan/Raytracer/tree/rust-rewrite/src). <br>

## About

A simple two-dimensional ray tracing simulation application.

## How To 
### Building your own Raytracer
Simply clone the repository:

```bash
$ git clone https://github.com/zrygan/Raytracer.git
```

Then, go to the directory and compile the project:

```bash
$ cd Raytracer

$ cargo build # you cal also do `$cargo run` to build and run
```

### Keybinds

> The complete list of keybinds is available in [`globals.rs`](https://github.com/zrygan/Raytracer/blob/rust-rewrite/src/globals.rs), all the constants that are prefixed with `KEYB_` are keybinds.

All objects created with keybinds will get default parameters.

> The complete list of default parameters is also in `globals.rs`, see all constants prefixed with `OBJC_`. **Future**: we may implement a way to have more options when adding Raytracer objects.

Also, all objects will be created in the cursor position.

1. `o` : creates a simple circle object.
2. `i` : creates an isotropic emitter object.
3. `c` : creates a collimated emitter object.
4. `s` : created a spotlight emitter object.
5. `backspace` ; delete the object on the cursor.
6. `\` : `debug tool`, show all objects on the scene
 

## Requirements

* [Macroquad](https://macroquad.rs/)
* [once_cell](https://docs.rs/once_cell/latest/once_cell/)

> (*If this is not updated*) you may visit the (`Cargo.toml`)[https://github.com/zrygan/Raytracer/blob/rust-rewrite/Cargo.toml] file for the complete list of dependencies.

There is no need to install these dependencies one-by-one since running the project (for the first time) would also download the dependencies.

## Features
* Light Emitters:
    * Isometric
    * Collimated

## Documentation

### Web Documentation

> Webpage removed, might re-add at a later time.

You may go to the [`docs/`](https://github.com/zrygan/Raytracer/tree/rust-rewrite/docs) directory in the repository.

Alternatively (the easy way), you may visit the [Github pages](https://zrygan.github.io/Raytracer/) of this repository.

### Building the Documentation

You may build the code documentation by:

1. Clone the repository

```bash
$ git clone https://github.com/zrygan/Raytracer.git
```

2. Go to the Raytracer directory

```bash
$ cd Raytracer
```

3. Create the `rustdoc`.

```bash
$ cargo doc --no-deps # --no-deps removes documentation for Raytracer's dependencies, remove this add needed.
```