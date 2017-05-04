def _init():
    pass

def _update():
    pass

def _draw_part1():
    px8_print("Hello World", 40, 64, 1)

    for i in range(0, 124, 8):
        spr(0, i, 20)

    for i in range(0, 124, 8):
        spr(0, i, 29, flip_x=True)

    for i in range(0, 124, 8):
        spr(0, i, 38, flip_y=True)


    for i in range(0, 124, 8):
        spr(0, i, 47, flip_x=True, flip_y=True)

def _draw_part2():
    spr(1, 0, 60)

def _draw():
    #_draw_part1()
    _draw_part2()