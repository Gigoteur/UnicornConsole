px8 / python cartridge
version 1
__python__

import random

DARK_GREY = 5
RED = 8
BLUE = 12
WHITE = 7
GREEN = 11
GREY = 6
BROWN = 4

LEFT_ARROW = 0
RIGHT_ARROW = 1
UP_ARROW = 2
DOWN_ARROW = 3

FONTS = [
    "pico8",
    "bbc",
    "cbmII",
    "appleII",
]

frame = 0
font_index = 0

msgs = [
    ("PX8 Console", GREEN),
    ("ABCDEFGHIJKLMNOP", RED),
    ("QRSTUVWXYZ", RED),
    ("abcdefghijklmnop", RED),
    ("qrstuvwxyz", RED),
    ("1234567890", RED),
    ("!@#$%^&*()-_=+{}", RED),
    ("[]:;|\?/<>,.'~`", RED),
    ("You are standing", BLUE),
    ("at the end of a", BLUE),
    ("a road before a", BLUE),
    ("small brick", BLUE),
    ("building.", BLUE),
    ("arrows to switch", WHITE)
]

mode_index = 0
MODES = [
    (128, 128, 1.0),
    (256, 256, 1.0),
    (512, 512, 1.0),
    (512, 256, 1.0),
    (512, 256, 2.0),
    (512, 128, 4.0),
    (1024, 128, 4.0),
    (800, 600, 800.0/600.0),
    (1024, 768, 1024.0/768.0),
    (1920, 1200, 1920.0/1200.0),
]

balls = []
MAX_BALL_SIZE = 5
NUM_BALLS = 100


class Ball:
    def __init__(self):
        screen_width, screen_height, _ = MODES[mode_index]
        self.x = random.random() * screen_width
        self.y = random.random() * screen_height
        self.vx = random.random() + 0.5
        self.vy = random.random() + 0.5
        self.size = int(random.random() * MAX_BALL_SIZE) + 1
        self.color = int(random.random() * 16)

    def update(self):
        screen_width, screen_height, _ = MODES[mode_index]
        self.x = self.x + self.vx
        self.y = self.y + self.vy

        if self.x < 0:
            self.vx = abs(self.vx)
        if self.y < 0:
            self.vy = abs(self.vy)
        if self.x >= screen_width:
            self.vx = -abs(self.vx)
        if self.y >= screen_height:
            self.vy = -abs(self.vy)

    def draw(self):
        rectfill(int(self.x), int(self.y), int(self.x) + self.size, int(self.y) + self.size,
                 self.color)


def _init():
    global balls

    mode(*MODES[mode_index])
    balls = [Ball() for i in range(NUM_BALLS)]


def _update():
    global font_index
    global mode_index
    mode_changed = False

    if btnp(LEFT_ARROW):
        font_index = font_index - 1
        if font_index < 0:
            font_index += len(FONTS)

    if btnp(RIGHT_ARROW):
        font_index = (font_index + 1) % len(FONTS)

    if btnp(DOWN_ARROW):
        mode_index = mode_index - 1
        if mode_index < 0:
            mode_index += len(MODES)
        mode(*MODES[mode_index])
        mode_changed = True

    if btnp(UP_ARROW):
        mode_index = (mode_index + 1) % len(MODES)
        mode_changed = True

    if mode_changed:
        mode(*MODES[mode_index])

    for ball in balls:
        ball.update()


def _draw():
    width, height, _ = MODES[mode_index]

    if height == 128:
        line_height = 8
    else:
        line_height = 10

    rectfill(0, 0, width-1, height-1, DARK_GREY)

    for ball in balls:
        ball.draw()

    font(FONTS[font_index])
    y = 0 
    px8_print("Mode: %dx%d (%.1f)" % MODES[mode_index], 0, y, WHITE)
    y += line_height
    px8_print("Font: %s" % FONTS[font_index], 0, y, WHITE)
    y += line_height
    for msg, color in msgs:
        px8_print(msg, 0, y, color)
        y += line_height
