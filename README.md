# rustbf

一个用 Rust 编写的简单 Brainfuck 解释器与库（CLI + lib），适合学习、嵌入或轻量测试 Brainfuck 程序。

## 用途（What this is）

rustbf 用尽量精简的代码实现了一个可执行的 Brainfuck 解释器，并将解释器核心放在 `src/lib.rs` 以便复用。你可以：

- 直接使用仓库提供的二进制（CLI）从文件运行 Brainfuck 程序。
- 在其它 Rust 程序中以库的形式调用 `run_bf`。

## 技术栈（Stack）

- 语言：Rust
- Edition：2024
- 主要实现：仅使用标准库（无额外依赖）

## 仓库结构（How it's organized）

```
Cargo.toml      # cargo package 配置
Cargo.lock
src/
  main.rs       # CLI：从文件读取 BF 源码并调用 run_bf
  lib.rs        # 解释器实现：pub fn run_bf(src: &str) -> Result<Vec<char>, String>
```

How it fits together:

- `main.rs` 作为命令行入口：解析命令行参数（Brainfuck 源文件路径），读取文件内容并调用 `run_bf`，然后把返回的字符输出到 stdout。
- `lib.rs` 提供了解释器实现并导出 `run_bf`，便于在其他程序中直接复用。

## 特性与行为摘要

- 内存模型：使用 `Vec<u8>`，初始包含 1 个字节（值 0）。数据指针从 0 开始，向右移动时会按需扩容；向左移出边界会返回 Err（错误信息为 "the index is negative."）。
- 算术：对单元使用 `wrapping_add` / `wrapping_sub`，按 `u8` 溢出回绕（wrap-around）。
- I/O：`,` 指令从标准输入读取字节（阻塞），`.` 指令把当前单元按为 `char` 推入输出向量；`run_bf` 在返回前会在输出末尾追加一个换行符 `'\n'`。
- 循环 `[]`：通过查找匹配括号实现跳转。当前实现为简单策略：遇到未匹配的括号时不会始终返回明确错误，行为尽量容错但可能导致非预期行为。

## 快速开始（How to run it）

在仓库根目录编译并运行 CLI：

```bash
# 直接用 cargo 运行（传入一个 .bf 源文件路径）
cargo run -- path/to/program.bf

# 或者构建 release 二进制后运行
cargo build --release
./target/release/rustbf path/to/program.bf
```

示例：一个常用的 Brainfuck 程序（打印 "Hello, World!" 风格的字符串，或可改成打印单个字符）：

```
++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.
```

将上面的程序保存为 `hello.bf`，然后运行：

```bash
cargo run -- hello.bf
```

如果程序需要输入（使用 `,`），可以在运行时从键盘输入，或者用 shell 重定向/管道提供输入：

```bash
# 将文件 input.bin 的内容作为程序 stdin
cargo run -- hello_with_input.bf < input.bin
```

## 作为库复用（二次使用 lib.rs）

如果你想在其他 Rust 项目中直接调用解释器：

方法 A - 本地路径依赖（本地开发）

在你的项目的 `Cargo.toml` 中添加：

```toml
[dependencies]
rustbf = { path = "../rustbf" }
```

在代码中使用：

```rust
use rustbf::run_bf;

fn main() {
    let program = ">+++."; // 一个简单的 BF 程序
    match run_bf(program) {
        Ok(output_chars) => {
            let s: String = output_chars.into_iter().collect();
            print!("{}", s);
        }
        Err(e) => eprintln!("运行时错误: {}", e),
    }
}
```

方法 B - 作为 Git / crates.io 依赖（如果你发布到 crates.io）

- 如果你将仓库发布到 crates.io，可以像常规依赖一样添加并使用该包。

注意（重要）：

- 目前 `run_bf` 在遇到 `,` 指令时直接从全局标准输入读取，这会在库被调用时造成阻塞或与上层程序的输入管理冲突。建议在需要可测试或可注入输入源的场景中，修改 `run_bf` 接口以接受一个实现了 `Read` 的输入来源。

## run_bf API 说明

```rust
pub fn run_bf(src: &str) -> Result<Vec<char>, String>
```

- 参数：`src` - 包含 Brainfuck 源代码的字符串切片（解释器会忽略非 BF 指令字符）。
- 返回值：
  - `Ok(Vec<char>)` - 运行结束时产生的输出字符序列（函数会在末尾追加一个换行符 `\n`）。
  - `Err(String)` - 目前仅在数据指针尝试移动到负值时返回错误（消息为 `"the index is negative."`）。其他错误情况（例如括号不匹配）不会总是以 `Err` 返回，行为可能是尽量容错或提前终止。

示例：把输出转换为字符串

```rust
let out = run_bf(">+.+.").unwrap();
let s: String = out.into_iter().collect();
println!("输出: {}", s);
```

## 限制与改进建议

- 输入抽象：当前 `,` 从全局 stdin 读取，无法在单元测试或程序化调用中方便地注入输入。建议将 `run_bf` 改为接受一个实现 `Read` 的输入参数，或提供一个可配置的输入回调。
- 错误处理：为未匹配的 `[` / `]` 提供明确的错误返回会更稳健；当前实现尝试跳转并在边界时中止，但不一定返回清晰的错误信息。
- 性能：当前实现使用 `Vec<u8>` 并按需扩容；对于大型程序或性能敏感场景，可以考虑使用固定大小的内存缓冲区或在解析阶段预计算括号匹配表以加速跳转。

## 贡献与许可

欢迎提交 issue / PR。仓库当前没有包含 LICENSE 文件；如需明确开源许可，请添加合适的许可证（例如 MIT 或 Apache-2.0）。

---

我已根据你要求的原结构重写了 README 内容，并将其提交到仓库的 README.md 文件中。若需要，我可以：

- 将 README 的修改直接提交到仓库（已完成）。
- 给 `run_bf` 添加一个接收自定义输入源的替代实现并提交示例补丁。
