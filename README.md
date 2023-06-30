# Substrate Kitties
Project to learn Substrate by developing custom `kitty` pallet.

### Running Apps
The project contains two starter applications:

  - [Substrate Front End Template](https://github.com/substrate-developer-hub/substrate-front-end-template) (`tutorials/solutions/kitties` branch)
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

## Pallet Development
The quickest way of building a custom pallet is to start with the example [pallets/template](node/pallets/template/).

```bash
cp -rf node/pallets/template node/pallets/kitty
```

Rename the package name in the new pallet's `Cargo.toml`
```diff
[package]
-name = "pallet-template"
+name = "pallet-kitty"
 version = "4.0.0-dev"
 description = "FRAME pallet template for defining custom runtime logic."
 authors = ["Substrate DevHub <https://github.com/substrate-developer-hub>"]
```

Add pallet project to `node` Cargo [workspace](node/Cargo.toml#L1)
```diff
members = [
     "node",
     "pallets/template",
+    "pallets/kitty",
     "runtime",
 ]
```

Build project to confirm that pallet was correctly configured.
```bash
cargo build -p pallet-kitty
```

### Custom Logic
Now that we have a skeleton of our custom pallet we can start adding custom logic to suit our business needs. For our pallet to be usable we need to create dispatchable (i.e. [extrinsic](https://wiki.polkadot.network/docs/learn-extrinsics)) function. To achieve that we need the fulfill the following requirements:

  - **Pallet Config** <br />
    For pallet to be used on the blockchain we need to define the custom parameters and types that our pallet will work with. This is defined in pallet's `lib.rs` file. For example: [KittyRandomness](node/pallets/kitty/src/lib.rs#L69).

  - **Custom Type** <br />
    Define your custom types that represent your custom assets/objects. This is defined in pallet's `lib.rs` file. For example: [Kitty](node/pallets/kitty/src/lib.rs#L41).

  - **Custom Storage** <br />
    Define your custom storage to track the states in your pallet. This is defined in pallet's `lib.rs` file. For example: [CountForKitties](node/pallets/kitty/src/lib.rs#L82).

  - **Custom Events** <br />
    Your pallet will emit custom events for different state transitions. This needs to be defined in `Event<T: Config>` enum. This is defined in pallet's `lib.rs` file. For example: [Created](node/pallets/kitty/src/lib.rs#L104).

  - **Custom Errors** <br />
    Your pallet can run into issues and we would want to throw pallet specific errors. This needs to be defined in `Error<T>` enum. This is defined in pallet's `lib.rs` file. For example: [NoKitty](node/pallets/kitty/src/lib.rs#L123).

  - **Dispatchable functions** <br />
    To allow others on the blockchain to interact with our pallet we need to expose functions that can be called from external source. These functions are called `extrinsic` and is defined inside `impl` block of `Pallet<T>` with `pallet::call` macro. This is defined in pallet's `lib.rs` file. For example: [create_kitty](node/pallets/kitty/src/lib.rs#L144).

  - **Internal helper functions** <br />
    Any function that is used only internally should be defined inside `impl` block of `Pallet<T>`. This is defined in pallet's `lib.rs` file. For example: [gen_dna](node/pallets/kitty/src/lib.rs#L228).


### Add Pallet to Runtime
Once we have completed our pallet development we need to add this pallet in our runtime. This can be done by performing the following steps:

  - Add our pallet as dependency to runtime's [Cargo.toml](node/runtime/Cargo.toml)
  ```diff
pallet-template = { version = "4.0.0-dev", default-features = false, path = "../pallets/template" }
+pallet-kitty = { version = "4.0.0-dev", default-features = false, path = "../pallets/kitty" }
  ```

 - Add additional dependencies (i.e. crates) that is WASM compatible
  ```diff
 pallet-kitty = { version = "4.0.0-dev", default-features = false, path = "../pallets/kitty" }
+pallet-insecure-randomness-collective-flip = { git = "https://github.com/paritytech/substrate", package = "pallet-insecure-randomness-collective-flip", default-features = false, branch = "polkadot-v0.9.42" }
  ```
  _Note: It is important that the `branch` of additional dependency is of the same version as other Substrate crates (e.g. `sp-core`)_

  - Configure concreate types for your pallet. This is defined in pallet's `node/runtime/src/lib.rs` file. For example:
  ```rs
  impl pallet_kitty::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = pallet_kitty::weights::SubstrateWeight<Runtime>;
    type Currency = Balances;
    type MaxKittiesOwned = frame_support::pallet_prelude::ConstU32<100>;
    type KittyRandomness = RandomnessCollectiveFlip;
  }
  ```

  - Add our pallet to  `construct_runtime!` macro in `node/runtime/src/lib.rs`
  ```diff
   TemplateModule: pallet_template,
+  RandomnessCollectiveFlip: pallet_insecure_randomness_collective_flip,
+  SubstrateKitties: pallet_kitty,
  ```

### Launch Blockchain
We can now build and launch the blockchain which includes our custom pallet.

```bash
cd /apps/node
cargo build --release
/tmp/target/release/node-template --dev
```

Interact with our kitty pallet using frontend
```bash
cd /apps/frontend
yarn start
```

Our local blockchain is now accessible at http://localhost:8000

That's it. Happy chaining!
