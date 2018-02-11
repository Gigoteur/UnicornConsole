# Based on Voxel framework / Tiny-TV Jam from TRAVERSAL_DOG
# http://www.lexaloffle.com/bbs/?tid=28480
# http://www.lexaloffle.com/bbs/?tid=28308


class Config(object):
    def __init__(self):
        self.cam = 0
        self.val = 0

config = Config()

def _init():
    pass

def _update():
    if btn(0):
        config.val-=0.01
    if btn(1):
        config.val+=0.01

    config.cam+=config.val
    config.val*=0.9

def _draw():
    cls()

    ocx=cos(config.cam)
    osx=-sin(config.cam)
    ocy=cos(config.cam+0.25)
    osy=-sin(config.cam+0.25)

    ssx = 0
    sox = 0

    ssy = 0
    soy = 0

    if config.cam%1>0.5:
        ssx=15
        sox=-1
    else:
        ssx=0
        sox=1

    if config.cam%1>0.25 and config.cam%1<0.75:
        ssy=15
        soy=-1
    else:
        ssy=0
        soy=1

    for l in range(0, 19):
        ly=96-l*4
        lx=64

        x = lx
        y = ly

        sx=ssx
        for ix in range(0,15):
            sy=ssy
            for iy in range(0,15):
                c=sget(sx+(l%8)*16,sy+flr(l/8)*16)
                if c != 0:
                    xx=3.99*((sx-7.5)*ocx+(sy-7.5)*ocy)
                    yy=1.2*((sx-7.5)*osx+(sy-7.5)*osy)

                    xx+=x
                    yy+=y

                    rectfill(xx-2,yy-2,xx+1,yy+1,c)
                sy+=soy
            sx+=sox