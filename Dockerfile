FROM rust:1.52

RUN apt update; apt install graphviz -y

WORKDIR /usr/src/doteur

COPY ./ .

RUN cargo install --path . --features "mysql_addons sqlite_addons"

RUN rm -rf ./* 

COPY ./samples .
