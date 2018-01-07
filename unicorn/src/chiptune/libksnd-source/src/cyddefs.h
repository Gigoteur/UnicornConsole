#ifndef CYDDEFS_H
#define CYDDEFS_H

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

#define RANDOM_SEED 0xf31782ce
#define CLOCK 985250
#define PRE_GAIN 1
#define PRE_GAIN_DIVISOR 4
#define OUTPUT_BITS 16
#define MAX_OVERSAMPLE 2
#define ACC_BITS (23 + MAX_OVERSAMPLE)
#define ENVELOPE_SCALE 2

#ifdef LOWRESWAVETABLE
#define WAVETABLE_RESOLUTION 2048
#else
#define WAVETABLE_RESOLUTION 65536
#endif

#define ACC_LENGTH (1 << (ACC_BITS - 1)) // Osc counter length
#define YM_LENGTH (ACC_LENGTH) // YM envelope counter length
#define MAX_VOLUME 128
#define CYD_WAVE_MAX_ENTRIES 128

#define CYD_BASE_FREQ 22050
#define CYD_MAX_CHANNELS 32
#define CYD_MAX_FX_CHANNELS 8
#define CYD_SUB_OSCS 3

#define CYD_PAN_CENTER 64
#define CYD_PAN_LEFT 0
#define CYD_PAN_RIGHT 128
#define CYD_STEREO_GAIN 2048

#define CYD_CUTOFF_MAX 2048

#define CYD_CHORUS_OVERSAMPLE 1

#define WAVE_AMP (1 << OUTPUT_BITS)
#define BUFFER_GRANULARITY 150 // mutex is locked and audio generated in 150 sample blocks

#endif
