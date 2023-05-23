FROM ubuntu:latest

ARG USER_ID=1000
ARG GROUP_ID=1000

RUN apt-get update && apt install curl -y

RUN addgroup --gid $GROUP_ID user
RUN adduser --disabled-password --gecos '' --uid $USER_ID --gid $GROUP_ID user

USER 1000:1000

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

WORKDIR /usr/src/fileshare

RUN mkdir assets
RUN mkdir target
RUN mkdir files
RUN mkdir db
RUN mkdir release
COPY ../.env ./.env

CMD ["./release/filesharing"]
