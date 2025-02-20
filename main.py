import pygame
import math
from src.circle import Circle
from src.variables import (
    X_SIZE,
    Y_SIZE,
    RUNNING,
    WHITE,
    BLACK,
    CIRCLES,
    DRAGGING_CIRCLE,
    CIRCLE_DEFAULT_RADIUS,
)

if __name__ == "__main__":
    screen = pygame.display.set_mode((X_SIZE, Y_SIZE))
    pygame.init()

    while RUNNING:
        mouse_x, mouse_y = pygame.mouse.get_pos()
        for event in pygame.event.get():
            if event.type == pygame.KEYDOWN:
                if event.key == pygame.K_o:
                    circle = Circle(
                        CIRCLE_DEFAULT_RADIUS,
                        mouse_x,
                        mouse_y,
                        "EMITTER",
                        WHITE,
                        emitter_type="POINT",
                        emit_color=None,
                    )
                    CIRCLES.append(circle)

                elif event.key == pygame.K_d:
                    circle = Circle(
                        CIRCLE_DEFAULT_RADIUS,
                        mouse_x,
                        mouse_y,
                        "EMITTER",
                        WHITE,
                        emitter_type="DIRECTIONAL",
                        emit_color=None,
                        directional_angle=math.pi / 4,
                        directional_width=None,
                    )
                    CIRCLES.append(circle)
                elif event.key == pygame.K_s:
                    circle = Circle(
                        CIRCLE_DEFAULT_RADIUS,
                        mouse_x,
                        mouse_y,
                        "EMITTER",
                        WHITE,
                        emitter_type="SPOT",
                        spot_angle=0,
                        spot_arc=math.pi / 2,
                    )
                    CIRCLES.append(circle)

            elif event.type == pygame.MOUSEBUTTONDOWN:
                if event.button == 1:
                    for circle in CIRCLES:
                        delta_x = mouse_x - circle.get_x()
                        delta_y = mouse_y - circle.get_y()
                        if delta_x**2 + delta_y**2 <= circle.get_radius() ** 2:
                            DRAGGING_CIRCLE = circle
                            break

            elif event.type == pygame.MOUSEBUTTONUP:
                if event.button == 1:
                    DRAGGING_CIRCLE = None

            elif event.type == pygame.MOUSEMOTION:
                if DRAGGING_CIRCLE:
                    DRAGGING_CIRCLE.move(mouse_x, mouse_y)

            elif event.type == pygame.QUIT:
                RUNNING = False

        # update the screen
        screen.fill(BLACK)
        for circle in CIRCLES:
            circle.draw(screen)
            for ray in circle.get_rays():
                ray.draw(screen)
        pygame.display.update()
