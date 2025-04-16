# Raytracer

By: Zhean Ganituen (zrygan). <br>
Started: February 2025 <br>
Last updated: April 2025 <br>

>❗**Notice** <br> <br>
> This project is currently being rewritten from Python to Rust using [Macroquad](https://macroquad.rs/). <br>
> The Python version is unmaintained as of April 2025 (however, working properly). <br>
> The Rust version is in progress (see the [Rust Rewrite tree](https://github.com/zrygan/Raytracer/tree/rust-rewrite/src). <br>

## About

A simple two-dimensional ray tracing application.

## Requirements

* [PyGame](https://www.pygame.org/)
* [Numpy](https://numpy.org/)

To install, simply:
```
$ pip install -r requirements.txt
```

## Features
* Light Emitters:
    * emitter point
    * Directional Light
    * Spot Light
* Absorbers and Shadowing
* Reflectors*
* Refractors*
* Diffractors*
* Semi-reflectivity**
* Variable opacity**
* Light intensity heat map**

*Note.* Ones with the star (*) are upcoming features. Ones with the double star (**) are very future features.

## How To
Simply install the requirements and run the `main.py` python file:
```
python main.py
```

Then, a Pygame window will appear and you may use the following commands:
1. `o` : creates a emitter point emittor at cursor position.
2. `d` : creates a directional emittor at cursor position.
3. `s` : creates a spot light emittor at cursor position.
4. `a` : creates a circle light absorber at the cursor position.
5. `,` : increments the directional angle of the directional and spot emitters at the cursor position.
6. `.` : decrements the directional angle of the directional and spot emitters at the cursor position.
7. `backspace` : deletes the object at the cursor position.
 
You can move an object by holding left click.
