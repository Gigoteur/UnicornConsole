/*
Copyright (c) 2009-2011 Tero Lindeman (kometbomb)

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

#include "cyd.h"
#include "macros.h"
#include <assert.h>
#include <stdlib.h>
#include <string.h>

#ifdef ENABLEAUDIODUMP
#include <time.h>
#endif

#include "cyddefs.h"
#include "cydwave.h"
#include "freqs.h"
#include "cydosc.h"

#ifndef USENATIVEAPIS
# ifndef NOSDL_MIXER
# include "SDL_mixer.h"
# endif
#else

# ifdef WIN32
# endif

#endif

#define envspd(cyd,slope) (slope!=0?(((Uint64)0xff0000 / ((slope) * (slope) * 256 / (ENVELOPE_SCALE * ENVELOPE_SCALE))) * CYD_BASE_FREQ / cyd->sample_rate):((Uint64)0xff0000 * CYD_BASE_FREQ / cyd->sample_rate))

// used lfsr-generator <http://lfsr-generator.sourceforge.net/> for this: 

inline static void shift_lfsr(Uint32 *v, int tap_0, int tap_1)
{
  typedef unsigned int T;
  const T zero = (T)(0);
  const T lsb = zero + (T)(1);
  const T feedback = (
    (lsb << (tap_0)) ^
    (lsb << (tap_1))
  );
  *v = (*v >> 1) ^ ((zero - (*v & lsb)) & feedback);
}


static void cyd_init_channel(CydEngine *cyd, CydChannel *chn)
{
	memset(chn, 0, sizeof(*chn));
	chn->pw = 0x400;
	cyd_set_filter_coeffs(cyd, chn, 2047, 0);
#ifdef STEREOOUTPUT
	cyd_set_panning(cyd, chn, CYD_PAN_CENTER);
#endif

	for (int s = 0 ; s < CYD_SUB_OSCS ; ++s)
	{
		chn->subosc[s].random = RANDOM_SEED;
		chn->subosc[s].reg4 = chn->subosc[s].reg5 = chn->subosc[s].reg9 = 1;
	}
	
#ifndef CYD_DISABLE_FM
	cydfm_init(&chn->fm);
#endif
}


static void cyd_init_log_tables(CydEngine *cyd)
{
	for (int i = 0 ; i < LUT_SIZE ; ++i)
	{
		cyd->lookup_table[i] = i * (i/2) / ((LUT_SIZE*LUT_SIZE / 65536)/2);
	}
	
#ifndef CYD_DISABLE_BUZZ
	for (int i = 0 ; i < YM_LUT_SIZE ; ++i)
	{
		static const int ymVolumeTable[16] = { 62,161,265,377,580,774,1155,1575,2260,3088,4570,6233,9330,13187,21220,32767}; // from leonard's code
		cyd->lookup_table_ym[i] = ymVolumeTable[i]; //(Uint32)32767 * (Uint32)(i+1) * (Uint32)(i+1) * (Uint32)(i+1) / (Uint32)(YM_LUT_SIZE * YM_LUT_SIZE * YM_LUT_SIZE);
	}
	
	cyd->lookup_table_ym[0] = 0;
#endif
}


void cyd_reset_wavetable(CydEngine *cyd)
{
#ifndef CYD_DISABLE_WAVETABLE
	memset(cyd->wavetable_entries, 0, sizeof(cyd->wavetable_entries[0]) * CYD_WAVE_MAX_ENTRIES);

	for (int i = 0 ; i < CYD_WAVE_MAX_ENTRIES ; ++i)
	{
		cyd_wave_entry_init(&cyd->wavetable_entries[i], NULL, 0, 0, 0, 0, 0);
	}
#endif
}


void cyd_init(CydEngine *cyd, Uint16 sample_rate, int channels)
{
	memset(cyd, 0, sizeof(*cyd));
	cyd->sample_rate = sample_rate;
	cyd->lookup_table = malloc(sizeof(*cyd->lookup_table) * LUT_SIZE);
	cyd->oversample = MAX_OVERSAMPLE;
#ifndef CYD_DISABLE_BUZZ
	cyd->lookup_table_ym = malloc(sizeof(*cyd->lookup_table) * YM_LUT_SIZE);
#endif
	
#ifndef USENATIVEAPIS

# ifdef USESDLMUTEXES
	cyd->mutex = SDL_CreateMutex();
# endif

#else

# ifdef WIN32
	InitializeCriticalSection(&cyd->mutex);
	InitializeCriticalSection(&cyd->thread_lock);
# endif

#endif
	
	cyd_init_log_tables(cyd);
	
	for (int i = 0 ; i < CYD_MAX_FX_CHANNELS ; ++i)
		cydfx_init(&cyd->fx[i], sample_rate);
#ifndef CYD_DISABLE_WAVETABLE
	cyd->wavetable_entries = calloc(sizeof(cyd->wavetable_entries[0]), CYD_WAVE_MAX_ENTRIES);
	
	cyd_reset_wavetable(cyd);
#endif
	
	cyd_reserve_channels(cyd, channels);
}


void cyd_set_oversampling(CydEngine *cyd, int oversampling)
{
	cyd->oversample = oversampling;
}


void cyd_reserve_channels(CydEngine *cyd, int channels)
{
	debug("Reserving %d Cyd channels", channels);
	cyd_lock(cyd, 1);

	cyd->n_channels = channels;
	
	if (cyd->n_channels > CYD_MAX_CHANNELS)
		cyd->n_channels = CYD_MAX_CHANNELS;
	
	if (cyd->channel)
		free(cyd->channel);
	
	cyd->channel = calloc(sizeof(*cyd->channel), CYD_MAX_CHANNELS);
	
	cyd_reset(cyd);
	
	cyd_lock(cyd, 0);
}


void cyd_deinit(CydEngine *cyd)
{
	if (cyd->lookup_table)
	{
		free(cyd->lookup_table);
		cyd->lookup_table = NULL;
	}
	
#ifndef CYD_DISABLE_BUZZ
	if (cyd->lookup_table_ym)	
	{
		free(cyd->lookup_table_ym);
		cyd->lookup_table_ym = NULL;
	}
#endif

	if (cyd->channel)
	{
		free(cyd->channel);
		cyd->channel = NULL;
	}
	
	for (int i = 0 ; i < CYD_MAX_FX_CHANNELS ; ++i)
		cydfx_deinit(&cyd->fx[i]);
	
#ifndef USENATIVEAPIS

# ifdef USESDLMUTEXES
	if (cyd->mutex)
		SDL_DestroyMutex(cyd->mutex);
	cyd->mutex = NULL;
# endif	

#else

# ifdef WIN32
	DeleteCriticalSection(&cyd->mutex);
	DeleteCriticalSection(&cyd->thread_lock);
# endif

#endif

#ifndef CYD_DISABLE_WAVETABLE
	if (cyd->wavetable_entries)
	{
		for (int i = 0 ; i < CYD_WAVE_MAX_ENTRIES ; ++i)
			cyd_wave_entry_deinit(&cyd->wavetable_entries[i]);
			
		free(cyd->wavetable_entries);
		cyd->wavetable_entries = NULL;
	}
#endif
}


void cyd_reset(CydEngine *cyd)
{
	for (int i = 0 ; i < cyd->n_channels ; ++i)
	{
		cyd_init_channel(cyd, &cyd->channel[i]);
		cyd->channel[i].sync_source = i;
	}
}


Uint32 cyd_cycle_adsr(const CydEngine *eng, Uint32 flags, Uint32 ym_env_shape, CydAdsr *adsr)
{
	if (!(flags & CYD_CHN_ENABLE_YM_ENV))
	{
#ifndef CYD_DISABLE_ENVELOPE
		// SID style ADSR envelope

		switch (adsr->envelope_state)
		{
			case SUSTAIN:
			case DONE: return flags; break;
			
			case ATTACK:
			
			adsr->envelope += adsr->env_speed;
			
			if (adsr->envelope >= 0xff0000) 
			{
				adsr->envelope_state = DECAY;
				adsr->envelope=0xff0000;
				adsr->env_speed = envspd(eng, adsr->d);
			}
			
			break;
			
			case DECAY:
			
				if (adsr->envelope > ((Uint32)adsr->s << 19) + adsr->env_speed)
					adsr->envelope -= adsr->env_speed;
				else
				{
					adsr->envelope = (Uint32)adsr->s << 19;
					adsr->envelope_state = (adsr->s == 0) ? RELEASE : SUSTAIN;
					adsr->env_speed = envspd(eng, adsr->r);;
				}
			
			break;
			
			case RELEASE:
			if (adsr->envelope > adsr->env_speed)
			{
				adsr->envelope -= adsr->env_speed;
			}
			else 
			{
				adsr->envelope_state = DONE;
				if ((flags & (CYD_CHN_ENABLE_WAVE|CYD_CHN_WAVE_OVERRIDE_ENV)) != (CYD_CHN_ENABLE_WAVE|CYD_CHN_WAVE_OVERRIDE_ENV)) flags &= ~CYD_CHN_ENABLE_GATE;
				adsr->envelope = 0;
			}
			break;
		}
#endif
	}
	else
	{
#ifndef CYD_DISABLE_BUZZ	
		// YM2149 style envelope HOLD is not processed
	
		switch (adsr->envelope_state)
		{
			case ATTACK:
			
				adsr->envelope += adsr->env_speed;
				
				if (adsr->envelope >= YM_LENGTH) 
				{
					if (ym_env_shape & CYD_YM_ENV_ALT)
					{
						adsr->envelope = YM_LENGTH - (adsr->envelope - YM_LENGTH);
						adsr->envelope_state = DECAY;
					}
					else
					{
						adsr->envelope &= YM_LENGTH - 1;
						adsr->envelope_state = ATTACK;
					}
				}
			
			break;
			
			case DECAY:
			
				if (adsr->envelope >= adsr->env_speed) 
					adsr->envelope -= adsr->env_speed;
				else
				{
					if (ym_env_shape & CYD_YM_ENV_ALT)
					{
						adsr->envelope = (Uint32)adsr->env_speed - adsr->envelope;
						adsr->envelope_state = ATTACK;
					}
					else
					{
						adsr->envelope -= adsr->env_speed;
						adsr->envelope &= YM_LENGTH - 1;
						adsr->envelope_state = DECAY;		
					}
				}
			
			break;
			
			case RELEASE:
				adsr->envelope_state = DONE;
				if ((flags & (CYD_CHN_ENABLE_WAVE|CYD_CHN_WAVE_OVERRIDE_ENV)) != (CYD_CHN_ENABLE_WAVE|CYD_CHN_WAVE_OVERRIDE_ENV)) flags &= ~CYD_CHN_ENABLE_GATE;
				adsr->envelope = 0;
			break;
			
			default: break;
		}
#endif
	}
	
	return flags;
}


#ifndef CYD_DISABLE_LFSR
static void run_lfsrs(CydChannel *chn)
{
	for (int s = 0 ; s < CYD_SUB_OSCS ; ++s)
	{
		shift_lfsr(&chn->subosc[s].reg4, 4, 3);
		shift_lfsr(&chn->subosc[s].reg5, 5, 3);
		
		if (chn->lfsr_type & 8)
			shift_lfsr(&chn->subosc[s].reg9, 9, 5);
		else
			shift_lfsr(&chn->subosc[s].reg9, 17, 14);
	}
}
#endif


static void cyd_cycle_channel(CydEngine *cyd, CydChannel *chn)
{
	chn->flags = cyd_cycle_adsr(cyd, chn->flags, chn->ym_env_shape, &chn->adsr);
	
	if (chn->flags & CYD_CHN_ENABLE_WAVE) 
	{
		for (int i = 0 ; i < CYD_SUB_OSCS ; ++i)
		{
			cyd_wave_cycle(&chn->subosc[i].wave, chn->wave_entry);
		}
	}
	
#ifndef CYD_DISABLE_FM
	if (chn->flags & CYD_CHN_ENABLE_FM) cydfm_cycle(cyd, &chn->fm);
#endif
	// cycle random lfsr
}


static void cyd_sync_channel(CydEngine *cyd, CydChannel *chn)
{
	if ((chn->flags & CYD_CHN_ENABLE_SYNC) && cyd->channel[chn->sync_source].sync_bit)
	{
		for (int i = 0 ; i < CYD_SUB_OSCS ; ++i)
		{
			chn->subosc[i].wave.acc = 0;
			chn->subosc[i].wave.direction = 0;
			chn->subosc[i].accumulator = 0;
			chn->subosc[i].random = RANDOM_SEED;
			chn->subosc[i].reg4 = 1;
			chn->subosc[i].reg5 = 1;
			chn->subosc[i].reg9 = 1;
			chn->subosc[i].lfsr_ctr = 0;
		}
	}
}


static void cyd_advance_oscillators(CydEngine *cyd, CydChannel *chn)
{
	for (int s = 0 ; s < CYD_SUB_OSCS ; ++s)
	{
		Uint32 prev_acc = chn->subosc[s].accumulator;
		chn->subosc[s].accumulator = (chn->subosc[s].accumulator + (Uint32)chn->subosc[s].frequency);
		
		/* only subosc #0 can set the sync bit */
		
		if (s == 0)
			chn->sync_bit |= chn->subosc[s].accumulator & ACC_LENGTH;
			
		chn->subosc[s].accumulator &= ACC_LENGTH - 1;
		
		if ((prev_acc & (ACC_LENGTH/32)) != (chn->subosc[s].accumulator & (ACC_LENGTH/32)))
		{
			if (chn->flags & CYD_CHN_ENABLE_METAL)
			{
				shift_lfsr(&chn->subosc[s].random, 0xe, 8);
				chn->subosc[s].random &= (1 << (0xe + 1)) - 1;
			}
			else
			{
				shift_lfsr(&chn->subosc[s].random, 22, 17);
				chn->subosc[s].random &= (1 << (22 + 1)) - 1;
			}
		}
	
#ifndef CYD_DISABLE_LFSR		
		if (chn->flags & CYD_CHN_ENABLE_LFSR)
		{
			chn->subosc[s].lfsr_acc = (chn->subosc[s].lfsr & 1) ? (WAVE_AMP - 1) : 0;
			
			if (chn->subosc[s].lfsr_ctr >= chn->subosc[s].lfsr_period)
			{
				chn->subosc[s].lfsr_ctr = 0;
			
				switch (chn->lfsr_type & 3)
				{
					case 0: 
						chn->subosc[s].lfsr ^= !!(chn->subosc[s].reg5 & chn->subosc[s].reg9 & 1);
						break;
					
					case 1:
					case 3: 
						chn->subosc[s].lfsr ^= !!(chn->subosc[s].reg5 & 1);
						break;
					
					case 2:
						chn->subosc[s].lfsr ^= !!(chn->subosc[s].reg5 & chn->subosc[s].reg4 & 1);
						break;
						
					case 4: 
						chn->subosc[s].lfsr ^= !!(chn->subosc[s].reg9 & 1);
						break;
						
					case 5: 
					case 7:
						chn->subosc[s].lfsr ^= 1;
						break;
						
					case 6: 
						chn->subosc[s].lfsr ^= !!(chn->subosc[s].reg4 & 1);
						break;
				}
			}
			
			++chn->subosc[s].lfsr_ctr;
			
			run_lfsrs(chn);
		}
#endif
	}
}


static Sint32 cyd_output_channel(CydEngine *cyd, CydChannel *chn)
{
	Sint32 ovr = 0;
	
	chn->sync_bit = 0;

#ifndef CYD_DISABLE_FM	
	const Uint32 mod = (chn->flags & CYD_CHN_ENABLE_FM) ? cydfm_modulate(cyd, &chn->fm, 0) : 0;
#endif
	
	for (int i = 0 ; i < (1 << cyd->oversample) ; ++i)
	{
		for (int s = 0 ; s < CYD_SUB_OSCS ; ++s)
		{
			if (chn->subosc[s].frequency != 0)
			{
#ifdef CYD_DISABLE_FM
				Uint32 accumulator = chn->subosc[s].accumulator;
#else
				Uint32 accumulator = chn->subosc[s].accumulator + mod;
#endif	
				ovr += cyd_osc(chn->flags, accumulator % ACC_LENGTH, chn->pw, chn->subosc[s].random, chn->subosc[s].lfsr_acc) - WAVE_AMP / 2;
			}
		}
		
		cyd_advance_oscillators(cyd, chn); // Need to move the oscillators per every oversample subcycle
		
#ifndef CYD_DISABLE_FM
		cydfm_cycle_oversample(cyd, &chn->fm);
#endif
	}
	
	return (ovr >> cyd->oversample);
}


Sint32 cyd_env_output(const CydEngine *cyd, Uint32 chn_flags, const CydAdsr *adsr, Sint32 input)
{
	if (chn_flags & CYD_CHN_ENABLE_YM_ENV)
	{
#ifndef CYD_DISABLE_BUZZ	
		int idx = adsr->envelope * (Uint32)YM_LUT_SIZE / YM_LENGTH;
		return input * cyd->lookup_table_ym[idx % YM_LUT_SIZE] / 32768 * (Sint32)(adsr->volume) / MAX_VOLUME;
#else
		return input * (Sint32)(adsr->volume) / MAX_VOLUME;
#endif
	}
	else
	{
#ifndef CYD_DISABLE_ENVELOPE	
		if (adsr->envelope_state == ATTACK)
			return ((Sint64)input * ((Sint32)adsr->envelope / 0x10000) / 256) * (Sint32)(adsr->volume) / MAX_VOLUME;
		else
			return ((Sint64)input * (cyd->lookup_table[(adsr->envelope / (65536*256 / LUT_SIZE) ) & (LUT_SIZE - 1)]) / 65536) * (Sint32)(adsr->volume) / MAX_VOLUME;
#else
		return input * (Sint32)(adsr->volume) / MAX_VOLUME;
#endif
	}
}


#ifdef STEREOOUTPUT
static void cyd_output(CydEngine *cyd, Sint32 *left, Sint32 *right)
#else
static Sint32 cyd_output(CydEngine *cyd)
#endif
{
#ifdef STEREOOUTPUT
	*left = *right = 0;
	Sint32 fx_l[CYD_MAX_FX_CHANNELS] = {0}, fx_r[CYD_MAX_FX_CHANNELS] = {0};
#else
	Sint32 v = 0, fx_input[CYD_MAX_FX_CHANNELS] = {0};
#endif
	Sint32 s[CYD_MAX_CHANNELS];
	
	for (int i = 0 ; i < cyd->n_channels ; ++i)
	{
		s[i] = (Sint32)cyd_output_channel(cyd, &cyd->channel[i]);
#ifndef CYD_DISABLE_WAVETABLE
		if ((cyd->channel[i].flags & CYD_CHN_ENABLE_WAVE) && cyd->channel[i].wave_entry && !(cyd->channel[i].flags & CYD_CHN_WAVE_OVERRIDE_ENV))
		{
			for (int sub = 0 ; sub < CYD_SUB_OSCS ; ++sub)
			{
				if (cyd->channel[i].subosc[sub].wave.playing && cyd->channel[i].subosc[sub].wave.frequency != 0)
				{
#ifdef CYD_DISABLE_FM
					CydWaveAcc accumulator = cyd->channel[i].subosc[sub].wave.acc;
#else
					CydWaveAcc accumulator = (cyd->channel[i].flags & CYD_CHN_ENABLE_FM) ? cydfm_modulate_wave(cyd, &cyd->channel[i].fm, cyd->channel[i].wave_entry, cyd->channel[i].subosc[sub].wave.acc) : cyd->channel[i].subosc[sub].wave.acc;
#endif	
					s[i] += cyd_wave_get_sample(&cyd->channel[i].subosc[sub].wave, cyd->channel[i].wave_entry, accumulator);
				}
			}
		}
#endif
	}
	
	for (int i = 0 ; i < cyd->n_channels ; ++i)
	{
		CydChannel *chn = &cyd->channel[i];
		Sint32 o = 0;
		if (chn->flags & CYD_CHN_ENABLE_GATE)
		{
			if (chn->flags & CYD_CHN_ENABLE_RING_MODULATION)
			{
				o = cyd_env_output(cyd, chn->flags, &chn->adsr, s[i] * (s[chn->ring_mod] + (WAVE_AMP / 2)) / WAVE_AMP);
			}
			else
			{
				o = cyd_env_output(cyd, chn->flags, &chn->adsr, s[i]);
			}

#ifndef CYD_DISABLE_WAVETABLE			
			if ((cyd->channel[i].flags & CYD_CHN_ENABLE_WAVE) && cyd->channel[i].wave_entry && (cyd->channel[i].flags & CYD_CHN_WAVE_OVERRIDE_ENV))
			{
				for (int s = 0 ; s < CYD_SUB_OSCS ; ++s)
				{
					if (cyd->channel[i].subosc[s].wave.playing && cyd->channel[i].subosc[s].wave.frequency != 0)
					{
#ifdef CYD_DISABLE_FM
						CydWaveAcc accumulator = cyd->channel[i].subosc[s].wave.acc;
#else
						CydWaveAcc accumulator = (cyd->channel[i].flags & CYD_CHN_ENABLE_FM) ? cydfm_modulate_wave(cyd, &cyd->channel[i].fm, cyd->channel[i].wave_entry, cyd->channel[i].subosc[s].wave.acc) : cyd->channel[i].subosc[s].wave.acc;
#endif	
						o += cyd_wave_get_sample(&cyd->channel[i].subosc[s].wave, chn->wave_entry, accumulator) * (Sint32)(chn->adsr.volume) / MAX_VOLUME;
					}
				}
			}
#endif

#ifndef CYD_DISABLE_FILTER
			if (chn->flags & CYD_CHN_ENABLE_FILTER) 
			{
				cydflt_cycle(&chn->flt, o);
				switch (chn->flttype)
				{
					case FLT_BP: o = cydflt_output_bp(&chn->flt); break;
					default: case FLT_LP: o = cydflt_output_lp(&chn->flt); break;
					case FLT_HP: o = cydflt_output_hp(&chn->flt); break;
				}
			}
#endif
			
#ifdef STEREOOUTPUT
			Sint32 ol = o * chn->gain_left / CYD_STEREO_GAIN, or = o * chn->gain_right / CYD_STEREO_GAIN;
#endif		

			if (chn->flags & CYD_CHN_ENABLE_FX)
			{
#ifdef STEREOOUTPUT
				fx_l[chn->fx_bus] += ol;
				fx_r[chn->fx_bus] += or;
#else
				fx_input[chn->fx_bus] += o;
#endif
			}
			else
			{
#ifdef STEREOOUTPUT
				*left += ol;
				*right += or;
#else
				v += o;
#endif		
			}
		}
	}
	
	for (int i = 0 ; i < CYD_MAX_FX_CHANNELS ; ++i)
	{
#ifdef STEREOOUTPUT
		Sint32 l, r;
		cydfx_output(&cyd->fx[i], fx_l[i], fx_r[i], &l, &r);
		*left += l;
		*right += r;
#else
		v += cydfx_output(&cyd->fx[i], fx_input[i]);
#endif
	}
	
#ifndef STEREOOUTPUT
	return v;
#endif
}


static void cyd_cycle(CydEngine *cyd)
{
	for (int i = 0 ; i < cyd->n_channels ; ++i)
	{
		cyd_cycle_channel(cyd, &cyd->channel[i]);
	}
	
	for (int i = 0 ; i < cyd->n_channels ; ++i)
	{
		cyd_sync_channel(cyd, &cyd->channel[i]);
	}
}


#ifdef NOSDL_MIXER
void cyd_output_buffer(void *udata, Uint8 *_stream, int len)
#else
void cyd_output_buffer(int chan, void *_stream, int len, void *udata)
#endif
{
	CydEngine *cyd = udata;
	Sint16 * stream = (void*)_stream;
	cyd->samples_output = 0;
	
	for (int i = 0 ; i < len ; i += sizeof(Sint16), ++stream, ++cyd->samples_output)
	{
	
#ifndef USENATIVEAPIS

#ifndef USESDLMUTEXES
#ifdef DEBUG
		Uint32 waittime = SDL_GetTicks();
#endif
		while (cyd->lock_request) 
		{
#ifdef DEBUG
			if (SDL_GetTicks() - waittime > 5000)
			{
				warning("Deadlock from cyd_output_buffer");
				waittime = SDL_GetTicks();
			}
#endif
			SDL_Delay(1);
		}
#endif

#endif
	
		if (cyd->flags & CYD_PAUSED) 
		{ 
			i += BUFFER_GRANULARITY * 2 * sizeof(Sint16); 
			stream += BUFFER_GRANULARITY * 2;
			continue; 
		}
		
		cyd_lock(cyd, 1);
		
		for (int g = 0 ; g < BUFFER_GRANULARITY && i < len ; i += sizeof(Sint16)*2, stream += 2, ++cyd->samples_output)
		{
		
			if (cyd->callback && cyd->callback_counter-- == 0)
			{
				cyd->callback_counter = cyd->callback_period-1;
				if (!cyd->callback(cyd->callback_parameter))
				{
					cyd_lock(cyd, 0);
					return;
				}
			}
			
#ifdef STEREOOUTPUT
			Sint32 output, left, right;
			cyd_output(cyd, &left, &right);
			output = (left + right) / 2;
#else
			Sint32 output = cyd_output(cyd);
#endif

#ifdef NOSDL_MIXER
			Sint32 o = (output * PRE_GAIN) / PRE_GAIN_DIVISOR;
#else
			Sint32 o = (Sint32)*(Sint16*)stream + (output * PRE_GAIN) / PRE_GAIN_DIVISOR;
#endif
			
			if (o < -32768) o = -32768;
			else if (o > 32767) o = 32767;
			
			*(Sint16*)stream = o;
			
			cyd_cycle(cyd);
		}

		cyd_lock(cyd, 0);
	}
	
}


#ifdef NOSDL_MIXER
void cyd_output_buffer_stereo(void *udata, Uint8 *_stream, int len)
#else
void cyd_output_buffer_stereo(int chan, void *_stream, int len, void *udata)
#endif
{
	CydEngine *cyd = udata;
	Sint16 *stream = (void*)_stream;
	cyd->samples_output = 0;
	cyd->flags &= ~CYD_CLIPPING;
	
	for (int i = 0 ; i < len ; )
	{
#ifndef USENATIVEAPIS
	
#ifndef USESDLMUTEXES
#ifdef DEBUG
		Uint32 waittime = SDL_GetTicks();
#endif
		while (cyd->lock_request) 
		{
#ifdef DEBUG
			if (SDL_GetTicks() - waittime > 5000)
			{
				warning("Deadlock from cyd_output_buffer");
				waittime = SDL_GetTicks();
			}
#endif
			SDL_Delay(1);
		}
#endif

#endif
	
		if (cyd->flags & CYD_PAUSED) 
		{ 
			i += BUFFER_GRANULARITY * 2 * sizeof(Sint16); 
			stream += BUFFER_GRANULARITY * 2;
			continue; 
		}
		
		cyd_lock(cyd, 1);
		
		for (int g = 0 ; g < BUFFER_GRANULARITY && i < len ; i += sizeof(Sint16)*2, stream += 2, ++cyd->samples_output)
		{
		
			if (cyd->callback && cyd->callback_counter-- == 0)
			{
				cyd->callback_counter = cyd->callback_period-1;
				if (!cyd->callback(cyd->callback_parameter))
				{
					cyd_lock(cyd, 0);
					return;
				}
			}

			Sint32 left, right;
#ifdef STEREOOUTPUT
			cyd_output(cyd, &left, &right);
#else
			left = right = cyd_output(cyd);
#endif

#ifdef NOSDL_MIXER
			Sint32 o1 = (left * PRE_GAIN) / PRE_GAIN_DIVISOR;
#else
			Sint32 o1 = (Sint32)*(Sint16*)stream + (left * PRE_GAIN) / PRE_GAIN_DIVISOR;
#endif
			
			if (o1 < -32768) 
			{
				o1 = -32768;
				cyd->flags |= CYD_CLIPPING;
			}
			else if (o1 > 32767) 
			{
				o1 = 32767;
				cyd->flags |= CYD_CLIPPING;
			}
			
			*(Sint16*)stream = o1;

#ifdef NOSDL_MIXER
			Sint32 o2 = (right * PRE_GAIN) / PRE_GAIN_DIVISOR;
#else
			Sint32 o2 = (Sint32)*((Sint16*)stream + 1) + (right * PRE_GAIN) / PRE_GAIN_DIVISOR;
#endif
			
			if (o2 < -32768) 
			{
				o2 = -32768;
				cyd->flags |= CYD_CLIPPING;
			}
			else if (o2 > 32767) 
			{
				o2 = 32767;
				cyd->flags |= CYD_CLIPPING;
			}
			
			*((Sint16*)stream + 1) = o2;
			
			cyd_cycle(cyd);
			++cyd->samples_played;
		}
		
		cyd_lock(cyd, 0);
	}
}


void cyd_set_frequency(CydEngine *cyd, CydChannel *chn, int subosc, Uint16 frequency)
{
	if (frequency != 0)
	{
		chn->subosc[subosc].frequency = (Uint64)(ACC_LENGTH >> (cyd->oversample))/16 * (Uint64)(frequency) / (Uint64)cyd->sample_rate;

#ifndef CYD_DISABLE_LFSR	
		chn->subosc[subosc].lfsr_period = (Uint64)cyd->sample_rate * 16 / frequency;
#endif
	}
	else
		chn->subosc[subosc].frequency = 0;
	
#ifndef CYD_DISABLE_FM
	if (subosc == 0)
		cydfm_set_frequency(cyd, &chn->fm, frequency);
#endif
}


void cyd_set_wavetable_frequency(CydEngine *cyd, CydChannel *chn, int subosc, Uint16 frequency)
{	
#ifndef CYD_DISABLE_WAVETABLE
	if (frequency != 0 && chn->wave_entry)
	{
		chn->subosc[subosc].wave.frequency = (Uint64)WAVETABLE_RESOLUTION * (Uint64)chn->wave_entry->sample_rate / (Uint64)cyd->sample_rate * (Uint64)frequency / (Uint64)get_freq(chn->wave_entry->base_note);
	}
	else
	{
		chn->subosc[subosc].wave.playing = false;
		chn->subosc[subosc].wave.frequency = 0;
	}
#endif
}


void cyd_set_env_frequency(CydEngine *cyd, CydChannel *chn, Uint16 frequency)
{
#ifndef CYD_DISABLE_BUZZ
	chn->adsr.env_speed = (Uint64)YM_LENGTH/16 * (Uint64)frequency / (Uint64)cyd->sample_rate;
#endif
}


void cyd_set_env_shape(CydChannel *chn, Uint8 shape)
{
#ifndef CYD_DISABLE_BUZZ
	chn->ym_env_shape = shape;
	
	if ((chn->flags & CYD_CHN_ENABLE_KEY_SYNC) || (chn->adsr.envelope_state == DONE || chn->adsr.envelope_state == SUSTAIN))
	{
		if (shape & CYD_YM_ENV_ATT)
		{
			chn->adsr.envelope = 0;
			chn->adsr.envelope_state = ATTACK;
		}
		else
		{
			chn->adsr.envelope = YM_LENGTH;
			chn->adsr.envelope_state = DECAY;
		}
	}
#endif
}


void cyd_enable_gate(CydEngine *cyd, CydChannel *chn, Uint8 enable)
{
	if (enable)
	{
		if (!(chn->flags & CYD_CHN_ENABLE_YM_ENV))
		{
#ifndef CYD_DISABLE_ENVELOPE
			chn->adsr.envelope_state = ATTACK;
			chn->adsr.envelope = 0x0;
			chn->adsr.env_speed = envspd(cyd, chn->adsr.a);
			chn->flags = cyd_cycle_adsr(cyd, chn->flags, chn->ym_env_shape, &chn->adsr);
#ifndef CYD_DISABLE_FM	
			chn->fm.adsr.envelope_state = ATTACK;
			chn->fm.adsr.envelope = chn->fm.attack_start << 19;
			chn->fm.adsr.env_speed = envspd(cyd, chn->fm.adsr.a);
			cyd_cycle_adsr(cyd, 0, 0, &chn->fm.adsr);
#endif
#endif
		}
		
		if (chn->flags & CYD_CHN_ENABLE_KEY_SYNC)
		{
			for (int s = 0 ; s < CYD_SUB_OSCS ; ++s)
			{
				chn->subosc[s].accumulator = 0;
				chn->subosc[s].reg4 = chn->subosc[s].reg5 = chn->subosc[s].reg9 = 1;
				chn->subosc[s].lfsr_ctr = 0;
			}
			
#ifndef CYD_DISABLE_FM
			chn->fm.accumulator = 0;
			chn->fm.wave.acc = 0;
#endif
		}
		
		chn->flags |= CYD_CHN_ENABLE_GATE;
	}
	else
	{
		chn->flags &= ~CYD_CHN_WAVE_OVERRIDE_ENV;
		chn->adsr.envelope_state = RELEASE;
		chn->adsr.env_speed = envspd(cyd, chn->adsr.r);
		
#ifndef CYD_DISABLE_FM
		chn->fm.adsr.envelope_state = RELEASE;
		chn->fm.adsr.env_speed = envspd(cyd, chn->fm.adsr.r);
#endif		
	}
}


void cyd_set_waveform(CydChannel *chn, Uint32 wave)
{
	chn->flags = (chn->flags & (~WAVEFORMS)) | (wave & WAVEFORMS);
}


void cyd_set_callback(CydEngine *cyd, int (*callback)(void*), void*param, Uint16 period)
{
	cyd_lock(cyd, 1);

	cyd->samples_played	= 0;
	cyd->callback_parameter = param;
	cyd->callback = callback;
	cyd->callback_period = cyd->sample_rate / period;
	cyd->callback_counter = cyd->callback_counter % cyd->callback_period;
	
	cyd_lock(cyd, 0);
}


void cyd_set_callback_rate(CydEngine *cyd, Uint16 period)
{
	cyd_lock(cyd, 1);
	
	cyd->callback_period = cyd->sample_rate / period;
	cyd->callback_counter = cyd->callback_counter % cyd->callback_period;
	
	cyd_lock(cyd, 0);
}


#ifdef USENATIVEAPIS
# ifdef WIN32

static void fill_buffer(CydEngine *cyd)
{
	//waveOutUnprepareHeader(cyd->hWaveOut, &cyd->waveout_hdr[cyd->waveout_hdr_idx],sizeof(WAVEHDR));

#ifdef NOSDL_MIXER
	cyd_output_buffer_stereo(cyd, cyd->waveout_hdr[cyd->waveout_hdr_idx].lpData, cyd->waveout_hdr[cyd->waveout_hdr_idx].dwBufferLength);
#else
	cyd_output_buffer_stereo(0, cyd->waveout_hdr[cyd->waveout_hdr_idx].lpData, cyd->waveout_hdr[cyd->waveout_hdr_idx].dwBufferLength, cyd);
#endif
	
	//waveOutPrepareHeader(cyd->hWaveOut, &cyd->waveout_hdr[cyd->waveout_hdr_idx],sizeof(WAVEHDR));
	
	cyd->waveout_hdr[cyd->waveout_hdr_idx].dwFlags = WHDR_PREPARED;
	
	if (waveOutWrite(cyd->hWaveOut, &cyd->waveout_hdr[cyd->waveout_hdr_idx], sizeof(cyd->waveout_hdr[cyd->waveout_hdr_idx])) != MMSYSERR_NOERROR)
		warning("waveOutWrite returned error");
	
	if (++cyd->waveout_hdr_idx >= CYD_NUM_WO_BUFFERS)
		cyd->waveout_hdr_idx = 0;
}


static DWORD WINAPI ThreadProc(void *param)
{
	CydEngine *cyd = param;
	
	for(;;)
	{
		EnterCriticalSection(&cyd->thread_lock);
		
		if (!cyd->thread_running)
		{
			LeaveCriticalSection(&cyd->thread_lock);
			break;
		}
		
		while (cyd->buffers_available > 0)
		{
			LeaveCriticalSection(&cyd->thread_lock);
			fill_buffer(cyd);
			EnterCriticalSection(&cyd->thread_lock);
			--cyd->buffers_available;
			
		}
		
		LeaveCriticalSection(&cyd->thread_lock);
		
		Sleep(1);
	}
	
	debug("Thread exit");
	
	return 0;
}


static DWORD WINAPI waveOutProc(void *param)
{
	CydEngine *cyd = (void*)param;
	MSG msg;
	
	while (GetMessage(&msg, 0, 0, 0) == 1)
	{
		if (msg.message == MM_WOM_DONE)
		{
			EnterCriticalSection(&cyd->thread_lock);
			
			++cyd->buffers_available;
			
			LeaveCriticalSection(&cyd->thread_lock);
		}
		
		if (msg.message == MM_WOM_CLOSE)
		{
			break;
		}
	}
	
	return 0;
}

# endif
#endif


#ifdef NOSDL_MIXER
int cyd_register(CydEngine * cyd, int buffer_length)
#else
int cyd_register(CydEngine * cyd)
#endif
{
#ifndef USENATIVEAPIS
# ifndef NOSDL_MIXER
	int frequency, channels;
	Uint16 format;
	if (Mix_QuerySpec(&frequency, &format, &channels))
	{
		switch (format)
		{
			case AUDIO_S16SYS:
			break;
			
			default:
			return 0;
			break;
		}
	
		switch (channels)
		{
			case 1: if (!Mix_RegisterEffect(MIX_CHANNEL_POST, cyd_output_buffer, NULL, cyd)) return 0; break;
			case 2: if (!Mix_RegisterEffect(MIX_CHANNEL_POST, cyd_output_buffer_stereo, NULL, cyd)) return 0; break;
			default: return 0; break;
		}
		
		return 1;
	}
	else return 0;
# else

	SDL_AudioSpec desired, obtained;

	/* 22050Hz - FM Radio quality */
	desired.freq=cyd->sample_rate;

	/* 16-bit signed audio */
	desired.format=AUDIO_S16SYS;

	/* Stereo */
	desired.channels=2;

	/* Large audio buffer reduces risk of dropouts but increases response time */
	desired.samples=buffer_length;

	/* Our callback function */
	desired.callback=cyd_output_buffer_stereo;
	desired.userdata=cyd;

	printf("Opening SDL audio\n");
	
	/* Open the audio device */
	if (SDL_OpenAudio(&desired, &obtained) < 0)
	{
		warning("Could not open audio device");
		return 0;
	}
	
	printf("Got %d Hz/format %d/%d channels\n", obtained.freq, obtained.format, obtained.channels);
	
	SDL_PauseAudio(0);
	
	return 1;
# endif
#else

# ifdef WIN32
	WAVEFORMATEX waveformat;
	waveformat.cbSize = 0;
	waveformat.wFormatTag = WAVE_FORMAT_PCM;
    waveformat.wBitsPerSample = 16;
	waveformat.nChannels = 2;
    waveformat.nSamplesPerSec = cyd->sample_rate;
	waveformat.nBlockAlign = waveformat.nChannels * waveformat.wBitsPerSample / 8;
    waveformat.nAvgBytesPerSec = waveformat.nSamplesPerSec * waveformat.nBlockAlign;
	
	CreateThread(NULL, 0, waveOutProc, cyd, 0, &cyd->cb_handle);
	
	MMRESULT result = waveOutOpen(&cyd->hWaveOut, 0, &waveformat, cyd->cb_handle, (DWORD)cyd, CALLBACK_THREAD);
	
	if (result != MMSYSERR_NOERROR)
	{
		warning("waveOutOpen failed (%x)", result);
		return 0;
	}
	
	for (int i = 0 ; i < CYD_NUM_WO_BUFFERS ; ++i)
	{
		WAVEHDR * h = &cyd->waveout_hdr[i];
		
		ZeroMemory(h, sizeof(*h));
		
		h->dwBufferLength = CYD_NUM_WO_BUFFER_SIZE * 2 * sizeof(Sint16);
		h->lpData = calloc(h->dwBufferLength, 1);
		
		waveOutPrepareHeader(cyd->hWaveOut, &cyd->waveout_hdr[i],sizeof(WAVEHDR));
	}
	
	cyd->buffers_available = CYD_NUM_WO_BUFFERS;
	cyd->thread_running = 1;
	
	CreateThread(NULL, 0, ThreadProc, cyd, 0, &cyd->thread_handle);
	SetThreadPriority((HANDLE)cyd->thread_handle, THREAD_PRIORITY_HIGHEST);
	
	return 1;
# else

# error Platform not supported for native apis

# endif

	return 0;
#endif
}


int cyd_unregister(CydEngine * cyd)
{
	debug("cyd_unregister");	
#ifndef USENATIVEAPIS
# ifndef NOSDL_MIXER
	int frequency, channels;
	Uint16 format;
	if (Mix_QuerySpec(&frequency, &format, &channels))
	{
		switch (channels)
		{
			case 1: if (!Mix_UnregisterEffect(MIX_CHANNEL_POST, cyd_output_buffer)) return 0; break;
			case 2: if (!Mix_UnregisterEffect(MIX_CHANNEL_POST, cyd_output_buffer_stereo)) return 0; break;
			default: return 0; break;
		}
		
		cyd_lock(cyd, 1);
		cyd_lock(cyd, 0);
		
		return 1;
	}
	else return 0;
# else

	debug("Waiting for stuff");	
	cyd_lock(cyd, 1);
	debug("Done waiting");	
	cyd_lock(cyd, 0);
	
	debug("Closing audio");	
	SDL_CloseAudio();
	debug("SDL_CloseAudio finished");	
	
	return 1;
# endif
#else

	cyd_pause(cyd, 0);

	debug("Waiting for thread");
	cyd_lock(cyd, 1);
	cyd->thread_running = 0;
	cyd_lock(cyd, 0);
	WaitForSingleObject((HANDLE)cyd->thread_handle, 2000);
	
	waveOutReset(cyd->hWaveOut);

	for (int i = 0 ; i < CYD_NUM_WO_BUFFERS ; ++i)
	{
		if (cyd->waveout_hdr[i].dwFlags & WHDR_PREPARED)
			waveOutUnprepareHeader(cyd->hWaveOut, &cyd->waveout_hdr[i], sizeof(cyd->waveout_hdr[i]));
		free(cyd->waveout_hdr[i].lpData);
	}

	waveOutClose(cyd->hWaveOut);
	
	WaitForSingleObject((HANDLE)cyd->cb_handle, 2000);
		
	return 1;
#endif
}


void cyd_set_filter_coeffs(CydEngine * cyd, CydChannel *chn, Uint16 cutoff, Uint8 resonance)
{
#ifndef CYD_DISABLE_FILTER
	static const Uint16 resonance_table[] = {10, 512, 1300, 1950};
	cydflt_set_coeff(&chn->flt, cutoff, resonance_table[resonance & 3]);
#endif
}


void cyd_lock(CydEngine *cyd, Uint8 enable)
{
	if (cyd->flags & CYD_SINGLE_THREAD) return; // For export, mainly

#ifndef USENATIVEAPIS

#ifndef USESDLMUTEXES
	if (enable)
	{
#ifdef DEBUG
		Uint32 waittime = SDL_GetTicks();
#endif
		cyd->lock_request = 1;
		while (cyd->lock_locked )
		{
#ifdef DEBUG
			if (SDL_GetTicks() - waittime > 5000)
			{
				warning("Deadlock from cyd_lock");
				waittime = SDL_GetTicks();
			}
#endif
			SDL_Delay(1);
		}
	}
	else
	{
		cyd->lock_request = 0;
		while (cyd->lock_locked)
		{
			SDL_Delay(1);
		}
	}	
#else	
	if (enable)
	{
		SDL_LockMutex(cyd->mutex);
	}
	else
	{
		SDL_UnlockMutex(cyd->mutex);
	}
#endif
#else

# ifdef WIN32
	if (enable)
	{
		EnterCriticalSection(&cyd->mutex);
	}
	else
	{
		LeaveCriticalSection(&cyd->mutex);
	}
# endif

#endif
}


#ifdef STEREOOUTPUT
void cyd_set_panning(CydEngine *cyd, CydChannel *chn, Uint8 panning)
{
	if (chn->panning == panning) return;
	
	chn->panning = my_min(CYD_PAN_RIGHT, my_max(CYD_PAN_LEFT, panning));
	float a = M_PI / 2 * (float)(chn->panning - CYD_PAN_LEFT) / (CYD_PAN_RIGHT - CYD_PAN_LEFT);
	chn->gain_left = cos(a) * CYD_STEREO_GAIN;
	chn->gain_right = sin(a) * CYD_STEREO_GAIN;
}
#endif


void cyd_set_wave_entry(CydChannel *chn, const CydWavetableEntry * entry)
{
	chn->wave_entry = entry;
	
	for (int s = 0 ; s < CYD_SUB_OSCS ; ++s)
	{
		chn->subosc[s].wave.playing = true;
		chn->subosc[s].wave.acc = 0;
		chn->subosc[s].wave.frequency = 0;
		chn->subosc[s].wave.direction = 0;
	}
}


void cyd_set_wavetable_offset(CydChannel *chn, Uint16 offset /* 0..0x1000 = 0-100% */)
{
#ifndef CYD_DISABLE_WAVETABLE
	if (chn->wave_entry)
	{
		for (int s = 0 ; s < CYD_SUB_OSCS ; ++s)
		{
			chn->subosc[s].wave.acc = (Uint64)offset * WAVETABLE_RESOLUTION * chn->wave_entry->samples / 0x1000;
		}
	}
#endif
}


void cyd_pause(CydEngine *cyd, Uint8 enable)
{
#ifdef USENATIVEAPIS
#ifdef WIN32


	if (enable)
		waveOutPause(cyd->hWaveOut);
	else
		waveOutRestart(cyd->hWaveOut);
#endif
#else
	cyd_lock(cyd, 1);
	
	if (enable)
		cyd->flags |= CYD_PAUSED;
	else
		cyd->flags &= ~CYD_PAUSED;
	
	cyd_lock(cyd, 0);
#endif
}
