FROM rust:1.44-slim-buster AS base
RUN rustup update

WORKDIR /usr/src/pathfinding
COPY ./Cargo.toml .
COPY ./src ./src

FROM base AS x86_64_linux
RUN rustup target add x86_64-unknown-linux-gnu
RUN cargo build --target=x86_64-unknown-linux-gnu --release --verbose --all

FROM base AS armv7l_linux
RUN apt-get update && apt-get install -y gcc-arm-linux-gnueabihf 
RUN rustup target add arm-unknown-linux-gnueabihf
RUN mkdir .cargo && echo '[target.arm-unknown-linux-gnueabihf]\nlinker = "arm-linux-gnueabihf-gcc"' > .cargo/config
RUN cargo build --target=arm-unknown-linux-gnueabihf --release --verbose --all

FROM base AS x86_64_windows
RUN apt-get update && apt-get install -y mingw-w64 
RUN rustup target add x86_64-pc-windows-gnu
RUN mkdir .cargo && echo '[target.x86_64-pc-windows-gnu]\nlinker = "x86_64-w64-mingw32-gcc"\nar = "x86_64-w64-mingw32-gcc-ar"' > .cargo/config
RUN cargo build --target=x86_64-pc-windows-gnu --release --verbose --all

FROM base AS x86_64_osx

RUN apt-get update && apt-get install -y clang gcc g++ zlib1g-dev libmpc-dev libmpfr-dev libgmp-dev wget cmake git  patch python libssl-dev lzma-dev libxml2-dev bash 

RUN git clone --depth=1 https://github.com/tpoechtrager/osxcross
RUN wget -nc https://s3.dockerproject.org/darwin/v2/MacOSX10.10.sdk.tar.xz
RUN mv MacOSX10.10.sdk.tar.xz osxcross/tarballs/
RUN UNATTENDED=yes OSX_VERSION_MIN=10.7 ./osxcross/build.sh

RUN rustup target add x86_64-apple-darwin

RUN mkdir .cargo && echo '[target.x86_64-apple-darwin]\nlinker = "x86_64-apple-darwin14-clang"\nar = "x86_64-apple-darwin14-ar"' > .cargo/config
RUN PATH=$PATH:/usr/src/pathfinding/osxcross/target/bin cargo build --target=x86_64-apple-darwin --release --verbose --all

FROM alpine AS artifact
WORKDIR /artifact
COPY --from=x86_64_linux /usr/src/pathfinding/target/x86_64-unknown-linux-gnu/release/libpathfinder_v2.so ./pathfinder_x86_64_linux.so
COPY --from=armv7l_linux /usr/src/pathfinding/target/arm-unknown-linux-gnueabihf/release/libpathfinder_v2.so ./pathfinder_armv7l_linux.so
COPY --from=x86_64_windows /usr/src/pathfinding/target/x86_64-pc-windows-gnu/release/pathfinder_v2.dll ./pathfinder_x86_64_windows.dll
COPY --from=x86_64_osx /usr/src/pathfinding/target/x86_64-apple-darwin/release/libpathfinder_v2.dylib ./pathfinder_x86_64_osx.dylib
