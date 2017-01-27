FROM japaric/armv7-unknown-linux-gnueabihf:v0.1.8

RUN apt-get update -q && \
    apt-get install -y wget python-dev python3-dev

RUN wget ftp://ftp.gnu.org/gnu/readline/readline-6.3.tar.gz && tar xvf readline-6.3.tar.gz && cd readline-6.3 && ./configure && make && make install

RUN wget https://www.libsdl.org/release/SDL2-2.0.5.tar.gz && tar xzf SDL2-2.0.5.tar.gz && cd SDL2-2.0.5 && ./configure && make && make install