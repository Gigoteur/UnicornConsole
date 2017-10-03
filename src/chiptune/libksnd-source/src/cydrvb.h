#ifndef CYDRVB_H
#define CYDRVB_H

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

#include "cydtypes.h"
#include <math.h>

// Max delay length in milliseconds
#define CYDRVB_SIZE 2000
#define CYDRVB_TAPS 16
#define CYDRVB_0dB 2048
#define CYDRVB_LOW_LIMIT (int)(100.0 * log(1.0 / (double)CYDRVB_0dB))

typedef struct
{
	int position;
#ifdef STEREOOUTPUT
	int gain_r, gain_l;
#else
	int gain;
#endif
	int delay;
} CydTap;

typedef struct
{
	Sint32 *buffer;
	int size, rate;
	int position;
	CydTap tap[CYDRVB_TAPS];
} CydReverb;

void cydrvb_init(CydReverb *rvb, int rate);
void cydrvb_deinit(CydReverb *rvb);

#ifdef STEREOOUTPUT
void cydrvb_cycle(CydReverb *rvb, Sint32 left, Sint32 right);
void cydrvb_output(CydReverb *rvb, Sint32 *left, Sint32 *right);
#else
void cydrvb_cycle(CydReverb *rvb, Sint32 input);
Sint32 cydrvb_output(CydReverb *rvb);
#endif

void cydrvb_set_tap(CydReverb *rvb, int idx, int delay_ms, int gain_db, int panning);

#endif
