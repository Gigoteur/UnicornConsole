#ifndef MACROS_H
#define MACROS_H

/*
Copyright (c) 2009-2010 Tero Lindeman (kometbomb)

Permission is hereby granted, free of charge, to any person
obtaining a copy of this software and associated documentation
files (the "Software"), to deal in the Software without
restriction, including without limitation the rights to use,
copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the
Software is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.
*/


#include <stdio.h>
#include "SDL_endian.h"

#define SQR(x) ((x)*(x))
#define my_min(a,b) (((a)<(b))?(a):(b))
#define my_max(a,b) (((a)>(b))?(a):(b))
#define my_lock(s) do { if (SDL_MUSTLOCK(s)) SDL_LockSurface(s); } while(0)
#define my_unlock(s) do { if (SDL_MUSTLOCK(s)) SDL_UnlockSurface(s); } while(0)
#define VER(file_version, first_version, last_version, block)\
	if ((((Uint16)file_version) >= ((Uint16)first_version)) && (((Uint16)file_version) <= ((Uint16)last_version)))\
	{\
		block;\
	} 
	
#define VER_READ(file_version, first_version, last_version, var, size) VER(file_version, first_version, last_version, SDL_RWread(ctx, var, !size ? sizeof(*var) : size, 1));
#define _VER_READ(x, size) VER_READ(version, 0, MUS_VERSION, x, size)
#define _VER_WRITE(x, size) fwrite(x, !size ? sizeof(*x) : size, 1, f)

#ifndef ANDROID

#ifdef DEBUG
# define debug(...) do { printf("[DEBUG] "); printf(__VA_ARGS__); printf("\n"); } while(0)
#else
# define debug(...) do {} while(0)
#endif

// Only define warning messages if not optimizing for size (CFG=size)
#ifndef REDUCESIZE
#define warning(...) do { fputs("[WARNING] ", stderr); fprintf(stderr, __VA_ARGS__); fputs("\n", stderr); } while(0)
#define fatal(...) do { fputs("[FATAL] ", stderr); fprintf(stderr, __VA_ARGS__); fputs("\n", stderr); } while(0)
#else
#define warning(...) do {} while(0)
#define fatal(...) do {} while(0)
#endif

#else

#include <android/log.h>

#ifdef DEBUG
# define debug(...) do { __android_log_print(ANDROID_LOG_DEBUG, "klystron", __VA_ARGS__); } while(0)
#else
# define debug(...) do {} while(0)
#endif

#define warning(...) do { __android_log_print(ANDROID_LOG_WARN, "klystron", __VA_ARGS__); } while(0)
#define fatal(...) do { __android_log_print(ANDROID_LOG_ERROR, "klystron", __VA_ARGS__); } while(0)


#endif

# define dumpvar(x) debug(#x " = %d", x)

#define FIX_ENDIAN(x) do { x = (sizeof(x) < 2 ? x : (sizeof(x) == 2 ? SDL_SwapLE16(x) : SDL_SwapLE32(x))); } while(0)

// Makes "warning: cast to pointer from integer of different size" disappear

#define CASTPTR(t,x) (*(t*)&x)
#ifdef __i386__
#  define CASTTOPTR(t,x) (t*)x
#else
#  define CASTTOPTR(t,x) (t*)(Uint64)x
#endif

#if __i386__
#define MAKEPTR(x) ((void*)(Uint32)(x))
#else
#define MAKEPTR(x) ((void*)(Uint64)(x))
#endif

#define STRINGIFY(x) #x
#define TOSTRING(x) STRINGIFY(x)

#define clamp(val, add, _min, _max) val = my_min(_max, my_max(_min, (int)val + add))

#endif
