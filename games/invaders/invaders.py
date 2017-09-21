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

class Bullet(object):
    def __init__(self, sp, x, y, dx, dy):
        self.sp = sp
        self.x = x
        self.y = y
        self.dx = dx
        self.dy = dy

    def update(self):
        self.x+=self.dx
        self.y+=self.dy

    def draw(self):
        spr(self.sp, self.x, self.y)

class Bullets(object):
    def __init__(self):
        self.bullets = []

    def add(self, sp, x, y, dx, dy):
        self.bullets.append(Bullet(sp, x, y, dx, dy))

    def update(self):
        to_del = []
        for k, b in enumerate(self.bullets):
            b.update()
            if b.x < 0 or b.x > SIZE_X or b.y < 0 or b.y > SIZE_Y:
               to_del.append(b)

        # delete bullets outside the screen
        for remove_element in to_del:
            self.bullets.pop(self.bullets.index(remove_element))

    def draw(self):
        for b in self.bullets:
            b.draw()

class Ship(object):
    def __init__(self, x, y, sp, h):
        self.x = x
        self.y = y
        self.sp = sp
        self.h = h
        self.max_h = h
        self.t = 0

    def update(self):
        if(self.t%6<3):
            self.sp=1
        else:
            self.sp=2

        self.t += 1

        if btn(0):
            self.x-=2
        if btn(1):
            self.x+=2
        if btn(2):
            self.y-=2
        if btn(3):
            self.y+=2

    def draw(self):
        spr(self.sp,self.x,self.y)

        for i in range(0, self.max_h):
            if i <= self.h:
                spr(33,SIZE_X-32+6*i,3)
            else:
                spr(34,SIZE_X-32+6*i,3)

class Invaders(object):
    def __init__(self, background):
        self.ship = Ship(sp=1, x=60, y=60, h=4)
        self.background = background()
        self.bullets = Bullets()

    def update(self):
        self.background.update()
        self.bullets.update()
        self.ship.update()

        if btnp(4):
            self.bullets.add(3, self.ship.x, self.ship.y, 0, -3)

    def draw(self):
        cls()
        self.background.draw()
        self.ship.draw()
        self.bullets.draw()

        self.draw_debug()

    def draw_debug(self):
        px8_print("BULLETS %d" % len(self.bullets.bullets), 0, SIZE_X - 8, 7)

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

    
