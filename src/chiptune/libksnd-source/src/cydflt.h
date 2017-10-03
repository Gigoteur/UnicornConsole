#ifndef CYDFLT_H
#define CYDFLT_H

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

typedef struct
{
	Sint32 f, q, p;
	Sint32 b0, b1, b2, b3, b4;             //filter coefficients
} CydFilter;

/* 0..2047 */
void cydflt_set_coeff(CydFilter *flt, Uint16 frequency, Uint16 cutoff);

void cydflt_cycle(CydFilter *flt, Sint32 input);
Sint32 cydflt_output_lp(CydFilter *flt);
Sint32 cydflt_output_hp(CydFilter *flt);
Sint32 cydflt_output_bp(CydFilter *flt);

#endif
