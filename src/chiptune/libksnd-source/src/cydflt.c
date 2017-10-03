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


#include "cydflt.h"
#include "macros.h"
#include <assert.h>
  
void cydflt_set_coeff(CydFilter *flt, Uint16 frequency, Uint16 resonance)
{
	flt->q = 2048 - frequency;
	flt->p = frequency + ((Sint32)(0.8f * 2048.0f) * frequency / 2048 * flt->q) / 2048;
	flt->f = flt->p + flt->p - 2048;
	flt->q = resonance;
}


void cydflt_cycle(CydFilter *flt, Sint32 input)
{
	input -= flt->q * flt->b4 / 2048;                          //feedback
	Sint32 t1 = flt->b1;  
	flt->b1 = (input + flt->b0) * flt->p / 2048- flt->b1 * flt->f / 2048;
	Sint32 t2 = flt->b2;  
	flt->b2 = (flt->b1 + t1) * flt->p / 2048 - flt->b2 * flt->f / 2048;
	t1 = flt->b3;  
	flt->b3 = (flt->b2 + t2) * flt->p / 2048 - flt->b3 * flt->f / 2048;
	flt->b4 = (flt->b3 + t1) * flt->p / 2048 - flt->b4 * flt->f / 2048;
	flt->b4 = my_min(32767, my_max(-32768, flt->b4));    //clipping
	
	flt->b0 = input;
}


Sint32 cydflt_output_lp(CydFilter *flt)
{
	return flt->b4;
}


Sint32 cydflt_output_hp(CydFilter *flt)
{
	return flt->b0 - flt->b4;
}


Sint32 cydflt_output_bp(CydFilter *flt)
{
	return 3 * (flt->b3 - flt->b4);
}
