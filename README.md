# `profligate`

`profligate` is a Caesar cipher library I'm working on as part of my university coursework.

## Features

- encryption and decryption with a manually specified key
- automatic decryption based on known plaintext
- automatic decryption based on English letter frequency tables and dictionary
- [WebAssembly bindings](https://gitlab.com/oreganoli/profligate-front-rs/)
- [React frontend](https://gitlab.com/oreganoli/profligate-front/)

## Not supported
- arbitrary UTF-8 input text
- alphabets other than English Latin

## Live example

The React frontend is available at https://zen-khorana-764c54.netlify.app/.

## Building and testing

Install the Rust toolchain for your platform and use

```
cargo build
```

```
cargo test
```
