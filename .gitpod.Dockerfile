# Our customized docker image uses Gitpod's "workspace-full" image as a base.
FROM gitpod/workspace-full:2023-01-16-03-31-28

# These "RUN" shell commands are run on top of the "workspace-full" image, and
# then committed as a new image which will be used for the next steps.
# In this chunk of "RUN" instructions, we are downloading:
# - The Soroban CLI
# - sccache: a compiler cache that avoids running compiling tasks when possible
# - deno: a JavaScript runtime built in Rust (we use this for the SQ cli)
RUN mkdir -p ~/.local/bin
RUN curl -L -o ~/.local/bin/soroban https://github.com/stellar/soroban-tools/releases/download/v0.6.0/soroban-cli-0.6.0-x86_64-unknown-linux-gnu
RUN chmod +x ~/.local/bin/soroban
RUN curl -L https://github.com/mozilla/sccache/releases/download/v0.3.3/sccache-v0.3.3-x86_64-unknown-linux-musl.tar.gz | tar xz --strip-components 1 -C ~/.local/bin sccache-v0.3.3-x86_64-unknown-linux-musl/sccache
RUN chmod +x ~/.local/bin/sccache

RUN curl -L https://github.com/watchexec/cargo-watch/releases/download/v8.1.2/cargo-watch-v8.1.2-x86_64-unknown-linux-gnu.tar.xz | tar xJ --strip-components 1 -C ~/.local/bin cargo-watch-v8.1.2-x86_64-unknown-linux-gnu/cargo-watch

RUN curl -LO https://github.com/denoland/deno/releases/download/v1.30.1/deno-x86_64-unknown-linux-gnu.zip
RUN unzip deno-x86_64-unknown-linux-gnu.zip -d ~/.local/bin

# These "ENV" instructions set environment variables that will be in the
# environment for all subsequent instructions in the build stage.
ENV RUSTC_WRAPPER=sccache
ENV SCCACHE_CACHE_SIZE=5G
ENV SCCACHE_DIR=/workspace/.sccache

# In order to reliably install the most up-to-date version of Rust, we first
# uninstall the existing toolchain.
# https://github.com/gitpod-io/workspace-images/issues/933#issuecomment-1272616892
RUN rustup self uninstall -y
RUN rm -rf .rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# In this chunk of "RUN" instructions, we are getting our rust environment
# ready and prepared to write some Soroban smart contracts.
RUN rustup install stable
RUN rustup target add --toolchain stable wasm32-unknown-unknown
RUN rustup component add --toolchain stable rust-src
RUN rustup install nightly
RUN rustup target add --toolchain nightly wasm32-unknown-unknown
RUN rustup component add --toolchain nightly rust-src
RUN rustup default stable

RUN cargo install soroban-cli --git https://github.com/tyvdh/soroban-tools --rev 7ff9ac57893ba54f630a8cb3b62d76a2a1a70858

# In this final "RUN" instruction, we are installing a compiler and toolchain
# library for WebAssembly.
RUN sudo apt-get update && sudo apt-get install -y binaryen
