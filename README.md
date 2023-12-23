# Connecting the world! 🌍📩

![logo](logo.png)

Zhoda aims to maintain simplicity and efficiency in its TCP implementation, while also providing a high level of customizability. Cross-platform support is also a priority, with binaries available for Linux, Windows and MacOS.

## Installation 🛠️

- Clone the repository:

```shell
git clone https://github.com/dy0gu/Zhoda.git
```

- Install the dependencies:

```shell
cargo install --path client
cargo install --path server
```

## Usage 🔷

- Run the server:

```shell
cargo run --bin zhodas
```

- Run the client:

```shell
cargo run --bin zhodac <address>
```

- For more options use the `--help` flag:

```shell
cargo run --bin zhodas -- --help
cargo run --bin zhodac -- --help
```
