#!/bin/bash

for file in "work_dir"/*; do
    if [[ ! $file =~ /[^/]*_debug$ ]]; then
        rm -rf "$file"
    fi
done