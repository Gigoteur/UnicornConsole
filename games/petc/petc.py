################################### Globals/Configuration #######################################

SIZE_X = 256
SIZE_Y = 256

CELL_SIZE = 32
CELL_BOUNDS = SIZE_X
CELL_FILL = flr(SIZE_X/CELL_SIZE+1)
SEED=rnd(1)

print("BOUNDS ", CELL_BOUNDS)
print("FILL ", CELL_FILL)

################################# Utils #######################################
def frange(start, stop, step):
    return [x*step+start for x in range(0,round(abs((stop-start)/step)+0.5001),
                                        int((stop-start)/step<0)*-2+1)]

def myrange(x):
    return random.randint(flr(x[0]), flr(x[1]))

def myrange_f(x):
    return random.uniform(x[0], x[1])

def lerp(f,to,t):
    return f+t*(to-f)

def ease(t):
    if t >= 0.5:
        return (t-1)*(2*t-2)*(2*t-2)+1
    return 4*t*t*t

class Vec2(object):
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def len2(self):
        return self.x * self.x + self.y * self.y

    def len(self):
        return sqrt(self.x*self.x+self.y*self.y)

    def _set(self, vec2):
        self.x = vec2.x
        self.y = vec2.y

    def _add(self, b):
        self.x = self.x + b.x
        self.y = self.y + b.y

    def _mul(self, s):
        self.x = self.x * s
        self.y = self.y * s

    def div(self, s):
        if s != 0:
            return Vec2(self.x/s, self.y/s)
        return Vec2(0, 0)

    def mul(self, s):
        return Vec2(self.x * s, self.y * s)

    def sub(self, b):
        return Vec2(self.x - b.x, self.y - b.y)

    def add(self, b):
        return Vec2(self.x + b.x, self.y + b.y)

    def lerp(self, b, t):
        return Vec2(lerp(self.x,b.x,t), lerp(self.y, b.y, t))

    def normalize(self):
        return self.div(self.len())

    def dist(self, b):
        return self.sub(b).len()
    
SHADOW_OFFSET = Vec2(2, 3).normalize().mul(0.2)
PERSPECTIVE_OFFSET = Vec2(64, 80)

class Configuration(object):
    def __init__(self, biomes, blobs):
        self.biomes = biomes
        self.blobs = blobs

        self.trees_height_range = [10,25]
        self.trees_girth_range = [4, 10]
        self.trees_gap = 16

        self.bushes_height_range = [0.5,1.5]
        self.bushes_count_range = [10,30]
        self.bushes_radius_range = [1,2.5]
        self.bushes_cluster_range = [2, 4]


        self.buildings_height_range = [2, 15]
        self.buildings_w_range = [8, 16]
        self.buildings_h_range = [8, 16]
        self.buildings_colours = [8, 9, 6]

        self.cell_fill = CELL_FILL
        self.cell_size = CELL_SIZE
        self.shadow_offset = SHADOW_OFFSET
        self.perspective_offset = PERSPECTIVE_OFFSET

################################# Trees #######################################
class Tree(object):
    def __init__(self, pos, height, girth, leaves):
        self.pos = pos
        self.height = height
        self.girth = girth
        self.leaves = leaves
        self.s = Vec2(pos.x, pos.y)

class Trees(object):
    def __init__(self, config):
        self.trees = {}
        self.config = config
        self.cell_size = self.config.cell_size

    def update(self, x, y, cell, cam, cells, blobs):
        trees = cell.trees

        cellp = Vec2(
            cam.pos.x%self.cell_size-x*self.cell_size,
            cam.pos.y%self.cell_size-y*self.cell_size
        )

        for tree in trees:
            tree.s = tree.pos.sub(cellp.add(self.config.perspective_offset))
            tree.s._mul(tree.height*0.015)
            tree.s._add(tree.pos)

            leaves_0 = tree.pos.lerp(tree.s,0.5)
            leaves_1 = tree.pos.lerp(tree.s,0.75)
            leaves_2 = tree.s
            tree.leaves[0] = [leaves_0.x, leaves_0.y]
            tree.leaves[1] = [leaves_1.x, leaves_1.y]
            tree.leaves[2] = [leaves_2.x, leaves_2.y]

            tree_pos = Vec2((cells.pos.x+x) * self.cell_size, (cells.pos.y+y)*self.cell_size).add(tree.pos)
            #blobs.add_blob(tree_pos, tree.girth)
            tree.name = str(tree)
            world_add(tree, tree_pos.x, tree_pos.y, 3, 3)

    def draw(self, a, b, cell, cam, shadow):
        camera(
            cam.c.x-a*self.cell_size,
            cam.c.y-b*self.cell_size
        )
        if cell.trees:
            if shadow:
                for tree in cell.trees:
                    circfill(
                        tree.pos.x+self.config.shadow_offset.x*tree.height/2,
                        tree.pos.y+self.config.shadow_offset.y*tree.height/2,
                        tree.girth,
                        5)
            else:
                for tree in cell.trees:
                    for x in range(-1,2):
                        for y in range(-1,2):
                            if abs(x)+abs(y)!=2:
                                line(tree.pos.x+x, tree.pos.y+y, tree.s.x, tree.s.y, 4)

                c=[[3,1],[11,0.7],[7,0.4]]
                for i in range(0, 3):
                    for tree in cell.trees:
                        circfill(tree.leaves[i][0], tree.leaves[i][1], tree.girth*c[i][1], c[i][0])


################################# Bushes #######################################

def CreateRandomBushProp():
    # [0.5, 0.5,[8,12,13,10]]
    return [rnd(1.0), rnd(1.0), [rnd(16), rnd(16), rnd(16), rnd(16)] ]

class Bush(object):
    def __init__(self, p, r, height, colour, bloom):
        self.pos = p
        self.r = r
        self.height = height
        self.colour = colour
        self.bloom = bloom
        self.s = Vec2(p.x, p.y)

class Bushes(object):
    def __init__(self, config):
        self.config = config

        self.cell_fill = self.config.cell_fill
        self.cell_size = self.config.cell_size

    def update(self, x, y, cell, cam, cells, blobs):
        bushes = cell.bushes
        cellp = Vec2(
            cam.pos.x%self.cell_size-x*self.cell_size,
            cam.pos.y%self.cell_size-y*self.cell_size
        )
        for bush in bushes:
            bush.s = bush.pos.sub(cellp.add(self.config.perspective_offset))
            bush.s = bush.s.mul(bush.height*0.015)
            bush.s._add(bush.pos)


    def draw(self, a, b, cell, cam, shadow):
        bushes = cell.bushes
        if bushes:
            camera(
                cam.c.x-a*self.cell_size,
                cam.c.y-b*self.cell_size
            )

            if shadow:
                for bush in bushes:
                    circfill(
                        bush.pos.x+self.config.shadow_offset.x*bush.height,
                        bush.pos.y+self.config.shadow_offset.y*bush.height,
                        bush.r,
                        5)
            else:
                for bush in bushes:
                    circfill(bush.s.x, bush.s.y, bush.r, 3)
                for bush in bushes:
                    if bush.bloom:
                        p=bush.s.add(bush.bloom)
                        pset(p.x,p.y,bush.colour)

################################# Buildings #######################################

class Building(object):
    def __init__(self, w, h, pos, height, color):
        self.w = w
        self.h = h
        self.pos = pos
        self.height = height
        self.color = color
        self.s = Vec2(0, 0)
        self.name = str(self)

class Buildings(object):
    def __init__(self, config):
        self.config = config
        self.cell_size = self.config.cell_size
        self.buildings = {}

    def update(self, x, y, cell, cam, cells, blobs):
        building = cell.building
        if building:
            cellp = Vec2(
                cam.pos.x%self.cell_size-x*self.cell_size,
                cam.pos.y%self.cell_size-y*self.cell_size
            )
            building.s = building.pos.sub(cellp.add(self.config.perspective_offset))

            s1=max(building.w,building.h)
            s2=min(building.w,building.h)
            for i in frange(-s1+s2/2,s1-s2/2,s2):
                p1 = Vec2((cells.pos.x+x)*self.cell_size, (cells.pos.y+y)*self.cell_size).add(building.pos)
                if s1 == building.w:
                    p1.x += i
                else:
                    p1.y += i
                world_add(tree, p1.x, p1.y, s2)

                blobs.add_blob(
                    p1,
                    s2
                )

            p2 = Vec2((cells.pos.x+x)*self.cell_size, (cells.pos.y+y)*self.cell_size).add(building.pos)
            if s1 == building.w:
                p2.x += s1-s2/2
            else:
                p2.y += s1-s2/2

            if p2.dist(p1) > 2:
                #world_add(tree, tree_pos.x, tree_pos.y, 3, 3)

                blobs.add_blob(
                    p2,
                    s2
                )

    def draw(self, a, b, cell, cam, shadow):
        building = cell.building
        if building:
            camera(
                cam.c.x-a*self.cell_size,
                cam.c.y-b*self.cell_size
            )

            if shadow:
                for i in frange(0, building.height/2, 4):
                    t = Vec2(building.s.x, building.s.y)
                    t._mul(i*0.015)
                    t._add(building.pos)

                    rectfill(t.x-building.w, t.y-building.h, t.x+building.w, t.y+building.h, 5)
            else:
                for i in frange(building.height/2, building.height-1, 4):
                    t = Vec2(building.s.x, building.s.y)
                    t._mul(i*0.015)
                    t._add(building.pos)

                    rectfill(t.x-building.w, t.y-building.h, t.x+building.w, t.y+building.h, 5)

                    s = building.s.mul(building.height*0.015)
                    s._add(building.pos)
                    rectfill(s.x-building.w, s.y-building.h, s.x+building.w, s.y+building.h, building.color)


################################# Cells/Maps #######################################

class Cell(object):
    def __init__(self, color):
        self.x = 0
        self.y = 0
        self.color = color
        self.seed = 0.0
        self.edges = {-1: {-1: 1, 0 : 1, 1 : 1},
                      0: {-1: 1, 0 : 1, 1 : 1},
                      1: {-1: 1, 0 : 1, 1 : 1}}
        self.trees = []
        self.bushes = []
        self.building = None
        self.init = False

class Cells(object):
    def __init__(self, x, y, mapdata, config):
        self.pos = Vec2(x, y)
        self.mapdata = mapdata
        self.config = config
        self.cell_fill = self.config.cell_fill
        self.cell_size = self.config.cell_size

        self.cells = []
        self._cache_cells = {}

        for _ in range(0, self.cell_fill*self.cell_fill):
            self.cells.append(None)

        self.set_cells()
        print("NUMBER OF CELLS", len(self.cells))

    def set_pos(self, pos):
        if self.pos.x != pos.x or self.pos.y != pos.y:
            self.pos.x = pos.x
            self.pos.y = pos.y
            self.set_cells()

    def get_cache_size(self):
        return len(self._cache_cells)

    def get_current(self, x, y):
        return self.cells[x*self.cell_fill+y]

    def get(self, x, y):
        if self.get_cache_size() > 256:
            self._cache_cells = {}

        key = "%d-%d" % (x, y)
        cell = self._cache_cells.get(key)
        if cell:
            return cell

        cell = Cell(1)
        self._cache_cells[key] = cell
        return cell

    def set_cells(self):
        for a in range(0, self.cell_fill):
            for b in range(0, self.cell_fill):
                x=flr(a+self.pos.x)
                y=flr(b+self.pos.y)

                cell = self.get(x, y)
                self.cells[a*self.cell_fill+b] = cell
                if cell.init:
                    continue

                cell.x = x
                cell.y = y
                cell.init = True

                self.set_bounds(x, y, cell)

                biome = self.config.biomes.get(cell.color)
                self.set_trees(cell, biome)
                self.set_bushes(cell, biome)
                self.set_buildings(cell, biome)

    def set_bounds(self, x, y, cell):
        if x<0 or x>CELL_BOUNDS-1 or y<0 or y>CELL_BOUNDS-1:
            cell.color = 1
        else:
            cell.color = self.mapdata[y][x]

        cell.seed=SEED+x*(CELL_BOUNDS*2)+y
        srand(cell.seed)

        for u in range(-1, 2):
            for v in range(-1, 2):
                if x+u<0 or x+u>CELL_BOUNDS-1 or y+v<0 or y+v>CELL_BOUNDS-1:
                    cell.edges[u][v]=1
                else:
                    cell.edges[u][v]=self.mapdata[y+v][x+u]

                if cell.edges[u][v]==14:
                    cell.edges[u][v]=3

                cell.edges[u][v] = cell.edges[u][v] or 1

    def set_trees(self, cell, biome):
        tree_freq=ease(myrange_f(biome.tree_range))

        if cell.color == 14:
            cell.color = 3
            height = myrange(self.config.trees_height_range)
            girth=min(self.cell_size,self.cell_size)*2/5
            p = Vec2(self.cell_size/2,
                     self.cell_size/2)
            leaves=[[0,0],[0,0],[0,0]]
            cell.trees.append(Tree(p, height, girth, leaves))
        else:
            for x in range(0, self.cell_size-self.config.trees_gap, self.config.trees_gap):
                for y in range(0, self.cell_size-self.config.trees_gap, self.config.trees_gap):
                    if rnd(1) < tree_freq:
                        height = myrange(self.config.trees_height_range)
                        girth = myrange(self.config.trees_girth_range)
                        p = Vec2(x+rnd(self.config.trees_gap),
                                 y+rnd(self.config.trees_gap))
                        leaves=[[0,0],[0,0],[0,0]]
                        tree = Tree(p, height, girth, leaves)
                        cell.trees.append(tree)
                        tree.p = Vec2(mid(tree.girth, tree.pos.x, self.cell_size - tree.girth),
                                      mid(tree.girth, tree.pos.y, self.cell_size - tree.girth))

    def set_bushes(self, cell, biome):
        # Bushes
        if rnd(1) < biome.bush_props[0]:
            x = rnd(self.cell_size)
            y = rnd(self.cell_size)
            r_add = 0
            bloom_colours = biome.bush_props[2]
            colour=bloom_colours[flr(rnd(len(bloom_colours)))%len(bloom_colours)]
            for j in range(0, myrange(self.config.bushes_cluster_range)):
                r = myrange_f(self.config.bushes_radius_range)
                height=myrange_f(self.config.bushes_height_range)
                p=Vec2(x+myrange_f([1,(r+r_add)])-myrange_f([1,(r+r_add)/2]),
                       y+myrange_f([1,(r+r_add)])-myrange_f([1,(r+r_add)/2])
                       )

                bloom = None
                if rnd(1) < biome.bush_props[1]:
                    a=rnd(1)
                    r_add=rnd(r/2.0)+r/4.0
                    bloom = Vec2(r*cos(a), r*sin(a))


                bush = Bush(p, r, height, colour, bloom)
                cell.bushes.append(bush)

    def set_buildings(self, cell, biome):
        if (len(cell.trees) + len(cell.bushes) == 0) and rnd(1) < biome.building_freq:
            cell.building = Building(
                myrange(self.config.buildings_w_range),
                myrange(self.config.buildings_h_range),
                Vec2(self.cell_size/2,self.cell_size/2),
                myrange(self.config.buildings_height_range),
                self.config.buildings_colours[flr(rnd(16))%len(self.config.buildings_colours)]
            )

def local_noise(nx, ny, nz=0.0, freq=10, zoom=100.0):
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

class MapFormat(object):
    def __init__(self, mapstring):
        self.mapstring = mapstring

        self.mapdata = [[]] * (SIZE_X)

        idx = 0
        for y in range(0, SIZE_Y):
            self.mapdata[y] = [0] * SIZE_Y
            for x in range(0, SIZE_X):
                self.mapdata[y][x] = int(self.mapstring[idx], 16)
                idx += 1

################################# Biome #######################################
class Biome(object):
    def __init__(self, colour, tree_range, bush_props, transition, footprints, foot_sfx):
        self.set(colour, tree_range, bush_props, transition, footprints, foot_sfx)
        self.building_freq = 0.0

    def set(self, colour, tree_range, bush_props, transition, footprints, foot_sfx):
        self.colour = colour
        self.tree_range = tree_range
        self.bush_props = bush_props
        self.transition = transition
        self.footprints = footprints
        self.foot_sfx = foot_sfx

class Biomes(object):
    def __init__(self):
        self.biomes = {}
        for i in range(0, 16):
            self.biomes[i] = Biome(i, [0, 0], [0, 0, [0, 0, 0, 0]], True, True, 3)

        # Biome 2
        self.biomes[2].bush_props = CreateRandomBushProp()

        # Biome 3
        self.biomes[3].tree_range = [0.25,0.3]
        self.biomes[3].bush_props = CreateRandomBushProp()

        # Biome 4
        self.biomes[4].bush_props = CreateRandomBushProp()

        # Biome 5
        self.biomes[5].bush_props = CreateRandomBushProp()

        # Biome 6
        self.biomes[6].bush_props = CreateRandomBushProp()

        # Biome 7
        self.biomes[7].tree_range = [0.0, 0.1]
        self.biomes[7].bush_props = CreateRandomBushProp()

        # Biome 8
        self.biomes[8].bush_props = CreateRandomBushProp()

        # Biome 9
        self.biomes[9].bush_props = CreateRandomBushProp()

        # Biome 10
        self.biomes[10].building_freq = 0.8

        # Biome 11
        self.biomes[11].tree_range = [0.1, 0.3]
        self.biomes[11].bush_props = CreateRandomBushProp()

        # Biome 13
        self.biomes[13].bush_props = CreateRandomBushProp()

        # Biome 14

        # Biome 15
        self.biomes[15].tree_range = [0,0.2]
        self.biomes[15].building_freq = 0.01

    def get(self, color):
        return self.biomes.get(color)

################################# Camera #######################################
class Camera(object):
    def __init__(self, vec2):
        self.pos = vec2
        self.c = Vec2(self.pos.x%CELL_SIZE, self.pos.y%CELL_SIZE)
        self.offset = Vec2(0, 0)
        self.sway=[0.15,0.15,50,50]
        self.pos_o = Vec2(self.pos.x, self.pos.y)
        self.v = Vec2(0, 0)

    def update(self, p_p_vec, p_v_vec):
        self.offset = p_v_vec.mul(-15).add(Vec2(SIZE_X/2, SIZE_Y/2))
        self.pos_o = Vec2(self.pos.x, self.pos.y)
        sway=Vec2(self.sway[0]*cos(px8_time_sec()/self.sway[2]),
                  self.sway[1]*sin(px8_time_sec()/self.sway[3]))
        self.pos = self.pos.lerp(p_p_vec.sub(self.offset),0.1).add(sway)

        self.v = self.pos.sub(self.pos_o)

        self.c.x = self.pos.x%CELL_SIZE
        self.c.y = self.pos.y%CELL_SIZE


################################# Bullet #######################################
class Bullet(object):
    def __init__(self, x=12, y=12, speed=3, angle=0, prox=0.2):
        self.x = x
        self.y = y
        self.speed = speed
        self.angle = angle
        self.prox = prox

    def update(self):
        self.x += self.dx
        self.y += self.dy

    def draw(self):
        pass

class Bullets(object):
    def __init__(self):
        self.bullets = []

    def update(self):
        for bullet in self.bullets:
            bullet.update()

    def draw(self):
        for bullet in self.bullets:
            bullet.draw()


################################# Player #######################################
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

        self.c=[4, 10, 3]

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
            pass

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


class Player2(object):
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
        self.name = 'player'

        self.c=[8, 7, 3]

        world_add(self, self.pos.x, self.pos.y, 8, 8)

    def update(self):
        v_dif = Vec2(0, 0)
        v = Vec2(self.v.x, self.v.y)

        if btn(0):
            v_dif.x -= self.speed.x
        if btn(1):
            v_dif.x += self.speed.x
        if btn(2):
            v_dif.y -= self.speed.y
        if btn(3):
            v_dif.y += self.speed.y

        if btnp(4):
            pass

        if abs(v_dif.x)+abs(v_dif.y) > 0.01:
            v._add(v_dif)
            self.a_o=self.a
            self.a=atan2(v.x, v.y)

        v._mul(self.damping)

        if abs(v.x) < 0.01:
            v.x = 0
        if abs(v.y) < 0.01:
            v.y = 0

        self.cur_speed=v.len()
        if self.cur_speed > self.max_speed:
            v._mul(self.max_speed/self.cur_speed)
            self.cur_speed=self.max_speed

        future_pos = self.pos.add(v)
        actualX, actualY, cols, len_cols = world_move(self, future_pos.x, future_pos.y)
        print(actualX, actualY, cols, len_cols)
        if cols:
            print("COLLISIONS", cols)

        self.v._set(v)
        self.pos._set(Vec2(actualX, actualY))
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
            self.pos.x+4+SHADOW_OFFSET.x*self.height,
            self.pos.y+4+SHADOW_OFFSET.y*self.height,
            self.r,5)

    def draw(self):
        s = self.cur_speed/self.max_speed*self.r/5+0.5
        p1=Vec2(self.pos.x, self.pos.y)
        p2=Vec2(p1.x + self.height*cos(self.a)*s, p1.y+self.height*sin(self.a)*s)


        circfill(p1.x+4, p1.y+4, self.r*3/4, self.c[0])

        p2=p1.lerp(p2,0.5)
        circfill(p2.x+4, p2.y+4, self.r/1.8, self.c[1])

        #p2=p1.lerp(p2,0.75)
        #circfill(p2.x,p2.y,self.r/2, self.c[2])

        p2=p1.lerp(p2,0.5)
        pset(p2.x+4, p2.y+4, 0)


################################# NPC #######################################

class NPC(object):
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
        self.name = 'npc'

        self.c=[10, 7, 3]

        world_add(self, self.pos.x, self.pos.y, 8, 8)

    def update(self):
        v_dif = Vec2(0, 0)

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

        p2=p1.lerp(p2,0.5)
        circfill(p2.x, p2.y, self.r/1.8, self.c[1])

        #p2=p1.lerp(p2,0.75)
        #circfill(p2.x,p2.y,self.r/2, self.c[2])

        p2=p1.lerp(p2,0.5)
        pset(p2.x,p2.y,0)


class NPCs(object):
    def __init__(self):
        self.npcs = [NPC(Vec2(256, 256))]

    def update(self):
        for npc in self.npcs:
            npc.update()

    def draw(self):
        for npc in self.npcs:
            npc.draw()

    def values(self):
        return self.npcs


################################# World #######################################
class Blobs(object):
    def __init__(self):
        self.blobs = {}

    def len(self):
        return len(self.blobs)

    def reset(self):
        self.blobs = {}

    def add_blob(self, p, r):
        if len(self.blobs) > CELL_FILL*CELL_FILL:
            self.reset()

        key = "%d-%d-%d" % (p.x, p.y, r)
        if key not in self.blobs:
            self.blobs[key] = [p, r*r, False]

    def update(self, players):
        for player in players:
            for blob in self.blobs.values():
                d = player.pos.sub(blob[0])
                l2 = d.len2()

                if l2 < blob[1] + player.r2:
                    blob[2] = True
                    player.v._add(d.div(sqrt(l2)))
                else:
                    blob[2] = False

class World(object):
    def __init__(self):
        self.blobs = Blobs()
        self.biomes = Biomes()
        self.config = Configuration(self.biomes, self.blobs)

        self.M = MapFormat(CreateRandomWorld())
        self.player = Player2(Vec2(128, 128))
        self.npcs = NPCs()

        self.camera = Camera(Vec2(0,0))

        self.cells = Cells(flr(self.camera.pos.x/CELL_SIZE),
                           flr(self.camera.pos.y/CELL_SIZE),
                           self.M.mapdata,
                           self.config)
        self.elements = []

        self.minicart = [[0]*32]*32
        self.refresh_minicart()

    def refresh_minicart(self):
        for x in range(0, CELL_FILL):
            for y in range(0, CELL_FILL):
                self.minicart[x][y] = 0

    def add_entity(self, element):
        self.elements.append(element(self.config))

    def update(self):        
        self.perspective_offset = Vec2(64+sin(px8_time_sec()/9)*4, 80+sin(px8_time_sec()/11)*4)

        self.player.update()
        self.npcs.update()

        self.camera.update(self.player.pos, self.player.v)
        self.cells.set_pos(Vec2(flr(self.camera.pos.x/CELL_SIZE),
                                flr(self.camera.pos.y/CELL_SIZE)))


        for x in range(0, CELL_FILL):
            for y in range(0, CELL_FILL):
                cell = self.cells.get_current(x, y)
                ## Update entities
                for element in self.elements:
                    element.update(x, y, cell, self.camera, self.cells, self.blobs)
        
        self.blobs.update(self.npcs.values() + [self.player])

    def draw(self):        
        camera(self.camera.pos.x, self.camera.pos.y)

        # Background
        self.draw_background()

        # shadow stuff
        for a in range(0, CELL_FILL):
            for b in range(0, CELL_FILL):
                cell = self.cells.get_current(a, b)
                for element in self.elements:
                    element.draw(a, b, cell, self.camera, True)

        # Player
        camera(self.camera.pos.x, self.camera.pos.y)
        self.player.draw_shadow()
        self.player.draw()

        # NPCs
        camera(self.camera.pos.x, self.camera.pos.y)
        self.npcs.draw()

        # Non shadow stuff
        for a in range(0, CELL_FILL):
            for b in range(0, CELL_FILL):
                cell = self.cells.get_current(a, b)
                for element in self.elements:
                    element.draw(a, b, cell, self.camera, False)

        self.draw_minicart()

        # Reset / Debug
        camera(0, 0)
        world_draw_debug(self.camera.pos.x, self.camera.pos.y)
        px8_print("P X %.2f Y %.2f %.2f %.2f" % (self.player.pos.x, self.player.pos.y, self.player.v.x, self.player.v.y), 0, SIZE_Y-16)
        px8_print("B %d C X %d Y %d" % (self.blobs.len(), flr(self.camera.pos.x), flr(self.camera.pos.y)), 0, SIZE_Y-8)

    def draw_minicart(self):
        camera(0, 0)
        size = 32

        pos_x = SIZE_X - size - 2
        pos_y = SIZE_Y - size - 2

        rect(pos_x, pos_y, pos_x+size+1, pos_y+size+1, 7)
        for x in range(0, size):
            for y in range(0, size):
                pset(pos_x+1+x, pos_y+1+y, self.minicart[x][y])

        entities = self.npcs.values()+[self.player]
        for player in entities:
            offset_x = flr(player.pos.x/SIZE_X)
            offset_y = flr(player.pos.y/SIZE_Y)
            
            pset(pos_x+1+offset_x, pos_y+1+offset_y, 8)

    def draw_background(self):
        camera(self.camera.pos.x, self.camera.pos.y)

        for a in range(0, CELL_FILL):
            for b in range(0, CELL_FILL):
                x = (self.cells.pos.x+a)*CELL_SIZE
                y = (self.cells.pos.y+b)*CELL_SIZE

                cell = self.cells.get_current(a, b)
                rectfill(x, y, x+CELL_SIZE, y+CELL_SIZE, cell.color)

                biome = self.biomes.biomes.get(cell.color)
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

#### Main entry points for PX8
W = None

def _init():
    global W

    mode(SIZE_X, SIZE_Y, 1)

    palt(0, False)
    palt(14, True)

    W = World()
    W.add_entity(Buildings)
    W.add_entity(Trees)
    W.add_entity(Bushes)

def _update():
    global W
    W.update()

def _draw():
    W.draw()
