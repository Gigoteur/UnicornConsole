#px8 / python cartridge
#version 1
#__python__

RED = 8
BLUE = 12

def _init():
    pass

def _update():
    pass

def _draw():
    cls()
    mx = mouse_x()
    my = mouse_y()
    px8_print("mouse-x:      %d" % mx, 0, 0, BLUE)
    px8_print("mouse-y:      %d" % my, 0, 8, BLUE)
    px8_print("mouse-statep: %d" % mouse_statep(), 0, 16, BLUE)
    rect(mx - 1, my - 1, mx + 1, my + 1, RED)
