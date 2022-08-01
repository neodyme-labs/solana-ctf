# Setup

To be able to write exploits, you need an appropriate development-environment.
If you have developed Solana contracts on your device before, feel free to just use your existing setup.

Depending on how comfortable you are with your current environment, there are three options:
- Fully provided development: You pull and start a docker container we provide. In there, you find a VS-Code instance, which you can access via the browser. This is the easiest to get going with, but some keybinds might differ from what you are used to.
- Fully local development: You checkout out a [git-repo](https://github.com/neodyme-labs/neodyme-breakpoint-workshop) and go from there. Recommended for experienced devs, more difficult to setup.
- Mixed development: Still get the benefits of a local VS-Code (your settings, all shortcuts), without having to install all dependencies. Also requires some local setup.

Should you have one of the new M1 Macs, this is a bit unfortunate as Solana's rBPF-JIT is not supported there, and we currently have no way of disabling it in our setup.

## Easy Option: Full Setup
We have provided a full docker image with all you need.

1. Install docker
2. `docker run --name breakpoint-workshop -p 2222:22 -p 8080:80 -e PASSWORD="password" neodymelabs/breakpoint-workshop:latest-code-prebuilt`

The container runs a headless instance of VS-Code, setup with rust-analyzer. To access it, go to `http://127.0.0.1:8080` and enter your password (`password`).

The workshop files are located at `/work` and the most basic tools are installed. As this container is Debian based you can install additional tools using `apt`.

If you are using Chrome, the usual VS-Code shortcuts will work. Firefox is a bit more restrictive, and you might have to use the menu instead of some shortcuts.

To get a terminal on the server, you can either use ssh, or simply use VS-Code's build-in terminal (Open with either `` Ctrl+Shift+` ``, or `Menu->View->Terminal`). The workspace is located at `/work`.
To ssh use this command `ssh user@127.0.0.1 -p 2222` and type in the password as before.

Go-to-definition can be done with `Ctrl+Left Mouse Button`, going back with `Ctrl+Alt+Minus`

## Flexible Option: Local Setup

For the local setup, you need to fetch our prepared contracts and exploit-harnesses on [Github](https://github.com/neodyme-labs/neodyme-breakpoint-workshop). In addition, you need an up-to-date version of Rust. Should you wish to render these docs locally, you can do so with mdbook: 

```
cargo install mdbook
mdbook serve
```

If you encounter the error

```
error: failed to download `solana-frozen-abi v1.8.2`
```

or

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/poc-framework-0.1.2/src/lib.rs:522:81
```

then the contract failed to build. This is likely caused by a too old Rust or Solana toolchain. Ensure you have the latest versions by running:

```sh
rustup default stable
rustup update
solana-install init 1.7.17
```



## Third Option: Combined Setup
It is possible to use the container with VS-Code via the [Remote Development](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.vscode-remote-extensionpack) extension. Unfortunately, this extension is only available for the original Microsoft binary builds, and not open-source builds of VSCode.

1. Install docker
2. `docker run --name breakpoint-workshop -p 2222:22 -p 8080:80 -e PASSWORD="password" neodymelabs/breakpoint-workshop:latest-code-prebuilt`
3. Open VS-Code
4. Install the Remote Development extension (`Remote - SSH`), if not installed already
5. Press `Ctrl+Shift+P` to open the command palette
6. Enter `Remote-SSH: Connect to Host...`
7. Enter the user and address of your assigned instance, e.g. `user@127.0.0.1:2222`
8. Enter your password when prompted
9. Open a terminal (on the remote)
10. Terminate the connection and reconnect via VSCode
11. Click `Open Folder` and open the workspace at `/work`
12. The workspace will open. You operate on the same files as you would via the fully-remote setup
13. Install the rust-analyzer extension on the remote

## Compiling the contracts and running the exploits

We provide five contracts and five exploit harnesses, all in the same cargo workspace. As they all use the same dependencies, we can save disk space and compile time that way.
Each contract is in its own crate (`level0 - level4`). For the exploits, we have pre-setup harnesses using our PoC-framework contained in the `pocs` folder, though more on that later.

To make compiling and running the exploits painless, especially on the remote instances, we have provided pre-configured build targets in VS-Code. To compile and run an exploit, you can press `Ctrl+Shift+B` and then select the exploit you are working on.

In the VS-Code based workflow, all contracts are rebuilt automatically whenever you run an exploit. You can also trigger this rebuilding manually by selecting the `build contracts` option in VS-Code's build menu.

If you don't want to use this workflow, you have to rebuild the contracts yourself whenever you change something (for example, introducing logging).

Compiling and running the old-fashioned way via terminal is possible as well. Each exploit complies to its own binary, which you can select via the `--bin` argument for cargo:

```sh
# compile all contracts
cargo build-bpf --workspace

# run level3 exploit
RUST_BACKTRACE=1 cargo run --bin level3
```
