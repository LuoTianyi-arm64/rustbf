# rustbf

一个用 Rust 编写的简单 Brainfuck 解释器与库（CLI + lib），适合学习、嵌入或轻量测试 Brainfuck 程序。

## 用途（What this is）
rustbf 用最小的代码实现了一个可执行的 Brainfuck 解释器，同时把解释器实现放在 `src/lib.rs` 中，方便你：
- 直接使用二进制从文件运行 Brainfuck 程序（CLI）。
- 在其他 Rust 程序中以库的形式复用解释器函数 `run_bf`。

## 技术栈（Stack）
- Language(s): Rust
- Edition: 2024
- 主要实现：原生标准库（无额外依赖）

## 仓库结构（How it's organized）
```
Cargo.toml      # cargo package 配置
Cargo.lock
src/
  main.rs       # CLI：从文件读取 BF 源码并调用 run_bf
  lib.rs        # 解释器实现：pub fn run_bf(src: &str) -> Result<Vec<char>, String>
```

How it fits together:
- `main.rs` 作为入口：读取命令行参数（Brainfuck 源文件路径），加载文件内容并调用 `run_bf`，把返回的字符逐个打印到 stdout。
- `lib.rs` 提供纯 Rust 的解释器实现，暴露 `run_bf` 供复用。

## 特性与行为摘要
- 内存模型：使用 Vec<u8>，初始包含 1 个字节（值 0）。指针从 0 开始，向右增长时会自动扩容；向左移动到负值时会返回 Err（错误消息为 "the index is negative."）。
- 算术：使用 wrapping_add / wrapping_sub（按 u8 溢出回绕）。
- I/O：`,` 操作从标准输入读取字节（阻塞），`.` 操作将当前单元作为 char 推入输出向量（最终 `run_bf` 在返回前会追加一个换行 '\n'）。
- 循环 `[]` 的实现：通过查找匹配括号来跳转。实现较为简单：遇到不匹配的括号时不会返回明确错误，而是尽可能跳过或回退，可能导致非预期行为。

## 快速开始（How to run it）
编译并运行 CLI（从仓库根目录）：

```bash
# 构建并运行（传入一个 .bf 源文件路径）
cargo run -- path/to/program.bf

# 或者用 release 模式
cargo build --release
./target/release/rustbf path/to/program.bf
```

示例：一个简单的 Brainfuck 程序（打印 'A'）
```
++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.
```
将上面的程序保存为 hello.bf，然后运行：

```bash
cargo run -- hello.bf
```

如果程序需要输入（使用 `,`），可以在运行时从键盘输入，或用 shell 重定向/管道提供输入：

```bash
# 将文件 input.bin 的内容作为程序 stdin
cargo run -- hello_with_input.bf < input.bin
```

## 作为库复用（二次使用 lib.rs）
如果你想在别的 Rust 项目中直接调用解释器：

方法 A - 本地路径依赖（本地开发）
在你的项目的 Cargo.toml 中添加：

```toml
[dependencies]
rustbf = { path = "../rustbf" }
```

然后在代码中：

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
- 如果你将仓库发布到 crates.io，可以像常规依赖一样添加并使用。

注意（重要）:
- `run_bf` 函数内部直接从标准输入读取用于 `,` 指令的输入；当你在库中调用 `run_bf` 时，它仍会读取进程的 stdin（这可能导致阻塞或与上层应用的输入流冲突）。如果你需要更可控的接口，建议修改 `lib.rs`，将输入源抽象为参数（例如传入一个实现 `Read` 的类型），或添加一个带输入切片/字符串的变体函数。

## run_bf API 说明
```rust
pub fn run_bf(src: &str) -> Result<Vec<char>, String>
```
- 参数：`src` - 含有 Brainfuck 源代码的字符串切片（任意非 BF 字符会被忽略，因为代码只匹配 BF 的八个指令字符）。
- 返回：
  - Ok(Vec<char>) - 解释器运行结束后产生的输出字符序列，函数会在末尾追加一个换行字符 '\n'。
  - Err(String) - 目前仅在指针尝试移动到负值时返回错误（消息为 "the index is negative."）。其他错误情况（如括号不匹配）不会以 Err 返回，行为可能是提前结束或跳出循环。

示例：将输出转换为字符串
```rust
let out = run_bf(">+.+.").unwrap();
let s: String = out.into_iter().collect();
println!("输出: {}", s);
```

## 限制与改进建议
- 输入抽象：当前 `,` 从 global stdin 读取，无法在单元测试或程序化调用中轻易注入输入。建议将 `run_bf` 改成接收一个实现 `Read` 的输入参数（或返回器/回调）。
- 错误处理：增加对未匹配括号的明确定义和错误返回会更好；目前实现尝试跳转并在边界时中止循环，但不返回错误信息。
- 性能：当前实现用 Vec<u8> 做内存并在需要时扩容；对于大程序或需要更高性能的场景可以优化为固定大小数组或更高效的跳转表（预计算括号匹配索引）。

## 贡献与许可
欢迎 issue / PR。仓库当前没有明确的 LICENSE，请在需要时补充开源许可证文件（例如 MIT / Apache-2.0）。

---
如果你希望，我可以：
- 把这份 README 提交到仓库（我已经为你创建了 README.md）。
- 将 `run_bf` 改造成接收自定义输入源的版本（示例 patch）。

