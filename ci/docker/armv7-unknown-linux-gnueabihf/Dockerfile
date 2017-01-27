FROM resin/rpi-raspbian

RUN apt-get update -q
RUN apt-get install -y curl wget libreadline-dev libsdl2-dev python-dev python3-dev g++ gcc libc6 libc6-dev binutils git make file ca-certificates zip dpkg-dev

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH /root/.cargo/bin:$PATH

RUN git clone https://github.com/Gigoteur/PX8 && cd PX8 && \
    cargo build --release --features="sdl_renderer cpython lua dyon" && ldd target/release/px8