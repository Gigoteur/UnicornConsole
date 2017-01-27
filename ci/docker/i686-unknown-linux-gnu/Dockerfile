FROM japaric/i686-unknown-linux-gnu:v0.1.8

RUN apt-get update -q && \
    apt-get install -y wget libreadline-dev python-dev python3-dev

RUN wget https://www.libsdl.org/release/SDL2-2.0.5.tar.gz && tar xzf SDL2-2.0.5.tar.gz && cd SDL2-2.0.5 && ./configure && make && make install