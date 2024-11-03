# mini-evm

`mini-evm` is a simple Ethereum Virtual Machine (EVM) implementation in Rust, designed to assist individuals in learning and understanding the EVM instruction set. This project primarily draws inspiration from the Python implementation in WTF Academy and utilizes the explanations and verification results from evm.codes.

## quick start

The project includes unit tests for all implemented instructions, providing user-friendly logs throughout the execution process to aid in comprehending how the EVM executes contract code.

To run a specific unit test, use:

```rust
cargo test <name-of-the-unit-test> 
```

Alternatively, you can debug directly within Visual Studio Code.

## version

| **Version** | **Release Date** | **Major Features** | **Compatibility** |
| --- | --- | --- | --- |
| 0.1.0 | 2024-11-02 | Initial implementation of basic EVM instructions | Rust 1.79 |

## Other

This project currently serves as a personal exercise in Rust, aiming to deepen my understanding of the EVM instruction set and familiarize myself with Rust. However, the current code style deviates from Rust's design principles and conventions. There are numerous areas that could be optimized by leveraging Rust's ownership and lifetime features. Therefore, a significant refactor and optimization process is likely in the future.

## References

[1] WTF [https://www.wtf.academy/docs/evm-opcodes-101/](https://www.wtf.academy/docs/evm-opcodes-101/)

[2] evm [https://www.evm.codes/](https://www.evm.codes/)