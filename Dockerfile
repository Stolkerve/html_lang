FROM alpine:3.21
RUN apk add --no-cache rust cargo
COPY . /build
WORKDIR /build
RUN ["cargo", "build", "--release"]
EXPOSE 8080
ENTRYPOINT ["cargo", "run", "--release"]
