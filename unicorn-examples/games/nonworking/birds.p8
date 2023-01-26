pico-8 cartridge // http://www.pico-8.com
version 36
__lua__
--      -- birds with guns --
--by yolwoocle & gouspourd
--extra credits :notgoyome
--music: simon t.

--bird ideas
--crow,owl,raven,pelican,goose,
--colibri,dinosaur,cockatiel
--peafowl

degaplus = 0

function _init()
	keyboard = false
	initguns()
	enemies,checker = {},{}
	--mouse
	mx,my=0,0  
	
	--flags
	solid,breakable,spawnable,lootable,notbulletsolid=0,1,2,3,4
	
	--camera
	shake,camy,camx,targetcamx=0,0,0,0
	
	cam_follow_player=true
	trainpal = {{8,2},{11,3},
	{7,13},{12,13},{10,9},{0,2}}
	
	pal_n,menu = 1,"main"
	
	actors = {}
	init_enemies()
	init_player(111)
	
	init_ptc()
	wl,diffi = 4,20
	
	--wagon length
	wagon_n,trainlen = 0,6
	tl = trainlen
	
	gen_train()
	update_room()
	random,enemies = {},{}
	parcourmap()
	
	drops={}
	
	init_menus()
	
	birdchoice,hardmodetimer=0,0
	ofsetboss,bullets_shooted,bullets_hit = 0,1,1
	
	stats={
	 time=0,
	 kills=0,
	 wagon=0,
	}
	
	is_boss = false
	boss_pos = {0,0}
	win,wintimer = false,0
	
	--darker blue
	--pal(1,130,1)
	--pal(1,129,1)
	pal()
	poke(0x5f2e,1)
	
	local s = stat(6)
	if s == "-"
	or s == "" 
	or s == nil then
		menu="main"
	else
		menu="game"
		birdchoice=tonum(stat(6))
		begin_game()
	end
	
end

function _update()
	mouse_x_y()
	for i=0x3100,0x3148 do
		--on
		poke(i,peek(i)&0b10111111)
	end
	

	if hardmodetimer > 90 and 
	menu == "main" and diffi != 17 then
		sfx(44)
		shake,diffi = 5,17
		degaplus = 1
		initguns()
		init_enemies()
	end

	grasstile()
	if(win) wintimer += 1
	if wintimer == 180 then
		menu="win" 
		set_stats()
	end
	--if(wintimer==180)sfx(46)
	if menu == "game" then
		delchecker()
		
		update_drops()
		player_update()
		
		for a in all(actors) do
			--actors are just bullets
			a:update()
			
			if a.destroy_flag then
				if a.dmg == 0 then
					animexplo(a)
					guns.explosion:fire(a.x-a.dx*2,a.y-a.dy*2,1)
				elseif a.dmg == 0.1 then
					--firework launcher
					for i=1,10 do
						sfx(32)
						guns.machinegun:shoot(a.x-a.dx*2,a.y-a.dy*2,i/10)
					end
				end
				del(actors,a)
			end
		end
		
		--for e in all(enemies) do
			update_enemy(e)
			--if(e.destroy_flag)del(enemy,e)
		--end
		
		for ptc in all(particles) do
			update_ptc(ptc)
			if(ptc.destroy)del(particles,ptc)
		end
		
		update_door()
		
		--shake = 0
		
		update_camera()
	elseif menus[menu] then
		local m = menus[menu]
		m.update(m)
		sprms = 127
	end
	
	shake = max(0,shake-0.3)
	
	local txt=keyboard and "keyboard" or "mouse+keys"
	menuitem(3,"mode:"..txt, function() keyboard = not keyboard end)
	menuitem(2,"⌂ main menu", function() run("-") end)
	
	if (btn(❎) or btn(🅾️)) keyboard = true 
	if (lmb) keyboard = false
end


function _draw()
	local ox = rrnd(shake)
	local oy = rrnd(shake)
	--if (keyboard == true) 
	camera(camx+ox, camy+oy)
	
	cls(15)
	
	--draw map
	drawgrass()
	
	draw_map()
	if wagon_n==0 and menu=="game" then
		local s= [[
  ⬆️        [e]
⬅️⬇️➡️ or [s d f] 
     move 

[click]  shoot
[scroll] change 
          weapon
		]]
		if(keyboard)s=[[
    ⬆️
  ⬅️⬇️➡️ move 

  ❎ (x) shoot
  🅾️ (c) change 
         weapon
		]]
		print(s,33,42,2)
	end

	draw_weel()
	
	draw_drops()
	
	for e in all(enemies) do
		draw_enemy(e)
	end
	draw_player()
	
	for a in all(actors) do
		a:draw()
	end
	
	for ptc in all(particles) do
		draw_ptc(ptc)
	end
	
	if(menu!="main")draw_player_ui(p)
	
	local m = menus[menu]
	if m then
		m.draw(m)
	end
	
	-->>no code below this<<--
	--draw mouse
	
	if(not keyboard or menu=="game")spr(sprms,mx-1,my-1)
	pal(1,129,1)
end

----------
function set_stats()
	stats.time=flr(
	 (time()-stats.time) * 10) / 10
	local t = stats.time
	local s = ""
	if(t%60 < 10) s="0"
	stats.time= tostr(t\60)..":"..s..tostr(t%60) 
	--stats.time=(flr(stats.time)\60)+((flr(stats.time)-(flr(stats.time)\60)*60)/100)
	stats.wagon=tostr(wagon_n+1).."/7"
end

function begin_game()
	sfx(37)
	music()
	
	local b=birdchoice or 0
	if b == 0 then
		b=flr(rnd(12))+1
	end
	init_player(111+b)
	
	stats.time = time()
	
	p.x,p.y = 48,56
	
	shake += 7
	for i=1,10 do
		make_ptc(
		 48 + rrnd(8),
		 56 + rrnd(8),
		 8+rnd(8),rnd({2,4,6}),0.97,
		 rrnd(2),rrnd(2)
		)
	end
end

function isleft(a)
	return a<.75 and .25<a
end

function ospr(s,x,y,col)
	for i=0,15 do
		pal(i,col)
	end
	
	for i=-1,1do
		for j=-1,1do
			spr(s,x+i,y+j)
		end
	end
	
	pal()
	spr(s,x,y)
end

function oprint(t,x,y,col,ocol)
	local ocol = ocol or 1
	for i=-1,1do
		for j=-1,1do
			print(t,x+i,y+j,ocol)
		end
	end
	
	local col = col or 7
	print(t,x,y,col)
end

function copy(t)
	local n={}
	for k,v in pairs(t) do
		n[k] = v
	end
	return n
end

function update_camera()
	local px = p.x
	local wl = wl
	local maxlen = 240
	
	--poke(0x5f40,0)
	poke(0x5f43,0)
	if px > 128*(wl-1)+8 then
		--pan cam to connector room
		cam_follow_player=false
		targetcamx=128*(wl-1)
		
		--block off old entrance
		if wagon_n != tl-1 then
			--mset(48,7,6)
			--mset(48,6,13)
		end
		
		--low-pass filter & slow
		--poke(0x5f40,15)
		if(wagon_n!=tl)poke(0x5f43,15)
	end
	if cam_follow_player then 
		--camera follows player
		camx = px-60
		--offset camera to cursor
		if not keyboard then
		camx += (stat(32)-64)/3
		end
		camx = mid(0, camx, 128*(wl-2)+8) \ 1
		camy = 0
	else
		--do a cool animation
		camx=ceil(
		 camx+(targetcamx-camx)/10)
		
		if targetcamx <= 0
		and ceil(camx)==targetcamx then
			cam_follow_player=true
		end
	end
end

function draw_ghost_connector()
	if camx<0 then
		map(0,16, -128,0, 16,16)
	end
end

function rrnd(n)
	--"radius rnd"
	return rnd(2*n)-n
end

function allbtn(b)
	return btn(b) or btn(b,1)
end

--[[
function debug_()
	local p =players[1]
	p.gunls[1] = debuggun
	wagon_n = tl-1
	for i in all(enemies)do 
		i.destory = true
	end
	p.maxlife = 10000
	p.life = 10000
end--]]

-->8
--player
function init_player(bird)
	b=0
	dx1=1
	dy1=0
	p = {
		n=1,
		agro = 9999,
		x=-64,y=-64,
		dx=0,dy=0,
		a=0,
		
		spd=.4,
		fric=0.75,
		
		bx=2,by=2,
		bw=4,bh=4,
		
		hx=2,hy=2,
		hw=4,
		
		life=10,
		maxlife=10,
		ammo=250,
		maxammo=250,
		
		spr=bird,
		
		gun=nil,
		gunn=1,
		gunls={},
	
		lmbp = true,
		iframes=30,
		
		damage=damage_player,
		spriteoffset = 0,
		kak = copy(kak)
	}

	p.gun = p.gunls[1]

	--should we keep this in? bird stats
	-- [[
	local n = bird-111
	--[[local bird_stats=split(
[[n,     1,  2,  3,  4,  5,  6,  7,  8,  9,  10, 11, 12
,life,   10, 10, 5,  10, 10, 10, 10, 10, 10, 2,  10, 10
,maxlife,10, 10, 5,  10, 10, 10, 10, 10, 10, 15, 10, 10
,spd,    .4, .4, .4, .4, .4, .4, .4, .4, .55, .4, .4, .4
,fric,   .75,.75,.75,.75,.75,.75,.75,.75,.75,.74,.75,.75
]]) 
	for i=1,#bird_stats,13 do
		p[bird_stats[i] ] = bird_stats[i+n]
	end
--]]
	--                        [    default    ][     pigeon    ][       duck         ][         sparrow            ][          parrot          ][    toucan         ][     flamingo   ][         eagle            ][    seagull       ][      ostrich     ][    penguin  ][      jay           ][     chicken    ] 
	local bird_weapons=split("revolver,shotgun,revolver,shotgun,revolver,flamethrower,boxing_glove,fireworklauncher,machinegun,fireworklauncher,boxing_glove,shotgun,revolver,ringcannon,boxing_glove,assaultrifle,machinegun,sniper,machinegun,gatlinggun,shotgun,sniper,shotgun,assaultrifle,revolver,bazooka")
	
	for i=1,2 do
		p.gunls[i] = copy(guns[bird_weapons[2*n+i] ])
	end
	
	update_gun(p)
end

function player_update()
	 --damage
	 p.iframes = max(0,p.iframes-1)
		--movement
		local dx,dy = p.dx,p.dy
		local spd = p.spd
		
		if (allbtn(⬅️)) p.dx-=spd dx1-=spd
		if (allbtn(➡️)) p.dx+=spd dx1+=spd
		if (allbtn(⬆️)) p.dy-=spd dy1-=spd
		if (allbtn(⬇️)) p.dy+=spd dy1+=spd
		
		--58
		
		p.dx *= p.fric
		p.dy *= p.fric
		
  if (abs(dx1)+abs(dy1))>0.1 then
		dx1 *= p.fric
		dy1 *= p.fric
		end
		
		collide(p,0.1)
		
		p.x += p.dx
		p.y += p.dy
		
		
		--animation
		
		if abs(p.dx)+abs(p.dy)>0.75  then
		 animplayer(p)
		else 
			p.spriteoffset = 0
		end
		
		--aiming
		if keyboard then
			sprms=76
			distmin=9999
			indexmininit={x=p.x-ofsetboss+dx1,y=p.y-ofsetboss+dy1}
			indexmin=indexmininit
			for e in all(enemies) do
			 	if loaded(e) and 
			 	canshoot(p,e) then
					local dist = dist(p,e)
					if (distmin>dist) distmin=dist indexmin=e
				end
			end

			ofsetboss = 0
			if(indexmin.spr == 1) ofsetboss = 4

			p.a = atan2(indexmin.x+ofsetboss-p.x,
			indexmin.y+ofsetboss-p.y)
			mx=indexmin.x+1+ofsetboss
			my=indexmin.y+1+ofsetboss
			if(indexmin==indexmininit)sprms=57
		else
			sprms = 127
			p.a = atan2(mx-p.x,
			my-p.y)	
		end
		p.flip = isleft(p.a)
		
		--ammo & life
		p.life=min(max(0,p.life),p.maxlife)
		p.gun.ammo=min(max(0,p.gun.ammo),p.gun.maxammo)
		
		--death
		if p.life <= 0 
		and menu!="death"then
			sfx(34)
			music(-1, 300)
			
			menu = "death"
			shake += 9
			burst_ptc(p.x+4,p.y+4,7)
			
			set_stats()
		end
		
		--shooting
		if stat(36) ==1 or stat(36) ==-1 or (btnp(🅾️)) then
			nextgun(p)
			--print(p.gun.cooldown,0,0)
			p.gun.timer = p.gun.cooldown/2
		end
		
		
		local fire=lmb or btn(❎) 
		local active=rmb
		
		local dofire
		
		p.kak:update()
		p.gun:update()
		test = p.gun.name
		-- not auto
		if fire and
		p.gun.timer<=0 and
		p.gun.ammo > 0 and p.gun.auto == false
		then
			if p.lmbp == true then
				dofire = true
				
				p.lmbp = false
			end
			
		-- auto
		elseif fire and p.gun.timer<=0 
		and p.gun.ammo > 0 then
			dofire = true
		elseif fire and
        p.gun.ammo < 1 and
        p.lmbp == true and
        p.kak.timer<=0 then
        coupdekak(p) 
        p.lmbp = false
		end
		
		if dofire then
			make_ptc(p.x+cos(p.a)*6+4, 
				p.y+sin(p.a)*3+4, rnd(3)+6,7,.7)
				
			p.gun.ammo -= 1
			p.gun:fire(p.x+4,p.y+4,p.a)
		end
		
		-- if mleft not pressed 
		if not fire then
			p.lmbp = true
		end
		
		--begin boss
		local w3=128*(wl-1)
		
		if wagon_n==tl and p.x>w3 
		and cam_follow_player 
		and not is_boss then
			is_boss = true
			begin_boss()
			p.x = w3+8
		end
		
		--next wagon
		if p.x>128*wl then
			random = {}
			
			wagon_n += 1
			update_room()
			enemiescleared=false
			pal_n += 1
			
			--pan cam to next wagon
			camx = -128
			targetcamx=0
			drops = {}
			enemies = {}
			parcourmap()
			--teleport players
			p.x -= 128*wl
			p.x = max(p.x, 0)
		end
		for e in all(enemies)do
			
			if touches_rect(
			p.x+4,p.y+4,
			e.x+1,e.y+1,e.x+7,e.y+7) then
				
				if (p.iframes == 0) then
					sfx(35)
					if(shake<=2)shake += 2
					if e.spr != 126 then
						p.life-=1+(degaplus*2)
					else 
						p.life-=1+degaplus
					end
					p.iframes = 30
					
					if e.spr == 109 then
					 p.life+=1
					 killbarelle(e)
					 p.iframes = 0
					end
				end
				knockback_player(p,e)
			
			end
		end
end

function draw_player()
	if (p.iframes%5) == 0  then
		local x=flr(p.x) + cos(p.a)*6 +0
		local y=flr(p.y) + sin(p.a)*3 +0
		
		if p.gun.name=="sniper" then
			local c,s=cos(p.a),sin(p.a)
			line(
			x+4+c*6,
			y+4+s*6,
			p.x+c*128,
			p.y+s*128,8)
		end
		spr(p.gun.spr,x,y,1,1, p.flip)
		
		
		palt(0,false)
		palt(1,true)
		
		spr(p.spr,p.x,p.y+p.spriteoffset,1,1, p.flip)
		
		palt()
	end
end

function draw_player_ui(p)
	--life counter
	rectfill(camx+1,1,camx+43,7,2)
	local l=40*(p.life/p.maxlife)
	rectfill(camx+2,2,camx+2+l,6,8)
	
	local s="♥"..p.life.."/"..p.maxlife.." "
	print(s, camx+2,2,7)
	
	--ammo bar
	rectfill(camx+84,1,camx+84+42,7,4)
	local l=40*(p.gun.ammo/p.gun.maxammo)
	if(p.ammo>0)rectfill(camx+85,2,camx+85+l,6,9)
	
	local s,col = tostr(p.gun.ammo),7
	if(s=="0") s,col="no ammo!",14
	spr(110,camx+89,2)
	print(s, camx+95,2,col)
	
	--weapon list
	for i=1,2 do
		local col = 1
		if(i==p.gunn)col=7
		
		ospr(p.gunls[i].spr, 
		camx+90+(i-1)*10, 10,col)
	end
	
	--wagon
	local color = 7
	if degaplus == 1 then
		color = 8
		else color = 7
	end
	oprint("wagon "..wagon_n+1 .."/7",
	camx+46,2,color,1)
	--print(test,0,80)
end

function nextgun(p)
	sfx(36)
	
	p.gunn += 1
	if(p.gunn > #p.gunls) p.gunn = 1
	update_gun(p)
	--[[local f = 0
	for i=1,#p.gunls do
	if p.gunls[i] == p.gunn then
		if (((i+stat(36))%#p.gunls)<1) f = #p.gunls
		 return p.gunls[(i+stat(36))%(#p.gunls)+f]
		end
	end]]
end

function update_gun(p)
	p.gun = p.gunls[p.gunn]
end

function knockback_player(p,e)
	if abs(p.dx)+abs(p.dy) < 3 then
			  p.dx+=e.dx*e.spd*2
				 p.dy+=e.dy*e.spd*2
			end	
end

function knockback_enemy(e,b)
	
		if (abs(e.dx)+abs(e.dy<30)) then
			e.dx+=b.dx*b.spd*.1
			e.dy+=b.dy*b.spd*.1
		end

end

function animplayer(p)
	if flr(time()*7)%2==1 then
		p.spriteoffset = 1
	else 
		p.spriteoffset = 0
	end
end

function coupdekak(p)
	local x=flr(p.x) + cos(p.a)*6 +0
	local y=flr(p.y) + sin(p.a)*3 +0
	
	p.kak:fire(x+4,y+4,p.a)
end
-->8
--gun & bullet

function make_gun(args,fire)
	local name_,sprr,cd_,spd,oa,dmg,is_enemy,auto,maxammo,sfxx,knockback=unpack(split(args))

	
	is_enemy = is_enemy == 1
	auto = auto  == 1
	if(is_enemy) dmg += degaplus * 2
	
	--todo:not have 3000 args
	local gun = {
		name=name_,
		spr=sprr,
		spd=spd,
		oa=oa,--offset angle in [0,1[
		dmg=dmg,
		shake=shake,
		auto=auto,
		
		ammo=maxammo,
		maxammo=maxammo,
		
		timer=0,
		cooldown=cd_,
		is_enemy=is_enemy,
		
		x=0,y=0,
		dir=0,
		burst=0,
		
		sfx=sfxx,
		knockback=knockback,
	}
	
	gun.fire = fire
	
	gun.shoot=function(gun,x,y,dir,spd,knockback)
		--remove? it complicates code
		if(gun.burst<=0)dir+=rrnd(gun.oa)
		
		if(gun.sfx) sfx(gun.sfx)
		
		local s=93
		local name = gun.name
		local palette = ""

		if(gun.is_enemy)s=95
		if(name=="kak")s=77 lifspa=5
		if(name=="boxing glove")s,lifspa=77,10
		if(name=="flamethrower") lifspa=40 palette="1,2,3,4,5,6,10,8,8,9"
		if(name=="explosion")s=57 lifspa=10
		if(name=="bazooka") palette="1,2,3,4,5,6,6,8,5,13"
		if not gun.is_enemy then
			if(shake<1 and name!="flamethrower")shake+=1 
		end
		
		spd = spd or gun.spd
		spawn_bullet(x,y,dir,
		spd,3,s,dmg,is_enemy,lifspa,palette)
		lifspa=nil
		gun.timer = gun.cooldown
		p.dx-=cos(dir)*gun.knockback
		p.dy-=sin(dir)*gun.knockback
	end
	
	gun.update=function(gun)
		gun.timer = max(gun.timer-1,0)
		gun.ammo = mid(0,gun.ammo,gun.maxammo)
		
		if gun.burst > 0 then
			gun:shoot(gun.x,gun.y,gun.dir)
			gun.burst -= 1
		end
	end
	
	return gun
end

function shoot1(gun,x,y,dir)
	gun:shoot(x,y,dir)
end

-- init guns

--degaplus = 0
                   --name      spr cd spd oa dmg is_enemy auto maxammo sfx knock
--debuggun = make_gun("debuggun, 64, 1, 3, .02, 0, 0,       1,   999999, 64, 1",
--		function(gun,x,y,dir)
--	  for i=1,7 do
--	 		p.life += 1
--	 		gun:shoot(x,y,dir+rrnd(.1), ospd)
--	 	end
--end
--		function(gun,x,y,dir)
--			gun:shoot(x,y,dir)
--		end
--	)
function initguns()
guns = {

                       --name   spr cd spd oa dmg is_enemy auto maxammo sfx
	revolver = make_gun("revolver, 64, 15,2.5,.02,3 ,0,       0,   100,    33, 0.3",
		shoot1
	),
	
	
	fireworklauncher = make_gun("firework launcher, 74, 25,2.5,.02,0.1 ,0,       0,   80,    52, 0.6",
		shoot1
	),
	
	boxing_glove = make_gun("boxing glove, 72, 18,3.3,.005,1 , 0, 0, 1,      53, -0.96",
 function(gun,x,y,dir)
  for i=1,7 do
			gun:shoot(x,y,dir)
		end
		p.iframes,gun.ammo = 9,gun.maxammo
		end
 ),
	
	
	bazooka = make_gun("bazooka, 69, 90,1.5,.007,0 ,0,       0,   20,    33, 4.5",
		shoot1
	),
	
	flamethrower = make_gun("flamethrower, 70, 2,1.5,.015,0.34 ,0,       1,   1500,    51, 0",
		shoot1
	),
	
	ringcannon = make_gun("ring cannon,    71, 45,2, .01,3,  0,   0,  50,    32, 0",
	 function(gun,x,y,dir)
	 	for i=1,20 do
	 		local o=i/20
	 		local ospd=gun.spd*(rnd(.2)+.9)
	 		gun:shoot(x,y,dir+o, ospd)
	 	end
	 end),
	                    --name    spr cd spd oa dmg is_enemy auto maxammo sfx
	shotgun = make_gun("shotgun,    65, 60,4, .05,1.25,  0,   0,  50,    32, 0.4",
	 function(gun,x,y,dir)
	 	for i=1,7 do
	 		local o=rrnd(.05)
	 		local ospd=gun.spd*(rnd(.2)+.9)
	 		gun:shoot(x,y,dir+o, ospd)
	 	end
	 end),
	 
	                         --name      spr cd spd oa dmg is_enemy auto maxammo sfx
	machinegun = make_gun("machinegun, 66, 7, 3, .05,2  ,0,       1, 250,    33, .2",
		shoot1
	),
	
	                          --name           spr cd spd oa dmg is_enemy auto maxammo sfx
	assaultrifle = make_gun("assault rifle, 67, 30,4, .02,1   ,0,       1, 75,      33, .3",
		function(gun,x,y,dir)
			gun.burst = 4
			gun.x, gun.y = x, y
			gun.dir =dir+(rrnd(1))*gun.oa
			gun:shoot(x,y,gun.dir)
		end
	),
	
	                   --name  spr cd spd oa dmg is_enemy auto maxammo sfx
	sniper = make_gun("sniper, 68, 40,7, .0, 5  ,0,        0,  35,     32, 3",
		shoot1
	),
	
	                            --name  spr cd spd oa dmg is_enemy auto maxammo sfx   kb
	gatlinggun = make_gun("gatling gun, 73, 3, 3, .08, 2  ,0,        1,  500,     33, 1",
		shoot1
	),
	
	                      --name      spr cd  spd oa   dmg is_enemy auto maxammo sfx kb
	gunslime = make_gun("gunslime, 64, 100,1.5, .02,2,  1,       1,   250,    32, 0",
		function(gun,x,y,dir)
			dir+=rrnd(gun.oa)
			gun:shoot(x,y,dir)
		end
	),
	
                              --name      spr cd spd   oa dmg is_enemy auto maxammo sfx
	gunslimebuff = make_gun("gunslimebuff, 64, 100,1, .04,2,  1,       1,   250, 32, 0",
		function(gun,x,y,dir)
			for i=0,2 do
				local o=rrnd(.05)
				local ospd=gun.spd*(rnd(.2)+.9)
				gun:shoot(x,y,dir+o, ospd)
			end
		end
	),
	
	
	shotgunmechant = make_gun("shotgunmechant, 65, 60,1.35, .04,3, 1, 1, 250, 32, 0",
		function(gun,x,y,dir)
	 		for i=1,4 do
	 			local o=rrnd(.05)
	 			local ospd=gun.spd*(rnd(.2)+.9)
	 			gun:shoot(x,y,dir+o, ospd)
	 		end
		end
	),
	 
	 null = make_gun("null, 57, 0,57, 0,1,  1,  1, 250, 32,0",
	 function() --opti: remove args
	 end),
	 
	 machinegunmechant = make_gun("machinegunmechant, 66, 5, .75,.05,2, 1, 1,250, 32, 0",
		shoot1
	),
	
	explosion = make_gun("explosion, 57, 0, 2,  0,5   ,1,  0, 1, 32, 0",
		function(gun,x,y,dir)

			for i=1,12 do
	 		local o=i/12
	 		gun:shoot(x,y,dir+o)

	 	end
	end

	),
	
	boss_targetgun = 
	make_gun("boss target gun, 65, 6, 1.2,.05,2, 1,  1, 250, 47, 0",
		shoot1
	),
	
	boss_360gun = 
	make_gun("boss 360 gun, 65, 1, 1,  0,2  ,1,  1,	250,    47, 0",
		function(gun,x,y,dir)
			gun.dir+=.176666
			gun:shoot(x,y,gun.dir)
		end
	),
	
	boss_enemygun = 
	make_gun("boss_enemygun, 65, 150, 1, 1,2   ,1,  1, 250, 33, 0",
		function(gun,x,y,dir)
			sfx(33)
			gun.timer = gun.cooldown
			if(rnd(2)<1)return spawn_enemy(x,y,enemy.hedgehog)
			spawn_enemy(x,y,enemy.hedgehogbuff)
		end
	),
}
	--table of number-indexed guns
	iguns={}
	for k,v in pairs(guns)do
		if(not v.is_enemy)add(iguns,v)
	end
end

kak = make_gun("kak, 57, 20,2.1,.005,3 , 0, 0, 0,      36, 1",
	shoot1
)


function rnd_gun()
	--todo: "power" param
	--later weapons should be  
	--more powerful
	return rnd(iguns)
end

function spawn_bullet(x,y,dir,spd,r,spr,dmg,is_enemy,lifspa,palette)
	local dx=cos(dir)*spd
	local dy=sin(dir)*spd
	add(actors,{
		x=x,  y=y,
		dx=dx,dy=dy,
		r=4,
		dmg=dmg,
		spd=spd,
		spr=spr,
		is_enemy=is_enemy,
		destroy_flag=false,
		dir=dir,
		
		update=update_bullet,
		draw=draw_bullet,
		lifspa=lifspa,

		palette=palette,
	})
end

function update_bullet(b)
 if not(b.lifspa== nil)then
  b.lifspa-=1
  if (b.lifspa== 0) b.destroy_flag = true
 end
	b.x += b.dx
	b.y += b.dy
	
	local bx,by = b.x,b.y 

	debug=""
	if b.is_enemy then
		
		local x2 = p.x+p.hx+p.hw
		local y2 = p.y+p.hx+p.hw 
		if touches_rect(bx,by,
		p.x+p.hx, p.y+p.hy,
		x2,y2) then
			
			if p.iframes == 0 then
				p.life-=b.dmg 
				p.iframes = 30
				sfx(35)
			end
			if(shake<=4)shake += 4
			knockback_player(p,b)
			make_ptc(bx,by,rnd(4)+6,7,.8)
			b.destroy_flag = true
			
		end
	end
		
	if not(b.is_enemy) or b.spr == 57 then
		
		for e in all(enemies)do
		 if loaded(e) then
			local x2 = e.x+e.hx+e.hw
			local y2 = e.y+e.hx+e.hw 
			
			if touches_rect(bx,by,
			e.x+e.hx,e.y+e.hy,
			x2,y2) then
				
				sfx(46)
				e.life -= b.dmg
				if e.life<=0 then
					--kill enemy
					stats.kills+=1 del(enemies,e)
					spawn_loot(e.x,e.y)
					if e.spr != 109 then
					burst_ptc(e.x+4,e.y+4,8,1,1,1)
					
					--boss death
					if e.spr==1 then
						shake += 10
						boss_pos = {e.x,e.y,e.flip}
						menu = "bossdeath"
					end
					
					else --animation explosion
						killbarelle(e)
					end
				end
				bullets_hit+=1
				
				knockback_enemy(e,b)
				
				e.timer = 5
				make_ptc(bx,by,rnd(4)+6,7,.8)
				b.destroy_flag = true
				return
			end
		end
		end
	end
	
	--destroy on collision
	if (is_solid(bx,by)
	and not check_flag(
	    notbulletsolid,bx,by)) 
	or bx+11<camx 
	or bx>camx+139 
	or by<-8 or by>132
	then
		
		if check_flag(breakable,bx,by) then
			--sfx(47)
			if check_flag(lootable,bx,by)then
				break_crate(bx,by)
			end
			mset(bx\8,by\8,39)
			add(random,{
			 x=(bx\8)*8+4,
			 y=(by\8)*8+4,
			 spr=rnd{55,22,39},
			 f=rnd{true,false},
			 r=rnd{true,false}
			})
		end
		make_ptc(bx,by,rnd(4)+6,7,.8)
			
		b.destroy_flag = true
	end
end

function draw_bullet(b)
	pal(split(b.palette))
	spr(b.spr, b.x-4, b.y-4,1,1, p.flip)
	pal()
end

function draw_random()
for i in all(random)do
	 spr(i.spr, i.x-4, i.y-4,1,1,i.f,i.r)
	end
end

function killbarelle(e)
 animexplo(e)
	e.gun:fire(e.x+4,e.y+4,e.a)
	del(enemies,e)
end

function animexplo(e)
for i=1,15 do
		make_ptc(
					e.x + rrnd(14),
					e.y + rrnd(14),
			8+rnd(8),rnd{9,10})
	end
sfx(37)
shake += 7
end

--------

--[[
function spawn_bullet(x,y,type_bullet,speed,timer_bullet1,sprite,nb_bullet,ecartement)
	if timer_bullet == 0 then
		local xy = get_traj(x,y,mouse_x,mouse_y)
		local traj_x = xy.x*speed
		local traj_y = xy.y*speed
		local angle = xy.angle
		timer_bullet = timer_bullet1
		
		if type_bullet == 1 then
			nvelement = {
			  x=x,y=y,
			  type_bullet=type_bullet,
			  traj_x=traj_x,
			  traj_y=traj_y,
			  sprite=sprite
			}
			rafale(10,nvelement)
		end
	 
	 if type_bullet == 2 then
	  for i=0,nb_bullet do
	   if nb_bullet == 0 then
	    add(bullet,{
	      x=x,y=y,
	      type_bullet=type_bullet,
	      traj_x=traj_x,
	      traj_y=traj_y,
	      sprite=sprite
	    })
				else 
					add(bullet,{
					  x=x,y=y,
					  type_bullet=type_bullet,
					  traj_x=cos((angle-((1/ecartement)/2)+(i/nb_bullet)/ecartement))*speed,
					  traj_y=sin((angle-((1/ecartement)/2)+(i/nb_bullet)/ecartement))*speed,
					  sprite=sprite
					})
	   end
	  end
	 end
 end
end

-- -(i/2)+i/nb_bullet
function update_bullet()
 if (timer_bullet>0)timer_bullet-=1
	for i in all(bullet) do
		if is_solid(i.x+(i.traj_x*1.5)+4,i.y+4+(i.traj_y*1.5)) then
		 del(bullet,i)
	 end
		i.x += i.traj_x
		i.y += i.traj_y
	end
end

function draw_bullet()
	for i in all(bullet) do
		spr(i.sprite,i.x,i.y)
	end
end

function rafale(nb,bullet)
 add(rafalels,{nb=nb,bullet=bullet})
end

function updaterafale()	
	for i in all(rafalels) do
  if (i.nb<1) del(rafalels,i)
  add(bullet,i.bullet)
  i.nb -=1
	end
end
--]]
-->8
--mouse
function mouse_x_y()
	poke(0x5f2d, 1)
	mx=stat(32)+flr(camx)
	my=stat(33)
	local s = stat(34)
	lmb=s&1 > 0
	rmb=s&2 > 0
end




--[[function get_traj(x_satr,y_start,x_end,y_end)
	angle=atan2(x_end-x_satr-4, 
	 y_end-y_start-4)
	return {x=cos(angle),
	 y=sin(angle),angle=angle}
end]]

function check_flag(flag,x,y)
	return fget(mget((x\8),(y\8)),flag)
end

-->8
--collision
function is_solid(x,y)
	if(x<0)return true 
	return check_flag(0,x,y)
end

function touches_rect(x,y,x1,y1,x2,y2)
	return x1 <= x
	   and x2 >= x
	   and y1 <= y
	   and y2 >= y
end
--[[
function circ_coll(a,b)
	--https://www.lexaloffle.com/bbs/?tid=28999
	--b: bullet
	local dx=a.x+4 - b.x
	local dy=a.y+4 - b.y
	local d = max(dx,dy)
	dx /= d
	dy /= d
	local sr = (a.r+b.r)/d
	
	return dx*dx+dy*dy < sr*sr 
end
--]]

function rect_overlap(a1,a2,b1,b2)
	--[[return not (a1.x>b2.x
	         or a1.y>b2.y 
	         or a2.x<b1.x
	         or a2.y<b1.y)--]]
	
	return a1.x<b2.x
	   and a1.y<b2.y 
	   and a2.x>b1.x
	   and a2.y>b1.y--]]
end

function collision(x,y,w,h,flag)
	return 
	   is_solid(x,  y)
	or is_solid(x+w,y)
	or is_solid(x,  y+h)
	or is_solid(x+w,y+h) 
end

function collide(o,bounce1)
	local x,y = o.x,o.y
	local dx,dy = o.dx,o.dy
	local w,h = o.bw,o.bh
	local ox,oy = x+o.bx,y+o.by
	local bounce = bounce1
	
	--collisions
	local we,he = w-1, h-1
	local coll_x = collision( 
	ox+dx, oy,    we, he)
	local coll_y = collision(
	ox,    oy+dy, we, he)
	local coll_xy = collision(
	ox+dx, oy+dy, we, he)
	
	if coll_x then
		o.dx *= -bounce
	end
	
	if coll_y then
		o.dy *= -bounce
	end
	
	if coll_xy and 
	not coll_x and not coll_y then
		--prevent stuck in corners 
		o.dx *= -bounce
		o.dy *= -bounce
	end
end
-->8
--map 
test = 0
wagon_n = 0

function gen_train()
	--gen talbe of all wagon nums
	nums = {}
	for i=10,29 do
		add(nums,i)
	end
	
	--gen train
	train = {[0]=9}
	
	for i=0,tl do
		local w = i*wl
		for j=0,2 do
			
			local n = 10+flr(rnd(21))
			if(#nums>0)n=nums[flr(rnd(#nums))+1]
			train[w+j]=n
			del(nums,n)
			
		end
		train[w+3] = 8
	end
	
	train[0]=9
	
	for j=0,2 do
		train[tl*4+j]=30
	end
	train[(tl+1)*4 - 1]=31
end

function clone_room(a,b)
	local ax = (a%8)*16
	local ay = (a\8)*16
	room_all= {}
	for j = 0,15 do
		for i = 0,15 do
			local t=mget(ax+i,ay+j)
			mset(b*16+i,j,t)
		end
	end
end

function update_door()
	local wl = wl
	--unlock next wagon
	if #enemies <= 0 and 
	not enemiescleared then
		
		sfx(37)
		sfx(42)
		
		local x=(wl-1)*16
		local g=rnd_gun()
		
		make_drop(x*8+60,56,g.spr,"gun",
		copy(g))
		enemiescleared=true
		
		for i=1,5 do
			make_ptc(
			  x*8 + rrnd(8),
			  56  + rrnd(8),
			8+rnd(8),rnd{9,10})
		end
		
		if wagon_n < tl then
			mset(x,6,40)
			mset(x,7,39)
		else
			init_boss_room(x)
		end
		
		shake += 5
		
	end 
end

function init_boss_room(x)
	mset(x,4,40)
	for i=5,9 do
		mset(x,i,39)
	end
	mset(x,10,12)
end

function begin_boss()
	local x = (wl-1)*16
	spawn_enemy(x*8+8*12,
	56,enemy.boss)
	
	mset(x,i,6)
	for i=5,9 do
		mset(x,i,6)
		burst_ptc(x*8,i*8,10)
	end
	mset(x,i,6)
end

function update_room()
	for i=0,3 do
		local w=wagon_n*wl + i
		
		printh("train["..tostr(w).."]:"..tostr(train[w]))
		clone_room(train[w],i)
	end
	printh("---")
end

function draw_map()
	-- wall palette
	if(pal_n>#trainpal) pal_n=1
	pal(8,trainpal[pal_n][1])
	pal(14,trainpal[pal_n][2])
	
	draw_ghost_connector()
	map()
	draw_random()
	
	palt()
	pal()
end

function break_crate(x,y)
	spawn_loot(x\8*8,y\8*8)
end



function swichtile(x,y)
 local t = mget((x\8),(y\8))
 mset((x\8),(y\8),t+1)
end

function parcourmap()

 local x1=0
 if(wagon_n==0)x1=16
 	for x=x1,16*(wl-1) do
  	for y=2,12 do
   	if x>3 or p.y-1000>y*8 or p.y+1000<y*8 then
    	if mget(x,y)==109 then
   	  mset(x,y,39)
   	  if rnd(4)>3 then
   	  spawn_enemy(x * 8,y * 8,enemy.explosive_barrel)
   	  end
    	elseif fget(mget(x,y),2) and ceil(rnd(max(3,diffi-(wagon_n*1.65))))==1 then
      sapwnrndenemy(x,y)
    	end
   	end
  	end
  end
 end


function sapwnrndenemy(x,y)
	--spawn explosive_barrel 
      if ceil(rnd(25))==10 then
      spawn_enemy(x * 8,y * 8,enemy.explosive_barrel)
     --spawn juggernaut
     elseif ceil(rnd(32))>31-(wagon_n*0.7) and wagon_n>1 then
      spawn_enemy(x * 8,y * 8,enemy.juggernaut)
     
     --spawn warm
     elseif ceil(rnd(30))>28.5-(wagon_n*0.5) and wagon_n>0 then
      for i=0,ceil(rnd(wagon_n/2))+8 do
      spawn_enemy(x * 8,y * 8,enemy.warm)
      end
     
     --spawn tourelle 

      elseif ceil(rnd(30))>32.5-(wagon_n*1) and wagon_n>2 then

      spawn_enemy(x * 8,y * 8,enemy.tourelle)
     --spawn hedgehog 
     else 
      if ceil(rnd(23))>27-(wagon_n*2.5) then
       spawn_enemy(x * 8,y * 8,enemy.hedgehogbuff)
      else spawn_enemy(x * 8,y * 8,enemy.hedgehog)
      
     end
     end
end
-->8
--enemies
function make_enemy(x,y,spr,
spd,life,agro,chase,seerange,
gunt)
	return {
		x=x, y=y,
		angle=0,
		
		pangle=0,
		
		dx=0,dy=0,
		spd=spd,
		agro=agro,
		
		bx=1,by=1,
		bw=6,bh=6,
		
		hx=0,hy=0,
		hw=8,
		
		chase=chase,
		seerange=seerange,
		spr=spr,
		life=life,
		
		gun=gunt,
		cd=30,
		timer = 0,
		a=0,
	}
end

function init_enemies()
	enemy= {
	
	hedgehog=make_enemy(
--x,y,sprite,speed,life,shootrange,  
	 x,y,108   ,1    ,5   ,7.75   ,
--chase,seerange
	 false,1,
	 guns.gunslime),
	 
	hedgehogbuff=make_enemy(
--x,y,sprite,speed,life,shootrange,  
	 x,y,92   ,1    ,10   ,7.75   ,
--chase,seerange
	 false,1,
	 guns.gunslimebuff),
	 
	 
  juggernaut=make_enemy(
--x,y,sprite,speed,life,shootrange,  
	 x,y,94    ,1.5  ,30  ,3   ,  
--chase,seerange
  true,8, 
	 guns.shotgunmechant),
	 
	 warm=make_enemy(
--x,y,sprite,speed,life,shootrange,  
	 x,y,126    ,1  ,1  ,0   ,  
--chase,seerange
  true,6.5, 
	 guns.null),
	 
	 tourelle=make_enemy(
--x,y,sprite,speed,life,shootrange,  
	 x,y,125   ,0    ,15   ,6.5   ,
--chase,seerange
	 false,1,
	 guns.machinegunmechant),
	 
	 explosive_barrel=make_enemy(
--x,y,spr,speed,life,shootrange,  
	 x,y,109  ,0    ,0.1 ,0,
--chase,seerange
	 false,0,
	 guns.explosion),
	 
	boss=make_enemy(
--x,y,spr,speed,life,shootrange,  
	 x,y,1  ,3    ,300 ,32,
--chase,seerange
	 true,32,

	 guns.boss_360gun),
}

local b=enemy.boss
b.bw = 15
b.bh = 15
b.hw = 16
b.guns = {guns.boss_targetgun,
guns.boss_360gun,
guns.boss_enemygun}
b.phase = 0
b.phasetimer = 0
b.pause=0

end

function spawn_enemy(x,y,name)
	local a=copy(name)
	a.x = x
	a.y = y
	a.gun = copy(a.gun)
	local r = rnd(60)
	if(a.spr==1)r = 0
	if (a.spr != 125)a.gun.cooldown += r
	
 if (a.spr == 126)a.spd = 0.8+rnd(0.4)
 if a.x<175 then
  a.gun.timer += 90
  a.timer = 90
 end
 
	add(enemies,a)
end

function update_enemy(e)
	for i in all(enemies) do
		if loaded(i) then
			mouvrnd = true
			
			i.gun.timer = max(i.gun.timer-1--/#enemies
			,0)
			
			if i.gun.timer<=0 and 
			(canshoot(i,p) or i.spr==1)then
				
				i.gun:fire(i.x+4,i.y+4,i.a)
			end
			if mouvrnd then
				changedirection(i)
			end
			collide(i,0.1)
			
			i.pangle=atan2(p.x-i.x,p.y-i.y)
			if not (i.spr== 109) then
				i.flip=isleft(i.pangle)
			end
			
			if(i.spr==1) update_boss(i)
			
			i.x += i.dx
			i.y += i.dy
		end
	end
end

function draw_enemy(e)
	local w = 1
	if(e.spr==1)w=2
	
	local x=flr(e.x)+
	cos(e.pangle)*6*w
	local y=flr(e.y)+
	sin(e.pangle)*3*w
	
	spr(e.gun.spr,x,y,1,1, e.flip)
	
	spr(e.spr,e.x,e.y,w,w, e.flip)
	
	--boss health bar
	if e.spr==1 then
		rectfill(camx+1,120,
		camx+126,126,4)
		local l=126*(e.life/300)
		rectfill(camx+2,121,camx+2+l,
		125,9)
		
		local s="🐱"..ceil(e.life).."/".."300"
		print(s, camx+3,121,7)
	end
	
	--print(e.life, e.x,e.y-8,7)
	--circ(e.x+4,e.y+4,e.r,12)
	--print(e.gun.timer,e.x,e.y)
	--print(abs(e.dy)+abs(e.dx),e.x,e.y+6)
end

function update_boss(i)
	i.phasetimer -= 1
	i.pause -= 1
	
	if i.phasetimer<0 then
		i.phasetimer=600+rnd(600)
		i.phase+=1
		i.pause = 200
	end
	if(i.phase>3)i.phase=1
	
	i.gun=i.guns[i.phase]
	if(i.pause>0) i.gun = guns.null
end

function changedirection(i)
	i.timer-=1
	if i.timer < 1 then
	 i.angle += rrnd(0.25)
	 i.timer=i.cd
	 i.dx=cos(i.angle)/8*i.spd 
	 i.dy=sin(i.angle)/8*i.spd
	end
end

function canshoot(e,pl)
	local angle = atan2(pl.x-e.x,
	pl.y-e.y)
	e.a=angle
	local x = cos(angle)
	local y = sin(angle) 
	local dist = dist(e,pl)
	
	if (abs(dist)<e.agro and abs(pl.x-e.x)<128) or e==p then
		return cansee(e,angle,x,y,dist)
	elseif abs(dist)<e.seerange and abs(dist)>e.agro and e.chase and cansee(e,angle,x,y,dist) then
  o= e.dx+e.dy
   e.dx=x*(e.spd*2)/max(dist,4)
   e.dy=y*(e.spd*2)/max(dist,4)
   
    mouvrnd = false
   
  
 end	
end

function dist(e,p)
	return sqrt(abs(p.y-e.y)^2+abs(p.x-e.x)^2)/8
end
 
function cansee(e,angle,x,y,dist)	 
 for i =1,dist do
	add(checker,{x=e.x+x*i*8,y=e.y+y*i*8})  
	 if is_solid(checker[#checker].x+4,checker[#checker].y+4) then
	 	delchecker()
	 	if (e!=p) e.gun.timer = e.gun.cooldown/2
	 return false 
	 end
	end
	delchecker() 
	return true 
end


function delchecker()
	checker = {}
end

function loaded(i)
	return abs(camx+64-i.x)<71--71
end
-->8
--particles & bg
function init_ptc()
	particles={}
	grass = {}
	for i=0,20 do 
	add(grass,{x=flr(rnd(16))*8, y=flr(rnd(16))*8, spr=56})
	end
	for i=0,5 do 
	add(grass,{x=flr(rnd(16))*8, y=rnd{0,112}, spr=56})
	end
	for i=0,3 do
	for v=4,14 do
	add(grass,{x=32*i,y=v*8,spr=24})
	end
	end
	weelflip = true
	weelframe = 5
	weelcount = weelframe
end

function make_ptc(x,y,r,col,fric,dx,dy,txt)
	fric=fric or rnd(.1)+.85
	dx=dx or 0
	dy=dy or 0
	add(particles, {
		x=x,  y=y,
		dx=dx,dy=dy,
		fric=fric,
		
		txt=txt,
		
		r=r, col=col,
		destroy=false,
	})
end

function update_ptc(ptc)
	ptc.x += ptc.dx
	ptc.y += ptc.dy
	
	ptc.dx *= ptc.fric
	ptc.dy *= ptc.fric
	
	ptc.r *= ptc.fric
	
	if(ptc.r<=1)ptc.destroy=true
end

function draw_ptc(ptc)
	--kinda bodgey but whatever
	if ptc.txt==nil then
		circfill(ptc.x,ptc.y,ptc.r,ptc.col)
	else
		print(ptc.txt,ptc.x,ptc.y,ptc.col)
	end
end

function burst_ptc(x,y,col)
	for i=1,5 do
		make_ptc(
		   x+rrnd(8),
		   y+rrnd(8),
		   rnd(5)+5,col,
		   0.9+rnd(0.07))
	end 
end

function grasstile()
	for i in all(grass)do
	 i.x = i.x
		i.x-=2.5
		if (i.x<-8)i.x = 128
	end
end

function drawgrass()
	for elt in all(grass)do
		spr(elt.spr,camx+elt.x,elt.y)
	end
end

function draw_weel()
 weelcount -=1
 if (weelcount<1) weelflip = not weelflip weelcount=weelframe
 for n=0,5 do
	for i=0,2 do
		spr(42,8+n*64+i*16,14*8, 2,2,weelflip)--, flip_x ,flip_y)
	end
	end
end

----


-->8
--menus
function init_menus()
	menus = {}
	menus.main = make_main_menu()
	menus.death = make_death_menu(false)
	menus.bossdeath = make_boss_death()
	menus.win = make_death_menu(true)
end

--[[
function draw_bar(t,x,y,w,w2,c,c2)
	rectfill(x,y,
	x+w+2,y+5,c2)
	local l=w*w2
	rectfill(x+1,y+1,
	x+1+l,y+5,c)
	
	print(t, x+1,y+1,7)
end--]]

function make_boss_death()
	local m={
	timer=400,
	
	update=function(m)
		for e in all(enemies)do del(e,enemies) end
		music(-1,0)
		if(m.timer==400)sfx(45)--sfx(46)
		
		m.timer -= 3
		
		if m.timer<0 then
			menu = "game"
			win=true
			shake += 40
			sfx(48)
			
			for i=1,18 do
				make_ptc(camx+rnd(128),
				rnd(128),50+rnd(20),
				rnd{8,9,10},.95)
				
--(x,y,r,col,fric,dx,dy,txt)
			end
		end
		
		mx,my=-10,-10
	end,
	
	draw=function(m)
		
		local x,y = boss_pos[1]+8,boss_pos[2]+8
		local flp = boss_pos[3]
		local t = m.timer
		circfill(x,y,t    ,8)
		circfill(x,y,t*.75,9)
		circfill(x,y,t*.5 ,10)
		circfill(x,y,t*.25,7)
		spr(1,x-8,y-8,2,2,flp)
	end
	}
	return m
end
------

function make_main_menu()
	--this code could be better
	local m = {
	  update=update_main_menu,
	  draw=draw_main_menu,
	  
	  sel=0,
	  done=false,
	  ui_oy=0,
	  ui_dy=0,
	  
	  has_active=false,
	}
	m.buttons={}
	
	local names=split("pigeon,duck,sparrow,parrot,toucan,flamingo,eagle,seagull,ostrich,penguin,jay,chicken")
	local x=4
	local y=105
	for i=1,12 do
		add(m.buttons,{
		  n=i,
		  spr=i+79,
		  bird=i+111,
		  
		  x=i*10-6,
		  y=105,
		  w=9,
		  h=17,
		  col=1,
		  sh=2,
		  
		  name=names[i],
		  active=false,
		})
	end
	
	-------
	function make_btn(args)
		n,sp,bird,x,y,w,h,name=unpack(split(args))
		return {
		  n=n,
		  spr=sp,
		  bird=bird,
		  
		  x=x,y=y,
		  w=w,h=h,
		  
		  oy=0,
		  col=1,
		  sh=1,
		  
		  name=name,
		  active=false,
		}
	end
	-------
	m.buttons[0]=make_btn("0,124,39,114,91,9,9,random")
		
	m.buttons[13]=make_btn("13,111,39,2,2,9,9,random")
	
	return m
end

function update_main_menu(m)
	local selection=1000
	
	if not m.done then
		--update buttons
		for k=0,#m.buttons do
			local i = m.buttons[k]
			
			--on hover
			if touches_rect(mx,my,i.x,i.y,
			i.x+i.w-1, i.y+i.h-1) then
				i.col = 7
				i.oy = 2
				
				if(not i.active) sfx(43)
				m.has_active=true
				m.sel = i.n
				i.active=true
				
				-- on click
				if lmb then 
					selection = i.n
				end
			
			else
				
				i.active = false
				i.col = 1
				i.oy = 0
				
			end--if
		end--for
		
		--buttons
		for n=0,1 do
		if(btnp()>0)sfx(43)
		if(btnp(⬅️,n))m.sel -= 1
		if(btnp(➡️,n))m.sel += 1
		if(btnp(⬆️,n))m.sel=(m.sel==0)and 13 or 0
		if(btnp(⬇️,n))m.sel=1
		end
		if(btn(❎)or btn(🅾️))selection=m.sel
		
		m.sel%=14
		local b=m.buttons[m.sel]
		b.active = true
		m.has_active=true
		b.oy,b.col = 2,7
		
		
		-- run selection
		if selection<=12 then
			m.done = true
		elseif selection==13 then
			hardmodetimer+=1
		else
			hardmodetimer=0
		end
	else
		--animation 
		m.ui_dy += .1
		m.ui_oy += m.ui_dy
		
		if m.ui_dy > 5 then
			birdchoice = m.sel
		 menu = "game"
			begin_game()
			return
		end 
	end
end

function draw_main_menu(m)
	local oy = m.ui_oy
	
	palt(0,false)
	
	draw_logo(44,5-oy)
	
	--player selection
	rectfill(112,89+oy,125,110+oy,12)
	rectfill(2,103+oy,125,124+oy,1)
	for k=0,#m.buttons do
		i = m.buttons[k]
		
		oy = abs(oy)
		if(k == 13)oy = -oy
		
		rectfill(i.x, 
		i.y-i.oy + oy, 
		i.x+i.w, 
		i.y+i.h-i.oy + oy, 
		i.col)
		spr(i.spr, 
		i.x+1, 
		i.y+1-i.oy + oy,
		1,i.sh)
		
		if i.n==13and i.active then
			oprint("a game by:",2,13, 14)
			oprint([[yOLWOOCLE
gOUSPOURD
nOTGOYOME
sIMON t.]],2,13)
			oprint([[code,art
code
code
music]],45,13, 13)
		end
	end
	oy=abs(oy)
	
	-- buttons
	local sel=m.buttons[m.sel]
	rectfill(
	2,93+oy,
	2+#sel.name*8, 102+oy,1)
	wide(sel.name,4,95+oy,7)
	palt()
	
	-- encaged bird
	palt(1,true)
	spr(sel.bird,6*8,7*8)
	spr(32,6*8,7*8)
	palt()
end

function draw_logo(x,y)
	--"birds"
	oxxl("birds",x,y,10)
	oxxl("guns",x+4,y+15, 6)
	
	--"with"
	oprint("with",x+11,y+10)
	
	oprint("with",x+11,y+9)
end

function oxxl(t,x,y,col)
	--credit to freds72
	for ix=-2,2 do
		for iy=-2,4 do
			if abs(ix)==2 
			or abs(iy)>=2 then
				print("\^p"..t,x+ix,y+iy,1)
			end
		end
	end
	
	col=col or 7 
	for ix=-1,1 do
		for iy=-1,1 do
			print("\^p"..t,
			x+ix,y+iy,col)
		end
	end
end

function wide(t,x,y,col,pre)
	--credit to yolwoocle uwu
	t1= "                ! #$%&'()  ,-./[12345[7[9:;<=>?([[c[efc[ij[l[[([([st[[[&yz[\\]'_`[[c[efc[ij[l[[([([st[[[&yz{|}~"
	t2="                !\"=$  '()*+,-./0123]5678]:;<=>?@abcdefghijklmnopqrstuvwx]z[\\]^_`abcdefghijklmnopqrstuvwx]z{|} "
	n1,n2="",""
	pre=pre or ""
	
	for i=1,#t do
		local char = sub(t,i,i)
		local c=ord(char)-16
		n1..=sub(t1,c,c).." "
		n2..=sub(t2,c,c).." "
	end
	
	if(col!=nil)color(col)
	print(pre..n1,x,y)
	print(pre..n2,x+1,y)
end


-->8
--drops
function make_drop(x,y,spr,type,q)
	add(drops,{
	 x=x, y=y,
	 bx=8,dy=8,
	 
	 spr=spr,
	 type=type,
	 
	 q=q,
	 touched=false,
	 cooldown=0,
	 
	 destroy=false,
	})
end

function update_drops()
	for d in all(drops) do
		d.cooldown=max(0,d.cooldown-1)
		
		local touches = touches_rect(
		p.x+4,p.y+4,
		d.x,d.y,d.x+8,d.y+8)
		
		if(not touches)d.touched=false
		if touches then
			
			local col=7
			local txt=""
			local do_ptc = false
			
			if d.type=="ammo" then
				d.destroy = true
				local q = flr(d.q*p.gun.maxammo)
				p.gun.ammo += q
				
				do_ptc=true
				col=9
				txt="+"..q.." ammo"
				
				sfx(38)
				
			elseif d.type=="health"then
				d.destroy = true
				p.life += d.q
				
				do_ptc=true
				col=8
				txt="+"..d.q.." health"
				
				sfx(38)
				
			elseif d.type=="gun" 
			and not d.touched
			and d.cooldown<=0 then
				d.touched = true
				d.cooldown = 60
				
				do_ptc=true
				col=6
				txt=d.q.name
				
				p.gunls[p.gunn],d.q=d.q,p.gunls[p.gunn]
				update_gun(p)
				d.spr = d.q.spr
				
				sfx(36)
			end
			
			if do_ptc then
				for i=1,5 do
					make_ptc(
						d.x+rrnd(8),
						d.y+rrnd(8),
						rnd(5)+5,col,
						0.9+rnd(0.07))
				end 
			end
			
			make_ptc(
				d.x+4-(#txt*2),
				d.y+4,
				rnd(5)+5,7,
				.98,0,-0.3,txt
			)
		end
		
		if(d.destroy)del(drops,d)
	end
end

function draw_drops()
	for d in all(drops)do
		spr(d.spr,d.x,d.y)
	end
end

function spawn_loot(x,y)
	local r = rnd(1)
	
	if r < .015 then
		local g = rnd_gun()
		make_drop(x,y,g.spr,"gun",
		copy(g))
	elseif r < .045 then
		make_drop(x,y,79,"ammo",1/4)
		
	elseif r < .075 and degaplus == 0 then
		make_drop(x,y,78,"health",2)
	
	elseif r < .017 and degaplus == 1 then
		make_drop(x,y,78,"health",1)
	end
end
-->8
--death menu
function make_death_menu(iswin)
	local m = {
	  update=update_death_menu,
	  draw=draw_death_menu,
	  
	  circt=1,
	  timer=0,
	  showtext=false,
	  
	  iswin=iswin,
	  nstats=0,
	}
	
	local t,t2="retry","change bird"
	if(iswin)t="play again","title screen"
	
	m.buttons = {}
	m.buttons[1]={
		n=1,
		t=t,
		x=0,y=0, oy=0,
		active=false
	}
	m.buttons[2]={
		n=2,
		t=t2,
		x=0,y=0, oy=0,
		active=false
	}
	
	return m
end

function update_death_menu(m)
	m.circt=min(m.circt*1.05,600)
	
	--circle timer
	if m.circt>=600 then
		if(m.timer==0) music(23)
		
		if(m.timer==220) music(24)
		if m.timer%60==59 
		
		--"ploop" sfx on every stat
		and m.nstats<3 then
			sfx(41) 
			m.nstats+=1
		end
		
		m.showtext=true
		m.timer += 1
	end
	
	-- buttons
	local o = 0
	if keyboard and m.timer>1 then
		if(btn(❎))o=1
		if(btn(🅾️))o=2
	end
	
	for i=1,#m.buttons do
		local b = m.buttons[i]
		local t=m.timer / 100
		
		local ox = #b.t*2
		b.x = camx + 64 - ox 
		+ cos(t+i/10)*1.5
		b.y = 1/t + 80 + i*15 
		+ sin(t+i/10)*1.5
		
		if touches_rect(mx,my,
		b.x-4,b.y-4,
		b.x+#b.t*4+3, b.y+9) then
			
			if(not b.active)sfx(43)
			b.active = true
			b.oy = 3
			if lmb then
				o = b.n
			end
			
			m.sel = i
			
		else
			b.active = false
			
		end
	end
	
	if(o==1)run(tostr(birdchoice))
	if(o==2)run("-")
end

function draw_death_menu(m)
	--circles animation
	palt(0,false)
	palt(1,true)
	local col,c2,c3,c4=0,1,2,9
	local txtcol = 7
	if m.iswin then
		col,c2,c3,c4=15,9,8,2
		txtcol = 10
		if(t()%2<1)txtcol = 9
	end
	
	local x,y = p.x+4,p.y+4
	local c = m.circt
	circfill(x,y,c    ,c4)
	circfill(x,y,c*.75,c3)
	circfill(x,y,c*.5 ,c2)
	--spr(p.spr,p.x,p.y)
	circfill(x,y,c*.25,col)

	palt()
	
	--text & buttons
	local t=m.timer/100
	local txt=m.iswin and 
	   "congrats!" or "game over"
	
	oxxl(txt,
	     camx+30+cos(t)*3,
	     1/t +20+sin(t)*3, txtcol)
	     
	for b in all(m.buttons) do
		local a = ""
		if keyboard then
			a = "🅾️"
			if(b.n == 1) a = "❎"
		end
		
		if b.active then
			oprint(a..b.t,b.x,b.y-b.oy,1,7)
		else
			oprint(a..b.t,b.x,b.y,14,1)
		end
	end
	
	--stats
	local i=0
	for k,v in pairs(stats)do
		if i<m.nstats then
			text_y = i*8+1/t+40+sin(t+.3)*3
			oprint(k,camx+35,text_y)
			oprint(v,camx+80,text_y, 13)
		end
		i+=1
	end
	
	for i=0,6 do
		local txt="▒"
		if(i<=wagon_n)txt="█"
		if not m.iswin then
			oprint(txt, camx+37+i*8,
			1/t+70+sin(t)*3, 7)
		end
	end
	
	--hard mode prompt
	if m.iswin then
		local txt="hold the 'i' button\non the title screen\nto unlock hard mode\n"
		if(degaplus!=0)txt="bro what !!!\nthis mode was not \nsupposed to be possible !"
		oprint(txt
		,camx+25,1/t+70+sin(t)*2, 13)
	end
end

__gfx__
00000000000d500000005d00777777770000000011111111777777766666666d44444444444444444444444444444444eeeeeeeeee1111ee11eeee11ee1111ee
000000000005750000057500777777776d000066111111117d6116dd65dddd5524444422444444444444444424411422eeeeeeeeee1111ee1eeeeee1ee1111ee
00700700000577794467750077777777ddd006dd111111117611116d6dddddd522244444222222224444444422211444ee1111eeee1111eeee1111eeee1111ee
000770000700576999976d009999999907776760777777777611116d6dddddd5444442242222222244444444444d1224ee1111eeee1111eeee1111eeee1111ee
0007700077604999999999007777777706666660999999997661166d6dddddd5444444444444444444444444444d4444ee1111eeee1111eeee1111eeee1111ee
007007007770455999995900111111116dd00dd6777777777661166d6dddddd5422244444444444444444444422d4444ee1111eeee1111eeee1111eeee1111ee
00000000679074855955940011111111dd0000dd777777777d6666dd65dddd5544444444222222222222222244464444ee1111eeee1111eeee1111ee8eeeeee8
0000000009906788998840001111111100000000777777776dddddddd555555522222222222222222222222222262222ee1111eeee1111eeee1111ee88eeee88
77777777049907777117600077777776222222224444444112211111555555554424424414444444111bb111444644446666666614444441eeeeeeee888e8ee8
77777777000ddd67777770007000770d222222224444444121124111555555554424424414444444111331b1244644226666666614444441eeeeeeee888e8ee8
7777777700dd66666d8867007007700d4244444444444441114444210000000042244444144444441b13313122264444666666661444444111111111888e8ee8
777777770055ddddd28e86007077000d2444444444444441122444210000000042444442144444441313333144464114666666661446444111111111888e8ee8
7777777705557777d28886507770700d42444467444444412422422100000000424244421444444413333411444d1441666666661443444111111111888e8ee8
7777777756d556666d2867557707000d2444444444444421244222110000000044424442124444441143341142ddd4417777777714b3344111111111188e8ee1
777777775dd5566666677d557070000d2244444422222221124244210000000044444444122222221144441111444414dddddddd14bb3441eeeeeeee118e8e11
7777777705550000000055506dddddd62444444222222211122122210000000044444244112222221122221122111122dddddddd14b33441eeeeeeee11111111
44444441ffffffff111111111111111111111111111111114444444111111111eeeeeeeeeeeeeeee1111122222211111661111111444444111111166111dd111
44444441f444444ffffff1111111111114444444444444114444444111111111eeeeeeeeeeeeeeee11112222222221116661111114444441111116661111d111
44444441f4f4f4fff44ff111111111111444444444444411444444411111111188888888eeeeeeee112221d1d1d222116661111114444441111116661111d111
22222221f4f4f4fff4ffffffffffffff1444444444444411222222211111111188888888ecc777ce1122d1d1d11d22116661111114444441111116661111d111
d16161d1fffffffff4f4ff444444444f1444444444444411d16861d11111111188888888ec777cce022d1d1dd1d11220556111111444444111111655111dd111
6161d1d144444444fff4ff4f4f4f4f4f14444444444444116868d9d11111111188888888e777ddde022011d22d1dd220665666111444444111666566111d1111
61d1d1d142424244ff44ff4f4f4f4f4f122222222222221161d9d1d11111111188888888eeeeeeee022ddd2222d00220666666611444444116666666111d1111
2222222144444444ffffff4f4f4f4f4f1d1d1d1d1d1616112222222111111111888888888888888802200d2222ddd220666666611444444116666666111d1111
666666611111111144444fffffffffff1d1d1d16161717114444444111111111ffffffff00000000622dd6d22d66622666666661144444411666666611111111
66666661ffffffff42224444444444441d161617b71717114444444111411441ffffffff0000000062266d6dd6d6d226556555611444444116555655111dd111
6666666177777777422fffff222222241617c79717b616114444444114444144ff3fffff000000006622d66d6d6d226655566651144444411566655511dddd11
ddddddd177777777444f4f4f2424242417c717a6b6b6bd112222222112424214ff3f3fff0000000066222d6d6d62226655566661144444411666655511dddd11
51d1d151ffffffff111fffff4444444417c6c626b6bd2d11d16a61d122222411ff3f3fff000000006662222222226666555666611222222116666555115dd511
d1d1515144444444111444441111111116c6262d2d2d2d11d9da6a6124424441ffffffff00000000777772222227777755566661122222211666655511155111
d1515151411141441114222411111111162d2d2d2d2d2d11d1d9d16112422421ffffffff00000000dddddddddddddddd55555551111661111555555511111111
ddddddd144444444111444441111111112222222222222112222222111211211ffffffff00000000dddddddddddddddd55555551111661111555555511111111
000000000000000000000000000000000066d6000000000000000000006d6d000008e00000000000000000000000000009aaaa90007777000111110002222200
000000000000000000000000000000000066d60000000000000000000d000060008882800055500000000000000000009a9009a9077777701678761029a9a920
06000000060000000000000000ddd000000dd000d00dd000ddd000dd6000000dee88288e444d5ddd005000800d66aa98a900009a700067771688861029242920
0066660000d666660446d6d6446664000d66666664466666664666ddd44550068888288edd465666d666788800a911a8a000000a000006771678761029a9a920
046ddd00444504404440500044ddd4dd4d44400064466666044050004444500d8888288e554d5ddddddd878809a655a0a000000a000006771766671024949420
4445000044000000440050004040500044050000d005050004408800440000068888288edd505000044000809a59aa90a900009a7000677715ddd51024949420
44000000000000000000000000405000000000000005000000009900060000d0008882805540000004000000a50000009a9009a9077777700111110002222200
0000000000000000000000000000050000000000000000000000880000d6d6000000000004400000000000000000000009aaaa90007777000000000000000000
11111111222222221111111111111111222222221111111155555555444444445555555522222222444444445555552806000005000000000667677000000000
1111111122222222111111111111111122222222112ee21155555555444444445555555522222222cd4444445555288804600054009aa90066668867008ee800
115dd511222222221111111111288821222222221eeeee215567765544446776555eee5522222210cccd4444555288880599999609a77a905c68788608e77e80
17dddd51225335221111111112888882222222102eeeffe157777775444677775efffff2222220007cccd44455d888e7091999910a7777a0666888860e7777e0
557d22d12333339a111111111822888892222100ee1ef0e1d677766d447777dd2fffff2222210000c77ccd4455667777099177190a7777a0556d88d60e7777e0
157d882553333aaa1111111112678866aa92009aee1ee7e1ddd77ddd44777d0622ff22052100000070dc1114566777770077747009a77a900666dd6008e77e80
11d8188d33339aa21111111116772666aaaaa0cce2127f7190077009446776065028e00549994000700c0111510aa77700966690009aa90005d005d0008ee800
11d8188d3003a44211242111177066119aaa0c1cee11fff17909a097444977665f8888fe21109990777cc044d0a22a7000440440000000000d500d5000000000
15dd88dd30139444994442111760566699aa0c1cee2100f1777997774499aa77f828828f2222a0001777764467a211a0004222000000049077700000cccccccc
53ddddd553333994a941422116000666e9909acc2ee000116799a977499a7777f2e22e2f2222a9101177764467911997042777200000400a00000000ccc77ccc
33d3ddd1253333391a742222161006688e00aaa71ee001110719a16749a47777fffeeffe22227aa9c1777444678999774271f1700028720077700000cccccccc
3233d3512233333216777222111288882807777712ee21110600a017444477775feeeff52222777acc1764447788887724f1f1f00288872077700000cc777ccc
2223333122533372116777611128888225777777e11ee21101009006444d77775feeee2522267777cd71444477e88877427fff440818881077700000ccc77ccc
2222232226557744411e1e111928882827777777ee12ee1100000010d6666777552eee2522267777d776444477788e77247777700881818000000000ccc77ccc
62626226d66d554424e11e11aa8889a805777777ee21ee211000001066666677555eeee2222777777774444477777777024777000288882000000000cc7777cc
6666626666666dd51224e421cca88caa00057777eee12ee110010000666666665552eeee222777777764444477777777002202200028820000000000cccccccc
11115d1111113b111111111111288e1110001111111eeee111167711111167111111eef111000011dccccd111111888111d66dd5003bbb000000000001100110
1115ddd1111333b111111111127078710779aa01111e7075116079a1111677711111f0ee1000000111c777c1117888811d6777dd03bbbbb00000000017711771
11152dd111130aa111111111127770770c799990e111ee151167799a1116079951111ff100a70999111d707d167779aa6777776d331bb3100050506017100171
5113dd7711133a991111441118870066077888892eeeee11100005191166777115544ff10000aa111111d771677077887772777d33b1b130000eef0001000010
56623331dddd334111414691aa88881100770118122ee1111000005166666671545544e100077611cd7ccc71677777787728e776333bbbb0000fff0001000010
d566222116664441111461111baa821100000111117171110007000156666771feff4411000776111cd7c761667777616778777622444f400050506017100171
1d666611116444111111111111cc2111100011111171711117700011157776111e11e11100776611117776111667761117777611022222000000000017711771
118181111191911111111111115151111c1c11111121211111911911111e1e111e11e111007550011115151111919111117611110244f4000000000001100110
93939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393
93939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393
8282928282829282828292828282928282829282828292828282928282829282c0829282c0829282c0829282c082c08282c082929292929292929282c0829282
82c09282c09282c09282c09282c092828282c09292c0c0c09292c08282c0928282829282828292828282928282829282c082928282829282828292828282c082
02137272721302727212a172727272a172122290909090909090905272121212d0c1a0a0d0a0a0c1d0a0a0c1d0c3d02232d060c1c1c1c1c1c1c1c160d0121272
13d0c3c1d0c3c1d0c3c1d0c3c1d0121272e2d01212d0d0d01212d04040d0126363425262727272721272727272727272d0a0a0c1a0a0a0a0a0a0a0c1a0a0d072
1272727272727272727272727272727272132390909090909090905372020202d0727213d0727272d0727213d061d02333d0f28183838383838381f2d0127272
13d07272d07272d07272d07372d0127272e3d072728282827272d0828282127262435363425262631312727272121372d0d37272d37272d37272d37272d3d072
727272d2d2727272e0727272d2d2727272727290909090909090907272720202d072f3d3d072f3d3d0d3f372d072d07272d0f28183838383838381f2d0127272
72d04182824182824182824182d012727272d07272a112a17272d072727272727272425243537272d613425213127272d022327272727272727272727213d072
7272d3d1d2d37272d07272d3d2d1d37272727272727272d672d6727272727202d0727272d0824182d0727272d073d07272d0f28183838383838381f2d0127272
72d07272727272727272d672d6d0727272e2d072727272727272d07272c0c27272724353727272727272435312727272d023337272727272727272d60212d072
727272d2d1727272d0727272d2d27272727272727272727272727272727272728282418282a172a1828241828241827272d090909090909090909090d0727272
72d07272727272727272727272d0727272e3d082929292928241d07272d0c3727272121372727272727263727272727282828282828272727282828282828272
7272d3d2d2d37272f07272d3d1d2d372727272c1a0a0a0a0a0a0c172727272727272727272727272727272727272727272d0a1a1727272727272f3d1d0727272
72d0a17272727272727272d6a1d072727272d07272727272d672d07272d07272727213727272727272726272727272727272727272a1727272a1727272727272
7272729151727272f1727272915172727272d3d17272727272a1d272727272727272727272727272d67272727272727272d0a1a1a1727272727272d3d0727272
7282828282828241418282828282727272728272727272727272827272d0c2121342527272425272726362627272727272727272727272727272727272727272
727272727272727272727272d6727272727272d2727272727272d1d37272727272727272c0824182c0824182c041c07272828282828282824141828282727272
72a112121272727272727272127272727272727272c092c07272727272d0c3137243534252435312124252121213727272727272727272727272727272727272
7272727272727272727272727272727272727291a0a0a0a0c1a051727272727272727202d0727291d0727272d012d002727272a1121222327272223272727272
727272727272727272727272727272727272c07272d040d07272c08282d0727272726343537272721343537272121372c082828282c0727272c082828282c072
721372727272c272d172e272d672727272727272d37272d372d372727272727272720312d04272f3d0f3a0f3d012d01272727272727223337272233372727272
7272727272727212727272727272727272e2d07272d040d07272d0a2b2d0c27272727212137272727262637272727272d022320213827272728213034252d072
121213021272c372d372e372727272a11272727272727272727272727272727272721362d0437212d072f312d013d01272727272727272727272727272727272
727272727272121213a172727272727272e3d07272d040d07272d0a3b3d0c37272727212727272727272137272727272d023331372417272724172134353d072
82829282828292828282928282829282828292828282928282829282828292828282928282829282828292828282928282829282828292828282928282829282
82829282828292828282928282829282828282828282928282828282828292828282928282829282828292828282928282829282828292828282928282829282
93939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393
93939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393
c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1
c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1
93939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393
93939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393
8282928282c09282828292828282928282829282828292828282928282829282c08292828282928282829282c082928282829282828292828282928282829282
8282829282828292828282928282829282c0829282c08282828282828282c08282828282828282828282828282828282c082828282829292829292c093939393
a1727272a1d0a1727272d2727213131372e2c272d1c272c1c172e2d172e2c272d0c2727272727272727272e2d0420372c1c1a2b2c1c1c1a2b2c1c1c1a2b2c1c1
7213d0808080b080808080b0808080d072d0425262d01212727272122232d07272021372727213137272121312721213d012727272727272727272829292c093
c2d272d1e2d07272727291a0a0a0c1c172e3c372d3c372727272e3d372e3c372d0c3727272425272727272e3d04372727270a3b370a170a3b370a170a3b37072
c002f0808080b180808080b1808080f072d0435363d012d672d672132333d07272c2d272c2d272c2d272c2d272c2d213d012727272727272727272f2727282c1
c3d372d3e3d07272727272727272727272727272727272727272727272727272d0a1727272435363127272a1d002727272707070707270707070727070707072
d072f18080808080b0b08080808080f172d0727272d07272c07272722232d07272c3d372c3d372c3d372c3d372c3d372d072727272727272727272f27272f331
7272727272d0137213e0c2d172d2e2727272727272a172727272a17272727272828241828282c082828241828272727272f27272f272f27272f272f27272f272
d072a19080808080b1b18080808090a172d0727272d07272d0d672721233d07272727272727272727272727272d672726072727272721272727272f272727231
72c2d1e272d02232a1d0c3d372d3e37272637272d3d2f37272f3d1d372726272727272722232d002020372727272727272f27272f213f27272f202f27272f272
827272729090909090909090909072727282828241d07272d07272721212d072505050505050505050505050505050506072727272721272727272f27272f331
72c3d3e372d0233313d0a172727272727262727272d1f3d672f3d27272726272727272722333d012037272727272727272f27272f213f27272f213f27272f272
727272727272d672d672d672727272727272727272824141828282c082828272010101010101010101010101010101016072727272721272727272f272727231
7272727272f07212d6d0c272d172727272637272d3d2d6f3f3d6d2d372726372c08241828282d08282824182c072727272f27272f212f27272f202f27272f272
c07272c272d172e27272c272d172e2727272727272727272727212d022327272303030303030303030303030303030306072727272721272727272f27272f331
7272727272f1727272d0c372d3727272727272727291a0a0a0a0517272727272d07272720262f0a172727272d072727272f27272f272f27272f272f27272f272
d07272c372d372e37272c372d372e3727272727272727272727272f02333727272727272727272727272727272d672726072727272721272727272f272727231
727272727272727272d0a1727272727272727272727272727272727272727272d00372727272f17272727212d0a1727272c17272c1c1c17272c1c1c17272c172
d07272727272727272727272727272727272727272c0a172727272f17272727272c2d272c2d272c2d272c2d272c2d272d072727272727272727272f27272f331
223272727272727272d0c2d272d1e27272e2c272d1c272727272e2d172e2c272d04202727272727272d60363d0223272e270f3f3e270c2f3f3e270c2f3f370c2
827272727272727272727272727272727272722232d07272727272127272727272c3d372c3d372c3d372c3d372c3d313d012727272727272727272f27272c093
23331312a1727272a1d0c3d372d3e37272e3c372d3c372c1c172e3d372e3c372d043620372727272d6031362d0233372e37013f3e370c3f3f3e370c3f31270c3
72727272c272d172e2c272d172e272727272721233d07272727272727272727272721272727212131372727272727213d012727272727272727272c082828293
82829282828292828282928282829282828292828282928282829282828282828282928282829282828292828282928282829282828292828282928282829282
82828292828282928282829282828292828282928282829282828292828282928282829282828292828282928282829282828282828292928292928293939393
93939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393
93939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393
c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1
c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1
__label__
66606660066006600000666000006660000066600000606000000000000000000000000000000000000000000000000000000000000000000000000000000000
60600600600060600000606000006060000000600000606006600000000000000000000000000000000000000000000000000000000000000000000000000000
66600600600060606660666000006060000066600000666060000000000000000000000000000000000000000000000000000000000000000000000000000000
60000600600060600000606000006060000060000000006060000000000000000000000000000000000000000000000000000000000000000000000000000000
60006660066066000000666000006660060066600600006006600000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
06000660060000006660666066006060000066606660000060006660606066606000066066606660600066600000066066606660666006600000600060006660
60006000006000000060606006006060000000600060000060006000606060606000606060006000600060000000600060606660600060000000600060006060
60006000006000006660606006006660666066606660000060006600060066606000606066006600600066000000600066606060660066600000600060006660
60006000006000006000606006000060000060006000000060006000606060606000606060006000600060000000606060606060600000600000600060006000
06000660060000006660666066600060000066606660000066606660606060606660660060006000666066600000666060606060666066000000666066606000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
66606060666066600000606066606000666000006660066066600000606066606000666000000000000000000000000000000000000000000000000000000000
06006060606060000000606060006000606000006000606060600000606060006000606000000000000000000000000000000000000000000000000000000000
06006660666066000000666066006000666000006600606066000000666066006000666000000000000000000000000000000000000000000000000000000000
06000060600060000000606060006000600000006000606060600000606060006000600000000000000000000000000000000000000000000000000000000000
06006660600066600000606066606660600000006000660060600000606066606660600000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
70000000700007700000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
07000000700070000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00700000700077700000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
07000000700000700000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
70000000777077000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
cc00ccc0ccc0ccc00cc0ccc00cc0ccc0c0c00000000000c000000000000000000000000000000000000000000000000000000000000000000000000000000000
c0c00c00c0c0c000c0000c00c0c0c0c0c0c00c0000000c0000000000000000000000000000000000000000000000000000000000000000000000000000000000
c0c00c00cc00cc00c0000c00c0c0cc00ccc0000000000c0000000000000000000000000000000000000000000000000000000000000000000000000000000000
c0c00c00c0c0c000c0000c00c0c0c0c000c00c0000000c0000000000000000000000000000000000000000000000000000000000000000000000000000000000
ccc0ccc0c0c0ccc00cc00c00cc00c0c0ccc000000000c00000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
66606660606066606660000066606660000066606600066000000000000000000000000000000000000000000000000000000000000000000000000000000000
00600060606060600060000060606060000060606060600000000000000000000000000000000000000000000000000000000000000000000000000000000000
06600060666060606660000066606660000066606060600000000000000000000000000000000000000000000000000000000000000000000000000000000000
00600060006060606000000060006060000060006060606000000000000000000000000000000000000000000000000000000000000000000000000000000000
66600060006066606660060060006660060060006060666000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
66606660666066000660606066606660606006606060660006600000666000006660666000006660660006600000000000000000000000000000000000000000
60600600606060606000606006000600606060006060606060000000006000006060606000006060606060000000000000000000000000000000000000000000
66000600660060606660606006000600666060006060606066606660066000006660666000006660606060000000000000000000000000000000000000000000
60600600606060600060666006000600606060606060606000600000006000006000606000006000606060600000000000000000000000000000000000000000
66606660606066606600666066600600606066600660606066000000666006006000666006006000606066600000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
66006660660006600000666066600000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
60600600606060600000606060600000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
60600600606060600000666066600000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
60600600606060600000600060600000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
66606660606066000600600066600000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
66606660066006606600666066000660000066000000666066600000666066000660000000000000000000000000000000000000000000000000000000000000
60600600600060606060060060606060000006000000606060600000606060606000000000000000000000000000000000000000000000000000000000000000
66600600600060606060060060606060666006000000666066600000666060606000000000000000000000000000000000000000000000000000000000000000
60000600600060606060060060606060000006000000600060600000600060606060000000000000000000000000000000000000000000000000000000000000
60006660066066006660666060606600000066600600600066600600600060606660000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
66606000666066600000666066600000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
60606000606006000000606060600000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
66606000666006000000666066600000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
60006000606006000000600060600000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
60006660606006000600600066600000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
66606060600066606660000066606660000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
60606060600060000060000060606060000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
66006060600066000600000066606660000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
60606060600060006000000060006060000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
60600660666066606660060060006660000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
06606060666000006660666000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
60006060060000006060606000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
66606600060000006660666000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00606060060000006000606000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
66006060666006006000666000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
06606060666000006660666000006660660006600000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
60006060060000006060606000006060606060000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
66606600060000006660666000006660606060000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00606060060000006000606000006000606060600000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
66006060666006006000666006006000606066600000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
66606060666000006660666000006660660006600000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
06006060060000006060606000006060606060000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
06006060060000006660666000006660606060000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
06006060060000006000606000006000606060600000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
06000660060006006000666006006000606066600000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
70000000700007707770770007007070777077707770770007707070777077707070077070707700077000007770000077707770000077707700077070700700
07000000700070707070707070007070707007007070707070007070070007007070700070707070700000000070000070707070000070707070700070700070
00700000700070707770707070000000770007007700707077707070070007007770700070707070777077700770000077707770000077707070700000000070
07000000700070707070707070000000707007007070707000707770070007007070707070707070007000000070000070007070000070007070707000000070
70000000777077007070777007000000777077707070777077007770777007007070777007707070770000007770070070007770070070007070777000000700
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
60000660666066006660660000006660666066606600066060606660666060600660606066000660000066600000666066600000666066000660000006006060
60006060606060606000606000006060060060606060600060600600060060606000606060606000000000600000606060600000606060606000000060006060
60006060666060606600606000006600060066006060666060600600060066606000606060606660666006600000666066600000666060606000000060006660
60006060606060606000606000006060060060606060006066600600060060606060606060600060000000600000600060600000600060606060000060000060
66606600606066606660666000006660666060606660660066606660060060606660066060606600000066600600600066600600600060606660000006000060
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
66606660660066600000066060606660666006600600000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00600060060060600000600060606060606060000060000000000000000000000000000000000000000000000000000000000000000000000000000000000000
66606660060066600000600066606660660066600060000000000000000000000000000000000000000000000000000000000000000000000000000000000000
60006000060060600000600060606060606000600060000000000000000000000000000000000000000000000000000000000000000000000000000000000000
66606660666066600000066060606060606066000600000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
70000000888800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
07000000888800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00700000888800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
07000000888800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
70000000888800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000

__gff__
0000040400040100040403040101010108000001030300110403030411030101030b0b0b010101040101010101030104030b0b0b01010100041101010103010400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
__map__
393939393939393939393939393939391c1c1c040404040404040404040404040404040404040404040404041c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c393939000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0c282828282829292829290c3939393904000000000000000000000000000004040000000000000000000000000000040c39393939393939393939393939390c282829282828292828282928282829280c2829280c28282828282828280c2828282828282828282828282828282828280c2829292929290c3939393939393939
0d212727272727272727272829290c3904000000000000000000000000000004040000000000000000000000000000040d07070739393939393939390707070d310d0808080b080808080b0808080d2f0d2425260d21212727272122230d272f203127272727272727213121272731300d2727272727272829290c3939393939
0d212727272727272727272f2727281304000000000000000000000000000004040000000000000000000000000000040d07070707393939393939070707070d200f0808081b080808081b0808080f2f0d3435360d21272727273132330d272f2c2d2e272c1d2e272c2d2e272c1d2e210d27272727272727272728290c393939
0d272727272727272727272f27273f1304000000000000000000000000000004040000000000000000000000000000040d07070707393939393939070707070d271f08080808080b0b08080808081f2f0d2727270d27270c27272722230d272f3c3d3e273c3d3e273c3d3e273c3d3e2f0d272727272727272727272728133939
06272727272727272727272f2727271304000000000000002100000000000004040000000021212100000000000000040d07070707393939393939070707070d271a09080808081b1b08080808091a2f0d2727270d27270d27272721330d272f2727272727272727272727272727271a0d272727272727272727272727131c1c
06272727272727272727272f27273f130400000000000021210000000000000404000000210000002100000000000004280707070739393939393907070707282727270909090909090909090927272f282828140d27270d27272721210d272f0505050505050505050505050505050506272727272727272727272727133939
06272727272727272727272f272727130400000000002100210000000000000404000000000000210000000000000004270707070808080808080808070707272727272727272727272727272727272f272727272814142828280c282828272f0202020202020202020202020202020206272727272727272727272727133939
06272727272727272727272f27273f1304000000000000002100000000000004040000000000210000000000000000040c07070707393939393939070707070c27272c271d272e27272c271d272e272f272727272727272727210d222327272f0303030303030303030303030303030306272727272727272727272727133939
06272727272727272727272f2727271304000000000021212121210000000004040000002121212121000000000000040d07070707393939393939070707070d27273c273d273e27273c273d273e272f272727272727272727270f323327272f2727272727272727272727272727271a0c272727272727272727272727133939
0d272727272727272727272f27273f1304000000000000000000000000000004040000000000000000000000000000040d07070707393939393939070707070d2727272727272727272727272727272f272727270c1a272727271f272727272f2c1d2e272c1d2e272c2d2e272c1d2e2f0d27272727272727272727270c133939
0d212727272727272727272f27270c1304000000000000000000000000000004040000000000000000000000000000040d07070707393939393939070707070d2727272727272727272727272727272f272722230d272727272721272727272f3c3d3e273c3d3e273c3d3e273c3d3e200d2727272727272727270c2928393939
0d212727272727272727270c2828283904000000000000000000000000000004040000000000000000000000000000040d07070717393939393939170707070d2727272c271d272e2c271d272e27272f272721330d272727272727272727272f272727272721313127272727272731300d2727272727270c2929283939393939
2828282828282929282929283939393904000000000000000000000000000004040000000000000000000000000000042817171739393939393939391717172828282928282829282828292828282928282829282828292828282928282829282828292828282928282829282828292828282929292929283939393939393939
3939393939393939393939393939393904000000000000000000000000000004040000000000000000000000000000043939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939
1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c04040404040404040404040404040404040404040404040404040404040404043939393939393939393939393939391c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c
1339393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393900000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0c39393939393939393939393939390c0c282928282829282828292828282928282829280c282928280c292828282928282829282828292828282928282829280c282928282829280c28292828282928280c292828282928282829282828290c28280c2828282928282829282828292828282928282829282828292828282928
0d07070739393939393939390707070d0d212223272121213131313131212727272731210d080b08080d2121272727271a271a27271a2d271c21211a27271a270d212121212131210d2c2d2e272c2d2e270d0b08272727273d1d272727272e0d21270d2121312727272727313127272727272721212727212127272121272727
0d07070707393939393939070707070d0d213233212131273121212721312721272727270f081b08080f272727272727272727273d271d2727272727272727270d2131316d2121210d3c3d3e273c3d3e270d1b0808272727272d272727273e0d31270d2127271a1a272727272727272727272e0c28282928282928280c2c2727
0d07070707040404040404070707070d0d2131303127272727273127273121272727271a1f090909091f1a2727272727272727272727190a0a0a0a0a1c27272728282828142828280d27272727272727270d0808272727273d1d27272727210d31270d2727271a1a271a27272727272727273e0d3127276d276d3131283c2727
0d070707071c1c1c1c1c1c070707070d0d263621272727272727272727273127272727272727272727272727272727272727272727272727272727272727272727272727272727270d27272727272727272828280c27270c28280c281414282827270d271a271a1a6d1a27270e2727272727270d272727272727272727272727
0d0707070739393939393907070707280d262027272727272727272727312121272c2d2e272727276d272c1d2e27272727272c2d2e27273d2727273d2727272727272727272727270d27272727272d27272727270d27270d27270d1a2727272727270d271a271a1a1a1a27270d27272727273d0d27272727276d27270c272727
060707070704042727040407070707270d242527272716272727272727272127273c3d3e2727272727273c3d3e27272727273c3d3e2727276d276d2727272727272c2d2e272c2d2e0f2c2d2e27272d27272727272814142827270d272727272727270d271a1a1a1a1d6d27270d27272727272728272c272a2b272e270d3d2727
0c07070707393917173939070707070c0d3435272727272727272727272731272727272727276d27272727272727272727272727272727272727272c2d2e2727273c3d3e273c3d3e1f3c3d3e27272d27270e27271a27271a27270d302624252727270d27271d1a1a2d2727270d27272727272727273c273a3b273e270d272727
0d07070707393939393939070707070d0d3026312727272727272727312721312727272727272727272727272727272727272727272727272727273c3d3e2727272727272727272727272727276d3d27270d27272727272727270d263634352727270f2727190a0a152727270d27272727272e0c27272727272727270d2c2727
0d07070707040404040404070707070d0d2030212727272727272727272121272727272c2d2e27272c2d2e2727272727273d27273d27273d272727272727272727272727272727272727272727272727270d2727276d276d27270d1a2727272727271f27273d3d3d3d2727210d21272727273e0d27272727272727270d3c2727
0d07070707393939393939070707070d0d3620202024252627222321213127272727273c3d3e27273c3d3e2727272727210a0a0a0a0a0a0a0a0a272727272727272c2d2e272c2d2e270a0a0a0a0a0a0a270d31272727272727300d2c272727272727272727272727272727210d21272727272728282829282829282828272727
0d07070717393939393939170707070d0d3636202634353636323321212626272727272727272727272727272727272721272727272727272721272727272727273c3d3e273c3d3e2721272721272721270d26212127272731200d3c272727272727272727272727272727210d21272727272721212727212127272121272727
2817171739393939393939391717172828282928282829282828292828282928282829282828292828282928282829282828292828282928282829282828292828282928282829282828292828282928282829282828292828282928282829282828292828282928282829282828292828282928282829282828292828282928
3939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939393939
1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c
__sfx__
0e0404080c4700c1700c1700c1700c1700c1700c1700c170001000010000100001000010000100001000010000100001000010000100001000010000100001000010000100001000010000100001000010000100
010c00000c8500b8310a811188500080000800168500080000050000000005000000000500000000050000000c8500b8310a81118850008001885016850000000c0500000000050000000c050000000005000000
030c000024635180003c6303c62024635180003c6303c62024635180003c6303c62024635180003c6303c62024635180003c6303c62024635180003c6303c620246353c6553c6353c6253c6353c6553c6353c625
010c00000c8500b8310a811188500080000800168500000002050009000e0500c90002050000000e050000000f8500e8310d8111b85000800008001a850000000505000000110500000005050000001105000000
010c00002455024530245100c5000c5000c50023550245502655026530265100c5000c5000c50024550265502755027530275100c5000c5000c500265502455022550225302251029554295550c5003055430555
010c00002b5542b5522b5422b5322b5222b5150c5020c5020c5020c5020c5020c5020c5020c5002b52429555275550c5020c5020c5020c5000c5000c5000c50026554265550c50027554275550c5002655426555
010c00002455024530245100c5000c5000c50023550245502655026530265100c5000c5000c50024550265502755027530275100c5000c5000c500265502755029550295302951027554275550c5002955429555
010c00002b5542b5522b5422b5322b5222b5152b5540c5022b5540c5042b5540c5042b5540c5042b554000042b554000042b554000042b554000042b554000042b5540000400004000042f554000043055430552
010c0000245542455224542245322452224515245540c502245540c504245540c504245540c50424554005042355400504235540050423554005042355400504235540050400504005042b554005042b5542b552
010c00000c850008000c85018850008001885016850008000c850008000c85018850008001885016850008000a850008000a85016850008001685014850008000a850008000a8501685000800168501485000000
010c000030552305422f5312e5212d5112c51100000000002455224555000003055230555000002e5542e5522e5522e5422d5312c5212b5112a51100000000002255222555000002e5522e555000002c5542c552
010c00002b5522b5422a53129521285112751100000000001f5521f555000002b5522b55500000295542955229552295422853127521265112551100000000001d5521d555000002955229555000002755427552
010c000008850008000885014850008001485013850008000885000800088501485000800148501385000800078500080007850008000785000800078500080007850008000b850008000e850008001185000000
010c00002c5522c5422b5312a521295112851100000000002055220555000002c5522c555000002b5542b5522b5522b5252c5542c5502b5502b5252c5502c5252b5502b525000002b0002f5512f5553055430552
010c000027552275422653125521245112351100000000001b5521b55500000275522755500000265542655226552265252755427550265502652527550275252655026525000002b0002b5512b5552b5542b552
010c0000088500080008850148500080014850138500080008850008000885014850008001485016850008000a850008000a85016850008001685014850008000a850008000a8501685000800168501785000000
010c00002c5522c5422b5312a521295112851100000000002055220555000002c5522c555000002e5542e5522e5522e5250000000000000000000000000000002b5522b555000002f5522f555000003555235555
010c000027552275422653125521245112351100000000001b5521b55500000275522755500000295542955229552295250000000000000000000000000000002655226555000002b5522b555000002f5522f555
010c00000c850008000c850188500c800188500c850008000b850008000b850178500b800178500b850008000a850008000a850168500a800168500a850008000985001800098501585009800158500985001800
050c000033752337423273131721307112f711327523275533752337553275232755337523375532752327513375133755327523275530752307552b7542b7522b7322b7222b7150070000700007000070000700
090c000030752307422f7312e7212d7112c7112b7522b7552b7522b7552b7522b7552b7522b7552b7522b7512b7512b7552b7522b7552b7522b75527754277522773227722277150070000700007000070000700
010c0000088500080008850148500880014850088500080007850008000785013850078001385007850008000e850028000e8501a8500e8001a8500e850028000c850008000c850178500b800178500b85000000
030c000024635180003c6303c62024635180003c6303c62024635180003c6303c62024635180003c6303c6253c600000003c6303c62500000000003c6303c6253c6353c65524635246253c6553c6552465524655
050c000033752337423273131721307112f7113275232755337523375532752327553375233751357513573235722357153375433732337223371532754327323272232715337543373233722337150070000700
090c000030752307422f7312e7212d7112c7112b7522b7552c7522c7552b7522b7552c7522c75132751327323272232715307543073230722307152f7542f7322f7222f715307543073230722307150070000700
010c0000088500080008850148500880014850088500080008850018000885014850058001485008850018000a850008000a850168500c800168500a850008000a8500a8500c8001385013850178001785017850
050c00002c7522c7422b7312a721297112871100700007002075220755007002c7522c755007002e7542e7522e7522e7250070000700007000070000700007002b7522b755007002f7522f755007003575235755
090c0000277522774224750247501f7501f7502475024750277522775500700277522775500700267542675226752267250070000700007000070000700007002375223755007001f7521f755007001a7521a755
010c00000c8500c8000c8500c8000c8500c8000c8500c8000c8500c8100c8500c8100c8500c8100c8500c8100c8500c8300c8500c8300c8500c8300c8500c8300a8500a820008000a8500a820008000c8500c820
030c000024630180003c6303c62024635180003c6303c62024635180003c6303c62024635180003c6303c62024635180003c6303c62024635180003c6303c6202463500000000002463500000000003c6303c625
090c00000050000500005000050000500005000050000500005000050000500005000050000500005000050000500005000050000500005000050000500005002e5502e522005002e5502e522005003055030522
090c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000267502672000000267502672000000277502772500000
00020000082700b3700f4701367027670052600f6600b350156500e653042430d6430363308623006130000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000200000a4700f6701e670022600d650044400762000420026200e603042030d6030360308603006030000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00060000272501c250142501b250152500e250102500825004250022500125000250002500025000250002400024000230002300022000210002100020000200002000020000200002000020000200002003e200
0003000019250132500f2500b25007250042500225001240002400024000240002300022000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0001000022650156003e650040500f6500b0300761004600046000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000200000b577121713d672004761d67400277017731c66601162226500174519646012302163501637216361c6340143019627002261b6220e6110f6150021603612026130c2040630702502026030a40605704
000300001e55020750235502375000500005000050000500005000050000500005000050000500005000050000500005000050000500005000050000500005000050000500005000050000500005000050000500
000100000c15515003000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0010000011150101500f1500a15009150081500215001150001500015000140001400012000110001000010000000000000000000000000000000000000000000000000000000000000000000000000000000000
010400001f05026050000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00060000180501c0501f0502405224000240000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000400000e5500b500005000050000500005000050000500005000050000500005000050000500005000050000500005000050000500005000050000500005000050000500005000050000500005000050000500
000400003c0002b1502d160251301b1501415015150191501b1501f0001e0002000000000280002b0002a0002c0002f00033000372002c20001200042000020000200012003d20038200312002a200272002b700
1b0700002f25021250272501b25022250122501a2400c24014240062400d230012300623000230002300020007650002000365000200026500020007650002000265000200002000c65000200056500165000200
000300001d0502205005000050000400003000030003d000230001d00018000260000d0000c0001700012000110002a000330003300015000280001b000370001f0001a0001700023000100001d0001700018000
000200001c65017600156001360000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000900000b573121713d672004761d67400277017731c66601162226500174519646012302163501637216361c6340143019627002261b6230e6130f613002130361302603072030460306203046030620304603
090c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002675026720000002675026720000002775027725
900200000560204602066020560206602036020000200002000020000200002000020000200002000020000200002000020000200002000020000200002000020000200002000020000200002000020000200002
900200000564204612066320562206622036220000200002000020000200002000020000200002000020000200002000020000200002000020000200002000020000200002000020000200002000020000200002
0002000008270342700f4700357027670045600f6602e250156500e653035430d643036332622300613005202340021400202201f4001e4000c2102c3002c3000000000000000000000000000000000000000000
00010000121600a1600446004460044601665003450034400244009640034400145001450014500340014400034000b4000040000400004000040000400004000040000400004000040000400004000040000400
__music__
00 01023232
00 01023232
01 03020432
00 01020532
00 03020632
00 03020708
00 09020a0b
00 0c020d0e
00 09020a0b
00 0f021011
00 09020a0b
00 0c020d0e
00 09020a0b
00 0f021011
00 12021314
00 15161718
00 12021314
02 19021a1b
00 41424344
00 41424344
00 41424344
00 41424344
00 41424344
04 1c1d1e31
01 01424344
02 03424344
00 41424344
00 41424344
00 41424344
00 41424344
00 41424344
00 41424344
01 252a4344
02 5f424344

