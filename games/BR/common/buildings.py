addglobals = lambda x: globals().update(x)

from utils import Vec2, frange

class Building(object):
    def __init__(self, w, h, pos, height, color):
        self.w = w
        self.h = h
        self.pos = pos
        self.height = height
        self.color = color
        self.s = Vec2(0, 0)

class Buildings(object):
    def __init__(self, config):
        self.config = config
        self.cell_size = self.config.cell_size

    def update(self, x, y, cell, cam, cells, blobs):
        building = cell.building
        if building:
            cellp = Vec2(
                cam.pos.x%self.cell_size-x*self.cell_size,
                cam.pos.y%self.cell_size-y*self.cell_size
            )
            building.s = building.pos.sub(cellp.add(self.config.perspective_offset))

            s1=max(building.w,building.h)
            s2=min(building.w,building.h)
            for i in frange(-s1+s2/2,s1-s2/2,s2):
                p1 = Vec2((cells.pos.x+x)*self.cell_size, (cells.pos.y+y)*self.cell_size).add(building.pos)
                if s1 == building.w:
                    p1.x += i
                else:
                    p1.y += i
                blobs.add_blob(
                    p1,
                    s2
                )

                p2 = Vec2((cells.pos.x+x)*self.cell_size, (cells.pos.y+y)*self.cell_size).add(building.pos)
                if s1 == building.w:
                    p2.x += s1-s2/2
                else:
                    p2.y += s1-s2/2

                if p2.dist(p1) > 2:
                    blobs.add_blob(
                        p2,
                        s2
                    )

    def draw(self, a, b, cell, cam, shadow):
        building = cell.building
        if building:
            camera(
                cam.c.x-a*self.cell_size,
                cam.c.y-b*self.cell_size
            )

            if shadow:
                for i in frange(0,building.height/2,4):
                    t = Vec2(building.s.x,building.s.y)
                    t._mul(i*0.015)
                    t._add(building.pos)

                    rectfill(t.x-building.w, t.y-building.h, t.x+building.w, t.y+building.h, 5)
            else:
                for i in frange(building.height/2,building.height-1,4):
                    t = Vec2(building.s.x,building.s.y)
                    t._mul(i*0.015)
                    t._add(building.pos)

                    rectfill(t.x-building.w, t.y-building.h, t.x+building.w, t.y+building.h, 5)

                    s = building.s.mul(building.height*0.015)
                    s._add(building.pos)
                    rectfill(s.x-building.w, s.y-building.h, s.x+building.w, s.y+building.h, building.color)
