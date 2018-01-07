#ifndef CYDTYPES_H
#define CYDTYPES_H

/* SDL-equivalent types */

#include "SDL.h"

#ifndef USENATIVEAPIS

typedef SDL_mutex * CydMutex;

#else

# ifdef WIN32

#include <windows.h>

/*typedef BYTE Uint8;
typedef CHAR Sint8;
typedef WORD Uint16;
typedef SHORT Sint16;
typedef DWORD Uint32;
typedef INT Sint32;
typedef unsigned long long Uint64;*/
typedef CRITICAL_SECTION CydMutex;

# else

#  error USENATIVEAPIS: Platform not supported

# endif

#endif

#ifdef LOWRESWAVETABLE
typedef Uint32 CydWaveAcc;
typedef Sint32 CydWaveAccSigned;
#else
typedef Uint64 CydWaveAcc;
typedef Sint64 CydWaveAccSigned;
#endif

#endif
