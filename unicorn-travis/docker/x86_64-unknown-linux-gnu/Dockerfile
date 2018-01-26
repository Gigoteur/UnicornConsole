FROM ubuntu:16.04

RUN apt-get update -q

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    cmake \
    gcc \
    libc6-dev \
    make \
    pkg-config

RUN apt-get install -y curl wget libsdl2-mixer-dev libsdl2-dev python-dev python3-dev g++ gcc libc6 libc6-dev binutils git make file ca-certificates zip dpkg-dev

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH /root/.cargo/bin:$PATH


RUN git clone https://github.com/Gigoteur/UnicornConsole && cd UnicornConsole && cd unicorn && \
    cargo build --release