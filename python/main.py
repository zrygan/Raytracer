from modules.util import *
from modules.objects import *
from modules.variables import *
from modules.shadowing import *

if __name__ == "__main__":
    print("Raytracer ver. 0.py\nBy: Zhean Ganituen\nSource Available on: https://github.com/zrygan/Raytracer")
    print("Notice. This is the Python legacy edition and only has features from ver 0.py.")
    print("This version is NOT MAINTAINED anymore.")
    pygame.init()

    while RUNNING:
        mouse_x, mouse_y = pygame.mouse.get_pos()
        for event in pygame.event.get():
            if event.type == pygame.KEYDOWN:
                # ADD: POINT EMITTER
                if event.key == pygame.K_o:
                    obj = Emitter_Point(mouse_x, mouse_y)

                    CIRCLES.append(obj)
                    EMMITERS.append(obj)

                # ADD: DIRECTIONAL EMITTER
                elif event.key == pygame.K_d:
                    obj = Emitter_Directional(mouse_x, mouse_y, math.pi / 4)

                    CIRCLES.append(obj)
                    EMMITERS.append(obj)
                # ADD: SPOT EMITTER
                elif event.key == pygame.K_s:
                    obj = Emitter_Spot(
                        mouse_x, mouse_y, degree_to_radian(90), degree_to_radian(20)
                    )

                    CIRCLES.append(obj)
                    EMMITERS.append(obj)

                # ADD: CIRCLE ABSORBER
                elif event.key == pygame.K_a:
                    obj = Absorber_Circle(mouse_x, mouse_y)

                    CIRCLES.append(obj)
                    ABSORBERS.append(obj)

                # CHANGE: SPOT, DIRECTIONAL EMITTER ANGLE
                elif event.key == pygame.K_COMMA or event.key == pygame.K_PERIOD:
                    object = get_hovered_object(mouse_x, mouse_y)

                    if (type(object) is Emitter_Directional) or (
                        type(object) is Emitter_Spot
                    ):
                        if event.key == pygame.K_COMMA:
                            object.set_angle(
                                object.get_angle() + DEFAULT_ANGLE_INCREMENT
                            )
                        elif event.key == pygame.K_PERIOD:
                            object.set_angle(
                                object.get_angle() - DEFAULT_ANGLE_INCREMENT
                            )
                        object.initialize_rays()
                        check_shadows()

                elif event.key == pygame.K_BACKSPACE:
                    object = get_hovered_object(mouse_x, mouse_y)

                    if object:
                        CIRCLES.remove(object)

                        if object in EMMITERS:
                            EMMITERS.remove(object)
                        elif object in ABSORBERS:
                            ABSORBERS.remove(object)

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

        # update the SCREEN
        SCREEN.fill(BLACK)

        for circle in CIRCLES:
            if circle not in ASSETS:
                ASSETS.append(circle)

            circle.draw(SCREEN)

            if circle in EMMITERS:
                for ray in circle.get_rays():
                    ray.draw(SCREEN)

        keys = pygame.key.get_pressed()
        current_time = pygame.time.get_ticks()
        last_change_time = 0

        if (keys[pygame.K_COMMA] or keys[pygame.K_PERIOD]) and (
            current_time - last_change_time >= DEFAULT_ANGLE_INCREMENT_DELAY
        ):
            mouse_x, mouse_y = pygame.mouse.get_pos()
            object = get_hovered_object(mouse_x, mouse_y)

            if (type(object) is Emitter_Directional) or (type(object) is Emitter_Spot):
                if keys[pygame.K_COMMA]:
                    object.set_angle(object.get_angle() - DEFAULT_ANGLE_INCREMENT)
                elif keys[pygame.K_PERIOD]:
                    object.set_angle(object.get_angle() + DEFAULT_ANGLE_INCREMENT)
                object.initialize_rays()
                check_shadows()
                last_change_time = current_time

        pygame.display.update()
