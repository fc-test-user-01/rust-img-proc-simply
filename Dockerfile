FROM rust:latest

RUN apt update
RUN apt install wget curl vim nmap -y
COPY /src /workspace/src
WORKDIR /workspace
RUN mkdir /workspace/dist
RUN cargo new test-img-proc
WORKDIR /workspace/test-img-proc
RUN cargo add image imageproc
RUN cargo add base64 url regex
COPY /src/input.jpg /workspace/test-img-proc/
COPY /src/main.rs /workspace/test-img-proc/src/
RUN cargo run
WORKDIR /workspace/test-img-proc
