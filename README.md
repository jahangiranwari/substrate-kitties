# Substrate Learning
Project to learn Substrate

### Running Apps
The project contains two starter applications:

  - [Substrate Front End Template](https://github.com/substrate-developer-hub/substrate-front-end-template)
  - [Substrate Node Template](https://github.com/substrate-developer-hub/substrate-node-template)

To run the project you can either manually run `docker-compose up` or take advantage of VS Code dev containers as shown in [rust-devcontainer](https://github.com/jahangiranwari/rust-devcontainer).

### Substrate Node
Once the container is running you can start the Substrate node by running the following:

```bash
cd /apps/node
cargo build --release
/tmp/target/release/node-template --dev
```

### Substrate Front End Template
To connect to the Substrate node backend you can run the Substrate frontend app that uses Polkadot JS:

```bash
cd /apps/frontend
yarn install
yarn start
```




