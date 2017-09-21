class Entity(object):
    def __init__(self, l, t, w, h):
        self.l = l
        self.t = t
        self.w = w
        self.h = h
        self.vx = 0
        self.vy = 0
        world_add(self, l, t, w, h)

class Block(Entity):
    def __init__(self, l, t, w, h):
        super(Block, self).__init__(l, t, w, h)

    def draw(self):
        pass


def _init():
    world_add("A", 0, 0, 64, 256)
    world_add("B", 0, -100, 32, 32)
    actualX, actualY, cols, len = world_move("B", 0, 64)

def _update():
    pass

def _draw():
    pass