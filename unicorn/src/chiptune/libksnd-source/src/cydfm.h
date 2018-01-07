#pragma once

#include "cydadsr.h"
#include "cydentry.h"
#include "cydtypes.h"
#include "cydwave.h"

typedef struct
{
	Uint32 flags;
	Uint8 feedback; // 0-7 
	Uint8 harmonic; // 0-15
	CydAdsr adsr;
	Uint32 period;
	Uint32 wave_period;
	Uint32 accumulator;
	const CydWavetableEntry *wave_entry;
	CydWaveState wave;
	Uint32 fb1, fb2, env_output;
	Uint32 current_modulation;
	Uint8 attack_start;
} CydFm;

#include "cyd.h"

struct CydEngine_t;

void cydfm_init(CydFm *fm);
void cydfm_cycle(const struct CydEngine_t *cyd, CydFm *fm);
void cydfm_cycle_oversample(const struct CydEngine_t *cyd, CydFm *fm);
void cydfm_set_frequency(const struct CydEngine_t *cyd, CydFm *fm, Uint32 base_frequency);
Uint32 cydfm_modulate(const struct CydEngine_t *cyd, const CydFm *fm, Uint32 accumulator);
CydWaveAcc cydfm_modulate_wave(const struct CydEngine_t *cyd, const CydFm *fm, const CydWavetableEntry *wave, CydWaveAcc accumulator);
void cydfm_set_wave_entry(CydFm *fm, const CydWavetableEntry * entry);
