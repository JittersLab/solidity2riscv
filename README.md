# Solidity2RISC-V 编译器

这是一个将 Solidity 智能合约编译为 RISC-V 汇编代码的编译器项目。

## 当前功能

### 已实现的功能
1. 基本语法解析
   - 函数声明
   - 参数列表解析
   - 返回值类型解析
   - 基本表达式解析（支持加减乘除）

2. 词法分析
   - 支持基本数据类型（uint, int, bool, address, string）
   - 支持运算符（+, -, *, /）
   - 支持标识符和数字字面量

3. 语法分析
   - 函数体解析
   - 变量声明
   - return 语句解析
   - 基本表达式解析

### 待实现功能
1. 中间表示（IR）生成
2. RISC-V 代码生成
3. 更复杂的表达式支持
4. 控制流语句（if, for, while）
5. 合约特性支持
6. 内存管理
7. 错误处理
8. 优化

## 项目结构

```
src/
├── frontend/
│   ├── lexer.rs    # 词法分析器
│   └── parser.rs   # 语法分析器
├── ir/             # 中间表示
├── backend/        # RISC-V 代码生成
└── main.rs         # 主程序入口
```

## 使用方法

```bash
# 编译项目
cargo build

# 运行编译器
./target/debug/solidity2riscv input.sol output.s
```

## 示例

输入 Solidity 代码：
```solidity
function add(uint a, uint b) returns (uint) {
    return a + b;
}
```

输出 RISC-V 汇编代码：
```assembly
# TODO: 待实现
```

## 开发状态

项目目前处于早期开发阶段，正在实现基本的语法解析功能。下一步计划是实现中间表示（IR）的生成。

## 贡献指南

欢迎提交 Issue 和 Pull Request 来帮助改进这个项目。

## 许可证

MIT License
