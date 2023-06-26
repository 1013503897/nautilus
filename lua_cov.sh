#!/bin/bash


directory=$1
lua_src="src/lua-5.4.6-cov/src"
lua_executable="$lua_src/lua"
timeout=5

if [ -z "$directory" ]; then
  #echo "Usage: $0 <directory>"
  #exit 1
  pids=$(pgrep -f "fuzzer")

  for pid in $pids; do
    cmdline=$(tr '\0' '\n' < "/proc/$pid/cmdline")

    directory=$(echo "$cmdline" | tr '\0' '\n' | grep -m1 "work_dir")
    echo "directory: $directory"
    if [[ -z "$directory" ]]; then
      exit 
    fi
  done
  echo "Using directory: $directory"
fi

for file in "$directory"/outputs/queue/*; do
  echo "Executing file: $file"
  timeout $timeout "$lua_executable" "$file"

  exit_status=$?
  if [ $exit_status -eq 124 ]; then
    rm "$file"
  elif [ $exit_status -ne 0 ]; then
    rm "$file"
  fi
done

hermit-cov --live -O -d $directory  -e="$lua_executable 'AFL_FILE'" -c $lua_src

echo "starting web server.."
python3 -m http.server -d $directory/cov/web 80

