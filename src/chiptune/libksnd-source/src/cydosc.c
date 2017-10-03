#include "cydosc.h"
#include "cyddefs.h"
#include "cyd.h"

static inline Uint32 cyd_pulse(Uint32 acc, Uint32 pw) 
{
	return (((acc >> ((ACC_BITS - 17))) >= (pw << 4) ? (WAVE_AMP - 1) : 0));
}


static inline Uint32 cyd_saw(Uint32 acc) 
{
	return (acc >> (ACC_BITS - OUTPUT_BITS - 1)) & (WAVE_AMP - 1);
}


static inline Uint32 cyd_triangle(Uint32 acc)
{
	return ((((acc & (ACC_LENGTH / 2)) ? ~acc : acc) >> (ACC_BITS - OUTPUT_BITS - 2)) & (WAVE_AMP * 2 - 1));
}


static inline Uint32 cyd_noise(Uint32 acc) 
{
	return acc & (WAVE_AMP - 1);
}

#ifndef CYD_DISABLE_LFSR
Uint32 cyd_lfsr(Uint32 bits) 
{
	return bits;
}
#endif

Sint32 cyd_osc(Uint32 flags, Uint32 accumulator, Uint32 pw, Uint32 random, Uint32 lfsr_acc)
{
	switch (flags & WAVEFORMS & ~CYD_CHN_ENABLE_WAVE)
	{
		case CYD_CHN_ENABLE_PULSE:
		return cyd_pulse(accumulator, pw);
		break;
		
		case CYD_CHN_ENABLE_SAW:
		return cyd_saw(accumulator);
		break;
		
		case CYD_CHN_ENABLE_TRIANGLE:
		return cyd_triangle(accumulator);
		break;
		
		case CYD_CHN_ENABLE_NOISE:
		return cyd_noise(random);
		break;
		
		case CYD_CHN_ENABLE_TRIANGLE|CYD_CHN_ENABLE_PULSE:
		return cyd_pulse(accumulator, pw) & cyd_triangle(accumulator);
		break;
		
		case CYD_CHN_ENABLE_SAW|CYD_CHN_ENABLE_PULSE:
		return cyd_saw(accumulator) & cyd_pulse(accumulator, pw);
		break;
		
		case CYD_CHN_ENABLE_NOISE|CYD_CHN_ENABLE_PULSE:
		return cyd_noise(random) & cyd_pulse(accumulator, pw);
		break;
		
		case CYD_CHN_ENABLE_TRIANGLE|CYD_CHN_ENABLE_SAW:
		return cyd_triangle(accumulator) & cyd_saw(accumulator);
		break;
		
		case CYD_CHN_ENABLE_NOISE|CYD_CHN_ENABLE_SAW:
		return cyd_noise(random) & cyd_saw(accumulator);
		break;
		
		case CYD_CHN_ENABLE_NOISE|CYD_CHN_ENABLE_TRIANGLE:
		return cyd_noise(random) & cyd_triangle(accumulator);
		break;
		
		case CYD_CHN_ENABLE_TRIANGLE|CYD_CHN_ENABLE_PULSE|CYD_CHN_ENABLE_SAW:
		return cyd_pulse(accumulator, pw) & cyd_triangle(accumulator) & cyd_saw(accumulator);
		break;
		
		case CYD_CHN_ENABLE_TRIANGLE|CYD_CHN_ENABLE_PULSE|CYD_CHN_ENABLE_NOISE:
		return cyd_pulse(accumulator, pw) & cyd_triangle(accumulator) & cyd_noise(random);
		break;
		
		case CYD_CHN_ENABLE_TRIANGLE|CYD_CHN_ENABLE_NOISE|CYD_CHN_ENABLE_SAW:
		return cyd_saw(accumulator) & cyd_triangle(accumulator) & cyd_noise(random);
		break;
		
		case CYD_CHN_ENABLE_PULSE|CYD_CHN_ENABLE_NOISE|CYD_CHN_ENABLE_SAW:
		return cyd_pulse(accumulator, pw) & cyd_saw(accumulator) & cyd_noise(random);
		break;
		
		case CYD_CHN_ENABLE_NOISE|CYD_CHN_ENABLE_SAW|CYD_CHN_ENABLE_PULSE|CYD_CHN_ENABLE_TRIANGLE:
		return cyd_saw(accumulator) & cyd_pulse(accumulator, pw) & cyd_triangle(accumulator) & cyd_noise(random);
		break;
		
#ifndef CYD_DISABLE_LFSR			
		case CYD_CHN_ENABLE_LFSR:
		return cyd_lfsr(lfsr_acc);
		break;
		
		case CYD_CHN_ENABLE_PULSE|CYD_CHN_ENABLE_LFSR:
		return cyd_pulse(accumulator, pw) & cyd_lfsr(lfsr_acc);
		break;
		
		case CYD_CHN_ENABLE_SAW|CYD_CHN_ENABLE_LFSR:
		return cyd_saw(accumulator) & cyd_lfsr(lfsr_acc);
		break;
		
		case CYD_CHN_ENABLE_TRIANGLE|CYD_CHN_ENABLE_LFSR:
		return cyd_triangle(accumulator) & cyd_lfsr(lfsr_acc);
		break;
		
		case CYD_CHN_ENABLE_NOISE|CYD_CHN_ENABLE_LFSR:
		return cyd_noise(random) & cyd_lfsr(lfsr_acc);
		break;
		
		case CYD_CHN_ENABLE_TRIANGLE|CYD_CHN_ENABLE_PULSE|CYD_CHN_ENABLE_LFSR:
		return cyd_pulse(accumulator, pw) & cyd_triangle(accumulator) & cyd_lfsr(lfsr_acc);
		break;
		
		case CYD_CHN_ENABLE_SAW|CYD_CHN_ENABLE_PULSE|CYD_CHN_ENABLE_LFSR:
		return cyd_saw(accumulator) & cyd_pulse(accumulator, pw) & cyd_lfsr(lfsr_acc);
		break;
		
		case CYD_CHN_ENABLE_NOISE|CYD_CHN_ENABLE_PULSE|CYD_CHN_ENABLE_LFSR:
		return cyd_noise(random) & cyd_pulse(accumulator, pw) & cyd_lfsr(lfsr_acc);
		break;
		
		case CYD_CHN_ENABLE_TRIANGLE|CYD_CHN_ENABLE_SAW|CYD_CHN_ENABLE_LFSR:
		return cyd_triangle(accumulator) & cyd_saw(accumulator) & cyd_lfsr(lfsr_acc);
		break;
		
		case CYD_CHN_ENABLE_NOISE|CYD_CHN_ENABLE_SAW|CYD_CHN_ENABLE_LFSR:
		return cyd_noise(random) & cyd_saw(accumulator) & cyd_lfsr(lfsr_acc);
		break;
		
		case CYD_CHN_ENABLE_NOISE|CYD_CHN_ENABLE_TRIANGLE|CYD_CHN_ENABLE_LFSR:
		return cyd_noise(random) & cyd_triangle(accumulator) & cyd_lfsr(lfsr_acc);
		break;
		
		case CYD_CHN_ENABLE_TRIANGLE|CYD_CHN_ENABLE_PULSE|CYD_CHN_ENABLE_SAW|CYD_CHN_ENABLE_LFSR:
		return cyd_pulse(accumulator, pw) & cyd_triangle(accumulator) & cyd_saw(accumulator) & cyd_lfsr(lfsr_acc);
		break;
		
		case CYD_CHN_ENABLE_TRIANGLE|CYD_CHN_ENABLE_PULSE|CYD_CHN_ENABLE_NOISE|CYD_CHN_ENABLE_LFSR:
		return cyd_pulse(accumulator, pw) & cyd_triangle(accumulator) & cyd_noise(random) & cyd_lfsr(lfsr_acc);
		break;
		
		case CYD_CHN_ENABLE_TRIANGLE|CYD_CHN_ENABLE_NOISE|CYD_CHN_ENABLE_SAW|CYD_CHN_ENABLE_LFSR:
		return cyd_saw(accumulator) & cyd_triangle(accumulator) & cyd_noise(random) & cyd_lfsr(lfsr_acc);
		break;
		
		case CYD_CHN_ENABLE_PULSE|CYD_CHN_ENABLE_NOISE|CYD_CHN_ENABLE_SAW|CYD_CHN_ENABLE_LFSR:
		return cyd_pulse(accumulator, pw) & cyd_saw(accumulator) & cyd_noise(random) & cyd_lfsr(lfsr_acc);
		break;
		
		case CYD_CHN_ENABLE_NOISE|CYD_CHN_ENABLE_SAW|CYD_CHN_ENABLE_PULSE|CYD_CHN_ENABLE_TRIANGLE|CYD_CHN_ENABLE_LFSR:
		return cyd_saw(accumulator) & cyd_pulse(accumulator, pw) & cyd_triangle(accumulator) & cyd_noise(random) & cyd_lfsr(lfsr_acc);
		break;
#endif
		default:
		return WAVE_AMP / 2;
		break;
	}
}
