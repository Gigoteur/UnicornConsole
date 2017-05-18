#px8 / python cartridge
#version 1
#__python__

RED = 8
BLUE = 12
WHITE = 7
GREEN = 11
GREY = 6
BROWN = 4

msgs = [
    ("PX8 Fantasy Games Console", GREEN),
    ("FONT TEST", WHITE),
    ("ABCDEFGHIJKLMNOPQRSTUVWXYZ", RED),
    ("abcdefghijklmnopqrstuvwxyz", RED),
    ("1234567890", RED),
    ("!@#$%^&*()-_=+{}[]:;|\?/<>,.'~`", RED),
    ("The quick brown fox jumps over", BLUE),
    ("the lazy dog", BLUE),
    ("YOU ARE STANDING AT THE END OF", GREY),
    ("A ROAD BEFORE A SMALL BRICK", GREY),
    ("BUILDING.", GREY),
    ("You are standing at the end of", BROWN),
    ("a road before a small brick", BROWN),
    ("building.", BROWN)
]

def _init():
    pass

def _update():
    pass

def _draw():
    cls()
    y = 0
    for msg,color in msgs:
        px8_print(msg, 0, y, color)
        y += 8
