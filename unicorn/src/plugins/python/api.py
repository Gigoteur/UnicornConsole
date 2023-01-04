import math
import random
from math import frexp, copysign
from sys import float_info

# Cart Data

# Collision
class CollisionRect(object):
    def __init__(self, x, y, w, h):
        self.x = x
        self.y = y
        self.w = w
        self.h = h

class CollisionCell(object):
    def __init__(self, itemCount, x, y):
        self.itemCount = 0
        self.x = x
        self.y = y
        self.items = {}

DELTA = 1e-10

def sign(x):
  if x > 0:
      return 1
  if x == 0:
      return 0
  return -1

def nearest(x, a, b):
    if abs(a - x) < abs(b - x):
        return a
    return b


def rect_getNearestCorner(x,y,w,h, px, py):
    return nearest(px, x, x+w), nearest(py, y, y+h)

def rect_getDiff(x1,y1,w1,h1, x2,y2,w2,h2):
    return x2 - x1 - w1, y2 - y1 - h1, w1 + w2, h1 + h2

def rect_containsPoint(x,y,w,h, px,py):
    return px - x > DELTA      and py - y > DELTA and x + w - px > DELTA  and y + h - py > DELTA

def rect_isIntersecting(x1,y1,w1,h1, x2,y2,w2,h2):
    return x1 < x2+w2 and x2 < x1+w1 and y1 < y2+h2 and y2 < y1+h1

def rect_getSquareDistance(x1,y1,w1,h1, x2,y2,w2,h2):
    dx = x1 - x2 + (w1 - w2)/2
    dy = y1 - y2 + (h1 - h2)/2
    return dx*dx + dy*dy

def rect_getSegmentIntersectionIndices(x,y,w,h, x1,y1,x2,y2, ti1,ti2):
    ti1, ti2 = ti1 or 0, ti2 or 1
    dx, dy = x2-x1, y2-y1
    nx, ny = 0, 0
    nx1, ny1, nx2, ny2 = 0,0,0,0
    p, q, r = 0, 0, 0

    for side in range(1,5):
        if  side == 1:
            nx,ny,p,q = -1,  0, -dx, x1 - x     #-- left
        elif side == 2:
            nx,ny,p,q =  1,  0,  dx, x + w - x1 #-- right
        elif side == 3:
            nx,ny,p,q =  0, -1, -dy, y1 - y     #-- top
        else:
            nx,ny,p,q =  0,  1,  dy, y + h - y1 #-- bottom

        if p == 0:
            if q <= 0:
                return None, None, None, None, None, None
        else:
            r = q / p
            if p < 0:
                if r > ti2:
                    return None, None, None, None, None, None
                elif r > ti1:
                    ti1,nx1,ny1 = r,nx,ny
            else: #-- p > 0
                if r < ti1:
                    return None, None, None, None, None, None
                elif r < ti2:
                    ti2,nx2,ny2 = r,nx,ny

    return ti1, ti2, nx1, ny1, nx2, ny2

# Adding same things like https://github.com/kikito/bump.lua
class Collisions(object):
    def __init__(self, cellsize=8):
        self.rects = {}
        self.cellsize = cellsize
        self.rows = {}
        self.nonEmptyCells = {}

    def grid_toCell(self, cellsize, x, y):
        return math.floor(x / cellsize) + 1, math.floor(y / cellsize) + 1

    def grid_toCellRect(self, cellsize, x,y,w,h):
        cx,cy = self.grid_toCell(cellsize, x, y)
        cr,cb = math.ceil((x+w) / cellsize), math.ceil((y+h) / cellsize)
        return cx, cy, cr - cx + 1, cb - cy + 1

    def rect_detectCollision(self, x1, y1, w1, h1, x2, y2, w2, h2, goalX, goalY):
        goalX = goalX or x1
        goalY = goalY or y1

        dx, dy = goalX - x1, goalY - y1
        x,y,w,h = rect_getDiff(x1,y1,w1,h1, x2,y2,w2,h2)
        #print(x, y, w, h)

        ti = None
        overlaps = None
        if rect_containsPoint(x,y,w,h, 0,0):
            px, py = rect_getNearestCorner(x,y,w,h, 0, 0)
            wi, hi = min(w1, abs(px)), min(h1, abs(py)) # -- area of intersection
            ti = -wi * hi
            overlaps = True
        else:
            ti1, ti2, nx1, ny1, _, _ = rect_getSegmentIntersectionIndices(x,y,w,h, 0,0,dx,dy, -math.inf, math.inf)

            if ti1 and (ti1 < 1) and (abs(ti1 - ti2) >= DELTA) and (0 < ti1 + DELTA or 0 == ti1 and ti2 > 0):
                ti, nx, ny = ti1, nx1, ny1
                overlaps = False

        #print(ti)
        
        if not ti:
            return None

        if overlaps:
            if dx == 0 and dy == 0:
                px, py = rect_getNearestCorner(x,y,w,h, 0,0)
                if abs(px) < abs(py):
                    py = 0
                else:
                    px = 0
                nx, ny = sign(px), sign(py)
                tx, ty = x1 + px, y1 + py
            else:
                ti1, _, nx, ny, _, _ = rect_getSegmentIntersectionIndices(x, y, w, h, 0, 0, dx, dy, -math.inf, 1)
                if not ti1:
                    return
                tx, ty = x1 + dx * ti1, y1 + dy * ti1
        else:
            tx, ty = x1 + dx * ti, y1 + dy * ti
        
        return {'overlaps': overlaps,
                'ti': ti,
                'move': {'x': dx, 'y': dy},
                'normal': {'x': nx, 'y': ny},
                'touch': {'x': tx, 'y': ty},
                'itemRect': {'x': x1, 'y': y1, 'w': w1, 'h': h1},
                'otherRect': {'x': x2, 'y': y2, 'w': w2, 'h': h2}
                }

    def slide(self, col, x,y,w,h, goalX, goalY):
        goalX = goalX or x
        goalY = goalY or y

        tch, move  = col.touch, col.move
        sx, sy     = tch.x, tch.y
        if move.x != 0 or move.y != 0:
            if col.normal.x == 0:
                sx = goalX
            else:
                sy = goalY

        col.slide = {'x': sx, 'y': sy}

        x,y = tch.x, tch.y
        goalX, goalY = sx, sy
        cols, len_  = self.project(col.item, x,y,w,h, goalX, goalY)
        return goalX, goalY, cols, len_

    def addItemToCell(self, item, cx, cy):
        #print("addItemToCell", item, cx, cy)
        self.rows[cy] = self.rows.get(cy) or {}
        row = self.rows[cy]
        row[cx] = row.get(cx) or CollisionCell(itemCount = 0, x = cx, y = cy)
        cell = row[cx]
        self.nonEmptyCells[cell] = True
        if not cell.items.get(item):
            cell.items[item] = True
            cell.itemCount = cell.itemCount + 1

    def getDictItemsInCellRect(self, cl, ct, cw, ch):
        items_dict = {}
        for cy in range(ct, ct+ch):
            row = self.rows.get(cy)
            if row:
                for cx in range(cl, cl+cw):
                    cell = row.get(cx)
                    if cell and cell.itemCount > 0: # no cell.itemCount > 1 because tunneling
                        for item in cell.items:
                            items_dict[item] = True

        return items_dict

    def project(self, item, x, y, w, h, goalX, goalY):
        visited = {}
        collisions = []

        if item:
            visited[item] = True

        tl, tt = min(goalX, x), min(goalY, y)
        tr, tb = max(goalX + w, x+w), max(goalY + h, y+h)
        tw, th = tr-tl, tb-tt

        cl, ct, cw, ch = self.grid_toCellRect(self.cellsize, tl, tt, tw, th)
        #print("PROJECT grid_toCellRect", cl, ct, cw, ch)
        dictItemsInCellRect = self.getDictItemsInCellRect(cl, ct, cw, ch)
        #print("PROJECT getDictItemsInCellRect", item, dictItemsInCellRect)
        for other in dictItemsInCellRect:
            if not visited.get(other):
                 ox, oy, ow, oh = self.getRect(other)
                 col = self.rect_detectCollision(x, y, w, h, ox, oy, ow, oh, goalX, goalY)
                 if col:
                     col['other'] = other
                     collisions.append(col)

        return collisions, len(collisions)

    def getRect(self, item):
        rect = self.rects.get(item.name)
        if not rect:
            return 0, 0, 0, 0

        return rect.x, rect.y, rect.w, rect.h

    def check(self, item, goalX, goalY):
        #print("CHECK", item)
        cols = []

        x, y, w, h = self.getRect(item)
        projected_cols, projected_len = self.project(item, x, y, w, h, goalX, goalY)
        for projected_col in projected_cols:
            touch = projected_col['touch']
            move = projected_col['move']
            normal = projected_col['normal']

            sx, sy = touch['x'], touch['y']

            if move['x'] != 0 or move['y'] != 0:
                if normal['x'] == 0:
                    sx = goalX
                else:
                    sy = goalY

            goalX, goalY = sx, sy
            cols.append(projected_col)

        #print(projected_cols, projected_len)

        return goalX, goalY, cols, len(cols)

    def add(self, item, x, y, w, h):
        obj = self.rects.get(item.name)
        if obj:
            #print("Data %s is already present" % item)
            return

        print("ADD", item, x, y, w, h)

        self.rects[item.name] = CollisionRect(x, y, w, h)
        cl, ct, cw, ch = self.grid_toCellRect(self.cellsize, x, y, w, h)

        print(item, x, y, w, h, self.getRect(item))
        print(cl, ct, cw, ch)
        cy = ct
        cx = cl
        for cy in range(ct, ct+ch):
            for cx in range(cl, cl+cw):
                self.addItemToCell(item, cx, cy)

    def removeItemFromCell(self, item, cx, cy):
        #print("removeItemFromCell", item)
        row = self.rows.get(cy)
        if not row or not row.get(cx) or not row[cx].items.get(item):
            return False

        cell = row.get(cx)
        if item in cell.items:
            del cell.items[item]
        cell.itemCount = cell.itemCount - 1
        if cell.itemCount == 0:
            del self.nonEmptyCells[cell]
        return True

    def remove(self, item):
        #print("REMOVE ", item)
        x, y, w, h = self.getRect(item)

        del self.rects[item.name]

        cl,ct,cw,ch = self.grid_toCellRect(self.cellsize, x, y, w, h)
        for cy in range(ct, ct+ch-1):
            for cx in range(cl, cl+cw-1):
                self.removeItemFromCell(item, cx, cy)
    
    def update(self, item, x2, y2, w2=None, h2=None):
        x1, y1, w1, h1 = self.getRect(item)

        w2, h2 = w2 or w1, h2 or h1
        #print("UPDATE", item, x2, y2, w2, h2, x1, y1, w1, h1)

        if x1 != x2 or y1 != y2 or w1 != w2 or h1 != h2:
            cl1, ct1, cw1, ch1 = self.grid_toCellRect(self.cellsize, x1,y1,w1,h1)
            cl2, ct2, cw2, ch2 = self.grid_toCellRect(self.cellsize, x2,y2,w2,h2)
            if cl1 != cl2 or ct1 != ct2 or cw1 != cw2 or ch1 != ch2:
                cr1, cb1 = cl1+cw1-1, ct1+ch1-1
                cr2, cb2 = cl2+cw2-1, ct2+ch2-1
                for cy in range(ct1, cb1):
                    cyOut = cy < ct2 or cy > cb2
                    for cx in range(cl1, cr1):
                        if cyOut or cx < cl2 or cx > cr2:
                            self.removeItemFromCell(item, cx, cy)

            
                for cy in range(ct2, cb2):
                    cyOut = cy < ct1 or cy > cb1
                    for cx in range(cl2, cr2):
                        if cyOut or cx < cl1 or cx > cr1:
                            self.addItemToCell(item, cx, cy)
        
            rect = self.rects.get(item.name)
            rect.x, rect.y, rect.w, rect.h = x2, y2, w2, h2
    
    def draw(self, camera_x=0, camera_y=0):
        for rect_ in self.rects.values():
            off_x = rect_.x - camera_x
            off_y= rect_.y - camera_y
            rect(off_x, off_y, off_x + rect_.w, off_y+rect_.h, 7)

PX8Collision = Collisions()

def world_add(name, x, y, w, h):
    global PX8Collision
    PX8Collision.add(name, x, y, w, h)

def world_check(item, goalX, goalY):
    global PX8Collision
    actualX, actualY, cols, len_cols = PX8Collision.check(item, goalX, goalY)
    return actualX, actualY, cols, len_cols

def world_move(item, goalX, goalY):
    global PX8Collision
    actualX, actualY, cols, len_cols = PX8Collision.check(item, goalX, goalY)
    PX8Collision.update(item, actualX, actualY)

    return actualX, actualY, cols, len_cols

def world_remove(item):
    global PX8Collision
    PX8Collision.remove(item)

def world_draw_debug(camera_x=0, camera_y=0):
    global PX8Collision
    PX8Collision.draw(camera_x, camera_y)


globals()["PX8Collision"] = PX8Collision
globals()["world_add"] = world_add
globals()["world_check"] = world_check
globals()["world_move"] = world_move
globals()["world_remove"] = world_remove
globals()["world_draw_debug"] = world_draw_debug

# Graphics

def mode_width():
    return unicorn_graphic.mode_width()

def mode_height():
    return unicorn_graphic.mode_height()


def camera(x=-1, y=-1):
    unicorn_graphic.camera(flr(x), flr(y))


def circ(x, y, r, color=-1):
    unicorn_graphic.circ(math.floor(x), math.floor(y),
                         math.floor(r), math.floor(color))


def circfill(x, y, r, color=-1):
    unicorn_graphic.circfill(math.floor(x), math.floor(y),
                             math.floor(r), math.floor(color))


def clip(x=-1, y=-1, w=-1, h=-1):
    unicorn_graphic.clip(math.floor(x), math.floor(y),
                         math.floor(w), math.floor(h))


def cls(value=-1):
    unicorn_graphic.cls(value)


def color(col):
    unicorn_graphic.color(col)


def ellipse(x, y, rx, ry, color=-1):
    unicorn_graphic.ellipse(math.floor(x), math.floor(y),
                            math.floor(rx), math.floor(ry),
                            math.floor(color))


def ellipsefill(x, y, rx, ry, color=-1):
    unicorn_graphic.ellipsefill(math.floor(x), math.floor(y),
                                math.floor(rx), math.floor(ry),
                                math.floor(color))


def fget(idx_sprite, flag=-1):
    if flag == -1:
        unicorn_graphic.fget_all(idx_sprite)
    return unicorn_graphic.fget(idx_sprite, flag)


def font(name="pico8"):
    unicorn_graphic.font(name)


def fset(idx_sprite, flag, value=-1):
    if value == -1:
        unicorn_graphic.fset_all(flag)
    else:
        unicorn_graphic.fset(idx_sprite, flag, value)


def line(x1, y1, x2, y2, color=-1):
    unicorn_graphic.line(math.floor(x1), math.floor(y1),
                         math.floor(x2), math.floor(y2),
                         math.floor(color))


def pal(c0=-1, c1=-1, p=0):
    unicorn_graphic.pal(math.floor(c0), math.floor(c1))


def palt(c=-1, t=False):
    unicorn_graphic.palt(math.floor(c), t)


def pget(x, y):
    return unicorn_graphic.pget(math.floor(x), math.floor(y))


def pset(x, y, color):
    unicorn_graphic.pset(math.floor(x), math.floor(y), math.floor(color))


def unicorn_print(str, x=-1, y=-1, col=-1):
    return unicorn_graphic.print(str, math.floor(x), math.floor(y), math.floor(col))


def rect(x1, y1, x2, y2, color=-1):
    unicorn_graphic.rect(math.floor(x1), math.floor(y1),
                         math.floor(x2), math.floor(y2),
                         math.floor(color))


def rectfill(x1, y1, x2, y2, color=-1):
    unicorn_graphic.rectfill(math.floor(x1), math.floor(y1),
                             math.floor(x2), math.floor(y2),
                             math.floor(color))


def sget(x, y):
    return unicorn_graphic.sget(x, y)


def spr_reg(n, d, width, height):
    return unicorn_graphic.spr_reg(n, d, width, height)


def spr(n, x, y, w=1, h=1, flip_x=False, flip_y=False, angle=0, zoom=1.0, dynamic=False):
    unicorn_graphic.spr(math.floor(n), math.floor(x), math.floor(y),
                        math.floor(w), math.floor(h), flip_x, flip_y,
                        angle, zoom, dynamic)


def sset(x, y, c=-1):
    unicorn_graphic.sset(x, y, c)


def sspr(sx, sy, sw, sh, dx, dy, dw=-1, dh=-1, flip_x=False, flip_y=False):
    if dw == -1:
        dw = sw

    if dh == -1:
        dh = sh

    unicorn_graphic.sspr(sx, sy, sw, sh, dx, dy, dw, dh, flip_x, flip_y)


def sspr_rotazoom(idx_sprite, sx, sy, sw, sh, dx, dy,
                  angle=0.0, zoom=1.0,
                  flip_x=False, flip_y=False):
    return unicorn_graphic.sspr_rotazoom(idx_sprite,
                                         sx, sy,
                                         sw, sh,
                                         dx, dy,
                                         angle, zoom,
                                         flip_x, flip_y)


def trigon(x1, y1, x2, y2, x3, y3, color):
    unicorn_graphic.trigon(math.floor(x1), math.floor(y1),
                           math.floor(x2), math.floor(y2),
                           math.floor(x3), math.floor(y3),
                           color)


def polygon(x, y, color):
    unicorn_graphic.polygon(x, y, color)


# Palette

def palette(col, r, g, b):
    unicorn_graphic.set_color_palette(col, r, g, b)


def palette_hexa(col, value):
    r = (value & 0xFF0000) >> 16
    g = (value & 0x00FF00) >> 8
    b = (value & 0x0000FF)
    unicorn_graphic.set_color_palette(col, r, g, b)


def palette_reset():
    unicorn_graphic.reset_palette()


def palette_switch(name):
    pass
    #unicorn_graphic.switch(name)


globals()["mode_width"] = mode_width
globals()["mode_height"] = mode_height
globals()["palette"] = palette
globals()["palette_hexa"] = palette_hexa
globals()["palette_reset"] = palette_reset
globals()["palette_switch"] = palette_switch

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
globals()["pal"] = pal
globals()["palt"] = palt
globals()["pset"] = pset
globals()["pget"] = pget
globals()["unicorn_print"] = unicorn_print
globals()["rect"] = rect
globals()["rectfill"] = rectfill
globals()["sget"] = sget
globals()["spr"] = spr
globals()["spr_reg"] = spr_reg
globals()["sset"] = sset
globals()["sspr"] = sspr
globals()["sspr_rotazoom"] = sspr_rotazoom
globals()["trigon"] = trigon
globals()["polygon"] = polygon

# Input


def btn(x, p=0):
    if type(x) == int:
        return unicorn_input.btn(x, p)
    return unicorn_input.btn2(ord(x))


def btnp(x, p=0):
    if type(x) == int:
        return unicorn_input.btnp(x, p)
    return unicorn_input.btnp2(ord(x))


def mouse_x():
    return unicorn_input.btn_mouse(0, 0)


def mouse_y():
    return unicorn_input.btn_mouse(1, 0)


def mouse_state():
    return unicorn_input.btn_mouse_state(0)


def mouse_statep():
    return unicorn_input.btn_mouse_statep(0)

def mouse_left_state():
    return unicorn_input.btn_mouse_state(0) & 0x000000FF


globals()["btn"] = btn
globals()["btnp"] = btnp

globals()["mouse_x"] = mouse_x
globals()["mouse_y"] = mouse_y
globals()["mouse_state"] = mouse_state
globals()["mouse_statep"] = mouse_statep

# Map


def mapdraw(cel_x, cel_y, sx, sy, cel_w, cel_h, layer=0):
    unicorn_map.mapdraw(cel_x, cel_y, sx, sy, cel_w, cel_h, layer)


def mget(x, y):
    return unicorn_map.mget(math.floor(x), math.floor(y))


def mset(x, y, v):
    unicorn_map.mset(math.floor(x), math.floor(y), math.floor(v))


globals()["mapdraw"] = mapdraw
globals()["mget"] = mget
globals()["mset"] = mset

# Math


def atan2(x, y):
    v = math.atan2(x,y)
    return (((v - math.pi) / (math.pi * 2)) + 0.25) % 1.0


def cos(x):
    return math.cos((x or 0) * (math.pi * 2))


def sin(x):
    return math.sin(-(x or 0) * (math.pi * 2))


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
    unicorn_mem.memcpy(dest_addr, source_addr, math.floor(len_buff))


globals()["memcpy"] = memcpy