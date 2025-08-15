bin_end := if os() == "windows" { ".dll" } else if os() == "macos" { ".dylib" } else { ".so" }
bin := if os_family() == "windows" { "config.dll" } else { "libconfig" + bin_end }

build:
    @echo "Checking for requirement"

    @echo "{{GREEN}}Neovim version:{{NORMAL}}"
    @nvim -v
    @echo "{{GREEN}}Git version:{{NORMAL}}"
    @git -v
    @echo "{{GREEN}}Cargo version:{{NORMAL}}"
    @cargo -V
    @echo "{{GREEN}}Make version:{{NORMAL}}"
    @make -v

    @echo "{{GREEN}}Building Config..{{NORMAL}}."
    @echo "{{YELLOW}}This shouldn't take a moment...{{NORMAL}}"
    cargo build --release
    @echo "{{GREEN}}Moving compiled binary{{NORMAL}}"
    @rm -f "./lua/config{{bin_end}}"
    @mkdir -p lua
    @mv "./target/release/{{bin}}" "./lua/config{{bin_end}}"
    @echo "{{GREEN}}ALL DONE{{NORMAL}}"

quick_build:
    cargo build --release
    rm -f "./lua/config{{bin_end}}"
    mkdir -p lua
    mv "./target/release/{{bin}}" "./lua/config{{bin_end}}"
