addglobals = lambda x: globals().update(x)

from utils import myrange, myrange_f, ease, Vec2
from bushes import Bush
from trees import Tree
from buildings import Building

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
            # Trees
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
