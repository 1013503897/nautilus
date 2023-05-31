#!/bin/zsh

if [ $# -eq 1 ]; then
    if [ "$1" != "php" ] && [ "$1" != "lua" ] && [ "$1" != "python" ]; then
        echo "Invalid parameter $1"
        exit 1
    else
        current_date=$(date +%Y%m%d)
        folder_name="work_dir/$1_${current_date}"
        if [ ! -d "$folder_name" ]; then
            mkdir "$folder_name"
        else
            echo "Folder already exists: $folder_name"
        fi

        if [ "$1" = "php" ]; then
            cargo run --release -- -g grammars/php.py -o "$folder_name" -- src/php-8.1.13/sapi/cli/php @@
        elif [ "$1" = "lua" ]; then
            cargo run --release -- -g grammars/lua.py -o "$folder_name" -- src/lua-5.4.6/src/lua @@
        elif [ "$1" = "python" ]; then
            cargo run --release -- -g grammars/python.py -o "$folder_name" -- src/Python-3.10.11/python @@
        fi
    fi
else
    echo "Invalid number of parameters"
fi
