FROM debian:12.2

RUN apt update && \
    apt-get install -y git llvm clang libclang-dev gcc-arm-none-eabi gdb-arm-none-eabi libc6-dev curl wget python-is-python3 make

RUN bash -c "curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable"

RUN ls -al /root/.cargo/bin

RUN /root/.cargo/bin/rustup target install thumbv8m.main-none-eabi
RUN /root/.cargo/bin/cargo install flip-link
RUN /root/.cargo/bin/cargo install cargo-binutils
RUN /root/.cargo/bin/rustup component add llvm-tools-preview







