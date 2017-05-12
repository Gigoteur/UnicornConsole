class Flappy(object):
    def __init__(self):
        self.s=0
        self.f=0
        palt(12, True)
        palt(0, False)
        self.px=30
        self.py=60
        self.pv=0
        self.pa=0
        self.pr=0.001
        self.g=0.5
        self.gap=6
        self.pipex=128
        self.pipey=flr(rnd(15-self.gap))
        self.score=0
        self.bkgx=0

    def update(self):
        self.f=(self.f+1) & 255
        if self.s == 0:
            self.bkgx+=1
            if btnp(5):
                self.pv=0
                self.pa=-5000
                self.s=1
        elif self.s == 1:
            self.bkgx+=1
            self.pipex-=1
            if self.pipex<=-24:
                self.pipey=flr(rnd(15-self.gap))
                self.pipex=128

            self.pa*=self.pr
            self.pv+=self.g+self.pa
            self.py+=self.pv

            if self.py>144 or self.py<-16 or ((self.px+12)>self.pipex and (self.px+8)<(self.pipex+24) and (self.py-4<(self.pipey+1)*8 or self.py+12>(self.pipey+1+self.gap)*8)):
                self.f=0
                self.s=2

            if self.pipex==24:
                self.score+=1

            if btnp(5):
                self.pv=0
                self.pa=-5000
        elif self.s == 2:
            if self.f < 10:
                camera(rnd(4)-2,rnd(4)-2)
            else:
                self.pa*=self.pr
                self.pv+=self.g+self.pa
                self.py+=self.pv
            if self.f >= 10 and self.py > 128:
                self.s=3
        elif self.s == 3:
            camera(0,0)
            if btnp(5):
                self.py=60
                self.pv=0
                self.pipex=-24
                self.score=0
                self.s=1


    def drawcity(self):
        for i in range(-2,4):
            spr(36,(self.bkgx/-8)%64+i*32,96,4,2)

        for i in range(-2,4):
            spr(40,(self.bkgx/-4)%64+i*32,106,4,2)

        for i in range(-2,4):
            spr(44,(self.bkgx/-2)%64+i*32,114,4,2)

    def drawpipes(self):
        for i in range(-1,self.pipey+1):
            spr(27,self.pipex,i*8,3,1)
        spr(11,self.pipex,(self.pipey+1)*8,3,1)
        spr(11,self.pipex,(self.pipey+1+self.gap)*8,3,1)

        for i in range(self.pipey+2+self.gap,18):
            spr(27,self.pipex,i*8,3,1)

    def draw(self):
        if self.s == 0:
            self.drawcity()
            self.drawpipes()
            if self.f & 4 == 4:
                spr(32, self.px, self.py, 2, 2)
            else:
                spr(34, self.px, self.py,2,2)
            px8_print("press x to flap",32,48,7)
            px8_print("flappy!",50,32,7)
        elif self.s == 1:
            self.drawcity()
            self.drawpipes()
            if self.pv<0 and self.f & 4 == 4:
                spr(32,self.px,self.py,2,2)
            else:
                spr(34,self.px,self.py,2,2)
        elif self.s <= 3:
            self.drawcity()
            self.drawpipes()
            spr(14,self.px,self.py,2,2)
            if self.s==3:
                px8_print("press x to flap",32,48,7)

        if self.s > 0:
            i=self.score
            sx=64

            d=i%10
            spr(d+1,sx,4,1,2)
            i=flr(i/10)
            sx-=8
            while i > 0:
                d=i%10
                spr(d+1,sx,4,1,2)
                i=flr(i/10)
                sx-=8

F = Flappy()

def _init():
    pass

def _update():
    F.update()

def _draw():
    global F

    rectfill(0, 0, 128, 128, 12)
    F.draw()