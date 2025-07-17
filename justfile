build:
    @echo "Building Config..."
    @echo "This shouldn't take a moment..."
    cargo build --release
    @echo "Moving compiled binary"
    @rm -f lua/config.dll
    @mkdir -p lua
    mv target/release/config.dll lua/config.dll
