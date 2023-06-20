FROM ubuntu:latest
RUN apt-get update \
    && apt-get install -y python3.11-dev\
    && mkdir /app
WORKDIR /app
COPY fuzzer /app/fuzzer
VOLUME ["/work_dir"]
CMD ["./fuzzer","-g","/work_dir/grammars.py","-o","/work_dir","--","/work_dir/bin","@@"]