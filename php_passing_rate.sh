#!/bin/bash

dir="$1"
total_files=0
passing_files=0

if [ -z "$dir" ]; then
  echo "Please provide a directory path."
  exit 1
fi

if [ ! -d "$dir" ]; then
  echo "Invalid directory path: $dir"
  exit 1
fi

for file in "$dir"/*; do
  if [ -f "$file" ]; then
    total_files=$((total_files + 1))
    syntax_check=$(php -l "$file")
    if [[ $syntax_check == *"No syntax errors"* ]]; then
      echo $syntax_check
      passing_files=$((passing_files + 1))
    fi
  fi
done

echo "passing_files: $passing_files"
echo "total_files: $total_files"
if [ "$total_files" -gt 0 ]; then
  percentage=$((passing_files * 100 / total_files))
else
  percentage=100
fi

echo "Syntax pass rate: $percentage%"
