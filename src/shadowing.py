import math
from src.variables import *
from src.rays import *


def shadow(x: int, y: int, r: int, ray: Ray) -> Tuple[int, int]:
    # determine the slope of the ray
    x1 = ray.get_x_start()
    x2 = ray.get_x_end()
    y1 = ray.get_y_start()
    y2 = ray.get_y_end()

    vector = [x2 - x1, y2 - y1]

    # Quadratic Coefficients
    A = vector[0] ** 2 + vector[1] ** 2
    B = 2 * (vector[0] * (x1 - x) + vector[1] * (y1 - y))
    C = (x1 - x) ** 2 + (y1 - y) ** 2 - r**2

    discriminant = B**2 - 4 * A * C

    # check if the Quadratic has a solution
    if discriminant < 0:
        return (None, None)

    # there must be two solutions
    sqrt_discriminant = math.sqrt(discriminant)
    solution_1 = (-B - sqrt_discriminant) / (2 * A) if A != 0 else 0
    solution_2 = (-B + sqrt_discriminant) / (2 * A) if A != 0 else 0

    # Initialize variables to None
    absorbed_at_x, absorbed_at_y = None, None

    # Check both solutions and choose the one that is after the ray's start
    if 0 <= solution_1 <= 1:
        absorbed_at_x = x1 + solution_1 * vector[0]
        absorbed_at_y = y1 + solution_1 * vector[1]

        # Make sure the intersection point is in the direction of the ray
        if solution_1 > 0:
            return (absorbed_at_x, absorbed_at_y)

    if 0 <= solution_2 <= 1:
        absorbed_at_x = x1 + solution_2 * vector[0]
        absorbed_at_y = y1 + solution_2 * vector[1]

        # Make sure the intersection point is in the direction of the ray
        if solution_2 > 0:
            return (absorbed_at_x, absorbed_at_y)

    return (None, None)


def reset_all_rays():
    """Reset all rays to their original lengths."""
    for ray in RAYS:
        ray.set_x(x_end=ray.x_start + DEFAULT_RAY_MAX_LENGTH * ray.angle[0])
        ray.set_y(y_end=ray.y_start + DEFAULT_RAY_MAX_LENGTH * ray.angle[1])


def check_shadows():
    """Check shadows for all rays against all absorbers."""
    # First reset all rays to their original length
    reset_all_rays()

    # Then check each absorber for shadow intersections
    for obj in ABSORBERS:
        try:
            x = obj.get_x()
            y = obj.get_y()
            r = obj.get_radius()

            for ray in RAYS:
                absorbed_at = shadow(x, y, r, ray)
                if (
                    absorbed_at[0] is not None and absorbed_at[1] is not None
                ):  # Check both coordinates
                    current_length = math.sqrt(
                        (ray.get_x_end() - ray.get_x_start()) ** 2
                        + (ray.get_y_end() - ray.get_y_start()) ** 2
                    )
                    new_length = math.sqrt(
                        (absorbed_at[0] - ray.get_x_start()) ** 2
                        + (absorbed_at[1] - ray.get_y_start()) ** 2
                    )

                    if new_length < current_length:
                        ray.set_x(x_end=absorbed_at[0])
                        ray.set_y(y_end=absorbed_at[1])

        except AttributeError:
            continue
