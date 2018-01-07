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

#include "cydentry.h"
#include "cyddefs.h"
#include "freqs.h"
#include "SDL_endian.h"
#include <stdlib.h>
#include <string.h>


void cyd_wave_entry_deinit(CydWavetableEntry *entry)
{
	if (entry->data) free(entry->data);
	entry->data = NULL;
}


void cyd_wave_entry_init(CydWavetableEntry *entry, const void *data, Uint32 n_samples, CydWaveType sample_type, int channels, int denom, int nom)
{
	if (data && n_samples > 0)
	{
		entry->data = realloc(entry->data, sizeof(*entry->data) * n_samples);
		
		for (int i = 0; i < n_samples ; ++i)
		{
			Sint32 v = 0;
			
			for (int c = 0; c < channels ; ++c)
			{
				switch (sample_type)
				{
					case CYD_WAVE_TYPE_SINT16:
						v += SDL_SwapLE16(((Sint16*)data)[i * channels + c]);
						break;
					
					case CYD_WAVE_TYPE_UINT8:					
						v += ((Sint16)(((Uint8*)data)[i * channels + c]) - 0x80) << 8;
						break;
						
					case CYD_WAVE_TYPE_SINT8:
						v += (Sint16)(((Sint8*)data)[i * channels + c]) << 8;
						break;
				}
			}
			
			if (channels > 1)
				v /= channels;
			
			entry->data[i] = v * denom / nom;
		}
		
		entry->samples = n_samples;
	}
	else
	{
		free(entry->data);
		entry->data = NULL;
		entry->samples = 0;
	}
}
