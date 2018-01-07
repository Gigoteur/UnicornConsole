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

#include "cydcrush.h"
#include "macros.h"


#ifdef STEREOOUTPUT
void cydcrush_output(CydCrush *crush, Sint32 in_l, Sint32 in_r, Sint32 *out_l, Sint32 *out_r)
{
	if (crush->counter++ >= crush->downsample)
	{
		crush->counter = 0;
		
		if (!crush->dither)
		{
			crush->hold_l = (((in_l + 32768) & crush->bit_drop) - 32768);
			crush->hold_r = (((in_r + 32768) & crush->bit_drop) - 32768);
		}
		else
		{
			crush->hold_l = ((my_max(0, my_min(65535, in_l + 32768 + crush->error_l)) & crush->bit_drop) - 32768);
			crush->hold_r = ((my_max(0, my_min(65535, in_r + 32768 + crush->error_r)) & crush->bit_drop) - 32768);
			
			crush->error_l += in_l - crush->hold_l;
			crush->error_r += in_r - crush->hold_r;
		}
	}

	*out_l = crush->hold_l * crush->gain / 128;
	*out_r = crush->hold_r * crush->gain / 128;
}
#else
Sint32 cydcrush_output(CydCrush *crush, Sint32 input)
{
	if (crush->counter++ >= crush->downsample)
	{
		crush->counter = 0;
		
		if (!crush->dither)
			crush->hold = (((input + 32768) & crush->bit_drop) - 32768);
		else
			crush->hold = ((my_max(0, my_min(65535, input + 32768 + crush->error)) & crush->bit_drop) - 32768);
			
		crush->error += input - crush->hold;
	}

	return crush->hold * crush->gain / 128;
}
#endif


void cydcrush_set(CydCrush *crush, int downsample, int bit_drop, int dither, int gain)
{
	crush->downsample = downsample * crush->sample_rate / 44100;
	//crush->counter = 0;
	if (bit_drop >= 0) crush->bit_drop = 0xffffffff << (bit_drop);
	if (dither >= 0) crush->dither = dither;
	if (gain >= 0) crush->gain = gain;
}


void cydcrush_init(CydCrush *crush, int sample_rate)
{
	crush->sample_rate = sample_rate;
	crush->counter = 0;
#ifdef STEREOOUTPUT
	crush->error_l = 0;
	crush->error_r = 0;
	crush->hold_l = 0;
	crush->hold_r = 0;
#else
	crush->error = 0;
	crush->hold = 0;
#endif
	crush->gain = 128;
}


void cydcrush_deinit(CydCrush *crush)
{
}

