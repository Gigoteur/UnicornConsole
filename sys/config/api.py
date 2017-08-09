import math
import random
from math import frexp, copysign
from sys import float_info

# Audio
# Chiptune
def chiptune_play(filetype, filename, loops, start_position):
    px8_audio.chiptune_play(filetype, filename, loops, start_position)

def chiptune_stop(music=1, sound=1):
    px8_audio.chiptune_stop(music, sound)

def chiptune_pause(music=1, sound=1):
    px8_audio.chiptune_pause(music, sound)

def chiptune_resume(music=1, sound=1):
    px8_audio.chiptune_resume(music, sound)

def chiptune_volume(volume):
    px8_audio.chiptune_volume(volume)

def chiptune_position():
    return px8_audio.chiptune_position()

# Classic music
def music_load(filename):
    return px8_audio.music_load(filename)

def music_play(filename, loops=-1):
    px8_audio.music_play(filename, loops)

def music_pause():
    px8_audio.music_pause()

def music_resume():
    px8_audio.music_resume()

def music_stop():
    px8_audio.music_stop()

def music_rewind():
    px8_audio.music_rewind()

def music_volume(volume):
    px8_audio.music_volume(volume)

def sound_load(filename):
    return px8_audio.sound_load(filename)

def sound_play(filename, loops=0, channel=-1):
    return px8_audio.sound_play(filename, loops, channel)

def sound_pause(channel=-1):
    px8_audio.sound_pause(channel)

def sound_resume(channel=-1):
    px8_audio.sound_resume(channel)

def sound_stop(channel=-1):
    px8_audio.sound_stop(channel)

def sound_volume(volume, channel=-1):
    px8_audio.sound_volume(volume, channel)

def sound_isplaying(channel=-1):
    return px8_audio.sound_isplaying(channel)

globals()["chiptune_play"] = chiptune_play
globals()["chiptune_stop"] = chiptune_stop
globals()["chiptune_pause"] = chiptune_pause
globals()["chiptune_resume"] = chiptune_resume
globals()["chiptune_volume"] = chiptune_volume
globals()["chiptune_position"] = chiptune_position

globals()["music_load"] = music_load
globals()["music_play"] = music_play
globals()["music_stop"] = music_stop
globals()["music_pause"] = music_pause
globals()["music_resume"] = music_resume
globals()["music_rewind"] = music_rewind
globals()["music_volume"] = music_volume

globals()["sound_load"] = sound_load
globals()["sound_play"] = sound_play
globals()["sound_pause"] = sound_pause
globals()["sound_resume"] = sound_resume
globals()["sound_stop"] = sound_stop
globals()["sound_volume"] = sound_volume
globals()["sound_isplaying"] = sound_isplaying

# Cart Data

# Graphics

def camera(x=-1, y=-1):
    px8_graphic.camera(flr(x), flr(y))

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

def ellipse(x, y, rx, ry, color=-1):
    px8_graphic.ellipse(math.floor(x), math.floor(y), math.floor(rx), math.floor(ry), math.floor(color))

def ellipsefill(x, y, rx, ry, color=-1):
    px8_graphic.ellipsefill(math.floor(x), math.floor(y), math.floor(rx), math.floor(ry), math.floor(color))

def fget(idx_sprite, flag=-1):
    if flag == -1:
        px8_graphic.fget_all(idx_sprite)
    return px8_graphic.fget(idx_sprite, flag)

def font(name = "pico8"):
    px8_graphic.font(name)

def fset(idx_sprite, flag, value=-1):
    if value == -1:
        px8_graphic.fset_all(flag)
    else:
        px8_graphic.fset(idx_sprite, flag, value)

def line(x1, y1, x2, y2, color=-1):
    px8_graphic.line(math.floor(x1), math.floor(y1), math.floor(x2), math.floor(y2), math.floor(color))

def mode(width, height, aspect_ratio=None):
    if aspect_ratio is None:
        aspect_ratio = float(width) / float(height)
    px8_graphic.mode(width, height, aspect_ratio)

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
    px8_graphic.spr(math.floor(n), math.floor(x), math.floor(y), math.floor(w), math.floor(h), flip_x, flip_y)

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

def polygon(x, y, color):
    px8_graphic.polygon(x, y, color)

globals()["camera"] = camera
globals()["circ"] = circ
globals()["circfill"] = circfill
globals()["clip"] = clip
globals()["cls"] = cls
globals()["color"] = color
globals()["ellipse"] = ellipse
globals()["ellipsefill"] = ellipsefill
globals()["fget"] = fget
globals()["fset"] = fset
globals()["line"] = line
globals()["mode"] = mode
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
globals()["polygon"] = polygon

# Input

def btn(x, p=0):
    if type(x) == int:
        return px8_input.btn(x, p)
    return px8_input.btn2(ord(x))

def btnp(x, p=0):
    if type(x) == int:
        return px8_input.btnp(x, p)
    return px8_input.btnp2(ord(x))

def mouse_x():
    return px8_input.btn_mouse(0)

def mouse_y():
    return px8_input.btn_mouse(1)

def mouse_state():
    return px8_input.btn_mouse_state()

def mouse_statep():
    return px8_input.btn_mouse_statep()

globals()["btn"] = btn
globals()["btnp"] = btnp

globals()["mouse_x"] = mouse_x
globals()["mouse_y"] = mouse_y
globals()["mouse_state"] = mouse_state
globals()["mouse_statep"] = mouse_statep

# Map

def spr_map(cel_x, cel_y, sx, sy, cel_w, cel_h, layer=0):
    px8_map.spr_map(cel_x, cel_y, sx, sy, cel_w, cel_h, layer)

def mget(x, y):
    return px8_map.mget(math.floor(x), math.floor(y))

def mset(x, y, v):
    px8_map.mset(math.floor(x), math.floor(y), math.floor(v))

globals()["spr_map"] = spr_map
globals()["mget"] = mget
globals()["mset"] = mset

# Math
def atan2(x, y):
    v = math.atan2(x,y)
    return (((v - math.pi) / (math.pi*2)) + 0.25) % 1.0

def cos(x):
    return math.cos((x or 0)*(math.pi*2))

def sin(x):
    return math.sin(-(x or 0)*(math.pi*2))

def flr(x):
    return math.floor(x)

def rnd(x):
    return random.random() * x

def srand(x):
    return random.seed(x)

def mid(x,y,z):
    x = x or 0
    y = y or 0
    z = z or 0
    return x > y and x or y > z and z or y

def bxor(a,b):
    return int(a) ^ int(b)

globals()["atan2"] = atan2
globals()["ceil"] = math.ceil
globals()["cos"] = cos
globals()["sin"] = sin
globals()["flr"] = flr
globals()["rnd"] = rnd
globals()["sqrt"] = math.sqrt
globals()["mid"] = mid
globals()["bxor"] = bxor

# Memory
def memcpy(dest_addr, source_addr, len_buff):
    px8_mem.memcpy(dest_addr, source_addr, len_buff)

globals()["memcpy"] = memcpy

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


# Noise
def noise(x, y, z):
    return px8_noise.noise(x, y, z)
def noise_set_seed(seed):
    return px8_noise.noise_set_seed(seed)
globals()["noise"] = noise
globals()["noise_set_seed"] = noise_set_seed

# Others
def px8_time():
    return px8_sys.time()

def px8_time_sec():
    return px8_sys.time_sec()

def show_mouse(value=True):
    px8_sys.show_mouse(value)

globals()["px8_time"] = px8_time
globals()["px8_time_sec"] = px8_time_sec
globals()["show_mouse"] = show_mouse


########################### External functions ###########################
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



