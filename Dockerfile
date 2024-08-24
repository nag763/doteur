FROM rust:1.80.1

RUN apt update; apt install graphviz gcc libssl-dev libsqlite3-dev -y

WORKDIR /usr/src/doteur

COPY ./ .

RUN cargo install --path doteur_cli --all-features

RUN rm -rf ./* 

COPY ./samples .
