#[cfg(feature = "unicorn_plugin_lua")]
pub mod plugin {
    use std::sync::{Arc, Mutex};

    use rand;
    use rand::Rng;

    use unicorn_plugin_lua as lua;
    use unicorn_plugin_lua::ffi::lua_State;
    use unicorn_plugin_lua::{State, Function, ThreadStatus};
    use libc::c_int;

    use config::Players;

    use unicorn::info::Info;
    use unicorn::noise::Noise;
    use sound::sound::Sound;

    use gfx::Screen;

    pub struct ExtraData {
        /* External objects */
        pub players: Arc<Mutex<Players>>,
        pub screen: Arc<Mutex<Screen>>,
        pub info: Arc<Mutex<Info>>,
        pub noise: Arc<Mutex<Noise>>,
        pub sound: Arc<Mutex<Sound>>,
    }

    pub struct LuaPlugin {
        lua_state: Arc<Mutex<lua::State>>,
        loaded_code: bool,
    }

    impl LuaPlugin {
        pub fn new() -> LuaPlugin {
            LuaPlugin {
                lua_state: Arc::new(Mutex::new(lua::State::new())),
                loaded_code: false,
            }
        }

        #[allow(unused)]
        pub fn load(&mut self,
                    players: Arc<Mutex<Players>>,
                    info: Arc<Mutex<Info>>,
                    screen: Arc<Mutex<Screen>>,
                    noise: Arc<Mutex<Noise>>,
                    sound: Arc<Mutex<Sound>>) {
            info!("[PLUGIN][LUA] Init plugin");

            let extra = ExtraData {
                players: players.clone(),
                info: info.clone(),
                screen: screen.clone(),
                noise: noise.clone(),
                sound: sound.clone(),
            };

            let mut lua_state = self.lua_state.lock().unwrap();
            lua_state.open_libs();
            lua_state.set_extra(Some(Box::new(extra)));

            lua_state.new_table();
            lua_state.set_fns(&UNICORN_LUA_LIB, 0);

            lua_state.push_value(-1);
            lua_state.set_global("UnicornLua");

            lua_state.new_metatable("UnicornLua");

            lua_state.push_value(-2);
            lua_state.set_field(-2, "__index");

            lua_state.pop(2);

            /* Create the UnicornLua object */
            let value = lua_state.do_string("UnicornObject = UnicornLua.new()");
            info!("[PLUGIN][LUA][Unicorn][CREATE Unicorn OBJECT] = {:?}", value);

            let value = lua_state.do_string(r#"debug_print = print"#);
            info!("[PLUGIN][LUA][Unicorn][EXPORT DEBUG PRINT FUNCTION] = {:?}", value);

            /* Audio */
            let value = lua_state.do_string(r#"chiptune_play = function(filetype, filename, loops, start_position, channel)
              if start_position == nil then
                start_position = 0
              end

              if loops == nil then
                loops = 0
              end

              if channel == nil then
                channel = -1
              end

              UnicornObject:chiptune_play(filetype, filename, loops, start_position, channel)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][SOUND_PLAY] = {:?}", value);

            let value = lua_state.do_string(r#"camera = function(x, y)

              x = math.floor(x)
              y = math.floor(y)

              UnicornObject:camera(x, y)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][CAMERA] = {:?}", value);

            let value = lua_state.do_string(r#"btn = function(x, p)

              x = math.floor(x)

              if p == nil then
                p = 0
              end

              return UnicornObject:btn(p, x) == 1
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][TBN] = {:?}", value);

            let value = lua_state.do_string(r#"btnp = function(x, p)

              x = math.floor(x)

              if p == nil then
                p = 0
              end

              return UnicornObject:btnp(p, x) == 1
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][BTNP] = {:?}", value);


            let value = lua_state.do_string(r#"rect = function(x0, y0, x1, y1, color)

              x0 = math.floor(x0)
              y0 = math.floor(y0)
              x1 = math.floor(x1)
              y1 = math.floor(y1)

              if color == nil then
                color = -1
              end

              color = math.floor(color)

              UnicornObject:rect(x0, y0, x1, y1, color)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][RECT] = {:?}", value);

            let value = lua_state.do_string(r#"rectfill = function(x0, y0, x1, y1, color)

              x0 = math.floor(x0)
              y0 = math.floor(y0)
              x1 = math.floor(x1)
              y1 = math.floor(y1)

              if color == nil then
                color = -1
              end

              color = math.floor(color)

              UnicornObject:rectfill(x0, y0, x1, y1, color)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][RECTFILL] = {:?}", value);

            let value = lua_state.do_string(r#"circ = function(x, y, r, color)
              x = math.floor(x)
              y = math.floor(y)
              r = math.floor(r)

              if color == nil then
                color = -1
              end

              color = math.floor(color)

              UnicornObject:circ(x, y, r, color)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][CIRC] = {:?}", value);

            let value = lua_state.do_string(r#"circfill = function(x, y, r, color)
              x = math.floor(x)
              y = math.floor(y)
              r = math.floor(r)

              if color == nil then
                color = -1
              end

              color = math.floor(color)

              UnicornObject:circfill(x, y, r, color)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][CIRCFILL] = {:?}", value);

            let value = lua_state.do_string(r#"clip = function(x, y, w, h)
              if x == nil then
                x = -1
              end
              if y == nil then
                y = -1
              end
              if w == nil then
                w = -1
              end
              if h == nil then
                h = -1
              end

              x = math.floor(x)
              y = math.floor(y)
              w = math.floor(w)
              h = math.floor(h)

              UnicornObject:clip(x, y, w, h)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][CLIP] = {:?}", value);

            let value = lua_state.do_string(r#"ellipse = function(x, y, rx, ry, color)
              x = math.floor(x)
              y = math.floor(y)
              rx = math.floor(rx)
              ry = math.floor(ry)

              if color == nil then
                color = -1
              end

              color = math.floor(color)

              UnicornObject:ellipse(x, y, rx, ry, color)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][ELLIPSE] = {:?}", value);

            let value = lua_state.do_string(r#"ellipsefill = function(x, y, rx, ry, color)
              x = math.floor(x)
              y = math.floor(y)
              rx = math.floor(rx)
              ry = math.floor(ry)

              if color == nil then
                color = -1
              end

              color = math.floor(color)

              UnicornObject:ellipsefill(x, y, rx, ry, color)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][ELLIPSEFILL] = {:?}", value);

            let value = lua_state.do_string(r#"fget = function(idx, flag)
              idx = math.floor(idx)
              flag = math.floor(flag)

              if flag == nil then
                return UnicornObject:fget_all(idx)
              end

              return UnicornObject:fget(idx, flag)

              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][FGET] = {:?}", value);

            let value = lua_state.do_string(r#"fset = function(idx, flag, value)
              idx = math.floor(idx)
              flag = math.floor(flag)

              if value == nil then
                UnicornObject:fset_all(idx, flag)
              else
                if value == true then
                    UnicornObject:fset(idx, flag, 1)
                else
                    UnicornObject:fset(idx, flag, 0)
                end
              end

              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][FSET] = {:?}", value);

            let value = lua_state.do_string(r#"line = function(x0, y0, x1, y1, color)

              x0 = math.floor(x0)
              y0 = math.floor(y0)
              x1 = math.floor(x1)
              y1 = math.floor(y1)

              if color == nil then
                color = -1
              end

              color = math.floor(color)

              UnicornObject:line(x0, y0, x1, y1, color)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][LINE] = {:?}", value);

            let value = lua_state.do_string(r#"trigon = function(x1, y1, x2, y2, x3, y3, color)
              x1 = math.floor(x1)
              y1 = math.floor(y1)
              x2 = math.floor(x2)
              y2 = math.floor(y2)
              x3 = math.floor(x3)
              y3 = math.floor(y3)

              if color == nil then
                color = -1
              end

              color = math.floor(color)

              UnicornObject:trigon(x1, y1, x2, y2, x3, y3, color)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][TRIGON] = {:?}", value);

            let value = lua_state.do_string(r#"rnd = function(x)
              if x == nil then
                x = 1
              end

              x = math.floor(x)
              return UnicornObject:rnd(x)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][RND] = {:?}", value);

            let value = lua_state.do_string(r#"add = function(t, v)
              t[#t+1] = v
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][ADD] = {:?}", value);

            let value = lua_state.do_string(r#"cls = function(v)
              if v == nil then
                v = -1
              end

              UnicornObject:cls(v)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][CLS] = {:?}", value);


            let value = lua_state.do_string(r#"palt = function(c, t)
              c = math.floor(c)

              if t == true then
                t = 1
              else
                t = 0
              end

              UnicornObject:palt(c, t)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][PALT] = {:?}", value);

            let value = lua_state.do_string(r#"pal = function(c0, c1, p)
              if c0 == nil then
                c0 = -1
              end

              if c1 == nil then
                c1 = -1
              end

              if p == nil then
                p = -1
              end

              UnicornObject:pal(c0, c1, p)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][PAL] = {:?}", value);

            let value = lua_state.do_string(r#"font = function(name)

              if name == nil then
                name = "pico8"
              end

              UnicornObject:font(name)

              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][FONT] = {:?}", value);

            let value = lua_state.do_string(r#"pset = function(x, y, color)
              x = math.floor(x)
              y = math.floor(y)

              if color == nil then
                color = -1
              end

              color = math.floor(color)

              UnicornObject:pset(x, y, color)

              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][PSET] = {:?}", value);

            let value = lua_state.do_string(r#"pget = function(x, y)
              x = math.floor(x)
              y = math.floor(y)

              return UnicornObject:pget(x, y)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][PGET] = {:?}", value);

            let value = lua_state.do_string(r#"sget = function(x, y)
              x = math.floor(x)
              y = math.floor(y)

              return UnicornObject:sget(x, y)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][SGET] = {:?}", value);

            let value = lua_state.do_string(r#"sset = function(x, y, color)
              x = math.floor(x)
              y = math.floor(y)

              if color == nil then
                color = -1
              end

              color = math.floor(color)

              UnicornObject:sset(x, y, c)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][SSET] = {:?}", value);

            let value = lua_state.do_string(r#"noise = function(x, y, z)
              return UnicornObject:noise(x, y, z)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][NOISE] = {:?}", value);

            let value = lua_state.do_string(r#"noise_set_seed = function(seed)
              return UnicornObject:noise_set_seed(seed)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][NOISE_SET_SEED] = {:?}", value);

            let value = lua_state.do_string(r#"map = function(cel_x, cel_y, sx, sy, cel_w, cel_h, layer)

              cel_x = math.floor(cel_x)
              cel_y = math.floor(cel_y)
              sx = math.floor(sx)
              sy = math.floor(sy)
              cel_w = math.floor(cel_w)
              cel_h = math.floor(cel_h)

              if layer == nil then
                layer = 0
              end

              UnicornObject:map(cel_x, cel_y, sx, sy, cel_w, cel_h, layer)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][MAP] = {:?}", value);

            let value = lua_state.do_string(r#"mapdraw = function(cel_x, cel_y, sx, sy, cel_w, cel_h, layer)
                map(cel_x, cel_y, sx, sy, cel_w, cel_h, layer)
                end
                "#);
            info!("[PLUGIN][LUA][Unicorn][MAPDRAW] = {:?}", value);

            let value = lua_state.do_string(r#"mget = function(x, y)
              x = math.floor(x)
              y = math.floor(y)

              return UnicornObject:mget(x, y)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][MGET] = {:?}", value);

            let value = lua_state.do_string(r#"mset = function(x, y, v)
              x = math.floor(x)
              y = math.floor(y)
              v = math.floor(v)

              UnicornObject:mset(x, y, v)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][MSET] = {:?}", value);

            let value = lua_state.do_string(r#"spr = function(n, x, y, w, h, flip_x, flip_y)

              n = math.floor(n)
              x = math.floor(x)
              y = math.floor(y)
              if w == nil then
                w = 1
              end
              if h == nil then
                h = 1
              end
              if flip_x == nil then
                flip_x = false
              end
              if flip_y == nil then
                flip_y = false
              end

              if flip_x == true then
                flip_x = 1
              else
                flip_x = 0
              end

              if flip_y == true then
                flip_y = 1
              else
                flip_y = 0
              end

              UnicornObject:spr(n, x, y, w, h, flip_x, flip_y)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][SPR] = {:?}", value);

            let value = lua_state.do_string(r#"sspr = function(sx, sy, sw, sh, dx, dy, dw, dh, flip_x, flip_y)
              sx = math.floor(sx)
              sy = math.floor(sy)
              sw = math.floor(sw)
              sh = math.floor(sh)
              dx = math.floor(dx)
              dy = math.floor(dy)

              if dw == nil then
                dw = sw
              end
              if dh == nil then
                dh = sh
              end
              if flip_x == nil then
                flip_x = false
              end
              if flip_y == nil then
                flip_y = false
              end

              if flip_x == true then
                flip_x = 1
              else
                flip_x = 0
              end

              if flip_y == true then
                flip_y = 1
              else
                flip_y = 0
              end

              UnicornObject:sspr(sx, sy, sw, sh, dx, dy, dw, dh, flip_x, flip_y)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][SSPR] = {:?}", value);

            let value = lua_state.do_string(r#"print = function(str, x, y, col)
              if x == nil then
                x = -1
              end

              if y == nil then
                y = -1
              end

              if col == nil then
                col = -1
              end

              x = math.floor(x)
              y = math.floor(y)
              col = math.floor(col)

              UnicornObject:print(str, x, y, col)

              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][PRINT] = {:?}", value);

            let value = lua_state.do_string(r#"time = function()
                return UnicornObject:time()
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][TIME] = {:?}", value);

            let value = lua_state.do_string(r#"sfx = function(n, channel, offset)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][SFX] = {:?}", value);

            let value = lua_state.do_string(r#"music = function(id, filename, channel, loops, start_position)
              if filename == nil then
                filename = ""
              end

              if channel == nil then
                channel = -1
              end

              if loops == nil then
                loops = -1
              end

              if start_position == nil then
                start_position = 0
              end

              UnicornObject:music(id, filename, channel, loops, start_position)

              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][MUSIC] = {:?}", value);

            let value = lua_state.do_string(r#"sfx = function(id, filename, channel, note, panning, rate, loops)
              if filename == nil then
                filename = ""
              end

              if channel == nil then
                channel = -1
              end

              if note == nil then
                note = 13312
              end

              if panning == nil then
                panning = 64
              end

              if rate == nil then
                rate = 50
              end

              if loops == nil then
                loops = -1
              end
              
              UnicornObject:sfx(id, filename, channel, note, panning, rate, loops)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][SFX] = {:?}", value);


            let value = lua_state.do_string(r#"flip = function()
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][FLIP] = {:?}", value);

            let value = lua_state.do_string(r#"color = function(c)
                c = math.floor(c)
                UnicornObject:color(c)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][COLOR] = {:?}", value);

            let value = lua_state.do_string(r#"peek = function(addr)
                return 0
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][PEEK] = {:?}", value);

            let value = lua_state.do_string(r#"poke = function(addr, val)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][POKE] = {:?}", value);

            let value = lua_state.do_string(r#"stat = function(x)
                v = UnicornObject:stat(x)
                return v
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][STAT] = {:?}", value);

            /* CARTDATA */
            let value = lua_state.do_string(r#"cartdata = function(x)
              x = math.floor(x)
              UnicornObject:cartdata(x)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][CARTDATA] = {:?}", value);

            let value = lua_state.do_string(r#"dget = function(x)
              x = math.floor(x)
              return UnicornObject:dget(x)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][DGET] = {:?}", value);

            let value = lua_state.do_string(r#"dset = function(x, y)
              x = math.floor(x)
              y = math.floor(y)

              UnicornObject:dset(x, y)
              end
              "#);
            info!("[PLUGIN][LUA][Unicorn][DSET] = {:?}", value);

            /* PICO8 compatible functions */

            let value = lua_state.do_string(r#"
              function warning(msg)
                log(debug.traceback("WARNING: "..msg,3))
              end

              function add(a,v)
                if a == nil then
                  warning("add to nil")
                  return
                end
                table.insert(a,v)
              end

              function del(a,dv)
                if a == nil then
                  warning("del from nil")
                  return
                end
                for i,v in ipairs(a) do
                  if v==dv  then
                    table.remove(a,i)
                  end
                end
              end

              function foreach(a,f)
                if not a then
                  warning("foreach got a nil value")
                  return
                end
                for i,v in ipairs(a) do
                  f(v)
                end
              end

              function count(a)
                return #a
              end

              function all(a)
                local i = 0
                local n = #a
                return function()
                    i = i + 1
                    if i <= n  then
                        return a[i]
                    end
                end
              end

              sub = string.sub
              "#);
            info!("[PLUGIN][LUA] LOADED LUA FUNCTIONS = {:?}", value);

            let value = lua_state.do_string(r#"
            function min(a,b)
                if a == nil or b == nil then
                        warning("min a or b are nil returning 0")
                        return 0
                end
                if a < b then
                    return a
                end
                return b
            end

            function max(a,b)
                if a == nil or b == nil then
                        warning("max a or b are nil returning 0")
                        return 0
                end
                if a > b then
                    return a
                end
                return b
            end

            function mid(x,y,z)
                x = x or 0
                y = y or 0
                z = z or 0
                return x > y and x or y > z and z or y
            end
            function __pico_angle(a)
              -- FIXME: why does this work?
              return (((a - math.pi) / (math.pi*2)) + 0.25) % 1.0
            end

            flr = math.floor
            ceil = math.ceil
            cos = function(x) return math.cos((x or 0)*(math.pi*2)) end
            sin = function(x) return math.sin(-(x or 0)*(math.pi*2)) end
            atan2 = function(y,x) return __pico_angle(math.atan2(y,x)) end

            sqrt = math.sqrt
            abs = math.abs

            sgn = function(x)
                if x < 0 then
                    return -1
                else
                    return 1
                end
            end

            band = function(x, y)
              x = math.floor(x)
              y = math.floor(y)
              return x & y
            end

            bor = function(x, y)
              x = math.floor(x)
              y = math.floor(y)
              return x | y
            end
            bxor = function(x, y)
              x = math.floor(x)
              y = math.floor(y)
              return x ~ y
            end
            bnot = function(x)
              x = math.floor(x)
              return ~x
            end
            shl = function(x, y)
              x = math.floor(x)
              y = math.floor(y)
              return x << y
            end
            shr = function(x, y)
              x = math.floor(x)
              y = math.floor(y)
              return x >> y
            end
            "#);
            info!("[PLUGIN][LUA] LOADED MATH FUNCTIONS = {:?}", value);
        }

        pub fn init(&mut self) {
            if !self.loaded_code {
                return;
            }

            let mut lua_state = self.lua_state.lock().unwrap();

            let value = lua_state.do_string("_init()");
            if value != ThreadStatus::Ok {
                error!("[PLUGIN][LUA] INIT = {:?}", value);
            } else {
                info!("[PLUGIN][LUA] INIT SUCCESS");
            }
        }

        pub fn draw(&mut self) -> bool {
            if !self.loaded_code {
                return false;
            }

            let mut lua_state = self.lua_state.lock().unwrap();

            let value = lua_state.do_string("_draw()");
            if value != ThreadStatus::Ok {
                error!("[PLUGIN][LUA] DRAW = {:?}", value);
            }

            return true;
        }

        pub fn update(&mut self) -> bool {
            if !self.loaded_code {
                return false;
            }

            let mut lua_state = self.lua_state.lock().unwrap();

            let value = lua_state.do_string("_update()");
            if value != ThreadStatus::Ok {
                let value = lua_state.do_string("_update60()");
                if value != ThreadStatus::Ok {
                    error!("[PLUGIN][LUA] UPDATE = {:?}", value);
                }
            }

            return true;

        }

        pub fn load_code(&mut self, data: String) -> bool {
            info!("[PLUGIN][LUA] LOAD CODE");
            let mut lua_state = self.lua_state.lock().unwrap();

            self.loaded_code = true;

            let value = lua_state.do_string(&data);
            if value != ThreadStatus::Ok {
                error!("[PLUGIN][LUA] LOAD CODE = {:?}", value);
                self.loaded_code = false;
            }

            self.loaded_code
        }
    }

    struct UnicornLua {}

    impl UnicornLua {
        fn new() -> UnicornLua {
            return UnicornLua {};
        }

        #[allow(non_snake_case)]
        unsafe extern "C" fn lua_new(lua_context: *mut lua_State) -> c_int {
            let mut state = State::from_ptr(lua_context);

            // construct new userdata in lua space and initialize it
            *state.new_userdata_typed::<UnicornLua>() = UnicornLua::new();
            // set the userdata's metatable so we can call methods on it
            state.set_metatable_from_registry("UnicornLua");
            // return the userdata on top of the stack
            1
        }

        unsafe extern "C" fn lua_chiptune_music(lua_context: *mut lua_State) -> c_int {
            debug!("LUA CHIPTUNE MUSIC");

            let mut state = State::from_ptr(lua_context);
            let mut state2 = State::from_ptr(lua_context);

            let id = state2.check_integer(2);
            let filename = state.check_string(3);
            let loops = state2.check_integer(4);
            let start_position = state2.check_integer(5);
            let channel = state2.check_integer(6);

            let sound = state2.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.sound.clone()
                                          });

            sound
                .lock()
                .unwrap()
                .music(id as i32, filename.to_string(), channel as i32, loops as i32, start_position as i32);

            1
        }

        unsafe extern "C" fn lua_chiptune_sfx(lua_context: *mut lua_State) -> c_int {
            debug!("LUA CHIPTUNE SFX");

            let mut state = State::from_ptr(lua_context);
            let mut state2 = State::from_ptr(lua_context);

            let id = state2.check_integer(2);
            let filename = state.check_string(3);
            let channel = state2.check_integer(4);
            let note = state2.check_integer(5);
            let panning = state2.check_integer(6);
            let rate = state2.check_integer(7);
            let loops = state2.check_integer(8);

            let sound = state2.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.sound.clone()
                                          });

            sound
                .lock()
                .unwrap()
                .sfx(id as i32, filename.to_string(), channel as i32, note as u16, panning as i32, rate as i32, loops as i32);

            1
        }

        unsafe extern "C" fn lua_camera(lua_context: *mut lua_State) -> c_int {
            debug!("LUA CAMERA");

            let mut state = State::from_ptr(lua_context);

            let x = state.check_integer(2);
            let y = state.check_integer(3);

            let screen = state.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.screen.clone()
                                          });

            screen.lock().unwrap().camera(x as i32, y as i32);

            1
        }

        unsafe extern "C" fn lua_color(lua_context: *mut lua_State) -> c_int {
            debug!("LUA COLOR");

            let mut state = State::from_ptr(lua_context);

            let value = state.check_integer(2);

            let screen = state.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.screen.clone()
                                          });

            screen.lock().unwrap().color(value as i32);

            1
        }

        unsafe extern "C" fn lua_btn(lua_context: *mut lua_State) -> c_int {
            debug!("LUA BTN");

            let mut state = State::from_ptr(lua_context);
            let mut state2 = State::from_ptr(lua_context);

            let players = state2.with_extra(|extra| {
                                                let data = extra
                                                    .as_ref()
                                                    .unwrap()
                                                    .downcast_ref::<ExtraData>()
                                                    .unwrap();
                                                data.players.clone()
                                            });

            let player = state.check_integer(2);

            let i = state.check_integer(3);

            let players_data = players.lock().unwrap();

            let value = players_data.get_value(player as u8, i as u8);

            state.push_integer(value as i64);

            1
        }


        unsafe extern "C" fn lua_btnp(lua_context: *mut lua_State) -> c_int {
            debug!("LUA BTN");

            let mut state = State::from_ptr(lua_context);
            let mut state2 = State::from_ptr(lua_context);

            let players = state2.with_extra(|extra| {
                                                let data = extra
                                                    .as_ref()
                                                    .unwrap()
                                                    .downcast_ref::<ExtraData>()
                                                    .unwrap();
                                                data.players.clone()
                                            });

            let player = state.check_integer(2);
            let i = state.check_integer(3);

            let mut players_data = players.lock().unwrap();

            let value = players_data.get_value_quick(player as u8, i as u8);

            state.push_integer(value as i64);

            1
        }

        unsafe extern "C" fn lua_cls(lua_context: *mut lua_State) -> c_int {
            let mut state = State::from_ptr(lua_context);

            let value = state.check_integer(2);

            let screen = state.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.screen.clone()
                                          });

            screen.lock().unwrap().cls(value as i8);

            1
        }

        #[allow(non_snake_case)]
        unsafe extern "C" fn lua_rect(lua_context: *mut lua_State) -> c_int {
            debug!("LUA RECT");

            let mut state = State::from_ptr(lua_context);

            let x0 = state.check_integer(2);
            let y0 = state.check_integer(3);
            let x1 = state.check_integer(4);
            let y1 = state.check_integer(5);
            let col = state.check_integer(6);

            let screen = state.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.screen.clone()
                                          });

            screen
                .lock()
                .unwrap()
                .rect(x0 as i32, y0 as i32, x1 as i32, y1 as i32, col as i32);

            1
        }

        #[allow(non_snake_case)]
        unsafe extern "C" fn lua_rectfill(lua_context: *mut lua_State) -> c_int {
            debug!("LUA RECTFILL");

            let mut state = State::from_ptr(lua_context);

            let x0 = state.check_integer(2);
            let y0 = state.check_integer(3);
            let x1 = state.check_integer(4);
            let y1 = state.check_integer(5);
            let col = state.check_integer(6);

            debug!("LUA RECTFILL x0:{:?} y0:{:?} x1:{:?} y1:{:?} col:{:?}",
                   x0,
                   y0,
                   x1,
                   y1,
                   col);

            let screen = state.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.screen.clone()
                                          });
            screen
                .lock()
                .unwrap()
                .rectfill(x0 as i32, y0 as i32, x1 as i32, y1 as i32, col as i32);

            1
        }

        #[allow(non_snake_case)]
        unsafe extern "C" fn lua_circ(lua_context: *mut lua_State) -> c_int {
            debug!("LUA CIRC");

            let mut state = State::from_ptr(lua_context);

            let x = state.check_integer(2);
            let y = state.check_integer(3);
            let r = state.check_integer(4);
            let col = state.check_integer(5);

            let screen = state.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.screen.clone()
                                          });

            screen
                .lock()
                .unwrap()
                .circ(x as i32, y as i32, r as i32, col as i32);

            1
        }

        #[allow(non_snake_case)]
        unsafe extern "C" fn lua_circfill(lua_context: *mut lua_State) -> c_int {
            debug!("LUA CIRCFILL");

            let mut state = State::from_ptr(lua_context);

            let x = state.check_integer(2);
            let y = state.check_integer(3);
            let r = state.check_integer(4);
            let col = state.check_integer(5);

            debug!("LUA CIRCFILL {:?} {:?} {:?} {:?}", x, y, r, col);

            let screen = state.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.screen.clone()
                                          });

            screen
                .lock()
                .unwrap()
                .circfill(x as i32, y as i32, r as i32, col as i32);

            1
        }

        #[allow(non_snake_case)]
        unsafe extern "C" fn lua_clip(lua_context: *mut lua_State) -> c_int {
            debug!("LUA CLIP");

            let mut state = State::from_ptr(lua_context);

            let x = state.check_integer(2);
            let y = state.check_integer(3);
            let w = state.check_integer(4);
            let h = state.check_integer(5);

            debug!("LUA CLIP {:?} {:?} {:?} {:?}", x, y, w, h);

            let screen = state.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.screen.clone()
                                          });

            screen
                .lock()
                .unwrap()
                .clip(x as i32, y as i32, w as i32, h as i32);

            1
        }

        #[allow(non_snake_case)]
        unsafe extern "C" fn lua_ellipse(lua_context: *mut lua_State) -> c_int {
            debug!("LUA ELLIPSE");

            let mut state = State::from_ptr(lua_context);

            let x = state.check_integer(2);
            let y = state.check_integer(3);
            let rx = state.check_integer(4);
            let ry = state.check_integer(5);
            let col = state.check_integer(6);

            debug!("LUA ELLIPSE x:{:?} y:{:?} rx:{:?} ry:{:?} col:{:?}",
                   x,
                   y,
                   rx,
                   ry,
                   col);

            let screen = state.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.screen.clone()
                                          });
            screen
                .lock()
                .unwrap()
                .ellipse(x as i32, y as i32, rx as i32, ry as i32, col as i32);

            1
        }

        #[allow(non_snake_case)]
        unsafe extern "C" fn lua_ellipsefill(lua_context: *mut lua_State) -> c_int {
            debug!("LUA ELLIPSEFILL");

            let mut state = State::from_ptr(lua_context);

            let x = state.check_integer(2);
            let y = state.check_integer(3);
            let rx = state.check_integer(4);
            let ry = state.check_integer(5);
            let col = state.check_integer(6);

            debug!("LUA ELLIPSEFILL x:{:?} y:{:?} rx:{:?} ry:{:?} col:{:?}",
                   x,
                   y,
                   rx,
                   ry,
                   col);

            let screen = state.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.screen.clone()
                                          });
            screen
                .lock()
                .unwrap()
                .ellipsefill(x as i32, y as i32, rx as i32, ry as i32, col as i32);

            1
        }


        #[allow(non_snake_case)]
        unsafe extern "C" fn lua_trigon(lua_context: *mut lua_State) -> c_int {
            debug!("LUA TRIGON");

            let mut state = State::from_ptr(lua_context);

            let x1 = state.check_integer(2);
            let y1 = state.check_integer(3);
            let x2 = state.check_integer(4);
            let y2 = state.check_integer(5);
            let x3 = state.check_integer(6);
            let y3 = state.check_integer(7);
            let col = state.check_integer(8);

            debug!("LUA TRIGON x1:{:?} y1:{:?} x2:{:?} y2:{:?} x3:{:?} y3:{:?} col:{:?}",
                   x1,
                   y1,
                   x2,
                   y2,
                   x3,
                   y3,
                   col);

            let screen = state.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.screen.clone()
                                          });
            screen
                .lock()
                .unwrap()
                .trigon(x1 as i32,
                        y1 as i32,
                        x2 as i32,
                        y2 as i32,
                        x3 as i32,
                        y3 as i32,
                        col as i32);

            1
        }

        #[allow(non_snake_case)]
        unsafe extern "C" fn lua_palt(lua_context: *mut lua_State) -> c_int {
            debug!("LUA PALT");

            let mut state = State::from_ptr(lua_context);

            let c = state.check_integer(2);
            let t = state.check_integer(3);

            debug!("LUA PALT {:?} {:?}", c, t);

            let screen = state.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.screen.clone()
                                          });

            screen.lock().unwrap().palt(c as i32, t == 1);

            1
        }


        unsafe extern "C" fn lua_pal(lua_context: *mut lua_State) -> c_int {
            debug!("LUA PAL");

            let mut state = State::from_ptr(lua_context);

            let c0 = state.check_integer(2);
            let c1 = state.check_integer(3);
            //            let p = state.check_integer(4);

            let screen = state.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.screen.clone()
                                          });

            screen.lock().unwrap().pal(c0 as i32, c1 as i32);

            1
        }

        unsafe extern "C" fn lua_pset(lua_context: *mut lua_State) -> c_int {
            debug!("LUA PSET");

            let mut state = State::from_ptr(lua_context);

            let x = state.check_integer(2);
            let y = state.check_integer(3);
            let col = state.check_integer(4);

            if x < 0 || y < 0 {
                return 1;
            }

            let screen = state.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.screen.clone()
                                          });

            screen.lock().unwrap().pset(x as i32, y as i32, col as i32);

            1
        }

        unsafe extern "C" fn lua_pget(lua_context: *mut lua_State) -> c_int {
            debug!("LUA PGET");

            let mut state = State::from_ptr(lua_context);

            let x = state.check_integer(2);
            let y = state.check_integer(3);

            if x < 0 || y < 0 {
                return 1;
            }

            let screen = state.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.screen.clone()
                                          });

            let value = screen.lock().unwrap().pget(x as u32, y as u32);

            state.push_integer(value as i64);

            1
        }


        unsafe extern "C" fn lua_sget(lua_context: *mut lua_State) -> c_int {
            debug!("LUA SGET");

            let mut state = State::from_ptr(lua_context);

            let x = state.check_integer(2);
            let y = state.check_integer(3);

            debug!("LUA SGET {:?} {:?}", x, y);

            if x < 0 || y < 0 {
                return 1;
            }

            if x as usize >= 128 || y as usize >= 128 {
                return 1;
            }

            let screen = state.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.screen.clone()
                                          });

            let value = screen.lock().unwrap().sget(x as u32, y as u32);

            state.push_integer(value as i64);

            1
        }

        unsafe extern "C" fn lua_sset(lua_context: *mut lua_State) -> c_int {
            debug!("LUA SSET");

            let mut state = State::from_ptr(lua_context);

            let x = state.check_integer(2);
            let y = state.check_integer(3);
            let col = state.check_integer(4);

            debug!("LUA SSET {:?} {:?} {:?}", x, y, col);

            if x < 0 || y < 0 {
                return 1;
            }

            if x as usize >= 128 || y as usize >= 128 {
                return 1;
            }

            let screen = state.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.screen.clone()
                                          });

            screen.lock().unwrap().sset(x as u32, y as u32, col as i32);

            1
        }

        unsafe extern "C" fn lua_noise(lua_context: *mut lua_State) -> c_int {
            debug!("LUA NOISE");

            let mut state = State::from_ptr(lua_context);

            let x = state.check_number(2);
            let y = state.check_number(3);
            let z = state.check_number(4);

            debug!("LUA NOISE {:?} {:?} {:?}", x, y, z);

            let noise = state.with_extra(|extra| {
                                             let data = extra
                                                 .as_ref()
                                                 .unwrap()
                                                 .downcast_ref::<ExtraData>()
                                                 .unwrap();
                                             data.noise.clone()
                                         });

            let value = noise.lock().unwrap().get(x, y, z);
            state.push_number(value);

            1
        }

        unsafe extern "C" fn lua_noise_set_seed(lua_context: *mut lua_State) -> c_int {
            debug!("LUA NOISE SET SEED");

            let mut state = State::from_ptr(lua_context);

            let seed = state.check_integer(2);

            debug!("LUA NOISE SET SEED {:?}", seed);

            let noise = state.with_extra(|extra| {
                                             let data = extra
                                                 .as_ref()
                                                 .unwrap()
                                                 .downcast_ref::<ExtraData>()
                                                 .unwrap();
                                             data.noise.clone()
                                         });

            noise.lock().unwrap().set_seed(seed as u32);

            1
        }

        unsafe extern "C" fn lua_line(lua_context: *mut lua_State) -> c_int {
            debug!("LUA LINE");

            let mut state = State::from_ptr(lua_context);

            let x0 = state.check_integer(2);
            let y0 = state.check_integer(3);
            let x1 = state.check_integer(4);
            let y1 = state.check_integer(5);
            let col = state.check_integer(6);

            let screen = state.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.screen.clone()
                                          });

            screen
                .lock()
                .unwrap()
                .line(x0 as i32, y0 as i32, x1 as i32, y1 as i32, col as i32);

            1
        }

        unsafe extern "C" fn lua_fget(lua_context: *mut lua_State) -> c_int {
            debug!("LUA FGET");

            let mut state = State::from_ptr(lua_context);

            let idx = state.check_integer(2);
            let flag = state.check_integer(3);

            let screen = state.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.screen.clone()
                                          });

            state.push_bool(screen.lock().unwrap().fget(idx as u32, flag as u8));

            1
        }

        unsafe extern "C" fn lua_fget_all(lua_context: *mut lua_State) -> c_int {
            debug!("LUA FGET ALL");

            let mut state = State::from_ptr(lua_context);

            let idx = state.check_integer(2);

            let screen = state.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.screen.clone()
                                          });

            state.push_integer(screen.lock().unwrap().fget_all(idx as u32) as i64);

            1
        }

        unsafe extern "C" fn lua_fset(lua_context: *mut lua_State) -> c_int {
            debug!("LUA FSET");

            let mut state = State::from_ptr(lua_context);

            let idx = state.check_integer(2);
            let flag = state.check_integer(3);
            let value = state.check_integer(4);

            let screen = state.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.screen.clone()
                                          });

            screen
                .lock()
                .unwrap()
                .fset(idx as u32, flag as u8, value == 1);

            1
        }

        unsafe extern "C" fn lua_fset_all(lua_context: *mut lua_State) -> c_int {
            debug!("LUA FSET ALL");

            let mut state = State::from_ptr(lua_context);

            let idx = state.check_integer(2);
            let flags = state.check_integer(3);

            let screen = state.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.screen.clone()
                                          });

            screen.lock().unwrap().fset_all(idx as u32, flags as u8);

            1
        }

        unsafe extern "C" fn lua_rnd(lua_context: *mut lua_State) -> c_int {
            debug!("LUA RND");

            let mut state = State::from_ptr(lua_context);

            let x = state.check_integer(2);

            let value: f64;

            if x == 0 {
                value = rand::thread_rng().gen_range(0.0, 1.0);
            } else {
                value = rand::thread_rng().gen_range(0.0, x as f64);
            }

            state.push_number(value);

            1
        }

        // spr n x y [w h] [flip_x] [flip_y]
        unsafe extern "C" fn lua_spr(lua_context: *mut lua_State) -> c_int {
            let mut state = State::from_ptr(lua_context);

            let n = state.check_integer(2);
            let x = state.check_integer(3);
            let y = state.check_integer(4);
            let w = state.check_integer(5);
            let h = state.check_integer(6);
            let flip_x = state.check_integer(7);
            let flip_y = state.check_integer(8);

            let screen = state.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.screen.clone()
                                          });

            screen
                .lock()
                .unwrap()
                .spr(n as u32,
                     x as i32,
                     y as i32,
                     w as u32,
                     h as u32,
                     flip_x == 1,
                     flip_y == 1);

            1
        }

        // sspr sx sy sw sh dx dy [dw dh] [flip_x] [flip_y]
        unsafe extern "C" fn lua_sspr(lua_context: *mut lua_State) -> c_int {
            let mut state = State::from_ptr(lua_context);

            let sx = state.check_integer(2);
            let sy = state.check_integer(3);
            let sw = state.check_integer(4);
            let sh = state.check_integer(5);
            let dx = state.check_integer(6);
            let dy = state.check_integer(7);
            let dw = state.check_integer(8);
            let dh = state.check_integer(9);
            let flip_x = state.check_integer(10);
            let flip_y = state.check_integer(11);

            let screen = state.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.screen.clone()
                                          });

            screen
                .lock()
                .unwrap()
                .sspr(sx as u32,
                      sy as u32,
                      sw as u32,
                      sh as u32,
                      dx as i32,
                      dy as i32,
                      dw as u32,
                      dh as u32,
                      flip_x == 1,
                      flip_y == 1);

            1
        }

        // map cel_x cel_y sx sy cel_w cel_h [layer]
        unsafe extern "C" fn lua_map(lua_context: *mut lua_State) -> c_int {
            debug!("LUA MAP");

            let mut state = State::from_ptr(lua_context);

            let cel_x = state.check_integer(2);
            let cel_y = state.check_integer(3);
            let sx = state.check_integer(4);
            let sy = state.check_integer(5);
            let cel_w = state.check_integer(6);
            let cel_h = state.check_integer(7);
            let layer = state.check_integer(8);

            let screen = state.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.screen.clone()
                                          });

            screen
                .lock()
                .unwrap()
                .map(cel_x as u32,
                     cel_y as u32,
                     sx as i32,
                     sy as i32,
                     cel_w as u32,
                     cel_h as u32,
                     layer as u8);


            1
        }

        unsafe extern "C" fn lua_mget(lua_context: *mut lua_State) -> c_int {
            debug!("LUA MGET");

            let mut state = State::from_ptr(lua_context);

            let x = state.check_integer(2);
            let y = state.check_integer(3);

            let screen = state.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.screen.clone()
                                          });

            let value = screen.lock().unwrap().mget(x as i32, y as i32);

            state.push_integer(value as i64);

            1
        }


        unsafe extern "C" fn lua_mset(lua_context: *mut lua_State) -> c_int {
            debug!("LUA MSET");

            let mut state = State::from_ptr(lua_context);

            let x = state.check_integer(2);
            let y = state.check_integer(3);
            let v = state.check_integer(4);

            let screen = state.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.screen.clone()
                                          });

            screen.lock().unwrap().mset(x as i32, y as i32, v as u32);

            1
        }

        unsafe extern "C" fn lua_print(lua_context: *mut lua_State) -> c_int {
            debug!("LUA PRINT");

            let mut state = State::from_ptr(lua_context);
            let mut state2 = State::from_ptr(lua_context);

            let str_data = state2.check_string(2);
            let x = state.check_integer(3);
            let y = state.check_integer(4);
            let col = state.check_integer(5);

            let screen = state.with_extra(|extra| {
                                              let data = extra
                                                  .as_ref()
                                                  .unwrap()
                                                  .downcast_ref::<ExtraData>()
                                                  .unwrap();
                                              data.screen.clone()
                                          });

            screen
                .lock()
                .unwrap()
                .print(str_data.to_string(), x as i32, y as i32, col as i32);

            1
        }

        unsafe extern "C" fn lua_time(lua_context: *mut lua_State) -> c_int {
            debug!("LUA TIME");

            let mut state = State::from_ptr(lua_context);

            let info = state.with_extra(|extra| {
                                            let data = extra
                                                .as_ref()
                                                .unwrap()
                                                .downcast_ref::<ExtraData>()
                                                .unwrap();
                                            data.info.clone()
                                        });

            state.push_integer(info.lock().unwrap().time());

            1
        }

        unsafe extern "C" fn lua_stat(lua_context: *mut lua_State) -> c_int {
            debug!("LUA STAT");

            let mut state = State::from_ptr(lua_context);

            let value = state.check_integer(2);

            let players = state.with_extra(|extra| {
                                               let data = extra
                                                   .as_ref()
                                                   .unwrap()
                                                   .downcast_ref::<ExtraData>()
                                                   .unwrap();
                                               data.players.clone()
                                           });

            let players_data = players.lock().unwrap();

            if value == 32 {
                state.push_integer(players_data.mouse.x as i64);
            } else if value == 33 {
                state.push_integer(players_data.mouse.y as i64);
            } else if value == 34 {
                state.push_integer(players_data.mouse.state as i64);
            } else {
                state.push_integer(0);
            }

            1
        }


        /***** CARTDATA *****/
        unsafe extern "C" fn lua_cartdata(_lua_context: *mut lua_State) -> c_int {
            debug!("LUA CARTDATA");

            /*
            let mut state = State::from_ptr(lua_context);
            let x = state.check_integer(2);
            */

            1
        }

        unsafe extern "C" fn lua_dget(_lua_context: *mut lua_State) -> c_int {
            debug!("LUA DGET");

            /*
            let mut state = State::from_ptr(lua_context);
            let x = state.check_integer(2);
            */

            1
        }


        unsafe extern "C" fn lua_dset(_lua_context: *mut lua_State) -> c_int {
            debug!("LUA DSET");

            /*
            let mut state = State::from_ptr(lua_context);
            let x = state.check_integer(2);
            */

            1
        }
    }

    pub const UNICORN_LUA_LIB: [(&'static str, Function); 41] =
        [("new", Some(UnicornLua::lua_new)),

         ("music", Some(UnicornLua::lua_chiptune_music)),
         ("sfx", Some(UnicornLua::lua_chiptune_sfx)),

         ("camera", Some(UnicornLua::lua_camera)),
         ("color", Some(UnicornLua::lua_color)),

         ("btn", Some(UnicornLua::lua_btn)),
         ("btnp", Some(UnicornLua::lua_btnp)),

         ("cls", Some(UnicornLua::lua_cls)),

         ("fget", Some(UnicornLua::lua_fget)),
         ("fget_all", Some(UnicornLua::lua_fget_all)),
         ("fset", Some(UnicornLua::lua_fset)),
         ("fset_all", Some(UnicornLua::lua_fset_all)),

         ("line", Some(UnicornLua::lua_line)),

         ("rect", Some(UnicornLua::lua_rect)),
         ("rectfill", Some(UnicornLua::lua_rectfill)),
         ("circ", Some(UnicornLua::lua_circ)),
         ("circfill", Some(UnicornLua::lua_circfill)),
         ("ellipse", Some(UnicornLua::lua_ellipse)),
         ("ellipsefill", Some(UnicornLua::lua_ellipsefill)),
         ("trigon", Some(UnicornLua::lua_trigon)),

         ("clip", Some(UnicornLua::lua_clip)),

         ("spr", Some(UnicornLua::lua_spr)),
         ("sspr", Some(UnicornLua::lua_sspr)),

         ("map", Some(UnicornLua::lua_map)),
         ("mget", Some(UnicornLua::lua_mget)),
         ("mset", Some(UnicornLua::lua_mset)),

         ("palt", Some(UnicornLua::lua_palt)),
         ("pal", Some(UnicornLua::lua_pal)),

         ("pget", Some(UnicornLua::lua_pget)),
         ("pset", Some(UnicornLua::lua_pset)),

         ("sget", Some(UnicornLua::lua_sget)),
         ("sset", Some(UnicornLua::lua_sset)),

         ("noise", Some(UnicornLua::lua_noise)),
         ("noise_set_seed", Some(UnicornLua::lua_noise_set_seed)),

         ("rnd", Some(UnicornLua::lua_rnd)),

         ("print", Some(UnicornLua::lua_print)),

         ("time", Some(UnicornLua::lua_time)),

         ("stat", Some(UnicornLua::lua_stat)),

         ("cartdata", Some(UnicornLua::lua_cartdata)),
         ("dget", Some(UnicornLua::lua_dget)),
         ("dset", Some(UnicornLua::lua_dset))];
}

#[cfg(not(feature = "unicorn_plugin_lua"))]
pub mod plugin {
    use std::sync::{Arc, Mutex};

    use config::Players;

    use unicorn::noise::Noise;
    use unicorn::info::Info;
    use sound::sound::Sound;

    use gfx::Screen;


    pub struct LuaPlugin {}

    impl LuaPlugin {
        pub fn new() -> LuaPlugin {
            LuaPlugin {}
        }

        // Keep the compatibility
        pub fn load(&mut self,
                    _players: Arc<Mutex<Players>>,
                    _info: Arc<Mutex<Info>>,
                    _screen: Arc<Mutex<Screen>>,
                    _noise: Arc<Mutex<Noise>>,
                    _sound: Arc<Mutex<Sound>>) {
            panic!("LUA plugin disabled");
        }
        pub fn load_code(&mut self, _data: String) -> bool {
            false
        }
        pub fn init(&mut self) {}
        pub fn draw(&mut self) -> bool {
            false
        }
        pub fn update(&mut self) -> bool {
            false
        }
    }
}
