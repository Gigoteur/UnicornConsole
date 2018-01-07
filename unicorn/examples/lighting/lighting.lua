-- dank tombs tech demo
-- by jakub wasilewski

-- positively not a game yet

-- press / to manipulate
-- light radius, d-pad to
-- walk around

------------------------------
-- utilities
------------------------------

function round(x)
    return flr(x+0.5)
end

-- copies props to obj
-- if obj is nil, a new
-- object will be created,
-- so set(nil,{...}) copies
-- the object
function set(obj,props)
    obj=obj or {}
    for k,v in pairs(props) do
        obj[k]=v
    end
    return obj
end

-- used for callbacks into
-- entities that might or
-- might not have a method
-- to handle an event
function event(ob,name,...)
    local cb=ob[name]
    return type(cb)=="function"
            and cb(ob,...)
            or cb
end

-- returns smallest element
-- of seq, according to key
-- function
function min_of(seq,key)
    local me,mk=nil,32767
    for e in all(seq) do
        local k=key(e)
        if k<mk then
            me,mk=e,k
        end
    end
    return me
end

------------------------------
-- class system
------------------------------

-- creates a "class" object
-- with support for basic
-- inheritance/initialization
function kind(kob)
    kob=kob or {}
    setmetatable(kob,{__index=kob.extends})

    kob.new=function(self,ob)
        ob=set(ob,{kind=kob})
        setmetatable(ob,{__index=kob})
        if (kob.create) ob:create()
            return ob
        end

        return kob
    end

    -------------------------------
    -- vectors
    -------------------------------

    -- for some stuff, we want
    -- vector math - so we make
    -- a vector type with all the
    -- usual operations
    vec={}
    function vec.__add(v1,v2)
        return v(v1.x+v2.x,v1.y+v2.y)
    end
    function vec.__sub(v1,v2)
        return v(v1.x-v2.x,v1.y-v2.y)
    end
    function vec.__mul(v1,a)
        return v(v1.x*a,v1.y*a)
    end
    function vec.__mul(v1,a)
        return v(v1.x*a,v1.y*a)
    end
    function vec.__div(v1,a)
        return v(v1.x/a,v1.y/a)
    end
    -- we use the ^ operator
    -- to mean dot product
    function vec.__pow(v1,v2)
        return v1.x*v2.x+v1.y*v2.y
    end
    function vec.__unm(v1)
        return v(-v1.x,-v1.y)
    end
    -- this is not really the
    -- length of the vector,
    -- but length squared -
    -- easier to calculate,
    -- and can be used for most
    -- of the same stuff
    function vec.__len(v1)
        local x,y=v1:split()
        return x*x+y*y
    end
    -- normalized vector
    function vec:norm()
        return self/sqrt(#self)
    end
    -- rotated 90-deg clockwise
    function vec:rotcw()
        return v(-self.y,self.x)
    end
    -- force coordinates to
    -- integers
    function vec:ints()
        return v(flr(self.x),flr(self.y))
    end
    -- used for destructuring,
    -- i.e.:  x,y=v:split()
    function vec:split()
        return self.x,self.y
    end
    -- has to be there so
    -- our metatable works
    -- for both operators
    -- and methods
    vec.__index = vec

    -- creates a new vector
    function v(x,y)
        local vector={x=x,y=y}
        setmetatable(vector,vec)
        return vector
    end

    -- vector for each cardinal
    -- direction, ordered the
    -- same way pico-8 d-pad is
    dirs={
        v(-1,0),v(1,0),
        v(0,-1),v(0,1)
    }

    -------------------------------
    -- boxes
    -------------------------------

    -- a box is just a rectangle
    -- with some helper methods
    box=kind()
    function box:translate(v)
        return make_box(
            self.xl+v.x,self.yt+v.y,
            self.xr+v.x,self.yb+v.y
        )
    end

    function box:overlaps(b)
        return
        self.xr>=b.xl and
                b.xr>=self.xl and
                self.yb>=b.yt and
                b.yb>=self.yt
    end

    function box:contains(pt)
        return pt.x>=self.xl and
                pt.y>=self.yt and
                pt.x<=self.xr and
                pt.y<=self.yb
    end

    function box:sepv(b)
        local candidates={
            v(b.xl-self.xr-1,0),
            v(b.xr-self.xl+1,0),
            v(0,b.yt-self.yb-1),
            v(0,b.yb-self.yt+1)
        }
        return min_of(candidates,vec.__len)
    end

    function make_box(xl,yt,xr,yb)
        if (xl>xr) xl,xr=xr,xl
            if (yt>yb) yt,yb=yb,yt
                return box:new({
                    xl=xl,yt=yt,xr=xr,yb=yb
                })
            end

            function vec_box(v1,v2)
                return make_box(
                    v1.x,v1.y,
                    v2.x,v2.y
                )
            end

            ------------------------------
            -- entity system
            ------------------------------

            -- entity root type
            entity=kind({
                t=0,state="s_default"
            })

            -- a big bag of all entities
            entities={}

            -- entities with some special
            -- props are tracked separately
            entities_with={}
            tracked_props={
                "render","cbox",
                "walls","shadow"
            }

            -- used to add/remove objects
            -- in the entities_with list
            function update_with_table(e,fn)
                for prop in all(tracked_props) do
                    if e[prop] then
                        local lst=
                        entities_with[prop] or {}
                        fn(lst,e)
                        entities_with[prop]=lst
                    end
                end
            end

            -- all entities do common
            -- stuff when created -
            -- mostly register in lists
            e_id=1
            function entity:create()
                if not self.name then
                    self.name=e_id..""
                    e_id+=1
                end
                local name=self.name
                entities[name]=self

                update_with_table(self,add)
            end

            -- this is the core of our
            -- _update() method - update
            -- each entity in turn
            function update_entities()
                for n,e in pairs(entities) do
                    local update_fn=e[e.state]
                    local result = update_fn
                            and update_fn(e,e.t)
                            or nil

                    if result then
                        if result==true then
                            -- remove entity
                            entities[n]=nil
                            update_with_table(e,del)
                        else
                            -- set state
                            set(e,{
                                state=result,t=0
                            })
                        end
                    else
                        -- bump timer in this state
                        e.t+=1
                    end
                end
            end

            ------------------------------
            -- entity rendering
            ------------------------------

            -- renders entities, sorted by
            -- y to get proper occlusion
            function render_entities()
                ysorted={}

                for d in all(entities_with.render) do
                    local y=d.pos and flr(d.pos.y) or 139
                    ysorted[y]=ysorted[y] or {}
                    add(ysorted[y],d)
                end
                for y=clipbox.yt,clipbox.yb do
                    for d in all(ysorted[y]) do
                        reset_palette()
                        d:render(d.t)
                    end
                    reset_palette()
                end
            end

            function render_blob_shadows()
                local sh_fill=fl_blend(5)
                for e in all(entities_with.shadow) do
                    local sh=e.shadow
                    local p=e.pos+e.shadow
                    if clipbox:contains(p) then
                        cellipse(p.x,p.y,
                            sh.rx,sh.ry,sh_fill)
                    end
                end
            end

            -------------------------------
            -- collisions
            -------------------------------

            function c_box(e)
                local b,p=e.cbox,e.pos
                return p
                        and b:translate(p:ints())
                        or b
            end

            cqueue={}
            function collide(ent,prop,cb)
                add(cqueue,{e=ent,p=prop,cb=cb})
            end

            function do_collisions()
                for c in all(cqueue) do
                    local e=c.e
                    local eb=c_box(e)
                    for o in all(entities_with[c.p]) do
                        if o~=e then
                            local ob=c_box(o)
                            if eb:overlaps(ob) then
                                local separate=c.cb(e,o)
                                if separate then
                                    local sepv=eb:sepv(ob)
                                    e.pos+=sepv
                                    eb=eb:translate(sepv)
                                end
                            end
                        end
                    end
                end
                cqueue={}
            end

            function debug_collisions()
                for e in all(entities_with.cbox) do
                    local eb=c_box(e)
                    rect(eb.xl,eb.yt,eb.xr,eb.yb,4)
                end
            end

            ------------------------------
            -- drawing shapes
            ------------------------------

            --  all shapes accept a fill
            -- function which is responsible
            -- for actual drawing
            --  the functions just do
            -- calculations and clipping

            -- draws a polygon from an
            -- array of points, using
            -- ln() to fill it
            -- points must be clockwise
            function ngon(pts,ln)
                local xls,xrs=ngon_setup(pts)
                for y,xl in pairs(xls) do
                    local xr=xrs[y]
                    ln(xl,xr,y)
                end
            end

            -- like ngon, but with a
            -- rectangular hole (used
            -- to mask shadows)
            dummy_hole={tl={y=16000},br={}}
            function holed_ngon(pts,hole,ln)
                local xls,xrs=ngon_setup(pts)
                hole=hole or dummy_hole
                local htop,hbot,hl,hr=
                hole.tl.y,hole.br.y,
                hole.tl.x,hole.br.x
                for y,xl in pairs(xls) do
                    local xr=xrs[y]
                    if y<htop or y>hbot then
                        ln(xl,xr,y)
                    else
                        local cl,cr=
                        min(hl,xr),max(hr,xl)
                        if xl<=cl then
                            ln(xl,cl,y)
                        end
                        if cr<=xr then
                            ln(cr,xr,y)
                        end
                    end
                end
            end

            -- sets up min/max x of
            -- each polygon line
            function ngon_setup(pts)
                local xls,xrs={},{}
                local npts=#pts
                for i=0,npts-1 do
                    ngon_edge(
                        pts[i+1],pts[(i+1)%npts+1],
                        xls,xrs
                    )
                end
                return xls,xrs
            end

            function ngon_edge(a,b,xls,xrs)
                local ax,ay=a.x,round(a.y)
                local bx,by=b.x,round(b.y)
                if (ay==by) return

                    local x,dx=
                    ax,(bx-ax)/abs(by-ay)
                    if ay<by then
                        for y=ay,by do
                            xrs[y]=x
                            x+=dx
                        end
                    else
                        for y=ay,by,-1 do
                            xls[y]=x
                            x+=dx
                        end
                    end
                end

                -- draws a filled rectangle
                -- with a custom fill fn
                function crect(x1,y1,x2,y2,ln)
                    x1,x2=max(x1,0),mid(x2,127)
                    y1,y2=max(y1,0),min(y2,127)
                    if (x2<x1 or y2<y1) return
                        for y=y1,y2 do
                            ln(x1,x2,y)
                        end
                    end

                    -- draws a filled ellipse
                    -- with a custom fill fn
                    function cellipse(cx,cy,rx,ry,ln)
                        cy,ry=round(cy),round(ry)
                        local w=0
                        local ryx,rxy=ry/rx,rx/ry
                        local dy=(-2*ry+1)*rxy
                        local dx=ryx
                        local ddx=2*ryx
                        local ddy=2*rxy
                        local lim=rx*ry
                        local v=ry*ry*rxy
                        local my=cy+ry-1
                        for y=cy-ry,cy-1 do
                            -- creep w up until we hit lim
                            while true do
                                if v+dx<=lim then
                                    v+=dx
                                    dx+=ddx
                                    w+=1
                                else
                                    break
                                end
                            end
                            -- draw line
                            if w>0 then
                                local l,r=
                                mid(cx-w,0,127),
                                mid(cx+w-1,0,127)
                                if (y>=0 and y<128) ln(l,r,y)
                                    if (my>=0 and my<128) ln(l,r,my)
                                    end
                                    -- go down
                                    v+=dy
                                    dy+=ddy
                                    my-=1
                                end
                            end

                            -------------------------------
                            -- basic fills
                            -------------------------------

                            -- a fill function is just
                            -- a function(x1,x2,y) that
                            -- draws a horizontal line

                            -- returns a fill function
                            -- that draws a solid color
                            function fl_color(c)
                                return function(x1,x2,y)
                                    rectfill(x1,y,x2,y,c)
                                end
                            end

                            -- used as fill function
                            -- for ignored areas
                            function fl_none()
                            end

                            -------------------------------
                            -- fast blend fill
                            -------------------------------

                            -- sets up everything
                            -- blend_line will need
                            function init_blending(nlevels)
                                -- tabulate sqrt() for speed
                                _sqrt={}
                                for i=0,4096 do
                                    _sqrt[i]=sqrt(i)
                                end

                                -- populate look-up tables
                                -- for blending based on
                                -- palettes in sprite mem
                                for lv=1,nlevels do
                                    -- light luts are stored in
                                    -- memory directly, table
                                    -- indexing is costly
                                    local addr=0x4300+lv*0x100
                                    local sx=lv-1
                                    for c1=0,15 do
                                        local nc=sget(sx,c1)
                                        local topl=shl(nc,4)
                                        for c2=0,15 do
                                            poke(addr,
                                                topl+sget(sx,c2))
                                            addr+=1
                                        end
                                    end
                                end
                            end

                            function fl_blend(l)
                                local lutaddr=0x4300+shl(l,8)
                                return function(x1,x2,y)
                                    local laddr=lutaddr
                                    local yaddr=0x6000+shl(y,6)
                                    local saddr,eaddr=
                                    yaddr+band(shr(x1+1,1),0xffff),
                                    yaddr+band(shr(x2-1,1),0xffff)
                                    -- odd pixel on left?
                                    if band(x1,1.99995)>=1 then
                                        local a=saddr-1
                                        local v=peek(a)
                                        poke(a,
                                            band(v,0xf) +
                                                    band(peek(bor(laddr,v)),0xf0)
                                        )
                                    end
                                    -- full bytes
                                    for addr=saddr,eaddr do
                                        poke(addr,
                                            peek(
                                                bor(laddr,peek(addr))
                                            )
                                        )
                                    end
                                    -- odd pixel on right?
                                    if band(x2,1.99995)<1 then
                                        local a=eaddr+1
                                        local v=peek(a)
                                        poke(a,
                                            band(peek(bor(laddr,v)),0xf) +
                                                    band(v,0xf0)
                                        )
                                    end
                                end
                            end

                            -------------------------------
                            -- lighting
                            -------------------------------

                            -- determines how far each
                            -- level of light reaches
                            -- this is distance *squared*
                            -- due to the ordering here,
                            -- light level 1 is the
                            -- brightest, and 6 is the
                            -- darkest (pitch black)
                            light_rng={
                                10*42,18*42,
                                26*42,34*42,
                                42*42,
                            }
                            -- special "guard" value
                            -- to ensure nothing can be
                            -- light level 0 without ifs
                            light_rng[0]=-1000

                            --  this is our "light" fill
                            -- function.
                            --  it operates by dimming
                            -- what's already there.
                            --  each horizontal line
                            -- is drawn by multiple
                            -- calls to another fill
                            -- function handling
                            -- the correct light level
                            -- for each segment.
                            light_fills={
                                fl_none,fl_blend(2),fl_blend(3),
                                fl_blend(4),fl_blend(5),fl_color(0)
                            }
                            brkpts={}
                            function fl_light(lx,ly,brightness,ln)
                                local brightnessf,fills=
                                brightness*brightness,
                                light_fills
                                return function(x1,x2,y)
                                    -- transform coordinates
                                    -- into light-space
                                    local ox,oy,oe=x1-lx,y-ly,x2-lx
                                    -- brightness range multiplier
                                    -- + per line flicker effect
                                    local mul=
                                    brightnessf*
                                            (rnd(0.16)+0.92)
                                    -- calculate light levels
                                    -- at left end, right end,
                                    local ysq=oy*oy
                                    local srng,erng,slv,elv=
                                    ysq+ox*ox,
                                    ysq+oe*oe
                                    for lv=5,0,-1 do
                                        local r=band(light_rng[lv]*mul,0xffff)
                                        if not slv and srng>=r then
                                            slv=lv+1
                                            if (elv) break
                                            end
                                            if not elv and erng>=r then
                                                elv=lv+1
                                                if (slv) break
                                                end
                                            end
                                            -- these will hold the
                                            -- lowest/highest light
                                            -- level within our line
                                            local llv,hlv=1,max(slv,elv)
                                            -- calculate breakpoints
                                            -- (x coordinates at which
                                            --  light level changes,
                                            --  in light-space)
                                            -- and lowest(brightest)
                                            -- light level within line
                                            local mind=max(x1-lx,lx-x2)
                                            for lv=hlv-1,1,-1 do
                                                local brng=band(light_rng[lv]*mul,0xffff)
                                                local brp=_sqrt[brng-ysq]
                                                brkpts[lv]=brp
                                                if not brp or brp<mind then
                                                    llv=lv+1
                                                    break
                                                end
                                            end
                                            -- everything calculated,
                                            -- draw all segments now!
                                            local xs,xe=lx+ox
                                            -- from left bound to
                                            -- start of most-lit segment
                                            -- decreasing light lv
                                            -- (brightness increasing)
                                            for l=slv,llv+1,-1 do
                                                xe=lx-brkpts[l-1]
                                                fills[l](xs,xe-1,y)
                                                xs=xe
                                            end
                                            -- from most-lit zone
                                            -- to last break point
                                            -- increasing light lv
                                            -- (brightness decreasing)
                                            for l=llv,elv-1 do
                                                xe=lx+brkpts[l]
                                                fills[l](xs,xe-1,y)
                                                xs=xe
                                            end
                                            -- last segment from
                                            -- last breakpoint to the
                                            -- right bound
                                            fills[elv](xs,x2,y)
                                        end
                                    end

                                    -------------------------------
                                    -- palette effects
                                    -------------------------------

                                    function init_palettes(n)
                                        pals={}
                                        for p=1,n do
                                            pals[p]={}
                                            for c=0,15 do
                                                pals[p][c]=sget(p,c)
                                            end
                                        end
                                    end

                                    function reset_palette()
                                        pal()
                                        palt(3,true)
                                        palt(0,false)
                                    end

                                    function set_palette(no)
                                        for c,nc in pairs(pals[no]) do
                                            pal(c,nc)
                                        end
                                    end

                                    function dim_object(o)
                                        local lcoeff=(o.pos-lgt.pos).y/25
                                        if lcoeff>0 then
                                            local pt=mid(flr(lcoeff/0.1),0,6)
                                            set_palette(8+pt)
                                        end
                                    end

                                    -------------------------------
                                    -- shadowcasting by walls
                                    -------------------------------

                                    function render_wall_shadows()
                                        local render_one=
                                        render_shadow_fn()
                                        for e in all(entities_with.walls) do
                                            foreach(e.walls,render_one)
                                        end
                                    end

                                    rsh_corners={
                                        v(-1,-1),v(1,-1),v(1,1),v(-1,1),
                                        v(-1,-1),v(1,-1),v(1,1),v(-1,1),
                                    }
                                    function render_shadow_fn()
                                        -- remember lighting info
                                        local p,rng=lgt.pos:ints(),lgt:range()
                                        local rngsq=rng*rng
                                        local black=fl_color(0)

                                        -- return function using it
                                        return function(wall)
                                            local s,e=wall.s,wall.e
                                            local dist=wall.d^(p-s)
                                            if (dist<=0) return
                                                local ds,de=s-p,e-p
                                                if (#ds>rngsq and #de>rngsq) return
                                                    local horiz=wall.d.x~=0

                                                    local cs,ce=
                                                    rng/max(abs(ds.x),abs(ds.y)),
                                                    rng/max(abs(de.x),abs(de.y))
                                                    local ps,pe=
                                                    ds*cs,de*ce

                                                    local qs,qe=quadrant(ps),quadrant(pe)
                                                    if (qs<qe) qs+=4

                                                        local pts={s,e,p+pe}
                                                        for q=qe,qs-1 do
                                                            add(pts,p+rsh_corners[q]*rng)
                                                        end
                                                        add(pts,p+ps)

                                                        if wall.h then
                                                            holed_ngon(pts,wall.h,black)
                                                        else
                                                            ngon(pts,black)
                                                        end
                                                    end
                                                end

                                                -- returns the quadrant a given
                                                -- point is in. quadrants start
                                                -- at 1 (left) and go clockwise
                                                function quadrant(v)
                                                    if abs(v.x)>abs(v.y) then
                                                        return v.x>0 and 3 or 1
                                                    else
                                                        return v.y>0 and 4 or 2
                                                    end
                                                end

                                                -------------------------------
                                                -- solids (light obscuring)
                                                -------------------------------

                                                gobs={
                                                    sd_spire={
                                                        sprite={n=67,w=2,h=4},
                                                        tile=115,
                                                        off=v(8,4),
                                                        walls={
                                                            {-8,0,8,0,3},
                                                            {8,0,8,-15,1},
                                                            {8,-15,-8,-15,4},
                                                            {-8,-15,-8,0,2}
                                                        },
                                                        hole={-4,-32,3,-15},
                                                        cbox=make_box(-8,-15,7,0)
                                                    }
                                                }

                                                solid=kind({
                                                    extends=entity
                                                })

                                                function solid:create()
                                                    local def,pos=
                                                    self.def,self.pos

                                                    self.cbox=self.def.cbox
                                                    local hole=self.def.hole
                                                    if hole then
                                                        hole={
                                                            tl=pos+v(hole[1],hole[2]),
                                                            br=pos+v(hole[3],hole[4])
                                                        }
                                                    end
                                                    self.walls={}
                                                    for wd in all(self.def.walls) do
                                                        add(self.walls,{
                                                            s=pos+v(wd[1],wd[2]),
                                                            e=pos+v(wd[3],wd[4]),
                                                            d=dirs[wd[5]],
                                                            h=hole
                                                        })
                                                    end
                                                    entity.create(self)
                                                end

                                                solid.walked_into=true

                                                function solid:render()
                                                    local s=self.def.sprite
                                                    local spos=
                                                    self.pos+v(-s.w*4,-s.h*8)
                                                    -- dynamic lighting
                                                    dim_object(self)
                                                    spr(s.n,spos.x,spos.y,s.w,s.h,self.flipped)
                                                end

                                                -------------------------------
                                                -- player object
                                                -------------------------------

                                                indiana=kind({
                                                    extends=entity,
                                                    frm=0,
                                                    shadow={x=0,y=0,rx=8,ry=4},
                                                    shoff=v(0,0),
                                                    cbox=make_box(-3,-5,4,1)
                                                })
                                                ind_shadow_locs={
                                                    v(2,0),v(-2,0),v(0,0),v(0,-3)
                                                }

                                                function indiana:s_default(t)
                                                    -- moving around
                                                    local moving=false
                                                    for i=0,3 do
                                                        if btn(i) then
                                                            if (not btn(4)) self.facing=i+1
                                                                self.pos+=dirs[i+1]*0.6
                                                                moving=true
                                                            end
                                                        end
                                                        if moving then
                                                            if t%6==0 then
                                                                self.frm=(self.frm+1)%3
                                                            end
                                                        else
                                                            self.frm=0
                                                        end
                                                        -- update shadow position
                                                        set(self.shadow,ind_shadow_locs[self.facing])
                                                        -- collision detection
                                                        collide(self,"cbox",self.hit_object)
                                                    end

                                                    function indiana:hit_object(ob)
                                                        return event(ob,"walked_into")
                                                    end

                                                    ind_sprites={
                                                        195,195,233,227
                                                    }
                                                    function indiana:render()
                                                        local pos=self.pos
                                                        local sprite=
                                                        ind_sprites[self.facing]+
                                                                self.frm*2
                                                        spr(sprite,pos.x-8,pos.y-16,2,2,self.facing==1)
                                                    end

                                                    -------------------------------
                                                    -- light object
                                                    -------------------------------

                                                    light=kind({
                                                        extends=entity,
                                                        off=v(0,0),
                                                        cbox=make_box(-1,-1,1,1)
                                                    })
                                                    light_offsets={
                                                        v(-7,-2),v(7,-2),
                                                        v(0,-9),v(0,6)
                                                    }

                                                    function light:s_default()
                                                        --anchor to avatar
                                                        self.pos=ply.pos
                                                        --controlling the light
                                                        if btn(4) and self.bri>0.2 then
                                                            self.bri-=0.02
                                                        end
                                                        if btn(5) and self.bri<63/44 then
                                                            self.bri+=0.02
                                                        end
                                                    end

                                                    function light:range()
                                                        return flr(42*self.bri)
                                                    end

                                                    function light:extents()
                                                        local p,r=self.pos:ints(),
                                                        self:range()
                                                        return
                                                        p.x-r,p.y-r,
                                                        p.x+r,p.y+r
                                                    end

                                                    function light:apply()
                                                        local p=self.pos:ints()
                                                        local light_fill=fl_light(
                                                            p.x,p.y,self.bri,
                                                            blend_line
                                                        )
                                                        local xl,yt,xr,yb=
                                                        self:extents()
                                                        crect(xl,yt,xr,yb,
                                                            light_fill)
                                                    end

                                                    -------------------------------
                                                    -- ghostly watcher
                                                    -------------------------------

                                                    watcher=kind({
                                                        extends=entity,
                                                        shadow={x=0,y=0,rx=8,ry=4}
                                                    })
                                                    function watcher:render(t)
                                                        local z=sin(t*0.007)*3-3
                                                        local p=self.pos-v(0,z)-
                                                                v(8,24)
                                                        spr(14,p.x,p.y,2,3)
                                                    end

                                                    -------------------------------
                                                    -- building a room
                                                    -------------------------------

                                                    wall=kind({
                                                        walked_into=true,
                                                        extends=entity
                                                    })

                                                    function build_room(mx,my)
                                                        local obtab={}
                                                        for k,gob in pairs(gobs) do
                                                            obtab[gob.tile]=gob
                                                        end

                                                        for ty=0,15 do
                                                            for tx=0,15 do
                                                                local tile=mget(mx+tx,my+ty)
                                                                local spawn=obtab[tile]
                                                                if spawn then
                                                                    solid:new({
                                                                        pos=v(tx,ty)*8+spawn.off,
                                                                        def=spawn
                                                                    })
                                                                    mset(tx,ty,128)
                                                                else
                                                                    mset(tx,ty,tile)
                                                                end
                                                            end
                                                        end
                                                    end

                                                    function flags(pos,mask)
                                                        local x,y=
                                                        mid(pos.x,0,16),
                                                        mid(pos.y,0,15)
                                                        return band(fget(mget(x,y)),mask)
                                                    end

                                                    function process_walls()
                                                        process_walls_with(v(0,1),v(1,0),4,3)
                                                        process_walls_with(v(0,1),v(1,0),8,4)
                                                        process_walls_with(v(1,0),v(0,1),1,1)
                                                        process_walls_with(v(1,0),v(0,1),2,2)
                                                        find_wall_fronts()
                                                    end

                                                    function process_walls_with(dout,din,mask,wdir)
                                                        for row=0,15 do
                                                            local l,c,bv,prv=
                                                            dout*row-din*2,-2,0
                                                            repeat
                                                                repeat
                                                                    prv=bv
                                                                    l+=din
                                                                    c+=1
                                                                    bv=flags(l,mask)
                                                                until c==16 or bv~=prv
                                                                if prv~=0 then
                                                                    add_wall(sl,l,wdir)
                                                                end
                                                                sl=l
                                                            until c==16
                                                        end
                                                    end

                                                    wparams={
                                                        {f=v(0,5),t=v(7,4),
                                                            we=v(0,4)},
                                                        {f=v(7,5),t=v(0,4),
                                                            we=v(7,4),wi=true},
                                                        {f=v(0,5),t=v(-1,13),
                                                            we=v(-1,5),h=v(-1,1),wi=true},
                                                        {f=v(0,12),t=v(-1,0),
                                                            we=v(-1,12),h=v(-1,14)},
                                                    }
                                                    function add_wall(from,to,dr)
                                                        local d,ps=dirs[dr],wparams[dr]
                                                        local cs,ce,co=
                                                        from*8+ps.f,
                                                        to*8+ps.we,
                                                        to*8+ps.t
                                                        local wbox=make_box(cs.x,cs.y,co.x,co.y)
                                                        local hole
                                                        if ps.h then
                                                            local ch=to*8+ps.h
                                                            local hbox=make_box(cs.x,cs.y,ch.x,ch.y)
                                                            hole={
                                                                tl=v(hbox.xl,hbox.yt),
                                                                br=v(hbox.xr,hbox.yb)
                                                            }
                                                        end
                                                        wall:new({
                                                            cbox=wbox,
                                                            walls={
                                                                {
                                                                    s=ps.wi and ce or cs,
                                                                    e=ps.wi and cs or ce,
                                                                    d=-d,
                                                                    h=hole
                                                                }
                                                            }
                                                        })
                                                    end

                                                    -------------------------------
                                                    -- front-facing walls
                                                    -------------------------------

                                                    -- front parts of walls are
                                                    -- drawn as entities to let
                                                    -- us darken them when they
                                                    -- should be in shadow
                                                    wallfront=kind({
                                                        extends=entity
                                                    })
                                                    function wallfront:render()
                                                        dim_object(self)
                                                        map(self.mx,self.my,
                                                            self.pos.x,self.pos.y-16,
                                                            self.mw,2)
                                                    end


                                                    function find_wall_fronts()
                                                        for y=0,14 do
                                                            local pc,c,sx=0
                                                            for x=0,16 do
                                                                c=flags(v(x,y),16)+
                                                                        flags(v(x,y+1),16)
                                                                if c~=pc or c==16 then
                                                                    if pc==32 then
                                                                        w=wallfront:new({
                                                                            mx=sx,my=y,mw=x-sx,
                                                                            pos=v(sx,y+2)*8
                                                                        })
                                                                    end
                                                                    sx=x
                                                                end
                                                                pc=c
                                                            end
                                                        end
                                                    end

                                                    -------------------------------
                                                    -- initialization
                                                    -------------------------------

                                                    function _init()
                                                        init_blending(6)
                                                        init_palettes(16)

                                                        build_room(0,0)
                                                        process_walls()

                                                        ply=indiana:new({
                                                            pos=v(64,120),facing=3
                                                        })
                                                        lgt=light:new({
                                                            pos=v(64,120),bri=1
                                                        })
                                                        watcher:new({
                                                            pos=v(112,24)
                                                        })
                                                    end

                                                    -------------------------------
                                                    -- room generation
                                                    -------------------------------

                                                    -- node
                                                    --  pos : vector
                                                    --  dir : vector/dirno
                                                    --  w : int

                                                    function generate_room()
                                                        --clear
                                                        g_rfill(make_box(0,0,18,19),156)
                                                        --prepare
                                                        local lx,ux,lw,uw=
                                                        flr(rnd(12)+1),
                                                        flr(15-rnd(11)),
                                                        flr(rnd(0)+2),
                                                        flr(rnd(0)+2)

                                                        carve_corridor(
                                                            {pos=v(lx,18),w=lw,d=v(0,-1)},
                                                            {pos=v(ux,0),w=uw,d=v(0,1)}
                                                        )
                                                        --finishing
                                                        g_connect_up(fpats)
                                                        g_connect_up(cpats)
                                                        g_randomize(reps)

                                                        -- indiana
                                                        ply=indiana:new({
                                                            pos=v(lx*8+lw*4,120),facing=3
                                                        })
                                                    end

                                                    function corr_box(n)
                                                        local p,d,w=n.pos,n.d,n.w
                                                        return vec_box(
                                                            p,p+d*w+d:rotcw()*w
                                                        )
                                                    end

                                                    function carve_corridor(n1,n2)
                                                        n1,n2=set({},n1),set({},n2)
                                                        local n,b1,b2
                                                        -- extend towards each other
                                                        while true do
                                                            b1,b2=
                                                            corr_box(n1),
                                                            corr_box(n2)
                                                            g_rfill(b1,128)
                                                            g_rfill(b2,128)
                                                            if ((n2.pos-n1.pos)^n1.d<=n2.w) break
                                                                n=rnd()>0.5 and n1 or n2
                                                                n.pos+=n.d
                                                            end
                                                            -- connect together
                                                            local cbox=make_box(
                                                                min(b1.xl,b2.xl),
                                                                min(b1.yt,b2.yt),
                                                                max(b1.xr,b2.xr),
                                                                max(b1.yb,b2.yb)
                                                            )
                                                            g_rfill(cbox,128)
                                                        end

                                                        function g_rfill(box,tile)
                                                            for x=box.xl,box.xr-1 do
                                                                for y=box.yt,box.yb-1 do
                                                                    mset(x,y,tile)
                                                                end
                                                            end
                                                        end

                                                        fpats={
                                                            --fronts
                                                            {0,0,32,0,1,0, 187},
                                                            {0,0,32,0,2,0, 171},
                                                        }
                                                        cpats={
                                                            --fronts
                                                            ---right
                                                            {0,0,16,0,1,16,1,0,0, 174},
                                                            {0,0,16,1,0,0, 190},
                                                            ---left
                                                            {0,0,16,0,1,16,-1,0,0, 173},
                                                            {0,0,16,-1,0,0, 189},
                                                            --walls
                                                            ---inner corners
                                                            {0,0,32,1,0,32,0,1,32,1,1,16, 132},
                                                            {0,0,32,-1,0,32,0,1,32,-1,1,16, 134},
                                                            {0,0,32,1,0,0,0,1,32,1,1,32, 164},
                                                            {0,0,32,-1,0,0,0,1,32,-1,1,32, 166},
                                                            ---outer corners
                                                            {0,0,32,0,1,16,1,0,0, 185},
                                                            {0,0,32,0,1,16,-1,0,0, 184},
                                                            {0,0,0, 0,1,32, -1,1,0, 168},
                                                            {0,0,0, 0,1,32, 1,1,0, 169},
                                                            ---straights
                                                            {0,0,32,0,1,16, 133},
                                                            {0,0,32,1,0,0, 148},
                                                            {0,0,32,1,0,16, 148},
                                                            {0,0,32,-1,0,0, 150},
                                                            {0,0,32,-1,0,16, 150},
                                                            {0,0,0, 0,1,32, 165}
                                                        }
                                                        function g_cmatch(x,y,m)
                                                            x,y=mid(x,0,15),mid(y,0,15)
                                                            local v=band(fget(mget(x,y)),0x30)
                                                            return v==m
                                                        end

                                                        function g_connect_up(pats)
                                                            for x=0,15 do
                                                                for y=0,15 do
                                                                    for p in all(pats) do
                                                                        local match=true
                                                                        for n=3,#p,3 do
                                                                            if not g_cmatch(x+p[n-2],y+p[n-1],p[n]) then
                                                                                match=false
                                                                                break
                                                                            end
                                                                        end
                                                                        if match then
                                                                            mset(x,y,p[#p])
                                                                            break
                                                                        end
                                                                    end
                                                                end
                                                            end
                                                        end

                                                        reps={
                                                            r128={128,144,160,176},
                                                            r187={187,187,188},
                                                            r171={171,171,172}
                                                        }
                                                        function g_randomize(reps)
                                                            for x=0,15 do
                                                                for y=0,15 do
                                                                    local r=reps["r"..mget(x,y)]
                                                                    if r then
                                                                        mset(x,y,r[flr(rnd(#r)+1)])
                                                                    end
                                                                end
                                                            end
                                                        end

                                                        -------------------------------
                                                        -- main loop
                                                        -------------------------------

                                                        function _update60()
                                                            -- let all objects update
                                                            update_entities()
                                                            -- check for collisions
                                                            -- collision callbacks happen
                                                            -- here
                                                            do_collisions()
                                                        end


                                                        function _draw()
                                                            cls()
                                                            palt()
                                                            palt(0,false)

                                                            -- clip to lit rectangle
                                                            local xl,yt,xr,yb=
                                                            lgt:extents()
                                                            clip(xl,yt,xr-xl+1,yb-yt+1)
                                                            -- store clipping coords
                                                            -- globally to let us
                                                            -- not draw certain objects
                                                            clipbox=make_box(
                                                                xl-8,yt,xr+8,yb+24
                                                            )
                                                            -- background from level map
                                                            map(0,0,0,0,16,16)
                                                            -- under-entity "blob" shadows
                                                            render_blob_shadows()
                                                            -- entities themselves
                                                            render_entities()
                                                            -- "foreground" layer of level
                                                            -- (parts that need to be
                                                            --  on top of entities)
                                                            map(0,0,0,0,16,16,128)
                                                            -- apply lighting to all that
                                                            lgt:apply()
                                                            -- "real" polygonal shadows
                                                            render_wall_shadows()

                                                            show_performance()
                                                        end

                                                        function show_performance()
                                                            clip()
                                                            local cpu=flr(stat(1)*100)
                                                            local fps=-60/flr(-stat(1))
                                                            local perf=
                                                            cpu .. "% cpu @ " ..
                                                                    fps ..  " fps"
                                                            print(perf,0,122,0)
                                                            print(perf,0,121,fps==60 and 7 or 8)
                                                        end