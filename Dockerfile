FROM rust:1.56 as builder
WORKDIR /usr/src/kasaki-discord-bot
COPY . .
RUN cargo install --path .
RUN apt-get update && apt-get install -y ffmpeg && rm -rf /var/lib/apt/lists/*
CMD ["kasaki-discord-bot"]
