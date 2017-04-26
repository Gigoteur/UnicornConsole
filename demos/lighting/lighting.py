# Demo of lighting engine from http://www.lexaloffle.com/bbs/?tid=28785
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

    circ(_mouse_x, _mouse_y, 2, 7)