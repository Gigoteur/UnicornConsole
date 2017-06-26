import socket

import sys
sys.path.append("./games/asteroid")
import packets, packet_types
import utils
import maps
import biomes

utils.addglobals(globals())
maps.addglobals(globals())

from utils import Vec2
JET_SOUND = "games/asteroid/assets/jet.wav"

CELL_SIZE = 32
CELL_BOUNDS = 256
CELL_FILL = flr(256/CELL_SIZE+1)
SEED=rnd(1)

def local_noise(nx, ny, nz=0.0, freq=10, zoom=300.0):
    return noise((freq*nx)/zoom, (freq*ny)/zoom, nz) / 2.0 + 1.0

def CreateRandomWorld():
    noise_set_seed(1)

    dynamic_map = [""] * (CELL_BOUNDS*CELL_BOUNDS)

    z = rnd(5)
    for x in range(0, CELL_BOUNDS):
        for y in range(0, CELL_BOUNDS):
            value = min(15, flr(local_noise(x, y, z)/0.06666666666666667))
            dynamic_map[x+y*CELL_BOUNDS] = "%x" % value

    return ''.join(dynamic_map)

class NetworkClient(object):
    def __init__(self, host, port):
        self.host = host
        self.port = port

    def connect(self):
        self.s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.s.connect((self.host, self.port))


class Stars(object):
    def __init__(self):
        self.stars = []
        for i in range(0, 20):
            x = flr(rnd(1) * 128)
            y = flr(rnd(1) * 128)
            self.stars.append((x, y))

    def draw(self):
        for star in self.stars:
            x = star[0] % 128
            y = star[1] % 128

            rectfill(x, y, x, y, 12)

class Particle(object):
    def __init__(self, x, y):
        self.x = x
        self.y = y
        self.dx = 0
        self.dy = 0
        self.drag = 0
        self.life = 30
        self.c = 8 * rnd(1)
        self.dc = 1
        self.size = 0

    def draw(self):
        x = self.x + self.size / 2
        y = self.y + self.size / 2

        color = self.c + self.dc * 8
        rectfill(x, y, x + self.size, y + self.size, color)

    def update(self):
        self.dx -= self.dx * self.drag
        self.dy -= self.dy * self.drag
        self.x += self.dx
        self.y += self.dy
        if self.dc > 0:
            self.c = (self.c + self.dc) % 8
        self.life -= 1

        if self.life < 0:
            return False

        return True

class Particles(object):
    def __init__(self):
        self.particles = []

    def add(self, x, y):
        p = Particle(x, y)
        self.particles.append(p)
        return p

    def update(self):
        to_del = []

        for p in self.particles:
            if not p.update():
                to_del.append(p)

        for remove_element in to_del:
            self.particles.pop(self.particles.index(remove_element))

    def draw(self):
        for p in self.particles:
            p.draw()

    def debug(self):
        px8_print("P %d" % (len(self.particles)), 0, 240, 7)

class Ship(object):
    def __init__(self, particles, x, y, angle):
        self.particles = particles
        self.x = x
        self.y = y
        self.angle = angle
        self.dx = 0
        self.dy = 0
        self.ddx = 0
        self.ddy = 0
        self.jet_timer = 2
        self.reverse = False
        self.max_speed = 3

    def update(self):
        self.ddx = -0.01 * self.dx
        self.ddy = -0.01 * self.dy

        if btn(1):
            self.angle -= 1 / 64
        if btn(0):
            self.angle += 1 / 64

        if btn(3):
            self.dx -= 0.02 * cos(self.angle)
            self.dy -= 0.02 * sin(self.angle)

        if btn(2):
            sound_play(JET_SOUND)
            self.dx += 0.04 * cos(self.angle)
            self.dy += 0.04 * sin(self.angle)
            self.jet(1 - self.angle + 0.25)

        self.x += self.dx
        self.y += self.dy

        self.dx += self.ddx
        self.dy += self.ddy

        self.clamp_speed()
        self._update_boundaries()

    def _update_boundaries(self):
        x=self.x/CELL_SIZE
        y=self.y/CELL_SIZE
        if x > CELL_BOUNDS:
            self.dx -= (x-CELL_BOUNDS)*2
        elif x < 0:
            self.dx -= x*2

        if y > CELL_BOUNDS:
            self.dy -= (y-CELL_BOUNDS)*2
        elif y < 0:
            self.dy -= y*2

    def clamp_speed(self):
        l = sqrt(self.dx * self.dx + self.dy * self.dy)
        if l > self.max_speed:
            self.dx = self.max_speed * self.dx / l
            self.dy = self.max_speed * self.dy / l

    def jet(self, a):
        self.jet_timer = (self.jet_timer + 1) % 3
        if self.jet_timer != 0:
            return

        j = self.particles.add(self.x + 4, self.y + 4)

        a += (rnd(1) - 0.5) * 0.15
        j.dx = sin(a) + self.dx
        j.dy = cos(a) + self.dy
        j.life = rnd(30) * rnd(1) + 15
        j.drag = 0.04

    def draw(self):
        nx = self.x + 4
        ny = self.y + 4
        sz = 2

        x1 = nx + sz * cos(self.angle - 0.4)
        y1 = ny + sz * sin(self.angle - 0.4)
        x2 = nx + sz * cos(self.angle)
        y2 = ny + sz * sin(self.angle)
        x3 = nx + sz * cos(self.angle + 0.4)
        y3 = ny + sz * sin(self.angle + 0.4)

        color(7)
        line(x1, y1, x2, y2)
        line(x2, y2, x3, y3)

    def debug(self):
        px8_print("%.02f:%.02f %.02f:%.02f" % (self.x, self.y, self.dx, self.dy), 0, 248, 7)

class Camera(object):
    def __init__(self, vec2):
        self.pos = vec2
        self.c = Vec2(self.pos.x%CELL_SIZE, self.pos.y%CELL_SIZE)
        self.offset = Vec2(128, 128)
        self.sway=[0.15,0.15,50,50]
        self.pos_o = Vec2(self.pos.x, self.pos.y)
        self.v = Vec2(0, 0)

    def update(self, p_p_vec, p_v_vec):
        self.offset = p_v_vec.mul(-15).add(Vec2(128,128))
        self.pos_o = Vec2(self.pos.x, self.pos.y)
        sway=Vec2(self.sway[0]*cos(px8_time()/self.sway[2]),
                  self.sway[1]*sin(px8_time()/self.sway[3]))
        self.pos = self.pos.lerp(p_p_vec.sub(self.offset),0.1).add(sway)

        self.v = self.pos.sub(self.pos_o)

        self.c.x = self.pos.x%CELL_SIZE
        self.c.y = self.pos.y%CELL_SIZE

class Configuration(object):
    def __init__(self, biomes):
        self.biomes = biomes
        self.cell_fill = CELL_FILL
        self.cell_size = CELL_SIZE
        self.cell_bounds = CELL_BOUNDS
        self.seed = SEED

S = None
P = None
CAM = None
N = None
CELLS = None
STARS = None
M = maps.MapFormat(CreateRandomWorld())
B = biomes.Biomes()
CONFIG = Configuration(B)

def _init():
    show_mouse(True)
    mode(256, 256)

    sound_load(JET_SOUND)
    global S, P, CAM, N, CELLS, STARS
    N = NetworkClient("localhost", 9000)
    #N.connect()

    a = rnd(1)
    STARS = Stars()
    P = Particles()
    S = Ship(P, 64, 64, a + 0.5)
    CAM = Camera(Vec2(S.x, S.y))
    CELLS = maps.Cells(flr(CAM.pos.x/CELL_SIZE),
                       flr(CAM.pos.y/CELL_SIZE),
                       M.mapdata,
                       CONFIG)

def _update():
    global S, P, CAM, CELLS

    P.update()
    S.update()
    CAM.update(Vec2(S.x, S.y), Vec2(S.dx, S.dy))
    CELLS.set_pos(Vec2(flr(CAM.pos.x/CELL_SIZE),
                       flr(CAM.pos.y/CELL_SIZE)))

def _draw():
    global S, P, CAM, CELLS, STARS

    cls()

    camera(CAM.pos.x, CAM.pos.y)

    for a in range(0, CELL_FILL):
        for b in range(0, CELL_FILL):
            x = (CELLS.pos.x+a)*CELL_SIZE
            y = (CELLS.pos.y+b)*CELL_SIZE

            cell = CELLS.get_current(a, b)
            rectfill(x, y, x+CELL_SIZE, y+CELL_SIZE, cell.color)

    P.draw()
    S.draw()

    camera()

    #STARS.draw()

    P.debug()
    S.debug()