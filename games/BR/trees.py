addglobals = lambda x: globals().update(x)

from utils import Vec2

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

            blobs.add_blob(Vec2((cells.pos.x+x) * self.cell_size, (cells.pos.y+y)*self.cell_size).add(tree.pos), tree.girth)

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
