#include "chiptune.h"
#include "music.h"
#include "cyd.h"
#include "macros.h"
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>


struct ChiptuneSound_t
{
	MusInstrument sound;
	CydWavetableEntry wavetable_entries[CYD_WAVE_MAX_ENTRIES];
};

struct ChiptuneSong_t 
{
	MusSong song;
	CydWavetableEntry wavetable_entries[CYD_WAVE_MAX_ENTRIES];
};


struct ChiptunePlayer_t 
{
	CydEngine cyd_music;
	MusEngine mus_music;

    bool cyd_registered;
};

KLYSAPI ChiptunePlayer* Chiptune_CreatePlayer(int sample_rate)
{
	ChiptunePlayer *player = Chiptune_CreatePlayerUnregistered(sample_rate);
	
	player->cyd_registered = true;

	cyd_register(&player->cyd_music, 4096);	
	cyd_reserve_channels(&player->cyd_music, CHANNELS);

	return player;
}

KLYSAPI ChiptunePlayer* Chiptune_CreatePlayerUnregistered(int sample_rate)
{
	ChiptunePlayer *player = malloc(sizeof(*player));
	
	player->cyd_registered = false;
	
    // Music
	cyd_init(&player->cyd_music, sample_rate, 1);
	mus_init_engine(&player->mus_music, &player->cyd_music);
	
	// Each song has its own wavetable array so let's free this
	free(player->cyd_music.wavetable_entries); 
	player->cyd_music.wavetable_entries = NULL;

	return player;
}

KLYSAPI ChiptuneSong* Chiptune_LoadMusic(ChiptunePlayer* player, const char *path)
{
	ChiptuneSong *song = calloc(sizeof(*song), 1);
	
	int i = 0;
	for (i = 0 ; i < CYD_WAVE_MAX_ENTRIES ; ++i)
	{
		cyd_wave_entry_init(&song->wavetable_entries[i], NULL, 0, 0, 0, 0, 0);
	}
	
	if (mus_load_song(path, &song->song, song->wavetable_entries))
	{
		return song;
	}
	else
	{
		free(song);
		return NULL;
	}
}

KLYSAPI ChiptuneSound* Chiptune_LoadSound(ChiptunePlayer* player, const char *path)
{
    ChiptuneSound *sound = calloc(sizeof(*sound), 1);

	if (mus_load_instrument(path, &sound->sound, NULL))
	{
		return sound;
	}
	else
	{
		free(sound);
		return NULL;
	}
}

static int RWread(struct RWops *context, void *ptr, int size, int maxnum)
{
	const int len = my_min(size * maxnum, context->mem.length - context->mem.ptr);
	memcpy(ptr, context->mem.base + context->mem.ptr, len);
	
	context->mem.ptr += len;
	
	return len;
}


static int RWclose(struct RWops *context)
{
	free(context);
	return 1;
}


KLYSAPI ChiptuneSong* Chiptune_LoadMusicFromMemory(ChiptunePlayer* player, void *data, int data_size)
{
#ifndef USESDL_RWOPS
	RWops *ops = calloc(sizeof(*ops), 1);
	ops->read = RWread;
	ops->close = RWclose;
	ops->mem.base = data;
	ops->mem.length = data_size;
#else
	RWops *ops = SDL_RWFromMem(data, data_size);
#endif
	
	ChiptuneSong *song = calloc(sizeof(*song), 1);
	
	int i = 0;
	for (i = 0 ; i < CYD_WAVE_MAX_ENTRIES ; ++i)
	{
		cyd_wave_entry_init(&song->wavetable_entries[i], NULL, 0, 0, 0, 0, 0);
	}
	
	if (mus_load_song_RW(ops, &song->song, song->wavetable_entries))
	{
		return song;
	}
	else
	{
		free(song);
#ifndef USESDL_RWOPS
		RWclose(ops);
#else
		SDL_RWclose(ops);
#endif
		return NULL;
	}
}


KLYSAPI ChiptuneSound* Chiptune_LoadSoundFromMemory(ChiptunePlayer* player, void *data, int data_size)
{
#ifndef USESDL_RWOPS
	RWops *ops = calloc(sizeof(*ops), 1);
	ops->read = RWread;
	ops->close = RWclose;
	ops->mem.base = data;
	ops->mem.length = data_size;
#else
	RWops *ops = SDL_RWFromMem(data, data_size);
#endif

    ChiptuneSound *sound = calloc(sizeof(*sound), 1);

	if (mus_load_instrument_RW2(ops, &sound->sound, NULL))
	{
		return sound;
	}
	else
	{
		free(sound);
#ifndef USESDL_RWOPS
		RWclose(ops);
#else
		SDL_RWclose(ops);
#endif
		return NULL;
	}
}


KLYSAPI void Chiptune_FreeSong(ChiptuneSong *song)
{
	int i = 0;
	for (i = 0 ; i < CYD_WAVE_MAX_ENTRIES ; ++i)
	{
		cyd_wave_entry_init(&song->wavetable_entries[i], NULL, 0, 0, 0, 0, 0);
	}

	mus_free_song(&song->song);
	free(song);
}


KLYSAPI int Chiptune_GetSongLength(const ChiptuneSong *song)
{
	return song->song.song_length;
}



KLYSAPI void Chiptune_SetPlayerQuality(ChiptunePlayer *player, int oversample)
{
	cyd_set_oversampling(&player->cyd_music, oversample);
}


KLYSAPI void Chiptune_FreePlayer(ChiptunePlayer *player)
{
	Chiptune_Stop(player);
	
	if (player->cyd_registered) 
		cyd_unregister(&player->cyd_music);
		
	cyd_deinit(&player->cyd_music);
	free(player);
}


KLYSAPI void Chiptune_PlayMusic(ChiptunePlayer *player, ChiptuneSong *song, int start_position)
{
	player->cyd_music.wavetable_entries = song->wavetable_entries;
	cyd_set_callback(&player->cyd_music, mus_advance_tick, &player->mus_music, song->song.song_rate);
	mus_set_fx(&player->mus_music, &song->song);
		
	mus_set_song(&player->mus_music, &song->song, start_position);
}

KLYSAPI void Chiptune_PlaySound(ChiptunePlayer *player, ChiptuneSound *sound, int chan, unsigned short note, int panning, int rate)
{
	if (chan == -1)
	{
		for (int i = SFX_CHANNELS_START ; i < player->cyd_music.n_channels ; ++i)
		{
			if (!(player->cyd_music.channel[i].flags & CYD_CHN_ENABLE_GATE))
				chan = i;
		}
	}

	if (chan >= SFX_CHANNELS_START) {
		cyd_set_callback(&player->cyd_music, mus_advance_tick, &player->mus_music, rate);
		mus_trigger_instrument(&player->mus_music, chan, &sound->sound, note, panning);
	}
}

KLYSAPI int Chiptune_FillBuffer(ChiptunePlayer *player, short int *buffer, int buffer_length)
{
#ifdef NOSDL_MIXER
	cyd_output_buffer_stereo(&player->cyd_music, (void*)buffer, buffer_length);
#else
	cyd_output_buffer_stereo(0, buffer, buffer_length, &player->cyd_music);
#endif

	return player->cyd_music.samples_output;
}


KLYSAPI void Chiptune_Stop(ChiptunePlayer *player)
{
	mus_set_song(&player->mus_music, NULL, 0);
	cyd_set_callback(&player->cyd_music, NULL, NULL, 1);
	player->cyd_music.wavetable_entries = NULL;
}

KLYSAPI void Chiptune_Pause(ChiptunePlayer *player, int state)
{
	cyd_pause(&player->cyd_music, state);
}

KLYSAPI int Chiptune_GetMusicPlayPosition(ChiptunePlayer *player)
{
	int song_position = 0;
	
	mus_poll_status(&player->mus_music, &song_position, NULL, NULL, NULL, NULL, NULL, NULL);
	
	return song_position;
}


KLYSAPI int Chiptune_GetSoundPlayPosition(ChiptunePlayer *player, int chan)
{
	if (chan >= MUS_MAX_CHANNELS) {
		return -1;
	}
	
	cyd_lock(&player->cyd_music, 1);

	MusChannel *chn = &player->mus_music.channel[chan];
	if (chn == NULL) {
		return -1;
	}
	int value = chn->program_tick;

	cyd_lock(&player->cyd_music, 0);

	return value;
}

KLYSAPI void Chiptune_SetVolume(ChiptunePlayer *player, int volume)
{
	cyd_lock(&player->cyd_music, 1);
	player->mus_music.volume = volume;
	cyd_lock(&player->cyd_music, 0);
}


KLYSAPI void Chiptune_GetVUMeters(ChiptunePlayer *player, int *dest, int n_channels)
{
	int temp[MUS_MAX_CHANNELS];
	mus_poll_status(&player->mus_music, NULL, NULL, NULL, NULL, temp, NULL, NULL);
	memcpy(dest, temp, sizeof(dest[0]) * n_channels);
}


KLYSAPI const ChiptuneSongInfo * Chiptune_GetSongInfo(ChiptuneSong *song, ChiptuneSongInfo *data)
{
	static ChiptuneSongInfo buffer;

	if (data == NULL)
		data = &buffer;

	data->song_title = song->song.title;
	data->n_instruments = song->song.num_instruments;
	data->n_channels = song->song.num_channels;
	
	int i = 0;
	for (i = 0 ; i < data->n_instruments ; ++i)
		data->instrument_name[i] = song->song.instrument[i].name;
		
	return data;
}


KLYSAPI void Chiptune_SetLooping(ChiptunePlayer *player, int looping)
{
	if (looping)
		player->mus_music.flags |= MUS_NO_REPEAT;
	else
		player->mus_music.flags &= ~MUS_NO_REPEAT;
}


KLYSAPI int Chiptune_GetPlayTime(ChiptuneSong *song, int position)
{
	return mus_get_playtime_at(&song->song, position);
}
