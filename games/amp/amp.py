# Original code from mhughson
# http://www.lexaloffle.com/bbs/?tid=28793
import math

class Vec2(object):
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def get_length(self):
        return math.sqrt(self.x^2+self.y^2)

class Camera(object):
    def __init__(self, target):
        self.init(target)

    def init(self, target):
        self.tar = target
        self.pos = Vec2(target.x, target.y)

        self.pull_threshold=16

        #--min and max positions of camera.
        #--the edges of the level.
        self.pos_min=Vec2(64,64)
        self.pos_max=Vec2(320,64)

        self.shake_remaining=0
        self.shake_force=0

    def update(self):
        #print("CAMERA UPDATE")
        self.shake_remaining=max(0,self.shake_remaining-1)

#        --follow target outside of
#        --pull range.
        if self.pull_max_x()<self.tar.x:
            self.pos.x = self.pos.x + min(self.tar.x-self.pull_max_x(),4)

        if self.pull_min_x()>self.tar.x:
            self.pos.x = self.pos.x + min(self.tar.x-self.pull_min_x(),4)

        if self.pull_max_y()<self.tar.y:
            self.pos.y = self.pos.y + min(self.tar.y-self.pull_max_y(),4)

        if self.pull_min_y()>self.tar.y:
            self.pos.y = self.pos.y + min(self.tar.y-self.pull_min_y(),4)

#        --lock to edge
        if(self.pos.x<self.pos_min.x): self.pos.x=self.pos_min.x
        if(self.pos.x>self.pos_max.x): self.pos.x=self.pos_max.x
        if(self.pos.y<self.pos_min.y): self.pos.y=self.pos_min.y
        if(self.pos.y>self.pos_max.y): self.pos.y=self.pos_max.y

    def get_pos(self):
        shk=Vec2(0,0)
        if self.shake_remaining>0:
            shk.x=rnd(self.shake_force)-(self.shake_force/2)
            shk.y=rnd(self.shake_force)-(self.shake_force/2)
        return self.pos.x-64+shk.x,self.pos.y-64+shk.y

    def pull_max_x(self):
        return self.pos.x+self.pull_threshold

    def pull_min_x(self):
        return self.pos.x-self.pull_threshold

    def pull_max_y(self):
        return self.pos.y+self.pull_threshold

    def pull_min_y(self):
        return self.pos.y-self.pull_threshold

    def shake(self,ticks,force):
        self.shake_remaining=ticks
        self.shake_force=force


class Animation(object):
    def __init__(self, list_of_animations):
        self.ticks = list_of_animations[0]
        self.frames = list_of_animations[1]

class Animations(object):
    def __init__(self, animations):
        self.animations = {}

        for animation in animations:
            self.animations[animation] = Animation(animations[animation])

    def get(self, anim):
        return self.animations[anim]

class JumpButton(object):
    def __init__(self):
        self.is_pressed = False
        self.is_down = False
        self.ticks_down = 0

    def update(self):
        self.is_pressed = False
        if btn(5):
            if not self.is_down:
                self.is_pressed = True
            self.is_down = True
            self.ticks_down = self.ticks_down + 1
        else:
            self.is_down = False
            self.is_pressed = False
            self.ticks_down = 0

class Actor(object):
    def __init__(self, x, y):
        self.x = x
        self.y = y

        self.dx = 0
        self.dy = 0

        self.w = 8
        self.h = 8

        self.max_dx = 1
        self.max_dy = 2

        self.jump_speed=-1.75 #,--jump veloclity
        self.acc=0.05 #,--acceleration
        self.dcc=0.8 #,--decceleration
        self.air_dcc=1 #,--air decceleration
        self.grav=0.15 #,

        self.jump_button = JumpButton()

        self.jump_hold_time=0 #,--how long jump is held
        self.min_jump_press=5 #,--min time jump can be held
        self.max_jump_press=15 #,--max time jump can be held

        self.jump_btn_released=True #,--can we jump again?
        self.grounded=False #,--on ground

        self.airtime=0

        self.anims = Animations({
            "stand": [1, [2]],
            "walk": [5, [3, 4, 5, 6]],
            "jump": [1, [1]],
            "slide": [1, [7]]
        })

        self.curanim="walk" #,--currently playing animation
        self.curframe=0 #,--curent frame of animation.
        self.animtick=0 #,--ticks until next frame should show.
        self.flipx=False

        self.col_roof = False
        self.col_side = False

    def set_anim(self, anim):
        if anim == self.curanim:
            return

        a=self.anims.get(anim)
        self.animtick=a.ticks #--ticks count down.
        self.curanim=anim
        self.curframe=1

    def collide_side(self):
        offset=self.w/3
        self.col_side = False
        for i in range(int(-self.w/3), int(self.w/3), 2):
            if fget(mget((self.x+(offset))/8,(self.y+i)/8),0):
                self.dx=0
                self.x=(flr(((self.x+(offset))/8))*8)-(offset)
                self.col_side = True
                return

            if fget(mget((self.x-(offset))/8,(self.y+i)/8),0):
                self.dx=0
                self.x=(flr((self.x-(offset))/8)*8)+8+(offset)
                self.col_side = True
                return

    def collide_roof(self):
        for i in range(int(-self.w/3), int(self.w/3), 2):
            if fget(mget((self.x+i)/8,(self.y-(self.h/2))/8),0):
                self.dy=0
                self.y=flr((self.y-(self.h/2))/8)*8+8+(self.h/2)
                self.jump_hold_time=0
                self.col_roof = True
                return

        self.col_roof = False

    def collide_floor(self):
        if self.dy <= 0:
            return False

        for i in range(int(-self.w/3), int(self.w/3), 2):
            x = int(self.x+i)
            y = int(self.y+(self.h/2))

            tile=mget((self.x+i)/8,(self.y+(self.h/2))/8)
            if fget(tile,0) or (fget(tile,1) and self.dy>=0):
                self.dy=0
                self.y=(flr((self.y+(self.h/2))/8)*8)-(self.h/2)

                self.grounded=True
                self.airtime=0
                return True
        return False

    def update(self):
        bl=btn(0)
        br=btn(1)

        if bl:
            self.dx = self.dx - self.acc
            br = False
        elif br:
            self.dx = self.dx + self.acc
        else:
            if self.grounded:
                self.dx = self.dx * self.dcc
            else:
                self.dx = self.dx * self.air_dcc

        # --move in x
        self.dx = mid(-self.max_dx,self.dx,self.max_dx)
        self.x = self.x + self.dx

        self.collide_side()

        self.jump_button.update()
        if self.jump_button.is_down:
            on_ground=(self.grounded or self.airtime<5)
            new_jump_btn=self.jump_button.ticks_down<10
            if self.jump_hold_time>0 or (on_ground and new_jump_btn):
                self.jump_hold_time = self.jump_hold_time + 1
                if self.jump_hold_time<self.max_jump_press:
                    self.dy=self.jump_speed
        else:
            self.jump_hold_time=0

        # --move in y
        self.dy = self.dy + self.grav
        self.dy=mid(-self.max_dy,self.dy,self.max_dy)
        self.y = self.y + self.dy

        if not self.collide_floor():
            self.set_anim("jump")
            self.grounded=False
            self.airtime = self.airtime + 1

        self.collide_roof()

        if self.grounded:
            if br:
                if self.dx < 0:
                    self.set_anim("slide")
                else:
                    self.set_anim("walk")

            elif bl:
                if self.dx > 0:
                    self.set_anim("slide")
                else:
                    self.set_anim("walk")
            else:
                self.set_anim("stand")

        #--flip
        if br:
            self.flipx = False
        elif bl:
            self.flipx = True

        self.animtick = self.animtick - 1
        if self.animtick<=0:
            self.curframe = self.curframe + 1
            a=self.anims.get(self.curanim)
            self.animtick=a.ticks #--reset timer
            if self.curframe > len(a.frames)-1:
                self.curframe=0#--loop

    def draw(self):
        a=self.anims.get(self.curanim)
        frame=a.frames[self.curframe]
        spr(frame,
            self.x-(self.w/2),
            self.y-(self.h/2),
            self.w/8,self.h/8,
            self.flipx,
            False)

class Jelpi(Actor):
    def __init__(self, x, y):
        super(Jelpi, self).__init__(x, y)

P = Jelpi(64, 100)
C = Camera(P)

def _init():
    P.set_anim("walk")
    C.init(P)

def _update():
    P.update()
    C.update()

    if btnp(4):
        C.shake(15,2)

def _draw_info(info, x, y, col=8):
    if info:
        rectfill(x, y, x+3, y+3, col)
    else:
        rectfill(x, y, x+3, y+3, 7)

def _draw():
    cls()

    x, y = C.get_pos()
    camera(x, y)

    spr_map(0,0,0,0,128,128)

    P.draw()

    camera(0, 0)

    _draw_info(P.grounded, 100, 124)
    _draw_info(P.col_roof, 105, 124, 2)
    _draw_info(P.col_side, 110, 124, 3)
