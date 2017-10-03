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

#include "cyddefs.h"
#include "cydrvb.h"
#include "macros.h"
#include <math.h>
#include <stdlib.h>
#include <string.h>

void cydrvb_init(CydReverb *rvb, int rate)
{
	memset(rvb, 0, sizeof(*rvb));
	
	int bufsize = CYDRVB_SIZE * rate / 1000;
	
	rvb->size = bufsize;
	rvb->rate = rate;
#ifdef STEREOOUTPUT
	rvb->buffer = calloc(sizeof(*rvb->buffer) * 2, bufsize);
#else
	rvb->buffer = calloc(sizeof(*rvb->buffer), bufsize);
#endif
	
	for (int i = 0 ; i < CYDRVB_TAPS ; ++i)
		cydrvb_set_tap(rvb, i, i * 100 + 50, (i + 1) * -30, CYD_PAN_CENTER);
}


void cydrvb_deinit(CydReverb *rvb)
{
	free(rvb->buffer);
	rvb->buffer = NULL;
}


#ifdef STEREOOUTPUT
void cydrvb_cycle(CydReverb *rvb, Sint32 left, Sint32 right)
{
	for (int i = 0 ; i < CYDRVB_TAPS ; ++i)
	{
		++rvb->tap[i].position;
		if (rvb->tap[i].position >= rvb->size)
			rvb->tap[i].position = 0;
	}
	
	++rvb->position;
	if (rvb->position >= rvb->size)
		rvb->position = 0;
		
	rvb->buffer[rvb->position * 2] = left;
	rvb->buffer[rvb->position * 2 + 1] = right;
}

#else

void cydrvb_cycle(CydReverb *rvb, Sint32 input)
{
	for (int i = 0 ; i < CYDRVB_TAPS ; ++i)
	{
		++rvb->tap[i].position;
		if (rvb->tap[i].position >= rvb->size)
			rvb->tap[i].position = 0;
	}
	
	++rvb->position;
	if (rvb->position >= rvb->size)
		rvb->position = 0;
		
	rvb->buffer[rvb->position] = input;
}
#endif

#ifdef STEREOOUTPUT
void cydrvb_output(CydReverb *rvb, Sint32 *left, Sint32 *right)
{
	*left = 0;
	*right = 0;
	
	for (int i = 0 ; i < CYDRVB_TAPS ; ++i)
	{
		if (rvb->tap[i].gain_l != 0 || rvb->tap[i].gain_r != 0)
		{
			*left += rvb->tap[i].gain_l * (Sint32)rvb->buffer[rvb->tap[i].position * 2] / CYDRVB_0dB;
			*right += rvb->tap[i].gain_r * (Sint32)rvb->buffer[rvb->tap[i].position * 2 + 1] / CYDRVB_0dB;
		}
	}
}

#else

Sint32 cydrvb_output(CydReverb *rvb)
{
	Sint32 o = 0;
	
	for (int i = 0 ; i < CYDRVB_TAPS ; ++i)
	{
		if (rvb->tap[i].gain != 0)
		{
			o += rvb->tap[i].gain * rvb->buffer[rvb->tap[i].position] / CYDRVB_0dB;
		}
	}
	
	return o;
}
#endif


void cydrvb_set_tap(CydReverb *rvb, int idx, int delay_ms, int gain_db, int panning)
{
	rvb->tap[idx].delay = delay_ms;
	rvb->tap[idx].position = (rvb->position - (delay_ms * rvb->rate / 1000) + rvb->size) % rvb->size;
	
	if (gain_db <= CYDRVB_LOW_LIMIT)
	{
#ifdef STEREOOUTPUT

		rvb->tap[idx].gain_l = 0;
		rvb->tap[idx].gain_r = 0;
#else
                rvb->tap[idx].gain = 0;
#endif
	}
	else
	{
#ifdef STEREOOUTPUT
		panning = my_min(CYD_PAN_RIGHT, my_max(CYD_PAN_LEFT, panning));
		float a = M_PI / 2 * (float)(panning - CYD_PAN_LEFT) / (CYD_PAN_RIGHT - CYD_PAN_LEFT);
		int gain = pow(10.0, (double)gain_db * 0.01) * CYDRVB_0dB;
		rvb->tap[idx].gain_l = gain * cos(a);
		rvb->tap[idx].gain_r = gain * sin(a);
#else
		int gain = pow(10.0, (double)gain_db * 0.01) * CYDRVB_0dB;
		rvb->tap[idx].gain = gain;
#endif
	}
}
