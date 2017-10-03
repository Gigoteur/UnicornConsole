#include "cydfm.h"
#include "cyddefs.h"
#include "freqs.h"
#include "cydosc.h"
#include "macros.h"
#include <stdlib.h>
#include <string.h>


#define MODULATOR_MAX 1024


void cydfm_init(CydFm *fm)
{
	memset(fm, 0, sizeof(*fm));
}


static Uint32 get_modulator(const CydEngine *cyd, const CydFm *fm)
{
	const static Uint32 fbtab[] = { 0, 64, 32, 16, 8, 4, 2, 1 };

	if ((fm->flags & CYD_FM_ENABLE_WAVE) && fm->wave_entry)
	{
		Uint32 acc = fm->wave.acc;
		CydWaveAcc length = (CydWaveAcc)(fm->wave_entry->loop_end - fm->wave_entry->loop_begin) * WAVETABLE_RESOLUTION;
		
		if (length == 0) return 0;
		
		if (fm->feedback) 
		{
			acc = acc + ((Uint64)(fm->fb1 + fm->fb2) / 2 * (length * 4 / fbtab[fm->feedback]) / MODULATOR_MAX);
		}
		
		return (Sint64)(cyd_wave_get_sample(&fm->wave, fm->wave_entry, acc % length)) * fm->env_output / 32768 + 65536;
	}
	else
	{
		Uint32 acc = fm->accumulator;
		if (fm->feedback) acc += ((Uint64)(fm->fb1 + fm->fb2) / 2 * (ACC_LENGTH * 4 / fbtab[fm->feedback]) / MODULATOR_MAX);
		return (Uint64)cyd_osc(CYD_CHN_ENABLE_TRIANGLE, acc % ACC_LENGTH, 0, 0, 0) * fm->env_output / WAVE_AMP + WAVE_AMP / 2;
	}
}


void cydfm_cycle_oversample(const CydEngine *cyd, CydFm *fm)
{
	
}


void cydfm_cycle(const CydEngine *cyd, CydFm *fm)
{
	cyd_cycle_adsr(cyd, 0, 0, &fm->adsr);
	
	fm->env_output = cyd_env_output(cyd, 0, &fm->adsr, MODULATOR_MAX);
	
	cyd_wave_cycle(&fm->wave, fm->wave_entry);
	
	fm->accumulator = (fm->accumulator + fm->period) % ACC_LENGTH;
	
	Uint32 mod = get_modulator(cyd, fm);
	
	fm->fb2 = fm->fb1;
	fm->fb1 = mod;
	fm->current_modulation = mod;
}


void cydfm_set_frequency(const CydEngine *cyd, CydFm *fm, Uint32 base_frequency)
{
	const int MUL = 2;
	static Sint32 harmonic[16] = { 0.5 * MUL, 1.0 * MUL, 2.0 * MUL, 3 * MUL, 4 * MUL, 5 * MUL, 6 * MUL, 7 * MUL, 8 * MUL, 9 * MUL, 10 * MUL, 10 * MUL, 12 * MUL, 12 * MUL, 15 * MUL, 15 * MUL };

	fm->period = ((Uint64)(ACC_LENGTH)/16 * (Uint64)base_frequency / (Uint64)cyd->sample_rate) * (Uint64)harmonic[fm->harmonic & 15] / (Uint64)harmonic[fm->harmonic >> 4];
	
	if (fm->wave_entry)
	{
		fm->wave.playing = true;
		fm->wave.frequency = ((Uint64)(WAVETABLE_RESOLUTION) * (Uint64)fm->wave_entry->sample_rate / (Uint64)cyd->sample_rate * (Uint64)base_frequency / (Uint64)get_freq(fm->wave_entry->base_note)) * (Uint64)harmonic[fm->harmonic & 15] / (Uint64)harmonic[fm->harmonic >> 4];
	}
}


Uint32 cydfm_modulate(const CydEngine *cyd, const CydFm *fm, Uint32 accumulator)
{
	Uint32 mod = (Uint64)fm->current_modulation * ACC_LENGTH * 8 / MODULATOR_MAX;
	
	return (mod + accumulator) % ACC_LENGTH;
}


CydWaveAcc cydfm_modulate_wave(const CydEngine *cyd, const CydFm *fm, const CydWavetableEntry *wave, CydWaveAcc accumulator)
{
	if (wave->loop_begin == wave->loop_end)
		return accumulator;
		
	CydWaveAcc length = (CydWaveAcc)(wave->loop_end - wave->loop_begin) * WAVETABLE_RESOLUTION;
	CydWaveAcc mod = (CydWaveAcc)fm->current_modulation * length * 8 / MODULATOR_MAX;
		
	return (mod + accumulator) % length;
}


void cydfm_set_wave_entry(CydFm *fm, const CydWavetableEntry * entry)
{
	fm->wave_entry = entry;
	fm->wave.frequency = 0;
	fm->wave.direction = 0;
}
