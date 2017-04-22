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
            self.biomes[i] = Biome(i, [0, 0], [0, 0, [0, 0, 0, 0]], False, True, 3)

        # Biome 3
        self.biomes[3].transition = True
        self.biomes[3].tree_range = [0.25,0.3]
        self.biomes[3].bush_props = [0.5, 0.5,[8,12,13,10]]

        # Biome 4
        self.biomes[4].transition = True

        # Biome 7
        self.biomes[7].transition = True
        self.biomes[7].tree_range = [0.0, 0.1]

        # Biome 10
        self.biomes[10].transition = True
        self.biomes[10].building_freq = 0.8

        # Biome 11
        self.biomes[11].transition = True
        self.biomes[11].tree_range = [0.1, 0.3]
        self.biomes[11].bush_props = [0.5,0.8,[8,12,13,10]]

        # Biome 14
        self.biomes[14].transition = True

        # Biome 15
        self.biomes[15].transition = True
        self.biomes[15].tree_range = [0,0.2]
        self.biomes[15].building_freq = 0.01

    def get(self, color):
        return self.biomes.get(color)