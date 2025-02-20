import pygame

from src.circle import Circle

__RUNNING = True
__XSIZE = 800
__YSIZE = 600
__CIRCLES = []
__DRAGGING_CIRCLE = None

# colors
__WHITE = (255, 255, 255)
__BLACK = (0, 0, 0) 

if __name__ == "__main__":
    screen = pygame.display.set_mode((__XSIZE, __YSIZE))    
    pygame.init()

    while __RUNNING:
        mouse_x, mouse_y = pygame.mouse.get_pos()      
        for event in pygame.event.get():
            if event.type == pygame.KEYDOWN:
                if event.key == pygame.K_o:
                    circle = Circle(50, mouse_x, mouse_y, __WHITE)
                    __CIRCLES.append(circle)

            elif event.type == pygame.MOUSEBUTTONDOWN:
                if event.button == 1:
                    for circle in __CIRCLES:
                        delta_x = mouse_x - circle.get_x()
                        delta_y = mouse_y - circle.get_y()
                        if delta_x ** 2 + delta_y ** 2 <= circle.get_radius() ** 2:
                            __DRAGGING_CIRCLE = circle
                            break

            elif event.type == pygame.MOUSEBUTTONUP:
                if event.button == 1:
                    __DRAGGING_CIRCLE = None

            elif event.type == pygame.MOUSEMOTION:
                if __DRAGGING_CIRCLE:
                    __DRAGGING_CIRCLE.move(mouse_x, mouse_y)

            elif event.type == pygame.QUIT:
                __RUNNING = False

        # update the screen
        screen.fill(__BLACK)
        for circle in __CIRCLES:
            circle.draw(screen)
        pygame.display.update()