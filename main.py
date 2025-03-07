import pygame
import math
from src.objects import Emitter_Directional, Emitter_Point, Emitter_Spot
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
                    CIRCLES.append(Emitter_Point(mouse_x, mouse_y))

                elif event.key == pygame.K_d:
                    CIRCLES.append(
                        Emitter_Directional(mouse_x, mouse_y, math.pi / 4, None)
                    )
                elif event.key == pygame.K_s:
                    CIRCLES.append(Emitter_Spot(mouse_x, mouse_y, 0, math.pi / 2))

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
