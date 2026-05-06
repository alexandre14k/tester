#!/bin/bash

OUT="out"
BIN=$(basename "$PWD")
DIR="$OUT|doc|Cargo.lock"

do_cargo_check() {
    if [ ! -d "$OUT" ]; then
        mkdir -p "$OUT"
    fi
    cargo check -q --target-dir "$OUT"/test
}

do_cargo_test() {
    if [ ! -d "$OUT" ]; then
        mkdir "$OUT"
    fi
    cargo test -q --target-dir "$OUT"/test
}

do_cargo_build() {
    if [ ! -d "$OUT" ]; then
        mkdir "$OUT"
    fi

    target="x86_64-unknown-linux-musl"

    if ! rustup target list --installed | grep -q "$target"; then
        echo "installing $target..."
        rustup target add "$target"
    fi

    cargo build -q --release \
        --message-format=human\
        --target $target \
        --target-dir "$OUT"/prod

    cp "$OUT"/prod/$target/release/$BIN $OUT/
}

do_show_stats() {
    if [ -f "$OUT/$BIN" ]; then
        echo $(echo $OUT/$BIN;\
          du -b $OUT/$BIN | awk '{print $1/1024 " KB"}')
        echo $(ldd $OUT/$BIN)
    fi
}

do_run() {
    if [ -f "$OUT/$BIN" ]; then
        read -p "run with script ? [y/N] " confirm
        if [[ $confirm =~ ^[Yy]$ ]]; then
            $OUT/$BIN test.md
        else
            $OUT/$BIN
        fi
    fi
}

do_erase() {
    if [ -d "$OUT" ] || [ -d "target" ]; then
        read -p "confirm removing '$OUT/' ? [y/N] " confirm
        if [[ $confirm =~ ^[Yy]$ ]]; then
            rm -rf "$OUT"
            echo "done"
        else
            echo "abort"
        fi
    fi
}

do_dir_project() {
    tree -I $DIR
}

do_default() {
    do_clear_screen
    do_menu
}

do_clear_screen() {
    clear
}

do_menu() {
    echo ""
    echo "   menu cargo bash $BIN"
    echo ""
    echo "   d -- dir project"
    echo "   c -- cargo check"
    echo "   t -- cargo test"
    echo "   b -- cargo build"
    echo "   s -- stats build"
    echo "   r -- run"
    echo "   e -- clean"
    echo "   x -- exit"
    echo ""
}

do_input() {
    read -p '>> ' value
    echo "$value"
}

main() {
    do_menu
    while true; do
        value=$(do_input)

        case "$value" in
            d) do_dir_project;;
            c) do_cargo_check;;
            t) do_cargo_test;;
            b) do_cargo_build;;
            s) do_show_stats;;
            r) do_run;;
            e) do_erase;;
            x) break;;
            *) do_default;;
        esac
    done
}

if [ -t 0 ]; then
    main
else
    title="$BIN"
    xfce4-terminal\
        --title="$title"\
        -e "bash -c './make.sh $@; exec bash'"
fi
