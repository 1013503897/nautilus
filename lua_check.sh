#!/bin/bash

pass_count=0
total_count=0
luac_path=/home/kali/fuzz/hermitcrab/src/lua-5.4.6/src/luac
directory=$1

for file in "$directory"/*; do
    if [ -f "$file" ]; then
        $luac_path -p "$file"

        if [ $? -eq 0 ]; then  
            echo "No syntax errors :$file"
            ((pass_count++))
        fi

        ((total_count++))
    fi
done

pass_rate=$((pass_count * 100 / total_count))
echo "Pass rate: $pass_rate%"