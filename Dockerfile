## Build demo applications for BL602
FROM rust:1.97-trixie AS builder-bl

# Install riscv32imafc target
RUN rustup target add riscv32imafc-unknown-none-elf

# Copy HAL and PAC
COPY bl602-pac /build/bl602-pac
COPY bl602-hal-suas /build/bl602-hal-suas

# Copy source code for demo apps
COPY bare-metal /build/bare-metal

# Run build
WORKDIR /build/bare-metal
RUN cargo build --release

## Build the statistical-testing BL602 app (same container)
# Copy source code for BL602 app
COPY statistical-testing-bl602 /build/app-bl602

# Run build
WORKDIR /build/app-bl602
RUN cargo build --release

## Build the statistical testing desktop app
FROM rust:1.97-trixie AS builder-desktop

# Install dependencies for udev access
RUN apt-get update
RUN apt-get -y install libudev-dev pkg-config

# Copy source code for the desktop app
COPY nistrs /build/nistrs
COPY statistical-testing-desktop /build/app-desktop

# Run build
WORKDIR /build/app-desktop
RUN cargo build --release

## Build modified scdrand
FROM debian:trixie AS builder-scdrand

# Install dependencies
RUN apt-get update
RUN apt-get install -y autoconf automake libtool make pkg-config
RUN apt-get install -y libgcrypt20 libgcrypt20-dev libassuan9 libassuan-dev libgpg-error0 libgpg-error-dev scdaemon

# Copy source code
COPY scdtools /build/scdtools

# Run build
WORKDIR /build/scdtools
RUN autoreconf -i
RUN ./configure
RUN make -j8

## Get output
FROM scratch

# Demo apps
COPY --from=builder-bl /build/bare-metal/target/riscv32imafc-unknown-none-elf/release/blinky /blinky
COPY --from=builder-bl /build/bare-metal/target/riscv32imafc-unknown-none-elf/release/button /button
COPY --from=builder-bl /build/bare-metal/target/riscv32imafc-unknown-none-elf/release/random /random

# Statistical testing apps
COPY --from=builder-bl /build/app-bl602/target/riscv32imafc-unknown-none-elf/release/sts /statistical-testing-bl602
COPY --from=builder-desktop /build/app-desktop/target/release/statistical-testing-desktop /statistical-testing-desktop

# scdrand
COPY --from=builder-scdrand /build/scdtools/src/scdrand /scdrand