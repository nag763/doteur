FROM rust:1.52

RUN apt update; apt install graphviz -y

WORKDIR /usr/src/doteur

COPY ./examples .

RUN cargo install doteur
