import pygame
import math, numpy as np
from src.variables import (
    N_RAYS,
    WHITE,
    X_SIZE,
    Y_SIZE,
    CIRCLE_DEFAULT_RADIUS,
    DIRECTIONAL_ANGLE_OFFSET,
)
from src.rays import Ray


class Circle:
    """
    A class to represent a circle.
    """

    def __init__(
        self,
        radius,
        x,
        y,
        object_type,
        color_fill=None,
        emitter_type=None,  # variables if object_type is EMITTER
        emit_color=None,
        directional_angle=None,  # variables if object_type is DIRECTIONAL
        directional_width=None,
        spot_angle=None,  # variables if object_type is SPOT
        spot_arc=None,
    ):
        self.radius = radius
        self.x = x
        self.y = y
        self.color_fill = color_fill if color_fill is not None else WHITE
        self.rays = None  # default array of rays as none
        self.object_type = (
            object_type if object_type is not None else "ABSORBER"
        )  # so by default, the circle is an ABSORBER

        # variables if object is an emitter
        self.emitter_type = emitter_type if emitter_type is not None else "POINT"
        self.emit_color = emit_color if emit_color is not None else WHITE

        # Object type is emitter
        if object_type == "EMITTER":
            # the rays of the circle is determined by the type of light source
            self.rays = []  # array of rays, see rays.py
            if self.emitter_type == "POINT":
                # emits light in all directions
                # default type if no type is specified
                for i in range(N_RAYS):
                    theta = (i * 2 * math.pi) / N_RAYS
                    theta_vector = (math.cos(theta), math.sin(theta))
                    self.rays.append(Ray(self.x, self.y, theta_vector, emit_color))

            elif self.emitter_type == "DIRECTIONAL":
                # use directional_angle value or default to 0
                directional_angle = (
                    directional_angle if directional_angle is not None else 0
                )

                directional_width = (
                    directional_width
                    if directional_width is not None
                    else 2 * self.radius  # the diameter
                )

                theta_vector = (
                    math.cos(directional_angle),
                    -math.sin(
                        directional_angle
                    ),  # negative due to PyGame's coordinate system
                )
                # Correct perpendicular vector
                perp_vector = (-theta_vector[1], theta_vector[0])

                spacing = directional_width / (N_RAYS - 1) if N_RAYS > 1 else 0

                for i in range(N_RAYS):
                    offset = (i - (N_RAYS - 1) / 2) * spacing
                    # position the ray starting point along the perpendicular direction
                    ray_position = (
                        self.x + offset * perp_vector[0],
                        self.y + offset * perp_vector[1],
                    )

                    self.rays.append(
                        Ray(
                            ray_position[0],
                            ray_position[1],
                            theta_vector,
                            emit_color,
                        )
                    )

            elif self.emitter_type == "SPOT":
                # emit light in a cone
                angles = np.linspace(spot_angle, spot_angle + spot_arc, N_RAYS)

                for angle in angles:
                    theta_vector = (math.cos(angle), -math.sin(angle))
                    ray_position = (
                        self.x + self.radius * np.cos(angle),
                        self.y + self.radius * np.sin(angle),
                    )

                    self.rays.append(
                        Ray(ray_position[0], ray_position[1], theta_vector, emit_color)
                    )

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
