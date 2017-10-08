#include "chiptune.h"
#include "music.h"


int main(int argc, char *argv[]) {

    ChiptunePlayer *player;
	ChiptuneSong *song;
	ChiptuneSound *sound, *sound2;

    SDL_Init(SDL_INIT_VIDEO|SDL_INIT_AUDIO|SDL_INIT_NOPARACHUTE|SDL_INIT_TIMER);

    printf("CREATE PLAYER\n");
	player = Chiptune_CreatePlayer(44100);
	printf("[E] CREATE PLAYER\n");

    printf("Load Music\n");
	song = Chiptune_LoadMusic(player, "./assets/ringmod.kt");
    printf("[E] Load Music\n");

    printf("Play Music\n");	
    Chiptune_PlayMusic(player, song, 0);
    printf("[E] Play Music\n");
    
    sleep(1);
    Chiptune_Pause(player, 1);    
    
    sleep(5);
    
    printf("Load Sound\n");
    sound = Chiptune_LoadSound(player, "./assets/sounds/the_horror.ki");
    sound2 = Chiptune_LoadSound(player, "./assets/sounds/clap.ki");
    
    printf("[E] Load Sound %x\n", sound);

	sleep(3);
    printf("MUSIC POSITION %d\n", Chiptune_GetMusicPlayPosition(player));

    printf("Play Sound 1\n");	
	Chiptune_PlaySound(player, sound, -1, 10000, CYD_PAN_CENTER, 50);
    printf("SOUND POSITION %d\n", Chiptune_GetSoundPlayPosition(player, 8));
        
    sleep(1);

    printf("SOUND POSITION %d\n", Chiptune_GetSoundPlayPosition(player, 8));
    
    printf("Play Sound 2\n");    
    Chiptune_PlaySound(player, sound2, -1, 10000, CYD_PAN_CENTER, 50);
    
    sleep(1);

    printf("Pause sound\n");
    Chiptune_Pause(player, 1);
    
    sleep(2);
    printf("Resume sound\n");	
	Chiptune_Pause(player, 0);

    printf("Change volume\n");	    
    Chiptune_SetVolume(player, 64);    

	sleep(5);

    Chiptune_Stop(player);
    printf("Stop sound\n");	

    sleep(1);

	Chiptune_FreePlayer(player);
	Chiptune_FreeSong(song);
		
	return 0;
}