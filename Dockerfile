FROM rust:1.70.0

RUN apt-get update \
  && apt-get install -y protobuf-compiler clang

# Add WebAssembly (wasm) targets
RUN rustup target add wasm32-unknown-unknown

# Node setup
ENV NVM_DIR /nvm
ENV NODE_VERSION 19

RUN mkdir -p /nvm \
    && curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash \
    && . $NVM_DIR/nvm.sh \
    && nvm install $NODE_VERSION \
    && nvm alias default $NODE_VERSION \
    && nvm use default \
    && npm install --global yarn

ENV NODE_PATH $NVM_DIR/v$NODE_VERSION/lib/node_modules
ENV PATH      $NVM_DIR/v$NODE_VERSION/bin:$PATH
