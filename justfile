bin := if os_family() == "windows" { "config.dll" } else { "libconfig.so" }

build:
    @echo "Building Config..."
    @echo "This shouldn't take a moment..."
    cargo build --release
    @echo "Moving compiled binary"
    @rm -f "./lua/{{bin}}"
    @mkdir -p lua
    mv "./target/release/{{bin}}" "./lua/{{bin}}"
