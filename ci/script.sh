set -ex

main() {
   # cargo build --release
   # cargo build --release --features="sdl_renderer"
   # cargo build --release --features="cpython"
   # cargo build --release --features="lua"
    cargo build --release --features="cpython lua"
}

main_deploy() {
    docker run --rm --privileged multiarch/qemu-user-static:register --reset
    docker build -t px8-$TARGET ci/docker/$TARGET
}

if [ ! -z $DEPLOY ]; then
    main_deploy
else
    main
fi
