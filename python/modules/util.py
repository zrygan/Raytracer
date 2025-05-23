from typing import Tuple
import math
from modules.variables import *


def hexstr_int(hex_value: str) -> int:
    ret = 0
    place_value = len(hex_value) - 1
    for char in hex_value:
        try:
            # for cases: 0 1 2 3 4 5 6 7 8 9
            if int(char) <= 9:
                ret += int(char) * pow(16, place_value)
        except ValueError:
            # for cases: A B C D E F
            match char:
                case "A":
                    ret += 10 * pow(16, place_value)
                case "B":
                    ret += 11 * pow(16, place_value)
                case "C":
                    ret += 12 * pow(16, place_value)
                case "D":
                    ret += 13 * pow(16, place_value)
                case "E":
                    ret += 14 * pow(16, place_value)
                case "F":
                    ret += 15 * pow(16, place_value)

        place_value -= 1

    return int(ret)


def hex_to_rbg(hex_value: str) -> Tuple[int, int, int]:
    return (
        hexstr_int(hex_value[0:2]),
        hexstr_int(hex_value[3:5]),
        hexstr_int(hex_value[6:]),
    )


def degree_to_radian(degree: float) -> float:
    return degree * (math.pi / 180)


def get_hovered_object(mouse_x: int, mouse_y: int) -> object:
    for object in ASSETS:
        delta_x = mouse_x - object.get_x()
        delta_y = mouse_y - object.get_y()
        if delta_x**2 + delta_y**2 <= object.get_radius() ** 2:
            return object
    return None
