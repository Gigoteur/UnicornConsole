# BR is an example from:
#  - duckduckontheloose (https://github.com/seleb/DuckDuckOnTheLoose)

import sys
sys.path.append("./games/BR")

CELL_SIZE = 32
CELL_BOUNDS = 128
CELL_FILL = flr(128/CELL_SIZE+1)
SEED=rnd(1)

import utils
import cells
import trees
import bushes
import buildings
utils.addglobals(globals())
cells.addglobals(globals())
trees.addglobals(globals())
bushes.addglobals(globals())
buildings.addglobals(globals())


from utils import myrange, myrange_f, ease, Vec2, frange
from biomes import Biomes
from cells import Cells
from bushes import Bushes
from trees import Trees
from buildings import Buildings

SHADOW_OFFSET=Vec2(2, 3).normalize().mul(0.2)
PERSPECTIVE_OFFSET = Vec2(64, 80)

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


class Player(object):
    def __init__(self, vec2):
        self.pos = vec2
        self.v = Vec2(0, 0)
        self.speed=Vec2(0.7,0.7)
        self.max_speed=3
        self.cur_speed=0
        self.damping=0.8
        self.a=0.75
        self.a_o = 0
        self.r = 4
        self.r2 = self.r * self.r
        self.height = 4

        self.c=[4,10,3]

        self.id_quack = sound_load("./examples/assets/wav/mygame_sfx_5.wav")

    def update(self):
        v_dif = Vec2(0, 0)
        if btn(0):
            v_dif.x -= self.speed.x
        if btn(1):
            v_dif.x += self.speed.x
        if btn(2):
            v_dif.y -= self.speed.y
        if btn(3):
            v_dif.y += self.speed.y

        if btnp(4):
            sound_play(self.id_quack)

        if abs(v_dif.x)+abs(v_dif.y) > 0.01:
            self.v._add(v_dif)
            self.a_o=self.a
            self.a=atan2(self.v.x, self.v.y)

        self.v._mul(self.damping)

        if abs(self.v.x) < 0.01:
            self.v.x = 0
        if abs(self.v.y) < 0.01:
            self.v.y = 0

        self.cur_speed=self.v.len()
        if self.cur_speed > self.max_speed:
            self.v._mul(self.max_speed/self.cur_speed)
            self.cur_speed=self.max_speed

        self.pos._add(self.v)

        self._update_boundaries()

    def _update_boundaries(self):
        x=self.pos.x/CELL_SIZE
        y=self.pos.y/CELL_SIZE
        if x > CELL_BOUNDS:
           self.v.x -= (x-CELL_BOUNDS)*2
        elif x < 0:
            self.v.x -= x*2

        if y > CELL_BOUNDS:
            self.v.y -= (y-CELL_BOUNDS)*2
        elif y < 0:
            self.v.y -= y*2

    def draw_shadow(self):
        circfill(
            self.pos.x+SHADOW_OFFSET.x*self.height,
            self.pos.y+SHADOW_OFFSET.y*self.height,
            self.r,5)

    def draw(self):
        s = self.cur_speed/self.max_speed*self.r/5+0.5
        p1=Vec2(self.pos.x,self.pos.y)
        p2=Vec2(p1.x + self.height*cos(self.a)*s, p1.y+self.height*sin(self.a)*s)

        circfill(p1.x, p1.y, self.r*3/4, self.c[0])

        circfill(p2.x, p2.y, self.r/2, self.c[1])

        p2=p1.lerp(p2,0.75)
        circfill(p2.x,p2.y,self.r/2, self.c[2])

        p2=p1.lerp(p2,0.5)
        pset(p2.x,p2.y,0)

class Camera(object):
    def __init__(self, vec2):
        self.pos = vec2
        self.c = Vec2(self.pos.x%CELL_SIZE, self.pos.y%CELL_SIZE)
        self.offset = Vec2(64, 64)
        self.sway=[0.15,0.15,50,50]
        self.pos_o = Vec2(self.pos.x, self.pos.y)
        self.v = Vec2(0, 0)

    def update(self, p_p_vec, p_v_vec):
        self.offset = p_v_vec.mul(-15).add(Vec2(64,64))
        self.pos_o = Vec2(self.pos.x, self.pos.y)
        sway=Vec2(self.sway[0]*cos(px8_time()/self.sway[2]),
                  self.sway[1]*sin(px8_time()/self.sway[3]))
        self.pos = self.pos.lerp(p_p_vec.sub(self.offset),0.1).add(sway)

        self.v = self.pos.sub(self.pos_o)

        self.c.x = self.pos.x%CELL_SIZE
        self.c.y = self.pos.y%CELL_SIZE

class Blobs(object):
    def __init__(self):
        self.blobs = {}

    def len(self):
        return len(self.blobs)

    def add_blob(self, p, r):
        key = "%d-%d-%d" % (p.x, p.y, r)
        if key not in self.blobs:
            self.blobs[key] = [p, r*r, False]

    def update(self, player):
        for blob in self.blobs.values():
            d = player.pos.sub(blob[0])
            l2=d.len2()

            if l2 < blob[1] + player.r2:
                blob[2] = True
                player.v._add(d.div(sqrt(l2)))
            else:
                blob[2] = False

class Configuration(object):
    def __init__(self, biomes, blobs):
        self.biomes = biomes
        self.blobs = blobs

        self.trees_height_range = [10,25]
        self.trees_girth_range = [4,10]
        self.trees_gap = 16

        self.bushes_height_range = [0.5,1.5]
        self.bushes_count_range = [10,30]
        self.bushes_radius_range = [1,2.5]
        self.bushes_cluster_range = [2,4]


        self.buildings_height_range = [10,35]
        self.buildings_w_range = [8,16]
        self.buildings_h_range = [8,16]
        self.buildings_colours = [8,9,6]

        self.cell_fill = CELL_FILL
        self.cell_size = CELL_SIZE
        self.shadow_offset = SHADOW_OFFSET
        self.perspective_offset = PERSPECTIVE_OFFSET

class Cloud(object):
    def __init__(self, x, y, r, height):
        self.p = Vec2(x, y)
        self.s = Vec2(x, y)
        self.ps = Vec2(x, y)
        self.r = r
        self.height = height

class Clouds(object):
    def __init__(self):
        self.count_range = random.randint(20, 40)
        self.height_range= [45,50]
        self.radius_range=[5,15]
        self.cluster_range=[5,7]
        self.size=256
        self.height_mult=0.015


        self.clouds = []

        for _ in range(0, self.count_range):
            x = rnd(self.size*2)
            y = rnd(self.size*2)
            r = 0

            for _ in range(0, random.randint(self.cluster_range[0], self.cluster_range[1])):
                c_r = myrange(self.radius_range)
                c_p=[x+myrange([1,(c_r+r)/2])-myrange([1,(c_r+r)/2]),
                     y+myrange([1,(c_r+r)/2])-myrange([1,(c_r+r)/2])]


                if rnd(1) > 0.5:
                    x=c_p[0]
                    y=c_p[1]
                    r=c_r

                self.clouds.append(Cloud(
                    c_p[0],
                    c_p[1],
                    c_r,
                    myrange(self.height_range)
                ))


    def update(self, cam):
        for cloud in self.clouds:
            cloud.p.x += 0.1-cam.v.x
            cloud.p.y += 0.1-cam.v.y

            if cloud.p.x > self.size+self.radius_range[1]:
                cloud.p.x -= self.size*2+self.radius_range[1]
            elif cloud.p.x < -self.size-self.radius_range[1]:
                cloud.p.x += self.size*2+self.radius_range[1]

            if cloud.p.y > self.size+self.radius_range[1]:
                cloud.p.y -= self.size*2+self.radius_range[1]
            elif cloud.p.y < -self.size-self.radius_range[1]:
                cloud.p.y += self.size*2+self.radius_range[1]

            cloud.s=cloud.p.sub(PERSPECTIVE_OFFSET)
            cloud.s._mul(cloud.height*self.height_mult)
            cloud.s._add(cloud.p)

            cloud.ps = cloud.p.add(SHADOW_OFFSET.mul(cloud.height))

    def draw_shadow(self):
        for cloud in self.clouds:
            circfill(cloud.ps.x, cloud.ps.y, cloud.r, 5)

    def draw(self):
        for cloud in self.clouds:
            circfill(cloud.s.x, cloud.s.y, cloud.r, 7)

class MapFormat(object):
    def __init__(self, mapstring):
        self.mapstring = mapstring

        self.mapdata = [[]] * (128)

        idx = 0
        for y in range(0, 128):
            self.mapdata[y] = [0] * 128
            for x in range(0, 128):
                self.mapdata[y][x] = int(self.mapstring[idx], 16)
                idx += 1

B = Biomes()
BLOBS = Blobs()

CONFIG = Configuration(B, BLOBS)
P = Player(Vec2(82,16).mul(32))
CAM = Camera(P.pos.sub(Vec2(64, 64+128)))
CLOUDS = Clouds()
TREES = Trees(CONFIG)
BUSHES = Bushes(CONFIG)
BUILDINGS = Buildings(CONFIG)
M = MapFormat(CreateRandomWorld())


P.pos.y -= 128

CELLS = Cells(flr(CAM.pos.x/CELL_SIZE),
              flr(CAM.pos.y/CELL_SIZE),
              M.mapdata,
              CONFIG)

def _init():
    print("CAMERA", CAM.pos.x, CAM.pos.y)
    palt(0, False)
    palt(14, True)

def _update():
    global PERSPECTIVE_OFFSET

    P.update()

    PERSPECTIVE_OFFSET = Vec2(64+sin(px8_time()/9)*4, 80+sin(px8_time()/11)*4)

    CAM.update(P.pos, P.v)
    CELLS.set_pos(Vec2(flr(CAM.pos.x/CELL_SIZE),
                       flr(CAM.pos.y/CELL_SIZE)))

    for x in range(0, CELL_FILL):
        for y in range(0, CELL_FILL):
            cell = CELLS.get_current(x, y)
            TREES.update(x, y, cell, CAM, CELLS, BLOBS)
            BUSHES.update(x, y, cell, CAM, CELLS, BLOBS)
            BUILDINGS.update(x, y, cell, CAM, CELLS, BLOBS)

    CLOUDS.update(CAM)
    BLOBS.update(P)

def _draw():
    cls()
    camera(CAM.pos.x, CAM.pos.y)

    draw_background()

    # shadow stuff
    for a in range(0, CELL_FILL):
        for b in range(0, CELL_FILL):
            cell = CELLS.get_current(a, b)
            TREES.draw(a, b, cell, CAM, True)
            BUSHES.draw(a, b, cell, CAM, True)
            BUILDINGS.draw(a, b, cell, CAM, True)

    draw_clouds(True)

    # Non shadow stuff
    for a in range(0, CELL_FILL):
        for b in range(0, CELL_FILL):
            cell = CELLS.get_current(a, b)
            TREES.draw(a, b, cell, CAM, False)
            BUSHES.draw(a, b, cell, CAM, False)
            BUILDINGS.draw(a, b, cell, CAM, False)

    draw_player()
    draw_clouds()

    camera(0, 0)
    px8_print("P X %.2f Y %.2f" % (P.pos.x, P.pos.y), 0, 112)
    px8_print("B %d C X %d Y %d" % (BLOBS.len(), flr(CAM.pos.x), flr(CAM.pos.y)), 0, 120)

def draw_clouds(shadow=False):
    camera()
    if shadow:
        CLOUDS.draw_shadow()
    else:
        CLOUDS.draw()


def draw_player():
    camera(CAM.pos.x, CAM.pos.y)

    P.draw_shadow()
    P.draw()

def draw_background():
    camera(CAM.pos.x, CAM.pos.y)

    for a in range(0, CELL_FILL):
        for b in range(0, CELL_FILL):
            x = (CELLS.pos.x+a)*CELL_SIZE
            y = (CELLS.pos.y+b)*CELL_SIZE

            cell = CELLS.get_current(a, b)
            rectfill(x, y, x+CELL_SIZE, y+CELL_SIZE, cell.color)

            biome = B.biomes.get(cell.color)
            if biome:
                srand(cell.seed)
                if biome.transition:
                    c = cell.edges[1][0]
                    if c != cell.color:
                        pal(0, c)
                        for v in range(0, flr(CELL_SIZE/8)):
                            spr(4+flr(rnd(4))*16,x+CELL_SIZE-8, y+v*8)

                    c=cell.edges[-1][0]
                    if c != cell.color:
                        pal(0, c)
                        for v in range(0, flr(CELL_SIZE/8)):
                            spr(3+flr(rnd(4))*16,x, y+v*8)

                    c=cell.edges[0][-1]
                    if c != cell.color:
                        pal(0, c)
                        for u in range(0, flr(CELL_SIZE/8)):
                            spr(2+flr(rnd(4))*16,x+u*8, y)

                    c=cell.edges[0][1]
                    if c != cell.color:
                        pal(0, c)
                        for u in range(0, flr(CELL_SIZE/8)):
                            spr(1+flr(rnd(4))*16,x+u*8, y+CELL_SIZE-8)

        pal(0,0)