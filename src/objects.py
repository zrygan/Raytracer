import pygame, math, numpy as np
from typing import Tuple
from src.util import *
from src.variables import *
from src.rays import Ray
from src.shadowing import *


class Circle:
    """
    A class to represent a circle.

    Note. No need to check if an attribute is none
    since it will be handled by the subclasses.
    """

    def __init__(
        self,
        radius: int,
        x: int,
        y: int,
        fill_color: Tuple[int, int, int],
        penetrable: bool = None,
    ):
        self.radius = radius
        self.x = x
        self.y = y
        self.fill_color = fill_color if fill_color is not None else WHITE
        self.penetrable = penetrable if penetrable is not None else False

        if self.penetrable:
            check_shadows()

    def draw(self, SCREEN):
        # the circle function here is a pygame function
        # different from the Circle class here
        pygame.draw.circle(SCREEN, self.fill_color, (self.x, self.y), self.radius)

    def move(self, x, y):
        self.x = x
        self.y = y

        if self.get_penetrable():
            for circle in EMMITERS:
                circle.initialize_rays()
            check_shadows()  # Use the new shadow checking function

    def change_color(self, fill_color):
        self.fill_color = fill_color

    def change_radius(self, radius):
        self.radius = radius

    def get_radius(self):
        return self.radius

    def get_x(self):
        return self.x

    def get_y(self):
        return self.y

    def get_color(self):
        return self.fill_color

    def get_rays(self):
        return self.rays

    def get_penetrable(self):
        return self.penetrable


class Emitter_Point(Circle):
    def __init__(
        self,
        x: int,
        y: int,
        radius: int = None,
        emitter_color: Tuple[int, int, int] = None,
        fill_color: Tuple[int, int, int] = None,
    ):
        # Attributes of the point emitter
        self.rays = []
        self.emitter_color = (
            emitter_color if emitter_color is not None else CORNFLOWER_BLUE
        )

        # Attributes of the super (Circle) class
        fill_color = fill_color if fill_color is not None else WHITE
        radius = radius if radius is not None else DEFAULT_CIRCLE_RADIUS
        x = x
        y = y
        super().__init__(radius, x, y, fill_color)

        self.initialize_rays()

    def initialize_rays(self):
        for i in range(DEFAULT_RAY_COUNT):
            theta = (i * 2 * math.pi) / DEFAULT_RAY_COUNT
            theta_vector = (math.cos(theta), math.sin(theta))
            self.rays.append(
                Ray(self.x, self.y, theta_vector, self, color=self.emitter_color)
            )

    def move(self, x, y):
        self.x = x
        self.y = y
        self.rays = []
        self.initialize_rays()

        if self.get_penetrable():
            check_shadows(self)


class Emitter_Directional(Circle):
    def __init__(
        self,
        x: int,
        y: int,
        angle: float,
        width: float = None,
        radius: int = None,
        emitter_color: Tuple[int, int, int] = None,
        fill_color: Tuple[int, int, int] = None,
    ):
        # Attribute of super (Circle) class but this needs to be at the top
        # since width may depend on it
        self.radius = radius if radius is not None else DEFAULT_CIRCLE_RADIUS

        # Attribubtes of the directional emitter
        self.rays = []
        self.angle = angle
        self.width = width if width is not None else self.radius * 2
        self.emitter_color = (
            emitter_color if emitter_color is not None else CORNFLOWER_BLUE
        )

        # Attrbutes of the super (Circle) class
        radius = radius if radius is not None else DEFAULT_CIRCLE_RADIUS
        fill_color = fill_color if fill_color is not None else WHITE
        x = x
        y = y
        super().__init__(radius, x, y, fill_color)

        self.initialize_rays(self.x, self.y)

    def initialize_rays(self, x: int = None, y: int = None):
        x = x if x is not None else self.x
        y = y if y is not None else self.y
        self.rays = []

        theta_vector = (
            math.cos(self.angle),
            -math.sin(self.angle),
        )

        perp_vector = (-theta_vector[1], theta_vector[0])

        spacing = self.width / (DEFAULT_RAY_COUNT - 1) if DEFAULT_RAY_COUNT > 1 else 0

        for i in range(DEFAULT_RAY_COUNT):
            offset = (i - (DEFAULT_RAY_COUNT - 1) / 2) * spacing

            ray_position = (
                x + offset * perp_vector[0],
                y + offset * perp_vector[1],
            )

            self.rays.append(
                Ray(
                    ray_position[0],
                    ray_position[1],
                    theta_vector,
                    self,
                    color=self.emitter_color,
                )
            )

    def move(self, x, y):
        # this is a method overriding
        self.x = x
        self.y = y
        self.rays = []
        self.initialize_rays(x, y)

        if self.get_penetrable():
            check_shadows(self)

    def get_angle(self):
        return self.angle

    def get_width(self):
        return self.width

    def set_angle(self, angle):
        self.angle = angle

    def set_width(self, width):
        self.width = width


class Emitter_Spot(Circle):
    def __init__(
        self,
        x: int,
        y: int,
        angle: float,
        arc: float,
        radius: int = None,
        emitter_color: Tuple[int, int, int] = None,
        fill_color: Tuple[int, int, int] = None,
    ):
        # Attributes of the spot emitter
        self.rays = []
        self.angle = angle
        self.arc = arc
        self.emitter_color = (
            emitter_color if emitter_color is not None else CORNFLOWER_BLUE
        )

        # Attributes of the super (Circle) class
        fill_color = fill_color if fill_color is not None else WHITE
        radius = radius if radius is not None else DEFAULT_CIRCLE_RADIUS
        x = x
        y = y
        super().__init__(radius, x, y, fill_color)

        self.initialize_rays()

    def initialize_rays(self):
        self.rays = []

        angles = np.linspace(self.angle, self.angle + self.arc, DEFAULT_RAY_COUNT)

        for angle in angles:
            theta_vector = (math.cos(angle), -math.sin(angle))
            ray_position = (
                self.x + self.radius * np.cos(angle),
                self.y + self.radius * np.sin(angle),
            )

            self.rays.append(
                Ray(
                    ray_position[0],
                    ray_position[1],
                    theta_vector,
                    self,
                    color=self.emitter_color,
                )
            )

    def move(self, x, y):
        self.x = x
        self.y = y
        self.rays = []
        self.initialize_rays()

        if self.get_penetrable():
            check_shadows(self)

    def get_angle(self):
        return self.angle

    def get_arc(self):
        return self.arc

    def set_angle(self, angle):
        self.angle = angle

    def set_arc(self, arc):
        self.arc = arc


class Absorber_Circle(Circle):
    def __init__(
        self,
        x: int,
        y: int,
        radius: int = None,
        fill_color: Tuple[int, int, int] = None,
    ):
        radius = radius if radius is not None else DEFAULT_CIRCLE_RADIUS
        fill_color = fill_color if fill_color is not None else WHITE

        super().__init__(radius, x, y, fill_color, penetrable=True)


# TODO: this class and the Line class
class Absorber_Line(
    # Line
):
    def __init__(self):
        pass


class Reflector_Circle(Circle):
    def __init__():
        pass


class Refractor(Circle):
    def __init__():
        pass


class Diffractor(Circle):
    def __init__():
        pass
