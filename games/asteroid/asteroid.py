
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
        px8_print("P %d" % (len(self.particles)), 0, 112, 7)

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
        self.max_speed = 1

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
            self.dx += 0.04 * cos(self.angle)
            self.dy += 0.04 * sin(self.angle)
            self.jet(1 - self.angle + 0.25)

        self.x += self.dx
        self.y += self.dy

        self.dx += self.ddx
        self.dy += self.ddy

        self.clamp_speed()

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
        j.life = 15 * rnd(1) + 15
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

        px8_print("%.02f:%.02f %.02f:%.02f" % (self.x, self.y, self.dx, self.dy), 0, 120, 7)

S = None
P = None

def _init():
    global S, P
    a = rnd(1)
    P = Particles()
    S = Ship(P, 64, 64, a + 0.5)

def _update():
    global S, P

    P.update()
    S.update()

def _draw():
    global S

    cls()

    P.draw()
    S.draw()