from math import pi

# This file contains all the global variables used in the program

# pygame variables and game loop
RUNNING = True
X_SIZE = 800
Y_SIZE = 600

# default values
DEFAULT_ANGLE_INCREMENT = (2 * pi) / 180
DEFAULT_CIRCLE_RADIUS = 10
DEFAULT_RAY_COUNT = 10
DEFAULT_RAY_MAX_LENGTH = 100000

# assets
ASSETS = []
RAYS = {}
CIRCLES = []
EMMITERS = {}
ABSORBERS = {}
DRAGGING_CIRCLE = None

# directional light specific
DIRECTIONAL_ANGLE_OFFSET = 10

# colors
WHITE = (255, 255, 255)
BLACK = (0, 0, 0)
CORNFLOWER_BLUE = (100, 149, 237)
