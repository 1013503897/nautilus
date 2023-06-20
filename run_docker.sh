current_date=$(date +%Y%m%d%H%M%S)
folder_name="work_dir/${current_date}"
mkdir "$folder_name"
cp grammars/lua.py "$folder_name/grammars.py"
cp src/lua-5.4.6/src/lua "$folder_name/bin"
docker run --name fuzzer_$current_date -v "$(pwd)/$folder_name:/work_dir" -v "$(pwd)/config.ron:/app/config.ron" hermitcrab:latest