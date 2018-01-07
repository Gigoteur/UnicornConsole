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

// note: for only loading files all the compression code below is useless
// note: compression code added for convenience (and klystrack)

#include "pack.h"
#include <stdlib.h>
#include <string.h>


/* bitpack() bitplane codes */
enum
{
	BITPACK_STATIC0,
	BITPACK_STATIC1,
	BITPACK_LITERAL,
	BITPACK_RLE
};


/* bitstream writer helpers*/
typedef struct
{
	Uint8 *buffer;
	Uint32 byte, size_byte; 	// Read/write position counters (byte component)
	Uint8 bit, size_bit;		// Read/write position counters (bit component)
} BitPtr;


#define BIT_BLOCK_SIZE 1024

static void bit_init(BitPtr *p, Uint8 *buffer, Uint32 size_bytes, Uint8 size_bits)
{
	memset(p, 0, sizeof(*p));
	p->buffer = buffer;
	p->size_byte = size_bytes;
	p->size_bit = size_bits;
}


static void bit_seek(BitPtr *p, Uint32 byte, Uint8 bit)
{
	p->byte = byte;
	p->bit = bit;
}


static void bit_w(BitPtr *p, int bit)
{
	if (p->bit == 0 && !(p->byte & (BIT_BLOCK_SIZE - 1)))
	{
		p->buffer = realloc(p->buffer, p->byte + BIT_BLOCK_SIZE);
	}

	if (bit)
		p->buffer[p->byte] |= 1 << p->bit;
	else
		p->buffer[p->byte] &= ~(1 << p->bit);

	++p->bit;
	
	if (p->bit == 8)
	{
		p->bit = 0;
		++p->byte;
	}
	
	p->size_byte = p->byte;
	p->size_bit = p->bit;
}


static int bit_r(BitPtr *p)
{
	if (p->size_byte <= p->byte && p->size_bit <= p->bit)
		return -1;

	int bit = (p->buffer[p->byte] & (1 << p->bit)) != 0;
	
	++p->bit;
	
	if (p->bit == 8)
	{
		p->bit = 0;
		++p->byte;
	}
	
	return bit;
}


/* By Sean Eron Anderson */
static int log2u(Uint32 v)
{
	const unsigned int b[] = {0x2, 0xC, 0xF0, 0xFF00, 0xFFFF0000};
	const unsigned int S[] = {1, 2, 4, 8, 16};
	int i;

	register unsigned int r = 0; // result of log2(v) will go here
	for (i = 4; i >= 0; i--) // unroll for speed...
	{
	  if (v & b[i])
	  {
		v >>= S[i];
		r |= S[i];
	  } 
	}
	
	return r;
}


/* Write Elias gamma coded value (has to be nonzero) */
static void bit_wgamma(BitPtr *p, Uint32 value)
{
	int l = log2u(value);
	
	for (int a=0; a < l; a++)
		bit_w(p, 0); //put 0s to indicate how many bits will follow
		
	bit_w(p, 1);      //mark the end of the 0s
	
	for (int a=l-1; a >= 0; a--) //Write the bits as plain binary
	{
		bit_w(p, value & 1 << a);
	}
	
}


/* Read Elias gamma coded value, zero return value signals read error */
static Uint32 bit_rgamma(BitPtr *p)
{
	int numberBits = 0;
	int bit;
	while (!(bit = bit_r(p)))
		numberBits++; //keep on reading until we fetch a one...
	
	if (bit < 0) return 0;
		
	Uint32 current = 0;
	for (int a=numberBits-1; a >= 0; a--) //Read numberBits bits
	{
		if ((bit = bit_r(p)))
			current |= 1 << a;
			
		if (bit < 0) return 0;
	}
	current |= 1 << numberBits; //last bit isn't encoded!
	
	return current;
}


/* Read bits bits */
static Sint32 bit_rbits(BitPtr *p, Uint8 bits)
{
	Sint32 rval = 0;

	for (int i = 0 ; i < bits ; ++i)
	{
		int bit = bit_r(p);
		
		if (bit < 0) return -1;
		
		rval |= bit << i;
	}
	
	return rval;
}


/* Write bits bits of v */
static void bit_wbits(BitPtr *p, Uint32 v, Uint8 bits)
{
	for (int i = 0 ; i < bits ; ++i)
	{
		bit_w(p, v & (1 << i));
	}
}


/* Gray and delta encoding helpers */
static inline Uint16 gray(Uint16 v)
{
	return v ^ (v >> 1);
}


static inline Uint16 degray(Uint16 v)
{
	v ^= (v>>8);
    v ^= (v>>4);
	v ^= (v>>2);
	v ^= (v>>1);
	return v;
}


static void gray_encode(Sint16 *buffer, const int n)
{
	for (int i = 0; i < n; ++i)
		buffer[i] = gray((Sint32)buffer[i] + 32768); // Gray code can not be negative
}


static void gray_decode(Sint16 *buffer, const int n)
{
	for (int i = 0; i < n; ++i)
		buffer[i] = (Sint32)degray(buffer[i]) - 32768; // Gray code can not be negative
}


static void delta_encode(Sint16 *buffer, const int n)
{
	Sint32 delta = 0;
	Sint32 original;
	for (int i = 0; i < n; ++i)
	{
		original = buffer[i];
		buffer[i] = original - delta;
		delta = original;
	}
}


static void delta_decode(Sint16 *buffer, const int n)
{
	Sint32 delta = 0;
	for (int i = 0; i < n; ++i)
	{
		buffer[i] = buffer[i] + delta;
		delta = buffer[i];
	}
}


/* Compress 16-bit signed data into bitstream, return compressed data size in packed_size (in bits) */
Uint8 * bitpack(const Sint16 *_buffer, const int n, int flags, Uint32 *packed_size)
{
	BitPtr bp;
	bit_init(&bp, NULL, 0, 0);
	
	Sint16 *buffer = malloc(sizeof(Sint16) * n);
	memcpy(buffer, _buffer, sizeof(Sint16) * n);
	
	if (flags & BITPACK_OPT_DELTA) delta_encode(buffer, n);
	if (flags & BITPACK_OPT_GRAY) gray_encode(buffer, n);
	
	for (int plane = 0 ; plane < sizeof(*buffer) * 8 ; ++plane)
	{
		const Sint16 mask = 1 << plane;
		int bit = mask & *buffer;
		
		int type = BITPACK_STATIC0 | (bit != 0);
		
		for (int i = 0 ; i < n ; ++i)
			if ((buffer[i] & mask) != bit)
			{
				// Was not all zeros or all ones, needs to compress
				type = BITPACK_RLE;
				break;
			}
			
		Uint32 p_byte = bp.byte;
		Uint32 p_bit = bp.bit;
		
again:
		
		bit_wbits(&bp, type, 2);
			
		switch (type)
		{
			case BITPACK_LITERAL:
				for (int i = 0 ; i < n ; ++i)
					bit_w(&bp, buffer[i] & mask);
				break;
			
			case BITPACK_STATIC0:			
			case BITPACK_STATIC1:
				// No data needed, bitplane is all zeros or all ones
				break;
				
			case BITPACK_RLE:
			{
				// Write starting bit state
				bit_w(&bp, bit);
				
				Uint32 ctr = 0;
				
				for (int i = 0 ; i < n ; ++i)
				{
					if (((buffer[i] & mask) == bit))
						++ctr;
						
					if ((buffer[i] & mask) != bit)
					{
						bit_wgamma(&bp, ctr);

						ctr = 1;
						
						// Flip the bit (no neighboring bits are the same state)
						bit ^= mask;
					}
				}
				
				if (ctr != 0) bit_wgamma(&bp, ctr);
				
				if ((bp.byte * 8 + bp.bit) - (p_byte * 8 + p_bit) > n + 2)
				{
					// RLE gave longer data than the original, dump data instead
					bit_seek(&bp, p_byte, p_bit);
					type = BITPACK_LITERAL;
					goto again;
				}
				
			}	
			break;
		}
		
	}
	
	free(buffer);
	
	*packed_size = bp.byte * 8 + bp.bit;
	
	return bp.buffer;
}


/* Decompress compressed bitstream into 16-bit signed data, decompress at most packed_size bits
	unpacked_size tells the function the data length (important)
 */
Sint16 * bitunpack(const Uint8 *packed_data, const Uint32 packed_size, Uint32 unpacked_size, int flags)
{
	BitPtr bp;
	bit_init(&bp, (Uint8*)packed_data, packed_size / 8, packed_size & 7);
	
	Sint16 *buffer = calloc(unpacked_size, sizeof(Sint16));
	
	for (int plane = 0 ; plane < sizeof(*buffer) * 8 ; ++plane)
	{
		const Sint16 mask = 1 << plane;
		
		Sint32 type = bit_rbits(&bp, 2);
		
		if (type < 0) goto read_error;
			
		switch (type)
		{
			case BITPACK_LITERAL:
				for (int i = 0 ; i < unpacked_size ; ++i)
				{
					int bit = bit_r(&bp);
					if (bit < 0) goto read_error;
					if (bit) buffer[i] |= mask;
				}
				break;
				
			case BITPACK_STATIC0:
				// Data bits are zero by default, no action needed
				break;
				
			case BITPACK_STATIC1:
				// Fill bitplane with set/unset bit
				for (int i = 0 ; i < unpacked_size ; ++i)
					buffer[i] |= mask;
				break;
				
			case BITPACK_RLE:
			{
				// Read the starting bit status
				int bit = bit_r(&bp);
				
				if (bit < 0) goto read_error;
				
				if (bit) bit = mask; else bit = 0;
				
				buffer[0] |= bit;
				
				for (int i = 0 ; i < unpacked_size ; )
				{
					Uint32 ctr = bit_rgamma(&bp);
					
					if (ctr == 0) goto read_error;
					
					for (; i < unpacked_size && ctr ; ++i, --ctr)
						buffer[i] |= bit;
					
					if (ctr) goto read_error;
					
					// Flip the bit (neighboring bits are always different)
					bit ^= mask;
				}
			}		
			break;
		}
	}
	
	if (flags & BITPACK_OPT_GRAY) gray_decode(buffer, unpacked_size);
	if (flags & BITPACK_OPT_DELTA) delta_decode(buffer, unpacked_size);
	
	if (0)
	{
read_error:
		free(buffer);
		return NULL;
	}
	
	return buffer;
}


/* Compress with best combination of options */
Uint8 * bitpack_best(const Sint16 *data, Uint32 data_size, Uint32 *_packed_size, int *flags)
{
	Uint32 best_packed_size = 0;
	Uint8 *best = NULL;
	int best_flags = 0;
	
	for (int i = 0 ; i < 4 ; ++i)
	{
		Uint32 packed_size;
		
		Uint8 * cdata = bitpack(data, data_size, i, &packed_size);
		
		if (best == NULL || best_packed_size > packed_size)
		{
			best_packed_size = packed_size;
			best = cdata;
			best_flags = i;
		}
		else
			free(cdata);
	}
	
	*_packed_size = best_packed_size;
	*flags = best_flags;
	
	return best;
}
