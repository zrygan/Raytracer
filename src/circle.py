import pygame

class Circle:
    """
    A class to represent a circle.
    """

    def __init__(self, radius, x, y, color_fill=None):
        self.radius = radius
        self.x = x
        self.y = y
        self.color_fill = color_fill
    def draw(self, screen):
        pygame.draw.circle(screen, self.color_fill, (self.x, self.y), self.radius)
    def move(self, x, y):
        self.x = x
        self.y = y
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
