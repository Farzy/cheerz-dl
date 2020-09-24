FROM rust:1.46 as builder
WORKDIR /usr/src/cheerz-dl
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/cheerz-dl /usr/local/bin/cheerz-dl
ENTRYPOINT ["cheerz-dl"]
CMD ["--help"]
