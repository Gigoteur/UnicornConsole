set -ex

main() {
    docker run --rm --privileged multiarch/qemu-user-static:register --reset
    docker build -t px8-$TARGET ci/docker/$TARGET
}

if [ -z $DEPLOY ]; then
    main
fi