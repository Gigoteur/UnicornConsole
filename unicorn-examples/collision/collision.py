class Block(object):
    def __init__(self, name):
        self.name = name

def _init():
    A = Block("A")
    B = Block("B")

    world_add(A, 0, 0, 64, 256)
    world_add(B, 0, -100, 32, 32)
    actualX, actualY, cols, len_cols = world_move(B, 0, 64)

def _update():
    pass

def _draw():
    world_draw_debug()