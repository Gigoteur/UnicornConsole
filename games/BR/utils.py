addglobals = lambda x: globals().update(x)

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