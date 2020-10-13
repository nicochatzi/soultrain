#!/bin/sh

set -e

latest_asset_path=$(curl -sSf https://github.com/nicochatzi/soultrain/releases |
        grep -o "/nicochatzi/soultrain/releases/download/.*/soultrain-osx-x64.zip" |
        head -n 1)
uri="https://github.com${latest_asset_path}"
root_dir="${SOULTRAIN_DIR:-$HOME/.soultrain}"
bin_dir="$root_dir/bin"
file="$bin_dir/soultrain"

main() {
    ensure downloader "$uri" "$file"
    enusre installer "$bin_dir" "$file"
    ensure exec

    done_message
}

downloader() {
    curl --progress-bar --proto '=https' --tlsv1.2 --silent --show-error --fail --location "$1" --output "$2"
}

installer() {
    ensure make_dir "$1"
    ensure unzip -d "$1" -o "$2.zip"
    ensure chmod +x "$2"
    ensure rm "$2.zip"
}

make_dir() {
    if [ ! -d "$1" ]; then
        mkdir -p "$1"
    fi
}

ensure() {
    if ! "$@"; then err "command failed: $*"; fi
}

done_message() {
    echo "Add soultrain to your PATH by adding the following lines in your ~/.bash_profile or ~/.zshrc"
	echo "  export SOULTRAIN_DIR=\"$root_dir\""
	echo "  export PATH=\"\$SOULTRAIN_DIR/bin:\$PATH\""
    echo ""
}

main "$@" || exit 1
