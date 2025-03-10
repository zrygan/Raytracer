from math import pi
import pygame

# This file contains all the global variables used in the program

# pygame variables and game loop
RUNNING = True
X_SIZE = 800
Y_SIZE = 600

# default values
SCREEN = pygame.display.set_mode((X_SIZE, Y_SIZE))
DEFAULT_CIRCLE_RADIUS = 10
DEFAULT_RAY_COUNT = 50
DEFAULT_RAY_MAX_LENGTH = 525
DEFAULT_ANGLE_INCREMENT = (2 * pi) / 180
DEFAULT_ANGLE_INCREMENT_DELAY = 100

# assets
ASSETS = []
RAYS = {}
CIRCLES = []
EMMITERS = []
ABSORBERS = []
DRAGGING_CIRCLE = None

# directional light specific
DIRECTIONAL_ANGLE_OFFSET = 10

# colors
WHITE = (255, 255, 255)
BLACK = (0, 0, 0)
CORNFLOWER_BLUE = (100, 149, 237)
