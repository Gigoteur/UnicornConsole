import random

stars = []
t = 0

class Star(object):
    def __init__(self, x, y, col, sp):
        self.x = x
        self.y = y
        self.col = col
        self.sp = sp

def draw_stars():
    global stars, t
    for star in stars:
        pset(star.x, (star.y + 4*t*star.sp) % 128, star.col)

def _init():
    for i in range(0, 32):
        stars.append(Star(random.randint(0, 128),
                          random.randint(0, 128),
                          random.randint(0, 16),
                          i/32.0))


def _update():
    global t

    t = t + 1

def _end():
    if px8_time() > 0.200:
        return True
    return False

def _draw():
    cls()
    draw_stars()
    for x in range(0, 128):
        pset(x, 0, random.randint(0, 16))