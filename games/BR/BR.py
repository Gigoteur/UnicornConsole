# BR is a mix between
#  - duckduckontheloose (https://github.com/seleb/DuckDuckOnTheLoose)
#  - dark tomb (http://www.lexaloffle.com/bbs/?tid=28785)

MAP = "11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111cccccccccccccccccc1111111111cccccccccccccccccccccccccccccccccccccc11111cccccccccccccccccccccccccccccccccc1111111111111cccc111ccccccccccccccccccccccccccccccccccccccccccccccccc44ccccccccccccccccccccccccccccccffffffffbbbbccccbbb3333ccccc1111111111ccccccccccccc11cc1111cccccffccccccccccccccccc444433cccc33444cccce33e333333444ccccccccccbbbfffaaafffbbfbbcbbb33bb333ccc111111111ccccccccccc11111cc11111ccc4ffffffffffffffffff44444433333334444333333333e3344444fffffbbbbbbfaaaaaaaaafbbbbc3333bbbb333cc11111111ccccccccc111111111cc1111cc4fffffffffffffffffffff44444333333344444333e3333444444fffffbbbbbbfaaaaa6aaaaafbbcc3333bbbb3333c1111111ccccccccc11111111111ccc1cccfffffffffffffffffffffff444444333344444443333334444433333ffbbbbbffaa6666666aaaaaaaaa3333333333c111111ccccccccc111111111111ccccccfffffffffffffffffffffffff44444444444444444444ccccccc344433bbbbbbffaa6aa6aaaaaaaaaaa44cc3333333c11111cccccccc11111111111cccc1ccccccccccccccffffffffffffffff444444444444444444ccccccccccc443bbbbbfffaa66666666666666664ccc333333c11111cccccc1111111ccccccccccccccccccccccccccccccccccccccfffff44444444444444cccccccccccccc43bbbbbfffaaaa6aaa6aaaaaaaa44ccce33333c1111ccccc11111111ccccccccccccccccccccccccccccccccccccccccccccff44444444ccccccccccc44ccccc43bbbbbfffaa6666666aaaaaaaaa3ccc433333c1111ccc11111111cccccccccccfbbbbbbbccccccccccccccccccccccccccccccccccccccccccccccc44bbcccc33bbbbbbffaaaaaa6aaaafc3343333c3333333c111ccc11111111ccccccccccfbbbfffffffffaaaffff4f4444cccccccccccccccccccccc4ffccccc33ebbbcccc3bbbbbbb33aaaafffaabcc33333333333333cc111cc11111111ccccccccbbbbbaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaffbfffffffff44ffffcccc43bbbbcccccffbbbbb3f33fffffffffc33333333333e33fcc111cc1111111cccccbbbbbbbbaaaaaaaaaaaaa6aaaaaaaaaaaaaaaaaffbbfffffffffffffffccc4433bbcccccffffbbbbbb33fffffffffc33333333333333fcc11ccc111111ccccffbbbbbbbbaa66666666666666666666666666666ffb333fffffffffffffcccc433cccccccbffffbbbb3bb3ffffffffc333343333e3333fcc11ccc111111ccccfffbfbbbbbaa6aaaaaaaaaa6aaaa6aaaaa6aaaaaaff33333fffffff333334cccc3ccccccccbb3333bbbbfbbfffffffccf333333333333ffcc11cccc111111cccfffbbbbbbbaa6aaaaaaaaaa6aaaa6aaaaa6aaaaaaa3333333fffff33333344cccccccccccb333b33bbbbbbbfffffffcffbb33333333bbfccc11ccccccc111ccccfffbbbbbaaa6aaaaebaaaa6aaaa6aafaa6aaffbbb3333e33333ff333e33344ccccccccbb33bbb33bbbbbbbfffffffcfffb3333333bbbfccc111ccccccc11ccccffffbbbbaa6666aabcaa666aaaa6aafaa6aaffbb3333b333333334433333344443ffffb33bbbb333bbbbbbffffffccfffbb33333bbbbfcc1111cccccccc11cccffffffffaaa6aaaaebaaaa6aaaa6aafafffafbb3333bb33333334444b333333333ffff33bbbbb333bbbbfbffffffcfffffbbb33bbbfffcc1111ccccccccccccccffffbfffaa6aaaaaaaaaaaaaaa6aaffffffbb333bbbb333334444bbbbbbbfffffffff3bbbbbe333bbbbbbffffffcffffffbbbbbbbbffcc1111cccccbccc1ccccbbffffffaa6aaaaaaaaaaaaaaa6aafffffbbbbbbbbb333333344bbbbbbbfffffffffb33bbbe3333bbbbbbfffffccffffffbbbbbbffffcc111cccccbbbcccccccbbbbffffaa6aaaaaa66666aaaa6aafffbbbbbbbbbb3333e33333bbfbbbffffffffbbbb33333333bbbbbbbfffffcfffffffffffffffffccc11ccbbb33bbccc1ccbbbbbaaaaa6aaa6aa6aaa6aaaa6aaffbbb3333bbbb3333333333bbffbfffffffffbbbbb3333333bbbbbbbbfffccffffffffffffffffffcc11cbbb3333bccccccbbb4ffaaaa6aaa6aa6aaa6aaaa6aafbbbbb333bbbb333333333bbbfbffffffffffffbbfbbbbbbbbbbbbbbbfffcfffffffffffffffffffcc11cb333333bbcccccbbbbff666666666aa6aaa6aaaa6aabbbbbbbb33bbbbb333333bbbbbffffffffffffbbbbbbbbbbbbbb333bbffccfffffffffffffbbfff3cc11cb33e333bbcccccbbbbffaaaa6aaa6aa6666666666aabbbbbbbb33ebbbbbb3bbbbbbbbffffffffffffbfbbbbbbbbbbb3333bbffcfffffffffffffbbbb333cc11cb333333bbfcccccbbbbaaaaa6aaa6aaaaaaaaaaa6aa3bbbbbbbbbbbbbbbbbbbbbbffffffffffffffbbbbbfbbbbbbb33333bbfccfffffffffffffbbb33333c11cbb3333bbffcccccbbbbbbfaa6aaa6aaaaaaaaaaa6aa333bbbbbbbbbbbbbbbbbbbbbfffffffffffffbbbffffbbbbb333333bbfcffffffffffffffbb333333c11c3bbbbbbbfffccccbbbfbbfaa6aaaaaaaaaaaaaaaaaaaaaaaaaaaaabbbbbbbbffbffffffffffffffffffffffbbbb3333333bbccfffffffffffffff3333333c11cbbbbbebbbfffcccbbbfbbfaa6aaaaaaaaaaaaaaaaaaaaaaaaaaaaffbbbbbbbbffffffffffffffbbbfffbbfbbbbb3333333bbcfffffffffffffff33333333c11cbb3bbbbbbffffcccbbbbffaa6666666666666aaaaa66666666666fffbbbbbffffffffffffffffbbbffbbbbbbbbbb33e333bcc33ffffffffffff333333333c11cbbbbbbbbbffffccccbbbffaaaaaa6aaaaaa6aaaaaaaaaaaa6aaaaffbbbbffffffffffffffffffbbffffffbbbbbbb3333bbbc3333fffffffffff33333333ec11cbb333bbffffffccccbbffffaaaaa6aaaaaa6aaaa6aaaaaaa6aaaaabbbbffffffffffffffffffffffffffffffbbbbbbbbbbcc3333ffffffffff333bb33333c11cb3333fffffffffccccbfffffffafffaffaa6aaaa6aafffaa6aafbbbbbffffffffffffffffffffffffffffffffffbbbbbbbc33333fffffffffb33bb3333e3c11cb3e333ffffffffcccccffffffffffffffafffaaa6aabffaa6aafbbbbbfffffffffffffffffffffffffffffffffffbbbbbcc33333ffffffffb333bb333333c11cb3e333ffffffffbcccccc4fffffffffffffff3aa6aabbbaa6aaaaaaaaaaaaffffffffffffffffffffff33333fffff3333c333e3fffffffffb333bb33333ec11cf3333fffffff33bbcccccc4ffffffffffb3333aa6aabbbaa6aaaaaaaaaaaffffffffffffffffffffff33333333333333cc33333ffffffff3333333333e33c11cff33ffffffff33bbcccccccffffffffffbb33baa6aabbbaa666666666666ffffffffffffffffffff333333333333333cc333e333ffffffb333333333333ec11ccffffffffff3333bbccccccccfffffbbbbbbbbaa6aabbbaa6aaaaaaaaaaafffffffffffffffffff333333333333333ccf333333ffffff3333b33333333e3c11ccffffffffff33333bbccccccccc44bbbbbbbbbaa6aafbfaa6aaaaaaaaaaaaffffffffffffffff333333333ff333334c4ff3333ffffff33333bb333333333c11cccfffffff3333333bbbcccccccccccccc4bbbfafffafbfaaaaafffffffffffffffffffffffff33333333ffffff444cc4fff33ffff33333333bbb33333e33c11cccffffff333e3333fbbbbccccccccccccccccccfffcccccaaaccfffffffffffffbbbffffbbb333333333fffff4cccc44fffffff3333333333beb3333333ec11cccfffff333333333fbbbbbcccccccccccccccccccccccccccccccccccccccccccbbbbfffb333333e3333ffff4cccc4ffffffff33333333333bbb33333333c11cccfff3333333333ffbbbfbbbfcccccccccccccccccccccccccccccccccccccccccccc333333333333333fffcccc4ffffffff33333333333333b333333333c11cccff3333333333ffbbbbbbbbffffccccccccccccccccccccccccccccccccccccccccccccc3333333333fcccccfffffffffff33333333333333333333333cc11ccff3333333333ffffbbbbfbbbbfbbbbb4b4444444bbfffffffccccccccccccccccccccccccccc33333ccccccccccccccccccccc33333333333333333333cc11ccff3333e3333fffffbbbbbbbbbffbbbbbbb44444bbbffffffffffffffccccccccccccccccccccccccccccccccccccccccccccccccccccc333333333333ccc11ccfff33e3333fffffbbbbbbbbbbbbbbbbbbbbbbbbbbffffffffffffffffffffffffffcccccccccccccccccccccccccfff33333333ccccccccccccccccc3cc111cffff333333fffffbbbbbbbbbbbbbbbfbbbbbbbbbbffffffffffffffffffffffffffffffffffbbffffffffffffffffff3333333333333333333333333cccc111cfffff3333ffffffbbb3333333bbbbbbbbbbbbbbbfffffffffffffffffffffffffffffffffffbffffffffffffffffff3333333333333333333333333333cc111c33ffffffffffffbbb3333333333bbbbbbbbbbbbffffffffffffffffffffffffffffffffffffffffffffffffffffff33333bb333333e3333333333333333c111c33fffffffffffbbb3333333e333bbbbbbbbbbbfffffffffffffffffffffffffffffffff33333333ffffffffffff333333bbbb3333333333333333333333cc11c333fffffffffbbb3333333333333bbbbbbbbb44ffffffbbfffffffffffffffffffff33333bb333333333fff33333333bbbbbbbb33333333333333333333cc11c3333fffffffbbbb33333333333333bbbbbbb44444ffffbbbfffffffffffffffffff33333bbbb333333333333333333bbbbb3333333333333333333333333c11c33333fffffbbbb333333333333333bbbbbbb44444bbbbbfbbffffbbbbbbffffff33333333bbb3333333333333333333bb33333333333333333333333333ec11c33e333fffbbbb3333333333333e333bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbffff333333e33333333333333333333333333333e3333333333333333333e333c11c3333333bbbbb33333333bbbb3333333bbbbbbbbbbbbbbbbfbbbffbbb3bbbff333333333333333333333e3333333333333333333333333333333e3e333333c11c33e33333333bbbb3333bbbbbb3333e33bbb3bbbbbbbbbbbbbbbbffbbbbbb33333e3333333333333e33333333333333333333333e33333333e33333333e3ec11c3333333333333333333bbbbbb33333333bbbbbbbbbbbbbbb3bbbbbbb333333333333333333333333333333333e333333333333e33e3333e33333e3e33333c11c3333333ee33e3333333bbbbbb33333333bbb3bbb3bbbbbbbbbbbbbb333333333333333333333333333333333333333e33333333333333e3333e3e333e33ec11c3333333333333333333bbbbbb33333333bbbbbbbbbbbbbbbbbbbbbb333333333333333333333333333333333333333333333333333333333333333333333c11c33333333333333333333bbbb333333333bbbbbbbbbbbbbbbbbbbbbf3333333333333333333333333333333333333e3333333333333e33333e33333333333c11c3333333333bbb33333333333333333e333bbbbbbbbbbfbffbbbffff333333333333333333333333333333333333333333333333e33333e33333e33e33333c11c33e33333bbb333333333333333333333333bfbbbbbbbbbbbbfffffff33333333e33333333333e33333333333333333333333333333333333333333333333c11c333333bbbbbbbb33333333333333333bb33bfffffffffffffffffffff33333333333333e3333333333333333333333333333333333e33333333333e3e333c11c3e333bbbbbfffbbbffff33333333bbbbbbbbbfffffffffffbfff4bffffff333333333333333333333333333333333333333333333333333333e33333333cc11c33333bbbbfffffffffffffffffffffffffbbbbffffffffff3ffff3bfffffff333333333333bbb33333e33333333fffff3333333333333333333333333e3cc11c3333bbbffffffffffffffffffffffffffffbbfffffffffbfffffffffffffffbbbbbbbb3333bbbbb33333333333fffffffff3333bbb33333333333333333cc11c33bbbbffffffffaaffaaffffffffffffffbbffffffffff44ffffffffffffffbbb3bbbbbb3333333333333333fffffffffffff3333bbb333333333333333cc11cbbbbbffffffffaaaaaaafffffffffffffbbbbfaaaffffffffb3fffff4bbffbbb33bbbbbbb33333333333ffffffffffffffffffff333333333bbb3333333cc11cbbbbffffffffaaa666aafffffffffa6666666666afffffffffffffffbbffbbbbbbbbbbb3bffffffffffffffffffffffffffffffffffffffbbbbbbb33333cc11ccbfbffffffffaa66a6aaaafaffffaa6aaaaaaaa6affffffffffffff3bffbbbbbbbbb3bbbbbffffffffffffffffffffffffffffffffffffffbbbbbbb3bb3cc11ccbbbffffffffaa66666666666666666a666666a6ffffffff34ffbfffffbbbb3bbbbbbb3bbbbbbfffffffff3ffffffffffbbbbbbbbbbbbbffffbbbbb33b3cc11ccbbfffffffffaa6aa6aaaaff33abba6a6bbbb6a6fffffffffffffffffbbbbbbbbbbbbbbbb3bbbbfffff4fffbfffffffbbbb444444444bbbffffbbbbb3b33c11ccbbbffffffffaa6666aaffff333bbb6a6beeb6a6bffffffffffffffffbbfbbbbbbbbbbbbbbbbbbfbfffffbffffffffbbbb444444444444bbffffbbbbbb33c11ccbbbfffffff444466444fff3333bbb6a6beeb6a6bfffffffff4fffffbbbbbeeeeeeeeeeeebbbbbfbffff4fffffffffbbb4443333334444bbffffbbbbbb33c11ccccfffffff444cc66c444f33333b3b6a6bbbb6a6bbffffffffb3ff4fbbbfbebbbbbbbbbbebbbbbfbbffffffffffffbbb443334444444444bbffffbbb3b33c11ccccccffcccccccc66ccccf33333bbb6666666666bb3fffffffffffffbbbbbebeeeeeeeebebbbbbfbbfffffffffffbbbb443444444444444bbbffffbb3bb3c111ccccccccccccccc66cccccc33333bbbbb4664bbbbb33ffffffffbfffbbbbbebebbbbbbebebbbbbffbfff3f3fffffbbbb4434444444444444bbffffff3bb3c111ccccccccccccccc66ccccccccc33b33b446644cccc33ffffffffffffbbbbbebebeeeebebebbb3bbfffb4ffffffffbbbb44344444474433444bbfffff33b3c1111cccccccccccccc66ccccccccccccccccc66ccccccc3ffffffffffffbbbbbebebebbebebebbbbbbbffffffffffffbbbb44444444777443444bbfffff33b3c1111cccccccccccccc66ccccccccccccccccc6cccccccccfffffffffffbbbbbbebebebeebebebbbb3bbbfffffffffffbbbb44444447777743444bbbffff33b3c11111ccccccccccccc66cccccccc111111cccc11cccccccfffffffffffbfbbbbebebebbbbebebfbbbbbbffffffffffffbbb44443444777443443bbbffff3333c11111ccccccccccccc66ccccccc11111111111111ccccccffffffffffbbbbbbbebebeeeeeebebbbbbfffffffffffffffbbbb4443444474434443bbbffff3333c11111ccccccccccccc66cccccccccc11111111111cccccfffffffffffbbbbbbbebebbbbbbbbebbbfffbffffffffffffffbbb4444344444444443bbbfffff333c1111cccccccccccccc66cccccccccccccc1111cccccccffffffffffffbbbbbbbebeeeeeeeeeebbffbbffffffffffffffffbbb444333344444433bbbfffff33cc1111ccccccccccc444664444fffcccccccccccccccccfffffffffffffbbb3bbbbbbbbbbbbbbbbffbbff3ffffffffffffffbbbb4444444443333bbbbffffffccc111ccccccccff44446666a4fffffffcccccccccccccffffffffffffffbbbbbbbbbbbbbbbbfbbbfbbffffffffffffffffffbbbbb44444333333bbbbbfffffcccc111cccccccffff4446446aafffffffffccccc6ccffffffffffffffbfbbbbbbbbbbbbbbbbbbbbbfbffffffbfffffffffffffbbbbb33333333bbbbbbffffcccccc11cccccccffffff4a6666aaffffaffffffff466fffffffffffffffbfbbbbbbbbbbbbbbbbbbbbfffffffbfffffffffffffffffbbbbbbbbbbbbbbbbffffcccccc111cccccffffffffaa64a6affffffffffffff4464fffffffffffffbbfbbb3bbbbbbbbfbfbbffffffffff4ff3ffffffffffffffffbbbbbbbbbbbfffffcccccccc111ccccfffffffffaa6666faffffffffff4ff4444fffffffffffffbbfbbbbb33bbbbffffffffffff4bfff4fffffffffffffffffffffffffffffffffcccccccc1111ccccfffffffffff66fffffff4fffffff4f4444f4fffffffffffbbbbbb3333bbbfffffffffffffffffffbffffffffffffffffffffffffffffffffcccccccc1111cccffffffffff4ffffafffffffffffff4f464ff4f4fffffffffbbbb3b333bbbfffffffffffffff3f4ffffffffffffffffffffff3333333333fffcccccccc1111cccfffffffffffffffff44444fff4ffffff444f4ffffffffffffbbbbbbbbbbbbbfffffffffffffffbfffffffffffffffffff333333333333333ccccccccc1111ccffffffffffaffffffff444444444ff4f446ff44fffffffffffbbbfbb3bfbbbffffffffffffffffffffffffffffffff33333333333333333333cccccccc1111ccfffffffffffffffffffff444444fff44f44ffffffffffffffffbbbbbbbbbbffffffffffffffffffffffffffffff3333333333333333aaaaaaaacccccccc111ccfffffffffffaf4fff4fffffffffffff4444ff4f4fffffffffffbbbbffbbbfffffffffffffffffffffffffffff3333333333333333aaaa6666aacccccccc111ccfffffffffffaff4fff44444444444ffff44fffffffffffffffffbbffbbbfffffffffffffffffffffffffffff3333333aaaaaaaa3aaaaa6aa6aaaaaccccc111ccfffffffffffff44ff44f4ccc44fff44fff4fff4fffffffffffffffffffffffffffffffffffffff33333333333aaaaaaa66666aaaaa6666aa6aaaaaaccccc11cccfffffffffff44fff4f44c1ccc44ff4fff44ffffffffffffffffffffbfffffffffffffffffffff33aaaaaaaa3aa6666a6a6b6aaaaa6aa6aa666666accccc11cfcfffffffffff44ff4f44ccccccc44444fff4ffffffffffffffffffbffffffffffffffffffffff33aaa6666aaaaa6aa6a6a6b6a66666aa6aa6cccc6aacccc11cfccffffffffff4fff4f4cccc1c1c444f4f4f4ffffffffffffffffffffffffffffffffffffffff33ba666aa6aaaaa6aa666a6b6a63336aa6666cbbc6aacccc11ccccffffffffff44ff444c1c11cccc4444f4f4ffffffffffffffbfbbfffbbfffffffffffffff333bba6a6aa666a666aa6a6a666a63e36aa6aa6cccc6aacccc11cccccfffffff4f44fff44ccc111c1ccf44f4f4ffffffffffffffffffffbbbbbffbfffffffff33bbbba6a6666a666a6aa6a6aaa6a63336666aa666666aacccc11cccccffffffffff4ffff44cc1cccccc444f4f4fffff4ffffffbfbbbbbbbbbbffffffff3e3e3ebbaaaa6aaaa666a6a6666a6a666666666aaaaaaaa6aaaacccc11ccccccffffffffffffff444cccccc444f4f4ffffffffffffffbfb3bbbb3bbfffffffff6bbbbbbaaaaa6a33aaaaa6aaaaaa6a6aaaaaaaaaa33333a6aa77cccc11cccccccffffffffff4fff44444c4444f4444ffffffffffffffffbbb3bbbbffbfffffff666666666666666666666666666666666666666666666666a7777ccc11ccccccccffffffffffffff4444444444ff4ffffffffffffffffffbbbbbffffffffffff666666666666666666666666666666666666666666666666a7777ccc11cccccccccffffffff4fff4fff44ff4fff44fffff4fffffffffbfffbfffffbfffffffff6bbbbbbaaaaa6aaaaa33a6aaaaaa6a3e3e3a6aa6a3e3e3a6a7777ccc11cccfffccccffffffffffffffff44444ffff44fffffffffffffffffffffffffffffffff3e3e3ebbaaaa6a666aaaa6aa666a6a33333a6aa6a33333a6aa777ccc11cccffffccccffffffffffffffffffffff444fffffffffffffffffffffffffffffffffffffff33bbbba666a6666a6aa636a6aaaaaaa6aa6aaaaaaa6aaa77ccc11cccfffffccccffffffffffffff444444444fffffffffffffffffffffffffffffffffffffffff333bbaaa666aa666aa6e6666aaaa6666a666666a666aa77ccc11ccccfffffccccffffffffff4fffff444ffffffffffffffffffff77777777ffffffffffffffffff33baaaa6aaa6a666636aa666666aa6a6a6bb6a6a6aa77ccc11cccccfffffccccfffffffffffffffffffffff4fffff4fffff777777777777777777777777ffffff33aaaa66666aaaa666aa6a6aa6aa6a6a6bb66666aa7cccc11ccccccfffffccccfffffffffffffffffffffffffffffff777777777777777777777777777777ffff333aaaaaaaaaaaaaaaa666aa666666a6666aaaaa77cccc11ccccccccffffcccccfffffffffffffffffffffffffff777777777777ccccccccccccccc7777777ffff33aaaaaa3333aaa3aaaaaaaaaaaaaaaaaaaaa77ccccc11cccccccccffffccccccccccccccccffffffffffff777777777ccccccc7777777777777ccccccc7777f33333333333333333333333333333333333777cccccc11ccccccccccfffffccccccccccccccccffffff77777777cccccc7777ccccccc777777cccc7777ccc777777733333333333333333333333333777777ccccc7cc11cccccccccccfffffffcccccccccccccccc7777777ccccc777777777777777cccccccc777777777ccccccc777733333333333333333333777777cccc777cccc111cccccccccc7fffffffffff7777ccccccccccccccc777777777777777777777ccccc7777777777777777ccc777777777773333333377777777ccccccccccc1111cccccccccc777ffffff777777777777cccccc77777777777777777777777777777ccc7777777777777777cccccc77777777733337777cccccc7777777ccc11111cccccccccc777ffffcccc7777777777cccccccccccc7777777777777777777777cccccc7cccccccccc7777777cccc7c777777777cccccc77777777cccc1111111ccccccccccccccccccccccc7777cccccc7cccc777cccc77777cc7777ccccccc77cccccccc77ccccc777ccc77777ccccccccccccccc77777777ccccc111111111111ccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111"

CELL_SIZE = 32
CELL_BOUNDS = 128
CELL_FILL = flr(128/CELL_SIZE+1)
SEED=rnd(1)

def lerp(f,to,t):
    return f+t*(to-f)

class Vec2(object):
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def len(self):
        return sqrt(self.x*self.x+self.y*self.y)

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


SHADOW_OFFSET=Vec2(2, 3).normalize().mul(0.2)
PERSPECTIVE_OFFSET = Vec2(64, 80)

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
            self.biomes[i] = Biome(i, Vec2(0,0), Vec2(0,0), False, True, 3)

        self.biomes[14].transition = True
        self.biomes[3].transition = True
        self.biomes[4].transition = True
        self.biomes[7].transition = True
        self.biomes[11].transition = True
        self.biomes[15].transition = True
        self.biomes[10].transition = True


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
        self.height = 4

        self.c=[4,10,3]

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
        self.c = Vec2(0, 0)
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

    def update2(self, p_p_vec, p_v_vec):
        self.pos.x = p_p_vec.x - 64
        self.pos.y = p_p_vec.y - 64


class Cell(object):
    def __init__(self, color):
        self.x = 0
        self.y = 0
        self.color = color
        self.seed = 0.0
        self.edges = {-1: {-1: 1, 0 : 1, 1 : 1},
                       0: {-1: 1, 0 : 1, 1 : 1},
                       1: {-1: 1, 0 : 1, 1 : 1}}

class Cells(object):
    def __init__(self, x, y, mapdata):
        self.pos = Vec2(x, y)
        self.mapdata = mapdata

        self.cells = []
        for _ in range(0, CELL_FILL*CELL_FILL):
            self.cells.append(Cell(1))

        self.set_cells()

    def set_pos(self, pos):
       # print("UPDATE CELLS", self.pos.x, pos.x, self.pos.y, pos.y)
        if self.pos.x != pos.x or self.pos.y != pos.y:
        #if (flr(self.pos.x*CELL_SIZE) != flr(pos.x*CELL_SIZE)) or (flr(self.pos.y/CELL_SIZE) != flr(pos.y/CELL_SIZE)):
        #print("UPDATE CELLS", flr(self.pos.x*CELL_SIZE), flr(pos.x*CELL_SIZE), flr(self.pos.y/CELL_SIZE), flr(pos.y/CELL_SIZE))
            self.pos.x = pos.x
            self.pos.y = pos.y
            self.set_cells()

    def get(self, x, y):
        return self.cells[x*CELL_FILL+y]

    def set_cells(self):
        for a in range(0, CELL_FILL):
            for b in range(0, CELL_FILL):
                cell = self.get(a, b)
                x=flr(a+self.pos.x)
                y=flr(b+self.pos.y)

                cell.x = x
                cell.y = y

                if x<0 or x>CELL_BOUNDS-1 or y<0 or y>CELL_BOUNDS-1:
                    cell.color = 1
                else:
                    cell.color = self.mapdata[y][x]

                cell.seed=SEED+x*(CELL_BOUNDS*2)+y

                for u in range(-1, 2):
                    for v in range(-1, 2):
                        if x+u<0 or x+u>CELL_BOUNDS-1 or y+v<0 or y+v>CELL_BOUNDS-1:
                            cell.edges[u][v]=1
                        else:
                            cell.edges[u][v]=self.mapdata[y+v][x+u]

                        if cell.edges[u][v]==14:
                            cell.edges[u][v]=3

                        cell.edges[u][v] = cell.edges[u][v] or 1

def myrange(x):
    return random.randint(flr(x[0]), flr(x[1]))

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
P = Player(Vec2(82,16).mul(32))
CAM = Camera(P.pos.sub(Vec2(64, 64+128)))
CLOUDS = Clouds()
M = MapFormat(MAP)

P.pos.y -= 128

CELLS = Cells(flr(CAM.pos.x/CELL_SIZE), flr(CAM.pos.y/CELL_SIZE), M.mapdata)

def _init():
    print("CAMERA", CAM.pos.x, CAM.pos.y)
    palt(0, False)
    palt(14, True)

    draw_background()
    draw_player()

def _update():
    global PERSPECTIVE_OFFSET

    P.update()

    PERSPECTIVE_OFFSET = Vec2(64+sin(px8_time()/9)*4, 80+sin(px8_time()/11)*4)

    CAM.update(P.pos, P.v)
    CELLS.set_pos(Vec2(flr(CAM.pos.x/CELL_SIZE),
                       flr(CAM.pos.y/CELL_SIZE)))
    CLOUDS.update(CAM)

def _draw2():
    camera(0, 0)
    px8_print("X: %d Y %d" %(P.pos.x, P.pos.y), 0, 120)

def _draw():
    camera(CAM.pos.x, CAM.pos.y)

    draw_background()

    # shadow stuff
    draw_clouds(True)

    draw_player()
    draw_clouds()

    camera(0, 0)
    px8_print("P X %f Y %f" %(P.pos.x, P.pos.y), 0, 112)
    px8_print("C X %d Y %d" %(flr(CAM.pos.x), flr(CAM.pos.y)), 0, 120)


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

    #print("CAMERA ", CAM.pos.x, CAM.pos.y)
    for a in range(0, CELL_FILL):
        for b in range(0, CELL_FILL):

            x = (CELLS.pos.x+a)*CELL_SIZE
            y = (CELLS.pos.y+b)*CELL_SIZE

            #x = CAM.pos.x+a*CELL_SIZE
            #y = CAM.pos.y+b*CELL_SIZE

            #print("RECT ", x, y)

            cell = CELLS.get(a, b)
            rectfill(x, y, x+CELL_SIZE, y+CELL_SIZE, cell.color)

            if cell.color in B.biomes:
                srand(cell.seed)

                biome = B.biomes[cell.color]
                if biome.transition:
                    c = cell.edges[1][0]
                    if c != cell.color:
                        pal(0, c)
                        for v in range(0, flr(CELL_SIZE/8)):
                            spr(4+flr(rnd(4))*16,x+CELL_SIZE-8, y+v*8)

                    c=cell.edges[-1][0]
                    #print(c, cell.color, c != cell.color)
                    if c != cell.color and c != 1:
                        pal(0, c)
                        for v in range(0, flr(CELL_SIZE/8)):
                            spr(3+flr(rnd(4))*16,x, y+v*8)

                    c=cell.edges[0][-1]
                    #print(c, cell.color, c != cell.color)
                    if c != cell.color and c != 1:
                        pal(0, c)
                        for u in range(0, flr(CELL_SIZE/8)):
                            spr(2+flr(rnd(4))*16,x+u*8, y)

                    c=cell.edges[0][1]
                    #print(c, cell.color, c != cell.color)
                    if c != cell.color and c != 1:
                        pal(0, c)
                        for u in range(0, flr(CELL_SIZE/8)):
                            spr(1+flr(rnd(4))*16,x+u*8, y+CELL_SIZE-8)

        pal(0,0)