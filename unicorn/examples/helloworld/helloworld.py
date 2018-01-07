T = 0

def _init():
    pass

def _update():
    global T
    T = T + 1

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

def _draw_part3():
    global T
    for i in range(1, 12):
        for j0 in range(0, 7):
            j = 7 - j0
            col = 7 + j - 1
            t1 = T + i * 4 - j * 2
            x = cos(j0) * 5
            y = 38 + j + cos(t1/50) * 5
            pal(7, col)
            spr(16+i, 8+i*8+x, y)

def _draw():
    cls()
    #_draw_part1()
    #_draw_part2()
    _draw_part3()