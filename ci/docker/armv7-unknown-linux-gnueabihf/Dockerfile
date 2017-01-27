FROM resin/rpi-raspbian

RUN apt-get update -q
RUN apt-get install -y curl wget libreadline-dev libsdl2-dev python-dev python3-dev

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH /root/.cargo/bin:$PATH
RUN rustc