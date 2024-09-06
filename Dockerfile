FROM rust:bullseye
LABEL authors="sten"

EXPOSE 8080

COPY . .

ENTRYPOINT ["cargo", "run"]