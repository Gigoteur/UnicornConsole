FROM ubuntu:16.04

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    cmake \
    gcc \
    libc6-dev \
    make \
    pkg-config

RUN apt-get update -q && \
    apt-get install -y curl wget libreadline-dev libsdl2-dev python-dev python3-dev

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN rustc