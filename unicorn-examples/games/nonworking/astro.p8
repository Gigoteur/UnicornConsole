pico-8 cartridge // http://www.pico-8.com
version 39
__lua__
-- astropocalypse
-- by picoter8
-- todo:
-- - each level is a solar system
-- - destroying each planet makes you more powerful
-- - destroy all planets before going for the sun
-- - visual damage on planets?
-- - black holes?

one_frame,g_time_scale=1/60,1
abs_one_frame=1/60

align_c,align_r=1,2

function printb(text,x,y,c,bc,a)
 local ox=(a==align_c) and #text*2 or
          (a==align_r) and #text*4 or
          0
 local tx=x-ox
  
 for i=tx-1,tx+1 do
  for j=y-1,y+1 do
   print(text,i,j,bc)
  end
 end
 
 print(text,tx,y,c)
end

--
function time_to_text(time)
 local hours,mins,secs,fsecs=flr(time/3600),flr(time/60%60),flr(time/1%60),flr(time%60*10/1%10)
 if(hours<0 or hours>9)return "8:59:59"
 return hours>0 and hours..":" or ""
        ..((mins>=10 or hours==0) and mins or "0"..mins)
        ..(secs<10 and ":0"..secs or ":"..secs).."."..fsecs
end

--
function angle_fix(a)
 while a>.5 do
  a-=1
 end
 
 while a<-.5 do
  a+=1
 end
 
 return a
end

--
function normalize(x,y)
  local d=max(abs(x),abs(y))
  local n=min(abs(x),abs(y))/d
  local m=sqrt(n*n+1)*d
  return x/m,y/m,m
end

--
function polar_to_xy(r,a)
 return r*cos(a),r*sin(a)
end

--
function dot(x1,y1,x2,y2)
 return x1*x2+y1*y2
end

--
function reflect(x,y,nx,ny,restitution,friction)
 local d=dot(x,y,nx,ny)
 
 if d>0 then
  return x,y
 end

 local vnx,vny=-d*nx,-d*ny
 local tx,ty=x+vnx,y+vny
 
 local rx,ry=restitution*vnx+friction*tx,restitution*vny+friction*ty
 return rx,ry
end


--[[]]
function make_oval(el,t,x,y,vx,vy,d,s1,s2,colors)
 local l=entities[el]
 
 local e=
 {
  tt=t,
  t=t,
  x=x,
  y=y,
  vx=vx,
  vy=vy,
  s1=s1,
  s2=s2,
  sc=s1,
  l=l,
  colors=colors,
  
  update=function(e)
   e.t-=one_frame
   if e.t<=0 then
    del(e.l,e)
   else
    e.sc+=one_frame*(e.s2-e.s1)/e.tt
    e.x+=e.vx*one_frame*60
    e.y+=e.vy*one_frame*60

    e.vx-=e.vx*d*one_frame
    e.vy-=e.vy*d*one_frame
   end
  end,
 
  draw=function(e)
   local ci=flr(1+#e.colors*(1-e.t/e.tt))
   ovalfill(e.x-e.sc,e.y-e.sc,e.x+e.sc,e.y+e.sc,e.colors[ci])
  end,
 }
 
 add(l,e)
 
 return e
end
--]]


function make_line(el,t,x,y,vx,vy,s,colors)
 local l=entities[el]
 
 local e=
 {
  tt=t,
  t=t,
  x=x,
  y=y,
  vx=vx,
  vy=vy,
  s=s,
  l=l,
  colors=colors,
  
  update=function(e)
   e.t-=one_frame
   if e.t<=0 then
    del(e.l,e)
   else
    e.x+=e.vx*one_frame*60
    e.y+=e.vy*one_frame*60
   end
  end,
 
  draw=function(e)
   local ci=flr(1+#e.colors*(1-e.t/e.tt))
   local ci2=min(#e.colors,flr(2+#e.colors*(1-e.t/e.tt)))
   line(e.x,e.y,e.x+2*e.s*e.vx,e.y+2*e.s*e.vy,e.colors[ci2])
   line(e.x,e.y,e.x+e.s*e.vx,e.y+e.s*e.vy,e.colors[ci])
  end,
 }
 
 add(l,e)
 
 return e
end

--
function player_init()
  px=-map_radius*.8
  py=0
  pangle=0
  pfx,pfy=2,0
  pspeed=.5
  pvx,pvy=0,0
  ox=0
  tox=0
  oy=0
  toy=0
  pevx,pevy=0,0

  pbounce_t=0

  penergized=false
  penergysound=false
  penergy_bar=60
  penergy_bar_visual=60

  peyeblink_t=0

  penergy_added_t=0
  penergy_added=0
  penergy_added_x,penergy_added_y=0,0

  pgameover=false
  pgamewon=false
  ptimer=0
end

--
function player_add_energy(v)
  if(pgameover)return
  penergy_added_t=.75
  penergy_added=v
  penergy_added_x,penergy_added_y=px,py
  penergy_bar=mid(0,penergy_bar+v,60)
end

--
function bounce_explosion(v,nx,ny,fast)
  local x,y=v.x-v.size*nx,v.y-v.size*ny
  local a=atan2(-nx,-ny)

  for i=1,24 do
    local vx,vy=polar_to_xy(4+rnd(fast and 12 or 6),a+.2-rnd(.4))-- 4*nx,4*ny
    local t,d,rs=.5+rnd(1.5),8,2
    make_oval(layer_post_enemy_fx,t,x,y,vx,vy,d,6+rs,0,{10,9,8,2})
  end

  for i=1,12 do
    local vx,vy=polar_to_xy(4+rnd(fast and 12 or 6),a+.2-rnd(.4))-- 4*nx,4*ny
    local t,d,rs=.5+rnd(1.5),8,2
    make_line(layer_post_enemy_fx,t,x,y,vx,vy,6+rs,{10,9,8,2})
  end
end

--
function player_update()
  if(not pgameover and not pgamewon)ptimer+=one_frame
  pbounce_t=max(0,pbounce_t-abs_one_frame)
  peyeblink_t=max(0,peyeblink_t-one_frame)
  penergy_added_t=max(0,penergy_added_t-abs_one_frame)

  local pcx,pcy=0,0

  if not pgameover then
    if(btn(0))pcx-=1
    if(btn(1))pcx+=1
    if(btn(2))pcy-=1
    if(btn(3))pcy+=1
  end

  local nx,ny,m=normalize(pcx,pcy)

  if m>0 then
    local a=angle_fix(atan2(nx,ny)-pangle)

    pangle+=a*.03
    pangle=angle_fix(pangle)
  end

  penergized=btn(4) and penergy_bar>0

  if penergized then
    pspeed=min(4,pspeed+.1)
  elseif m>0 and pspeed<=2 then
    --speed up
    pspeed=min(2,pspeed+.025)
  else
    --slow down
    pspeed=max(1,pspeed-.0125)
  end

  if penergized then
    if not penergysound then
      penergysound=true
      sfx(3)
    end
  else
    if penergysound then
      penergysound=false
      sfx(3,-2)
    end
  end

  if not pgameover and not pgamewon then
    if btn(4) then
      penergy_bar=max(0,penergy_bar-2*one_frame)
    else
      penergy_bar=max(0,penergy_bar-one_frame)
    end
  end

  if penergy_bar<=0 then
    pgameover=true
  end

  pvx,pvy=polar_to_xy(pspeed,pangle)

  px+=(pvx+pevx)*one_frame*60
  py+=(pvy+pevy)*one_frame*60

  -- check bounce with edge of map
  local nx,ny,m=normalize(px,py)

  if m>(map_radius-16) then
    pvx,pvy=reflect(pvx,pvy,-nx,-ny,.75,.75)
    m=map_radius-16
    px,py=m*nx,m*ny
    pangle=atan2(pvx,pvy)
    pspeed=1
    pevx,pevy=pvx,pvy
    pbounce_t=.4

    sfx(6)
  end

  -- check bounce with enemy planets
  for k,v in pairs(entities[layer_enemies]) do
    if v.bounce_t<=0 then
      local nx,ny,m=normalize(v.x-px,v.y-py)
      if m<=12+v.size then

        if pbounce_t<=.2 then
          -- todo: spawn particles here
          local fast=pspeed>3.5

          bounce_explosion(v,nx,ny,fast)

          pbounce_t=.4
        end

        enemy_damage(v,nx,ny)

        pevx,pevy=reflect(pvx,pvy,-nx,-ny,.75,.75)
        pvx,pvy=0,0
        m=12+v.size
        px,py=v.x-m*nx,v.y-m*ny
        pangle=atan2(pevx,pevy)
        pspeed=0
      end
    end
  end

  if abs(pevx)>.01 then
    pevx*=.99
  else
    pevx=0
  end

  if abs(pevy)>.01 then
    pevy*=.99
  else
    pevy=0
  end

  --random blinks
  if rnd()>.995 then
    peyeblink_t=.2
  end

  -- spawn player effects
  --make_oval(el,t,x,y,vx,vy,d,s1,s2,colors,outlined)
  local nx,ny,m=normalize(pvx,pvy)
  local fast=penergized
  local x,y=px+(fast and (2-rnd(4)) or 0),py+(fast and (2-rnd(4)) or 0)
  local vx,vy=-4*nx-2*pvx+(fast and (2-rnd(4)) or 0),-4*ny-2*pvy+(fast and (2-rnd(4)) or 0)
  local t,d,rs=.5+rnd(),40,rnd(fast and 4 or 1)
  make_oval(layer_pre_pre_player_fx,t,x,y,vx,vy,d,18+rs,0,penergized and {10} or {7})
  make_oval(layer_pre_player_fx,t,x,y,vx,vy,d,14+rs,0,penergized and {7,9,8,2} or {9,8,2})
end

--
function player_draw()

  local x,y=px+(penergized and (1-rnd(2)) or 0),py+(penergized and (1-rnd(2)) or 0)
	 
  if not pgameover then
    if(btn(0))tox=-8 toy=0
    if(btn(1))tox=0 toy=0

    if(btn(2))toy=-3
    if(btn(3))toy=5
  end

  dif=(tox-ox)
  if abs(dif)>1 then
    ox+=dif*.2
  else
    ox=tox
  end

  dif=(toy-oy)
  if abs(dif)>1 then
    oy+=dif*.2
  else
    oy=toy
  end

  oox=(ox<-4 and -1 or 1)

  circfill(x-oox,y,12,0)
  circfill(x,y,12,8)

  -- mouth
  local mox,moy=x+4+ox,y+4+oy
  local moxs,moys=penergized and 4 or 1,penergized and 3 or 1
  ovalfill(mox-moxs,moy-moys,mox+moxs,moy+moys,0)

  -- eyes
  if pbounce_t<=0 and peyeblink_t<=0 then
    local ex,ey=x+ox,y+oy
    local eys=penergized and 1 or 2
    ovalfill(ex+1-2,ey-eys,ex+1+2,ey+eys,7)
    ovalfill(ex+7-2,ey-eys,ex+7+2,ey+eys,7)
    pset(x+1+ox+oox,y+oy,0)
    pset(x+7+ox+oox,y+oy,0)
  end

  -- eyelashes
  line(x-2+ox,y-4+oy,x+4+ox,y-1+oy,0)
  line(x+10+ox,y-4+oy,x+5+ox,y-1+oy,0)
end

--
function make_player()
  local e=
  {
    init=player_init,
    update=player_update,
    draw=player_draw,
  }

  add(entities[layer_player],e)

  return e
end


--[[]]
function make_eb(t,x,y,vx,vy,s)
 local l=entities[layer_ebs]
 
 local e=
 {
  t=t,
  x=x,
  y=y,
  vx=vx,
  vy=vy,
  size=s,
  l=l,
  lt=t,
  
  update=function(e)
    e.t-=one_frame
    if e.t<=0 then
      del(e.l,e)
      --explosion(e.x,e.y,.3)
    else
      e.x+=e.vx*one_frame*60
      e.y+=e.vy*one_frame*60
    end

    local nx,ny,m=normalize(e.x-px,e.y-py)
    if m<=12+e.size then
      if pbounce_t<=.2 then
        local fast=pspeed>3.5
        bounce_explosion(e,nx,ny,fast)
        pbounce_t=.4
      end

      pevx,pevy=reflect(pvx,pvy,-nx,-ny,.75,.75)
      pvx,pvy=0,0
      m=12+e.size
      px,py=e.x-m*nx,e.y-m*ny
      pangle=atan2(pevx,pevy)
      pspeed=0

      player_add_energy(-5)

      del(e.l,e)
    end
  end,
 
  draw=function(e)
   local t=time()%.2>.1
   circfill(e.x,e.y,e.size+1,t and 10 or 8)
   circfill(e.x,e.y,e.size,t and 8 or 10)
  end,
 }
 
 add(l,e)
 
 return e
end
--]]

--
function enemy_init(e,radius,speed,size)
  e.radius,e.speed,e.size=radius,speed,size

  e.angle=rnd()
  e.x,e.y=polar_to_xy(e.radius,e.angle)

  e.color=rnd({2,4,7,8,9,13})

  e.bounce_t=0

  e.evx,e.evy=0,0

  e.damage=0
end

--
function enemy_explosion(e)
  local oval_count=e.sun and 48 or 32
  local oval_size=e.sun and 18 or 12
  local debris_size=e.sun and 30 or 20
  local line_count=e.sun and 24 or 12

  local x,y=e.x,e.y
  for i=1,oval_count do
    local vx,vy=polar_to_xy(4+rnd(8),rnd())
    local t,d=1+rnd(3),8
    make_oval(layer_post_enemy_fx,t,x,y,vx,vy,d,oval_size,0,{10,9,8,2})
    make_oval(layer_post_enemy_fx,t,x,y,vx*.4,vy*.4,d,debris_size,0,{7,6,5,1})
  end

  for i=1,line_count do
    local vx,vy=polar_to_xy(4+rnd(8),rnd())
    local t,d=1+rnd(3),8
    make_line(layer_post_enemy_fx,t,x,y,vx,vy,12,{10,9,8,2})
  end
end

-- 
function enemy_random_crash_sfx()
  local r=rnd()
  sfx(r>.66 and 0 or r>.33 and 1 or 2)
end

--
function enemy_damage(e,nx,ny)
  e.bounce_t=.4

  if not e.sun then
    e.damage+=1

    e.evx=.75*pspeed*nx
    e.evy=.75*pspeed*ny

    if e.damage>=5 then
      enemy_explosion(e)
      del(entities[layer_enemies],e)
      player_add_energy(10)
      sfx(4)
    else
      enemy_random_crash_sfx()
    end
  else
    if #entities[layer_enemies]==1 then
      e.damage+=1
      e.size-=5

      if e.damage>=10 then
        enemy_explosion(e)
        del(entities[layer_enemies],e)
        pgamewon=true
        sfx(4)
      else
        enemy_random_crash_sfx()
      end
    else
      enemy_random_crash_sfx()
    end

    player_add_energy(5)
  end
end

--
function enemy_update(e)
  e.bounce_t=max(0,e.bounce_t-one_frame)

  e.evx*=.99
  e.evy*=.99

  if(abs(e.evx)<.1)e.evx=0
  if(abs(e.evy)<.1)e.evy=0

  if e.bounce_t>0 then
    local nx,ny,m=normalize(e.x+e.evx*one_frame*60,e.y+e.evy*one_frame*60)
    e.angle=atan2(nx,ny)
    e.radius=m
  else
    e.angle+=e.speed*one_frame*60/e.radius
  end

  -- check out of bounds bounces
  if not e.sun then
    local min_r=sun.size+e.size
    local max_r=map_radius-e.size

    -- check bounce with edge of map
    if e.radius>max_r or e.radius<min_r then
      local nx,ny,m=normalize(e.x+e.evx*one_frame*60,e.y+e.evy*one_frame*60)

      if e.radius>max_r then
        e.evx,e.evy=reflect(nx,ny,-nx,-ny,.75,.75)
      else
        e.evx,e.evy=reflect(-nx,-ny,nx,ny,.75,.75)
      end
    end
  end

  if e.sun and #entities[layer_enemies]==1 then
    -- all planets are gone and only the sun is left
    if rnd()>.98 then
      local nx,ny,m=normalize(px-e.x,py-e.x)
      if m<256 then
        local dx=mid(-20,(e.x-px)/4,20)/20
        local mouth_x,mouth_y=e.x-.3*e.size*dx,e.y+.25*e.size
        local speed=1
        --make_eb(t,x,y,vx,vy,s)
        nx,ny,m=normalize(px-e.x+m/speed*(pvx+pevx),py-e.x+m/speed*(pvy+pevy))
        make_eb(10,mouth_x,mouth_y,speed*nx,speed*ny,5)
      end
    end
  end

  e.x,e.y=polar_to_xy(e.radius,e.angle)
end

--
function enemy_draw(e)
  local shake=e.damage
  local x,y,s=e.x+shake-rnd(shake*2),e.y+shake-rnd(shake*2),e.size
  
  -- body
  if e.sun then
    local rs=rnd(2)
    circ(x,y,s+3+rs,10)
    local rs=rnd(2)
    circ(x,y,s+5+rs,10)
  else
    circfill(x,y,s+3,0)
  end

  circ(x,y,s+2,10)
  circ(x,y,s+1,10)
  circfill(x,y,s,e.color)

  local dx=mid(-20,(x-px)/4,20)/20

  -- eyes
  local e1x,e2x=x-.3*s-.3*s*dx,x+.3*s-.3*s*dx
  local ey,es=y-.25*s,.15*s

  if e.bounce_t>0 then
    ovalfill(e1x-.2*s,ey-.025*s,e1x+.2*s,ey+.025*s,0)
    ovalfill(e2x-.2*s,ey-.025*s,e2x+.2*s,ey+.025*s,0)

  else
    ovalfill(e1x-es,ey-es,e1x+es,ey+es,0)
    ovalfill(e2x-es,ey-es,e2x+es,ey+es,0)
  end

  -- mouth
  local mouth_x,mouth_y,mouth_s=x-.3*s*dx,y+.25*s,.3*s

  if e.bounce_t>0 then
    ovalfill(mouth_x-mouth_s*1.2,mouth_y-mouth_s*.1,mouth_x+mouth_s*1.2,mouth_y+mouth_s*.1,0)
  else
    ovalfill(mouth_x-mouth_s*1.2,mouth_y-mouth_s,mouth_x+mouth_s*1.2,mouth_y+mouth_s,0)
  end
end

--
function make_enemy(radius,speed,size)
  local e=
  {
    init=function(e) enemy_init(e,radius,speed,size) end,
    update=enemy_update,
    draw=enemy_draw,
  }

  add(entities[layer_enemies],e)

  return e
end

--
function make_sun()
  local e=make_enemy(0,.1,60+rnd(20))
  e.sun=true
  e:init()

  return e
end

--
function cam_init()
  -- camera
  cam_tx,cam_ty=px-64,py-64
  camx,camy=cam_tx,cam_ty
end

--
function cam_update()
  local tx,ty=px+12*pvx,py+12*pvy
  cam_tx,cam_ty=mid(tx-64-8,cam_tx,tx-64+8),mid(ty-64-8,cam_ty,ty-64+8)

  camx+=(cam_tx-camx)*.4
  camy+=(cam_ty-camy)*.4

  -- todo: make camera shakers better
  cam_shake=40*pbounce_t
end

--
function make_camera()
  local e=
  {
    init=cam_init,
    update=cam_update,
  }

  add(entities[layer_camera],e)

  return e
end

function approx_dist(dx,dy)
 local maskx,masky=dx>>31,dy>>31
 local a0,b0=(dx+maskx)^^maskx,(dy+masky)^^masky
 if a0>b0 then
  return a0*0.9609+b0*0.3984
 end
 return b0*0.9609+a0*0.3984
end

--[[]]
function make_minimap()
  local e=
  {
    draw=function(e)
      camera()
      local mx,my,ms=64,128-12,24
      fillp(0b0101101001011010.1)
      circfill(mx,my,ms,1)
      fillp()

      pset(mx,my-1,t()%.2>.1 and 12 or 13)

      for k,v in pairs(entities[layer_enemies]) do
        local dx,dy=camx+64-v.x,camy+64-v.y

        local m=approx_dist(dx,dy)

        if m<map_radius then
          local x,y=mx-ms*dx/map_radius,my-ms*dy/map_radius
          if v.sun then
            circfill(x,y,1,10)
          else
            pset(x,y,8)
          end
        end
      end

      circ(mx,my,ms+1,7)

      -- energy bar
      penergy_bar_visual+=(penergy_bar-penergy_bar_visual)*.1

      local bc=(penergy_bar_visual<=15 and t()%.2>.1) and 8 or 7
      for a=.17,.33,.001 do
        local x,y=polar_to_xy(112,a)
        circfill(mx+x,my+y,3,bc)
      end

      local color=penergy_bar_visual<15 and 8 or
                  penergy_bar_visual<30 and 9 or 10
      local ea=.17+(.33-.17)*(1-penergy_bar_visual/60)
      for a=.17,.33,.001 do
        local x,y=polar_to_xy(112,a)
        circfill(mx+x,my+y,2,a<ea and 0 or color)
      end

      printb(flr(penergy_bar_visual).."",64,2,0,bc,align_c)

      -- energy added points
      if penergy_added_t>0 then
        local positive_energy=penergy_added>0
        local c=positive_energy and 10 or 8
        local blink=penergy_added_t%.2>.1
        local oy=20-20*penergy_added_t/.75
        local text=positive_energy and ("+"..penergy_added) or (""..penergy_added)
        printb(text,
               penergy_added_x-camx,
               penergy_added_y-camy-10-oy,
               blink and 1 or c,
               blink and c or 1,align_c)
      end

      local blink=t()%.2>.1
      local c,bc=blink and 10 or 0,blink and 8 or 7

      if penergy_bar_visual>0 and penergy_bar_visual<15 then
        if(t()%1>.9)sfx(5)
      end

      if pgameover then
        printb("game over!",64,20,c,bc,align_c)
        printb("press z+x to play again!",64,30,c,bc,align_c)
      elseif pgamewon then
        printb("sun destroyed!",64,20,c,bc,align_c)
        printb("thanks for playing this demo!",64,30,c,bc,align_c)
        printb("press z+x to play again!",64,40,c,bc,align_c)
      elseif #entities[layer_enemies]==1 then
        printb("destroy the sun!",64,20,c,bc,align_c)
      end

      --printb(,1,120,0,7)
      printb("\147"..time_to_text(ptimer),3,120,0,7)
      --rect(0,0,127,127,8)
    end,
  }

  add(entities[layer_ui],e)

  return e
end
--]]

function make_background()

  local e=
  {
    init=function(e)
      stars={}
      local sr=map_radius+128
      local star_count=100+game_level*50
      for i=1,star_count do
        add(stars,{
          x=sr-rnd(sr*2),
          y=sr-rnd(sr*2),
          z=1+rnd(4),
          t=.1+rnd(.4),
          s=rnd(),
          c=rnd({7,9,8,2})})
      end
    end,

    draw=function(e)
      --[[ map grid
      for i=-1024,1024,64 do
      line(-1024,i,1024,i,1)
      line(i,-1024,i,1024,1)
      end
      --]]

      local ccx,ccy=camx+64,camy+64
      for k,v in pairs(stars) do
        local x,y=(v.x-ccx)/v.z,(v.y-ccy)/v.z
        if abs(x)<64 and abs(y)<64 then
          local s=v.s+v.s*sin(v.t*t())
          ovalfill(x+ccx-s,y+ccy-s,x+ccx+s,y+ccy+s,v.c)
        end
      end

      for i=64,map_radius-64,64 do
        circ(0,0,i,1)
      end

      for a=0,1,.0625 do
        local x,y=map_radius*cos(a),map_radius*sin(a)
        line(0,0,x,y,1)
      end

      for k,v in pairs(entities[layer_enemies]) do
        circ(0,0,v.radius,8)
      end

      -- arena radius
      circ(0,0,map_radius-2,10)
      circ(0,0,map_radius,8)
      circ(0,0,map_radius+2,10)
    end,
  }

  add(entities[layer_background],e)

  return e
end


-- entity stuff
layer_buttons,
layer_background,
layer_pre_pre_player_fx,
layer_pre_player_fx,
layer_player,
layer_camera,
layer_enemy_fx,
layer_enemies,
layer_pickups,
layer_prop,
layer_pbs,
layer_post_enemy_fx,
layer_ebs,
layer_ui
=1,2,3,4,5,6,7,8,9,10,11,12,13,14

entities={}

entities[layer_buttons],
entities[layer_background],
entities[layer_pre_pre_player_fx],
entities[layer_pre_player_fx],
entities[layer_player],
entities[layer_camera],
entities[layer_pbs],
entities[layer_enemy_fx],
entities[layer_enemies],
entities[layer_pickups],
entities[layer_prop],
entities[layer_post_enemy_fx],
entities[layer_ebs],
entities[layer_ui]
={},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}


--
function _init()
  music(31,0,3)

  game_level=6

  map_radius=256+game_level*80
  enemy_count=6+game_level*2


  p=make_player()
  cam=make_camera()
  map=make_minimap()
  background=make_background()

  sun=make_sun()

--[[]]
  for i=1,enemy_count do
    local speed=rnd()>.5 and (.1+rnd(.2)) or (-.1-rnd(.2))
    local size=16+rnd(game_level*4)
    local min_r=sun.size+size
    local max_r=map_radius-size
    local radius=min_r+rnd(max_r-min_r)
    make_enemy(radius,speed,size)
  end
--]]

  for i=1,#entities do 
    for k,v in pairs(entities[i]) do
      if(v.init)v:init()
    end
  end  
end

--
function _update()
  if pbounce_t<=.25 or pbounce_t>=.35 then
   if(g_time_scale<1)g_time_scale=min(1,g_time_scale+.25)
  else
   local scale=0
   if(g_time_scale>scale)g_time_scale=max(scale,g_time_scale-.5)
  end

  one_frame=g_time_scale/stat(8)
  abs_one_frame=1/stat(8)

  if pgameover or pgamewon then
    if btn(4) and btn(5) then
      run()
    end
  end

  for i=1,#entities do 
    for k,v in pairs(entities[i]) do
    if(v.update)v:update()
    end
  end  
end

function _draw()
  cls()
  camera(camx+cam_shake*.5-rnd(cam_shake),camy+cam_shake*.5-rnd(cam_shake))

  if pgameover then
    pal(split"5,5,3,4,5,6,7,5,6,7,11,12,13,6,15,0")
  else
    pal()
  end

  for i=1,#entities do 
    for k,v in pairs(entities[i]) do
      if(v.draw)v:draw()
    end
  end

  camera()
  --print(""..stat(1),0,122,7)
end
__gfx__
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00700700000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00077000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00077000000000000000000000000000000000000000000000000000000000009999900000000000000000000000000000000000000000000000000000000000
00700700000000000000000000000000000000000090000000000000000000990990099900000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000099999900000000000099999099009999000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000009990999000000099999999909990099999900000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000999999990000009999900900999999999999000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000009999999990000090990099099999999990999000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000009999999999990009999099999999999990909990000000000000000000000000000000000000000000000000000000000
00000000000000000000000000099999999999990900099909999999999999999099900000000000000000000000000000000000000000000000000000000000
00000000000000000000000099999999999900000909990999099099999099090090900000000000000000000000000000000000000000000000000000000000
00000000000000000000000999999999999999990090999999909999099990900909000000000000000000000000000000000000000000000000000000000000
00000000000000000000009999999999999999909999000999999999999999999090099000000000000000000000000000000000000000000000000000000000
00000000000000000000999999999900000009999999999000099090999999999909990000000000000000000000000000000000000000000000000000000000
00000000000000000099909999900000000000999990090099909909999999999999900000000000000000000000000000000000000000000000000000000000
00000000000000000909990990000000000000999999999999990999999999999999900000000000000000000000000000000000000000000000000000000000
00000000000000009099909000000eeeeeee00000999999990999999999999999999900000000000000000000000000000000000000000000000000000000000
000000000000000909999900000eeeeeeeeee0000099999999999099999999999999000000000000000000000000000000000000000000000000000000000000
00000000000000909999900000eeeeeeeeeee0e00009099900990909999999999999000000000000000000000000000000000000000000000000000000000000
000000000000099999009000eeeeeeeeeeeeeeee0000909909909099999999990990000000000000000000000000000000000000000000000000000000000000
000000000000099999990000eeeeeeeeee0eeeee0000999990990999999999999900000000000000000000000000000000000000000000000000000000000000
00000000000099999999000ee0eeeeeee0eeeeeee000099990999999999999909000000000000000000000000000000000000000000000000000000000000000
00000000000099999999000eee00eeee0eeeeeeee0e0099999990999999099090000000000000000000000000000000000000000000000000000000000000000
00000000000909999999000eeeee00e0e777eeeee000009999909999999090900000000000000000000000000000000000000000000000000000000000000000
0000000000090999999900eeee777e0e77777eeeee0e009999009999999909000000000000000000000000000000000000000000000000000000000000000000
0000000000099999999000eee77007ee70077eeeeeee009999999999999990000000000000000000000000000000000000000000000000000000000000000000
0000000000099999999000eee77007ee70077eeeeee0000999090999999000000000000000000000000000000000000000000000000000000000000000000000
0000000000999999999000eee77777ee77777eeee0e0000999909999990000000000000000000000000000000000000000000000000000000000000000000000
0000000000999999999000eee77777eee777eeeee000000999999999000000000000000000000000000000000000000000000000000000000000000000000000
000000000090999909900eeeee777eeeeeeeeeee0e0e000999999900000000000000000000000000000000000000000000000000000000000000000000000000
000000000090999999900eeeeeeeeeeeeeeeeeeee0ee000999099000000000000000000000000000000000000000000000000000000000000000000000000000
0000000000909999999000eeeeeeee0ee0eeeeeee000000999990000099000000000000000000000000000000000000000000000000000000000000000000000
00000000009999909990000eeeeeeee00eeeeeee00e0009999900000999900000000000000000000000000000000000000000000000000000000000000000000
000000000099999909990000eeeeeeeeeeeeeeee0e00009999000099999900000000000000000000000000000000000000000000000000000000000000000000
0000000000099999099990000eeeeeeeeeeeeee0e000009999000999999900000000000000000000000000000000000000000000000000000000000000000000
00000000000090990999900000eeeeeeeeeeeee00000099999099009099000000000000000000000000000000000000000000000000000000000000000000000
000000000000999900999900000eeeeeeeeeee000000999099999999099000000000000000000000000000000000000000000000000000000000000000000000
0000000000000999999999900000eeeeeee000000009909990999999990000000000000000000000000000000000000000000000000000000000000000000000
00000000000009099999999000000eeeeeee00000099099999999999900000000000000000000000000000000000000000000000000000000000000000000000
00000000000000999999999900000000000000000990999999099999000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000090999999099000000000000009999909990999990000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000090999999999990000000000090909999999999900000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000009099999999909990000009999099099990990000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000900999999999099999999999999909999900000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000099099999999909999900999999009999000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000909999999999999999999990999990000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000099909900999000000009999999000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000999999999999999999999900000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000999999999999900090000000000000000000000000000000000000000000000000000000000000000000000000000000000000
__label__
999999999999999999999990000000000000009999999999999999999999999aa099999999997777000000000000000000000000000000000000010800000000
999999999999999999999900000000000000000999999999777777777777777777777777777777778000000000000000000000aaaaaaaaaaa000001080000000
999999999999999999990000000000000000000007777777aaaaaaaaaaa7000707aaaaaaaaaaaaaa777777000000000000aaaaaaaaaaaaaaaaaaa00008000000
99999999999999999990000000000000000007777aaaaaaaaaaaaaaaaaa707770777aaaaaaaaaaaaaaaaaa777770000aaaaaaadddddddddddaaaaaaa00000000
9999999999999999990000000000000007777aaaaaaaaaaaaaaaaaaaaaa700070007aaaaaaaaaaaaaaaaaaaaaaa7777aaadddddddddddddddddddaaaaa000000
999999999999999999000000000000777aaaaaaaaaaaaaaaaaaaaaaaaaa777070707aaaaaaaaaaaaaaaaaaaaaaaaaaa777ddddddddddddddddddddddaaaa0000
999999999999999990000000000777aaaaaaaaaaaaaaaaaaaaaaaaaaaaa700070007aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa7777ddddddddddddddddddddaaaa00
999999999999999990000000777aaaaaaaaaaaaaaaaaaaaaaa7777777777777777777777777777aaaaaaaaaaaaaaaaaaaaaaaa77dddddddddddddddddddda0a0
999999999999999900000777aaaaaaaaaaaaaaaaaaaa777777999999999999aa09999999999999777777aaaaaaaaaaaaaaaaaaaa777ddddddddddddddddddaaa
999999999999999900077aaaaaaaaaaaaaaaaaaa7777099999999999999999aa0999999999999999777777777aaaaaaaaaaaaaaaaaa77ddddddddddddddddd0a
9999999999999999077aaaaaaaaaaaaaaaaa77770000099999999999999999aa09999999999999999770a0add7777aaaaaaaaaaaaaaaa77ddddddddddddddddd
99999999999999777aaaaaaaaaaaaaaa777700000000099999999999999999aa0999999999999999990a0addddddd777aaaaaaaaaaaaaaa777dddddddddddddd
99999999999977aaaaaaaaaaaaaaa77700000000000009999999999999999aa0999999999999999990a0addddddddddd777aaaaaaaaaaaaaa077dddddddddddd
999999999997aaaaaaaaaaaaaa77700000000000000009999999999999999aa0999999999999999900aaddddddddddddddd777aaaaaaaaaaaa0077dddddddddd
99999999977aaaaaaaaaaaaa7700000000000000000099999999999999999aa099999999999999990aaddddddddddddddddddd77aaaaaaaaaa00007ddddddddd
999999997aaaaaaaaaaaa777000000000000000000009999999999999999aa009999999999999990aa0ddddddddddddddddddddd777aaaaaaa000007dddddddd
99999997aaaaaaaaaaa77000000000000000000000099999999999999999aa09999999999999990a0addddddddddddddddddddddddd77aaaa00000007ddddddd
99999997aaaaaaaaa7700000000000000000000000099999999999999990aa09999999999999990aadddddddddddddddddddddddddddd777000000007ddddddd
99999997aaaaaaa77990000000000000000000000099999999999999999aa09999999999999990aadddddddddddddddddddddddddddddddd700000007ddddddd
999999997aaaa7799999000000000000000000000999999999999999990aa09999999999999900aaddddddddddddddddddddddddddddddddd7700007dddddddd
9999999997777999999999000000000000000009999999999999999999aa09999999999999990aadddddddddddddddddddddddddddddddddddd7777ddddddddd
999999999999999999999990000000000000009999999999999999999a0a09999999999999990aadddddddddddd0000ddddddddddddddd0000dddddddddddddd
999999999999999999999999990000000009999999999999999999999aa09999999999999990aadddddddddddd000000ddddddddddddd000000ddddddddddddd
99999999999999999999999999999999999999999999999999999999aa009999999999999990aaddddddddddd00000000ddddddddddd00000000dddddddddddd
9999999999999999999999999999999999999999999999999999999a0a099999999999999990aadddddddddd0000000000ddddddddd0000000000ddddddddddd
9999999999999999999999999999999999999999999999999999990aa099999999999999990aaddddddddddd0000000000ddddddddd0000000000ddddddddddd
999999999999999999999999999999999999999999999999999999aa0999999999999999990aaddddddddddd0000000000ddddddddd0000000000ddddddddddd
99999999999999999999999999999999999999999999999999999aa00999999999999999990aaddddddddddd0000000000ddddddddd0000000000ddddddddddd
9999999999999999999999999999999999999999999999999999a0a09999999999999999990aaddddddddddd0000000000ddddddddd0000000000ddddddddddd
999999999999999999999999999999999999999999999999999a0a09999999999999999990aaddddddddddddd00000000ddddddddddd00000000dddddddddddd
99999999999999999999999999999999999999999999999999a0a099999999999999999990aadddddddddddddd000000ddddddddddddd000000ddddddddddddd
0999999999999999999999999999999999999999999999990aaa0799999999999999999990aaddddddddddddddd0000ddddddddddddddd0000dddddddddddddd
aa999999999999999999999999999999999999999999999aaa007999999999999999999990aaddddddddddddddddddddddd0000000dddddddddddddddddddddd
a0a9999999999999999999999999999999999999999999a0a0077999999999999999999990aadddddddddddddddddddd0000000000000ddddddddddddddddddd
0aaaa999999999999999999999999999999999999999aaaa07777999999999999999999990aaddddddddddddddddddd000000000000000dddddddddddddddddd
000a0a0999999999999999999999999999999999990a0a0077777999999999999999999990aadddddddddddddddddd00000000000000000ddddddddddddddddd
0000aaaa099999999999999999999999999999990aaaa00077779999999999999999999990aaddddddddddddddddd0000000000000000000dddddddddddddddd
000000aaaaa999999999999999999999999999aaaaa0000077779999999999999999999990aadddddddddddddddd000000000000000000000ddddddddddddddd
00000000aaaaaa999999999999999999999aaaaaa000000777779999999999999999999990aadddddddddddddddd000000000000000000000ddddddddddddddd
00000000000aaaaaaa9999999999999aaaaaaa000000000777799999999888888809999990aaddddddddddddddd00000000000000000000000dddddddddddddd
00000000000000aaaaaaaaaaaaaaaaaaaaa0000080000007777999999888888888880999990aadddddddddddddd00000000000000000000000dddddddddddddd
000000000000000000aaaaaaaaaaaaa00000000008000007777999988888888888888809990aadddddddddddddd00000000000000000000000dddddddddddddd
000000000000000000000000000000010000000000800077777999888888888888888880990aadddddddddddddd00000000000000000000000dddddddddddddd
000000000000000000000000000000100000000000080077777998888888888888888888090aadddddddddddddd00000000000000000000000dddddddddddddd
0000000000000000000000000000010000000000000087777799888888888888888888888090aaddddddddddddd00000000000000000000000dddddddddddddd
0000000000000000000000000000100000000000000087777999888888888888888888888090aadddddddddddddd000000000000000000000ddddddddddddddd
aa00000000000000000000000001000000000000000007777998888888888888888888888800aadddddddddddddd000000000000000000000ddddddddddddddd
aaaa0000000000000000000000100000000000000000777779988888888888888888888888090aadddddddddddddd0000000000000000000dddddddddddddddd
00a0a000000000000000000001000000000000000000777799888888888888888888888888800aaddddddddddddddd00000000000000000ddddddddddddddddd
000aaa000000000000000000100000000000000000007777998888888888888888888888888000aaddddddddddddddd000000000000000dddddddddddddddddd
a0000aaa0000000000000001000000000000000000007777998888888888888888888888888090aadddddddddddddddd0000000000000ddddddddddddddddddd
aa0000a0a0000000000000100000000000000000000077779988888888888888888888888880970aadddddddddddddddddd0000000dddddddddddddddddddddd
0aaa000aaa000000000001000000000000000000000077779988888088888888888088888880970a0adddddddddddddddddddddddddddddddddddddddddddddd
22a0a0000aaa00000000100000000000000000000000777799888888008888888008888888809770aa0ddddddddddddddddddddddddddddddddddddddddddddd
222aaa0000a0a00000010000000000000000000000007777998888888700888007888888888097770aaddddddddddddddddddddddddddddddddddddddddddddd
22220aaa000a0a00001000000000000000000000000077779998888877770007777888888809977700aadddddddddddddddddddddddddddddddddddddddddddd
222222a0a000a0a0010000000000000000000000000077777998888870777870777888888809777770a0addddddddddddddddddddddddddddddddddddddddddd
2222222aaa000a0a1000000000000000000000000000077779998888777778777778888880997777000a0adddddddddddddddddddddddddddddddddddddddddd
222222220aa000a1a0000000000000000000000000000777799988888777888777888888809977770000a0addddddddddddddddddddddddddddddddddddddddd
2222222220aa001a0a0000000000000000000000000007777799988888888088888888880997777700000aaa0ddddddddddddddddddddddddddddddddddddd0a
22222222220aa100a0a00000000000000000000000000077777999888888000888888880997777700000900aaadddddddddddddddddddddddddddddddddddaaa
222222222220aa000a0a00000000000000000000000000777779999888888088888888099977777000011900a0addddddddddddddddddddddddddddddddda0a0
2222222222220aa000a0a00000000000000000000000000777779999988888888888099997777781011999990aaaadddddddddddddddddddddddddddddaaaa00
22222222222220aa000aa0000000000000000000000000007777799999988888880999997777708110099999000aaaadddddddddddddddddddddddddaaaa0000
222222222222220aa000aa00000000000000000000000000077777799999999999999977777711081000999000000aaaaadddddddddddddddddddaaaaa000000
2222222222222220aa000aa000000000000000000000000000777777999999999999977777710008100000000000000aaaaaaadddddddddddaaaaaaa00000000
2222222222222222a0a00a0a00000000000000000000000000077777777999999977777777000008100000000000000000aaaaaaaaaaaaaaaaaaa00000000000
22222222222222222aa000aa000000000000000000000000000077777777777777777777700000008100000000000000000000aaaaaaaaaaa000008000000000
222222222222222222aa000aa0000000000000000000000000000777777777777777777700000000810000000000000000000000000000000000008000000000
2222222222222222220aa000aa000000000000000000000000000007777777777777770000000000801000000000000000000000000000800000008000000000
2222222222222222222a0a00a0a00000000000000000000000000000007777777770000000000000081000000000000000000000000000800000008000000000
22222222222222222222aa000aa00000000000000000000000000000000180000000000000000000081000000000000000000000000000800000000800000000
222222222222222222222aa000aa0000000000000000000000000000011080000000000000000000008100000000000000000000000000080000000800000000
2222222222222222222220aa00aa0000000000000000000000000011100008000000000000000000008100000000000000000000000000080000000800000000
2222222222222222222222aa000aa000000000000000000000001100000008000000000000000000008100000000000000000000000000080000000080000000
22222222222222222222222aa00a0a00000000000000000001110000000008000000000000000000008010000000000000770000000000008000000080000000
22222222222222222222222a0a00aa00000000000000000110000000000000800000000000000088000810000000000000770000000000008000000080000000
222222222222222222222222aa00a0a0000000000000011000000000000000800000000000000088000810000000000000000000000000008000000080000000
222222222222222222222222a0a00aa0000000000011100000000000000000800000000000000088000810000000000000000000000000008000000080000000
2222222222222222222222222aa00a0a000000001100000000000000000000080000000000000000000081000000000000000000000000008000000008000000
2222222222222222222222222a0a00aa000001110000000000000000000000080000000000000000000081000000000000000000000000000800000008000000
22222222222222222222222222aa00aa000110000000000000000000000000080000000000000000000081000000000000000000000000000800000008000000
22222222222222222222222222a0a00aa11000000000000000000000000000008000000000000000000081000000000000000000000000000800000008000000
000000002222222222222222222aa01aa00000000000000000000000000000008000000000000000000008100000000000000000000000000800000008000000
000000000022222222222222222aa100aa0000000000000000000000000000008000000000000000000008100000000000000000000000000800000000800000
0000000000022222222222222222aa00aa0000000000000000000000000000000800000000000000000008100000000000000000000000000080000000800000
0000000000002222222222222222aa00aa0000000000000000000000000000090800000000000000000008100000000000000000000000000080000000800000
00000000000002222222222222220aa00aa000000000000000000000000000999800000000000000000000810000000000000000000000000080000000800000
00000000000000222222222222222aa00aa000000000000000000000000000090800000000000000000000810000000000000000000000000080000000800000
00000000000000222222222222222aa00aa000000000000000000000000000000800000000000000000000810000000000000000000000000080000000800000
000000000000000222222222222222aa00aa00000000000000000000000000000080000000000000000000810000000000000000000000000080000000080000
000000000000000222222222222222aa00aa00000000000000000000000077777777700000000000000000810000000000000000000000000008000000080000
000000000000000222222222222222aa00aa00000000000000000000777710101010177770000000000000810000000000000000000000000008000000080000
000000000000000222222222222222aa000aa0000000000000000077010101010181010107700000000000081000000000000000000000000008000000080000
0000000000000002222222222222222aa00aa0000000000000007710101010101010101010177000000000081000000000000000000000000008000000080000
0000000000000002222222222222222aa00aa0000000000000770101010101010101010101010770000000081000000000000000000000000008000000080000
0000000000000002222222222222222aa00aa0000000000007001010101010101018101010101007000000081000000000000000000000000008000000080000
0000000000000002222222222222222aa000aa000000000071010101010101010101010101010101700000081000000000000000000000000008000000080000
00000000000000222222222222222222aa00aa000000000710101010101010101018101010101010170000081000000000000000000000000008000000080000
00000000000000222222222222222222aa00aa000000007101010101010101010101010101010101017000081000000000000000000000000008000000080000
00000000000002222222222222222222aa00aa000000071010101010101010101018101010101010101700081000000000000000000000000000800000008000
00000000000022222222222222222222aa00aa000000710101010101010101010101010101010101010170008100000000000000000000000000800000008000
00000000000222222222222222222222aa00aa000007001010101010101010101010101010101010101007008100000000000000000000000000800000008000
00000000002222222222222222222222aa00aa000007010101010101010101010101810101010101010107008100000000000000000000000000800000008000
000000002222222222222222222222222aa00aa00070101010101010101010101010101010101010101010708100000000000000000000000000800000008000
222222222222222222222222222222222aa00aa00071010101010101010101010101810101010101010101708100000000000000000000000000800000008000
222222222222222222222222222222222aa00aa00710101010101010101017101010101010101010101010178100000000000000000000000000800000008000
222222222222222222222222222222222aa00aa00701010101010101010101010101810101010101010101078100000000000000000000000000800000008000
222222222222222222222222222222222aa00aa07010101010101010101010101010181010101010101010107100000000000000000000000000800000008000
222222222222222222222222222222222aa00aa07101010101010101010101010101810101010101010101017100000000000000000000000000800000008000
222222222222222222222222222222222aa00aa07010101010101010101010101010101010101010101010107100000000000000000000000000800000008000
222222222222222222222222222222222aa00aa07101010101010101010101010101810101010101010101017100000000000000000000000000800000008000
222222222222222222222222222222222aa11aa71111111111111111111111118111111111111111111111111711111111111111111111111111811111118111
222222222222222222222222222222222aa00aa70101010101010101010101810101810101010101010101018700000000000000000000000000800000008000
022222222222222222222222222222222aa00aa71010101010101010101010101010101010101010101010101700000000000000000000000000800000008000
002222222222222222222222222222222aa00aa7010101010101010101010101d801810101010101010101018700000200000000000000000000800000008000
000222222222222222222222222222222aa00aa710101810101010101010a0101010101010101010101010101700000000000000000000000000800000008000
000022222222222222222222222222222aa00aa70101010101010101010aaa010101810101010101010101018700000000000000000000000000800000008000
000002222222222222222222222222222aa00aa710101010101080101010a0101010101010108010101010101700000000000000000000000000800000008000
0007777777777772227777777772227777700aa70101010101010801010101010101810101010101010101018700000000000000000000000000800000008000
0007000007700077777000707072227000700aa71010101010101010101018101010101010101010101010101700000000000000000000000000800000008000
000770007770707707777070707222707070aa007101010101010101010101010101810101010101010101017100000000000000000000000000800000008000
000077077270707777700070007222707070aa007010101080101010101018101010101010101010101010107100000000000000000000000000800000008000
000770007770707707707777707777707070aa007101010101010101010108010101010801010101010101017100000000000000000000000000800000008000
000700000770007777700072707707700070aa007010101010801010101010101018101010101010101010187000000000000000000888000000800000008000
000777777777777222777772777777777770aa000701010101010101010101010101010101010101010101071000000000000000000888000008000000080000
00000000022222222222222222222222aa00aa000710101010101010101010101018101010101010101010171000000000000000000000000008000000080000
0000000002222222222222222222222aa000aa000071010101010101010101010101010801010101010101781000000000000000000000000008000000080000

__sfx__
4b03000036650256501165026650166501d65011650106400b6400c6400b6400b64014640096300963008630086200e6200761006610056100561004610096100361002610006100161000600006000060000600
4b0300003b65030650156502765031650236501b650106400c6400a6400864009640206400e6300e6300c6300b620066200561003610046100461004610056100361002610006100161000600006000060000600
4b03000014650366501365031650256502a65011650106400f6400c640176400b64014640136301b6301963007620066200c6100f610056100561004610096100361002610006100161000600006000060000600
9a0404083552310523125230e5230b513205130a513165130c513115130d5130b5130751303513005030050300503005030050300503005030050300503005030050300503005030050300503005030050300503
4a0500003b66030660156602766031650236501b650156500c6500a6501b65026640206400e6400e6400c6400b6300e630166300d620096200462004620056200361002610006100161000600006000060000600
11040000050120601207022090320b0320d0421004211052130521505218052180521805218052180521805218052180521804218042180321803218022180220000200002000020000200002000020000200002
d703000005053070530b05310053130531405315053140431204311043100430d0430a04309033070330503303023020230201301013010130201302013010130201301013010130101300003000030000300003
001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
170e00001552215522155221552211522115221152211522155221552211522115221752217522175221752211522115221152211522175221752215522155221852218522185221852215522155221552215522
090e00000c7530070300703091350c753007030c7530c7530c753007030c753007030c753007030b1350b1350c753007030c7530c7530c7530b1350c7530b1350c753007030c1350c1350c7530c1350c7530c135
170e000015522155221552215522155221552215522155221152211522115221152215522185220000018525175221752217522175221752217522175221752213522135221352213522175221a522000001a522
090e00000c7530070300703091350c753007030c7530c7530c753007030c753007030c7530070309135091350c753007030c7530c7530c7530b1350c7530b1350c753007030b1350b1350c7530b1350c7530b135
170e0000185221852218522185221852218522185221852215522155221552215522185221c522000001c5221d5221c5221a522185221c5221a52218522175221a52218522175221552218522175221552213522
090e00000c75300703007030c1350c753007030c7530c7530c753007030c753007030c753007030c1350c1351a721185220c7530c75318722175220c7531352217722155220c7530c753157220c753117220c753
170e00001852218522185221852215522155221552215522185221852215522155221a5221a5221a5221a522155221552215522155221a5221a52218522185221c5221c5221c5221c52218522185221852218522
090e00000c75300703007030c1350c753007030c7530c7530c753007030c753007030c753007030e1350e1350c753007030c7530c7530c7530e1350c7530e1350c7530070310135101350c753101350c75310135
010e00002d5342d5302d5202d5122d512000052d522000052d5320000000000000003052430512000003051234534345303452034512345120000034522000003453200000000000000029524295120000028512
730e00000c75300000091350913500000091350c753000000c7530000009135091350c135000000c1350c1350c75300000101351013500000101350c753000000c75300000101351013511135000001113510135
010e00002653426530265202651226512000002652200000265320000000000000002852428522000002652224534245302452024512245120000024522000002453200000000000000026524265220000024522
730e00000c75300000021350213500000021350c753000000c753000000213502135041350000002135021350c75300000001350013500000001350c753000000c75300000001350013502135000000213500135
010e0000235342353023520235122351200000235220000023532000000000000000245242452200000235222153421530215202151221512000002152200000215320000000000000001853223532215321f532
730e00000c753000000b1350b135000000b1350c753000000c753000000b1350b1350c135000000c1350b1350c75300000091350913500000091350c753000000c7530000009135091350b135000000913509135
090e0000215242151021512215122472424710247122471228524285102851228512297242971029712297122352423510235122351226724267102671226712295242951029512295122b7242b7102b7122b712
730e00000c7530e1000e1240e1120c7530c75311124111120c7531510015124151120c75317100171240c7530c7530000010124101120c7530c75313124131120c7530000017124171120c75300000181240c753
__music__
01 0a0b4344
00 0a0b4344
00 10114344
00 10114344
00 0c0d4344
00 0e0f4344
00 12134344
00 14154344
02 16174344
00 41424344
00 41424344
00 41424344
00 41424344
00 41424344
00 41424344
00 41424344
00 41424344
00 41424344
00 41424344
00 41424344
00 41424344
00 41424344
00 41424344
00 41424344
00 41424344
00 41424344
00 41424344
00 41424344
00 41424344
00 41424344
00 41424344
01 18194344
00 0c0d4344
00 0e0f4344
00 0c0d4344
00 0e0f4344
00 0a0b4344
00 0a0b4344
00 10114344
00 10114344
00 12134344
00 14154344
00 16174344
00 12134344
00 14154344
02 16174344

