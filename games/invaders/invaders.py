SIZE_X = 256
SIZE_Y = 256

class Rect(object):
    def __init__(self, x, y, w, h):
        self.x = x
        self.y = y
        self.w = w
        self.h = h

class Cell(object):
    def __init__(self, itemCount, x, y):
        self.itemCount = 0
        self.x = x
        self.y = y
        self.items = {}

DELTA = 1e-10

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

# Adding same things like https://github.com/kikito/bump.lua
class Collisions(object):
    def __init__(self, cellsize=1):
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
        print(x, y, w, h)

        ti = None
        if rect_containsPoint(x,y,w,h, 0,0):
            print("ICI")
            px, py = rect_getNearestCorner(x,y,w,h, 0, 0)
            wi, hi = min(w1, abs(px)), min(h1, abs(py)) # -- area of intersection
            ti = -wi * hi
        else:
            pass
        print(ti)
        
        if not ti:
            return None

        return {'ti': ti}

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
        print("addItemToCell", item, cx, cy)
        self.rows[cy] = self.rows.get(cy) or {}
        row = self.rows[cy]
        row[cx] = row.get(cx) or Cell(itemCount = 0, x = cx, y = cy)
        cell = row[cx]
        print("CELL", cell)
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
        print("PROJECT getDictItemsInCellRect", item, dictItemsInCellRect)
        for other in dictItemsInCellRect:
            if not visited.get(other):
                 ox, oy, ow, oh = self.getRect(other)
                 print(ox, oy, ow, oh)
                 col = self.rect_detectCollision(x, y, w, h, ox, oy, ow, oh, goalX, goalY)
                 print(col)

        return collisions, len(collisions)

    def getRect(self, item):
        rect = self.rects.get(item)
        if not rect:
            return 0, 0, 0, 0

        return rect.x, rect.y, rect.w, rect.h

    def check(self, item, goalX, goalY):
        cols = []
        x, y, w, h = self.getRect(item)
        projected_cols, projected_len = self.project(item, x, y, w, h, goalX, goalY)
        #print(projected_cols, projected_len)

        return goalX, goalY, cols, len(cols)

    def add(self, item, x, y, w, h):
        obj = self.rects.get(item)
        if obj:
            print("Data %s is already present" % item)
            return

        print("ADD", item, x, y, w, h)

        self.rects[item] = Rect(x, y, w, h)
        cl, ct, cw, ch = self.grid_toCellRect(self.cellsize, x, y, w, h)

        print(item, x, y, w, h, self.getRect(item))
        print(cl, ct, cw, ch)
        cy = ct
        cx = cl
        for cy in range(ct, ct+ch):
            for cx in range(cl, cl+cw):
                self.addItemToCell(item, cx, cy)

    def removeItemFromCell(self, item, cx, cy):
        print("removeItemFromCell", item)
        row = self.rows.get(cy)
        if not row or not row.get(cx) or not row[cx].items[item]:
            return False

        cell = row.get(cx)
        del cell.items[item]
        cell.itemCount = cell.itemCount - 1
        if cell.itemCount == 0:
            del self.nonEmptyCells[cell]
        return True

    def remove(self, item):
        print("REMOVE ", item)
        x, y, w, h = self.getRect(item)

        del self.rects[item]

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
        
            rect = self.rects.get(item)
            rect.x, rect.y, rect.w, rect.h = x2, y2, w2, h2

C = Collisions()

def world_add(name, x, y, w, h):
    global C
    C.add(name, x, y, w, h)
   # world_add("A", 0, 0, 64, 256)
   #     actualX, actualY, cols, len = world_move("B", 0, 64)

def world_move(item, goalX, goalY):
    global C
    actualX, actualY, cols, len_ = C.check(item, goalX, goalY)
    C.update(item, actualX, actualY)

    return actualX, actualY, cols, len_

def world_remove(item):
    global C
    C.remove(item)

class SF(object):
    def __init__(self, max_speed, scroll_speed):
        self.reset(max_speed, scroll_speed)
    
    def reset(self, max_speed, scroll_speed):
        self.lasty = -1
        self.blink = rnd(0.4)
        self.bt = self.blink
        self.speed = rnd(max_speed)
        self.x = rnd(SIZE_X)
        self.y = -rnd(scroll_speed+self.speed)
        self.color = 5

class StarsBackground(object):
    def __init__(self):
        self.next = 0
        self.max_speed = 10
        self.one_frame = 1/60.0

        self.sf = []
        
        for i in range(0, 50):
            self.sf.append(SF(self.max_speed, self.one_frame))

    def update(self):
        for sf in self.sf:
            sf.lasty=sf.y
            if sf.y < SIZE_Y:
                sf.y+=sf.speed+self.one_frame
            else:
                sf.reset(self.max_speed, self.one_frame)

    def draw(self):
        for sf in self.sf:
            sf.bt-=self.one_frame
            if sf.bt<=0 :
                sf.bt = sf.blink
                sf.color = (sf.color==5) and 6 or 5
            line(sf.x, sf.lasty, sf.x, sf.y, sf.color)

class Enemy(object):
    def __init__(self, sp, m_x, m_y, x, y, r):
        self.sp = sp
        self.m_x = m_x
        self.m_y = m_y
        self.x = x
        self.y = y
        self.r = r

    def update(self, t):
        self.x = self.r*sin(t/50) + self.m_x
        self.y = self.r*cos(t/50) + self.m_y

    def draw(self):
        spr(self.sp, self.x, self.y)

class Enemies(object):
    def __init__(self, nb):
        self.enemies = []
        for i in range(0, nb):
            self.enemies.append(Enemy(sp=17, m_x=i*16, m_y=60-i*8, x=-32, y=-32, r=12))

    def update(self, t):
        for e in self.enemies:
            e.update(t)

    def draw(self):
        for e in self.enemies:
            e.draw()

    def get(self):
        return self.enemies

class Bullet(object):
    def __init__(self, name, sp, x, y, dx, dy):
        self.name = name
        self.sp = sp
        self.x = x
        self.y = y
        self.dx = dx
        self.dy = dy

    def update(self):
        future_x = self.x + self.dx
        future_y = self.y + self.dy

        next_x, next_y, cols, len_cols = world_move(self, future_x, future_y)
        if cols:
            print("COLLISIONS", cols)
        self.x, self.y = next_x, next_y

    def draw(self):
        spr(self.sp, self.x, self.y)

class Bullets(object):
    def __init__(self):
        self.bullets = []
        self.idx = 0

    def add(self, sp, x, y, dx, dy):
        bullet = Bullet("bullet", sp, x, y, dx, dy)
        bullet.name = str(bullet) + str(self.idx)
        self.bullets.append(bullet)
        world_add(bullet.name, bullet.x, bullet.y, 8, 8)
        self.idx += 1

    def update(self):
        to_del = []
        for k, b in enumerate(self.bullets):
            b.update()
            if b.x < 0 or b.x > SIZE_X or b.y < 0 or b.y > SIZE_Y:
               to_del.append(b)

        # delete bullets outside the screen
        for remove_element in to_del:
            bullet = self.bullets.pop(self.bullets.index(remove_element))
            world_remove(bullet.name)

    def draw(self):
        for b in self.bullets:
            b.draw()

    def get(self):
        return self.bullets

class Ship(object):
    def __init__(self, x, y, sp, h):
        self.x = x
        self.y = y
        self.sp = sp
        self.h = h
        self.max_h = h

    def update(self, t):
        if(t%6<3):
            self.sp=1
        else:
            self.sp=2

        future_x = self.x
        future_y = self.y

        if btn(0):
            future_x-=1
        if btn(1):
            future_x+=1
        if btn(2):
            future_y-=1
        if btn(3):
            future_y+=1

        next_x, next_y, cols, len_cols = world_move(self, future_x, future_y)
        if cols:
            print("COLLISIONS", cols)
        self.x, self.y = next_x, next_y

    def draw(self):
        spr(self.sp,self.x,self.y)

        for i in range(0, self.max_h):
            if i <= self.h:
                spr(33,SIZE_X-32+6*i,3)
            else:
                spr(34,SIZE_X-32+6*i,3)

class Block(object):
    def __init__(self, x, y, w, h):
        self.x = x
        self.y = y
        self.w = w
        self.h = h

        world_add(self, x, y, w, h)

    def draw(self):
        rectfill(self.x, self.y, self.x+self.w, self.y+self.h, 8)

class Invaders(object):
    def __init__(self, background):
        self.ship = Ship(sp=1, x=128, y=200, h=4)
        self.background = background()
        self.bullets = Bullets()
        self.enemies = Enemies(10)
        self.block = Block(128, 128, 10, 10)
        self.t = 0

        world_add(self.ship, self.ship.x, self.ship.y, 8, 8)

    def update(self):
        self.background.update()

        self.enemies.update(self.t)
        self.bullets.update()
        self.ship.update(self.t)

        if btnp(4):
            self.bullets.add(3, self.ship.x, self.ship.y, 0, -3)

        self.t += 1

    def draw(self):
        cls()
        self.background.draw()
        self.block.draw()

        self.enemies.draw()
        self.ship.draw()
        self.bullets.draw()

        self.draw_debug()

    def draw_debug(self):
        global C
        px8_print("BULLETS %d" % len(self.bullets.bullets), 0, SIZE_X - 16, 7)
        px8_print("COLLISIONS %d" % len(C.rects), 0, SIZE_X - 8, 7)

I = None
def _init():
    global I
    mode(SIZE_X, SIZE_Y, 1)
    I = Invaders(StarsBackground)

def _update():
    global I
    I.update()

def _draw():
    global I
    I.draw()

    
