#!/bin/sh

set -e

latest_asset_path=$(curl -sSf https://github.com/nicochatzi/soultrain/releases |
        grep -o "/nicochatzi/soultrain/releases/download/.*/soultrain-osx-x64.zip" |
        head -n 1)
uri="https://github.com${latest_asset_path}"
root_dir="${SOULTRAIN_DIR:-$HOME/.soultrain}"
bin_dir="$root_dir/bin"
file="$bin_dir/soultrain"

err() {
    printf "$1" >&2
    exit 1
}

ensure() {
    if ! "$@"; then err "command failed: $*"; fi
}

downloader() {
    echo "~~> Downloading this release: $2"
    ensure curl --proto '=https' --tlsv1.2 -o "$1".zip --location "$2"
}

make_dir() {
    if [ ! -d "$1" ]; then
        mkdir -p "$1"
    fi
}

installer() {
    ensure unzip "$1".zip -d "$2"
    ensure chmod +x "$1"
    ensure rm -f "$1".zip
    exec
}

done_message() {
    echo ""
    echo "Add soultrain to your PATH by adding the following lines in your ~/.bash_profile or ~/.zshrc"
	echo "  export SOULTRAIN_DIR=\"$root_dir\""
	echo "  export PATH=\"\$SOULTRAIN_DIR/bin:\$PATH\""
    echo ""
}

main() {
    make_dir "$bin_dir"
    downloader "$file" "$uri"
    installer "$file" "$bin_dir"
    done_message
}

main "$@" || exit 1
