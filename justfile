default:
    just --list

build-usb:
    cargo build --release
    cargo make uf2