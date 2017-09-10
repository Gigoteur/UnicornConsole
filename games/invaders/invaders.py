T = 0

def _init():
    pass

def _update():
    pass

def _draw():
    cls()

    draw_menu()

def draw_menu():
    global T

    gridcolor = 8
    w=127
    n=15

    T += .50
    for i in range(1,n):
        z=(i*n+T%n)
        y=w*n/z+32
        line(0,y,w,y,gridcolor)
        v=i+T%n/n-n/2
        line(v*9+64,40,v*60+64,w, gridcolor)
