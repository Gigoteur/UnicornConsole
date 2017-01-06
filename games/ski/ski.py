import math
import random
from collections import deque

class Config(object):
    def __init__(self):
        self.speed = 0.7
        self.dist = 0.0
        self.stop = False

    def update(self):
        if not self.stop:
            self.speed += 0.003
            self.dist += self.speed

class Logo(object):
    def __init__(self):
        self.x = 34
        self.y = -150
        self.y_dest = 50
        self.y_dist = 0

class SnowParticle(object):
    def __init__(self):
        pass

    def draw(self):
        circfill(self.x, self.y, self.r, 7)


class Trail(object):
    def __init__(self, x, y):
        self.x = x
        self.y = y

class Player(object):
    def __init__(self, config):
        self.config = config
        self.frame_offset = 0
        self.x = 30
        self.y = 40
        self.dead = False

        self.trail = []
        self.add_trail()

        self.current_state = "center"

        self.state = {
            "center": [10],
            "right": [11, 27],
            "left": [12, 28],
            "dead": [18, 19, 20],
        }

    def add_trail(self):
        self.trail.append(Trail(self.x, self.y))

    def trail_update(self):
        for trail in self.trail:
            trail.y -= self.config.speed

    def trail_draw(self):
        for i in range(0, len(self.trail)):
            n = i + 1
            a = self.trail[i]
            if (i + 1) >= len(self.trail):
                b = self
            else:
                b = self.trail[i+1]

            if b:
                line(a.x + 2, a.y + 8, b.x + 2, b.y + 8, 6)
                line(a.x + 5, a.y + 8, b.x + 5, b.y + 8, 6)

    def set_dead(self):
        if not self.dead:
            self.dead = True
            self.current_state = "dead"
            self.y += 4

    def update(self, timer):
        self.trail_update()

        if not self.dead:
            if btn(0):
                self.x = self.x - 1.5
                self.current_state = "left"
                self.frame_offset = 0
                self.add_trail()
            elif btn(1):
                self.x = self.x + 1.5
                self.current_state = "right"
                self.frame_offset = 0
                self.add_trail()
            else:
                self.current_state = "center"
                self.frame_offset = 0
                if btnp(2):
                    if self.config.speed >= 0.1:
                        self.config.speed -= 0.1
                if btnp(3):
                    self.config.speed += 0.1

        if timer % 5:
            self.frame_offset = (self.frame_offset + 1) % len(self.state[self.current_state])

        if self.x < 0:
            self.x = 0
        if self.x > 120:
            self.x = 120

    def draw(self):
        self.trail_draw()

        if not self.dead:
            spr(3 + self.state[self.current_state][0], self.x, self.y + 3)

        spr(self.state[self.current_state][self.frame_offset], self.x, self.y)


class Background(object):
    def __init__(self, config):
        self.dots = []
        self.config = config

        for i in range(0, 20):
            self.dots.append([random.randint(0, 128), random.randint(0, 128)])

    def update(self):
        for dot in self.dots:
            dot[1] -= self.config.speed
            if dot[1] < 0:
                dot[0] = random.randint(0, 128)
                dot[1] = 127

    def draw(self):
        for dot in self.dots:
            rectfill(dot[0], dot[1], dot[0], dot[1], 6)


class Tree(object):
    def __init__(self, config):
        self.config = config
        self.f = random.randint(0, 4) + 5
        self.x = random.randint(0, 18) * 8
        self.y = 127 + random.randint(0, 8) * 8
        self.col = 0
        if self.f == 5:
            self.col = 3
        elif self.f == 6:
            self.col = 5

    def update(self):
        self.y -= self.config.speed

    def draw(self):
        spr(self.f, self.x, self.y)


class Peoples(object):
    def __init__(self, config, nb):
        self.config = config

        self.l = []
        for i in range(0, nb):
            self.l.append(People(config))

    def update(self):
        idx = 0
        l = []
        for people in self.l:
            if people.y < -10:
                l.append(idx)
                self.l.append(People(self.config))
            idx += 1

        deque((list.pop(self.l, i) for i in sorted(l, reverse=True)), maxlen=0)

        for people in self.l:
            people.update()


    def draw(self):
        for people in self.l:
            people.draw()

class People(object):
    def __init__(self, config):
        self.config = config
        self.dead = False
        self.x = ( random.randint(0, 7) * 8 ) + 28
        self.y = 127 + (random.randint(0, 8)) * 8
        self.f_start = 21
        self.intern_speed = -1

        self.f_offset = 0
        if random.randint(0, 3) < 1:
            self.f_offset = 16

        self.f = self.f_start
        self.anim_tick = 0

    def update(self):
        global timer

        self.y -= self.config.speed + self.intern_speed

        self.anim_tick += 0.5
        if timer % 7 == 0:
            if self.f == 21:
                self.f = 22
            else:
                self.f = 21

    def draw(self):
        spr(3, self.x, self.y + 2)
        spr(self.f + self.f_offset, self.x, self.y)

class Particle(object):
    def __init__(self, config, x, y, col):
        self.config = config
        self.x = x
        self.y = y
        self.col = col
        self.dx = random.randint(0, 2) -1
        self.dy = random.randint(0, 2) -1
        self.vx = random.randint(0, 4) +1
        self.vy = random.randint(0, 4) +1

    def update(self):
        self.vx -= 0.2
        self.vy -= 0.2

        self.x += (self.dx * self.vx)
        self.y += (self.dy * self.vx)

    def draw(self):
        rectfill(self.x, self.y, self.x + 1, self.y + 1, self.col)


class Particles(object):
    def __init__(self, config):
        self.config = config
        self.l = []

    def add(self, x, y, col, num):
        for _ in range(0, num):
            self.l.append(Particle(self.config, x, y, col))

    def update(self):
        idx = 0
        to_del = []
        for particle in self.l:
            if particle.vx < 0 or particle.vy < 0:
                to_del.append(idx)

        deque((list.pop(self.l, i) for i in sorted(to_del, reverse=True)), maxlen=0)

        for particle in self.l:
            particle.update()

    def draw(self):
        for particle in self.l:
            particle.draw()

config = Config()
logo = Logo()
background = Background(config)

snow_particles = []
players = [Player(config)]
peoples = Peoples(config, 3)
particles = Particles(config)

timer = 0
shakescreen = 0

trees = []
for i in range(0, 1):
    trees.append(Tree(config))

state = 'splash'

def _init():
    print("SKI _INIT")

def tween(current, dest, speed):
    fps = 60
    return dest * fps / speed + current

def logo_update():
    global logo
    logo.y_dist = logo.y_dest - logo.y
    logo.y = tween(logo.y, logo.y_dist, 900)

def collides(a, b):
    bx1 = b.x +2
    bx2 = b.x +6
    by1 = b.y +5
    by2 = b.y +8

    return not ((a.y+8<by1) or (a.y>by2) or	(a.x+8<bx1) or (a.x>bx2))

def _update():
    global config, state, players, background, timer, trees, shakescreen, particles

    print("SKI _UPDATE")

    timer += 1
    if state == 'splash':
        logo_update()
        if btnp(2):
            fade_out()
            state = 'main'
    else:
        background.update()
        particles.update()
        peoples.update()
        players[0].update(timer)

        config.update()

        idx = 0
        to_del = []
        for tree in trees:
            if (tree.y < -10):
                to_del.append(idx)
                trees.append(Tree(config))
            idx += 1

        deque((list.pop(trees, i) for i in sorted(to_del, reverse=True)), maxlen=0)

        for tree in trees:
            tree.update()

        for tree in trees:
            if collides(tree, players[0]):
                print("BOOM")
                if not players[0].dead:
                    shakescreen = 50
                players[0].set_dead()
                config.speed = 0
                config.stop = True
                particles.add(tree.x + 4, tree.y + 4, tree.col, 10)

                tree.y = -150

    if timer % 100 == 0:
            trees.append(Tree(config))

def fade_out(fa=0.2):
    fa=max(min(1,fa),0)
    fn=8
    pn=15
    fc=1/fn
    fi=math.floor(fa/fc)+1
    fades = [
            [1,1,1,1,0,0,0,0],
            [2,2,2,1,1,0,0,0],
            [3,3,4,5,2,1,1,0],
            [4,4,2,2,1,1,1,0],
            [5,5,2,2,1,1,1,0],
            [6,6,13,5,2,1,1,0],
            [7,7,6,13,5,2,1,0],
            [8,8,9,4,5,2,1,0],
            [9,9,4,5,2,1,1,0],
            [10,15,9,4,5,2,1,0],
            [11,11,3,4,5,2,1,0],
            [12,12,13,5,5,2,1,0],
            [13,13,5,5,2,1,1,0],
            [14,9,9,4,5,2,1,0],
            [15,14,9,4,5,2,1,0],
    ]

    for n in range(1, pn):
        pal(n,fades[n][fi],0)

def fade_out2():
    dpal=[0,1,1, 2,1,13,6,
          4,4,9,3, 13,1,13,14]

    for i in range(0, 40):
        for j in range(1, 15):
            col = j
            for k in range(1, math.floor(((i+(j%5))/4))):
                col=dpal[col]
            pal(j,col,1)


def logo_draw():
    global logo
    w = 8
    h = 4
    start = 67
    remap = 115

    for x in range(0, w):
        for y in range(0, h):
            #print(x+start + (y * 16), logo.x + (x * 8), logo.y + (y*8))
            spr(x+start + (y * 16), logo.x + (x * 8), logo.y + (y*8))

def do_shakescreen():
    global shakescreen

    shakescreen -= 1
    if shakescreen <= 0:
        camera(0,0)
    else:
        camera(random.randint(0, 4)-4, random.randint(0, 4)-4)

def _draw():
    global snow_particles, state, players, background, trees, shakescreen, config

    if state == 'splash':
        rectfill(0, 0, 127, 127, 15)
        rectfill(0, 43, 128, 44, 14)
        rectfill(0, 38, 128, 40, 14)
        rectfill(0, 0, 128, 35, 14)
        rectfill(0, 0, 128, 8, 8)
        rectfill(0, 10, 128, 11, 8)

        spr_map(16,0, 0,0, 128,128)
        spr_map(0,0, 0,0, 128,128)

        logo_draw()

        for snow_particle in snow_particles:
            snow_particle.draw()
    else:
        pal()

        if shakescreen > 0:
            do_shakescreen()
        else:
            camera(0, 0)

        rectfill(0, 0, 127, 127, 7)

        background.draw()
        peoples.draw()
        particles.draw()
        players[0].draw()

        for tree in trees:
            tree.draw()

        px8_print(str(config.dist), 110, 5, 12)