# Raytracer

By: Zhean Ganituen (zrygan). <br>
Started: February 2025 <br>
Last updated: March 2025 <br>

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
    * Point Light
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
1. `o` : creates a point light emittor at cursor position.
2. `d` : creates a directional emittor at cursor position.
3. `s` : creates a spot light emittor at cursor position.
4. `a` : creates a circle light absorber at the cursor position.
5. `,` : increments the directional angle of the directional and spot emitters at the cursor position.
6. `.` : decrements the directional angle of the directional and spot emitters at the cursor position.
7. `backspace` : deletes the object at the cursor position.
 
You can move an object by holding left click.
