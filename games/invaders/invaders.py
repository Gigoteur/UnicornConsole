class SF(object):
    def __init__(self, max_speed, scroll_speed):
        self.reset(max_speed, scroll_speed)
    
    def reset(self, max_speed, scroll_speed):
        self.lasty = -1
        self.blink = rnd(0.4)
        self.bt = self.blink
        self.speed = rnd(max_speed)
        self.x = rnd(128)
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
            if sf.y < 128:
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

    def draw(self):
        spr(self.sp,self.x,self.y)

        for i in range(0, self.max_h):
            if i <= self.h:
                spr(33,80+6*i,3)
            else:
                spr(34,80+6*i,3)

class Invaders(object):
    def __init__(self, background):
        self.ship = Ship(sp=1, x=60, y=60, h=4)
        self.background = background()

    def update(self):
        self.background.update()
        self.ship.update()

    def draw(self):
        cls()
        self.background.draw()
        self.ship.draw()

I = None
def _init():
    global I
    I = Invaders(StarsBackground)

def _update():
    global I
    I.update()

def _draw():
    global I
    I.draw()

    
