#!/bin/zsh

if [ $# -eq 1 ]; then
    if [ "$1" != "php" ] && [ "$1" != "lua" ] && [ "$1" != "python" ] && [ "$1" != "lua_bin" ]; then
        echo "Invalid parameter $1"
        exit 1
    else
        current_date=$(date +%Y%m%d%H%M%S)
        folder_name="work_dir/$1_${current_date}"
        echo "Creating folder $folder_name"
        mkdir "$folder_name"

        if [ "$1" = "php" ]; then
            cargo run --release -- -g grammars/php.py -o "$folder_name" -- src/php-8.1.13/sapi/cli/php @@
        elif [ "$1" = "lua" ]; then
            cargo run --release -- -g grammars/lua.py -o "$folder_name" -- src/lua-5.4.6/src/lua @@
        elif [ "$1" = "python" ]; then
            cargo run --release -- -g grammars/python.py -o "$folder_name" -- src/Python-3.10.11/python @@
        elif [ "$1" = "lua_bin" ]; then
            cargo run afl-qemu-trace -- -g grammars/python.py -o "$folder_name" -- src/lua-5.4.6-bin/src/lua @@
        fi
    fi
else
    echo "Invalid number of parameters"
fi
