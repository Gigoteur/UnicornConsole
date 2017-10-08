CC=gcc
CFLAGS= -O2 -Wall -Wextra -std=gnu99 -fPIC

ECHO = echo
MKDIR= mkdir
AR= ar rcu
RANLIB= ranlib
RM= rm -f

ifdef COMSPEC
	SDLFLAGS := -I c:/mingw/include/SDL2
	SDLLIBS :=  -lSDL2main -lSDL2 -lSDL2_image -lwinmm
else
	SDLFLAGS := `sdl2-config --cflags`
	SDLLIBS := `sdl2-config --libs`
endif

ifdef SDL_FLAGS
	SDLFLAGS := -I $(SDL_FLAGS)
endif

ifdef SDL_LIBS
	SDLLIBS := $(SDL_LIBS)
endif

INCLUDEFLAGS := $(SDLFLAGS) -I ./
CFLAGS += -O3 -Wall ${INCLUDEFLAGS} -s -D USESDLMUTEXES -D NOSDL_MIXER

all: libchiptune.a

libchiptune.a: chiptune.o cyd.o cydchr.o cydcrush.o cydentry.o cydflt.o cydfm.o cydfx.o cydosc.o cydrvb.o cydwave.o freqs.o music.o pack.o
	@$(ECHO) "Linking libchiptune..."
	@$(AR) $@ $^
	@$(RANLIB) $@

%.o: %.c 
	@$(ECHO) "Compiling "$(notdir $<)"..."
	@$(MKDIR) -p objs
	$(CC) -c $(CFLAGS) -o $@ $<

sample: libchiptune.a
	$(CC) -c $(CFLAGS) samples/sample.c -o samples/sample.o -I ./
	$(CC) -o samples/sample samples/sample.o libchiptune.a $(SDLLIBS) -lm

clean:
	rm -fr libchiptune.a *.o samples/sample.o samples/sample