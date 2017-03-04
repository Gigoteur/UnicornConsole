import math

# Audio
def sound_load(filename):
    px8_audio.sound_load(filename)

def sound_play(filename):
    px8_audio.sound_play(filename)

def sound_stop(filename):
    px8_audio.sound_stop(filename)

globals()["sound_load"] = sound_load
globals()["sound_play"] = sound_play
globals()["sound_stop"] = sound_stop

# Cart Data

# Graphics

def camera(x=-1, y=-1):
    px8_graphic.camera(x, y)

def circ(x, y, r, color=-1):
    px8_graphic.circ(math.floor(x), math.floor(y), math.floor(r), math.floor(color))

def circfill(x, y, r, color=-1):
    px8_graphic.circfill(math.floor(x), math.floor(y), math.floor(r), math.floor(color))

def clip(x=-1, y=-1, w=-1, h=-1):
    px8_graphic.clip(math.floor(x), math.floor(y), math.floor(w), math.floor(h))

def cls():
    px8_graphic.cls()

def color(col):
    px8_graphic.color(col)

def line(x1, y1, x2, y2, color=-1):
    px8_graphic.line(math.floor(x1), math.floor(y1), math.floor(x2), math.floor(y2), math.floor(color))

def pal(c0 = -1, c1 = -1, p=0):
    px8_graphic.pal(math.floor(c0), math.floor(c1))

def palt(c = -1, t = False):
    px8_graphic.palt(math.floor(c), t)

def pget(x, y):
    return px8_graphic.pget(math.floor(x), math.floor(y))

def pset(x, y, color):
    px8_graphic.pset(math.floor(x), math.floor(y), math.floor(color))

def px8_print(str, x=-1, y=-1, col=-1):
    return px8_graphic.print(str, x, y, col)

def rect(x1, y1, x2, y2, color=-1):
    px8_graphic.rect(math.floor(x1), math.floor(y1), math.floor(x2), math.floor(y2), math.floor(color))

def rectfill(x1, y1, x2, y2, color=-1):
    px8_graphic.rectfill(math.floor(x1), math.floor(y1), math.floor(x2), math.floor(y2), math.floor(color))

def sget(x, y):
    return px8_graphic.sget(x, y)

def spr(n, x, y,  w=1, h=1, flip_x=False, flip_y=False):
    px8_graphic.spr(n, x, y, w, h, flip_x, flip_y)

def spr_dyn(id, x, y, flip_x=False, flip_y=False):
    return px8_graphic.spr_dyn(id, x, y, flip_x, flip_y)

def spr_dyn_load(data, width, height):
    return px8_graphic.spr_dyn_load(data, width, height)

def sset(x, y, c=-1):
    px8_graphic.sset(x, y, c)

def sspr(sx, sy, sw, sh, dx, dy, dw=-1, dh=-1, flip_x=False, flip_y=False):
    if dw == -1:
        dw = sw

    if dh == -1:
        dh = sh

    px8_graphic.sspr(sx, sy, sw, sh, dx, dy, dw, dh, flip_x, flip_y)

def trigon(x1, y1, x2, y2, x3, y3, color):
    px8_graphic.trigon(math.floor(x1), math.floor(y1), math.floor(x2), math.floor(y2), math.floor(x3), math.floor(y3), color)

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
globals()["spr_dyn"] = spr_dyn
globals()["spr_dyn_load"] = spr_dyn_load
globals()["sset"] = sset
globals()["sspr"] = sspr
globals()["trigon"] = trigon

# Input

def btn(x, p=0):
    return px8_input.btn(x, p)

def btnp(x, p=0):
    return px8_input.btnp(x, p)

def mouse_x():
    return px8_input.btn_mouse(0)

def mouse_y():
    return px8_input.btn_mouse(1)

def mouse_state():
    return px8_input.btn_mouse_state()


globals()["btn"] = btn
globals()["btnp"] = btnp

globals()["mouse_x"] = mouse_x
globals()["mouse_y"] = mouse_y
globals()["mouse_state"] = mouse_state

# Map

def spr_map(cel_x, cel_y, sx, sy, cel_w, cel_h):
    px8_map.spr_map(cel_x, cel_y, sx, sy, cel_w, cel_h)

def mget(x, y):
    return px8_map.mget(math.floor(x), math.floor(y))

def mset(x, y, v):
    px8_map.mset(math.floor(x), math.floor(y), math.floor(v))

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

# Palette

def set_palette_color(col, r, g, b):
    px8_palette.set_palette_color(col, r, g, b)

def reset_palette():
    px8_palette.reset_palette()

def switch_palette(name):
    px8_palette.switch_palette(name)

globals()["set_palette_color"] = set_palette_color
globals()["reset_palette"] = reset_palette
globals()["switch_palette"] = switch_palette

# Peek/Poke

def stat(x):
    return 0

# Others
def px8_time():
    return px8_sys.time()

globals()["px8_time"] = px8_time



def img_to_rgb(data):
    from PIL import Image

    res = []
    im = Image.open(data)
    pix = im.load()
    width, height = im.size

    for x in range(width):
        for y in range(height):
            v = pix[x, y][:-1]

            res.append(v[0])
            res.append(v[1])
            res.append(v[2])

    return res, width, height



