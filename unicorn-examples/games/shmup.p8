pico-8 cartridge // http://www.pico-8.com
version 36
__lua__
-- cherry bomb
-- by lazy devs

-- todo
-- ------------

blinkt=1
t=0
lockout=0
shake=0
flash=0
debug=""
peekerx=64
mode="start"
stars={} 
version="v1"
highscore=dget(0)

function _init()
 --this will clear the screen
 cls(0)
 
 --cartdata("lazydevs_shmup")
 highscore=dget(0)
 
 version="v1"
 startscreen()
 blinkt=1
 t=0
 lockout=0
 
 shake=0
 flash=0

 debug=""
 
 peekerx=64
end

function _update()
 t = t + 1
 
 blinkt = blinkt + 1

 if mode=="game" then
  update_game()
 elseif mode=="start" then
  update_start()
 elseif mode=="wavetext" then
  update_wavetext()
 elseif mode=="over" then
  update_over()
 elseif mode=="win" then
  update_win()
 end
 
end

function _draw()

 doshake()
 
 if mode=="game" then
  draw_game()
 elseif mode=="start" then
  draw_start()
 elseif mode=="wavetext" then
  draw_wavetext()
 elseif mode=="over" then
  draw_over()
 elseif mode=="win" then
  draw_win()
 end
 
 camera()
 print(debug,2,9,7)
end

function startscreen()
 makestars()
 mode="start"
 music(7)
end

function startgame()
 t=0
 wave=0
 lastwave=9
 nextwave()
 
 ship=makespr()
 ship.x=60
 ship.y=90
 ship.sx=0
 ship.sy=0
 ship.spr=2
   
 flamespr=5
 
 bultimer=0
 
 muzzle=0
 
 score=0
 cher=0
 
 lives=4
 invul=0
 
 attacfreq=60
 firefreq=20
 nextfire=0
 
 makestars()
  
 buls={}
 ebuls={}
 
 enemies={}
 
 parts={}
 
 shwaves={}
 
 pickups={}
 
 floats={}
end

-->8
-- tools

function makestars()
 for i=1,100 do
  local newstar={}
  newstar.x=flr(rnd(128))
  newstar.y=flr(rnd(128))
  newstar.spd=rnd(1.5)+0.5
  add(stars,newstar)
 end 
end

function starfield()
 
 for i=1,#stars do
  local mystar=stars[i]
  local scol=6
  
  if mystar.spd<1 then
   scol=1
  elseif mystar.spd<1.5 then
   scol=13
  end   
  
  pset(mystar.x,mystar.y,scol)
 end
end

function animatestars(spd)
 if spd==nil then
  spd=1
 end
 
 for i=1,#stars do
  local mystar=stars[i]
  mystar.y=mystar.y+mystar.spd*spd
  if mystar.y>128 then
   mystar.y=mystar.y-128
  end
 end

end

function blink()
 local banim={5,5,5,5,5,5,5,5,5,5,5,6,6,7,7,6,6,5}
 
 if blinkt>#banim then
  blinkt=1
 end

 return banim[blinkt]
end

function drwoutline(myspr)
 spr(myspr.spr,myspr.x+1,myspr.y,myspr.sprw,myspr.sprh)
 spr(myspr.spr,myspr.x-1,myspr.y,myspr.sprw,myspr.sprh)
 spr(myspr.spr,myspr.x,myspr.y+1,myspr.sprw,myspr.sprh)
 spr(myspr.spr,myspr.x,myspr.y-1,myspr.sprw,myspr.sprh)
end

function drwmyspr(myspr)
 local sprx=myspr.x
 local spry=myspr.y
 
 if myspr.shake>0 then
  myspr.shake = myspr.shake - 1
  if t%4<2 then
   sprx = sprx + 1
  end
 end
 if myspr.bulmode then
  sprx = sprx - 2
  spry = spry - 2
 end
 
 spr(myspr.spr,sprx,spry,myspr.sprw,myspr.sprh)
end

function col(a,b)
 if a.ghost or b.ghost then 
  return false
 end

 local a_left=a.x
 local a_top=a.y
 local a_right=a.x+a.colw-1
 local a_bottom=a.y+a.colh-1
 
 local b_left=b.x
 local b_top=b.y
 local b_right=b.x+b.colw-1
 local b_bottom=b.y+b.colh-1

 if a_top>b_bottom then return false end
 if b_top>a_bottom then return false end
 if a_left>b_right then return false end
 if b_left>a_right then return false end
 
 return true
end

function explode(expx,expy,isblue)
 
 local myp={}
 myp.x=expx
 myp.y=expy
 
 myp.sx=0
 myp.sy=0
 
 myp.age=0
 myp.size=10
 myp.maxage=0
 myp.blue=isblue
 
 add(parts,myp)
	  
 for i=1,30 do
	 local myp={}
	 myp.x=expx
	 myp.y=expy
	 
	 myp.sx=rnd()*6-3
	 myp.sy=rnd()*6-3
	 
	 myp.age=rnd(2)
	 myp.size=1+rnd(4)
	 myp.maxage=10+rnd(10)
	 myp.blue=isblue
	 
	 add(parts,myp)
 end
 
 for i=1,20 do
	 local myp={}
	 myp.x=expx
	 myp.y=expy
	 
	 myp.sx=(rnd()-0.5)*10
	 myp.sy=(rnd()-0.5)*10
	 
	 myp.age=rnd(2)
	 myp.size=1+rnd(4)
	 myp.maxage=10+rnd(10)
	 myp.blue=isblue
	 myp.spark=true
	 
	 add(parts,myp)
 end
 
 big_shwave(expx,expy)
 
end

function bigexplode(expx,expy)
 
 local myp={}
 myp.x=expx
 myp.y=expy
 
 myp.sx=0
 myp.sy=0
 
 myp.age=0
 myp.size=25
 myp.maxage=0
 
 add(parts,myp)
	  
 for i=1,60 do
	 local myp={}
	 myp.x=expx
	 myp.y=expy
	 
	 myp.sx=rnd()*12-6
	 myp.sy=rnd()*12-6
	 
	 myp.age=rnd(2)
	 myp.size=1+rnd(6)
	 myp.maxage=20+rnd(20)
	 
	 add(parts,myp)
 end
 
 for i=1,100 do
	 local myp={}
	 myp.x=expx
	 myp.y=expy
	 
	 myp.sx=(rnd()-0.5)*30
	 myp.sy=(rnd()-0.5)*30
	 
	 myp.age=rnd(2)
	 myp.size=1+rnd(4)
	 myp.maxage=20+rnd(20)
	 myp.spark=true
	 
	 add(parts,myp)
 end
 
 big_shwave(expx,expy)
 
end

function page_red(page)
 local col=7
 
 if page>5 then
  col=10
 end
 if page>7 then
  col=9
 end
 if page>10 then
  col=8
 end
 if page>12 then
  col=2
 end
 if page>15 then
  col=5
 end
 
 return col
end

function page_blue(page)
 local col=7
 
 if page>5 then
  col=6
 end
 if page>7 then
  col=12
 end
 if page>10 then
  col=13
 end
 if page>12 then
  col=1
 end
 if page>15 then
  col=1
 end
 
 return col
end

function smol_shwave(shx,shy,shcol)
 if shcol==nil then
  shcol=9
 end 
 local mysw={}
 mysw.x=shx
 mysw.y=shy
 mysw.r=3
 mysw.tr=6
 mysw.col=shcol
 mysw.speed=1
 add(shwaves,mysw)
end

function big_shwave(shx,shy)
 local mysw={}
 mysw.x=shx
 mysw.y=shy
 mysw.r=3
 mysw.tr=25
 mysw.col=7
 mysw.speed=3.5
 add(shwaves,mysw)
end

function smol_spark(sx,sy)
 --for i=1,2 do
 local myp={}
 myp.x=sx
 myp.y=sy
 
 myp.sx=(rnd()-0.5)*8
 myp.sy=(rnd()-1)*3
 
 myp.age=rnd(2)
 myp.size=1+rnd(4)
 myp.maxage=10+rnd(10)
 myp.blue=isblue
 myp.spark=true
 
 add(parts,myp)
 --end
end

function makespr()
 local myspr={}
 myspr.x=0
 myspr.y=0
 myspr.sx=0
 myspr.sy=0
 
 myspr.flash=0
 myspr.shake=0
 
 myspr.aniframe=1
 myspr.spr=0
 myspr.sprw=1
 myspr.sprh=1
 myspr.colw=8
 myspr.colh=8
 
 return myspr
end

function doshake()

 local shakex=rnd(shake)-(shake/2)
 local shakey=rnd(shake)-(shake/2)
 
 camera(shakex,shakey)
 
 if shake>10 then
  shake = shake * 0.9
 else
  shake = shake - 1
  if shake<1 then
   shake=0
  end
 end
end

function popfloat(fltxt,flx,fly)
 local fl={}
 fl.x=flx
 fl.y=fly
 fl.txt=fltxt
 fl.age=0
 add(floats,fl)
end

function cprint(txt,x,y,c)
 print(txt,x-#txt*2,y,c)
end
-->8
--update

function update_game()
 --controls
 ship.sx=0
 ship.sy=0
 ship.spr=2
 
 if btn(0) then
  ship.sx=-2
  ship.spr=1
 end
 if btn(1) then
  ship.sx=2
  ship.spr=3
 end
 if btn(2) then
  ship.sy=-2
 end
 if btn(3) then
  ship.sy=2
 end
  
 if btnp(4) then
  if cher>0 then
   cherbomb()
   cher=0
  else
   sfx(32)
  end
 end
 
 if btn(5) then
  if bultimer<=0 then
	  local newbul=makespr()
	  newbul.x=ship.x+1
	  newbul.y=ship.y-3
	  newbul.spr=16
	  newbul.colw=6
	  newbul.sy=-4
	  newbul.dmg=1
	  add(buls,newbul)
	  
	  sfx(0)
	  muzzle=5
	  bultimer=4
  end
 end
 bultimer = bultimer - 1
 
 --moving the ship
 ship.x = ship.x + ship.sx
 ship.y = ship.y + ship.sy
 
 --checking if we hit the edge
 if ship.x>120 then
  ship.x=120
 end
 if ship.x<0 then
  ship.x=0
 end
 if ship.y<0 then
  ship.y=0
 end
 if ship.y>120 then
  ship.y=120
 end
 
 --move the bullets
 for mybul in all(buls) do
  move(mybul)
  if mybul.y<-8 then
   del(buls,mybul)
  end
 end
 
 --move the ebuls
 for myebul in all(ebuls) do
  move(myebul)
  animate(myebul)
  if myebul.y>128 or myebul.x<-8 or myebul.x>128 or myebul.y<-8 then
   del(ebuls,myebul)
  end
 end 
 
 --move the pickups
 for mypick in all(pickups) do
  move(mypick)
  if mypick.y>128 then
   del(pickups,mypick)
  end
 end 
 
 --moving enemies 
 for myen in all(enemies) do
  --enemy mission
  doenemy(myen)
  
  --enemy animation
  animate(myen)
    
  --enemy leaving screen
  if myen.mission~="flyin" then 
   if myen.y>128 or myen.x<-8 or myen.x>128 then
    del(enemies,myen)
   end
  end
 end
 
 --collision enemy x bullets
 for myen in all(enemies) do
  for mybul in all(buls) do
   if col(myen,mybul) then
    del(buls,mybul)
    smol_shwave(mybul.x+4,mybul.y+4)
    smol_spark(myen.x+4,myen.y+4)
    if myen.mission~="flyin" then
     myen.hp = myen.hp - mybul.dmg
    end
    sfx(3)
    if myen.boss then
     myen.flash=5
    else
     myen.flash=2
    end
    if myen.hp<=0 then
     killen(myen)
    end
   end
  end
 end
 
 --collision ebuls x bullets
 for mybul in all(buls) do
  if mybul.spr==17 then
	  for myebul in all(ebuls) do
	   if col(myebul,mybul) then
	    del(ebuls,myebul)
	    score = score + 5
	    smol_shwave(ebuls.x,ebuls.y,8)
	   end
	  end
  end
 end
 
 --collision ship x enemies
 if invul<=0 then
	 for myen in all(enemies) do
	  if col(myen,ship) then
    explode(ship.x+4,ship.y+4,true)
	   lives = lives - 1
	   sfx(1)
	   shake=12
	   invul=60
    ship.x=60
    ship.y=100
    flash=3
	  end
	 end
 else
  invul = invul - 1
 end
 
 --collision ship x ebuls
 if invul<=0 then
	 for myebul in all(ebuls) do
	  if col(myebul,ship) then
    explode(ship.x+4,ship.y+4,true)
	   lives = lives - 1
	   shake=12
	   sfx(1)
	   invul=60
    ship.x=60
    ship.y=100
    flash=3
	  end
	 end
 end
 
 --collision pickup x ship
 for mypick in all(pickups) do
  if col(mypick,ship) then
   del(pickups,mypick)
   plogic(mypick)
  end
 end
 
 
 if lives<=0 then
  mode="over"
  lockout=t+30
  music(6)
  return
 end
 
 --picking
 picktimer()
 
 --animate flame
 flamespr=flamespr+1
 if flamespr>9 then
  flamespr=5
 end
 
 --animate mullze flash
 if muzzle>0 then
  muzzle=muzzle-1
 end
  
 if mode=="wavetext" then
  animatestars(2)
 else
  animatestars()
 end
 
 --check if wave over
 if mode=="game" and #enemies==0 then
  ebuls={}
  nextwave()
 end
 
end

function update_start()
 animatestars(0.4)
 
 if btn(4)==false and btn(5)==false then
  btnreleased=true
 end

 if btnreleased then
  if btnp(4) or btnp(5) then
   startgame()
   btnreleased=false
  end
 end
end

function update_over()
 if t<lockout then
  return
 end
 
 if btn(4)==false and btn(5)==false then
  btnreleased=true
 end

 if btnreleased then
  if btnp(4) or btnp(5) then
   if score>highscore then
    highscore=score
    dset(0,score)
   end
   startscreen()
   btnreleased=false
  end
 end
end

function update_win()
 if t<lockout then
  return
 end
 
 if btn(4)==false and btn(5)==false then
  btnreleased=true
 end

 if btnreleased then
  if btnp(4) or btnp(5) then
   if score>highscore then
    highscore=score
    dset(0,score)
   end
   startscreen()
   btnreleased=false
  end
 end
end

function update_wavetext()
 update_game()
 wavetime = wavetime - 1
 if wavetime<=0 then
  mode="game"
  spawnwave()
 end
end
-->8
-- draw

function draw_game()
 if flash>0 then
  flash = flash - 1
  cls(2)
 else
  cls(0)
 end
 
 starfield()

 if lives>0 then
	 if invul<=0 then
	  drwmyspr(ship)
	  spr(flamespr,ship.x,ship.y+8)
	 else
	  --invul state
	  if sin(t/5)<0.1 then
	   drwmyspr(ship)
	   spr(flamespr,ship.x,ship.y+8)
	  end
	 end
 end
 
 --drawing pickups
 for mypick in all(pickups) do
  local mycol=7
  if t%4<2 then
   mycol=14
  end
  for i=1,15 do
   pal(i,mycol)
  end
  drwoutline(mypick)
  pal()
  drwmyspr(mypick)
 end
 
 --drawing enemies
 for myen in all(enemies) do
  if myen.flash>0 then
   if t%4<2 then
    pal(3,8)
    pal(11,14)
   end
   myen.flash = myen.flash - 1
   if myen.boss then
    myen.spr=64
   else
    for i=1,15 do
     pal(i,7)
    end
   end
  end
  drwmyspr(myen)
  pal()
 end
  
 --drawing bullets
 for mybul in all(buls) do
  drwmyspr(mybul)
 end
 
 if muzzle>0 then
  circfill(ship.x+3,ship.y-2,muzzle,7)
  circfill(ship.x+4,ship.y-2,muzzle,7)
 end
 
 --drawing shwaves
 for mysw in all(shwaves) do
  circ(mysw.x, mysw.y, mysw.r, mysw.col)
  mysw.r = mysw.r + mysw.speed
  if mysw.r>mysw.tr then
   del(shwaves,mysw)
  end
 end
 
 --drawing particles
 for myp in all(parts) do
  local pc=7

  if myp.blue then
   pc=page_blue(myp.age)
  else
   pc=page_red(myp.age)
  end
  
  if myp.spark then
   pset(myp.x,myp.y,7)
  else
   circfill(myp.x,myp.y,myp.size,pc)
  end
  
  myp.x = myp.x + myp.sx
  myp.y = myp.y + myp.sy
  
  myp.sx=myp.sx*0.85
  myp.sy=myp.sy*0.85
  
  myp.age = myp.age + 1
  
  if myp.age>myp.maxage then
   myp.size = myp.size - 0.5
   if myp.size<0 then
    del(parts,myp)
   end
  end
 end
 
 --drawing ebuls
 for myebul in all(ebuls) do
  drwmyspr(myebul)
 end
 
 --floats
 for myfl in all(floats) do
  local mycol=7
  if t%4<2 then
   mycol=8
  end
  cprint(myfl.txt,myfl.x,myfl.y,mycol)
  myfl.y = myfl.y - 0.5
  myfl.age = myfl.age + 1
  if myfl.age>60 then
   del(floats,myfl)
  end
 end
 
 print("score:"..makescore(score),40,2,12)
 
 for i=1,4 do
  if lives>=i then
   spr(13,i*9-8,1)
  else
   spr(14,i*9-8,1)
  end 
 end

 spr(48,108,1)
 print(cher,118,2,14)
 
 --print(#buls,5,5,7)
end

function makescore(val)
 if val==0 then
  return "0"
 end
 return val.."00"
end

function draw_start()
 cls(0)
 starfield()
 print(version, 1, 1, 1)

 spr(21,peekerx,28+sin(time()/3.5)*4 )
 if sin(time()/3.5)>0.5 then
  peekerx=30+rnd(60)
 end
   
 spr(212,17,30,12,2)
 cprint("short shwave shmup",64,45,6)
 
 if highscore>0 then
  cprint("highscore:",64,63,12)
  cprint(makescore(highscore),64,69,12)
 end

 cprint("press any key to start",64,90,blink())
end

function draw_over()
 draw_game()
 cprint("game over",64,40,8) 
 
 cprint("score:"..makescore(score),64,60,12)
 if score>highscore then
  local c=7
  if t%4<2 then
   c=10
  end
  cprint("new highscore!",64,66,c) 
 end
 
 cprint("press any key to continue",64,90,blink())
end

function draw_win()
 draw_game()
 cprint("congratulations",64,40,12)
 cprint("score:"..makescore(score),64,60,12)

 if score>highscore then
  local c=7
  if t%4<2 then
   c=10
  end
  cprint("new highscore!",64,66,c) 
 end

 cprint("press any key to continue",64,90,blink())
end

function draw_wavetext()
 draw_game()
 if wave==lastwave then
  cprint("final wave!",64,40,blink())
 else
  cprint("wave "..wave.." of "..lastwave,64,40,blink())
 end
end
-->8
-- waves and enemies

function spawnwave()
 if wave<lastwave then
  sfx(28)
 else
  music(10)
 end
 
 if wave==1 then
  --space invaders
  attacfreq=60
  firefreq=20
  placens({
   {0,1,1,1,1,1,1,1,1,0},
   {0,1,1,1,1,1,1,1,1,0},
   {0,1,1,1,1,1,1,1,1,0},
   {0,1,1,1,1,1,1,1,1,0}
  })
 elseif wave==2 then
  --red tutorial
  attacfreq=60
  firefreq=20
  placens({
   {1,1,2,2,1,1,2,2,1,1},
   {1,1,2,2,1,1,2,2,1,1},
   {1,1,2,2,2,2,2,2,1,1},
   {1,1,2,2,2,2,2,2,1,1}
  })
 elseif wave==3 then
  --wall of red
  attacfreq=50
  firefreq=20
  placens({
   {1,1,2,2,1,1,2,2,1,1},
   {1,1,2,2,2,2,2,2,1,1},
   {2,2,2,2,2,2,2,2,2,2},
   {2,2,2,2,2,2,2,2,2,2}
  })
 elseif wave==4 then
  --spin tutorial
  attacfreq=50
  firefreq=15
  placens({
   {3,3,0,1,1,1,1,0,3,3},
   {3,3,0,1,1,1,1,0,3,3},
   {3,3,0,1,1,1,1,0,3,3},
   {3,3,0,1,1,1,1,0,3,3}
  })
 elseif wave==5 then
  --chess
  attacfreq=50
  firefreq=15
  placens({
   {3,1,3,1,2,2,1,3,1,3},
   {1,3,1,2,1,1,2,1,3,1},
   {3,1,3,1,2,2,1,3,1,3},
   {1,3,1,2,1,1,2,1,3,1}
  })
 elseif wave==6 then
  --yellow tutorial
  attacfreq=40
  firefreq=10
  placens({
   {2,2,2,0,4,0,0,2,2,2},
   {2,2,0,0,0,0,0,0,2,2},
   {1,1,0,1,1,1,1,0,1,1},
   {1,1,0,1,1,1,1,0,1,1}
  })
  
 elseif wave==7 then
  --double yellow
  attacfreq=40
  firefreq=10
  placens({
   {3,3,0,1,1,1,1,0,3,3},
   {4,0,0,2,2,2,2,0,4,0},
   {0,0,0,2,1,1,2,0,0,0},
   {1,1,0,1,1,1,1,0,1,1}
  })
 elseif wave==8 then
  --hell
  attacfreq=30
  firefreq=10
  placens({
   {0,0,1,1,1,1,1,1,0,0},
   {3,3,1,1,1,1,1,1,3,3},
   {3,3,2,2,2,2,2,2,3,3},
   {3,3,2,2,2,2,2,2,3,3}
  })
 elseif wave==9 then
  --boss
  attacfreq=60
  firefreq=20
  placens({
   {0,0,0,0,5,0,0,0,0,0},
   {0,0,0,0,0,0,0,0,0,0},
   {0,0,0,0,0,0,0,0,0,0},
   {0,0,0,0,0,0,0,0,0,0}
  })
 end  
end

function placens(lvl)

 for y=1,4 do
  local myline=lvl[y]
  for x=1,10 do
   if myline[x]~=0 then
    spawnen(myline[x],x*12-6,4+y*12,x*3)
   end
  end
 end
 
end

function nextwave()
 wave = wave + 1
 
 if wave>lastwave then
  mode="win"
  lockout=t+30
  music(4)
 else
  if wave==1 then
   music(0)
  else
   music(3)  
  end
  
  mode="wavetext"
  wavetime=80
 end

end

function spawnen(entype,enx,eny,enwait)
 local myen=makespr()
 myen.x=enx*1.25-16
 myen.y=eny-66
 
 myen.posx=enx
 myen.posy=eny
 
 myen.type=entype
 
 myen.wait=enwait

 myen.anispd=0.4
 
 myen.mission="flyin"
 
 if entype==nil or entype==1 then
  -- green alien
  myen.spr=21
  myen.hp=3
  myen.ani={21,22,23,24}
  myen.score=1
 elseif entype==2 then
  -- red flame guy
  myen.spr=148
  myen.hp=2
  myen.ani={148,149}
  myen.score=2
 elseif entype==3 then
  -- spinning ship
  myen.spr=184
  myen.hp=4
  myen.ani={184,185,186,187}
  myen.score=3
 elseif entype==4 then
  -- yellow guy
  myen.spr=208
  myen.hp=20
  myen.ani={208,210}
  myen.sprw=2
  myen.sprh=2
  myen.colw=16
  myen.colh=16
  myen.score=5
 elseif entype==5 then
  myen.hp=130
  myen.spr=68
  myen.ani={68,72,76,72}
  myen.sprw=4
  myen.sprh=3
  myen.colw=32
  myen.colh=24
  
  myen.x=48
  myen.y=-24
  myen.posx=48
  myen.posy=25
  
  myen.boss=true
 end
  
 add(enemies,myen)
end
-->8
--behavior

function doenemy(myen)
 if myen.wait>0 then
  myen.wait = myen.wait - 1
  return
 end
 
 --debug=myen.hp
 
 if myen.mission=="flyin" then
  --flying in
  --basic easing function
  --x+=(targetx-x)/n
  
  local dx=(myen.posx-myen.x)/7
  local dy=(myen.posy-myen.y)/7
  
  if myen.boss then
   dy=min(dy,1)
  end
  myen.x = myen.x + dx
  myen.y = myen.y + dy
  
  if abs(myen.y-myen.posy)<0.7 then
   myen.y=myen.posy
   myen.x=myen.posx
   if myen.boss then
    sfx(50)
    myen.shake=20
    myen.wait=28
    myen.mission="boss1"
    myen.phbegin=t
   else
    myen.mission="protec"
   end
  end
  
 elseif myen.mission=="protec" then
  -- staying put
 elseif myen.mission=="boss1" then
  boss1(myen)
 elseif myen.mission=="boss2" then
  boss2(myen)
 elseif myen.mission=="boss3" then
  boss3(myen)
 elseif myen.mission=="boss4" then
  boss4(myen)
 elseif myen.mission=="boss5" then
  boss5(myen)
 elseif myen.mission=="attac" then  
  -- attac
  if myen.type==1 then
   --green guy
   myen.sy=1.7
   myen.sx=sin(t/45)
   
   -- just tweaks
   if myen.x<32 then
    myen.sx = myen.sx + (1-(myen.x/32))
   end
   if myen.x>88 then
    myen.sx = myen.sx - (myen.x-88)/32
   end
  elseif myen.type==2 then
   --red guy
   myen.sy=2.5
   myen.sx=sin(t/20)
   
   -- just tweaks
   if myen.x<32 then
    myen.sx = myen.sx + (1-(myen.x/32))
   end
   if myen.x>88 then
    myen.sx = myen.sx - (myen.x-88)/32
   end   
   
  elseif myen.type==3 then
   --spinny ship
   if myen.sx==0 then
    --flying down
    myen.sy=2
    if ship.y<=myen.y then
     myen.sy=0
     if ship.x<myen.x then
      myen.sx=-2
     else
      myen.sx=2
     end
    end
   end
   
  elseif myen.type==4 then
   --yellow ship
   myen.sy=0.35
   if myen.y>110 then
    myen.sy=1
   else
    
    if t%25==0 then
     firespread(myen,8,1.3,rnd())
    end
   end   
  end
  
  move(myen)
 end
  
end

function picktimer()
 if mode~="game" then
  return
 end

 if t>nextfire then
  pickfire()
  nextfire=t+firefreq+rnd(firefreq)
 end
 
 if t%attacfreq==0 then
  pickattac()
 end
end

function pickattac()
 local maxnum=min(10,#enemies)
 local myindex=flr(rnd(maxnum))
 
 myindex=#enemies-myindex
 local myen=enemies[myindex]
 if myen==nil then return end
 
 if myen.mission=="protec" then
  myen.mission="attac"
  myen.anispd = myen.anispd * 3
  myen.wait=60
  myen.shake=60
 end
end

function pickfire()
 local maxnum=min(10,#enemies)
 local myindex=flr(rnd(maxnum))
 
 for myen in all(enemies) do
  if myen.type==4 and myen.mission=="protec" then
   if rnd()<0.5 then
    firespread(myen,12,1.3,rnd())
    return
   end
  end
 end
 
 myindex=#enemies-myindex
 local myen=enemies[myindex]
 if myen==nil then return end
 
 if myen.mission=="protec" then
  if myen.type==4 then
   --yellow guy
   firespread(myen,12,1.3,rnd())
  elseif myen.type==2 then
   --red guy
   aimedfire(myen,2)
  else
   fire(myen,0,2)
  end
 end
end


function move(obj)
 obj.x = obj.x + obj.sx
 obj.y = obj.y + obj.sy
end

function killen(myen)
 if myen.boss then
  myen.mission="boss5"
  myen.phbegin=t
  myen.ghost=true
  ebuls={}
  music(-1)
  sfx(51)
  return
 end

 del(enemies,myen)   
 sfx(2)
 

 explode(myen.x+4,myen.y+4)
 local cherchance=0.1
 local scoremult=1
 
 if myen.mission=="attac" then
  scoremult=2
  if rnd()<0.5 then
   pickattac()
  end
  cherchance=0.2
 end
 
 score = score + myen.score*scoremult
 if scoremult~=1 then
  popfloat(makescore(myen.score*scoremult),myen.x+4,myen.y+4)
 end
 
 if rnd()<cherchance then
  dropickup(myen.x,myen.y)
 end
end

function dropickup(pix,piy)
 local mypick=makespr()
 mypick.x=pix
 mypick.y=piy
 mypick.sy=0.75
 mypick.spr=48
 add(pickups,mypick)
end

function plogic(mypick)
 cher = cher + 1
 smol_shwave(mypick.x+4,mypick.y+4,14)
 if cher>=10 then
  --get a life
  if lives<4 then
   lives = lives + 1
   sfx(31)
   cher=0
   popfloat("1up!",mypick.x+4,mypick.y+4)
  else
   --points
   score = score + 50
   popfloat(makescore(50),mypick.x+4,mypick.y+4)
   sfx(30)
   cher=0
  end
 else
  sfx(30)
 end
end

function animate(myen)
 myen.aniframe =  myen.aniframe + myen.anispd
 if flr(myen.aniframe) > #myen.ani then
  myen.aniframe=1
 end
 myen.spr=myen.ani[flr(myen.aniframe)]
end
-->8
--bullets

function fire(myen,ang,spd)
 local myebul=makespr()
 myebul.x=myen.x+3
 myebul.y=myen.y+6
 
 if myen.type==4 then
  myebul.x=myen.x+7
  myebul.y=myen.y+13
 elseif myen.boss then
  myebul.x=myen.x+15
  myebul.y=myen.y+23 
 end
 
 myebul.spr=32
 myebul.ani={32,33,34,33}
 myebul.anispd=0.5
 
 myebul.sx=sin(ang)*spd
 myebul.sy=cos(ang)*spd
 
 myebul.colw=2
 myebul.colh=2
 myebul.bulmode=true
 
 if myen.boss~=true then
  myen.flash=4
  sfx(29)
 else
  sfx(34)
 end
 
 add(ebuls,myebul)
 
 return myebul
end

function firespread(myen,num,spd,base)
 if base==nil then
  base=0
 end
 for i=1,num do
  fire(myen,1/num*i+base,spd)
 end
end

function aimedfire(myen,spd)
 local myebul=fire(myen,0,spd)
 
 local ang = atan2((ship.y+4)-myebul.y, (ship.x+4)-myebul.x)

 myebul.sx=sin(ang)*spd
 myebul.sy=cos(ang)*spd 
end

function cherbomb()
 local spc=0.25/(cher*2)
 
 for i=0,cher*2 do
  local ang=0.375+spc*i
  
  local newbul=makespr()
  newbul.x=ship.x
  newbul.y=ship.y-3
  newbul.spr=17
  newbul.dmg=3
  
  newbul.sx=sin(ang)*4
  newbul.sy=cos(ang)*4
 
  add(buls,newbul)
 end
 
 big_shwave(ship.x+3,ship.y+3)
 shake=5
 muzzle=5
 invul=30
 flash=3
 sfx(33)
 
end
-->8
--boss

function boss1(myen)
 -- movement
 local spd=2
 
 if myen.sx==0 or myen.x>=93 then
  myen.sx=-spd
 end
 if myen.x<=3 then
  myen.sx=spd
 end
 -- shooting
 if t%30>3 then
  if t%3==0 then
   fire(myen,0,2)
  end
 end
 
 -- transition
 if myen.phbegin+8*30<t then
  myen.mission="boss2"
  myen.phbegin=t
  myen.subphase=1
 end
 move(myen)
end

function boss2(myen)
 local spd=1.5
 
 -- movement
 if myen.subphase==1 then
  myen.sx=-spd
  if myen.x<=4 then
   myen.subphase=2
  end
 elseif myen.subphase==2 then
  myen.sx=0
  myen.sy=spd
  if myen.y>=100 then
   myen.subphase=3
  end 
 elseif myen.subphase==3 then
  myen.sx=spd
  myen.sy=0
  if myen.x>=91 then
   myen.subphase=4
  end  
 elseif myen.subphase==4 then
  myen.sx=0
  myen.sy=-spd
  if myen.y<=25 then
   -- transition
   myen.mission="boss3"
   myen.phbegin=t
   myen.sy=0
  end  
 end 
 -- shooting
 if t%15==0 then
  aimedfire(myen,spd)
 end

 move(myen)
end

function boss3(myen)
 -- movement
 local spd=0.5
 
 if myen.sx==0 or myen.x>=93 then
  myen.sx=-spd
 end
 if myen.x<=3 then
  myen.sx=spd
 end

 -- shooting
 if t%10==0 then
  firespread(myen,8,2,time()/2)
 end 
 
 -- transition
 if myen.phbegin+8*30<t then
  myen.mission="boss4"
  myen.subphase=1
  myen.phbegin=t
 end
 move(myen)
end

function boss4(myen)
 local spd=1.5
 
 -- movement
 if myen.subphase==1 then
  myen.sx=spd
  if myen.x>=91 then
   myen.subphase=2
  end
 elseif myen.subphase==2 then
  myen.sx=0
  myen.sy=spd
  if myen.y>=100 then
   myen.subphase=3
  end 
 elseif myen.subphase==3 then
  myen.sx=-spd
  myen.sy=0
  if myen.x<=4 then
   myen.subphase=4
  end  
 elseif myen.subphase==4 then
  myen.sx=0
  myen.sy=-spd
  if myen.y<=25 then
   -- transition
   myen.mission="boss1"
   myen.phbegin=t
   myen.sy=0
  end  
 end 

 -- shooting
 if t%12==0 then
  if myen.subphase==1 then
   fire(myen,0,2)
  elseif myen.subphase==2 then
   fire(myen,0.25,2)
  elseif myen.subphase==3 then
   fire(myen,0.5,2)
  elseif myen.subphase==4 then
   fire(myen,0.75,2)
  end
 end
 -- transition
 move(myen)
end

function boss5(myen)
 myen.shake=10
 myen.flash=10 
 
 if t%8==0 then
  explode(myen.x+rnd(32),myen.y+rnd(24))
  sfx(2)
  shake=2
 end

 if myen.phbegin+3*30<t then
	 if t%4==2 then
	  explode(myen.x+rnd(32),myen.y+rnd(24))
	  sfx(2)
   shake=2
	 end
 end

 if myen.phbegin+6*30<t then
  flash=3
  score = score + 100
  popfloat(makescore(100),myen.x+16,myen.y+6)
  bigexplode(myen.x+16,myen.y+12)
  shake=15
  enemies={}
  sfx(35)
 end
end
__gfx__
00000000000220000002200000022000000000000000000000000000000000000000000000000000000000000000000000000000088008800880088000000000
000000000028820000288200002882000000000000077000000770000007700000c77c0000077000000000000000000000000000888888888008800800000000
007007000028820000288200002882000000000000c77c000007700000c77c000cccccc000c77c00000000000000000000000000888888888000000800000000
0007700000288e2002e88e2002e882000000000000cccc00000cc00000cccc0000cccc0000cccc00000000000000000000000000888888888000000800000000
00077000027c88202e87c8e202887c2000000000000cc000000cc000000cc00000000000000cc000000000000000000000000000088888800800008000000000
007007000211882028811882028811200000000000000000000cc000000000000000000000000000000000000000000000000000008888000080080000000000
00000000025582200285582002285520000000000000000000000000000000000000000000000000000000000000000000000000000880000008800000000000
00000000002992000029920000299200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
09999000009999000000000000000000000000000330033003300330033003300330033000000000000000000000000000000000000000000000000000000000
9977990009aaaa9000000000000000000000000033b33b3333b33b3333b33b3333b33b3300000000000000000000000000000000000000000000000000000000
9a77a9009aa77aa90000000000000000000000003bbbbbb33bbbbbb33bbbbbb33bbbbbb300000000000000000000000000000000000000000000000000000000
9a77a9009a7777a90000000000000000000000003b7717b33b7717b33b7717b33b7717b300000000000000000000000000000000000000000000000000000000
9a77a9009a7777a90000000000000000000000000b7117b00b7117b00b7117b00b7117b000000000000000000000000000000000000000000000000000000000
99aa99009aa77aa90000000000000000000000000037730000377300003773000037730000000000000000000000000000000000000000000000000000000000
09aa900009aaaa900000000000000000000000000303303003033030030330300303303000000000000000000000000000000000000000000000000000000000
00990000009999000000000000000000000000000300003030000003030000300330033000000000000000000000000000000000000000000000000000000000
00ee000000ee00000077000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0e22e0000e88e00007cc700000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
e2e82e00e87e8e007c77c70000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
e2882e00e8ee8e007c77c70000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0e22e0000e88e00007cc700000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00ee000000ee00000077000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000bbbb0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000b0bb0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00b00b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00b00880000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
08808788000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
87880888000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
88880880000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
08800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000ee00000bbbbbbbb00000ee0000000000ee00000bbbbbbbb00000ee0000000000ee00000bbbbbbbb00000ee0000000000ee00000bbbbbbbb00000ee00000
ee0008e7e1bbbbbaabbbbb1e7e8000eeee0008e7e1bbbbbaabbbbb1e7e8000eeee0008e7e1bbbbbaabbbbb1e7e8000eeee0008e7e1bbbbbaabbbbb1e7e8000ee
e7e0138873bbbaa77aabbb3788310e7ee7e0138873bbbaa77aabbb3788310e7ee7e0138873bbbaa77aabbb3788310e7ee7e0138873bbbaa77aabbb3788310e7e
8e783b333bbabaa77aababb333b387e88e783b333bbabaa77aababb333b387e88e783b333bbabaa77aababb333b387e88e783b333bbabaa77aababb333b387e8
08e813bbbbbbbba77abbbbbbbb318e8008e813bbbbbbbbbaabbbbbbbbb318e8008e813bbbbbbbbbaabbbbbbbbb318e8008e813bbbbbbbbbaabbbbbbbbb318e80
088811bbbbbbbbbaabbbbbbbbb11888008881133b33bbbbbbbbbb33b3311888008881133b33bbbbbbbbbb33b3311888008881133b33bbbbbbbbbb33b33118880
0011133bbbbb33bbbb33bbbbb331110000113b11bbb3333333333bbb11b3110000113b11bbb3333333333bbb11b3110000113b11bbb3333333333bbb11b31100
00bb113bbabbb33bb33bbbabb311bb0000bb13bb13bbb333333bbb31bb31bb0000bb13bb13bbb333333bbb31bb31bb0000bb13bb13bbb333333bbb31bb31bb00
bb333113bbabbbbbbbbbbabb311333bbbb3331333333bba77abb3333331333bbbb3331333333bba77abb3333331333bbbb3331333333bba77abb3333331333bb
bbbb31333bbaa7bbbb7aabb33313bbbbb7713ee6633333bbbb3333366ee3177bb7713ee6633333bbbb3333366ee3177bb7713ee6633333bbbb3333366ee3177b
3b333313333bbb7777bbb333313333b337113eefff663333333366fffee3117337113eefff663333333366fffee3117337113eefff663333333366fffee31173
c333333bb33333bbbb33333bb333333cc3773efff77f17711111f77fffe3773cc3773efff77f17711111f77fffe3773cc3773efff77f17711111f77fffe3773c
0c3bb3b3bbb3333333333bbb3b3bb3c00c3b3eff777717711c717777ffe3b3c00c3b3eff777717711c717777ffe3b3c00c3b3eff777717711c717777ffe3b3c0
00c1bb3b33bbbb3333bbbb33b3bb1c0000c1b3ef7777711cc7177777fe3b1c0000c1b3ef7777711cc7177777fe3b1c0000c1b3ef7777711cc7177777fe3b1c00
00013bb3bb333bbbbbb333bb3bb3100000013b3eff777711117777ffe3b3100000013b3eff777711117777ffe3b3100000013b3eff777711117777ffe3b31000
0331c3bb33aaa333333aaa33bb3c13300331c3b3eef7777777777fee3b3c13300031c3b3eef7777777777fee3b3c13000031c3b3eef7777777777fee3b3c1300
3bb31c3bbb333a7777a333bbb3c13bb33bb31c3b33eee777777eee33b3c13bb303b31c3b33eee777777eee33b3c13b30003b1c3b33eee777777eee33b3c1b300
3ccc13c3bbbbb333333bbbbb3c31ccc33ccc13c3bb333eeeeee333bb3c31ccc33bcc13c3bb333eeeeee333bb3c313cb303bc13c3bb333eeeeee333bb3c31cb30
00003b3c33bbbba77abbbb33c3b3000000003b3c33bbb333333bbb33c3b300003c003b3c33bbb333333bbb33c3b300cc03c0333c33bbb333333bbb33c3330c30
0003b3ccc333bbbbbbbb333ccc3b30000003b3ccc333bba77abb333ccc3b300000003b3cc333bba77abb333cc3b3000000003b3cc333bba77abb333cc3b30000
00033c003bc33bbbbbb33cb300c3300000033c003bc33bbbbbb33cb300c33000000033c03bc33bbbbbb33cb30c33000000003bc03bc33bbbbbb33cb30cb30000
0003c0003b3c3cb22bc3c3b3000c30000003c0003b3c3cb22bc3c3b3000c300000003c003b3c3cb22bc3c3b300c30000000003c0c3bc3cb22bc3cb3c0c300000
0000000033c0cc2112cc0c33000000000000000033c0cc2112cc0c330000000000000000c330cc2112cc033c00000000000000000c30cc2112cc03c000000000
00000000cc0000c33c0000cc0000000000000000cc0000c33c0000cc00000000000000000cc000c33c000cc0000000000000000000cc00c33c00cc0000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000070000020000200200002002000020020000205555555555555555555555555555555502222220022222200222222002222220
000bb000000bb0000007700000077000022ff220022ff220022ff220022ff2200578875005788750d562465d0578875022e66e2222e66e2222e66e2222e66e22
0066660000666600606666066066660602ffff2002ffff2002ffff2002ffff2005624650d562465d05177150d562465d27761772277617722776177227716772
0566665065666656b566665bb566665b0077d7000077d700007d77000077d700d517715d051771500566865005177150261aa172216aa162261aa612261aa162
65637656b563765b056376500563765008577580085775800857758008577580056686500566865005d24d50056686502ee99ee22ee99ee22ee99ee22ee99ee2
b063360b006336000063360000633600080550800805508008055080080550805d5245d505d24d500505505005d24d5022299222229999222229922222299222
006336000063360000633600006336000c0000c007c007c007c00c7007c007c05005500505055050050000500505505020999902020000202099990202999920
0006600000066000000660000006600000c7c7000007c0000077cc000007c000dd0000dd0dd00dd005dddd500dd00dd022000022022002202200002202200220
00ff880000ff88000000000000000000200000020200002000000000000000003350053303500530000000000000000000000000000000000000000000000000
0888888008888880000000000000000022000022220000220000000000000000330dd033030dd030005005000350053000000000000000000000000000000000
06555560076665500000000000000000222222222222222200000000000000003b8dd8b3338dd833030dd030030dd03003e33e300e33e330033e333003e333e0
6566665576555565000000000000000028222282282222820000000000000000032dd2300b2dd2b0038dd830338dd833e33e33e333e33e333e33e333e33e333e
57655576555776550000000000000000288888822888888200000000000000003b3553b33b3553b3033dd3300b2dd2b033300333333003333330033333300333
0655766005765550000000000000000028788782287887820000000000000000333dd333333dd33303b55b303b3553b3e3e3333bbe33333ebe3e333be3e3333b
0057650000655700000000000000000028888882080000800000000000000000330550330305503003bddb30333dd3334bbbbeb44bbbebb44bbbbeb44bbbebe4
00065000000570000000000000000000080000800000000000000000000000000000000000000000003553000305503004444440044444400444444004444440
0066600000666000006660000068600000888000002222000022220000222200002222000cccccc00c0000c00000000000000000000000000000000000000000
055556000555560005585600058886000882880002eeee2002eeee2002eeee2002eeee20c0c0c0ccc000000c0000000000000000000000000000000000000000
55555560555855605588856058828860882228802ee77ee22ee77ee22eeeeee22ee77ee2c022220ccc2c2c0cc022220c00222200000000000000000000000000
55555550558885505882885088222880822222802ee77ee22ee77ee22ee77ee22ee77ee2cc2cac0cc02aa20cc0cac2ccc02aa20c000000000000000000000000
15555550155855501588855018828850882228802eeeeee22eeeeee22eeeeee22eeeeee2c02aa20cc0cac2ccc02aa20ccc2cac0c000000000000000000000000
01555500015555000158550001888500088288002222222222222222222222222222222200222200c022220ccc2c2c0cc022220c000000000000000000000000
0011100000111000001110000018100000888000202020200202020220202020020202020000000000000000c000000cc0c0c0cc000000000000000000000000
00000000000000000000000000000000000000002000200002000200002000200002000200000000000000000c0000c00cccccc0000000000000000000000000
000880000009900000089000000890000000000001111110011111100000000000d89d0000189100001891000019810000005500000050000005000000550000
706666050766665000676600006656000000000001cccc1001cccc10000000000d5115d000d515000011110000515d0000055000000550000005500000055000
1661c6610161661000666600001666000000000001cccc1001cccc1000000000d51aa15d0151a11000155100011a151005555550055555500555555005555550
7066660507666650006766000066560000000000017cc710017cc71000000000d51aa15d0d51a15000d55d00051a15d022222222222222222222222222222222
0076650000766500007665000076650000000000017cc710017cc710000000006d5005d6065005d0006dd6000d50056026060602260606022666666226060602
000750000007500000075000000750000000000001111110011111100000000066d00d60006d0d600066660006d0d60020000002206060622222222020606062
00075000000750000007500000075000000000001100001101100110000000000760067000660600000660000060660020606062222222200000000022222220
00060000000600000006000000060000000000001100001101100110000000000070070000070700000770000070700022222220000000000000000000000000
0007033000700000007d330003330333000000000022220000000000000000000000000000000000000000000000000000000000000000000000000000000000
000d3300000d33000028833003bb3bb3000000000888882000000000000000000000000000000000000000000000000000000000000000000000000000000000
0778827000288330071ffd1000884200002882000888882000288200000000000000000000000000000000000000000000000000000000000000000000000000
071ffd10077ffd700778827008ee8e800333e33308ee8e80088ee883000000000000000000000000000000000000000000000000000000000000000000000000
00288200071882100028820008ee8e8003bb4bb308ee8e8008eeee83000000000000000000000000000000000000000000000000000000000000000000000000
07d882d00028820007d882d00888882008eeee800088420008eeee80000000000000000000000000000000000000000000000000000000000000000000000000
0028820007d882d000dffd0008888820088ee88003bb3bb3088ee880000000000000000000000000000000000000000000000000000000000000000000000000
00dffd0000dffd000000000000222200002882000333033300288200000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000077777777777777777777777777777777777777777777777777777777777777777777777777777777777777700000
00000000000000000000000000000000007777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777000
0000149aa94100000000012222100000077778888778877778878888887888887777888887788777788777778888877777788887777887777788788888777700
00019777aa921000000029aaaa920000077888888878877778878888887888888777888888778877887777778888887778888888877888777888788888877700
0d09a77a949920d00d0497777aa920d077788e778878877778878877777887788877887788878877887777778877887778877888877888777888788778877770
0619aaa9422441600619a779442941607788e7777778877778878877777887778877887778877888877777778877887788777888887888878888788778877770
07149a922249417006149a9442244160778877777778888888878888887887788877887788877888877777778888877788778888887888878888788888777770
07d249aaa9942d7006d249aa99442d60778877777778888888878888887888888777888888777788777777778888887788888888887887888788788888877770
067d22444422d760077d22244222d7707788e7777778877778878877777888887777888887777788777777778877e8878888888888788788878878877e887770
0d666224422666d00d776249942677d077788e778878877778878877777887888777887888777788777777778877788778888888877887787788788777887770
066d51499415d66001d1529749251d10077888888878877778878888887887788877887788877788777777778888888778888888877887787788788888887700
0041519749151400066151944a1516600777788887788777788788888878877788878877788877887777777788888e7777788887777887777788788888e77700
00a001944a100a0000400149a4100400007777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777000
00000049a400090000a0000000000a00000077777777777777777777777777777777777777777777777777777777777777777777777777777777777777700000
00000000000000000000000000000900000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
__sfx__
000100003452032520305202e5202b520285202552022520205201b52018520165201352011520010200f5200c5200a5200852006520055200452003510015200052000000000000000000000000000100000000
000100002b650366402d65025650206301d6201762015620116200f6100d6100a6100761005610046100361002610026000160000600006000060000600006000000000000000000000000000000000000000000
00010000377500865032550206300d620085200862007620056100465004610026000260001600006200070000700006300060001600016200160001600016200070000700007000070000700007000070000700
000100000961025620006000060000600006000060000600006000060000600006000060000600006000060000600006000060000600006000060000600006000060000600006000060000600006000060000600
00060000010501605019050160501905001050160501905016050190601b0611b0611b061290001d000170002600001050160501905016050190500105016050190501b0611b0611b0501b0501b0401b0301b025
00060000205401d540205401d540205401d540205401d54022540225502255022550225500000000000000000000025534225302553022530255301d530255302253019531275322753027530275322753027530
000600001972020720227201b730207301973020740227401b74020740227402274022740000000000000000000001672020720257201b730257301973025740227401b740277402274027740277402774027740
011000001f5501f5501b5501d5501d550205501f5501f5501b5501a5501b5501d5501f5501f5501b5501d5501d550205501f5501b5501a5501b5501d5501f5502755027550255502355023550225502055020550
011000000f5500f5500a5500f5501b530165501b5501b550165500f5500f5500a5500f5500f5500a550055500a5500e5500f5500f550165501b5501b550165501755017550125500f5500f550125501055010550
011000001e5501c5501c550175501e5501b550205501d550225501e55023550205501c55026550265500000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0110000017550145501455010550175500b550195500d5501b5500f5501c550105500455016550165500000000000000000000000000000000000000000000000000000000000000000000000000000000000000
090d00001b0001b0001b0001d0001b0301b0001b0201d0201e0302003020040200401e0002000020000200001b7001d7001b7001b7001b7001d700227001a7001b7001b700167001b7001b7001b7001c7001c700
050d00001f5001f0001f500215001f5301f0001f52021520225302453024530245302250024500245002450000000000000000000000000000000000000000000000000000000000000000000000000000000000
010d00002200022000220002400022030220002203024030250302703027030270302500027000270002700000000000000000000000000000000000000000000000000000000000000000000000000000000000
4d1000002b0202b0202b0202b0202b0202b0202b0202b0202b020290202b0202c0202b0202b0202b0202602026020260202702027020270202b0202b0202b0202a0302a0302a0302703027030270302003020030
4d1000002003028030280302c0302a0302a0302a0302703027030270302c0302a030290302e0302e0300000000000000000000000000000000000000000000000000000000000000000000000000000000000000
010f00001e050000001e0501d0501b0501a0601a0621a062000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
050f00001b540070001b5401a54018540175501755217562075000700007000070000700007000070000700007000070000700007000070000700007000070000700007000070000700007000070000700007000
010c0000290502c0002a00029055290552a000270502900024000290002705024000240002400027050240002a05024000240002a0552a055240002905024000240002400029050240002a000290002405026200
510c00001431519315203251432519315203151432519325203151431519325203251431519315203251432519315203151432519325203151431519325203251431519315203251432519315203151432518325
010c00000175001750017500175001750017500175001750017500175001750017500175001750017500175001750017500175001750017500175001750017500175001750017500175001750017500175001750
010c0000195502c5002a50019555195552a500185502950024500295001855024500245002450018550245001b55024500245001b5551b555245001955024500245002450019550245002a500295001855026500
010c0000290502c0002a00029055290552a000270502900024000290002000024000240352504527050240002a050240002f0052d0552c0552400029050240002400024000240002400024030250422905026200
010c0000195502c5002a50019555195552a500185502950024500295002050024500145351654518550245001b550245002f5051e5551d5552450019550245002450024500245002450014530165401955026500
010c00002c05024000240002a05529055240002e050240002400029000270502400024000240002e050240003005024000240002e0552d05524000300502400024000290002905024000270002a0002900028000
510c0000143151931520325143251931520315163251932516315183151932516325183151931516325183251b3151e315183251b3251e315183151b3251e325183151b3151d325183251b3151d315183251b325
010c00000175001750017500175001750017500175001750037500375003750037500375003750037500375006750067500675006750067500675006750067500575005750057500575005750057500575005750
010c00001d55024500245001b55519555245001e550245002450029500165502450024500245001e550245001e55024500245001d5551b555245001d5502450024500295001855024500275002a5002950028500
11050000385623555233552315522f5522d5522b5522954227552265522355222552215521e5421d5421a5421854217542155421454212542105420e5420d5320b53209522075120551203512015120051200512
48020000173520f302113420932208322073200735000400004000040000400004000040000400004000040000400004000040000400004000040000400004000040000400004000040000400004000040000400
080c000013056170661c06620066220362905631036320063600632006270061f0061900617000120002a00027000000000000000000000000000000000000000000000000000000000000000000000000000000
000a0000070560c0660f07616076180661f056220472703733037330573c0673e0062b00625006200061b0061700614006110060f0060d0060c0060a006090060600606006050060500600000000000000000000
000400000744007420074200a40000400004000040000400004000040000400004000040000400004000040000400004000040000400004000040000400004000040000400004000040000400004000040000400
4a0200002c6412f66130661316613766132661326612b6612866125671226611e661146611a651166510864111641056410c64105641046410264102631026310163101621006210062100611006110061100611
010100000914008150081600f160121400f1400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
020400003b6702b6403b67021620376702867031670266502c6502a650276502565022650206501d6501b6501965017640166401464012640106400d6400c6300a63008620076200562004620026200162000620
010a00000c4200c4200c4200c4200c4200c4200c4200c4200f4200f4200f4200f4200f4200f4200f4200f42010420104201042010420104201042010420104201442014420144201442014420144201442014420
010a00000532105320053200532005320053200532005320083200832008320083200832008320083200832009320093200932009320093200932009320093200d3200d3200d3200d3200d3200d3200d3200d320
000a002034615296152b6161e6061c6401d6452b6152760528615296152b6151e6001c6401d6452b6152761534615296152b6161e6061c6401d6452b6152760528615356152b6151e6051c6401d6452b61527615
050a00200232002320023200232002320023200232002320023200230502325023250232002325023200232503320033200332003320033200332003320033200732007320073200732007320073200732007320
010a000002320023200232002320023200232002320023200a3200a3200a3200a3200a3200a3200a3200a32005320053200532005320053200532005320053200332003320033200332003320033200332003320
010a000009220092200922009220092200922009220092200e2200e2200e2200e2200e2200e2200e2200e2200a2200a2200a2200a2200a2200a2200a2200a2200022000220002200022001220012200122001220
010a000005220052200522005220052200522005220052200e2200e2200e2200e2200e2200e2200e2200e2200a2200a2200a2200a2200a2200a2200a2200a2200022000220002200022001220012200122001220
010a00000d2200d2200d2200d2200d2200d2200d2200d220052200522005220052200522005220052200522011220112201122011220112201122011220112200322003220032200322003220032200322003220
150a00001522015220152201522015220152201522015220152201522015220152201322013220152201522016220162201622016220162201622016220162201922019220192201922019220192201922019220
150a00001a2201a2201a2201a2201a2201a2201a2251a2251d2201d2201d2201d2201d2201d2201d2201d22019220192201922019220192201922019220192201622016220162201622016220162201622016220
150a0000192201922019220192201922019220192251922511220112201122011220112201122011220112201d2201d2201d2201d2201d2201d2201d2201d22018220192211a2211d22121221252212622126221
090a00001d2171a217212172221729217262172d2172e2171d2171a2172121722217112170e21715217162171d2171a217212172221729217262172d2172e2171d2171a2172121722217112170e2171521716217
090a000029217262172d2172e2173521732217392173a21729217262172d2172e2171d2171a2172121722217112170e21715217162171d2171a2172121722217112170e21715217162170521702217092170a217
010a00000e003296000e0031e600286151d6052b605276150e003296052b6151e600286151d6452b615276051f6501f6301f6201e6001f6251f6251f625276050e003356052b6051e605106111c6112862133631
5c030000131212513131151381711b1613b1513b1413c14116141291413913135131321312d13228132221321c13216132131321d1320e1320d1320a132091320813206122051220412203122031220312201120
5c0400000817120161181610f17108171171711017109171071710d1610f161091510715106151051410514105132041320313202132021320113201132001320113201132011320112200122001220012200122
__music__
04 04050644
00 07084749
04 090a484a
04 0b0c0d44
00 0e084344
04 0f0a4344
04 10114e44
01 12131415
00 16131417
02 18191a1b
00 24256844
01 26272844
00 26282966
00 26272a65
00 262a2b65
00 26272c44
00 26292d44
00 26272c44
00 262a2e44
00 28292f44
00 28293044
00 272b2f44
02 25243144

