set -ex

main() {
    cd unicorn
    cargo build --release
}

main
