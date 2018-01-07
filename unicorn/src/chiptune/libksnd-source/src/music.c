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

#define GENERATE_VIBRATO_TABLES

#include "music.h"
#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include "freqs.h"
#include "macros.h"
#include "pack.h"

#ifdef GENERATE_VIBRATO_TABLES

#include <math.h>

#endif

#define VIB_TAB_SIZE 128

#ifndef GENERATE_VIBRATO_TABLES

static const Sint8 rnd_table[VIB_TAB_SIZE] = {
	110, -1, 88, -31, 64,
	-13, 29, -70, -113, 71,
	99, -71, 74, 82, 52,
	-82, -58, 37, 20, -76,
	46, -97, -69, 41, 31,
	-62, -5, 99, -2, -48,
	-89, 17, -19, 4, -27,
	-43, -20, 25, 112, -34,
	78, 26, -56, -54, 72,
	-75, 22, 72, -119, 115,
	56, -66, 25, 87, 93,
	14, 82, 127, 79, -40,
	-100, 21, 17, 17, -116,
	-110, 61, -99, 105, 73,
	116, 53, -9, 105, 91,
	120, -73, 112, -10, 66,
	-10, -30, 99, -67, 60,
	84, 110, 87, -27, -46,
	114, 77, -27, -46, 75,
	-78, 83, -110, 92, -9,
	107, -64, 31, 77, -39,
	115, 126, -7, 121, -2,
	66, 116, -45, 91, 1,
	-96, -27, 17, 76, -82,
	58, -7, 75, -35, 49,
	3, -52, 40
};

static const Sint8 sine_table[VIB_TAB_SIZE] =
{
	0, 6, 12, 18, 24, 31, 37, 43, 48, 54, 60, 65, 71, 76, 81, 85, 90, 94, 98, 102, 106, 109, 112,
	115, 118, 120, 122, 124, 125, 126, 127, 127, 127, 127, 127, 126, 125, 124, 122, 120, 118, 115, 112,
	109, 106, 102, 98, 94, 90, 85, 81, 76, 71, 65, 60, 54, 48, 43, 37, 31, 24, 18, 12, 6,
	0, -6, -12, -18, -24, -31, -37, -43, -48, -54, -60, -65, -71, -76, -81, -85, -90, -94, -98, -102,
	-106, -109, -112, -115, -118, -120, -122, -124, -125, -126, -127, -127, -128, -127, -127, -126, -125, -124, -122,
	-120, -118, -115, -112, -109, -106, -102, -98, -94, -90, -85, -81, -76, -71, -65, -60, -54, -48, -43, -37, -31, -24, -18, -12, -6
};

#else

static Sint8 rnd_table[VIB_TAB_SIZE];
static Sint8 sine_table[VIB_TAB_SIZE];

#endif

static int mus_trigger_instrument_internal(MusEngine* mus, int chan, MusInstrument *ins, Uint16 note, int panning);

#ifndef USESDL_RWOPS

static int RWread(struct RWops *context, void *ptr, int size, int maxnum)
{
	return fread(ptr, size, maxnum, context->fp);
}


static int RWclose(struct RWops *context)
{
	if (context->close_fp) fclose(context->fp);
	free(context);
	return 1;
}


#define my_RWread(ctx, ptr, size, maxnum) ctx->read(ctx, ptr, size, maxnum)
#define my_RWclose(ctx) ctx->close(ctx)
#define my_RWtell(ctx) 0


#else

#include "SDL_rwops.h"

#define my_RWread SDL_RWread
#define my_RWclose SDL_RWclose
#define my_RWtell SDL_RWtell


#endif


static RWops * RWFromFP(FILE *f, int close)
{
#ifdef USESDL_RWOPS
	SDL_RWops *rw = SDL_RWFromFP(f, close);
	
	if (!rw)
	{
		warning("SDL_RWFromFP: %s", SDL_GetError());
	}
	
	return rw;
#else
	RWops *rw = calloc(sizeof(*rw), 1);
	
	rw->fp = f;
	rw->close_fp = close;
	rw->read = RWread;
	rw->close = RWclose;
	
	return rw;
#endif
}


static RWops * RWFromFile(const char *name, const char *mode)
{
#ifdef USESDL_RWOPS
	return SDL_RWFromFile(name, mode);
#else
	FILE *f = fopen(name, mode);
	
	if (!f) return NULL;
	
	return RWFromFP(f, 1);
#endif
}



static void update_volumes(MusEngine *mus, MusTrackStatus *ts, MusChannel *chn, CydChannel *cydchn, int volume)
{
	if (chn->instrument && (chn->instrument->flags & MUS_INST_RELATIVE_VOLUME))
	{
		ts->volume = volume;
		cydchn->adsr.volume = (chn->flags & MUS_CHN_DISABLED) ? 0 : (int)chn->instrument->volume * volume / MAX_VOLUME * (int)mus->volume / MAX_VOLUME * (int)mus->play_volume / MAX_VOLUME * (int)chn->volume / MAX_VOLUME;
	}
	else
	{
		ts->volume = volume;
		cydchn->adsr.volume = (chn->flags & MUS_CHN_DISABLED) ? 0 : ts->volume * (int)mus->volume / MAX_VOLUME * (int)mus->play_volume / MAX_VOLUME * (int)chn->volume / MAX_VOLUME;
	}
}


static void update_all_volumes(MusEngine *mus)
{
	for (int i = 0 ; i < MUS_MAX_CHANNELS && i < mus->cyd->n_channels ; ++i)
		update_volumes(mus, &mus->song_track[i], &mus->channel[i], &mus->cyd->channel[i], mus->song_track[i].volume);
}


static void mus_set_buzz_frequency(MusEngine *mus, int chan, Uint16 note)
{
#ifndef CYD_DISABLE_BUZZ
	MusChannel *chn = &mus->channel[chan];
	if (chn->instrument && chn->instrument->flags & MUS_INST_YM_BUZZ)
	{
#ifndef CYD_DISABLE_INACCURACY
		Uint16 buzz_frequency = get_freq(note + chn->buzz_offset) & mus->pitch_mask;
#else
		Uint16 buzz_frequency = get_freq(note + chn->buzz_offset);
#endif
		cyd_set_env_frequency(mus->cyd, &mus->cyd->channel[chan], buzz_frequency);
	}
#endif
}


static void mus_set_wavetable_frequency(MusEngine *mus, int chan, Uint16 note)
{
#ifndef CYD_DISABLE_WAVETABLE
	MusChannel *chn = &mus->channel[chan];
	CydChannel *cydchn = &mus->cyd->channel[chan];
	MusTrackStatus *track_status = &mus->song_track[chan];
	
	if (chn->instrument && (chn->instrument->cydflags & CYD_CHN_ENABLE_WAVE) && (cydchn->wave_entry))
	{
		for (int s = 0 ; s < CYD_SUB_OSCS ; ++s)
		{
			Uint16 final = 0;
			
			if (s == 0 || (chn->instrument->flags & MUS_INST_MULTIOSC))
			{
				switch (s)
				{
					default:
					case 0:
						final = note;
						break;
					
					case 1: 
						if (track_status->extarp1 != 0)
							final = note + ((Uint16)track_status->extarp1 << 8);
						else
							final = 0;
						break;
						
					case 2: 
						if (track_status->extarp2 != 0)
							final = note + ((Uint16)track_status->extarp2 << 8);
						else
							final = 0;
						break;
				}
			}
		
			Uint16 wave_frequency = 0;
		
			if (final != 0)
			{
#ifndef CYD_DISABLE_INACCURACY
				wave_frequency = get_freq((chn->instrument->flags & MUS_INST_WAVE_LOCK_NOTE) ? cydchn->wave_entry->base_note : (final)) & mus->pitch_mask;
#else
				wave_frequency = get_freq((chn->instrument->flags & MUS_INST_WAVE_LOCK_NOTE) ? cydchn->wave_entry->base_note : (final));
#endif
			}
			
			cyd_set_wavetable_frequency(mus->cyd, cydchn, s, wave_frequency);
		}
	}
#endif
}


static void mus_set_frequency(MusEngine *mus, int chan, Uint16 note, int divider)
{
	MusChannel *chn = &mus->channel[chan];
	MusTrackStatus *track_status = &mus->song_track[chan];
	
	for (int s = 0 ; s < CYD_SUB_OSCS ; ++s)
	{
		Uint16 final = 0;
			
		if (s == 0 || (chn->instrument->flags & MUS_INST_MULTIOSC))
		{
			switch (s)
			{
				default:
				case 0:
					final = note;
					break;
				
				case 1: 
					if (track_status->extarp1 != 0)
						final = note + ((Uint16)track_status->extarp1 << 8);
					else
						final = 0;
					break;
					
				case 2: 
					if (track_status->extarp2 != 0)
						final = note + ((Uint16)track_status->extarp2 << 8);
					else
						final = 0;
					break;
			}
		}
		
		Uint16 frequency = 0;
		
		if (final != 0)
		{
#ifndef CYD_DISABLE_INACCURACY
			frequency = get_freq(final) & mus->pitch_mask;
#else
			frequency = get_freq(final);
#endif
		}
		
		cyd_set_frequency(mus->cyd, &mus->cyd->channel[chan], s, frequency / divider);
	}
}


static void mus_set_note(MusEngine *mus, int chan, Uint16 note, int update_note, int divider)
{
	MusChannel *chn = &mus->channel[chan];
	
	if (update_note) chn->note = note;
	
	mus_set_frequency(mus, chan, note, divider);
	
	mus_set_wavetable_frequency(mus, chan, note);
	
	mus_set_buzz_frequency(mus, chan, note);
}


static void mus_set_slide(MusEngine *mus, int chan, Uint16 note)
{
	MusChannel *chn = &mus->channel[chan];
	chn->target_note = note;
	//if (update_note) chn->note = note;
}


void mus_init_engine(MusEngine *mus, CydEngine *cyd)
{
	memset(mus, 0, sizeof(*mus));
	mus->cyd = cyd;
	mus->volume = MAX_VOLUME;
	mus->play_volume = MAX_VOLUME;
	
	for (int i = 0 ; i < MUS_MAX_CHANNELS ; ++i)
		mus->channel[i].volume = MAX_VOLUME;
		
#ifndef CYD_DISABLE_INACCURACY
	mus->pitch_mask = ~0;
#endif

#ifdef GENERATE_VIBRATO_TABLES
	for (int i = 0 ; i < VIB_TAB_SIZE ; ++i)
	{
		sine_table[i] = sin((float)i / VIB_TAB_SIZE * M_PI * 2) * 127;
		rnd_table[i] = rand();
	}
#endif
}


static void do_command(MusEngine *mus, int chan, int tick, Uint16 inst, int from_program)
{
	MusChannel *chn = &mus->channel[chan];
	CydChannel *cydchn = &mus->cyd->channel[chan];
	CydEngine *cyd = mus->cyd;
	MusTrackStatus *track_status = &mus->song_track[chan];
	
	switch (inst & 0x7f00)
	{
		case MUS_FX_PORTA_UP:
		{
			Uint16 prev = chn->note;
			chn->note += ((inst & 0xff) << 2);
			if (prev > chn->note) chn->note = 0xffff;
			
			mus_set_slide(mus, chan, chn->note);
		}
		break;
		
		case MUS_FX_PORTA_DN:
		{
			Uint16 prev = chn->note;
			chn->note -= ((inst & 0xff) << 2);
			
			if (prev < chn->note) chn->note = 0x0;
			
			mus_set_slide(mus, chan, chn->note);
		}
		break;
		
		case MUS_FX_PORTA_UP_LOG:
		{
			Uint16 prev = chn->note;
			chn->note += my_max(1, ((Uint32)frequency_table[MIDDLE_C] * (Uint32)(inst & 0xff) / (Uint32)get_freq(chn->note)));
			
			if (prev > chn->note) chn->note = 0xffff;
			
			mus_set_slide(mus, chan, chn->note);
		}
		break;
		
		case MUS_FX_PORTA_DN_LOG:
		{
			Uint16 prev = chn->note;
			chn->note -= my_max(1, ((Uint32)frequency_table[MIDDLE_C] * (Uint32)(inst & 0xff) / (Uint32)get_freq(chn->note)));
			
			if (prev < chn->note) chn->note = 0x0;
			
			mus_set_slide(mus, chan, chn->note);
		}
		break;
		
		case MUS_FX_PW_DN:
		{
			track_status->pw -= inst & 0xff;
			if (track_status->pw > 0xf000) track_status->pw = 0;
		}
		break;
		
		case MUS_FX_PW_UP:
		{
			track_status->pw += inst & 0xff;
			if (track_status->pw > 0x7ff) track_status->pw = 0x7ff;
		}
		break;
		
#ifndef CYD_DISABLE_FILTER
		case MUS_FX_CUTOFF_DN:
		{
			track_status->filter_cutoff -= inst & 0xff;
			if (track_status->filter_cutoff > 0xf000) track_status->filter_cutoff = 0;
			cyd_set_filter_coeffs(mus->cyd, cydchn, track_status->filter_cutoff, track_status->filter_resonance);
		}
		break;
		
		case MUS_FX_CUTOFF_UP:
		{
			track_status->filter_cutoff += inst & 0xff;
			if (track_status->filter_cutoff > 0x7ff) track_status->filter_cutoff = 0x7ff;
			cyd_set_filter_coeffs(mus->cyd, cydchn, track_status->filter_cutoff, track_status->filter_resonance);
		}
		break;
#endif
				
#ifndef CYD_DISABLE_BUZZ		
		case MUS_FX_BUZZ_DN:
		{
			if (chn->buzz_offset >= -32768 + (inst & 0xff))
				chn->buzz_offset -= inst & 0xff;
				
			mus_set_buzz_frequency(mus, chan, chn->note);
		}
		break;
		
		case MUS_FX_BUZZ_UP:
		{
			if (chn->buzz_offset <= 32767 - (inst & 0xff))
				chn->buzz_offset += inst & 0xff;
				
			mus_set_buzz_frequency(mus, chan, chn->note);
		}
		break;
#endif
		case MUS_FX_TRIGGER_RELEASE:
		{
			if (tick == (inst & 0xff)) 
				cyd_enable_gate(mus->cyd, cydchn, 0);
		}
		break;
		
		case MUS_FX_FADE_VOLUME:
		{
			if (!(chn->flags & MUS_CHN_DISABLED))
			{
				track_status->volume -= inst & 0xf;
				if (track_status->volume > MAX_VOLUME) track_status->volume = 0;
				track_status->volume += (inst >> 4) & 0xf;
				if (track_status->volume > MAX_VOLUME) track_status->volume = MAX_VOLUME;
				
				update_volumes(mus, track_status, chn, cydchn, track_status->volume);
			}
		}
		break;
#ifdef STEREOOUTPUT
		case MUS_FX_PAN_RIGHT:
		case MUS_FX_PAN_LEFT:
		{
			int p = cydchn->panning;
			if ((inst & 0xff00) == MUS_FX_PAN_LEFT) 
			{
				p -= inst & 0x00ff;
			}
			else
			{
				p += inst & 0x00ff;
			}
			
			p = my_min(CYD_PAN_RIGHT, my_max(CYD_PAN_LEFT, p));
			
			cyd_set_panning(mus->cyd, cydchn, p);
		}
		break;
#endif
		case MUS_FX_EXT:
		{
			// Protracker style Exy commands
		
			switch (inst & 0xfff0)
			{
				case MUS_FX_EXT_NOTE_CUT:
				{
					if (!(chn->flags & MUS_CHN_DISABLED))
					{
						if ((inst & 0xf) <= tick)
						{
							cydchn->adsr.volume = 0;
							track_status->volume = 0;
						}
					}
				}
				break;
				
				case MUS_FX_EXT_RETRIGGER:
				{
					if ((inst & 0xf) > 0 && (tick % (inst & 0xf)) == 0)
					{
						Uint8 prev_vol_tr = track_status->volume;
						Uint8 prev_vol_cyd = cydchn->adsr.volume;
						mus_trigger_instrument_internal(mus, chan, chn->instrument, chn->last_note, -1);
						track_status->volume = prev_vol_tr;
						cydchn->adsr.volume = prev_vol_cyd;
					}
				}
				break;
			}
		}
		break;
	}
	
	if (tick == 0) 
	{
		// --- commands that run only on tick 0
		
		switch (inst & 0xff00)
		{
			case MUS_FX_EXT:
			{
				// Protracker style Exy commands
			
				switch (inst & 0xfff0)
				{
					case MUS_FX_EXT_FADE_VOLUME_DN:
					{
						if (!(chn->flags & MUS_CHN_DISABLED))
						{
							track_status->volume -= inst & 0xf;
							if (track_status->volume > MAX_VOLUME) track_status->volume = 0;
							
							update_volumes(mus, track_status, chn, cydchn, track_status->volume);
						}
					}
					break;
					
					case MUS_FX_EXT_FADE_VOLUME_UP:
					{
						if (!(chn->flags & MUS_CHN_DISABLED))
						{
							track_status->volume += inst & 0xf;
							if (track_status->volume > MAX_VOLUME) track_status->volume = MAX_VOLUME;
							
							update_volumes(mus, track_status, chn, cydchn, track_status->volume);
						}
					}
					break;
					
					case MUS_FX_EXT_PORTA_UP:
					{
						Uint16 prev = chn->note;
						chn->note += ((inst & 0x0f));
						
						if (prev > chn->note) chn->note = 0xffff;
						
						mus_set_slide(mus, chan, chn->note);
					}
					break;
					
					case MUS_FX_EXT_PORTA_DN:
					{
						Uint16 prev = chn->note;
						chn->note -= ((inst & 0x0f));
						
						if (prev < chn->note) chn->note = 0x0;
						
						mus_set_slide(mus, chan, chn->note);
					}
					break;
				}
			}
			break;
			
			default:
			
			switch (inst & 0xf000)
			{
#ifndef CYD_DISABLE_FILTER			
				case MUS_FX_CUTOFF_FINE_SET:
				{
					track_status->filter_cutoff = (inst & 0xfff);
					if (track_status->filter_cutoff > 0x7ff) track_status->filter_cutoff = 0x7ff;
					cyd_set_filter_coeffs(mus->cyd, cydchn, track_status->filter_cutoff, track_status->filter_resonance);
				}
				break;
#endif
				
#ifndef CYD_DISABLE_WAVETABLE				
				case MUS_FX_WAVETABLE_OFFSET:
				{
					cyd_set_wavetable_offset(cydchn, inst & 0xfff);
				}
				break;
#endif
			}
			
			switch (inst & 0x7f00)
			{
				case MUS_FX_SET_GLOBAL_VOLUME:
				{
					mus->play_volume = my_min((inst & 0xff), MAX_VOLUME);
					
					update_all_volumes(mus);
				}
				break;
				
				case MUS_FX_FADE_GLOBAL_VOLUME:
				{
					mus->play_volume -= inst & 0xf;
					
					if (mus->play_volume > MAX_VOLUME) mus->play_volume = 0;
					
					mus->play_volume += (inst & 0xf0) >> 4;
					
					if (mus->play_volume > MAX_VOLUME) mus->play_volume = MAX_VOLUME;
					
					update_all_volumes(mus);
				}
				break;
				
				case MUS_FX_SET_CHANNEL_VOLUME:
				{
					chn->volume = my_min((inst & 0xff), MAX_VOLUME);
					update_volumes(mus, track_status, chn, cydchn, track_status->volume);
				}
				break;
			
				case MUS_FX_PW_SET:
				{
					track_status->pw = (inst & 0xff) << 4;
				}
				break;
#ifndef CYD_DISABLE_BUZZ				
				case MUS_FX_BUZZ_SHAPE:
				{
					cyd_set_env_shape(cydchn, inst & 3);
				}
				break;
				
				case MUS_FX_BUZZ_SET_SEMI:
				{
					chn->buzz_offset = (((inst & 0xff)) - 0x80) << 8;
						
					mus_set_buzz_frequency(mus, chan, chn->note);
				}
				break;
			
				case MUS_FX_BUZZ_SET:
				{
					chn->buzz_offset = (chn->buzz_offset & 0xff00) | (inst & 0xff);
					
					mus_set_buzz_frequency(mus, chan, chn->note);
				}
				break;
#endif

#ifndef CYD_DISABLE_FM
				case MUS_FX_FM_SET_MODULATION:
				{
					cydchn->fm.adsr.volume = inst % MAX_VOLUME;
				}
				break;
				
				case MUS_FX_FM_SET_HARMONIC:
				{
					cydchn->fm.harmonic = inst % 256;
				}
				break;
				
				case MUS_FX_FM_SET_FEEDBACK:
				{
					cydchn->fm.feedback = inst % 8;
				}
				break;
				
				case MUS_FX_FM_SET_WAVEFORM:
				{
					if ((inst & 255) < CYD_WAVE_MAX_ENTRIES)
					{
						cydchn->fm.wave_entry = &mus->cyd->wavetable_entries[inst & 255];
					}
				}
				break;
#endif
				
#ifdef STEREOOUTPUT
				case MUS_FX_SET_PANNING:
				{
					cyd_set_panning(mus->cyd, cydchn, inst & 0xff);
				}
				break;
#endif

#ifndef CYD_DISABLE_FILTER
				case MUS_FX_FILTER_TYPE:
				{
					cydchn->flttype = (inst & 0xf) % FLT_TYPES;
				}
				break;
			
				case MUS_FX_CUTOFF_SET:
				{
					track_status->filter_cutoff = (inst & 0xff) << 3;
					if (track_status->filter_cutoff > 0x7ff) track_status->filter_cutoff = 0x7ff;
					cyd_set_filter_coeffs(mus->cyd, cydchn, track_status->filter_cutoff, track_status->filter_resonance);
				}
				break;
				
				case MUS_FX_CUTOFF_SET_COMBINED:
				{
					if ((inst & 0xff) < 0x80)
					{
						track_status->filter_cutoff = (inst & 0xff) << 4;
						cydchn->flttype = FLT_LP;
						cyd_set_filter_coeffs(mus->cyd, cydchn, track_status->filter_cutoff, track_status->filter_resonance);
					}
					else
					{
						track_status->filter_cutoff = ((inst & 0xff) - 0x80) << 4;
						cydchn->flttype = FLT_HP;
						cyd_set_filter_coeffs(mus->cyd, cydchn, track_status->filter_cutoff, track_status->filter_resonance);
					}
				}
				break;
				
				case MUS_FX_RESONANCE_SET:
				{
					track_status->filter_resonance = inst & 3;
					cyd_set_filter_coeffs(mus->cyd, cydchn, track_status->filter_cutoff, inst & 3);
				}
				break;
#endif
				
				case MUS_FX_SET_SPEED:
				{
					if (from_program)
					{
						chn->prog_period = inst & 0xff;
					}
					else
					{
						mus->song->song_speed = inst & 0xf;
						if ((inst & 0xf0) == 0) mus->song->song_speed2 = mus->song->song_speed;
						else mus->song->song_speed2 = (inst >> 4) & 0xf;
					}
				}
				break;
				
				case MUS_FX_SET_RATE:
				{
					mus->song->song_rate = inst & 0xff;
					if (mus->song->song_rate < 1) mus->song->song_rate = 1;
					cyd_set_callback_rate(mus->cyd, mus->song->song_rate);
				}
				break;
			
				case MUS_FX_PORTA_UP_SEMI:
				{
					Uint16 prev = chn->note;
					chn->note += (inst&0xff) << 8;
					if (prev > chn->note || chn->note >= (FREQ_TAB_SIZE << 8)) chn->note = ((FREQ_TAB_SIZE-1) << 8);
					mus_set_slide(mus, chan, chn->note);
				}
				break;
				
				case MUS_FX_PORTA_DN_SEMI:
				{
					Uint16 prev = chn->note;
					chn->note -= (inst&0xff) << 8;
					if (prev < chn->note) chn->note = 0x0;
					mus_set_slide(mus, chan, chn->note);
				}
				break;
				
				case MUS_FX_ARPEGGIO_ABS:
				{	
					chn->arpeggio_note = 0;
					chn->fixed_note = (inst & 0xff) << 8;
				}
				break;
				
				case MUS_FX_ARPEGGIO:
				{
					if (chn->fixed_note != 0xffff)
					{
						chn->note = chn->last_note;
						chn->fixed_note = 0xffff;
					}
					
					if ((inst & 0xff) == 0xf0)
						chn->arpeggio_note = track_status->extarp1;
					else if ((inst & 0xff) == 0xf1)
						chn->arpeggio_note = track_status->extarp2;
					else
						chn->arpeggio_note = inst & 0xff;
				}
				break;
				
				case MUS_FX_SET_VOLUME:
				{
					track_status->volume = my_min(MAX_VOLUME, inst & 0xff);
					
					update_volumes(mus, track_status, chn, cydchn, track_status->volume);
				}
				break;
				
				case MUS_FX_SET_SYNCSRC:
				{
					if ((inst & 0xff) != 0xff)
					{
						cydchn->sync_source = (inst & 0xff) % CYD_MAX_FX_CHANNELS;
						cydchn->flags |= CYD_CHN_ENABLE_SYNC;
					}
					else
						cydchn->flags &= ~CYD_CHN_ENABLE_SYNC;
				}
				break;
				
				case MUS_FX_SET_RINGSRC:
				{
					if ((inst & 0xff) != 0xff)
					{
						cydchn->ring_mod = (inst & 0xff) % CYD_MAX_FX_CHANNELS;
						cydchn->flags |= CYD_CHN_ENABLE_RING_MODULATION;
					}
					else
						cydchn->flags &= ~CYD_CHN_ENABLE_RING_MODULATION;
				}
				break;
				
#ifndef CYD_DISABLE_FX
				case MUS_FX_SET_FXBUS:
				{
					cydchn->fx_bus = (inst & 0xff) % CYD_MAX_FX_CHANNELS;
				}
				break;
				
				case MUS_FX_SET_DOWNSAMPLE:
				{
					cydcrush_set(&cyd->fx[cydchn->fx_bus].crush, inst & 0xff, -1, -1, -1);
				}
				break;
#endif

#ifndef CYD_DISABLE_WAVETABLE
				case MUS_FX_SET_WAVETABLE_ITEM:
				{
					if ((inst & 255) < CYD_WAVE_MAX_ENTRIES)
					{
						cydchn->wave_entry = &mus->cyd->wavetable_entries[inst & 255];
					}
				}
				break;
#endif
				
				case MUS_FX_SET_WAVEFORM:
				{
					int final = 0;
					
					if (inst & MUS_FX_WAVE_NOISE)
						final |= CYD_CHN_ENABLE_NOISE;
						
					if (inst & MUS_FX_WAVE_PULSE)
						final |= CYD_CHN_ENABLE_PULSE;
						
					if (inst & MUS_FX_WAVE_TRIANGLE)
						final |= CYD_CHN_ENABLE_TRIANGLE;
						
					if (inst & MUS_FX_WAVE_SAW)
						final |= CYD_CHN_ENABLE_SAW;
						
					if (inst & MUS_FX_WAVE_WAVE)
						final |= CYD_CHN_ENABLE_WAVE;
						
#ifndef CYD_DISABLE_LFSR						
					if (inst & MUS_FX_WAVE_LFSR)
						final |= CYD_CHN_ENABLE_LFSR;
#endif
				
					cyd_set_waveform(cydchn, final);
				}
				break;
				
				case MUS_FX_RESTART_PROGRAM:
				{
					if (!from_program)
					{
						chn->program_counter = 0;
						chn->program_tick = 0;
						chn->program_loop = 1;
					}
				}
				break;
			}
			
			break;
		}
	}
}


static void mus_exec_track_command(MusEngine *mus, int chan, int first_tick)
{
	MusTrackStatus *track_status = &mus->song_track[chan];
	const Uint16 inst = track_status->pattern->step[track_status->pattern_step].command;
	const Uint8 vol = track_status->pattern->step[track_status->pattern_step].volume;
	
	switch (vol & 0xf0)
	{
		case MUS_NOTE_VOLUME_PAN_LEFT:
			do_command(mus, chan, mus->song_counter, MUS_FX_PAN_LEFT | ((Uint16)(vol & 0xf)), 0);
			break;
			
		case MUS_NOTE_VOLUME_PAN_RIGHT:
			do_command(mus, chan, mus->song_counter, MUS_FX_PAN_RIGHT | ((Uint16)(vol & 0xf)), 0);
			break;
			
		case MUS_NOTE_VOLUME_SET_PAN:
			{
			Uint16 val = vol & 0xf;
			Uint16 panning = (val <= 8 ? val * CYD_PAN_CENTER / 8 : (val - 8) * (CYD_PAN_RIGHT - CYD_PAN_CENTER) / 8 + CYD_PAN_CENTER);
			do_command(mus, chan, mus->song_counter, MUS_FX_SET_PANNING | panning, 0);
			debug("Panned to %x", panning);
			}
			break;
			
		case MUS_NOTE_VOLUME_FADE_UP:
			do_command(mus, chan, mus->song_counter, MUS_FX_FADE_VOLUME | ((Uint16)(vol & 0xf) << 4), 0);
			break;
			
		case MUS_NOTE_VOLUME_FADE_DN:
			do_command(mus, chan, mus->song_counter, MUS_FX_FADE_VOLUME | ((Uint16)(vol & 0xf)), 0);
			break;
			
		default:
			if (vol <= MAX_VOLUME)
				do_command(mus, chan, first_tick ? 0 : mus->song_counter, MUS_FX_SET_VOLUME | (Uint16)(vol), 0);
			break;
	}
	
	switch (inst & 0xff00)
	{
		case MUS_FX_ARPEGGIO:
			if (!(inst & 0xff)) break; // no params = use the same settings
		case MUS_FX_SET_EXT_ARP:
		{
			track_status->extarp1 = (inst & 0xf0) >> 4;
			track_status->extarp2 = (inst & 0xf);
		}
		break;
		
		default:
		do_command(mus, chan, mus->song_counter, inst, 0);
		break;
	}
}


static void mus_exec_prog_tick(MusEngine *mus, int chan, int advance)
{		
	MusChannel *chn = &mus->channel[chan];
	int tick = chn->program_tick;
	int visited[MUS_PROG_LEN] = { 0 };
	
	do_it_again:;

	const Uint16 inst = chn->instrument->program[tick];
	
	switch (inst)
	{	
		case MUS_FX_END:
		{
			chn->flags &= ~MUS_CHN_PROGRAM_RUNNING;
			return;
		}
		break;
	}
	
	int dont_reloop = 0;
	
	if(inst != MUS_FX_NOP)
	{
		switch (inst & 0xff00)
		{
			case MUS_FX_JUMP:
			{
				/* This should handle infinite jumping between two jump instructions (program hang) */
			
				if (!visited[tick])
				{
					visited[tick] = 1;
					tick = inst & (MUS_PROG_LEN - 1);
				}
				else return;
			}
			break;
			
			case MUS_FX_LABEL:
			{
				
				
			}
			break;
			
			case MUS_FX_LOOP:
			{
				if (chn->program_loop == (inst & 0xff))
				{
					if (advance) chn->program_loop = 1;
				}
				else
				{
					if (advance) ++chn->program_loop;
					
					int l = 0;
					
					while ((chn->instrument->program[tick] & 0xff00) != MUS_FX_LABEL && tick > 0) 
					{
						--tick;
						if (!(chn->instrument->program[tick] & 0x8000)) ++l;
					}
						
					--tick;
					
					dont_reloop = l <= 1;
				}
			}
			break;
			
			default:
			
			do_command(mus, chan, chn->program_counter, inst, 1);
			
			break;
		}
	}
	
	if (inst == MUS_FX_NOP || (inst & 0xff00) != MUS_FX_JUMP)
	{
		++tick;
		if (tick >= MUS_PROG_LEN)
		{
			tick = 0;
		}
	}

	// skip to next on msb
	
	if ((inst & 0x8000) && inst != MUS_FX_NOP && !dont_reloop)
	{
		goto do_it_again;
	}
	
	if (advance) 
	{
		chn->program_tick = tick;
	}
}

static Sint8 mus_shape(Uint16 position, Uint8 shape)
{
	switch (shape)
	{
		case MUS_SHAPE_SINE:
			return sine_table[position % VIB_TAB_SIZE];
			break;
			
		case MUS_SHAPE_SQUARE:
			return ((position % VIB_TAB_SIZE) & (VIB_TAB_SIZE / 2)) ? -128 : 127;
			break;
			
		case MUS_SHAPE_RAMP_UP:
			return (position % VIB_TAB_SIZE) * 2 - 128;
			break;
			
		case MUS_SHAPE_RAMP_DN:
			return 127 - (position % VIB_TAB_SIZE) * 2;
			break;
			
		default:
		case MUS_SHAPE_RANDOM:
			return rnd_table[(position / 8) % VIB_TAB_SIZE];
			break;
	}
}


#ifndef CYD_DISABLE_PWM

static void do_pwm(MusEngine* mus, int chan)
{
	MusChannel *chn = &mus->channel[chan];
	MusInstrument *ins = chn->instrument;
	MusTrackStatus *track_status = &mus->song_track[chan];

	track_status->pwm_position += ins->pwm_speed;
	mus->cyd->channel[chan].pw = track_status->pw + mus_shape(track_status->pwm_position >> 1, ins->pwm_shape) * ins->pwm_depth / 32;
}

#endif


//***** USE THIS INSIDE MUS_ADVANCE_TICK TO AVOID MUTEX DEADLOCK
int mus_trigger_instrument_internal(MusEngine* mus, int chan, MusInstrument *ins, Uint16 note, int panning)
{
	if (chan == -1)
	{
		for (int i = 0 ; i < mus->cyd->n_channels ; ++i)
		{
			if (!(mus->cyd->channel[i].flags & CYD_CHN_ENABLE_GATE))
				chan = i;
		}
		
		if (chan == -1)
			chan = (rand() %  mus->cyd->n_channels);
	}
	
	CydChannel *cydchn = &mus->cyd->channel[chan];
	MusChannel *chn = &mus->channel[chan];
	MusTrackStatus *track = &mus->song_track[chan];
	
	chn->flags = MUS_CHN_PLAYING | (chn->flags & MUS_CHN_DISABLED);
	if (ins->prog_period > 0) chn->flags |= MUS_CHN_PROGRAM_RUNNING;
	chn->prog_period = ins->prog_period;
	chn->instrument = ins;
	if (!(ins->flags & MUS_INST_NO_PROG_RESTART))
	{
		chn->program_counter = 0;
		chn->program_tick = 0;
		chn->program_loop = 1;
	}
	cydchn->flags = ins->cydflags;
	chn->arpeggio_note = 0;
	chn->fixed_note = 0xffff;
	cydchn->fx_bus = ins->fx_bus;
	
	if (ins->flags & MUS_INST_DRUM)
	{
		cyd_set_waveform(cydchn, CYD_CHN_ENABLE_NOISE);
	}
	
	if (ins->flags & MUS_INST_LOCK_NOTE)
	{
		note = ((Uint16)ins->base_note) << 8;
	}
	else
	{
		note += (Uint16)((int)ins->base_note-MIDDLE_C) << 8;
	}
	
	mus_set_note(mus, chan, ((Uint16)note) + ins->finetune, 1, ins->flags & MUS_INST_QUARTER_FREQ ? 4 : 1);
	chn->last_note = chn->target_note = (((Uint16)note) + ins->finetune);
	chn->current_tick = 0;
	
	track->vibrato_position = 0;
	track->vib_delay = ins->vib_delay;
	
	track->slide_speed = 0;
	
	update_volumes(mus, track, chn, cydchn, (ins->flags & MUS_INST_RELATIVE_VOLUME) ? MAX_VOLUME : ins->volume);
	
	cydchn->sync_source = ins->sync_source == 0xff? chan : ins->sync_source;
	cydchn->ring_mod = ins->ring_mod == 0xff? chan : ins->ring_mod;
	
	if (cydchn->ring_mod >= mus->cyd->n_channels)
		cydchn->ring_mod = mus->cyd->n_channels -1;
		
	if (cydchn->sync_source >= mus->cyd->n_channels)
		cydchn->sync_source = mus->cyd->n_channels -1;
	
	cydchn->flttype = ins->flttype;
	cydchn->lfsr_type = ins->lfsr_type;
	
	if (ins->cydflags & CYD_CHN_ENABLE_KEY_SYNC)
	{
		track->pwm_position = 0;
	}	
	
#ifndef CYD_DISABLE_FILTER
	if (ins->flags & MUS_INST_SET_CUTOFF)
	{
		track->filter_cutoff = ins->cutoff;
		track->filter_resonance = ins->resonance;
		cyd_set_filter_coeffs(mus->cyd, cydchn, ins->cutoff, ins->resonance);
	}
#endif
	
	if (ins->flags & MUS_INST_SET_PW)
	{
		track->pw = ins->pw;
#ifndef CYD_DISABLE_PWM
		do_pwm(mus,chan);
#endif
	}
	
	if (ins->flags & MUS_INST_YM_BUZZ)
	{
#ifndef CYD_DISABLE_BUZZ	
		cydchn->flags |= CYD_CHN_ENABLE_YM_ENV;
		cyd_set_env_shape(cydchn, ins->ym_env_shape);
		mus->channel[chan].buzz_offset = ins->buzz_offset;
#endif
	}
	else
	{
		cydchn->flags &= ~CYD_CHN_ENABLE_YM_ENV;
		
		
		cydchn->adsr.a = ins->adsr.a;
		cydchn->adsr.d = ins->adsr.d;
		cydchn->adsr.s = ins->adsr.s;
		cydchn->adsr.r = ins->adsr.r;
	}
	
#ifndef CYD_DISABLE_WAVETABLE
	if (ins->cydflags & CYD_CHN_ENABLE_WAVE)
	{
		cyd_set_wave_entry(cydchn, &mus->cyd->wavetable_entries[ins->wavetable_entry]);
	}
	else
	{
		cyd_set_wave_entry(cydchn, NULL);
	}

#ifndef CYD_DISABLE_FM	
	if (ins->fm_flags & CYD_FM_ENABLE_WAVE)
	{
		cydfm_set_wave_entry(&cydchn->fm, &mus->cyd->wavetable_entries[ins->fm_wave]);
	}
	else
	{
		cydfm_set_wave_entry(&cydchn->fm, NULL);
	}
#endif
#endif

#ifdef STEREOOUTPUT
	if (panning != -1)
		cyd_set_panning(mus->cyd, cydchn, panning);
		
#endif		

#ifndef CYD_DISABLE_FM
	CydFm *fm = &cydchn->fm;
	
	fm->flags = ins->fm_flags;
	fm->harmonic = ins->fm_harmonic;
	fm->adsr.a = ins->fm_adsr.a;
	fm->adsr.d = ins->fm_adsr.d;
	fm->adsr.s = ins->fm_adsr.s;
	fm->adsr.r = ins->fm_adsr.r;
	fm->adsr.volume = ins->fm_modulation;
	fm->feedback = ins->fm_feedback;
	fm->attack_start = ins->fm_attack_start;
#endif
	
	//cyd_set_frequency(mus->cyd, cydchn, chn->frequency);
	cyd_enable_gate(mus->cyd, cydchn, 1);
	
	return chan;
}


int mus_trigger_instrument(MusEngine* mus, int chan, MusInstrument *ins, Uint16 note, int panning)
{
	cyd_lock(mus->cyd, 1);

	chan = mus_trigger_instrument_internal(mus, chan, ins, note, panning);
	
	cyd_lock(mus->cyd, 0);
	
	return chan;
}


static void mus_advance_channel(MusEngine* mus, int chan)
{
	MusChannel *chn = &mus->channel[chan];
	MusTrackStatus *track_status = &mus->song_track[chan];
		
	if (!(mus->cyd->channel[chan].flags & CYD_CHN_ENABLE_GATE))
	{
		chn->flags &= ~MUS_CHN_PLAYING;
		return;
	}
	
	MusInstrument *ins = chn->instrument;
	
	if (ins->flags & MUS_INST_DRUM && chn->current_tick == 1) 
	{
		cyd_set_waveform(&mus->cyd->channel[chan], ins->cydflags);
	}
		
	if (track_status->slide_speed != 0)
	{
		if (chn->target_note > chn->note)
		{
			chn->note += my_min((Uint16)track_status->slide_speed, chn->target_note - chn->note);
		}
		else if (chn->target_note < chn->note)
		{
			chn->note -= my_min((Uint16)track_status->slide_speed , chn->note - chn->target_note);
		}
	}
		
	++chn->current_tick;
	if (mus->channel[chan].flags & MUS_CHN_PROGRAM_RUNNING)
	{
		int u = (chn->program_counter + 1) >= chn->prog_period;
		mus_exec_prog_tick(mus, chan, u);
		++chn->program_counter;
		if (u) chn->program_counter = 0;
		
		/*++chn->program_counter;	
		if (chn->program_counter >= chn->instrument->prog_period)
		{
			++chn->program_tick;
		
			if (chn->program_tick >= MUS_PROG_LEN)
			{
				chn->program_tick = 0;
			}
			chn->program_counter = 0;
		}*/
	}
	
#ifndef CYD_DISABLE_VIBRATO
	
	Uint8 ctrl = 0;
	int vibdep = my_max(0, (int)ins->vibrato_depth - (int)track_status->vib_delay);
	int vibspd = ins->vibrato_speed;
	
	if (track_status->pattern)
	{
		ctrl = track_status->pattern->step[track_status->pattern_step].ctrl;
		if ((track_status->pattern->step[track_status->pattern_step].command & 0xff00) == MUS_FX_VIBRATO)
		{
			ctrl |= MUS_CTRL_VIB;
			if (track_status->pattern->step[track_status->pattern_step].command & 0xff)
			{
				vibdep = (track_status->pattern->step[track_status->pattern_step].command & 0xf) << 2;
				vibspd = (track_status->pattern->step[track_status->pattern_step].command & 0xf0) >> 2;
				
				if (!vibspd)
					vibspd = ins->vibrato_speed;
				if (!vibdep)
					vibdep = ins->vibrato_depth;
			}
		}
			
		/*do_vib(mus, chan, track_status->pattern->step[track_status->pattern_step].ctrl);
		
		if ((track_status->last_ctrl & MUS_CTRL_VIB) && !(track_status->pattern->step[track_status->pattern_step].ctrl & MUS_CTRL_VIB))
		{
			cyd_set_frequency(mus->cyd, &mus->cyd->channel[chan], mus->channel[chan].frequency);
		}
		
		track_status->last_ctrl = track_status->pattern->step[track_status->pattern_step].ctrl;*/
	}
	
#endif
	
	Sint16 vib = 0;
	
#ifndef CYD_DISABLE_VIBRATO
	if (((ctrl & MUS_CTRL_VIB) && !(ins->flags & MUS_INST_INVERT_VIBRATO_BIT)) || (!(ctrl & MUS_CTRL_VIB) && (ins->flags & MUS_INST_INVERT_VIBRATO_BIT)))
	{
		track_status->vibrato_position += vibspd;
		vib = mus_shape(track_status->vibrato_position >> 1, ins->vib_shape) * vibdep / 64;
		if (track_status->vib_delay) --track_status->vib_delay;
	}
#endif

#ifndef CYD_DISABLE_PWM	
	do_pwm(mus, chan);
#endif
	
	Sint32 note = (mus->channel[chan].fixed_note != 0xffff ? mus->channel[chan].fixed_note : mus->channel[chan].note) + vib + ((Uint16)mus->channel[chan].arpeggio_note << 8);
	
	if (note < 0) note = 0;
	if (note > FREQ_TAB_SIZE << 8) note = (FREQ_TAB_SIZE - 1) << 8;
	
	mus_set_note(mus, chan, note, 0, ins->flags & MUS_INST_QUARTER_FREQ ? 4 : 1);
}


Uint32 mus_ext_sync(MusEngine *mus)
{
	cyd_lock(mus->cyd, 1);
	
	Uint32 s = ++mus->ext_sync_ticks;
	
	cyd_lock(mus->cyd, 0);
	
	return s;
}


int mus_advance_tick(void* udata)
{
	MusEngine *mus = udata;
	if (!(mus->flags & MUS_EXT_SYNC))
		mus->ext_sync_ticks = 1;
	
	while (mus->ext_sync_ticks-- > 0)
	{
		if (mus->song)
		{
			for (int i = 0 ; i < mus->song->num_channels ; ++i)
			{
				MusTrackStatus *track_status = &mus->song_track[i];
				CydChannel *cydchn = &mus->cyd->channel[i];
				MusChannel *muschn = &mus->channel[i];
			
				if (mus->song_counter == 0)
				{
					while (track_status->sequence_position < mus->song->num_sequences[i] && mus->song->sequence[i][track_status->sequence_position].position <= mus->song_position)
					{
						track_status->pattern = &mus->song->pattern[mus->song->sequence[i][track_status->sequence_position].pattern];
						track_status->pattern_step = mus->song_position - mus->song->sequence[i][track_status->sequence_position].position;
						if (track_status->pattern_step >= mus->song->pattern[mus->song->sequence[i][track_status->sequence_position].pattern].num_steps) 
							track_status->pattern = NULL;
						track_status->note_offset = mus->song->sequence[i][track_status->sequence_position].note_offset;
						++track_status->sequence_position;
					}
				}
				
				int delay = 0;
				
				if (track_status->pattern)
				{
					if ((track_status->pattern->step[track_status->pattern_step].command & 0x7FF0) == MUS_FX_EXT_NOTE_DELAY)
						delay = track_status->pattern->step[track_status->pattern_step].command & 0xf;
				}
				
				if (mus->song_counter == delay)
				{			
					if (track_status->pattern)
					{
						
						if (1 || track_status->pattern_step == 0)
						{
							Uint8 note = track_status->pattern->step[track_status->pattern_step].note < 0xf0 ? 
								track_status->pattern->step[track_status->pattern_step].note + track_status->note_offset :
								track_status->pattern->step[track_status->pattern_step].note;
							Uint8 inst = track_status->pattern->step[track_status->pattern_step].instrument;
							MusInstrument *pinst = NULL;
							
							if (inst == MUS_NOTE_NO_INSTRUMENT)
							{
								pinst = muschn->instrument;
							}
							else
							{
								if (inst < mus->song->num_instruments)
								{
									pinst = &mus->song->instrument[inst];
									muschn->instrument = pinst;
								}
							}
							
							if (note == MUS_NOTE_RELEASE)
							{
								cyd_enable_gate(mus->cyd, &mus->cyd->channel[i], 0);
							}
							else if (pinst && note != MUS_NOTE_NONE)
							{
								track_status->slide_speed = 0;
								int speed = pinst->slide_speed | 1;
								Uint8 ctrl = track_status->pattern->step[track_status->pattern_step].ctrl;
								
								if ((track_status->pattern->step[track_status->pattern_step].command & 0xff00) == MUS_FX_SLIDE)
								{
									ctrl |= MUS_CTRL_SLIDE | MUS_CTRL_LEGATO; 
									speed = (track_status->pattern->step[track_status->pattern_step].command & 0xff);
								}
								
								if (ctrl & MUS_CTRL_SLIDE)
								{
									if (ctrl & MUS_CTRL_LEGATO)
									{
										mus_set_slide(mus, i, (((Uint16)note + pinst->base_note - MIDDLE_C) << 8) + pinst->finetune);
									}
									else
									{
										Uint16 oldnote = muschn->note;
										mus_trigger_instrument_internal(mus, i, pinst, note << 8, -1);
										muschn->note = oldnote;
									}
									track_status->slide_speed = speed;
								}
								else if (ctrl & MUS_CTRL_LEGATO)
								{
									mus_set_note(mus, i, (((Uint16)note + pinst->base_note - MIDDLE_C) << 8) + pinst->finetune, 1, pinst->flags & MUS_INST_QUARTER_FREQ ? 4 : 1);
									muschn->target_note = (((Uint16)note + pinst->base_note - MIDDLE_C) << 8) + pinst->finetune;
								}
								else 
								{
									Uint8 prev_vol_track = track_status->volume;
									Uint8 prev_vol_cyd = cydchn->adsr.volume;
									mus_trigger_instrument_internal(mus, i, pinst, note << 8, -1);
									muschn->target_note = (((Uint16)note + pinst->base_note - MIDDLE_C) << 8) + pinst->finetune;
									
									if (inst == MUS_NOTE_NO_INSTRUMENT)
									{
										track_status->volume = prev_vol_track;
										cydchn->adsr.volume = prev_vol_cyd;
									}
								}
								
								if (inst != MUS_NOTE_NO_INSTRUMENT)
								{
									if (pinst->flags & MUS_INST_RELATIVE_VOLUME)
									{
										track_status->volume = MAX_VOLUME;
										cydchn->adsr.volume = (muschn->flags & MUS_CHN_DISABLED) 
											? 0 
											: (int)pinst->volume * (int)mus->volume / MAX_VOLUME * (int)mus->play_volume / MAX_VOLUME * (int)muschn->volume / MAX_VOLUME;
									}
									else
									{
										track_status->volume = pinst->volume;
										cydchn->adsr.volume = (muschn->flags & MUS_CHN_DISABLED) ? 0 : (int)pinst->volume * (int)mus->volume / MAX_VOLUME * (int)mus->play_volume / MAX_VOLUME * (int)muschn->volume / MAX_VOLUME;
									}
								}
							}
						}
					}
				}
				
				if (track_status->pattern) mus_exec_track_command(mus, i, mus->song_counter == delay);
			}
			
			++mus->song_counter;
			if (mus->song_counter >= ((!(mus->song_position & 1)) ? mus->song->song_speed : mus->song->song_speed2))
			{
				for (int i = 0 ; i < mus->cyd->n_channels ; ++i)
				{
					MusTrackStatus *track_status = &mus->song_track[i];
				
					if (track_status->pattern)
					{
						Uint32 command = track_status->pattern->step[track_status->pattern_step].command;
						if ((command & 0xff00) == MUS_FX_LOOP_PATTERN)
						{
							Uint16 step = command & 0xff;
							track_status->pattern_step = step;
						}
						else if ((command & 0xff00) == MUS_FX_SKIP_PATTERN)
						{
							mus->song_position += my_max(track_status->pattern->num_steps - track_status->pattern_step - 1, 0);
							track_status->pattern = NULL;
							track_status->pattern_step = 0;
						}
						else
						{
							++track_status->pattern_step;
						}
						
						if (track_status->pattern && track_status->pattern_step >= track_status->pattern->num_steps)
						{
							track_status->pattern = NULL;
							track_status->pattern_step = 0;
						}
					}
				}
				mus->song_counter = 0;
				++mus->song_position;
				if (mus->song_position >= mus->song->song_length)
				{
					if (mus->song->flags & MUS_NO_REPEAT)
						return 0;
					
					mus->song_position = mus->song->loop_point;
					for (int i = 0 ; i < mus->cyd->n_channels ; ++i)
					{
						MusTrackStatus *track_status = &mus->song_track[i];
						
						track_status->pattern = NULL;
						track_status->pattern_step = 0;
						track_status->sequence_position = 0;
					}
				}
			}
		}
		for (int i = 0 ; i < mus->cyd->n_channels ; ++i)
		{
			if (mus->channel[i].flags & MUS_CHN_PLAYING) 
			{
				mus_advance_channel(mus, i);
			}
		}

#ifndef CYD_DISABLE_MULTIPLEX		
		if (mus->song && (mus->song->flags & MUS_ENABLE_MULTIPLEX) && mus->song->multiplex_period > 0)
		{
			for (int i = 0 ; i < mus->cyd->n_channels ; ++i)
			{
				CydChannel *cydchn = &mus->cyd->channel[i];
				
				if ((mus->multiplex_ctr / mus->song->multiplex_period) == i)
				{
					update_volumes(mus, &mus->song_track[i], &mus->channel[i], cydchn, mus->song_track[i].volume);
				}
				else
				{
					cydchn->adsr.volume = 0;
				}
			}
			
			if (++mus->multiplex_ctr >= mus->song->num_channels * mus->song->multiplex_period)
				mus->multiplex_ctr = 0;
		}
#endif
	}
	
	return 1;
}


void mus_set_song(MusEngine *mus, MusSong *song, Uint16 position)
{
	cyd_lock(mus->cyd, 1);
	cyd_reset(mus->cyd);
	mus->song = song;
	
	if (song != NULL)
	{
		mus->song_counter = 0;
		mus->multiplex_ctr = 0;
#ifndef CYD_DISABLE_INACCURACY
		mus->pitch_mask = (~0) << song->pitch_inaccuracy;
#endif
	}
	
	mus->song_position = position;
	mus->play_volume = MAX_VOLUME;
	
	for (int i = 0 ; i < MUS_MAX_CHANNELS ; ++i)
	{
		mus->song_track[i].pattern = NULL;
		mus->song_track[i].pattern_step = 0;
		mus->song_track[i].sequence_position = 0;
		mus->song_track[i].last_ctrl = 0;
		mus->song_track[i].note_offset = 0;
		mus->song_track[i].extarp1 = mus->song_track[i].extarp2 = 0;
		
		if (song)
		{
			mus->channel[i].volume = song->default_volume[i];
#ifdef STEREOOUTPUT
			if (i < mus->cyd->n_channels)
				cyd_set_panning(mus->cyd, &mus->cyd->channel[i], song->default_panning[i] + CYD_PAN_CENTER);
#endif
		}
		else
		{
			mus->channel[i].volume = MAX_VOLUME;
		}
	}
	
	cyd_lock(mus->cyd, 0);
}


int mus_poll_status(MusEngine *mus, int *song_position, int *pattern_position, MusPattern **pattern, MusChannel *channel, int *cyd_env, int *mus_note, Uint64 *time_played)
{
	cyd_lock(mus->cyd, 1);
	
	if (song_position) *song_position = mus->song_position;
	
	if (pattern_position)
	{
		for (int i = 0 ; i < MUS_MAX_CHANNELS ; ++i)
		{
			pattern_position[i] = mus->song_track[i].pattern_step;
		}
	}
	
	if (pattern)
	{
		for (int i = 0 ; i < MUS_MAX_CHANNELS ; ++i)
		{
			pattern[i] = mus->song_track[i].pattern;
		}
	}
	
	if (channel)
	{
		memcpy(channel, mus->channel, sizeof(mus->channel));
	}
	
	if (cyd_env)
	{
		for (int i = 0 ; i < my_min(mus->cyd->n_channels, MUS_MAX_CHANNELS) ; ++i)
		{
			if (mus->cyd->channel[i].flags & CYD_CHN_ENABLE_YM_ENV)
				cyd_env[i] = mus->cyd->channel[i].adsr.volume;
			else
				cyd_env[i] = cyd_env_output(mus->cyd, mus->cyd->channel[i].flags, &mus->cyd->channel[i].adsr, MAX_VOLUME);
		}
	}
	
	if (mus_note)
	{
		for (int i = 0 ; i < my_min(mus->cyd->n_channels, MUS_MAX_CHANNELS) ; ++i)
		{
			mus_note[i] = mus->channel[i].note;
		}
	}
	
	if (time_played)
	{
		*time_played = mus->cyd->samples_played * 1000 / mus->cyd->sample_rate;
	}
	
	cyd_lock(mus->cyd, 0);
	
	return mus->song != NULL;
}


int mus_load_instrument(const char *path, MusInstrument *inst, CydWavetableEntry *wavetable_entries)
{
	RWops *ctx = RWFromFile(path, "rb");
	
	if (ctx)
	{
		int r = mus_load_instrument_RW2(ctx, inst, wavetable_entries);
	
		my_RWclose(ctx);
		
		return r;
	}
	
	return 0;
}


static void load_wavetable_entry(Uint8 version, CydWavetableEntry * e, RWops *ctx)
{
	VER_READ(version, 12, 0xff, &e->flags, 0);
	VER_READ(version, 12, 0xff, &e->sample_rate, 0);
	VER_READ(version, 12, 0xff, &e->samples, 0);
	VER_READ(version, 12, 0xff, &e->loop_begin, 0);
	VER_READ(version, 12, 0xff, &e->loop_end, 0);
	VER_READ(version, 12, 0xff, &e->base_note, 0);
	
	FIX_ENDIAN(e->flags);
	FIX_ENDIAN(e->sample_rate);
	FIX_ENDIAN(e->samples);
	FIX_ENDIAN(e->loop_begin);
	FIX_ENDIAN(e->loop_end);
	FIX_ENDIAN(e->base_note);
	
	if (e->samples > 0)
	{
		if (version < 15)
		{
			Sint16 *data = malloc(sizeof(data[0]) * e->samples);
			
			my_RWread(ctx, data, sizeof(data[0]), e->samples);
			
			cyd_wave_entry_init(e, data, e->samples, CYD_WAVE_TYPE_SINT16, 1, 1, 1);
			
			free(data);
		}
		else
		{
			Uint32 data_size; 
			VER_READ(version, 15, 0xff, &data_size, 0);
			FIX_ENDIAN(data_size);
			Uint8 *compressed = malloc(sizeof(Uint8) * data_size);
			
			my_RWread(ctx, compressed, sizeof(Uint8), (data_size + 7) / 8); // data_size is in bits
			
			Sint16 *data = NULL;

#ifndef CYD_DISABLE_WAVETABLE
			data = bitunpack(compressed, data_size, e->samples, (e->flags >> 3) & 3);
#endif
			
			if (data)
			{
				cyd_wave_entry_init(e, data, e->samples, CYD_WAVE_TYPE_SINT16, 1, 1, 1);
				free(data);
			}
			else
			{
				warning("Sample data unpack failed");
			}
			
			free(compressed);
		}
	}
}


static int find_and_load_wavetable(Uint8 version, RWops *ctx, CydWavetableEntry *wavetable_entries)
{
	for (int i = 0 ; i < CYD_WAVE_MAX_ENTRIES ; ++i)
	{
		CydWavetableEntry *e = &wavetable_entries[i];
			
		if (e->samples == 0)
		{
			load_wavetable_entry(version, e, ctx);
			return i;
		}
	}
	
	return 0;
}


int mus_load_instrument_RW(Uint8 version, RWops *ctx, MusInstrument *inst, CydWavetableEntry *wavetable_entries)
{
	mus_get_default_instrument(inst);
	
	debug("Loading instrument at offset %x", (Uint32)my_RWtell(ctx));

	_VER_READ(&inst->flags, 0);
	_VER_READ(&inst->cydflags, 0);
	_VER_READ(&inst->adsr, 0);
	_VER_READ(&inst->sync_source, 0);
	_VER_READ(&inst->ring_mod, 0); 
	_VER_READ(&inst->pw, 0);
	_VER_READ(&inst->volume, 0);
	Uint8 progsteps = 0;
	_VER_READ(&progsteps, 0);
	if (progsteps)
		_VER_READ(&inst->program, (int)progsteps*sizeof(inst->program[0]));
	_VER_READ(&inst->prog_period, 0); 
	_VER_READ(&inst->vibrato_speed, 0); 
	_VER_READ(&inst->vibrato_depth, 0); 
	_VER_READ(&inst->pwm_speed, 0); 
	_VER_READ(&inst->pwm_depth, 0); 
	_VER_READ(&inst->slide_speed, 0);
	_VER_READ(&inst->base_note, 0);
	
	if (version >= 20)
		_VER_READ(&inst->finetune, 0);
	
	Uint8 len = 16;
	VER_READ(version, 11, 0xff, &len, 0);
	if (len)
	{
		memset(inst->name, 0, sizeof(inst->name));
		_VER_READ(inst->name, my_min(len, sizeof(inst->name)));
		inst->name[sizeof(inst->name) - 1] = '\0';
	}
	
	VER_READ(version, 1, 0xff, &inst->cutoff, 0);
	VER_READ(version, 1, 0xff, &inst->resonance, 0);
	VER_READ(version, 1, 0xff, &inst->flttype, 0);
	VER_READ(version, 7, 0xff, &inst->ym_env_shape, 0);
	VER_READ(version, 7, 0xff, &inst->buzz_offset, 0);
	VER_READ(version, 10, 0xff, &inst->fx_bus, 0);
	VER_READ(version, 11, 0xff, &inst->vib_shape, 0);
	VER_READ(version, 11, 0xff, &inst->vib_delay, 0);
	VER_READ(version, 11, 0xff, &inst->pwm_shape, 0);
	VER_READ(version, 18, 0xff, &inst->lfsr_type, 0);
	VER_READ(version, 12, 0xff, &inst->wavetable_entry, 0);
	
	VER_READ(version, 23, 0xff, &inst->fm_flags, 0);
	VER_READ(version, 23, 0xff, &inst->fm_modulation, 0);
	VER_READ(version, 23, 0xff, &inst->fm_feedback, 0);
	VER_READ(version, 23, 0xff, &inst->fm_harmonic, 0);
	VER_READ(version, 23, 0xff, &inst->fm_adsr, 0);
	VER_READ(version, 25, 0xff, &inst->fm_attack_start, 0);
	VER_READ(version, 23, 0xff, &inst->fm_wave, 0);

#ifndef CYD_DISABLE_WAVETABLE	
	if (wavetable_entries)
	{
		if (inst->wavetable_entry == 0xff)
		{
			inst->wavetable_entry = find_and_load_wavetable(version, ctx, wavetable_entries);
		}
		
		if (version >= 23)
		{
			if (inst->fm_wave == 0xff)
			{
				inst->fm_wave = find_and_load_wavetable(version, ctx, wavetable_entries);
			}
			else if (inst->fm_wave == 0xfe)
			{
				inst->fm_wave = inst->wavetable_entry;
			}
		}
	}
#endif
	
	/* The file format is little-endian, the following only does something on big-endian machines */
	
	FIX_ENDIAN(inst->flags);
	FIX_ENDIAN(inst->cydflags);
	FIX_ENDIAN(inst->pw);
	FIX_ENDIAN(inst->cutoff);
	FIX_ENDIAN(inst->buzz_offset);
	
	for (int i = 0 ; i < progsteps ; ++i)
		FIX_ENDIAN(inst->program[i]);
		
	FIX_ENDIAN(inst->fm_flags);
	
	if (version < 26)
	{
		inst->adsr.a *= ENVELOPE_SCALE;
		inst->adsr.d *= ENVELOPE_SCALE;
		inst->adsr.r *= ENVELOPE_SCALE;
		
		inst->fm_adsr.a *= ENVELOPE_SCALE;
		inst->fm_adsr.d *= ENVELOPE_SCALE;
		inst->fm_adsr.r *= ENVELOPE_SCALE;
	}
	
	return 1;
}


int mus_load_instrument_RW2(RWops *ctx, MusInstrument *inst, CydWavetableEntry *wavetable_entries)
{
	char id[9];
				
	id[8] = '\0';

	my_RWread(ctx, id, 8, sizeof(id[0]));
	
	if (strcmp(id, MUS_INST_SIG) == 0)
	{
		Uint8 version = 0;
		my_RWread(ctx, &version, 1, sizeof(version));
		
		if (version > MUS_VERSION)
			return 0;
	
		mus_load_instrument_RW(version, ctx, inst, wavetable_entries);
		
		return 1;
	}
	else
	{
		debug("Instrument signature does not match");
		return 0;
	}
}


void mus_get_default_instrument(MusInstrument *inst)
{
	memset(inst, 0, sizeof(*inst));
	inst->flags = MUS_INST_DRUM|MUS_INST_SET_PW|MUS_INST_SET_CUTOFF;
	inst->pw = 0x600;
	inst->cydflags = CYD_CHN_ENABLE_TRIANGLE;
	inst->adsr.a = 1 * ENVELOPE_SCALE;
	inst->adsr.d = 12 * ENVELOPE_SCALE;
	inst->volume = MAX_VOLUME;
	inst->base_note = MIDDLE_C;
	inst->finetune = 0;
	inst->prog_period = 2;
	inst->cutoff = 2047;
	inst->slide_speed = 0x80;
	inst->vibrato_speed = 0x20;
	inst->vibrato_depth = 0x20;
	inst->vib_shape = MUS_SHAPE_SINE;
	inst->vib_delay = 0;
	
	for (int p = 0 ; p < MUS_PROG_LEN; ++p)
		inst->program[p] = MUS_FX_NOP;
}


void mus_set_fx(MusEngine *mus, MusSong *song)
{
	cyd_lock(mus->cyd, 1);
	for(int f = 0 ; f < CYD_MAX_FX_CHANNELS ; ++f)
	{
		cydfx_set(&mus->cyd->fx[f], &song->fx[f]);
	}

#ifndef CYD_DISABLE_INACCURACY	
	mus->pitch_mask = (~0) << song->pitch_inaccuracy;
#endif
	
	cyd_lock(mus->cyd, 0);
}


static void inner_load_fx(RWops *ctx, CydFxSerialized *fx, int version)
{
	Uint8 padding;
	
	debug("fx @ %u", (Uint32)my_RWtell(ctx));
	
	if (version >= 22)
	{
		Uint8 len = 16;
		my_RWread(ctx, &len, 1, 1);
		if (len)
		{
			memset(fx->name, 0, sizeof(fx->name));
			_VER_READ(fx->name, my_min(len, sizeof(fx->name)));
			fx->name[sizeof(fx->name) - 1] = '\0';
		}
	}

	my_RWread(ctx, &fx->flags, 1, 4);
	my_RWread(ctx, &fx->crush.bit_drop, 1, 1);
	my_RWread(ctx, &fx->chr.rate, 1, 1);
	my_RWread(ctx, &fx->chr.min_delay, 1, 1);
	my_RWread(ctx, &fx->chr.max_delay, 1, 1);
	my_RWread(ctx, &fx->chr.sep, 1, 1);
	
	Uint8 spread = 0;
	
	if (version < 27)
		my_RWread(ctx, &spread, 1, 1);
	
	if (version < 21)
		my_RWread(ctx, &padding, 1, 1);
	
	int taps = CYDRVB_TAPS;
	
	if (version < 27)
		taps = 8;
	
	for (int i = 0 ; i < taps ; ++i)	
	{
		my_RWread(ctx, &fx->rvb.tap[i].delay, 2, 1);
		my_RWread(ctx, &fx->rvb.tap[i].gain, 2, 1);
		
		if (version >= 27)
		{
			my_RWread(ctx, &fx->rvb.tap[i].panning, 1, 1);
			my_RWread(ctx, &fx->rvb.tap[i].flags, 1, 1);
		}
		else
		{
			fx->rvb.tap[i].flags = 1;
			
			if (spread > 0)
				fx->rvb.tap[i].panning = CYD_PAN_LEFT;
			else
				fx->rvb.tap[i].panning = CYD_PAN_CENTER;
		}
		
		FIX_ENDIAN(fx->rvb.tap[i].gain);
		FIX_ENDIAN(fx->rvb.tap[i].delay);
	}
	
	if (version < 27)
	{
		if (spread == 0)
		{
			for (int i = 8 ; i < CYDRVB_TAPS ; ++i)
			{
				fx->rvb.tap[i].flags = 0;
				fx->rvb.tap[i].delay = 1000;
				fx->rvb.tap[i].gain = CYDRVB_LOW_LIMIT;
			}
		}
		else
		{
			for (int i = 8 ; i < CYDRVB_TAPS ; ++i)
			{
				fx->rvb.tap[i].flags = 1;
				fx->rvb.tap[i].panning = CYD_PAN_RIGHT;
				fx->rvb.tap[i].delay = my_min(CYDRVB_SIZE, fx->rvb.tap[i - 8].delay + (fx->rvb.tap[i - 8].delay * spread) / 2000);
				fx->rvb.tap[i].gain = fx->rvb.tap[i - 8].gain;
			}
		}
	}

	my_RWread(ctx, &fx->crushex.downsample, 1, 1); 
	
	if (version < 19)
	{
		fx->crushex.gain = 128;
	}
	else
	{
		my_RWread(ctx, &fx->crushex.gain, 1, 1);
	}
	
	FIX_ENDIAN(fx->flags);
}


int mus_load_fx_RW(RWops *ctx, CydFxSerialized *fx)
{
	char id[9];
	id[8] = '\0';

	my_RWread(ctx, id, 8, sizeof(id[0]));
	
	if (strcmp(id, MUS_FX_SIG) == 0)
	{
		Uint8 version = 0;
		my_RWread(ctx, &version, 1, sizeof(version));
		
		debug("FX version = %u", version);
		
		inner_load_fx(ctx, fx, version);
		
		return 1;
	}
	else
		return 0;
}


int mus_load_fx_file(FILE *f, CydFxSerialized *fx)
{
	RWops *rw = RWFromFP(f, 0);
	
	if (rw)
	{
		int r = mus_load_fx_RW(rw, fx);
		
		my_RWclose(rw);
		
		return r;
	}
	
	return 0;
}


int mus_load_fx(const char *path, CydFxSerialized *fx)
{
	RWops *rw = RWFromFile(path, "rb");
	
	if (rw)
	{
		int r = mus_load_fx_RW(rw, fx);
		
		my_RWclose(rw);
		
		return r;
	}
	
	return 0;
}


int mus_load_song_RW(RWops *ctx, MusSong *song, CydWavetableEntry *wavetable_entries)
{
	char id[9];
	id[8] = '\0';

	my_RWread(ctx, id, 8, sizeof(id[0]));
	
	if (strcmp(id, MUS_SONG_SIG) == 0)
	{
		Uint8 version = 0;
		my_RWread(ctx, &version, 1, sizeof(version));
		
		debug("Song version = %u", version);
		
		if (version > MUS_VERSION)
		{
			debug("Unsupported song version");
			return 0;
		}
		
		if (version >= 6) 
			my_RWread(ctx, &song->num_channels, 1, sizeof(song->num_channels));
		else 
		{
			if (version > 3) 
				song->num_channels = 4;
			else 
				song->num_channels = 3;
		}	
		
		my_RWread(ctx, &song->time_signature, 1, sizeof(song->time_signature));
		
		if (version >= 17)
		{
			my_RWread(ctx, &song->sequence_step, 1, sizeof(song->sequence_step));
		}
		
		my_RWread(ctx, &song->num_instruments, 1, sizeof(song->num_instruments));
		my_RWread(ctx, &song->num_patterns, 1, sizeof(song->num_patterns));
		my_RWread(ctx, song->num_sequences, 1, sizeof(song->num_sequences[0]) * (int)song->num_channels);
		my_RWread(ctx, &song->song_length, 1, sizeof(song->song_length));
		
		my_RWread(ctx, &song->loop_point, 1, sizeof(song->loop_point));
		
		if (version >= 12)
			my_RWread(ctx, &song->master_volume, 1, 1);
		
		my_RWread(ctx, &song->song_speed, 1, sizeof(song->song_speed));
		my_RWread(ctx, &song->song_speed2, 1, sizeof(song->song_speed2));
		my_RWread(ctx, &song->song_rate, 1, sizeof(song->song_rate));
		
		if (version > 2) my_RWread(ctx, &song->flags, 1, sizeof(song->flags));
		else song->flags = 0;
		
		if (version >= 9) my_RWread(ctx, &song->multiplex_period, 1, sizeof(song->multiplex_period));
		else song->multiplex_period = 3;
		
		if (version >= 16)
		{
			my_RWread(ctx, &song->pitch_inaccuracy, 1, sizeof(song->pitch_inaccuracy));
		}
		else
		{
			song->pitch_inaccuracy = 0;
		}
		
		/* The file format is little-endian, the following only does something on big-endian machines */
		
		FIX_ENDIAN(song->song_length);
		FIX_ENDIAN(song->loop_point);
		FIX_ENDIAN(song->time_signature);
		FIX_ENDIAN(song->sequence_step);
		FIX_ENDIAN(song->num_patterns);
		FIX_ENDIAN(song->flags);
		
		for (int i = 0 ; i < (int)song->num_channels ; ++i)
			FIX_ENDIAN(song->num_sequences[i]);
		
		Uint8 title_len = 16 + 1; // old length
		
		if (version >= 11)
		{
			my_RWread(ctx, &title_len, 1, 1);
		}
		
		if (version >= 5) 
		{
			memset(song->title, 0, sizeof(song->title));
			my_RWread(ctx, song->title, 1, my_min(sizeof(song->title), title_len));
			song->title[sizeof(song->title) - 1] = '\0';
		}
		
		Uint8 n_fx = 0;
		
		if (version >= 10)
			my_RWread(ctx, &n_fx, 1, sizeof(n_fx));
		else if (song->flags & MUS_ENABLE_REVERB)
			n_fx = 1;
		
		if (n_fx > 0)
		{
			debug("Song has %u effects", n_fx);
			if (version >= 10)
			{
				memset(&song->fx, 0, sizeof(song->fx[0]) * n_fx);
				
				debug("Loading fx at offset %x (%d/%d)", (Uint32)my_RWtell(ctx), (int)sizeof(song->fx[0]) * n_fx, (int)sizeof(song->fx[0]));
				
				for (int fx = 0 ; fx < n_fx ; ++fx)
					inner_load_fx(ctx, &song->fx[fx], version);
			}
			else
			{
				for (int fx = 0 ; fx < n_fx ; ++fx)
				{
					song->fx[fx].flags = CYDFX_ENABLE_REVERB;
					if (song->flags & MUS_ENABLE_CRUSH) song->fx[fx].flags |= CYDFX_ENABLE_CRUSH;
					
					for (int i = 0 ; i < 8 ; ++i)	
					{
						Sint32 g, d;
						my_RWread(ctx, &g, 1, sizeof(g));
						my_RWread(ctx, &d, 1, sizeof(d));
							
						song->fx[fx].rvb.tap[i].gain = g;
						song->fx[fx].rvb.tap[i].delay = d;
						song->fx[fx].rvb.tap[i].panning = CYD_PAN_CENTER;
						song->fx[fx].rvb.tap[i].flags = 1;
						
						FIX_ENDIAN(song->fx[fx].rvb.tap[i].gain);
						FIX_ENDIAN(song->fx[fx].rvb.tap[i].delay);
					}
				}
			}
		}
		
		
		for (int i = 0 ; i < MUS_MAX_CHANNELS ; ++i)
		{
			song->default_volume[i] = MAX_VOLUME;
			song->default_panning[i] = 0;
		}
		
		if (version >= 13)
		{
			debug("Loading default volumes at offset %x", (Uint32)my_RWtell(ctx));
			my_RWread(ctx, &song->default_volume[0], sizeof(song->default_volume[0]), song->num_channels);
			debug("Loading default panning at offset %x", (Uint32)my_RWtell(ctx));
			my_RWread(ctx, &song->default_panning[0], sizeof(song->default_panning[0]), song->num_channels);
		}
		
		if (song->instrument == NULL)
		{
			song->instrument = malloc((size_t)song->num_instruments * sizeof(song->instrument[0]));
		}
		
		for (int i = 0 ; i < song->num_instruments; ++i)
		{
			mus_load_instrument_RW(version, ctx, &song->instrument[i], NULL); 
		}
		
		
		for (int i = 0 ; i < song->num_channels ; ++i)
		{
			if (song->num_sequences[i] > 0)
			{
				if (song->sequence[i] == NULL)
					song->sequence[i] = malloc((size_t)song->num_sequences[i] * sizeof(song->sequence[0][0]));
			
				if (version < 8)
				{
					my_RWread(ctx, song->sequence[i], song->num_sequences[i], sizeof(song->sequence[i][0]));
				}
				else
				{
					for (int s = 0 ; s < song->num_sequences[i] ; ++s)
					{
						my_RWread(ctx, &song->sequence[i][s].position, 1, sizeof(song->sequence[i][s].position));
						my_RWread(ctx, &song->sequence[i][s].pattern, 1, sizeof(song->sequence[i][s].pattern));
						my_RWread(ctx, &song->sequence[i][s].note_offset, 1, sizeof(song->sequence[i][s].note_offset));
					}
				}
				
				for (int s = 0 ; s < song->num_sequences[i] ; ++s)
				{
					FIX_ENDIAN(song->sequence[i][s].position);
					FIX_ENDIAN(song->sequence[i][s].pattern);
				}
			}
		}
		
		if (song->pattern == NULL)
		{
			song->pattern = calloc((size_t)song->num_patterns, sizeof(song->pattern[0]));
			//memset(song->pattern, 0, (size_t)song->num_patterns * sizeof(song->pattern[0]));
		}
		
		for (int i = 0 ; i < song->num_patterns; ++i)
		{
			Uint16 steps;
			my_RWread(ctx, &steps, 1, sizeof(song->pattern[i].num_steps));
			
			FIX_ENDIAN(steps);
			
			if (song->pattern[i].step == NULL)
				song->pattern[i].step = calloc((size_t)steps, sizeof(song->pattern[i].step[0]));
			else if (steps > song->pattern[i].num_steps)
				song->pattern[i].step = realloc(song->pattern[i].step, (size_t)steps * sizeof(song->pattern[i].step[0]));
				
			song->pattern[i].num_steps = steps;
			
			if (version >= 24)
				my_RWread(ctx, &song->pattern[i].color, 1, sizeof(song->pattern[i].color));
			else
				song->pattern[i].color = 0;
			
			if (version < 8)
			{
				size_t s = sizeof(song->pattern[i].step[0]);
				if (version < 2) 
					s = sizeof(Uint8)*3;
				else
					s = sizeof(Uint8)*3 + sizeof(Uint16) + 1; // aligment issue in version 6 songs
					
				for (int step = 0 ; step < song->pattern[i].num_steps ; ++step)
				{
					my_RWread(ctx, &song->pattern[i].step[step], 1, s);
					FIX_ENDIAN(song->pattern[i].step[step].command);
				}
			}
			else
			{
				int len = song->pattern[i].num_steps / 2 + (song->pattern[i].num_steps & 1);
				
				Uint8 *packed = malloc(sizeof(Uint8) * len);
				Uint8 *current = packed;
				
				my_RWread(ctx, packed, sizeof(Uint8), len);
				
				for (int s = 0 ; s < song->pattern[i].num_steps ; ++s)
				{
					Uint8 bits = (s & 1 || s == song->pattern[i].num_steps - 1) ? (*current & 0xf) : (*current >> 4);
					
					if (bits & MUS_PAK_BIT_NOTE)
						my_RWread(ctx, &song->pattern[i].step[s].note, 1, sizeof(song->pattern[i].step[s].note));
					else
						song->pattern[i].step[s].note = MUS_NOTE_NONE;
						
					if (bits & MUS_PAK_BIT_INST)
						my_RWread(ctx, &song->pattern[i].step[s].instrument, 1, sizeof(song->pattern[i].step[s].instrument));
					else
						song->pattern[i].step[s].instrument = MUS_NOTE_NO_INSTRUMENT;
						
					if (bits & MUS_PAK_BIT_CTRL)
					{
						my_RWread(ctx, &song->pattern[i].step[s].ctrl, 1, sizeof(song->pattern[i].step[s].ctrl));
						
						if (version >= 14)
							bits |= song->pattern[i].step[s].ctrl & ~7;
						
						song->pattern[i].step[s].ctrl &= 7;
					}
					else
						song->pattern[i].step[s].ctrl = 0;
						
					if (bits & MUS_PAK_BIT_CMD)
						my_RWread(ctx, &song->pattern[i].step[s].command, 1, sizeof(song->pattern[i].step[s].command));
					else
						song->pattern[i].step[s].command = 0;
						
					FIX_ENDIAN(song->pattern[i].step[s].command);
					
					if (bits & MUS_PAK_BIT_VOLUME)
					{
						my_RWread(ctx, &song->pattern[i].step[s].volume, 1, sizeof(song->pattern[i].step[s].volume));
					}
					else
					{
						song->pattern[i].step[s].volume = MUS_NOTE_NO_VOLUME;	
					}

					if (s & 1) ++current;
				}
				
				free(packed);
			}
		}
		
		for (int i = 0 ; i < CYD_WAVE_MAX_ENTRIES ; ++i)
		{
			cyd_wave_entry_init(&wavetable_entries[i], NULL, 0, 0, 0, 0, 0);
		}
		
		if (version >= 12)
		{
			Uint8 max_wt = 0;
			my_RWread(ctx, &max_wt, 1, sizeof(Uint8));
					
			for (int i = 0 ; i < max_wt ; ++i)
			{
				load_wavetable_entry(version, &wavetable_entries[i], ctx);
			}
			
			song->wavetable_names = malloc(max_wt * sizeof(char*));
			
			if (version >= 26)
			{
				for (int i = 0 ; i < max_wt ; ++i)
				{
					Uint8 len = 0;
					song->wavetable_names[i] = malloc(MUS_WAVETABLE_NAME_LEN + 1);
					memset(song->wavetable_names[i], 0, MUS_WAVETABLE_NAME_LEN + 1);
					
					my_RWread(ctx, &len, 1, 1);
					my_RWread(ctx, song->wavetable_names[i], len, sizeof(char));
				}
			}
			else
			{
				for (int i = 0 ; i < max_wt ; ++i)
				{
					song->wavetable_names[i] = malloc(MUS_WAVETABLE_NAME_LEN + 1);
					memset(song->wavetable_names[i], 0, MUS_WAVETABLE_NAME_LEN + 1);
				}
			}
			
			song->num_wavetables = max_wt;
		}
		else
			song->num_wavetables = 0;
		
		return 1;
	}
	
	return 0;
}


int mus_load_song(const char *path, MusSong *song, CydWavetableEntry *wavetable_entries)
{
	RWops *ctx = RWFromFile(path, "rb");
	
	if (ctx)
	{	
		int r = mus_load_song_RW(ctx, song, wavetable_entries);
		my_RWclose(ctx);
		
		return r;
	}
	
	return 0;
}


void mus_free_song(MusSong *song)
{
	free(song->instrument);
	
	for (int i = 0 ; i < MUS_MAX_CHANNELS; ++i)
	{
		free(song->sequence[i]);
	}
	
	for (int i = 0 ; i < song->num_patterns; ++i)
	{
		free(song->pattern[i].step);
	}
	
	for (int i = 0 ; i < song->num_wavetables; ++i)
	{
		free(song->wavetable_names[i]);
	}
	
	free(song->wavetable_names);
	
	free(song->pattern);
}


void mus_release(MusEngine *mus, int chan)
{	
	cyd_lock(mus->cyd, 1);
	cyd_enable_gate(mus->cyd, &mus->cyd->channel[chan], 0);
	
	cyd_lock(mus->cyd, 0);
}


int mus_load_instrument_file(Uint8 version, FILE *f, MusInstrument *inst, CydWavetableEntry *wavetable_entries)
{
	RWops *rw = RWFromFP(f, 0);
	
	if (rw)
	{
		int r = mus_load_instrument_RW(version, rw, inst, wavetable_entries);
		
		my_RWclose(rw);
		
		return r;
	}
	
	return 0;
}


int mus_load_instrument_file2(FILE *f, MusInstrument *inst, CydWavetableEntry *wavetable_entries)
{
	RWops *rw = RWFromFP(f, 0);
	
	if (rw)
	{
		int r = mus_load_instrument_RW2(rw, inst, wavetable_entries);
		
		my_RWclose(rw);
		
		return r;
	}
	
	return 0;
}


int mus_load_song_file(FILE *f, MusSong *song, CydWavetableEntry *wavetable_entries)
{
	RWops *rw = RWFromFP(f, 0);
	
	if (rw)
	{
		int r = mus_load_song_RW(rw, song, wavetable_entries);
		
		my_RWclose(rw);
		
		return r;
	}
	
	return 0;
}


Uint32 mus_get_playtime_at(MusSong *song, int position)
{
	Uint32 ticks = 0;
	int pos = 0;
	int seq_pos[MUS_MAX_CHANNELS] = {0}, pattern_pos[MUS_MAX_CHANNELS] = {0};
	MusPattern *pattern[MUS_MAX_CHANNELS] = {0};
	int spd1 = song->song_speed, spd2 = song->song_speed2, rate = song->song_rate;
	
	while (pos < position)
	{
		for (int t = 0 ; t < song->num_channels ; ++t)
		{
			if (seq_pos[t] < song->num_sequences[t])
			{
				if (song->sequence[t][seq_pos[t]].position == pos)
				{
					if (seq_pos[t] < song->num_sequences[t])
					{
						pattern_pos[t] = 0;
						
						pattern[t] = &song->pattern[song->sequence[t][seq_pos[t]].pattern];
						
						seq_pos[t]++;
					}
				}
				
				if (pattern[t] && pattern_pos[t] < pattern[t]->num_steps)
				{
					Uint16 command = pattern[t]->step[pattern_pos[t]].command;
					
					if ((command & 0xff00) == MUS_FX_SET_SPEED)
					{
						spd1 = command & 0xf;
						spd2 = (command & 0xf0) >> 4;
						
						if (!spd2)
							spd2 = spd1;
					}
					else if ((command & 0xff00) == MUS_FX_SET_RATE)
					{
						rate = command & 0xff;
						
						if (rate < 1)
							rate = 1;
					}
					else if ((command & 0xff00) == MUS_FX_SKIP_PATTERN)
					{
						pos += pattern[t]->num_steps - 1 - pattern_pos[t];
					}
				}
				
			}
			
			pattern_pos[t]++;
		}
		
		int spd = pos & 1 ? spd2 : spd1;
		
		ticks += (1000 * spd) / rate;
		
		++pos;
	}
	
	return ticks;
}


void mus_set_channel_volume(MusEngine* mus, int chan, int volume)
{
	MusChannel *chn = &mus->channel[chan];
	CydChannel *cydchn = &mus->cyd->channel[chan];
	MusTrackStatus *track_status = &mus->song_track[chan];
	
	chn->volume = my_min(volume, MAX_VOLUME);
	update_volumes(mus, track_status, chn, cydchn, track_status->volume);
}	
