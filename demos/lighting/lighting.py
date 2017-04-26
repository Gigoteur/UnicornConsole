# Demo of lighting engine from http://www.lexaloffle.com/bbs/?tid=28785

import math

def pointInRectangle(x, y, coord):
    return (coord[0] <= x <= coord[2] and
            coord[1] <= y <= coord[3])

class Palette(object):
    def __init__(self, sprites, size):
        self.palettes = [[]] * (len(sprites)*8)
        self.light_rng=[
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

PALETTE1 = Palette([0, 16], 6)

def _init():
    pass

def _update():
    pass

def _draw():
    cls()
    palt()
    palt(0,False)


    spr_map(0,0,0,0,16,16)
    spr_map(0,0,0,0,16,16,128)

    _mouse_x = mouse_x()
    _mouse_y = mouse_y()

    light_rect = [_mouse_x - 20, _mouse_y - 20, _mouse_x + 20, _mouse_y + 20]

    for x in range(0, 128):
        for y in range(0, 128):
            #pset(x, y, PALETTE1.get(pget(x, y), 4))
            if not pointInRectangle(x, y, light_rect):
                pset(x, y, 0)

    circ(_mouse_x, _mouse_y, 2, 7)