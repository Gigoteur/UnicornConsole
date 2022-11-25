# Demo of lighting engine from http://www.lexaloffle.com/bbs/?tid=28785

import math

class Palette(object):
    def __init__(self, sprites, size):
        self.palettes = [[]] * (len(sprites)*8)
        self.light_rng=[
            -1000,
            10*42,18*42,
            26*42,34*42,
            42*42,
        ]

        print(self.light_rng)

        idx = 0
        for sprite in sprites:
            sprite_x = (sprite % 16) * 8
            sprite_y = math.floor(sprite / 16) * 8
            for y in range(0, 8):
                self.palettes[idx] = [0] * size

                for x in range(0, size):
                    self.palettes[idx][x] = sget(sprite_x+x, sprite_y+y)
                idx += 1

        for col in self.palettes:
            print(col)

    def get(self, value, level):
        return self.palettes[value][level]

class Player(object):
    def __init__(self):
        self.x = 0
        self.y = 0

    def update(self, x, y):
        self.x = x
        self.y = y

    def draw(self):
        # Draw a simple circle
        circ(self.x, self.y, 2, 7)

class Lighting(object):
    def __init__(self, pallettes):
        self.palettes = pallettes

    def apply(self, lx, ly, xl, yt, xr, yb):
        for y in range(yt, yb):
            #print(y)
            ysq = (y - ly) * (y - ly)
            brkpts = {}
            for lv in range(5, -1, -1):
                 lrng = self.palettes.light_rng[lv]
                 xsq = lrng - ysq
                 if xsq > 0:
                    brkpts[lv] = lx - flr(sqrt(xsq))

            if brkpts:
                bright_level = 6 - len(brkpts)
                if bright_level == 5:
                    bright_level = 6

                for x in range(lx, xl-1, -1):
                    x_opp = xr - (x - xl)

                    if brkpts.get(bright_level):
                        value = brkpts[bright_level]
                        if value > x:
                            bright_level += 1

                        pset(x, y, self.palettes.get(pget(x, y), bright_level-1))
                        pset(x_opp, y, self.palettes.get(pget(x_opp, y), bright_level-1))

                    else:
                        pset(x, y, 0)
                        pset(x_opp, y, 0)

        line(xl, yt, xr, yt, 0)

PLAYER = Player()
PALETTE1 = Palette([0, 16], 6)
LIGHT = Lighting(PALETTE1)

def _init():
    pass

def _update():
    _mouse_x = mouse_x()
    _mouse_y = mouse_y()
    PLAYER.update(_mouse_x, _mouse_y)

def _draw():
    cls()
    palt()
    palt(0,False)

    r = flr(42*1)
    xl, yt, xr, yb = PLAYER.x - r, PLAYER.y - r, PLAYER.x + r, PLAYER.y + r
    clip(xl, yt, xr-xl, yb-yt)
    spr_map(0,0,0,0,16,16)

    LIGHT.apply(PLAYER.x, PLAYER.y, xl, yt, xr, yb)
    PLAYER.draw()