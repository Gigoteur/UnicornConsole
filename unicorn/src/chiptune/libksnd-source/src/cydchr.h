#ifndef CYDCHR_H
#define CYDCHR_H

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

#define CYDCHR_SIZE 50

#include "cydtypes.h"

typedef struct
{
	Sint32 *buffer, *lut;
	int sample_rate;
	int min_delay, max_delay;
	int pos_l, pos_r, pos_buf, buf_size, lut_size;
} CydChorus;

void cydchr_output(CydChorus *chr, Sint32 in_l, Sint32 in_r, Sint32 *out_l, Sint32 *out_r);
void cydchr_set(CydChorus *chr, int rate /* 1 = 0.1 Hz */, int min_delay, int max_delay, int stereo_separation);

void cydchr_init(CydChorus *chr, int sample_rate);
void cydchr_deinit(CydChorus *chr);

#endif
