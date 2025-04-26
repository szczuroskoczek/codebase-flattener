# default shell on Unix (and everywhere except Windows)
set shell := ["bash", "-cu"]

# override on Windows
set windows-shell := ["powershell.exe", "-c"]

# justfile
default:
    @just --list --justfile {{justfile()}}

# Watch and rerun your Rust app automatically
watch:
    cargo watch -x run

release-linux:
    cargo build --release --target x86_64-unknown-linux-gnu; mv target/x86_64-unknown-linux-gnu/release/codebase-flattener bin/linux

release-windows:
    cargo build --release --target x86_64-pc-windows-msvc; mv target/x86_64-pc-windows-msvc/release/codebase-flattener.exe bin/windows.exe

publish-cargo:
    cargo publish

publish-npm:
    npm publish

package-debian:
    cargo deb