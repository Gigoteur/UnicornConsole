addglobals = lambda x: globals().update(x)

from utils import Vec2

class Bush(object):
    def __init__(self, p, r, height, colour, bloom):
        self.pos = p
        self.r = r
        self.height = height
        self.colour = colour
        self.bloom = bloom
        self.s = Vec2(p.x, p.y)

class Bushes(object):
    def __init__(self, config):
        self.config = config

        self.cell_fill = self.config.cell_fill
        self.cell_size = self.config.cell_size

    def update(self, x, y, cell, cam, cells, blobs):
        bushes = cell.bushes
        cellp = Vec2(
            cam.pos.x%self.cell_size-x*self.cell_size,
            cam.pos.y%self.cell_size-y*self.cell_size
        )
        for bush in bushes:
            bush.s = bush.pos.sub(cellp.add(self.config.perspective_offset))
            bush.s = bush.s.mul(bush.height*0.015)
            bush.s._add(bush.pos)


    def draw(self, a, b, cell, cam, shadow):
        bushes = cell.bushes
        if bushes:
            camera(
                cam.c.x-a*self.cell_size,
                cam.c.y-b*self.cell_size
            )

            if shadow:
                for bush in bushes:
                    circfill(
                        bush.pos.x+self.config.shadow_offset.x*bush.height,
                        bush.pos.y+self.config.shadow_offset.y*bush.height,
                        bush.r,
                        5)
            else:
                for bush in bushes:
                    circfill(bush.s.x, bush.s.y, bush.r, 3)
                for bush in bushes:
                    if bush.bloom:
                        p=bush.s.add(bush.bloom)
                        pset(p.x,p.y,bush.colour)
