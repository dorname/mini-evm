# mini-evm

mini-evm 是一个基于 Rust 的简单以太坊虚拟机（EVM）实现，旨在帮助个人学习和理解 EVM 的指令集。本项目主要参考了WTF里面的python实现思路、evm.codes的文档并通过模拟器验证结果。

## 快速开始

本项目实现的所有指令实现提供了单元测试用例，整个执行过程输出相对友好的日志，帮助个人去理解evm执行一个合约代码的过程

```rust
cargo test 单元测试函数的名称 
```

注意:也可以直接在vscode上调试

## 版本

| **Version** | **Release Date** | **Major Features** | **Compatibility** |
| --- | --- | --- | --- |
| 0.1.0 | 2024-11-02 | Initial implementation of basic EVM instructions | Rust 1.79 |

## 其他

这个项目目前仍属我个人的rust练手项目，除了帮助我学习evm的指令集之外，也在帮助我熟悉rust，但很可惜目前的代码风格实际与rust的设计理念和风格还有不少差距，并且也有很多可以根据rust所有权、生命周期特性优化的地方，所以后续大概率是会有一个重构优化的过程，欢迎交流和提建议。

## 参考文档

[1] WTF [https://www.wtf.academy/docs/evm-opcodes-101/](https://www.wtf.academy/docs/evm-opcodes-101/)

[2] evm [https://www.evm.codes/](https://www.evm.codes/)
