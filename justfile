bin_end := if os() == "windows" { ".dll" } else if os() == "macos" { ".dylib" } else { ".so" }
bin := if os_family() == "windows" { "config.dll" } else { "libconfig" + bin_end }

build:
    @echo "Building Config..."
    @echo "This shouldn't take a moment..."
    cargo build --release
    @echo "Moving compiled binary"
    @rm -f "./lua/config{{bin_end}}"
    @mkdir -p lua
    mv "./target/release/{{bin}}" "./lua/config{{bin_end}}"
