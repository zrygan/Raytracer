import pygame
from typing import Tuple
from src.variables import *
from src.variables import WHITE


class Ray:
    def __init__(
        self,
        x_start: int,
        y_start: int,
        angle: float,
        emitter_object: object,
        x_end: int = None,
        y_end: int = None,
        color=None,
    ):
        self.x_start = x_start
        self.y_start = y_start
        self.angle = angle
        self.x_end = (
            x_end
            if x_end is not None
            else self.x_start + DEFAULT_RAY_MAX_LENGTH * self.angle[0]
        )
        self.y_end = (
            y_end
            if y_end is not None
            else self.y_start + DEFAULT_RAY_MAX_LENGTH * self.angle[1]
        )
        self.color = color if color is not None else WHITE
        self.emitter = emitter_object

        RAYS[self] = self.emitter

    def draw(self, screen):
        pygame.draw.line(
            screen,
            self.color,
            (self.x_start, self.y_start),
            (self.x_end, self.y_end),
        )

    def change_angle(self, angle):
        self.angle = angle

    def change_color(self, color):
        self.color = color

    def get_angle(self):
        return self.angle

    def get_x_start(self):
        return self.x_start

    def get_x_end(self):
        return self.x_end

    def get_y_start(self):
        return self.y_start

    def get_y_end(self):
        return self.y_end

    def set_x(self, x_start: int = None, x_end: int = None):
        self.x_start = x_start if x_start is not None else self.x_start
        self.x_end = x_end if x_end is not None else self.x_end

    def set_y(self, y_start: int = None, y_end: int = None):
        self.y_start = y_start if y_start is not None else self.y_start
        self.y_end = y_end if y_end is not None else self.y_end
