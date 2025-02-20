import pygame
import math
from src.variables import N_RAYS, WHITE
from src.rays import Ray


class Circle:
    """
    A class to represent a circle.
    """

    def __init__(self, radius, x, y, color_fill=None, type=None):
        self.radius = radius
        self.x = x
        self.y = y
        self.color_fill = color_fill if color_fill is not None else WHITE
        self.type = type if type is not None else "POINT"
        self.rays = []  # array of rays, see rays.py

        # the rays of the circle is determined by the type of light source
        if self.type == "POINT":
            # emits light in all directions
            # default type if no type is specified
            for i in range(N_RAYS):
                theta = (i * 2 * math.pi) / N_RAYS
                theta_vector = (math.cos(theta), math.sin(theta))
                self.rays.append(Ray(self.x, self.y, theta_vector))

        elif self.type == "DIRECTIONAL":
            # emits light in a parallel light
            pass
        elif self.type == "SPOT":
            # emits light in a cone
            pass

    def draw(self, screen):
        pygame.draw.circle(screen, self.color_fill, (self.x, self.y), self.radius)

    def move(self, x, y):
        self.x = x
        self.y = y
        for ray in self.rays:
            ray.change_x(x)
            ray.change_y(y)

    def change_color(self, color_fill):
        self.color_fill = color_fill

    def change_radius(self, radius):
        self.radius = radius

    def get_radius(self):
        return self.radius

    def get_x(self):
        return self.x

    def get_y(self):
        return self.y

    def get_color(self):
        return self.color_fill

    def get_rays(self):
        return self.rays
