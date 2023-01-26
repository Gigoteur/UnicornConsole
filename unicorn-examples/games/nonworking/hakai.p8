pico-8 cartridge // http://www.pico-8.com
version 36
__lua__
 --twitter: @freezypop8
 --how ya doin
dialouge={"  \"i fight to relieve my\n  overwhelming rage towards \n  income inequality.\"",
"  \"waku waku! you look strong!\n    let's fight! hey! did i\n    say you could leave?!\"",
"\"combat is the peak of dance.\ni was the greatest ballerina in\nthe world yet when i fought back\nand won as i was mugged in an\nalley i felt more choreographic\nbeauty than i ever did onstage.\ni devoted my life to combat, and\nby breaking all limits i gained\npower beyond what a human being\nshould physically be capable of.\nnow, show me what you got!\"",
"\"hey, let's have a good match,\nalright? just so you know i'm\ngoing to try and end this quick,\ni've got to pick my son up from\nschool in an hour.\""
}

poke(0x5f5c,-1)
poke(0x5f2e,1)
poke(0x5f38,1)
poke(0x5f39,1)
charcolor=split("7,8,12,11,5,9,12,11,13,8,1,3,10,9,1,3,10,14,1,3,4,9,12,11")
plyvals=split("x,y,dx,dy,ddx,ddy,h,stn,nrg,atk_hld,a_t,a_dpw,a_ft,a_blk,a_altatk,win,kicharge,km_c,km_aim,a_st,bt,bn,d_c,5000,9980,0,0,0,0.08,100,0,50,0,0,0,0,0,0,0,0,0,0,0,0,240,0")
skills=split("bullet,heavy,sniper,track,barrage,strike,beam,bomb,cOFFA,zOSSIMOV,aLISINA,rACHEL")
depth_mode7=split("6000,200,150,125,100,80,56,46,34,26,18,10,3,1")
function _init()
	pal()
	cursor,clerp,clds,coros,d_coros,l_coros,aitog,manaual,t,timer,shake,g_state,zoom,cam_x,cam_y,versus=split "0,0",split"0,0",{},{},{},{},unsplit"1,1,0,0,0,1,1,0,0"
	menu,coshake,customtbl=cocreate(main_menu),cocreate(screenshake),{} --events
	for i=1,200 do
		add(l_coros,{})
		add(clds,1.5*rnd(2)^3\1+5)
	end
end

function roundstart()
	-- playerstuff
	p1,mob={},{}
	for i=1,23 do
		local plyderp=plyvals[i+23]
		p1[plyvals[i]]=plyderp
		mob[plyvals[i]]=plyderp
	end
	p1.en,mob.en,flag=mob,p1
	joy,joy2,stars,cam,mob.x,t,cam_dx,cam_dy,bug,hs,combo,co_shake,clsh={},{},{},{},unsplit"5800,5940,0,0,0,0,0,0,0"
end

function _update60()
	t-=1
	controller()
	in_game,kicharge_sfx=type(g_state)=="string",1
	if in_game then
		timer+=1/60
		if clsh>0 then
			clsh-=1
		else
			sfx(13,-2)
		end
		if g_state=="practice" then 
			menuitem(1,"enemy ai",function() aitog*=-1 end)
			menuitem(4,"command info",function() manual*=-1 end)
			if combo==0 then
				mob.h,p1.h=100,100
			end
		end
		-- menuitem(2,"debug",function() bug=1 end)
		--*game running*
		edir=unitvector(p1.x,p1.y,mob.x,mob.y)
		sight,d_fr,curr_player=edir.m,joyoutput(joy,37) --superdash frame
		player(p1)
		camcntrol()
		multicord(edir,-1)
		joy2,joy,curr_player=joy,joy2,versus and 1 or 28 --control swap
		player(mob)
		curr_player=1 -- for player arrow
		if(p1.km_c<3 and mob.km_c<3)sfx(20,-2) sfx(30,-2) sfx(15,-2)
		if(kicharge_sfx)sfx(26,-2)
	end 
	if in_game then
		bground%=4
		mode7()
		ground()
		fillp()
		speedlines()
		palp()
	end

	coresume(menu)
	corotbldo(coros)
	if in_game then
		dash_coros()
		fillp(0b1010010110100101)
		kamepal()
		hudstf(p1)
		palp()
		animator(mob)
		multicord(edir,-1)
		curr_player=0
		animator(p1)
	end

	if(costatus(menu)=="dead")menu=cocreate(main_menu)
	coresume(coshake,0)
	pal(split "140,130,3,4,6,141,7,8,142,10,11,12,13,143,15,16",1)
	pal(unsplit"0,129,1")
end

function tiles(lev)
	for i=0,3 do
		local ick=i*18+14
		spr(200+lev+i*2,10,ick,2,2)
		if lev==0 then
			for p=0,16,16 do
				spr(64+i+p,28+p/1.6,ick)
			end
		end
	end
end

function tileselect(a)
	rect(9,13+a,26,30+a)
end

function main_menu()
	local intro,round,p1win,mobwin,lenucam,menucam=unsplit"0,1,0,0,200,200"
	while true do
		cursy,clursy,lenucam,winnar=cursor[1],clerp[1],lerp(lenucam,menucam,.2)
		
		if not in_game then
			cls()
			palt(0b0000010000000000)
			if g_state<5 then
				camera()
				dash_coros()
				for to=0,1 do --testing 
					mkedst(rnd(128),148-to*168,0,-.4+to*.8,rnd(20),8)
				end
			end
			for i=0,1 do
				local addc,i1 = btnp(3,i) and 1 or btnp(2,i) and -1,i+1
				clerp[i1]=lerp(clerp[i1],cursor[i1],.3)+.005
				if g_state<6 then
					if addc then
						cursor[i1]+=addc
						sfx(22)
					end
					cursor[i1] %= g_state>2 and 4 or 3
					if btnp(4,i) then
						sfx(24)
					elseif btnp(5,i) then
						sfx(23)
					end
				end
			end
			if g_state<5 then
				camera(ceil(lenucam-.5))
				spr(246,233,82+clursy*10,1,1) 
				?unsplit"@FREEZYDEV,201,28,12"
				-- spr(unsplit"183,232,26")
				-- ?unsplit"MADE BY FREEZYDEV\n   @FREEZYPOP8,230,20,12"
				sspr(unsplit"0,96,63,24,201,27,126,48")
				?unsplit"arcade\|q\rlocal versus\|q\rpractice,244,83,10"
				-- ?unsplit"hakai out now on itch.io!\n\n  play in your browser!\n\n   play on your phone!,214,78,10"
				color(0xaa)
			end
		end
		

		if g_state==1 then --main menu
			menucam=200
			if btnp(4) then
				g_state,aitog,manual=unsplit"3,1,1"
				if cursy==0 then
					g_type,bground=7,0
				elseif cursy==1 then
					versus,g_type=1,"vs"
				else
					aitog,manual,g_type=unsplit"-1,-1,practice"
				end
				
				roundstart()
				cursorzero()
			end
		elseif g_state==3 then --player select
				menucam=0
				?unsplit"\^#select character and skills,10,3,9"
				local skip
				for tp=0,4,4 do
					local p1done,tp1=customtbl[3],tp==0
					tiles(0)
					for i=1,4 do
						local tpt,practicebot,tbl=i+tp,not tp1 and g_type=="practice" and p1done
						local clect,blink=customtbl[tpt],practicebot and 1 or tp/4+1
						local link=clerp[blink]
						if i<4 then
							color(0xaa)
							if clect then
								link=clect-1
								color(0xcc)
							end

							local sprit=link*18
							if i==1 then
								sspr(64+link*16,unsplit"96,16,16,2,89,32,32")
								if(versus)spr(145-tp/4,unsplit"0,12")
								tileselect(sprit)
							else
								sspr(link*8,unsplit(i==2 and "32,8,8,36,89,8,8" or "40,8,8,36,106,8,8"))
								?skills[ceil(link-.5)+(i==2 and 1 or 5)],36,i==2 and 98 or 116
								rect(7+i*10,13+sprit,16+i*10,22+sprit)
							end
						end
						if(i==4)spr(56,2,118+cos(t/15),2,1)
						if(skip)break --stops double input
						if not clect then
							if btnp(4,blink-1) then
								if (bground or customtbl[7]) and p1done then
									roundreset(1)
									g_state,timer=4,0 --for map select
									cursorzero()
								elseif i<4 or practicebot then
									customtbl[tpt]=cursor[blink]+1
									if(tp1 and i==3) skip=1 cursorzero() 
								end
							elseif btnp(5,blink-1) then
								if i>1 and not (customtbl[5] and tp1 and g_type=="practice") then
									cursor[blink],customtbl[tpt-1]=customtbl[tpt-1]-1
									clerp[blink]=cursor[blink]
								elseif tp1 and not p1done then
									g_state,customtbl,g_type,bground,versus=1,{}
									cursorzero()
								end
							end
							break
						end
					end
					
					if(g_state==1)break
					if bground then
						--story
						?unsplit"it's may 7th in the\nyear 20xx. the 28th\nhakai tournament\nhas begun. the\ngrand prize is\n10 million dollars\nand a belt.\n\nfighters from all\nover the world\ncome to compete.\ndo you have what it\ntakes to win?,50,15,10"
						break
					end
					camera(lenucam-63)
				end
				camera()
		elseif g_state==4 then --map select
			if bground then
				g_state=g_type
			else
				palt(0b0000000000000000)
				tiles(32)
				tileselect(clursy*18)
				?unsplit"oCEAN\|y\rbOUND wASTES\|y\rmETROPOLIS\|y\riNCLEMENT wEATHER,30,14"
				if btnp(4) then
					bground,g_state=cursy,g_type
				elseif btnp(5) then
					g_state=3
				end
			end
		elseif bground==4 then--arcade complete screen
			cls(0)
			musicplay(48)
			sspr(unsplit"64,64,32,32,30,5,64,64")
			spr(unsplit"140,10,77,4,3")
			for p=0,4,4 do
				spr(188+(t/4)%4,13+4*p,78+p)
			end
			sspr(unsplit"0,80,32,16,58,70,64,32")
			?"congratulations!!!",30,105,t
			?"time:"..timer\1,unsplit"50,115,12"
		elseif g_state==7 then --intermission
			sfx(41)
			local tloop="\^d1"
			while true do
				local cara,choice,playercol,flip=p1
				for pop=0,75,75 do
					choice=cara[1]
					sspr(48+choice*16,96,16,16,10+pop,12,32,32,flip)
					playercol,flip,cara=charcolor[choice],true,mob
					?skills[choice+8],10+pop,48,playercol
				end
				spr(unsplit"100,55,20,2,2")
				print(tloop..dialouge[choice],0,58)
				?"press 🅾️ to continue",24,2,t
				tloop=""
				if(btnp(4))break
				yield()
			end
			g_state="arcade"
		elseif g_state==6 then --game over screen
			sfx(49)
			for i=0,180 do
				ovalfill(unsplit"42,111,80,117,0x22")
				pal(12,charcolor[p1[1]])
				spr(unsplit"96,54,100,2,2")
				?unsplit"game over,45,60,10"
				yield()
			end
			_init()
		elseif g_state==5 then --versus end screen
			local winner,dude = mob.win==3 and 1,mob
			p1win,mobwin=0,0
			musicplay(48)
			for i=60,0,-60 do
				camera(-i)
				local side,choice = winner and 2 or 0,dude[1]
				pal(12,charcolor[choice])
				spr(96+side,unsplit"23,40,2,2")
				?skills[choice+8],unsplit"18,60,12"
				?winner and "wins!!!" or "doesn't...",unsplit"18,70,10"
				dude,winner=p1,not winner and 1
				pal()
				palt(0b0000010000000000)
			end
			if versus then
				?unsplit"🅾️ rematch\n\n❎ exit,40,100"
				if btnp(5) then
					music(-1)
					_init()
				end
				if btnp(4) then
					p1.win,mob.win,g_state=unsplit"0,0,vs"
					music(-1)
					return
				end
			else
				?"🅾️ continue",40,100
				if btnp(4) then
					g_state,p1.win,mob.win=unsplit"7,0,0"
					bground+=1
					roundreset(bground+1)
					music(-1)
				end
			end

		else --game running checks
		
			if t<=0 and g_state!="practice" then
				if p1.h<mob.h then
					p1.h=0
				elseif mob.h<p1.h then
					mob.h=0
				else
					winnar=164 --draw
				end
			end
			
			if p1.h<=0 then
				-- win stuff
				mobwin+=1
				winnar=mob
			elseif mob.h<=0 then
				p1win+=1
				winnar=p1
			end
			p1.win,mob.win=p1win,mobwin

			while intro<60 do
				if intro<=30 then
					spr(unsplit"102,50,53,3,1")
					spr(239+round,73,53)
				else
					sspr(unsplit"32,88,24,8,40,48,48,16")
				end
				intro+=1
				yield()
			end
			flag=1
			
			if winnar then
				for i=1,80 do
					hs,combo=10,0
					if(i==1)sfx(52,0)
					sspr(unsplit(winnar==164 and "32,80,24,8,40,48,48,16" or t<=0 and "72,48,24,16,15,30,96,64" or "32,64,24,16,15,30,96,64"))
					yield()
				end
				if g_state!="practice" then
					if versus then
						if p1win==3 or mobwin==3 then
							g_state=5
						end
					else
						if p1win==3 then
							g_state,round=unsplit"5,0"
						elseif mobwin==3 then
							--lose screen
							g_state=6
						end
					end
					round+=1
					roundstart()
					roundreset(bground+1)
					intro=0
				end
			end
			p1.win,mob.win=p1win,mobwin
		end
		yield()
	end
end

function unsplit(a)
	return unpack(split(a))
end


function cursorzero()
	cursor[1],clerp[1]=0,0
end

function musicplay(n)
	if(stat(54)==-1)music(n)
end

function sfxcheck(n,c)
	if(stat(46+c)!=n)sfx(n,c)
end

-->8
--general math etc.
function lerp(v1,v2,w)
	--keep w between 0 and 1
	return v1-(v1-v2)*w
end

function roundreset(o)
	for i=1,3 do
		p1[i]=customtbl[i]
		mob[i]=bground and not versus and o or customtbl[i+4]
	end	
end

function rndcent(o)
	--random number centered at 0
	return rnd(o)-o/2
end

function delta(self,o,val)
	-- val = val or 1
	self.dx+=o.x*val
	self.dy+=o.y*val
end

function clock(b)
	return t%b==0
end

function palp()
	pal()
	palt(0b0001010000000000)
end

function vec(ox,oy,om)
	return {x=ox,y=oy,m=om}
end

function multicord(o,val)
	o.x*=val
	o.y*=val
end

function unitvector(x1,y1,x2,y2)
	--finds unit vector from origin as well as magnitude
	local ox,oy=x2-x1,y2-y1
	local mag=100*sqrt((ox/100)^2+(oy/100)^2)
	return mag==0 and vec(unsplit"0,0,0") or vec(ox/mag,oy/mag,mag)
end

function atanxy(o) --SAVES 1 TOKEN
	return atan2(o.x,o.y)
end

function corothing(tbl,fnc,...)
	add(tbl,cocreate(fnc))
	coresume(tbl[#tbl],...)
end

function dash_coros()
	for i=1,3 do
		corotbldo(d_coros)
	end
end

function corotbldo(tbl)
	for i in all(tbl) do
        if costatus(i)=='dead' then
            del(tbl,i)
        else
            coresume(i)
        end
    end
end

function draw_polygon148(points)
	local xl,xr,ymin,ymax={},{},10000,0
	for k,v in pairs(points) do
		local p2=points[k%#points+1]
		local x1,y1,x2,y2=v.x,v.y & -1,p2.x,p2.y & -1
		if y1>y2 then
			y1,y2,x1,x2=y2,y1,x2,x1
		end
		local d=y2-y1
		for y=y1,y2 do
			local xval=x1+(x2-x1)*(d==0 and 1 or (y-y1)/d) & -1
			xl[y],xr[y]=min(xl[y] or 32767,xval),max(xr[y] or 0x8001,xval)
		end
		ymin,ymax=min(y1,ymin),max(y2,ymax)
	end
	for y=ymin,ymax do
		rectfill(xl[y],y,xr[y],y)
	end
end

-->8
--level drawing and hud
function mode7()
	--draws bground
	local scoom,hscoom,camdx,pyval2,iter
	local fog_center,fog_dist,clr=cam_x\2200,5-cam_y\2000,split"0x94,0x49,0xe9,0x9e,0xc5,0x55,0x77,0x11,0x11,0x11"
	if kamepal() then
		cls(0)
	elseif bground>1 then
		pal(split "1,2,7,0,13,6,5,8,2,10,9,12,13,6,13")
		rectfill(unsplit"0,0,127,34,0x11")
		gradient(34,0x01)
	else
		cls(12)
		gradient(30,0x5c)
		line(unsplit"0,61,128,61,0xff")
		gradient(62,0xc1)
	end
	if bground!=2 then
		fog_center,fog_dist=0,6
	end
	for twinkles=0,1 do --ocean twinkles
		for i=1,14 do
			local izoom,scoomd,hscoomd,camdxd,pyval2d=1/zoom,scoom,hscoom,camdx,pyval2
			iter=depth_mode7[i]+izoom
			scoom=1000/iter
			camdx,hscoom,pyval2=cam_x/iter-63,scoom/2,(10000-cam_y)/iter+62 --screen coord: model coord*focal length/model dist
			
			if i>1 then
				local pyvaldif=pyval2d-pyval2
				if twinkles==0 then
					fillp(i%2!=0 and 0b1010010110100101)
					if bground==0 then --ocean twinkles
						if i>2 then
							local iterrnd,rando=i,rnd(300)
							for sea=pyval2,pyval2d,pyvaldif/2 do 
								local dydx=(pyval2d-sea)/pyvaldif
								local dscop=lerp(scoomd,scoom,dydx)
								srand(iterrnd)
								iterrnd+=1
								for lf=-10*dscop-lerp(camdxd,camdx,dydx),140,dscop*30/i do
									local move,tnk=lf+dscop*rndcent(6),dscop*cos(t/60+iterrnd/1.2)/12
									if(tnk>0)line(move-tnk,sea,move+tnk,sea,0x77)
								end
							end
							srand(rando)
						end
					else
						rectfill(0,pyval2d,127,pyval2,clr[ceil(i/4)]) --ground
					end
				else --if twinkles==1
					if bground==3 then
						clip(0,0,128,pyval2)
						corotbldo(l_coros[i])
						clip()
					end
					if i>10 then
						for w=0,10,10 do --boundary walls
							local xval=w*scoomd-camdxd
							line(xval,pyval2d,xval,0,0xaa)
							for b=0,14 do
								line(w*scoom-camdx,pyval2-b*scoom,xval,pyval2d-b*scoomd,0xaa)
							end
						end
					end
					local hscoomd,drive=hscoomd,t/20%1
					local clty,cltyd=pyval2-scoom*.08,pyval2d-scoomd*.08
					for q_side=0,1 do
						for q_ter=fog_dist,0,-1 do
							local q = (q_side==0 and q_ter or -q_ter)+fog_center
							fillp(0b1010010110100101)
							local mtx,mtxd=q*2.2*scoom-camdx,q*2.2*scoomd-camdxd
							if bground==2 then --city
								if i>2 then
									local cltx,cltxd,bheight=mtx+scoom*1.3,mtxd+scoomd*1.3,clds[q+i*5]
									color(0x2a)
									for o=32,40,8 do --cars
										local lercx,lercy,size=lerp(cltx,cltxd,drive),lerp(clty,cltyd,drive),lerp(scoom,scoomd,drive)/14
											if i>11 then
												sspr(32,o,16,8,lercx,lercy,size,size/2)
											else
												pset(lercx,lercy)
											end
										drive=1-drive
										cltx+=hscoom
										cltxd+=hscoomd
										color(0x28)
									end
									local rndbuild,rndbuildd=bheight*hscoom,bheight*hscoomd
									if (i+1)%3!=0 and abs(mtxd+hscoomd-63)<(63+hscoomd) and pyval2d-rndbuildd<128 then 
										--spritescrapers
										local front_bheight,check,heig,scoomdif=clds[q+i*5+5],mtx>63,pyval2-rndbuild,scoom-scoomd
										local dy=heig-pyval2d+rndbuildd
										local slope_r,building_color=(mtx-mtxd)/dy,0x6d
										for o=0,1 do
											poke(0x5F3A,o)
											if check then --walls
												local dx=mtxd-mtx
												for wl=(mid(1,128,mtx)-mtx)\1,dx,sgn(dx) do
													local slope,wallx=wl/dx,mtx+wl
													local wally_b=pyval2+slope*pyvaldif
													local wally_t=wally_b-rndbuild-slope*(rndbuildd-rndbuild)
													if abs(wallx-64)<64 then
														if i>10 and wl%2==0 then
															tline(wallx,wally_b,wallx,wally_t,slope*2,0,0,4/(scoom-scoomdif*slope))
														else
															line(wallx,wally_b,wallx,wally_t,building_color)	
														end
													else
														break
													end
												end
											end
											if o==0 then
												mtx+=scoom
												mtxd+=scoomd
											end
											check,building_color=mtx<63,0x02
										end
										if heig>64 and heig-dy<128 then --roofs
											color(0x7d)
											for i=0,dy do
												line(mtx-i*(mtx-mtxd)/dy,heig-i,mtx-scoom-i*slope_r,heig-i)
											end
										end
										local bottom = pyval2-((i+2)%3==0 and 0 or front_bheight*hscoom)
										if bheight>front_bheight or bottom==pyval2 then --faces
											if i>9 then
												--detailed faces
												local scooba=scoom/30
												rectfill(mtx,heig,mtx-scoom,bottom,0x11)
												for lev=heig,bottom,hscoom/3.5 do
													if lev>127 then
														break
													end
													color(0x22)
													for dop=0,scooba,scooba do
														rectfill(mtx,lev+dop,mtx-scoom,lev+dop+scooba)
														color(0x00)
													end
												end
												for wal=0,scoom,scoom/7.5 do
													pilx=mtx-wal
													color(0x66)
													for dop=0,scooba,scooba do
														rectfill(pilx-dop,bottom,pilx-dop-scooba,heig)
														color(0x55)
													end
												end
											else
												rectfill(mtx,bottom,mtx-scoom+1,heig,0x16)
											end
										end
									end
								end
							else
								if bground==1 then --chaingang
									local q30=q%3==0
									if i%4==0 and q30 then 
										for ds=hscoom,40*hscoom,hscoom*.93 do
											local chainx=mtx+ds*.6
											sspr(112,48,16,16,chainx,pyval2-ds,hscoom,hscoom)
										end
									end
								end
								if q%(bground==3 and 1 or 4)==0 and i%2==0 and i!=14 then --clouds
									for z=1,3 do
										fillp(z==1 and 0b1010010110100101.1 or 0b1010010110100101)
										local o_line,cldx,cldy=z*i/12,mtx+clds[i*5+q]*scoom,(3000-cam_y)/iter+67+clds[i*10+q]*scoom/3.7
										if  bground==3 and rnd(0.02)==0.0001 and z==1 then
											corothing(l_coros[i],lightning,rnd(10000),rnd(2500)+3500,depth_mode7[i])
										end
										if scoom-o_line>1 then
											local sqz,cclrs=1,4
											for y=0,1 do
												ovalfill(cldx+o_line,cldy+o_line,cldx+scoom*4-o_line,cldy+scoom/sqz-o_line,clr[z+cclrs])
												if bground==0 then --shadows
													cldy,sqz,cclrs=pyval2,3,7
													fillp()
												else
													break
												end
											end
										end
									end
								end
							end
						end 
					end
				end
			end
		end
	end
end

function ground()
	liny=zoomcam(10007)
	if liny-cam_y<130 then
		--ground stuff
		local ground,size,gcol=bground==2 and 80 or bground==0 and 112 or 96,ceil(40*zoom),0x00
		if bground==3 then
			pal(split "1,2,3,0,5,6,6,8,2,10,11,12,13,6,13")
		else
			gcol=0x99
		end
		rectfill(0,liny,127,liny+100,gcol)
		for i=-45,45 do
			sspr(ground,16,16,16,63+i*size-cam_x%40*zoom,liny,size,size)
		end
	end
end

--camera stuff
function camcntrol()
	local lrp,dzoom=min(lerp(1,sight,.5),400)
	cam.dx,cam.dy=p1.x\1-63,p1.y\1-63
	if versus then
		delta(cam,edir,sight/2) --much more zoom out
		dzoom=100/sight
	else
		--lerp camera to enemy
		delta(cam,edir,lrp)
		dzoom=50/lrp
	end

	zoom=mid(0.05,dzoom<=zoom and dzoom or zoom+min(.007,dzoom-zoom),1)+.005

	cam_dx,cam_dy=cam.dx-cam_x,cam.dy-cam_y
	cam_x,cam_y=cam.dx,cam.dy
end

function zoomcam(o,c,zom)
	-- c = c or cam_y
	return (o-(c or cam_y)-63)*(zom or zoom)+63
end

function screenshake(camx)
    while true do
		camera(camx,cos(t/3)*shake)
        shake = max(0,shake-0.5)
        camx=yield()
    end
end

function hudstf(o)
	-- if bug==1 then
	-- 	-- debug stats
	-- 	?p1.vec.m,unsplit"3,60,0"
	-- 	?sight,unsplit"3,69,0"
	-- end
	for i=-1,1,2 do --mirror for mob
		pal(split "0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0")
		local healthbar=36.5
		for q=-1,1,2 do
			camera(q)
			for p=0,2 do
				local xval=63.5+44.5*i
				palt(0b0000010000000000)
				spr(198+o[1]*2,56+56*i,0,2,2,i>0)
				if(not(p==2 and o.h<=0))rectfill(xval,3,xval-i*healthbar,5,0x88)
				color(o.nrg>65 and clock(6) and 0x88 or o.nrg<25 and 0x55 or 0xaa)
				line(xval,6,xval-i*o.nrg*0.365,6)
				for wi=0,o.win do
					if(wi>0)spr(167,59+5*i+wi*i*6,8)
				end
				for bob=0,o.bn/80 do
					if(bob>0)spr(120,60+49*i-bob*i*5,8)
				end
			
				--combo
				if combo>0 then
					local comx,combot=co_shake + (combousr==mob and 118 or 10),combousr.en.stn
					combo=min(combo,99)
					for i=0,1 do
						spr(070+combo%10,comx,unsplit"21,1,2")
						if combo>9 then
							spr(070+combo\10,comx-8,unsplit"21,1,2")
						end
						if p!=2 then
							break
						end
						clip(0,38-combot/5,127,500)
						pal(13,10)
						co_shake\=-1.2
					end
					clip()
					if combot<=0 then
						combo=0
					end
				end
				?max(0,t)\60,unsplit"60,3,10"
				camera(0,q)
				if p==1 then
					if q==1 then
						camera()
						pal()
						healthbar=o.h*0.365
					else
						break
					end
				end
			end
		end
		if o.a_blk>0 then 
			spr(128,52+51*i,43,3,1)
		end
		o=mob
	end
	camera()
	if g_state=="practice" and manual<0 then
		if p1.a_t<=0 then
			?unsplit"\^#❎:shoot\nhold ❎:super\n⬆️⬇️⬅️➡️ away:block\n⬆️⬇️⬅️➡️+❎:attack,2,103"
			?p1.g and "\^#🅾️:jump\nskid+🅾️:side jump\n⬇️🅾️:super jump" or "\^#🅾️:dodge\nhold 🅾️:charge\nhold 🅾️+⬆️⬇️⬅️➡️:fly",2,83
		else
			?unsplit"\^#🅾️:dodge\n❎:cancel\nhold ❎:heavy (no cancel),2,103"
		end
	end
end

-->8
--draw functions
function dustblast(o,pow)
	for i=-3,3 do
		mkedst(o.x,o.y,i*rnd(4),pow^0.4*-3/rnd(8),rnd(24))
		sfx(3)
	end
	shake=3
end

function speedlines()
	--speedlines!!
	local cambo=unitvector(0,0,cam_dx,cam_dy)
	if cambo.m*zoom>3 and not p1.g then
		if #stars<30 then
			for i=1,2 do
				rd=rndcent(180)
				add(stars,vec(-cambo.y*rd+147*cambo.x+63,cambo.x*rd+147*cambo.y+63))
			end
		end
		pal(split "12,13,3,4,5,13,7,8,9,10,11,12,7,14,15,16")
		for i in all(stars) do
			i.x-=cam_dx
			i.y-=cam_dy
			ix,iy=i.x,i.y
			if(abs(ix)>210 or abs(iy)>210) del(stars,i)
			line(ix,iy,ix-cam_dx,iy-cam_dy,pget(mid(0,ix,127),mid(0,iy,127)))
		end 
	end
end

function kamepal()
	local o = p1.darken and mob.darken and 1 or p1.darken and p1 or mob.darken and mob 
	if o==1 then
		charcol(clock(2) and p1 or mob)
	elseif o then
		charcol(o)
	else
		return
	end
	return true
end

function charcol(o)
	local c_char=o[1]
	if p1[1]==c_char and o==mob then
		c_char+=12
	end
	local fch=charcolor[c_char+8]
	pal{0,0,0,fch,fch,0,charcolor[c_char],0,fch,charcolor[c_char+4],0,0,fch,0,0}
end

dither=split "0x1000.0000,0x1000.8000,0x1000.8020,0x1000.a020,0x1000.a0a0,0x1000.a4a0,0x1000.a4a1,0x1000.a5a1,0x1000.a5a5,0x1000.e5a5,0x1000.e5b5,0x1000.f5b5,0x1000.f5f5,0x1000.fdf5,0x1000.fdf7,0x1000.fff7,0x1000.ffff,0x1000.ffff"

function gradient(y,cols)
	poke(24372,1)--dual color 
    local ptn=1
    for i=y,y+30,2 do
        rectfill(0,i,127,i+1,cols+dither[ptn])
        ptn=min(ptn+1,17)
    end
end

function dtrail(ox,oy,dx,dy,o_r,typ,en,time)
	--trail for dash and ki
	local fr,dcol,pox,poy=0,split "0x44,0xaa,0x77"
	while o_r>2 do
		yield()
		if en then
			charcol(en)
			if o_r<10 then
				return
			end
		end
		if fr==3 then
			ox+=dx
			oy+=dy
			if typ==4 then
				--genki dama
				o_r=0
			elseif typ==5 then
				o_r/=1.06
				ox+=rndcent(3)
			elseif typ==7 then
				o_r=0
			elseif typ==8 then
				--menu screen
				o_r-=.1
				dcol=split"0x22,0x66,0xdd"
			elseif not typ then
				o_r/=1.05
				dy+=0.07
			else
				o_r/=1.2
			end
			fr=0
			if(#d_coros>20 and in_game)o_r/=1.1
		end
		fr+=1
		pox,poy=zoomcam(ox,cam_x),zoomcam(oy)
		if typ==4 and time>100 then -- genki dama
			fillp(0b1111010111111010.1)
			circfill(pox,poy,1000*zoom,0xaa)
			if(clock(6))sfxcheck(28,1)
			fillp()
		end
		pal(7,7)
		color(typ and dcol[fr] or 7)
		circfill(pox,poy,max(o_r>20 and 2 or 0,o_r*zoom)-(typ==8 and fr*2 or typ and fr or 0))
	end
end --dtrail

function aim(o,tgt,spd,kdir)
	--math to hit a moving target. man this function is tight
	local deltav=(tgt.dx-o.dx)*kdir.y-(tgt.dy-o.dy)*kdir.x
	if spd>abs(deltav) then
		delta(o,kdir,spd-abs(deltav))
	else
		deltav=spd*sgn(deltav)
	end
	o.dx+=kdir.y*deltav
	o.dy-=kdir.x*deltav
end

function mkedst(...)
	corothing(d_coros,dtrail,...)
end

function mkesprk(...)
	corothing(coros,spark,...)
end

function lightning(ox,oy,izoom)
--animate lightning
	yield()
	local rx,ry,colo,dur,scat=rndcent(60),rndcent(60),unsplit"0x77,4,75"
	if izoom then
		ry,dur,colo,scat=unsplit"200,20,0xaa,500"
		rx*=4
		sfx(17)
	else
		izoom=0
	end
	
	local f={ox,oy}
	for i=1,rnd(35)+35 do
		add(f,f[i]+rndcent(scat)+(i%2==0 and ry or rx))
	end
	for i=1,dur do
		local iizoom=1/(izoom+1/zoom)
		for j=1,count(f)-4,2 do
			line(zoomcam(f[j],cam_x,iizoom),zoomcam(f[j+1],cam_y,iizoom),zoomcam(f[j+2],cam_x,iizoom),zoomcam(f[j+3],cam_y,iizoom),colo)
		end
		yield()
	end
end

-->8
--player control and animator stuff
function controller()
	--input direction vector block
	local joy_p2
	for o=0,1 do
		local cjoy=vec(btn(0,joy_p2) and -0.707 or btn(1,joy_p2) and 0.707 or 0,btn(2,joy_p2) and -0.707 or btn(3,joy_p2) and 0.707 or 0)
		cjoy.no=1
		if cjoy.x==0 then
			cjoy.y/=0.707
			if(cjoy.y==0)cjoy.no=nil
		end
		if(cjoy.y==0)cjoy.x/=0.707
		joy2,joy_p2=cjoy,1
		if o==0 then
			joy=cjoy
		end
	end
end

function joyoutput(o,fr)
	--[[frame to return
	returns 1-5 based on vertical direction
	this does not affect flip]]
	return fr+(1-sgn(o.y)*(o.y^2+0.05))*2+.4
end

function player(o)
	--only called for p1 unless vs
	local dx,dy,acc,joyx,isply,isai,enang=o.dx,o.dy,0.3,joy.x,o==p1 or versus,aitog>0,atanxy(edir)
	local kmx,kmc,en,aimda,dfflty=o.km_x,o.km_c,o.en,enang-o.km_aim,clock(21-o[1]*5)
	if abs(aimda)>0.5 then
		aimda-=sgn(aimda)
	end
	o.vec,o.bn,o.bt,o.d_x,o.darken,o.skd=unitvector(0,0,dx,dy),min(240,o.bn+(o.bt<=0 and 2 or 0)),max(0,o.bt-1),o.d_c>=15 and not kmx and 1,kmc>10 or kmx and 1--darken screen check
	if o.stn<=0 and hs<=0 and flag then
		delta(o,vec(dx,dy),o.vec.m>28 and -0.06 or o.vec.m>15 and -0.02 or 0) --lerp speed down
		if not kmx then
			if clsh<=0 and hs<=0 then
				if isply then
					if btn(5,curr_player) then
						if joy.no and kmc==0 or o.a_t>0 then
							dashkick(o,joy)
						else
							superatk(o)
						end
					else
						ki(o)
						o.atk_hld=0
					end
				elseif isai then --enemy kick shoot and super ai goes here!!
					if o.a_t>0 then
						if o.nrg>25 and (dfflty or en.stn>0) then
							dashkick(o)
						end
					elseif sight<300 then
						if en.a_t>0 and dfflty then
							--dodge!
							local sid=sgn(rndcent(1))
							dodge(o,vec(edir.y*sid,edir.x*sid))
						elseif clock(10) then
							dashkick(o)
						end
					elseif sight<600 and clock(12) then
						superatk(o)
						ki(o)
					else
						if o.nrg<80 then
							o.d_x=1
						elseif o.nrg>50 then
							if sight>600 or kmc>60 then
								--if nearly full, commits to charge
								superatk(o)
							elseif kmc>0 then
								o.km_c=0
							end
						end
					end
				end
			else
				o.atk_hld,o.km_c=0,0
			end
		end

		if isply then
			if o.g then
				if dx*joyx<0 then
					o.skd,o.ddx=joyx,joyx*0.5
				end
				o.bn,o.dy=240,0
				if(joyx==0)o.dx/=1.05
				if btnp(4,curr_player) then
					local jump=2
					if btn(3,curr_player) then --superjump
						jump=8
						sfx(12)
						dustblast(o,8)
					end
					o.dy=-jump
					if(o.skd)o.dx=o.skd*jump
					sfx(1)
				end
			else
				o.dy+=joy.y/20 --air control
				if(btnp(4,curr_player)) dodge(o,joy)
				acc=0.1
			end
			o.d_c=btn(4,curr_player) and o.a_t<=0 and o.d_c+1 or 0
			if(abs(dx)<7)o.ddx=joyx*acc --max run spd
		elseif isai then
			if o.g then
				if sight>60 then
					o.ddx=.1*sgn(edir.x)
				else
					o.dx/=4
				end
				if o.y-o.en.y>=40 and aitog>0 then
					o.dy,o.g=-8
					sfx(1)
				end
			else
				--fly towards player 
				if sight>30 then
					o.ddx,o.ddy=edir.x*0.3,edir.y*0.05+0.08
				end	
			end
		elseif o.g then
			o.dx/=1.05
		end
	end
	
	if kmc<=0 then
		o.km_x=nil
	elseif kmx then
		o.km_c=max(kmc-0.25,0)
	end

	if kmc<0 then
		o.km_c+=1
	elseif kmc==0 then
		o.km_aim,o.a_kdir=enang
	elseif kmx==2 then
		mkki(5,o)
	elseif kmx==1 and clock(7) then
		ki(o,7)--barrage
	end

	if abs(aimda)>0.002 then
		o.km_aim+=aimda*(kmx and 0.05 or 0.05)
		o.km_aim%=1
	end

	if o.h<=0 and g_state!="practice" then
		o.stn,clsh=200,0
	--dead
	end

	o.nrg,o.a_blk=mid(o.nrg,100,0),max(0,o.a_blk-1)--block time

	if hs>0 then --hitstop
		hs-=1
	else
		if sight<10 then
			delta(o,edir,-1)
	   	end --no overlapping

		if o.stn>0 then
			o.km_c,o.d_c,o.atk_hld=unsplit"0,0,0"
			o.stn=min(o.stn-1,85)
			--no spd limit when stun
		end
		if clsh==1 then
			--break clash
			o.dx+=edir.x*3
			o.dy-=4
			o.a_ft=30
		elseif o==p1 or clsh<=0 then
			o.x+=dx
			o.y+=o.dy
			o.dx+=o.ddx
			o.dy+=o.ddy
		else --clash positions
			mob.x,mob.y,mob.dx,mob.dy=p1.x+14,p1.y,p1.dx,p1.dy
		end

		if o.a_t>0 then
			if(not kmx)o.a_t-=1
		elseif o.a_ft>0 then
			o.a_ft-=1
			o.a_str=nil
		else
			o.orient = dx>0 and 1 or dx==0 and o.orient
			o.fl,o.atk_hld,o.a_dir=o.orient,0
		end
		o.ddy,o.ddx=0.08,0 --gravity
		if o.d_x then
			--flying
			if joy.no then
				o.nrg-=0.35
				delta(o,joy,1)
				mkki(5,o)
				o.dy-=o.ddy/1.2--lower grav
			else
				--charge ki and brake
				o.nrg+=0.01*o.kicharge
				mkki(6,o)
				sfxcheck(26,3)
				o.kicharge,kicharge_sfx=min(o.kicharge+.3,45)
				delta(o,o.vec,-.2)
			end
		else
			o.kicharge=0
		end
		if o.a_t>2 then 	
			--track attack
			if sight<45  then
				--attack after dash 
				local dir = (o==p1 or versus and joy.no) and joy or edir
				o.a_altatk=(o.a_altatk+16)%32
				o.a_dir=joyoutput(dir,16+o.a_altatk)
				hitcrc(o,edir,nil,dir,kmx==2 and 2)
				zoom+=.3
				-- o.nrg+=10
				shake,o.atk_hld,o.a_t,o.a_str,o.km_x=unsplit"3,0,0"
				o.x,o.y=en.x-dir.x*12,en.y-dir.y*12 --place enemy
				o.fl = dir.x>0 and 1
			else
				if o.g then
					o.dy-=2 
					o.y-=1
				end
				delta(o,edir,o.a_dpw)
				o.a_dpw+=o.a_dpw<0.9 and 0.03 or o.a_str and -0.005 or 0
				if sight<100 then --magnet in
					o.x,o.y=lerp(o.x,en.x,.2),lerp(o.y,en.y,.2)
				end
				o.a_dir=joyoutput(edir,37)
				o.fl = dx>0 and 1
				if o.a_str then
					o.nrg-=0.6
					mkki(5,o)
					--strong aura
					if(o.nrg<=0)o.a_str=nil
				else
					if(clock(2))mkedst(o.x,o.y,unsplit"0,0,4")
				end
				--slight tracking
			end
		end
	end --hitstop

	--terrain collision
	if o.y>=10000 then
		if not o.g then
			sfx(4) --land sound
		end
		if dy>4 then
			dustblast(o,dy)
			--dust landing
		end
		if o.stn>0 then
			o.dx/=1.01 --stun slide
			-- o.stn+=.3
			-- o.g=1
			if dy>3 then
				o.dy*=-0.5
				o.stn+=30
				hs,o.y,o.g=10,9992 --bounce
			else
				o.g,o.dy=0,1
			end
		else
			if clsh>0 or o.d_x then
				o.dy*=-1.2 o.g=nil
			else
				o.dy,o.g=1,1
			end
		end
	else
		o.g=nil
	end
	--ceiling floor
	if o.x<200 or o.x>10000 then
		o.dx*=-0.4
		if o.stn>0 and abs(dx)>5 then
			o.stn+=60
			hs=10
			sfx(16)
			if(o.stn>80)o.dx*=2
		end
	end
	if(o.y<100)o.dy*=-1
	o.x,o.y=mid(200,o.x,10000),mid(100,o.y,10000)
end --player

function animator(o)
	--controls which animation starts
	local abdx,a_nf,a_sp,a_fr=abs(o.dx),1,20
	if o.a_t<2 and o.stn<2 and not o.g then
		o.fl = edir.x>=0 and 1
	end
	local fl,enem=o.fl,o.en
	
	if o.stn>0 then
	--stun animations
		if o.g then
			a_fr = o.stn<10 and 22 or 21
			--recovery on ground
		else
			if(clock(3) and hs<=0 and o.vec.m>5)mkedst(o.x+3,o.y,unsplit"0,0,8")
			o.fl,a_fr=enem.fl,(hs>0 and enem.a_dir) and enem.a_dir+7-enem.a_altatk or 25+-o.vec.y\.5
		end
	elseif clsh>0 then
	--atatatatatata
	o.nrg+=0.2
	a_nf,a_fr,fl=4,12
		if o==mob then
			a_fr,fl=28,1
			if(clock(8))corothing(coros,hitspark)
		end
	elseif o.a_kdir then
		a_fr=o.a_kdir
	elseif o.a_dir then
		--kick
		a_fr=o.a_dir
	elseif o.d_x then
		--super dash
		a_fr=d_fr
		--make booms while clashing
	elseif o.g then
		a_fr,a_nf = unsplit(abdx<0.8 and "0,1" or o.skd and "6,1" or "1,2")
	elseif o.bt<=0 and o.a_ft<30 then 
	--jumps
		a_fr = o.dy<-.3 and 3 or o.dy>0.2 and 5 or 4
	else
	--somersault
		a_fr,a_nf,a_sp=unsplit"48,8,30"
	end
	if o.skd and abdx>3 and clock(3) then
		mkedst(o.x,o.y,unsplit"0,-2,10")
		sfx(2)
	end

	local ox,oy,czoom,kmc,aimx,aimy,kcol=zoomcam(o.x,cam_x),zoomcam(o.y),max(zoom,0.13)*14,o.km_c,cos(o.km_aim),sin(o.km_aim),split "0xaa,0x7a,0x77"
	local charging,strug=not o.km_x and kmc>7
		--super attacks
	if(clock(60/a_sp))o.a_st+=1
	o.a_st%=a_nf
	o.a_fr=a_fr+o.a_st
	local sx,sy=o.a_fr%16\1*8,o.a_fr\16*8
	--finds spritesheet coords
	if zoom<0.3 then
		spr(145-curr_player,mid(0,ox-8,120),mid(0,oy-12,120))
	end
	if o.atk_hld>10 then --signal strong atk
		for po=0,2 do
			circ(ox,oy,80-o.atk_hld*4+po,0xaa)
		end
	end
	charcol(o)
	pal(7,7)
	for i=0,2,1 do
		color(kcol[i+1])
		--kamehame aura
		if charging or o.km_x==3 then 
			circfill(ox,oy,zoom*(3*kmc+4*sin(t/20))-i)
		end
		if charging then
			circ(ox,oy,zoom*3*(80-kmc-i))
			line(ox,oy,ox+aimx*1000,oy+aimy*1000) --aim line
			for i=0,1,.2 do
				local linezo=t/40+i
				for p=0,2 do
					line(ox+i,oy,ox+i+140*cos(linezo)+p,oy+140*sin(linezo)+p)
				end
			end
		end
		if o.km_x==3 then	
			--kamehameha fire!!
			sfxcheck(30,3)
			local kamel,tip=(40-kmc)*90*zoom,60*zoom
			o.a_kdir=joyoutput(edir,7)
			local girth=min(60,kmc)*zoom
			if i==0 then
				local cx,cy=enem.x-o.x,enem.y-o.y
				local dislong=cx*aimx+cy*aimy
				if abs(cy*aimx-cx*aimy)<(girth+5)/zoom and dislong<kamel/zoom and dislong>0 then
					strug=enem.km_x==3 --beam struggle
					if not strug then
						if(clock(3))hitcrc(o,edir,1,edir,3)
					end
				end
				--beam hitbox
			end
			if strug then --beam struggle
				o.km_aim,kamel=atanxy(edir),zoom*sight/2
				o.nrg+=.1
				tip*=2
				sfxcheck(51,2)
			end
			circfill(ox+aimx*kamel,oy+aimy*kamel,tip-i) --tip
			for p=0,1 do
				for j=-girth+i,girth-i do
					local xpart,ypart=ox-aimy*j,oy+aimx*j
					line(xpart+p,ypart,xpart+p+aimx*kamel,ypart+aimy*kamel) --shaft
				end
			end
			if(i==2)corothing(coros,lightning,o.x,o.y)
		end
		color(0x77)
	end
	ox-=zoom*4
	oy-=zoom*4
	
	if(kmc==4)sfx(20)
		pal(7,0)
		if not o.d_x and not o.a_str and o.km_c<8 then
			for i=-1,1,2 do
				camera(i)
				for p=0,1 do
					sspr(sx,sy,8,8,ox,oy,czoom,czoom,fl)
					camera(0,i)
				end
			end
			camera()
			charcol(o)
		end
		sspr(sx,sy,8,8,ox,oy,czoom,czoom,fl)
	palp()
end

function spark(o,flash,blang)
	local tbl,ox,oy={}
	local iter = blang and 8 or flash and 6 or 10
    for i=1,5 do
        add(tbl,i*.2+rnd(.3))
    end
    for i=0,iter do
		if flash or i==0 then --ki
			ox,oy=o.x,o.y
		end
		local ox,oy=zoomcam(ox,cam_x),zoomcam(oy)
		yield()
		if blang then --block line
			local bib=1000-i
			color(7)
			for s=1,2 do
				for dog=1,2 do
					circ(ox,oy,4*i+s*dog)
				end
				color(12)
			end
		elseif zoom<0.8 then
			for p in all(tbl) do
				for q=1,2 do
					local girth=(rnd(5)-(i+q)/2)*zoom
					if(flash)girth/=1.5
					local cang,sang=girth*cos(p),girth*sin(p)
					draw_polygon148{vec(ox+sang,oy-cang),vec(ox-sang,oy+cang),vec(ox-18*cang,oy-18*sang)}
					color(0x77)
				end
			end
		end
		if not flash then --shockwave
			if(i>8)color(0x5d)
			for f=0,1 do
				circ(ox,oy,i*40*zoom-f)
			end
		end
    end
end

function hitspark(str)
	--kick fx
    local ang=rndcent(.3)
    for i=0,4 do
		color(str and 0xaa or 0x77)
		for p=ang,ang+1,.2 do
			local angx,angy=63+90*cos(p),63+90*sin(p)
			draw_polygon148{vec(63,63),vec(angx,angy),vec(angx+5,angy+5)}
		end
        yield()
    end
end

-->8
--enemy controller and attacks

function hitcrc(usr,adir,hk,throw,nova)
	--spd vector,from enemy, dir, type
	local tgt,stun,pow,dam,hit,str=usr.en,unsplit"10,7,7"
	local impulse,joyo=vec(usr.dx-tgt.dx,usr.dy-tgt.dy),tgt==mob and hk and joy or joy2

	if (usr.a_str and tgt.a_str or nova==2 and tgt.km_x==2) and not hk then
		delta(tgt,impulse,1)
		clsh,tgt.km_c,tgt.a_t=unsplit"50,0,0"
		sfx(13) --clash
	elseif nova or (hk==2 and not tgt.a_str) or usr.a_str and not hk then
		if tgt.km_x!=2 then
			hit=1
		end
	elseif tgt.a_t<=0 then
		--hold away to block
		local blang=abs(atanxy(adir)-atanxy(joyo))
		if blang>0.5 then
			blang=abs(blang-1)
		end
		if tgt.stn<=0 and ((tgt.a_blk>0 or blang<0.3) and joyo.no and not (tgt==mob and not versus)) 
		and hk!=5 or (hk and tgt.km_x) then 
			-- if hk then
			-- 	sfx(22)
			-- else
			sfx(10) --block
			mkesprk(tgt,1,1)
			tgt.a_blk,hs,usr.a_dir=30,10 --block time
			usr.a_ft+=20
			tgt.h-=2
			-- end
		else
			hit=1
		end
	elseif not (hk or tgt.a_str or tgt.km_x==2) then --bounce when two lights hit
		pow,hs,throw,tgt.a_dir,tgt.a_t=12,20,edir,joyoutput(vec(1,-edir.y),16+tgt.a_altatk),0
		tgt.nrg+=5
		usr.nrg+=5
		sfx(10) --block
	end
	if hit then
		co_shake,combo=5,usr!=combousr and 1 or combo+1
		--sets combo, timer, combo user
		combousr,tgt.a_t,tgt.a_ft,tgt.atk_hld,tgt.g,tgt.km_x=usr,unsplit"0,20,0"
		tgt.y-=6
		if not hk then
			hs+=20
			corothing(coros,hitspark,usr.a_str)
			if usr.a_str then
				dam,pow,stun=unsplit"15,3,60"
				usr.a_ft+=10
			else
				usr.nrg+=10
			end
			if nova then
				dam,pow,stun,usr.km_c,hs=unsplit"40,20,80,0,40"
				sfx(42)
			else
				sfx(6) 
			end
			tgt.nrg+=5
		else
			sfx(14) --ki
			pow,stun,dam=unsplit((hk==2 or hk==5) and "1,25,4" or nova and "0.1,5,1.5" or "1,15,2")
		end
		tgt.h-=dam
		tgt.stn+=stun
	end
	if throw then
		delta(tgt,impulse,1)
		delta(tgt,throw,pow)
	else
		hs=0
	end
end

function mkki(ktyp,usr)
	local varki,ip,spd,dt,dr=usr[2],{dx=usr.dx,dy=usr.dy},unsplit"15,150,35"
	local rando = ktyp==5 and 3 or 28

	if ktyp==1 then --shot
		if varki==2 then
			usr.nrg-=3
		end
		usr.km_c,dr,spd = unsplit(varki==1 and "-6,25,15" or varki==2 and "-35,80,15" or varki==3 and "-12,35,40" or "-8,35,15")
		--aiming
	elseif ktyp==4 then --genki dama \o/
		dt,dr,spd,varki=unsplit"300,80,10,7"
	elseif ktyp==5 then --fly
		dt,dr=1,30
		if usr.km_x==2 then
			dr=70
			sfxcheck(15,0) --nova strike
		else
			sfxcheck(5,0)
		end
	elseif ktyp==6 then --charge
		dt,dr,ktyp=unsplit"1,50,5"
	elseif ktyp==7 then --barrage
		dr,varki=38,5
	end
	if ktyp>4 then
		delta(ip,vec(rndcent(rando),rndcent(rando)),1)
	else
		aim(ip,usr.en,spd,edir)
	end
	corothing(coros,anmtki,usr.x,usr.y,spd,ip,dr,dt,ktyp,usr,varki)
end

function anmtki(ox,oy,spd,kspd,pour,dt,typ,usr,varki)
	--moves ki blasts
	--ki trail fx is done in dtrail
	yield()
	while dt>0 do
		yield()
		local tgt,track=usr.en,varki==5 and max(dt/300,.2) or .2
		local kdir=unitvector(ox,oy,tgt.x,tgt.y)
		local hit=kdir.m<pour+2
	
		if typ!=5 then
			spr(usr==mob and 118 or 119,zoomcam(tgt.x,cam_x)-16*kdir.x-4,zoomcam(tgt.y)-16*kdir.y-3)
			if varki==4 then
				local speedo=unitvector(0,0,kspd.dx,kspd.dy)
				delta(kspd,speedo,min(0,15-speedo.m))
				track=1
			end
			aim(kspd,tgt,track,kdir)
			if typ==4 then --genki dama
				if dt<100 then
					pour=min(1000,(50-abs(dt-50))*45)
					sfxcheck(29,2)
					if kdir.m<pour then
						kspd.dx,kspd.dy=0,0
						if(clock(3))hitcrc(usr,kdir,1,nil,4)
					end
				end
				if hit then
					dt=min(dt,100)
				end
			else
				if hit then
					hitcrc(usr,kdir,varki)
					usr.nrg+=1
					break
				end
				if oy>10100 then
					break
				end
			end
		end
		ox+=kspd.dx
		oy+=kspd.dy

		mkedst(ox,oy,kspd.dx/2,kspd.dy/2,pour,typ,usr,dt)
		if usr.km_x==2 or typ==4 then
			corothing(coros,lightning,ox,oy)
		end
		dt-=1
	end
end --antmki
function superatk(o)
	if o.km_c==0 and o.nrg>3 or (o.nrg>65 and o.km_c>0)then
		o.km_c+=1
	end
	if o.km_c>40 then
		--super attack!
		local supeo=o[3]
		o.nrg,o.km_x=0,supeo
		if supeo==2 then
			--nova strike
			dashkick(o)
			delta(o,edir,30)
			o.a_str=1
		elseif supeo==4 then
			--genki dama \o/
			o.km_c=-60
			mkki(4,o)
		end
		sfx(20,-2)
	end
end

function dodge(o,dir)
	if o.bn>80 and o.bt<=0 then 
		--boost
		if(not dir.no and dir==joy)dir=o.vec
		delta(o,dir,20)
		--if no input go up
		o.bn-=80
		o.bt,shake=14,2
		sfx(7)
		mkesprk(o)
	end
end

function dashkick(o,dir)
	local atksp=5
	if not o.a_str then
		if(o.nrg>25)o.atk_hld+=1 --hold to strong kick
		if(btnp(5,curr_player))o.a_t=0 --press to cancel
	end
	if o.a_ft<=0 or o.atk_hld>20 then
		if o.atk_hld>20 then
			o.a_str,o.atk_hld,atksp=unsplit"1,0,7"
		else
			o.a_t,o.a_ft,o.atk_hld,o.a_dpw=unsplit"120,10,1,0"
			sfx(11)
		end
		delta(o,dir or edir,atksp)
		mkesprk(o)
	end
end

function ki(o,typ)
	if o.km_c>0 and o.nrg>3 or typ then
		sfx(8)
		mkesprk(o,1)
		if not typ then --typ is for barrage
			typ=1
			o.nrg-=3
		end
		mkki(typ,o)
		o.a_kdir=joyoutput(edir,7)
	end
end--shoots ki blats!1


__gfx__
33377333333333333333333333773333337733333377333333333377333333333333773333377333333333333373333337377333333773333333773333337733
33377333773333337733333333773333337733333377333333333377333773333333773333377333373773333377733373377333333773333377773333337733
33337733773777337733333337777333377773333777733333377773337777733337777377777773377773333377733337773333337733333737773333777777
33377773337777733377773337777333377773333777733333377773337777773377777333777737337777333337773333773333337777333337777337377333
33777773737777333377773333777333337777333377733333777773377777773777773337777333333777333337773333777333337773333377777333777733
33377733377777733377773333777733377777733773733333737333373737373337773337777733377773333377733333377777333777333773737333777333
33373773337733773333773333333733337337333737333333373333333737333337377333733733373377333373773333373333333377337337733337373333
33733373333373333333777333333373333333337333333333733333333333333333333733333373337333733337373333733333333373333333333373333333
77777733333337333337733373333377333733333333333333333333333733333337737333333773333773333377333333337733333377333737733333377333
77777773333373773337733337733777333733333333333333333333377733733337773333377773337777733377773333777733333377337337733333377333
37777337333337777777773333777773333737333333333333333333377737733377733333777777377777733377773337377733337777773777333333773333
37337333333337733337777333377773333733733333333377333333337777333377733733777333377737333777777333377773373773333377333333777733
37337333333377773377777333337737333733733333333377333333337777333337777333377733337333733733777333777773337777333377733333777333
33737333333777733377773333337733733777733333333337777733333773333333773333333773333733333333733337737373337773333337777733377733
33337333337337333337337333373333377777773773333337777773333333333333333333333333333333333333333373377333373733333337333333337733
33337333373333333333333733333333337777777777777737373737333333333333333333333333333333333333333333333333733333333373333333337333
33773333333337733333337777333333337333333333733333333333333333333333333333377333666666666666666677777777777777777777777777777777
337737333333377333377777337777333373773333337333333333733333333377333333333773330000000000000000efee7feee7feeffef7ffffffff7fffff
333773733337777333733777333777333373773333377333333337337773333377773333333777331111111111111111e9eff9efeeefee7fffffff7fffffffff
337773333373777733333777337777733377777333377333333777337777733337777333337777331d111dd11dd111d1e77eeee77e9ee7e9fffffffffffff7ff
33377333333777777777777737377373377777373377773337777333377777773377733333377333ddddddddddddddddffffeeffffeeffeefff7ffffffffffff
33377733337737333333337733333337377773333337773377777333337733333333773333377333d66d66d66d66d66deeeee77feeeeef777ffffffff7ffffff
33373333373373333333337333333337377333333337733377773333333333333333337333337333ddddddddddddddddfe9efffffe9effffffffffffffff7fff
333733337333333333333733333333333337333333377333777333333333333333333333333373336d666d6666d666d67777eef7777ffe77ffff7fffffffffff
33333333333333333333333333337733337733333333333333333333333333330000000000000005dddddddddddddddd77feeeeefffeeeefffffffffefefefef
33773333333333333333333333337773333773333773333333333333333377330aaaa0aaaa0a0a0511dd111dd111dd11ffeee99eeeee99eeefefefeffefefefe
33773333377773333337773733377773337777337777733333773773333777330a0000a00a0a0a0510000001100000017ffffeeef7777777fefefefeeeeeeeee
33377733377777333337777733777733337777337777773337777773337777330a0aa0a00a0a0a05d110011dd110011dffffeeeffff77777eeeeeeeeeeeeeeee
33777733337777773777777333777733337773333377777377777333337777330a00a0a00a0000051000000110000001ffeeeeeeefffffffeeeeeeeeeeeeeeee
33777733333777773773773333777333333377333337777373777333377773330aaaa0aaaa0a0a05ccc00cccccc00ccceeeee9eeeeeeffffeeeeeeeee9e9e9e9
333773333333377333333333337733333333773333333333333333333777333300000000000000052cccccc22cccccc2ee999999eeeeeeeee9e9e9e99e9e9e9e
33337733333333333333333333333333333333333333333333333333337733330000000000000005222222222222222299999999999eeeee9e9e9e9e99999999
595555558555555500555555555555553333dd6666dd33335555ddd555555ddd5555ddd55555ddd5555ddddd555ddddd5555ddd55ddddddd5555ddd55555ddd5
55a5595558e855550115555555bb55553332000000002333555ddddd555ddddd55dddddd555ddddd555ddddd555ddddd555ddddd5ddddddd555ddddd555ddddd
555755a55e7eee5551c15555533bbb5536d0000000000d6355dddddd5ddddddd55dddddd55dddddd55dddddd555ddddd55dddddd5ddddddd55dddddd55dddddd
5555555758e777e5551cc5555355b77533d6662222666d3355dd55dd5dddddd55ddd55dd55dd55dd55dd5dd555dd555555dd55dd55555ddd55dd55dd55dd55dd
9555955555e77775555c7c55535557753daaaddddddaaad35ddd55dd5555ddd55dd555dd55dd55dd5dd55dd555dd55555ddd55555555ddd55ddd55dd5ddd55dd
5a555a5555e777755555c7755535555532aad222222daa235dd555dd5555ddd55dd55ddd555555dd5dd55dd55dd555555dd55555555ddd555dd555dd5dd555dd
55755575555e77e5555557755555555530222222222222035dd555dd555ddd5555555dd555555ddddd55dd555ddddd555ddddd55555ddd555dd55ddd5dd55ddd
555555555555555555555555555555553300333333330033ddd555dd555ddd555555ddd5555dddd5dd55dd555dddddd5ddddddd555ddd55555ddddd55ddddddd
a9559a7a777777e87777c555777777773333dd6666dd3333dd555dd5555ddd55555ddd55555dddd5ddddddd555555dd5ddd55dd555ddd5555ddddd5555ddddd5
7a95a777700777e87777c5557777777b333d00022000d333dd555dd5555ddd5555ddd55555555dd5ddddddd555555dd5ddd55dd55ddd5555ddd55dd555555dd5
77a59a7a700077e8777cc555bb777bb33360000000000633dd555dd555ddd55555dd555555555dd5555dd55555555dd5dd555dd55ddd5555dd555dd555555dd5
7a9559a977007e8877cc7c5533bbb33533888d2222d88833dd55ddd555ddd5555ddd5555dd55dd55555dd555dd55ddd5dd55ddd55ddd5555dd55ddd55555ddd5
a955555577777e85ccc777c5553335553d88dddddddd88d3dd55dd5555ddd5555dd55555dd55dd55555dd555dd55dd55dd55dd55ddd55555dd55dd55dd55dd55
55559a95777ee885555c777c555555553266d222222d6623dddddd555ddd5555dddddd55dddddd5555dd5555dddddd55dddddd55ddd55555dddddd55dddddd55
5559a7a9eee888555555c7775b5b5b553022222222222203ddddd5555ddd5555dddddd55ddddd55555dd5555ddddd555ddddd555ddd55555ddddd555ddddd555
555a777a8888555555555c7755bbb55533003333333300335ddd55555ddd5555dddddd555ddd555555dd55555ddd55555ddd5555ddd555555ddd55555ddd5555
5555555555555555000000000000000000000000000000000000003333333333333333330000000000000030000000000000000000000000333333333333d633
555ccc555555555500ee00000000ee0077000007a00000000eeee0000000000000000333099999909909900099099990000000000000000033333333333d6233
55ccccc6666555550eee00000000eee07a00000a900000000f00f0fff0f0f0ff00ff00330eeeeee0ee0eee0eee0eeee000000000000000003333333dd66d2333
55ccccc6666655550ee000eeee000ee007a0007a00000000077770707070707070707033000ff000ff0fffffff0ff0000000000000000000333333d622d62333
56cccee6666665550ee00e0ee0e00ee00a9000a9000000000f0f00f0f0f0f0f0f0f0f033330ff030ff0fffffff0ff0330000000000000000333333622d622233
566eee22266665550eee0eeeeee0eee000a907a0000000000e00e0eee0eee0e0e0ee0033330770307707777777077000000000000000000033333d2236222233
56665222226665550eee0ee00ee0eee00099a9900777aa9000000000000000000000003333077030770770707707777000000000000000003333362333332233
56666522226665550eee0ee00ee0eee00009990077a0099933333333333333333333333333077030770770007707777000000000000000003333d22333362233
556eeecc222666550ccc00eeee00ccc0000090007a00000033333333333333335555555533077030770770307707700000000000000000003333623333322333
55ceeeecccc6665500cccceeeecccc00000000000a7aa900333333333333333355555555330ff030ff0ff030ff0ff0330000000000000000333d223333622333
55cceeecccc6665500cccccccccccc000000000000a99990333333333333333355555555330ff030ff0ff030ff0ff0000000000000000000333622d233223333
55cccc55ccc66655000cccccccccc000000000000000009933388833333aaa33555aa555330ee030ee0ee030ee0eeee0000000000000000033362d6236223333
55ccc655ccc66655000cccccccccc0000000000077a00a9933383833333a3a33555aa555330990309909903099099990000000000000000033336d2262233333
55566655ccceee550000cccccccc0000000000000a99999033388833333aaa335555555533000030000000300000000000000000000000003333d62222333333
55566655ccceee550000cccccccc000000000000000000003333333333333333555555553333333333333333333333330000000000000000333d623333333333
55666555555eee5500000cccccc0000000000000000000003333333333333333555555553333333333333333333333330000000000000000333d633333333333
00000000500000000000000000000000000030000000000300000000dd11dd11555555555555555555555555eeefff7755555555555555555555555555555555
0eee00e050eeee0eee0e00e000000000099030990099990009900990dd11dd115555555555555555555555eeeffff7775555555555555555555666d555555555
0f00f0f050f00f0f000f0f00000000000ee000ee0eeeeee00ee00ee0dd00dd00555555555555555555555eeeffff777755555555555566666662006d55555555
077700705070070705077005000000000ff00ff00ff00ff00ff00ff0dd22dd225555555555555555555eeeeffff7777755555666666622222222006d55555555
070070705070070705077005000000000ff00ff00ff00ff00ff00ff0dd11dd1155555555555555555eeeefffff77777756666222222222222222006d55555555
0f00f0f000f00f0f000f0f0000000000077077000770077007700770dd11dd115555555555555555eeeefffff777777766d22222222222222222200d55555555
0eee00eee0eeee0eee0e00e000000000077777030770077007700770dd00dd0055555555555555eeeeefffff7777777f66d222222222222222222006d5555555
00000000000000000000000000000000077770030770077007700770dd22dd225555555555555eeeeffffff7777777ff66d222222222222222222006d5555555
500055555000555500000000000000000777770307700770077007702200220055555555555eeeeeffffff7777777fff66d222222222222222222006d5555555
0aaa055508880555000000000000000007707700077007700770077022002200555555555eeeeefffffff7777777fffe566d22222222222222222200d5555555
0a0a00050808005500000000000000000ff00ff00ff00ff0000000002200220055555555eeeeefffffff7777777ffffe566d222222222222222222006d555555
0aaa0aa00888080500000000000000000ff00ff00ff00ff00000000022222222555555eeeeeeffffff77777777ffffee566d222222222222222222006d555555
0a0000a00800880500000000000000000ee000ee0eeeeee00ee00ee02200220055555eeeeefffffff777777777fffeee566d222222222222222226666d555555
50050a0050050805000000000000000009900099009999000990099022002200555eeeeeefffffff777777777ffffeee556d22222222226666666aaaaadd5555
55550aa0555088800000000000000000000030000000000300000000220022005eeeeeeefffff5f777577777ffffeeee5566d226666666aaaaaaaaaaaaaadd55
5555500055550000000000000000000033333333333333333333333322222222eeeeeeffffff5f755775777fffffeeee5566dddaaaaaaaaaaaaaaaaaaaaadddd
5555555555552222222255555555555530000003333333333333333355555555eeeeefffffff5575575577fffffeeeee55556ddaaaaaaaaaaaaaaddddddddd6d
555555555552eaa77aae25555555555500eeee03000333333333333355555555eeefffffffff7555555777fff55eeee55555666ddaaaaaddddddd6666666666d
555555222224222222224222225555550ff00f000f0000000000000355555555effffffffff7775555777ffff55eeee5555566666ddddd66666666666666666d
5552222c42e2777227772e24c222255507000707700777007000770355575755fffff55ff77777755777ffff5555eee55555666666d66666666666666666666d
5222cccce2a2727227272a2ecccc222507007707000707707070700355577755fffff55f777777755777ffff5555eee55555666666d666666666666666666ddd
22ecccccf2a272a22a272a2fccccce220f0ff00f00f0fff0fffff03355557555ffff555577777755557ffff5e55e5e55555556d666d666666666666666665555
2ffc2c2ca272a2aaaa2a272ac2c2cff20ee0000e00eee0e00e0e003355555555ffff55557777775775fffff5e55e5e555555555dd6d666666666655555555555
2aac222ca272a222222a272ac222caa200003300000000000000033355555555fff575575777775775fffffe5555ee55555555555dddd6555555555555555555
2ffc2c2ca272f2aaaa2f272ac2c2cff20000000000000000000000005555ccc5ff75755757777757f5fffffe5ee5ee5555555555555555555555555555555555
22ecccccf2a2e2f22f2e2a2fccccce220eeee0e0eeee0e00e0eee0e05cc5ccccf77755557775555555555fee5ee5e55575755555575555555555557557555555
5222cccce2a292e22e292a2ecccc22250f0000f0f0000f00f00f00f055ccccc5777757757775555555555fee5ee5e55557555555777555555755555577755555
5552222c42e2449229442e24c222255507770070707707777007007055ccccc57777577577755555555555555555555575755555575555555555575557555555
555555222224222222224222225555550f0000f0f00f0f00f00f0000555ccc557777577577755555555555555555555555555555555575755555575555557575
555555555552eaa77aae2555555555550e0000e0eeee0e00e00e00e055ccc5557555555555555555555555555555555555555755555557555557757755555755
55555555555522222222555555555555000330000000000000000000555555557555555555555555555555555555555555555555575575755555575555557575
55555555555555555555555555555555333333333333333333333333555555557555555555555555555555555555555555555555555555555555575555555555
5555555555555555555555555555555555555555555555555555555555585555557777777777775588888888888888885555cc11c11c11555bbb3b33b3333355
555555555555555555555555555555555555555555555555558888855558555557777777777777758888888888888888555cc1c11c1111153bb333b333333335
5555555555555555555555555555555555555555555555555555855588888885577777fee7777797888880088888888855cccc1111eee111bb3b333339333335
555555555555555555555555555555555555555555555555555585558558558557777fffee7777e9888888008988800855ccc111000eee11b3b333b399393333
888555588855558888855558885555888555588888555588855585558558555557777fffeeeeeee9888887700e900088551c1111999ee0013b333939e3eee333
8885555888555588888555588855558885555888885555888558888588888855577777fffeeeeee9888889770e4708885cc1c111119ee911b3339eeeeeeee933
888555588855558888855558885555888555588888555588855885858855585557ee7f4000099490888989eeef4778885c11111700efe111b30000eeeeee0033
888555588855588888885558885558885555888888855588858585858585855557e97fe7707004078888474eef44488851ff11feeeffe701bb30370eeee03733
888555588855588888885558885588885555888888855588855588885558555557e9e7fe7777e97788889474e909488851fee1feeeffe9413333377eee933733
8885555888555888588855588855888555558885888555888555855855858555557eef7feeeeee498888e477449948885c1eeffeeeeee941b33eeeeeee999933
88855558885558855588555888588885555588555885558885555585885558855577efffeeeeefe985888f07770488885c11effeeeee9941333eeeeeefe99433
888555588855588555885558888888555555885558855588855555555555555555577effeeeee99555888ff00044885851cc1ffeee0009153b3eeeeeee9993b5
888555588855588555885558888885555555885558855588855585555585555555557e9feee009458888e4ee9948885551c111fee1771455b339ee0000094335
888888888855888888888558888885555558888888885588855585888888888555577e94feee94558888e9400085585555cc119fee11415533399ee000945355
888888888855888888888558888888555558888888885588855585555585555555777ee944994555888ee9440088555555c1c1e9ee941115333394eee9553555
8888888888558888888885588858888555588888888855888555855888888855577777ee99447555888eee9944888555555c111e994111113333395555555555
8885555888558885558885588855888555588855588855888558885858585855cccccccccc777777ccccccdccccc6d6c202211111111dd1d1111111111111111
88855558885588855588855888558888555888555888558885558558888888557777cccccccc777ccccccdcccccc66cc02202111111dddd11ddddddd11111111
888555588858885555588858885558885588855555888588855585555585555577cccc7777ccccccc777777cccc6dccd20220111111d1d1dddddddddd1111111
8885555888588855555888588855558885888555558885888555858888888885c5c5c5c5c5c5c5c5ccc77ccccc6d6ccd0220221111d1ddd111dddddd1111111d
8885555888588855555888588855558885888555558885888555855555855555cccccccccccccccccccdcccccc66c7772022021010dd1d1d1111a1dddd111ddd
88855558885888555558885888555588858885555588858885558855585858555c5c5c5c5c5c5c5ccccdcc7776d7ccd70220220101d1ddd11a1a1ddddddd11dd
5555555555555555555555555555555555555555555555555558555588558555c5c5c5c5c5c5c5c5ccdcccc76d6ccdcc2022021010dd1d1d11aa111dddd111a1
55555555555555555555555555555555555555555555555555555588588558855555555555555555cdcccccc66ccdccc0220220101d1ddd111a111111111a1a1
3000333330000333000003330000003300000033000000005555555575d0aa4011111111111111114d444446d444d4442022020000dd1d1d222a222222222aa2
00e0333300ee00330eee00330e00e0330eeee033000000005558555589909e901111111111111111d494946d649d94940220220000d1ddd1626a626262626a62
0ff033330f00f0330000f0330f00f0330f0000330000000055588855c110c5c01c1c171c1c171c1ce9e9e966e9e9e9e92022022222dd1d1dd6d6d6d6d6daa6d6
0070333330070033307700330777703307770033000000005588aa85b33033b0c1c1c1c1c1c1c1c1eeeee6deeeeeeeee022022866a61ddd1ddddddddddddaddd
30f0333300f000333000f0330000f0330000f03300000000558aa88800000000c7ccccc7ccc7ccc7eeee6d6eeeeeeeee20220222222d1d1ddddddddddddddddd
30e033330eeee0330eee00333330e0330eee00330000000058a88855000000001c1c1c1c1c1c1c1cfefe66efefefefef0226686666a666d1d5d5d5d5d5d5d5d5
3000333300000033000003333330003300000333000000005888555500000000ccccc7ccccccc7ccefedfefefefefefe202222222222221d5d5d5d5d5d5d5d5d
33333333333333333333333333333333333333330000000088555555000000007ccccccc7cccccc7ffffffffffffffff66668666666a66665555555555555555
__label__
dddtdddddtttitttdddddddttdddddddddddddddttttttiittdddddtdddtttiihhhhiitttttiihhhhhhhiiiiiiihhhhiiitttddddddddddddddddddddddddddd
tttttttttttiittdddddddddtddddddddddddddddttiiiiittdddddttttttiiihhhhiittdttiihhhhhhhhhhhhhhhhhhhiiittttddddddddddddddddddddddddd
tttitttttiiiittdddddddddttddddddddddddddddttiiiiittdddttttttiiihhhhhiitttttiihhhhhhhhhhhhhhhhhhhhiiittttttdddddddddddddddddddddd
iiiiiiiiiiiiittdddddddddttddddddddddddddddttiihiitttttttiiiiiihhhhhhhiitttiihhhhhhhhhhhhhhhhhhhhhhiiiitttttddddddddddddddddddddd
iiiiiiiiihhiitttdddddddttdddddddddddddddddttiihhiiitttiiitttiiihhhhhhiitttiihhhhhhhhiiihhhhhhhhhhiiitttttttttdddddtttddddddddddd
hhhhhhhhhhhhiittdddddddttdddddddddddddddddttiihhhiiiiiitttttttiihihhiitttttiihhhhhhiiiiihhhhhhhhiiitttttttttttdddddtttdddddddddd
hhhhhhhhhhhhiiitttdddttttdddddddddddddddddttiihhhhiiiiittdddttiiihhhiittdttiihhhhhiiitiiihhhhhhhiitttdddtttttdddddddtttddddddddd
hhhhhhhhhhhhhiiitttttttttddddddddddddddddttiiihhhhhhiittdddddttiihhhiitttttiihhhhhiitttiihhhiiiiitttdddddtttdddddddddttttddddddd
hhhhhhhhhhhhhhiiitttttittdddddddddddddddtttiihhhhhhhiittdddddttiihhhhiitttiihhhhhhiiitiiihhiiiiiittdddddddtdddddddddddtttttddddd
hiiiiihhhhhhhhhiiiiiiiittddddddddddddddtttiiihhhhhhhiittdddddttiihhhhiiiiiiihhhhhhhiiiiihhhiitiiittdddddddtdddddddddddtttttttddd
iiiiiiiihhhhhhhhhiiiiiittdddddddddddddtttiiihhhhhhhhiiittdddttiiihhhhhhiiihhhhhhhhhhiiihhhhiiiiiittdddddddtdddddddddddttiiittttt
itttttiiihhhhhhhhhhhhiitttdddddddtttttttiiihhhhhhhhhhiitttttttiihhhhhhhhhhhiiihhhhhhhhhhhhhhiiiiitttdddddttdddddddddddttiiiitttt
tttttttiiihhhhhhhhhhhhiitttdddttttttttiiiihhhhhhhhhhhhiiitttiiihhhhhhhhhhhiiiiihhhhhhhhhhhhhhhhhiitttdddtttdddddddddddttiiiitttt
ttdddtttiiihhhhhhhhhhhiiittttttttiiiiiiiihhhhhhhhihhhhhiiiiiiihhhhhhhhhhhhiitiiiiiiihhhhhhhhhhhhiiitttttttttdddddddddttiiiittttt
tdddddtttiihhhhiiihhhhhiiitttttiiiiiiiihhhhhhhhhhhhhhhhhiiiiihhhhhhhhhhhhhiiiiiiiiiiihhhhhhhhhhhhiiitttttitttdddddddtttiiitttddd
dddddddttiihhhiiiiihhhhhiiiiiiiiihhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhiiiiitttiiihhhhhhhhhhhhiiiiiiiiitttdddddtttiiitttdddd
dddddddttiiiiiiitiihhhhhhhiiiiihhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhiitttttttiihhhhhhhhhhhhiiiiiiiiitttttttttiiiittddddd
dddddddttiiiiiiiiiihhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhiiittdddttiiihhhhhhhhhhhhhhhhhhiiiitttttiiiiiittddddd
tdddddtttitttiiiiihhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhiittdddddttiihhhhhhhhhhhhhhhhhhhhiiiiiiiiihhiittddddd
ttdddtttitttttiiihhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhiittdddddttiihhhhhhhhhhhhhhhhhhhhhhiiiiihhhhiitttdddd
tttttttitttdtttiihhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhiittdddddttiihhhhhhhhhhhhhhhhhhhhhhiiiiihhhhhiitttddd
itttttiittdddttiihhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhiiittdddttiiihhhhhhhhhhhhhhhhhhhhhhiitiihhhhhiiittttt
iiiiiiiitttdtttiihhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhiiihhhiitttttttiihhhhhhhhhhhhhhhhhhhhhhhiiiiihhhhhhiiitttt
hiiiiiiiitttttiiihhhhhhhhhhhhhhhhhhhiiihhhhhhhhhhhhhhhhhhhhhhhhhhhhhhiiiiihhhiiitttiiihhhhhhhhhhhhhhhhhhhhhhhhhiiiihhhhhhhiiiiii
hhhhhhhiiitttiiihhhhhhhhhhhhhhhhhhhiiiiihhhhhhhhhhhhhhhhhhhhhhhhhhhhhiitiihhhhiiiiiiihhhhhhhhhhhhhhhhhhhhhhhhhiiiiiiihhhhhhhiiii
hhhhhhhhiiiiiiihhhhhhhhhhhhhhhhhhhhiitiihhhhhhhhhhhhhhhhhhhhhhhhhhhhhiiiiihhhhhiiiiihhhhhhhhhhhhhhhhhhhhhhhhhhiitttiihhhhhhhhhhh
hhhhhhhhhiiiiihhhhhhhhhhhhhhhiiihhhiiiiihhhhhhhhhhhhhhhhhhhhhhhhhhhhhhiiihhhhhhhhhhhhhhihhhhhhhhhhhhhhhhhhhhhiitttttiihhhhhhhhhh
hhhhhhhhhhhhhhhhhhhhhhhhhhhhiiiiihhhiiihhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhiiihhhhhhhhhhhhhhhhiittdttiih88hhhhhhh
hhchhhhhhhhhhhhhhhhhhhhhhhhhiitiihhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhiiiiihhhhhhhhhhhhhhhiitttttiih88hhhhhhh
hchchccchcchhccchccchccchchcicciiccchchchhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhiitiihhhhhhh8888888888itttiihh88hhhhhhh
hchchcchhchchcchhcchhhhchccchcichcchhchchhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhiiiiihhhhhhh8888888888iiiiiihh88hhhhhhh
hchhhchhhcchhchiichhhchhhhhchchchchhhccchhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhiiiihhhhhhhhhhhh88hhhhhi88888888888888h
hhcchchhhchchhcciicchccchcchhcchhhcchhchhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhiiiiihhhhhhhhhhhh88hhhhhh88888888888888h
hhhhhhhhhhhhhhiitiihhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhiitiihhhhhhhhhhhh88hhhhhh88hihh88hhhh88h
hhhhhhhhhhhhhhiiiiihhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhiiiiihhhhhhhhhhhh88hhhhhh88hhhh88hhhh88h
h888888hhhhhhhh888888hhhhhhhh8888888888hhiiihhh888888hhhhhhhh888888hhhhhhhh8888888888hhhhiiih888888hhhhhh88hhhhhh88hhhh88hhhhhhh
h888888hhhhhhhh888888hhhhhhhh8888888888hiiiiihh888888hhhhhhhh888888hhhhhhhh8888888888hhhhhhhh888888hhhhhh88hhhhhh88hhhh88hhhhhhh
h888888hhhhhhhh888888hhhhhhhh8888888888hiitiihh888888hhhhhhhh888888hhhhhhhh8888888888hhhhhhhh888888hhhh88888888hh888888888888hhh
h888888hhhhhhhh888888hhhhhhhh8888888888hiiiiihh888888hhhhhhhh888888hhhhhhhh8888888888hhhhhhhh888888hhhh88888888hh888888888888hhh
h888888hhhhhhhh888888hhhhhhhh8888888888hhiiihhh888888hhhhhhhh888888hhhhhhhh8888888888hhhhhhhh888888hhhh8888hh88hh8888hhhhhh88hhh
h888888hhhhhhhh888888hhhhhhhh8888888888hhhhhhhh888888hhhhhhhh888888hhhhhhhh8888888888hhhhhhhh888888hhhh8888hh88hh8888hhhhhh88hhh
h888888hhhhhhhh888888hhhhhh88888888888888hhhhhh888888hhhhhh888888hhhhhhhh88888888888888hhhhhh888888hh88hh88hh88hh88hh88hh88hhhhh
h888888hhhhhhhh888888hhhhhh88888888888888hhhhhh888888hhhhhh888888hhhhhhhh88888888888888hhhhhh888888hh88hh88hh88hh88hh88hh88hhhhh
h888888hhhhhhhh888888hhhhhh88888888888888hhhhhh888888hhhh88888888hhhhhhhh88888888888888hhhhhh888888hhhhhh88888888hhhhhh88hhhhhhh
h888888hhhhhhhh888888hhhhhh88888888888888hhhhhh888888hhhh88888888hhhhhhhh88888888888888hhhhhh888888hhhhhh88888888hhhhhh88hhhhhhh
h888888hhhhhhhh888888hhhhhh888888hh888888hhhhhh888888hhhh888888hhhhhhhhhh888888hh888888hhhhhh888888hhhhhh88hhhh88hhhh88hh88hhhhh
h888888hhhhhhhh888888hhhhhh888888hh888888hhhhhh888888hhhh888888hhhhhhhhhh888888hh888888hhhhhh888888hhhhhh88hhhh88hhhh88hh88hhhhh
h888888hhhhhhhh888888hhhhhh8888hhhhhh8888hhhhhh888888hh88888888hhhhhhhhhh8888hhhhhh8888hhhhhh888888hhhhhhhhhh88hh8888hhhhhh8888h
h888888hhhhhhhh888888hhhhhh8888hhhhhh8888hhhhhh888888hh88888888hhhhhhhhhh8888hhhhhh8888hhhhhh888888hhhhhhhhhh88hh8888hhhhhh8888h
h888888hhhhhhhh888888hhhhhh8888hhhhhh8888hhhhhh88888888888888hhhhhhhhhhhh8888hhhhhh8888hhhhhh888888hhhhhhhhhhhhhhhhhhhhhhhhhhhhh
h888888hhhhhhhh888888hhhhhh8888hhhhhh8888hhhhhh88888888888888hhhhhhhhhhhh8888hhhhhh8888hhhhhh888888hhhhhhhhhhhhhhhhhhhhhhhhhhhhh
h888888hhhhhhhh888888hhhhhh8888hhhhhh8888hhhhhh888888888888hhhhhhhhhhhhhh8888hhhhhh8888hhhhhh888888hhhhhh88hhhhhhhhhh88hhhhhhhhh
h888888hhhhhhhh888888hhhhhh8888hhhhhh8888hhhhhh888888888888hhhhhhhhhhhhhh8888hhhhhh8888hhhhhh888888hhhhhh88hhhhhhhhhh88hhhhhhhhh
h88888888888888888888hhhh888888888888888888hhhh888888888888hhhhhhhhhhhh888888888888888888hhhh888888hhhhhh88hh888888888888888888h
h88888888888888888888hhhh888888888888888888hhhh888888888888hhhhhhhhhhhh888888888888888888hhhh888888hhhhhh88hh888888888888888888h
h88888888888888888888hhhh888888888888888888hhhh88888888888888hhhhhhhhhh888888888888888888hhhh888888hhhhhh88hhhhhhhhhh88hhhhhhhhh
h88888888888888888888hhhh888888888888888888hhhh88888888888888hhhhhhhhhh888888888888888888hhhh888888hhhhhh88hhhhhhhhhh88hhhhhhhhh
h88888888888888888888hhhh888888888888888888hhhh888888hh88888888hhhhhhhh888888888888888888hhhh888888hhhhhh88hhhh88888888888888hhh
h88888888888888888888hhhh888888888888888888hhhh888888hh88888888hhhhhhhh888888888888888888hhhh888888hhhhhh88hhhh88888888888888hhh
h888888hhhhhhhh888888hhhh888888hhhhhh888888hhhh888888hhhh888888hhhhhhhh888888hhhhhh888888hhhh888888hhhh888888hh88hh88hh88hh88hhh
h888888hhhhhhhh888888hhhh888888hhhhhh888888hhhh888888hhhh888888hhhhhhhh888888hhhhhh888888hhhh888888hhhh888888hh88hh88hh88hh88hhh
h888888hhhhhhhh888888hhhh888888hhhhhh888888hhhh888888hhhh88888888hhhhhh888888hhhhhh888888hhhh888888hhhhhh88hhhh88888888888888hhh
h888888hhhhhhhh888888hhhh888888hhhhhh888888hhhh888888hhhh88888888hhhhhh888888hhhhhh888888hhhh888888hhhhhh88hhhh88888888888888hhh
h888888hhhhhhhh888888hh888888hhhhhhhhhh888888hh888888hhhhhh888888hhhh888888hhhhhhhhhh888888hh888888hhhhhh88hhhhhhhhhh88hhhhhhhhh
h888888hhhhhhhh888888hh888888hhhhhhhhhh888888hh888888hhhhhh888888hhhh888888hhhhhhhhhh888888hh888888hhhhhh88hhhhhhhhhh88hhhhhhhhh
h888888hhhhhhhh888888hh888888hhhhhhhhhh888888hh888888hhhhhhhh888888hh888888hhhhhhhhhh888888hh888888hhhhhh88hh888888888888888888h
h888888hhhhhhhh888888hh888888hhhhhhhhhh888888hh888888hhhhhhhh888888hh888888hhhhhhhhhh888888hh888888hhhhhh88hh888888888888888888h
h888888hhhhhhhh888888hh888888hhhhhhhhhh888888hh888888hhhhhhhh888888hh888888hhhhhhhhhh888888hh888888hhhhhh88hhhhhhhhhh88hhhhhhhhh
h888888hhhhhhhh888888hh888888hhhhhhhhhh888888hh888888hhhhhhhh888888hh888888hhhhhhhhhh888888hh888888hhhhhh88hhhhhhhhhh88hhhhhhhhh
h888888hhhhhhhh888888hh888888hhhhhhhhhh888888hh888888hhhhhhhh888888hh888888hhhhhhhhhh888888hh888888hhhhhh8888hhhhhh88hh88hh88hhh
h888888hhhhhhhh888888hh888888hhhhhhhhhh888888hh888888hhhhhhhh888888hh888888hhhhhhhhhh888888hh888888hhhhhh8888hhhhhh88hh88hh88hhh
hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh88hhhhhhhh8888hhhh88hhhhh
hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh88hhhhhhhh8888hhhh88hhhhh
hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh8888hh8888hhhh8888h
hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh8888hh8888hhhh8888h
hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh
hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh
hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh
hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh
hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh
hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh
hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh
hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh
hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh8hhhhhhhaaahaaahhaahaaahaahhaaahhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh
hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh888hhhhhahahahahahhhahahahahahhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh
hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh88aa8hhhhaaahaahhahhhaaahahahaahhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh
hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh8aa888hhhahahahahahhhahahahahahhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh
hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh8a888hhhhhahahahahhaahahahaaahaaahhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh
hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh888hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh
hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh88hhhhihhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh
hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhiiihhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhihhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh
hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhihhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh
hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh
hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhahhhhaahhaahaaahahhhhhhhahahaaahaaahhaahahahhaahhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh
hhhhhhhhhhhhhhhhhhhhhhhhiiihhhhhhhhhhhhhhhhhahhhahahahhhahahahhhhhhhahahahhhahahahhhahahahhhhhhhhhhhhhhihhhhhhhhhhhhhhhhhhhhhhhh
hhhhhhhhhhhhhhhhhhhhhhiiiiiiihhhhhhhhhhhhhhhahhhahahahhhaaahahhhhhhhahahaahhaahhaaahahahaaahhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh
hhhhhhhhhhhhhhhhhhhhhhiitttiihhhhhhhhhhhhhhhahhhahahahhhahahahhhhhhhaaahahhhahahhhahahahhhahiiihhhhhhiiihhhhhhhhhhhhhhhhhhhhhhhh
hhhhhhhhhhhhhhhhhhhhhiitttttiihhhhhhhhhhhhhhaaahaahhhaahahahaaahhhhhhahhaaahahahaahhhaahaaiiiiiiihhhiiiiihhhhhhhhhhhhhhhhhhhhhhh
hhhhhhhhhhhhhhhhhhhhhiittdttiihhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhiiihhhhhhhhhhhhhhhiitttiihhiiitiiihhhhhhhhhhhhhhhhhhhhhh
hhhhhhhhhhhhhhiiihhhhiitttttiihhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhihhhhhhhhhhhhhhhiitttttiihiitttiihhhhhhhhhhhhhhhhhhhhhh
hhhhhhhhhhhhhiiiiihhhhiitttiihhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhiittdttiihiiitiiihhhhhhhhhhhhhhhhhhhhhh
hhhhhhhhhhhhiiitiiihhhiiiiiiihhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhihhhhhhhhhhhhhhhhhhhhhhhhhhiitttttiihhiiiiihhhhhhhhhhhhhhhhhhhhhhh
hhhhhhhhhhhhiitttiihhhhhiiihhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhiitttiihhhhiiihhhhhhhhhhhhhhhhiiihhhhh
hhhhhhhhhhhhiiitiiiihhhhhhhhhhhhhhhhhhhhhhhhaaahaaahaaahhaahaaahaaahhaahaaahhhiiihhhhhhhhhiiiiiiihhhhhhhhhhhhhhhhhhhhhiiiiiiihhh
hhhhhhhhhhhhhiiiitiiiiiiihhhhhhhhhhhhhhhhhhhahahahahahahahhhhahhhahhahhhahhhhiiiiihhhhhhhhhhiiihhhhhhhhhhhhhhhhhhhhhhhiitttiihhh
hhhhhhhhhhhhhhiiiiiiiiiiiihhhhhhhhhhhhhhhhhhaaahaahhaaahahhhhahhhahhahhhaahhiiitiiihhhhhhhhhhhhhhhhhhiiihhhhhhhhhhhhhiitttttiihh
hhhhhhhhhhhhhhhhiiiiitttiiihhhhhhhhhhhhhhhhhahhhahahahahahhhhahhhahhahhhahhhiitttiihhhhhhhhhhhhhiiiiiiiiihhhhhhhhhhhhiittdttiihh
hhhhhhhhhhhhhhhhhiitttttttiihhhhhhhhhhhhhhhhahiiaiahahahhaahhaihaaahhaahaaaiiiitiiihhhhhhhhhhhhiiiiiiitiihhhhhhhhhhhhiitttttiihh
hhhhiiiiihhhhhhhiiittdddttiiihhhhhhhhhhhhhhhiiiiiiiiiiiihhhhhiiihhhhhhhhhiiiiiiiiihhhhhhhhhhhhiiitttiiiiihhhhhhhhhhhhhiitttiihhh
hhiiiiiiiiihhhiiiittdddddttiihhhhhhhhhhhhhhiiitttttiiiiiihhhhhihhhhhhhhhiiitttiiihhhhhhhhhhhhiitttttttiiihhhhhhhhhhhhhiiiiiiihhh
hiiitttttiiihiiiiittdddddttiihhhhhhhhhhhhhiiitttttttiitiiihhhhhhhhhhhhhiitttttttiihhhhhhhhhhiiittdddttiiiihhhhhhhhhhhhhhiiihhihh
iiitttttttiiiiitiittdddddttiihhhhhhhhhhhhhiitttdddttttttiihhhhhhhhhhhhiiittdddttiiihhhhhhhhhiittdddddttiihhhhhhhhhhhhhhhhhhhiiih
iitttdddtttiiitttiittdddttiiihhhhhhhhhhhhiitttdddddttttiiihhhhhhhhhhhhiittdddddttiihhhhhhhhhiittdddddttiihhhhhhhhhhhhhhhhhhhhihh
ittdddddddttiiitiiitttttttiihhhhhhhhhhhhhiittdddddddttiiihhhhhhhhhhhhhiittdddddttiihhhhhhhhhiittdddddttiihhhhhhhhhhhhhhhhhhhhhhh
tttdddddddtttiiiiiiiitttiiiiiiiihhhhhhhhhiittdddddddttiihhhhhhhhhhhhhiitttdddddttiihhhhhhhhhiiittdddttiiiiiiiihhhhhhhhhhhhhhhhhh
ttdddddddddttiiiiiiiiiiiiiiiiiiiiihhhhhhiiittdddddddttiihiiiiihhhhhhhiittdtdddttiiihhhhhhhhhhiitttttttiiiiiiiiihhhhhhhhhhhhhhhhh
ttdddddddddttiiiiiitttttiiitttttiiihhhhiiiitttdddddtttiiiiiiiiihhhhhhiitttttttttiihhhhhhhiiihhiiitttiiiiiitttiiihhhhhhhhhhhhhhhh
ttdddddddddttiiiittttttttttttttttiiihhhhihiitttdddtttiiiiitttiiihhhhhhiittttttiiihhhhhiiiiiiihhiiiiiiiiitttttttiihhhhhhhhhhhhhhh
tttdddddddtttiiitttdddddttttdddtttiiihhhhiitttdddddtttiiitttttiiihhhhhiiiitiiiiihhhhiiiiiitiihhhiiiiiiiittdddttiiihhhhhhihhhhhhh
ittdddddddttiiitttdddddddtdddddddttiihhhhiittdddddddttiitttdtttiihhhiiiiiiiiiiihhhhiiitttttiiihhhhhhhiittdddddttiihhhhhiiihhhhhh
iitttdddtttiiiittddddddddddddddddtttiihhhiittdddddddttiittdddttiihiiiitttttiiiihhhiiitttttttiiihhhhhhiittdddddttiiihhhhhihhhhhhh
iiitttttttiiiittddddddddddddddddddttiihhhiittdddddddttiitttdtttiiiiitttttttttiiiihiitttdddtttiihhhhhhiittdddddttiiiiihhhhhhhhhhh
hiiitttttiiiiittddddddddddddddddddttiihhhiitttdddddtttiiitttttiiiiitttdddddtttiiiiitttdddddtttiihhhhiiiittdddttitttiihhhhiiiiiii
hhiiiiiiiiihiittddddddddddddddddddttiihhhhiitttddddtttiiiitttiiiiitttdddddddtttttiittdddddddttiihhhiiittttttttttttttiihiiiiiiiii
hhhhiiiiihhhiittdddddddddddddddddtttiihhhhiittdddddddttiiiiiiiihiittdddddddddttttttttdddddddttiihhiiittttdddddtttdttiiiiittttttt
iiiihhhhhhhhiittdddddddddddddddddttiiiiihiitttdddddddtttiiiiiihiittddddddddddddddttttdddddddttiihiiitttdddddddddttttttittttttttt
iiiiiiihhhhhhiittdddddddddttdddttttttiiiiiittdddddddddttiihiiihiittddddddddddddddddtttdddddtttiiiiitttdddddddddddddtttttttdddddt
ttttiiiiihhhhiitttddddddddttttttttttttttiiittdddddddddttiihhihiittddddddddddddddddddtttdddtttiiiiitttddddddddddddddddtttdddddddd

__map__
8797000000000000000000000000008e00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0000000000000000000000000000008e00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0000000000000000000000000000008e00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0000000000000000000000000000aeaf00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000afbebf00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
beaeafbfaeafbfbebf00aeafbeaeafbf00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00bebf00bebf00aeaf00bebf00bebf0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000bebf0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
__sfx__
49040300286301c630166201362510200102000e2000c2000b2000a2000a200082000620005200032000320002200022000120001200002000020000200002001b200192001b2001b200192001b200192001b200
010200000336003620026600263002640026400263001620026000160001600016000160001600016000160000100001000010000100001000010000100001000010000100001000010000100000000000000000
0103000006630056200562004620046200262002620006202360023600236002360023600226002260021600206001f6001e6001e6001d6001c6001b600196001860016600166001660017600176001860018600
0102000001450024400e7300e730087300661006610096100b6200d620106201361016610186101b6101d6101f61022610246102461024610206101e6101d6101b6101a610186100861000000000000000000000
0006000019610186100e6000c6000d1000d1000010000100001000010000100001000010000100001000010000100001000010000100001000010000100001000010000100001000010000100001000010000100
490608000062002621006210262100621006210262100621036012e60115601006010060100601006010060100601006010060100601006010060100601006010060100601006010060100601006010060100000
010400002365007650076400663105631046210462103611006000d7011350100501005010c601125010050100501005010050100501005010050100501005010050100000000000000000000000000000000000
010200001765013350136501165010050106300e6200c6200b6200a6200a62008620066200562003610006101b600196001b6001b600196001b600196001b6000000000000000000000000000000000000000000
010200001b4210f031216211d03116021110110c0110c011090110501103011030110301102011010110001100011016010160101601026010160101601016010160100601006010960107601056010460100401
490300000365104751057410474103631036310363102631026310263102621016110161001600016010160101601066010460102601016010360101601000010000100001000010000100001000010000100001
05030e003c121040213b121366213962135421326212862119621136210e6210b6210960107601076210010100101001010010100101001010010100101001010010100101001010010100101001010010100101
010400000b02109021070210501104011020110001110001020010100100001010010100101001010010100101001010010100101001010010100100001010010100101001000010000100001000010000100001
000700000836104661036610065100651006410064100631006310063100621006210062100621006110061100611006010060500601006010060100601006010060100601006010060100601006010060100601
01010008236500765007640066310563104621046210361102611016110161100611006100161000610006000d7011350100501005010c6011250100501005010050100501005010050100501005010050100501
060400003d6513d6413a64137641336312c631146310763103531085010f501015010050100501005010150100501005010050100501005010050100501005010050100501005010050100501005010050100501
390800020064200642006420064200642006420064200642376023260219602046020460204602046020460204602046020460204602046020460204602046020460204602046020460204602046020460204602
1b050e00146400306022660136600a65008650066310563102631026210162101611016112900130001300012d0012d0012d0012d0012d0012d0012d0012d0012d0012d0012d0012d0012d0012d0012d00100001
930b0e000d61003620056200562005610056100461530000300012d0012d0012d0012d0012d0012d0012d0012d0012d0012d0012d0012d0012d0012d0012d0010000100000000000000000000000000000000000
050c00100c063000000c0630000021635000000c063000000c06330800188000000021635000000c0630c06300000000000000000000000000000000000000000000000000000000000000000000000000000000
010c00100c073000003080500000156650000030805000000c07300000308050000015665000000c0531560500000000000000000000000000000000000000000000000000000000000000000000000000000000
6904000c267441b744267441d744267441f744267442274426744247442674426744297042970430704307042d7042d7042d7042d7042d7042d7042d7042d7042d7042d7042d7042d7042d7042d7042d70400704
3d0800083102500005310250000531025000053102500005000050000500005000050000500005000050000500005000050000500005000050000500005000050000500005000050000500005000050000500005
910700000d6500c6210c6111f60018600146000060000600006000060000600006000060000600006000060000600006000060000600006000060000600006000060000600006000060000600006000060000600
351000000145300403004030040300403004030040300403004030040300403004030040300403004030040300403004030040300403004030040300403004030040300403004030040300403004030040300403
010400002364007640076300662105621046110461103611006000d7011350100501005010c601125010050100501005010050100501005010050100501005010050100000000000000000000000000000000000
01181000050200c020050200c020050200c020050200c020050200c020050200c020050200c020050200c02000000000000000000000000000000000000000000000000000000000000000000000000000000000
1d08000400222092220022209222002220922200222092220d2020d2020d202192021920219202192021920219202192021920219202192021920219202192021920219202192021920219202192020020200202
0906000c267441b744267441d744267441f744267442274426744247442674426744297042970430704307042d7042d7042d7042d7042d7042d7042d7042d7042d7042d7042d7042d7042d7042d7042d70400704
341c04000054702547035570455700527025270353704537005370253703537045370053702547035470454700547025570355704557005570255703557045570056702567035670456700567025670357704577
c1101800036201a63027640306403564037640366403664034640326402d6402564021630156200a610026101d6001d6001d6001d6001d6001d6001d600180110060000600006000060000600006000060000600
a9020405006200962010620186202b6202f620306202a6002a6002a6002a6001d6001d6001d6001d6001d6001d6001d6001d6001d6001d6001d6001d6001d6000060000600006000060000600006000060000600
0d0c00001d74020740227402474018700247402274022705247402474518700247402274024700247402270524740247452470024740227402470024740227452474018700227400000020740207412274122741
0d0c000024740247402470022740000002274020740227052274022740247002274000000207401d740227052274022740247002275000000207501d750227052275018700207702070020770207611d7511d751
910c00080532000300053200030005320053200030005320053200030005320003000030000300003000030000300003000030000300003000030000000000000000000000000000000000000000000000000000
910c00002405024050240402404024040240302403024020240202401024010000000000000000000000000000000000000000000000000000000000000000000000000000000000000024060270612906130061
01180002110300a030050300b030050300c030050300c030000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
011800100554005545085400c540055400554508540075400a5400a5450a540085400854508540055400c54014540145401854014540135401354018540145401454018540145401354014540145401854014540
090c00001a5501a5301a5501a5301a5501a5301a5401a5201a5401a5201a5301a5101a5201a5101a5101a51000500005000050000500005000050000500005001156011550115301152014560145501453014520
210c00000000000000180751c0001800018000180651a0001800018000180551f000180001800018045130001300018000180451f000180001800018035130001000018000180351f00018000180001802513000
011800100c5000c5500c5000c550055050c5500a550085550a55008550055500b5500a550055500b5500555013550185501455014550185501455013550145501455018550145500000000000000000000000000
090c00001a5501a5301a5501a5301a5501a5301a5401a5201a5401a5201a5301a5101a5201a5101a5101a51000000000000000000000000000000016560165600000000000165601656014560145611856018560
111600000075500755007550074500745007450073500735007350072500725007250071500715007150071508705037050870508705147051070508705087050070500705007050070500705007050070000700
000800002266106663036630265102651026410064100631006310063100621006210062100621006110061100611006010060500601006010060100601006010060100601006010060100601006010060100601
550e14001f55221552235522455223552245522655224552265522a552265522a5522b5522b5522b5522b5522b5522b5422b5422b512005020050200502005020050200502005020050200500005000050000500
900509000361004620056200361004610016100060000600006000060000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
150e00100c0500000000000000000a05000000000000005000000000000005000000000000a0500000000000000000000013050110501105011050010001105001000010000100011050000001f0501f05009000
010e00080e0530000018605000000c655180030000010605189000e05300000289502495018950000001065518950000000000000000000000000000000000000000000000000000000000000000000000000000
010e0010000500000000000000000c05000000000000a05000000000000905000000050500000007050000001b000000001b0001a0001b000000001800000000180001b050290000000000000000002b00000000
910e00040e0330360500655006050c635180031863510605189000e05300000289502495018950000001065518950000000000000000000000000000000000000000000000000000000000000000000000000000
010c00002d0502805024050230502b0502605022050210502905024050200501f05028050230501f0501c0501c0521c0521c0521c0521c0521c0521c0521c0520000200000000000000000000000000000000000
010c00001a0521c0521c0521c052000021c05200002000021c05200002000021c052000020000200002000021a0521c0521c0521c052000021c05200002000021c05200002000021c05200002000020000200002
320804003a6603a6603a6603a6603a6603a6603a6603a6703a6703a6703a6703a6703a6603a6603a6603a6603b660046400463003630036100261001610016110165000600006000060000600006000060000600
090622002435024340243302432524350243402433024325243502434024340243302432024320243102431524306183010030000300003000030000300003000030000300003000030018001003000030618011
480f18000527003261022510125100241002410023000230246702466024640246200c0730c3000930000300246702466024640246200c0730c30009300003000c30021700092000c3000d6000c3000930000300
090e100013250112510f2510c251112500f2510d2510a2510e2500c2510a25107251092500725105251022510c261002011826100201002010020100201002010020100201002010020100201002010020100201
510e001005370000001826303360196750c3000c3630c37000073247000c2330f36010665000000f3600c35000705187450070500705007050070500705007050070500705007050070500705007050070500000
510e00080c0730000018233003001a6550c3000c3630030300073247000c2330f36310665000000f3600c35000705187450070500705007050070500705007050070500705007050070500705007050070500000
480e0008110531d603110530500035633053530535305000186050c0630c0632470024700247001866500705181630c1630070518765007050070500705007050070500705007050070500705007050070500000
4d0e1000143401434014340143400f3600f3600f3600f3600a3600a3600a3600a360033720337203372033720c163007051876500705007050070500705007050070500705007050070500705007050000000000
990e200005350000001823303330196450c3000c3030c30005370000001826303360196750c3000c3030c30005370000001826303360053700000018263033600537000000182630336005370000001826303360
490e002005270032610225101251002410024100230003700c373217000925300350016550035009353003330c373217000925300350016550035009353003330c37321700092530035001655003500935300333
490e00080c37321700092530c3530d6000c35309353003330c37321700092530c3500d6550c35009353003330c37321700092530c3500d6550c35009353003330c37321700092530c3500d6550c3500935300333
490e20000c37321700092530c3530c37321700092530c3530c37321700092530c3530c37321700092530c3530c373217000c373217000c373217000c373217000c3730c3730c3730c3730c3730c3730c3730c373
__music__
01 12135644
00 12135044
00 12135044
00 12135044
01 191f1344
00 20191344
00 21195044
00 21191244
00 22231244
00 22231244
00 1f135644
00 22231244
00 22231244
00 21195044
00 21191244
00 24255944
00 24252644
00 27251244
02 27281244
00 41424344
00 41424344
00 41424344
00 41424344
00 41424344
01 36424344
00 36424344
01 3b424344
01 38424344
01 38424344
00 38424344
00 3b424344
00 37764344
00 37764344
00 3c764344
00 3c764344
00 3c764344
00 3c764344
00 3d764344
02 3e764344
02 3e764344
00 3d764344
02 3d764344
00 692a4344
02 692a4344
00 41424344
00 41424344
00 41424344
00 41424344
01 6b2b4344
01 2d2e4344
00 2d2e4344
00 2f304344
02 2f304344
00 41424344
00 41424344
00 41424344
01 35424344
00 35424344
00 36424344
00 36424344
00 37424344
00 37424344
00 36384344
02 36384344

