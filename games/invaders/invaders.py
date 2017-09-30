SIZE_X = 256
SIZE_Y = 256

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
    def __init__(self, idx, sp, m_x, m_y, x, y, r):
        self.name = str(self) + str(idx)
        self.sp = sp
        self.m_x = m_x
        self.m_y = m_y
        self.x = x
        self.y = y
        self.r = r
        self.die = False
        self.speed = 0.5

    def set_die(self):
        self.die = True

    def update(self, t):
        if self.die:
            return False

        future_x = self.r*sin(t/100) + self.m_x
        future_y = self.r*cos(t/100) + self.m_y + self.speed

        next_x, next_y, cols, len_cols = world_move(self, future_x, future_y)
        if cols:
            print("COLLISIONS ENEMY", cols)
            val = False
            for col in cols:
                if 'Enemy' not in col['other'].name:
                    col['other'].set_die()
                    val = True
            if val == True:
                return False
        else:
            self.m_y += self.speed

        self.x, self.y = next_x, next_y
        return True

    def draw(self):
        spr(self.sp, self.x, self.y)

class Enemies(object):
    def __init__(self, nb):
        self.enemies = []
        self.nb = nb
        self.respawn()

    def respawn(self):
        for i in range(0, self.nb):
            enemy = Enemy(idx=i, sp=17, m_x=i*16+64, m_y=60-i*16, x=80, y=64+i*16, r=32)
            self.enemies.append(enemy)
            world_add(enemy, enemy.x, enemy.y, 8, 7)

    def update(self, t):
        self.to_del = []
        for k, e in enumerate(self.enemies):
            if not e.update(t) or (e.y > SIZE_Y):
               self.to_del.append(e)

    def remove(self):
        for remove_element in self.to_del:
            enemy = self.enemies.pop(self.enemies.index(remove_element))
            world_remove(enemy)

    def draw(self):
        for e in self.enemies:
            e.draw()

    def get(self):
        return self.enemies

class Bullet(object):
    def __init__(self, name, sp, x, y, vel_inc=1.8):
        self.name = name
        self.sp = sp
        self.x = x
        self.y = y
        self.dx = 0
        self.dy = 0
        self.vel_inc = vel_inc
        self.max_inc = 2.0

    def set_die(self):
        pass

    def update(self):
        dy = self.dy - self.vel_inc

        future_x = self.x
        future_y = self.y + dy

        next_x, next_y, cols, len_cols = world_move(self, future_x, future_y)
        if cols:
            print("COLLISIONS BULLET", cols)
            for col in cols:
                col['other'].set_die()
            return False

        self.dy = dy
        self.x, self.y = next_x, next_y
        _, self.dx, self.dy = clampvec_getlen(self.dx, self.dy, self.max_inc)

        return True

    def draw(self):
        spr(self.sp, self.x, self.y)

class Bullets(object):
    def __init__(self):
        self.bullets = []
        self.idx = 0

    def add(self, sp, x, y):
        bullet = Bullet("bullet", sp, x, y)
        bullet.name = str(bullet) + str(self.idx)
        self.bullets.append(bullet)
        world_add(bullet, bullet.x, bullet.y, 2, 3)
        self.idx += 1

    def update(self):
        self.to_del = []
        for k, b in enumerate(self.bullets):
            if not b.update() or (b.x < 0 or b.x > SIZE_X or b.y < 0 or b.y > SIZE_Y):
               self.to_del.append(b)

    def remove(self):
        # delete bullets outside the screen
        for remove_element in self.to_del:
            bullet = self.bullets.pop(self.bullets.index(remove_element))
            world_remove(bullet)

    def draw(self):
        for b in self.bullets:
            b.draw()

    def get(self):
        return self.bullets

def muls(ax, ay, b):
    return ax*b, ay*b

def dot(ax, ay, bx, by):
    return ax*bx+ay*by

def clampvec_getlen(vx, vy, n):
    l = sqrt(dot(vx,vy,vx,vy))
    if l > n:
        vx, vy = muls(vx, vy, n/l)
        l = n
    return l, vx, vy

class Ship(object):
    def __init__(self, x, y, sp, life):
        self.orig_x = x
        self.orig_y = y
        self.sp = sp
        self.max_life = life
        self.name = "ship"
        self.respawn()

    def respawn(self):
        self.x = self.orig_x
        self.y = self.orig_y
        self.dx = 0
        self.dy = 0
        self.vel_inc = 1.5
        self.max_inc = 2.0
        self.life = self.max_life
        self.die = False

    def set_die(self):
        self.life -= 1
        if self.life <= 0:
            self.die = True
            self.respawn()

    def update(self, t):
        if(t%6<3):
            self.sp=1
        else:
            self.sp=2

        dx = 0
        dy = 0

        if btn(0):
            dx = self.dx - self.vel_inc
        if btn(1):
            dx = self.dx + self.vel_inc
        if btn(2):
            dy = self.dy - self.vel_inc
        if btn(3):
            dy = self.dy + self.vel_inc
        
        future_x = self.x+dx
        future_y = self.y+dy

        next_x, next_y, cols, len_cols = world_move(self, future_x, future_y)
        if cols:
            print("COLLISIONS SHIP", cols)
            for col in cols:
                col['other'].set_die()
            self.set_die()

        self.dx = dx
        self.dy = dy

        self.x, self.y = next_x, next_y
        _, self.dx, self.dy = clampvec_getlen(self.dx, self.dy, self.max_inc)

    def draw(self):
        spr(self.sp,self.x,self.y)

        for i in range(0, self.max_life):
            if i <= self.life:
                spr(33,SIZE_X-32+6*i,3)
            else:
                spr(34,SIZE_X-32+6*i,3)

class Block(object):
    def __init__(self, name, x, y, w, h):
        self.x = x
        self.y = y
        self.w = w
        self.h = h
        self.name = name

        world_add(self, x, y, w, h)

    def set_die(self):
        pass

    def update(self):
        next_x, next_y, cols, len_cols = world_check(self, self.x, self.y)
        if cols:
            print("COLLISIONS BLOCK", cols)

    def draw(self):
        rectfill(self.x, self.y, self.x+self.w, self.y+self.h, 10)

class Invaders(object):
    def __init__(self, background):
        self.ship = Ship(sp=1, x=128, y=200, life=5)
        self.background = background()
        self.bullets = Bullets()
        self.enemies = Enemies(5)
        self.block = Block("block", 128, 128, 10, 10)
        self.t = 0

        world_add(self.ship, self.ship.x, self.ship.y, 8, 8)

    def update(self):
        self.background.update()
        
        if len(self.enemies.get()) == 0:
            self.enemies.respawn()

        if btnp(4):
            self.bullets.add(3, self.ship.x+2, self.ship.y-8)

        self.enemies.update(self.t)

        self.bullets.update()
        self.block.update()
        self.ship.update(self.t)

        self.bullets.remove()
        self.enemies.remove()


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
        world_draw_debug()
        px8_print("SHIP %d %d BULLETS %d" % (self.ship.x, self.ship.y, len(self.bullets.bullets)), 0, SIZE_X - 16, 7)
        px8_print("COLLISIONS %d" % len(PX8Collision.rects), 0, SIZE_X - 8, 7)

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

    
