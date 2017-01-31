import math

# Audio

# Cart Data

# Graphics

def camera(x=-1, y=-1):
    global_obj.camera(x, y)

def circ(x, y, r, color=-1):
    global_obj.circ(math.floor(x), math.floor(y), math.floor(r), color)

def circfill(x, y, r, color=-1):
    global_obj.circfill(math.floor(x), math.floor(y), math.floor(r), color)

def clip(x=-1, y=-1, w=-1, h=-1):
    global_obj.clip(math.floor(x), math.floor(y), math.floor(w), math.floor(h))

def cls():
    global_obj.cls()

def color(col):
    global_obj.color(col)

def line(x1, y1, x2, y2, color=-1):
    global_obj.line(math.floor(x1), math.floor(y1), math.floor(x2), math.floor(y2), color)

def pal(c0 = -1, c1 = -1, p=0):
    global_obj.pal(math.floor(c0), math.floor(c1))

def palt(c = -1, t = False):
    global_obj.palt(math.floor(c), t)

def pget(x, y):
    return global_obj.pget(math.floor(x), math.floor(y))

def pset(x, y, color):
    global_obj.pset(math.floor(x), math.floor(y), math.floor(color))

def px8_print(str, x=-1, y=-1, col=-1):
    return global_obj.print(str, x, y, col)

def rect(x1, y1, x2, y2, color=-1):
    global_obj.rect(math.floor(x1), math.floor(y1), math.floor(x2), math.floor(y2), color)

def rectfill(x1, y1, x2, y2, color=-1):
    global_obj.rectfill(math.floor(x1), math.floor(y1), math.floor(x2), math.floor(y2), color)

def sget(x, y):
    return global_obj.sget(x, y)

def spr(n, x, y,  w=1, h=1, flip_x=False, flip_y=False):
    global_obj.spr(n, x, y, w, h, flip_x, flip_y)

def sset(x, y, c=-1):
    global_obj.sset(x, y, c)

def sspr(sx, sy, sw, sh, dx, dy, dw=-1, dh=-1, flip_x=False, flip_y=False):
    if dw == -1:
        dw = sw

    if dh == -1:
        dh = sh

    global_obj.sspr(sx, sy, sw, sh, dx, dy, dw, dh, flip_x, flip_y)

def trigon(x1, y1, x2, y2, x3, y3, color):
    global_obj.trigon(math.floor(x1), math.floor(y1), math.floor(x2), math.floor(y2), math.floor(x3), math.floor(y3), color)

globals()["camera"] = camera
globals()["circ"] = circ
globals()["circfill"] = circfill
globals()["clip"] = clip
globals()["cls"] = cls
globals()["color"] = color
globals()["line"] = line
globals()["pal"] = pal
globals()["palt"] = palt
globals()["pset"] = pset
globals()["pget"] = pget
globals()["px8_print"] = px8_print
globals()["rect"] = rect
globals()["rectfill"] = rectfill
globals()["sget"] = sget
globals()["spr"] = spr
globals()["sset"] = sset
globals()["sspr"] = sspr
globals()["trigon"] = trigon

# Input

def btn(x, p=0):
    return global_obj.btn(x, p)

def btnp(x, p=0):
    return global_obj.btnp(x, p)

def mouse_x():
    return global_obj.btn_mouse(0)

def mouse_y():
    return global_obj.btn_mouse(1)

def mouse_state():
    return global_obj.btn_mouse_state()


globals()["btn"] = btn
globals()["btnp"] = btnp

globals()["mouse_x"] = mouse_x
globals()["mouse_y"] = mouse_y
globals()["mouse_state"] = mouse_state

# Map

def spr_map(cel_x, cel_y, sx, sy, cel_w, cel_h):
    global_obj.spr_map(cel_x, cel_y, sx, sy, cel_w, cel_h)

def mget(x, y):
    return global_obj.mget(math.floor(x), math.floor(y))

def mset(x, y, v):
    global_obj.mset(math.floor(x), math.floor(y), math.floor(v))

globals()["spr_map"] = spr_map
globals()["mget"] = mget
globals()["mset"] = mset

# Math
def atan2(x, y):
    v = math.atan2(y,x)
    return (((v - math.pi) / (math.pi*2)) + 0.25) % 1.0

def cos(x):
    return math.cos((x or 0)*(math.pi*2))

def sin(x):
    return math.sin(-(x or 0)*(math.pi*2))

def flr(x):
    return math.floor(x)

def rnd(x):
    return random.random() * x

globals()["atan2"] = atan2
globals()["ceil"] = math.ceil
globals()["cos"] = cos
globals()["sin"] = sin
globals()["flr"] = flr
globals()["rnd"] = rnd
globals()["sqrt"] = math.sqrt

# Memory

# Peek/Poke

def stat(x):
    return 0

# Others
def px8_time():
    return global_obj.time()

globals()["px8_time"] = px8_time





