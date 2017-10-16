#ifndef Chiptune_DLL_H
#define Chiptune_DLL_H

#define CHANNELS 32
#define SFX_CHANNELS_START 16

/**
 * @file ksnd.h
 * @author  Tero Lindeman <kometbomb@kometbomb.net>
 * @brief This is a wrapper library for simple klystrack music playback.
 */

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Song instance created by Chiptune_LoadMusic() and Chiptune_LoadSongFromMemory()
 *
 * Use Chiptune_FreeSong() to free.
 */
typedef struct ChiptuneSong_t ChiptuneSong;
typedef struct ChiptuneSound_t ChiptuneSound;

/**
 * Player context created by Chiptune_CreatePlayer() and Chiptune_CreatePlayerUnregistered()
 *
 * Use Chiptune_FreePlayer() to free.
 */
typedef struct ChiptunePlayer_t ChiptunePlayer;

/**
 * Song information returned by Chiptune_GetSongInfo()
 */
typedef struct 
{
	char *song_title; 			/**< Song title as a null-terminated string */
	char *instrument_name[128]; /**< Instrument names as an array */
	int n_instruments;			/**< Number of instruments used by the song */
	int n_channels;				/**< Number of channels used by the song */
} ChiptuneSongInfo;


#ifdef WIN32 
#ifdef DLLEXPORT
#define KLYSAPI _cdecl __declspec(dllexport)
#else
#define KLYSAPI 
#endif
#else
#define KLYSAPI 
#endif

/**
 * Loads a klystrack song file
 *
 * @param player the @c ChiptunePlayer context you will use to play the song
 * @param path path to the klystrack song to be loaded
 * @return @c ChiptuneSong or @c NULL if there was an error.
 */
KLYSAPI extern ChiptuneSong * Chiptune_LoadMusic(ChiptunePlayer* player, const char *path);
KLYSAPI extern ChiptuneSound * Chiptune_LoadSound(ChiptunePlayer* player, const char *path);

/**
 * Loads a klystrack song from memory
 *
 * @param player the @c ChiptunePlayer context you will use to play the song
 * @param[in] data pointer to data containing the song
 * @param data_size size of data pointed by @a data
 * @return @c ChiptuneSong or @c NULL if there was an error.
 */
KLYSAPI extern ChiptuneSong * Chiptune_LoadMusicFromMemory(ChiptunePlayer* player, void *data, int data_size);
KLYSAPI extern ChiptuneSound* Chiptune_LoadSoundFromMemory(ChiptunePlayer* player, void *data, int data_size);

/**
 * Free memory reserved for a @c ChiptuneSong instance
 *
 * @param song song to be freed
 */
KLYSAPI extern void Chiptune_FreeSong(ChiptuneSong *song);

/**
 * Get song length measured in pattern rows
 */
KLYSAPI extern int Chiptune_GetSongLength(const ChiptuneSong *song);

/**
 * Get song information from a @c ChiptuneSong.
 *
 * If @c NULL is passed as the @a data pointer, an internal, thread-unsafe buffer will
 * be used to store the data. This internal buffer will also change every time this 
 * function is called, thus it is preferable to supply your own @c ChiptuneSongInfo or 
 * otherwise copy the returned data before a new call.
 *
 * If @c song is freed using Chiptune_FreeSong(), the corresponding @c ChiptuneSongInfo may be 
 * invalidated and point to unallocated memory.
 *
 * @param song @c ChiptuneSong whose information is queried
 * @param[out] data pointer to a @c ChiptuneSongInfo structure, or @c NULL
 * @return pointer to the @c ChiptuneSongInfo structure passed as @a data or the internal buffer
 */
KLYSAPI extern const ChiptuneSongInfo * Chiptune_GetSongInfo(ChiptuneSong *song, ChiptuneSongInfo *data);

/**
 * Returns the amount of milliseconds that would need to elapse if the song was played
 * from the beginning to pattern row @a position. 
 *
 * Use this in conjunction with Chiptune_GetPlayPosition() to find out the current playback 
 * time. Or, use with Chiptune_GetSongLength() to get the song duration.
 *
 * @param song song whose play time is polled
 * @param position 
 * @return @c position measured in milliseconds
 */
KLYSAPI extern int Chiptune_GetPlayTime(ChiptuneSong *song, int position);

/**
 * Create a @c ChiptunePlayer context and playback thread.
 *
 * After setting a @c ChiptuneSong to be played with Chiptune_PlaySong() the song will start playing
 * automatically.
 *
 * @param sample_rate sample rate in Hz
 * @return @c ChiptunePlayer context or @c NULL if there was an error
 */
KLYSAPI extern ChiptunePlayer* Chiptune_CreatePlayer(int sample_rate);

/**
 * Create a @c ChiptunePlayer context but don't create a playback thread.
 *
 * You will need to use Chiptune_FillBuffer() to request audio data manually.
 *
 * @param sample_rate sample rate in Hz
 * @return @c ChiptunePlayer context or @c NULL if there was an error
 */
KLYSAPI extern ChiptunePlayer* Chiptune_CreatePlayerUnregistered(int sample_rate);

/**
 * Free a @c ChiptunePlayer context and associated memory.
 */
KLYSAPI extern void Chiptune_FreePlayer(ChiptunePlayer *player);

/**
 * Start playing song @c song from position @c start_position (measured in pattern rows)
 */
KLYSAPI extern void Chiptune_PlayMusic(ChiptunePlayer *player, ChiptuneSong *song, int start_position);
KLYSAPI extern int Chiptune_PlaySound(ChiptunePlayer *player, ChiptuneSound *sound, int chan, unsigned short note, int panning, int rate);

KLYSAPI extern void Chiptune_GetSoundInfo(ChiptuneSound *sound);

/**
 * Stop playback on a player context.
 */
KLYSAPI extern void Chiptune_Stop(ChiptunePlayer* player);
KLYSAPI extern void Chiptune_StopMusic(ChiptunePlayer* player);
KLYSAPI extern void Chiptune_StopSound(ChiptunePlayer* player);

/**
 * Pause or continue playback on a player context.
 *
 * This will pause the actual player thread so it has no effect on players created with Chiptune_CreatePlayerUnregistered().
 *
 * @param player player context
 * @param state set pause state, value of @c 1 pauses the song and @c 0 continues playback
 *
 */
KLYSAPI extern void Chiptune_Pause(ChiptunePlayer *player, int state);

/**
 * Fill a buffer of 16-bit signed integers with audio. Data will be interleaved for stereo audio.
 *
 * Use only with a @c ChiptunePlayer context created with Chiptune_CreatePlayerUnregistered().
 *
 * @param player player context
 * @param[out] buffer buffer to be filled
 * @param buffer_length size of @a buffer in bytes
 * @return number of samples output
 */
KLYSAPI extern int Chiptune_FillBuffer(ChiptunePlayer *player, short int *buffer, int buffer_length);

/**
 * Set player oversampling quality.
 *
 * Oversample range is [0..4]. 0 implies no oversampling and 4 implies 16-fold oversampling.
 * The less oversampling the less CPU intensive the playback is but the sound quality will suffer 
 * for very high frequencies. Can be adjusted realtime.
 *
 * @param player @c ChiptunePlayer context
 * @param oversample oversample rate
 */
KLYSAPI extern void Chiptune_SetPlayerQuality(ChiptunePlayer *player, int oversample);

/**
 * Set playback volume.
 *
 * @param player player context which is currently playing a song set with Chiptune_PlaySong()
 * @param volume volume [0..128]
 */
KLYSAPI extern void Chiptune_SetVolume(ChiptunePlayer *player, int volume);

/**
 * Enable or disable song looping.
 *
 * If a non-zero value is passed via @c looping, the playback routine will exit Chiptune_FillBuffer()
 * if it encounters the song end. This is useful if you want to detect song end and avoid extra 
 * audio data in the buffer after the song end. 
 *
 * @param player player context
 * @param looping looping enabled
 */
KLYSAPI extern void Chiptune_SetLooping(ChiptunePlayer *player, int looping);

/**
 * Returns current position in the song.
 *
 * @param player player context which is currently playing a song set with Chiptune_PlaySong()
 * @return current playback position measured in pattern rows
 */
KLYSAPI extern int Chiptune_GetMusicPlayPosition(ChiptunePlayer* player);
KLYSAPI extern int Chiptune_GetSoundPlayPosition(ChiptunePlayer *player, int chan);

/**
 * Get the current envelope values for each player channel.
 *
 * Envelope values range from 0 (no audio) to 128 (full volume).
 *
 * @param player player context which is currently playing a song set with Chiptune_PlaySong()
 * @param[out] envelope array of 32-bit signed integers, should contain @c n_channel items
 * @param[in] n_channels get envelope values for @c n_channel first channels
 */
KLYSAPI extern void Chiptune_GetVUMeters(ChiptunePlayer *player, int *envelope, int n_channels);

#ifdef __cplusplus
}
#endif

#endif
