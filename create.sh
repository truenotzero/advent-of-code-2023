#!/bin/bash

# remove comment to enable debug mode
# set -x

CRATES_DIR="./crates"

new() {
    local create=1
    # first check if the directory exists
    if [ -d "$CRATES_DIR/$1" ]; then
        echo Found "$CRATES_DIR/$1", checking validity...

        # check that it's a valid rust project
        if [ ! -f "$CRATES_DIR/$1/Cargo.toml" ]; then
            echo Invalid project...
            return 1
        else
            echo Project valid, continuing
            create=0
            return 0
        fi
    fi

    if [ "$create" -eq 1 ]; then
        local project="$CRATES_DIR/$1"
        echo Creating new project...
        cargo new "$project"

        echo Setting up hierarchy...
        echo Removing "src/main.rs"
        rm "$project/src/main.rs"
        echo Removing git repo...
        rm -rf "$project/.git"
        rm "$project/.gitignore"
        echo Creating "src/bin/"
        mkdir "$project/src/bin"
        echo Creating input.txt
        touch "$project/input.txt"
        echo Creating example.txt
        touch "$project/example.txt"
        echo New project creation finished
    fi
}

edit_cfg() {
    local toml="$1"
    local day="$2"
    local part="$3"
    echo Editing "$toml"
    echo "[[bin]]"                      >> $toml
    echo "name=\"$day-$part\""          >> $toml
    echo "path=\"src/bin/$part.rs\""    >> $toml
}

add_part1() {
    local day="$1"
    new "$day"
    local project="$CRATES_DIR/$day"
    if [ $? -eq 0 ]; then
        local part="part1"
        local src="./part.rs"
        local dest="$project/src/bin/$part.rs"

        if [ -f "$dest" ]; then
            echo Found, "$dest", skipping creation
            return 0
        fi

        echo Copying "$dest"...
        cp "$src" "$dest"

        local toml="$project/Cargo.toml"
        edit_cfg "$toml" "$day" "$part"
    fi
}

add_part2() {
    local day="$1"
    add_part1 "$day"
    if [ $? -ne 0 ]; then return "$?"; fi 

    local project="$CRATES_DIR/$day"
    local part="part2"
    local src="$project/src/bin/part1.rs"
    local dest="$project/src/bin/$part.rs"

    if [ -f "$dest" ]; then
        echo Found, "$dest", skipping creation
        return 0
    fi

    echo Copying "$dest"...
    cp "$src" "$dest"

    local toml="$project/Cargo.toml"
    edit_cfg "$toml" "$day" "$part" 
}

run() {
    local day="$1"
    local part="$2"
    if [ -z "$part" ]; then local part="part2"; fi

    pushd "$CRATES_DIR/$day"
    if [ "$?" -ne 0 ]; then return 1; fi
    if [ -f "./Cargo.toml" ]; then
        local prefix="./src/bin"
        if [ ! -f "$prefix/$part.rs" ]; then
            local part="part1"
        fi

        cargo run --bin "$day-$part"
    fi

    popd
}

command="$1"
arg="$2"
arg2="$3"

if [ "$command" == "add" ]; then
    new "$arg"
elif [ "$command" == "part1" ]; then
    add_part1 "$arg"
elif [ "$command" == "part2" ]; then
    add_part2 "$arg" 
elif [ "$command" == "run" ]; then
    run "$arg" "$arg2"
else false; fi # so that 'cargo run' doesn't run below

if [ "$?" -ne 0 ]; then
    echo Failed to run with args: $@
    exit
fi

echo All done, running cargo check...
cargo check --package "$arg"
