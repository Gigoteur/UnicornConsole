CC=gcc
CFLAGS= -O2 -Wall -Wextra -std=gnu99 -fPIC

ECHO = echo
MKDIR= mkdir
AR= ar rcu
RANLIB= ranlib
RM= rm -f

CFLAGS += -O3 -Wall

all: libduktape.a

libduktape.a: duktape.o duktape_binding.o
	@$(ECHO) "Linking libduktape..."
	@$(AR) $@ $^
	@$(RANLIB) $@

%.o: %.c 
	@$(ECHO) "Compiling "$(notdir $<)"..."
	@$(MKDIR) -p objs
	$(CC) -c $(CFLAGS) -o $@ $<

sample: libduktape.a
	$(CC) -c $(CFLAGS) samples/sample.c -o samples/sample.o -I ./
	$(CC) -o samples/sample samples/sample.o libduktape.a -lm

clean:
	rm -fr libduktape.a *.o objs* samples/sample.o samples/sample