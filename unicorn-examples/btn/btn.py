#px8 / python cartridge
#version 1
#__python__

BUTTONS = [[], []]
F = 0
MAX_BUTTONS = 6

def _init():
    for p in range(0, 2):
        BUTTONS[p] = [[]] * MAX_BUTTONS
        for b in range(0, MAX_BUTTONS):
            BUTTONS[p][b] = [8] * 32

def _update():
    global F, BUTTONS

    for p in range(0, 2):
        for b in range(0, MAX_BUTTONS):
            if btnp(b, p):
                bb=24+b+1
            elif btn(b, p):
                bb=8+b+1
            else:
                bb=8
            BUTTONS[p][b][F]=bb
    F=1+F%31

def draw_data(xx, yy, n):
    global F, BUTTONS

    y=yy
    for b in range(0, MAX_BUTTONS):
        x=xx
        for f in range(0, 32):
            spr(BUTTONS[n][b][f], x, y)
            x+=4
        spr(4,xx+F*4-4,y)
        y+=4

def draw_pad(x, y, n, o):
    mapdraw(0, 0, x, y, 11, 5)

    if btn(0,n):
        spr(48,x+8,y+16)
    if btnp(0,n):
        spr(54,x+8,y+16)

    if btn(1,n):
        spr(49,x+24,y+16)
    if btnp(1,n):
        spr(55,x+24,y+16)

    if btn(2,n):
        spr(50,x+16,y+8)
    if btnp(2,n):
        spr(56,x+16,y+8)

    if btn(3,n):
        spr(51,x+16,y+24)
    if btnp(3,n):
        spr(57,x+16,y+24)

    if btn(4,n):
        spr(52,x+56,y+24)
    if btnp(4,n):
        sspr(80,22,10,10,x+55,y+23)

    if btn(5,n):
        spr(53,x+72,y+24)
    if btnp(5,n):
        sspr(80,22,10,10,x+71,y+23)

def _draw():
    cls()
    draw_pad(0, 0, 0, 6)
    draw_pad(40, 70, 1, 6)
    draw_data(0, 39, 0)
    draw_data(0, 103, 1)