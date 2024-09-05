# Dockerfile with multi-stage builds for efficient dependency caching and lightweight final image.
# For more on Docker stages, visit: https://docs.docker.com/build/building/multi-stage/

# We use Cargo Chef to compile dependencies before compiling the rest of the crates.
# This approach ensures proper Docker caching, where dependency layers are cached until a dependency changes.
# Code changes in our crates won't affect these cached layers, making the build process more efficient.
# More info on Cargo Chef: https://github.com/LukeMathWalker/cargo-chef

# We start by creating a base image using 'clux/muslrust' with additional required tools.
FROM clux/muslrust:1.80.0-stable AS chef
WORKDIR /app
COPY . .
COPY . /app
RUN apt update && apt install -y clang unzip
RUN cargo install cargo-chef
ENV PROTOC_VERSION=25.1
RUN curl -L "https://github.com/protocolbuffers/protobuf/releases/download/v$PROTOC_VERSION/protoc-$PROTOC_VERSION-linux-x86_64.zip" -o protoc.zip && unzip ./protoc.zip -d $HOME/.local &&  rm ./protoc.zip
ENV PROTOC=/root/.local/bin/protoc
# Add the x86_64-unknown-linux-musl target to rustup for compiling statically linked binaries.
# This enables the creation of fully self-contained binaries that do not depend on the system's dynamic libraries,
# resulting in more portable executables that can run on any Linux distribution.
RUN rustup target add x86_64-unknown-linux-musl

# Install dependencies
RUN apt update -y && apt install -y lsb-release \
    wget \
    curl \
    git \
    build-essential \
    libclang-dev \
    libz-dev \
    libzstd-dev \
    libssl-dev \
    pkg-config \
    gnupg


# Install LLVM 18
RUN echo "deb http://apt.llvm.org/jammy/ llvm-toolchain-jammy-18 main" > /etc/apt/sources.list.d/llvm-18.list
RUN echo "deb-src http://apt.llvm.org/jammy/ llvm-toolchain-jammy-18 main" >> /etc/apt/sources.list.d/llvm-18.list
RUN wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -

RUN apt update -y && apt install -y --ignore-missing --allow-downgrades \
    libmlir-18-dev \
    libpolly-18-dev \
    llvm-18-dev \
    mlir-18-tools
ENV MLIR_SYS_180_PREFIX=/usr/lib/llvm-18/
ENV LLVM_SYS_181_PREFIX=/usr/lib/llvm-18/
ENV TABLEGEN_180_PREFIX=/usr/lib/llvm-18/

RUN echo "------------a----------------"

RUN apt-get install -y python3-pip
RUN pip3 install ziglang

RUN cargo install --locked cargo-zigbuild

RUN echo "------------b----------------"

RUN RUST_BACKTRACE=debug RUSTFLAGS='-C codegen-units=1' cargo zigbuild --target x86_64-unknown-linux-musl --release --package papyrus_base_layer --verbose 2>&1 | tee out.log

# RUN CARGO_PROFILE_RELEASE_BUILD_OVERRIDE_DEBUG=true RUST_BACKTRACE=1 cargo build --target x86_64-unknown-linux-musl --release --package papyrus_base_layer --locked --verbose

RUN echo "------------c----------------"

# RUN CARGO_PROFILE_RELEASE_BUILD_OVERRIDE_DEBUG=true RUST_BACKTRACE=1 cargo build --target x86_64-unknown-linux-musl --release --package papyrus_node --locked --verbose

ENTRYPOINT ["tail", "-f", "/dev/null"]

# #####################
# # Stage 1 (planer): #
# #####################
#
# FROM chef AS planner
# COPY . .
#
# # * Running Cargo Chef prepare that will generate recipe.json which will be used in the next stage.
# RUN cargo chef prepare
#
# #####################
# # Stage 2 (cacher): #
# #####################
# # Compile all the dependecies using Cargo Chef cook.
# FROM chef AS cacher
#
#
# COPY --from=chef /usr/lib/llvm-18/ /usr/lib/llvm-18/
# ENV MLIR_SYS_180_PREFIX=/usr/lib/llvm-18/
# ENV LLVM_SYS_181_PREFIX=/usr/lib/llvm-18/
# ENV TABLEGEN_180_PREFIX=/usr/lib/llvm-18/
#
# # Copy recipe.json from planner stage
# COPY --from=planner /app/recipe.json recipe.json
#
# ENTRYPOINT ["tail", "-f", "/dev/null"]
#
#
#
# # Build dependencies - this is the caching Docker layer!
# RUN cargo chef cook --target x86_64-unknown-linux-musl --release --package papyrus_node
#
# ######################
# # Stage 3 (builder): #
# ######################
# FROM chef AS builder
# COPY . .
# COPY --from=cacher /app/target target
# # Disable incremental compilation for a cleaner build.
# ENV CARGO_INCREMENTAL=0
#
# # Compile the papyrus_node crate for the x86_64-unknown-linux-musl target in release mode, ensuring dependencies are locked.
# RUN cargo build --target x86_64-unknown-linux-musl --release --package papyrus_node --locked
#
# ###########################
# # Stage 4 (papyrus_node): #
# ###########################
# # Uses Alpine Linux to run a lightweight and secure container.
# FROM alpine:3.17.0 AS papyrus_node
# ENV ID=1000
# WORKDIR /app
#
# # Copy the node executable and its configuration.
# COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/papyrus_node /app/target/release/papyrus_node
# COPY config config
#
# COPY --from=chef /usr/lib/llvm-18/ /usr/lib/llvm-18/
# ENV MLIR_SYS_180_PREFIX=/usr/lib/llvm-18/
# ENV LLVM_SYS_181_PREFIX=/usr/lib/llvm-18/
# ENV TABLEGEN_180_PREFIX=/usr/lib/llvm-18/
#
# # Install tini, a lightweight init system, to call our executable.
# RUN set -ex; \
#     apk update; \
#     apk add --no-cache tini; \
#     mkdir data
#
# # Create a new user "papyrus".
# RUN set -ex; \
#     addgroup --gid ${ID} papyrus; \
#     adduser --ingroup $(getent group ${ID} | cut -d: -f1) --uid ${ID} --gecos "" --disabled-password --home /app papyrus; \
#     chown -R papyrus:papyrus /app
#
# # Expose RPC and monitoring ports.
# EXPOSE 8080 8081
#
# # Switch to the new user.
# USER ${ID}
#
# # Set the entrypoint to use tini to manage the process.
# ENTRYPOINT ["/sbin/tini", "--", "/app/target/release/papyrus_node"]
