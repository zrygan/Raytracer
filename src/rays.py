import pygame
import math
from src.variables import WHITE


class Ray:
    def __init__(self, x, y, angle, color=None):
        self.x = x
        self.y = y
        self.angle = angle
        self.color = color if color is not None else WHITE

    def draw(self, screen):
        pygame.draw.line(
            screen,
            self.color,
            (self.x, self.y),
            (self.x + 10000 * self.angle[0], self.y + 10000 * self.angle[1]),
        )

    def change_angle(self, angle):
        self.angle = angle

    def change_color(self, color):
        self.color = color

    def get_angle(self):
        return self.angle

    def get_x(self):
        return self.x

    def get_y(self):
        return self.y

    def change_x(self, x):
        self.x = x

    def change_y(self, y):
        self.y = y
