# This script takes care of testing your crate

set -ex

# TODO This is the "test phase", tweak it as you see fit
main() {
    cross build --target $TARGET --release --features="sdl_renderer cpython lua dyon"

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross test --target $TARGET --release --features="sdl_renderer cpython lua dyon"
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi