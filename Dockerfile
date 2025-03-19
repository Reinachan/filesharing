FROM ubuntu:latest

ARG USER_ID=1000
ARG GROUP_ID=1000

USER 1000:1000

WORKDIR /usr/fileshare

RUN mkdir assets
RUN mkdir target
RUN mkdir files
RUN mkdir db
COPY ./.env ./.env
COPY ./filesharing ./filesharing

CMD ["./filesharing"]
