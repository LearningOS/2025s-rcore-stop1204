# 2025s-rcore Lab1

## 一、前言

作为业余爱好者，因兴趣参与 rCore 学习，期间尝试解决各种新奇问题。

## 二、学习

### Day 1 (26/3/2025)

完成 rustlings，但跳过 algorithm 5 - 10 部分（后续计划补充学习），正式开启 OS 前置内容学习。对博客使用尚不熟悉，发布后不知链接页面所在。

### Day 2 (27/3/2025)

**环境转移与配置**：将博客转移至 notion，完成后统一更新。

**安装工具**：在 Macos 系统下，通过`brew install qemu tmux`安装 qemu 模拟器和 tmux GDB 调试器。从[https://github.com/rustsbi/rustsbi-qemu/releases](https://github.com/rustsbi/rustsbi-qemu/releases)下载最新的`rustsbi-qemu.bin`，替换`rCore-Tutorial-Code-2025S.git`的`bootloader/`目录下同名文件。

### Day 3 (28/3/2025)

**编译裸机程序**

查看默认平台：`rustc --version --verbose`，得到`host: aarch64-apple-darwin`。

查找 riscv 相关目标平台：`rustc --print target-list | grep riscv`，获取如`riscv64gc-unknown-none-elf`等平台信息。

添加目标平台：`rustup target add riscv64gc-unknown-none-elf`。

编译到目标平台：`cargo run --target riscv64gc-unknown-none-elf`。

**分析裸机程序**

查看文件格式：`file target/riscv64gc-unknown-none-elf/debug/os`，得知为`ELF 64-bit LSB executable`。

安装文件头信息工具：`rustup component add llvm-tools-preview`。

查看文件头：`rust-readobj -h target/riscv64gc-unknown-none-elf/debug/os`。

反汇编：`rust-objdump -S target/riscv64gc-unknown-none-elf/debug/os`。发现程序入口地址`entry: 0x0`，反汇编未生成汇编代码，确认是移除`main`函数导致的空程序。

**修改入口点并重新反汇编**

提供`_start`入口，代码如下：



```
\#!\[no\_std] &#x20;

\#!\[no\_main]&#x20;

\#\[\*\*unsafe\*\*(no\_mangle)]&#x20;

extern "C" fn \_start(){

&#x20;   loop{};

}
```

重新反汇编后得到`Entry: 0x11158`及相应汇编代码。

### Day 4 (29/3/2025)

遇到众多兼容性问题，发现[https://learningos.cn/rCore-Tutorial-Guide-2025S/index.html](https://learningos.cn/rCore-Tutorial-Guide-2025S/index.html)文档信息不完整，转而参考[https://learningos.cn/rCore-Tutorial-Book-v3/chapter0/5setup-devel-env.html](https://learningos.cn/rCore-Tutorial-Book-v3/chapter0/5setup-devel-env.html)。重新编译的程序无法运行，原因是入口地址不符合 RustSBI 约定的`0x80200000`，需通过 cargo 配置连接脚本`src/linker.ld`。因配置问题不明，选择休息一天。

### Day 5 (30/3/2025)

参考[Fontlos 大神的文章](https://fontlos.com/post/rCore-01)成功完成配置。

正确设置`LOG=TRACE`：`make run LOG=TRACE`。

修正`boot_stack_low_bound`：实际应为`boot_stack`（依据`entry.asm`）。

调整`make docker`命令：设置为`docker start`和`docker attach`，避免每次创建新容器。

### Day 6 (31/3/2025)

参考[https://fontlos.com/post/rCore-02](https://fontlos.com/post/rCore-02)，使用 IDE 的 Docker。了解到 RISC-V 寄存器编号相关知识，如`x10~x17`对应`a0~a7`，`x1`对应`ra`。

系统调用功能接口整理如下：



| 编号 | 系统调用                   | 所在章节 | 功能描述               |
| -- | ---------------------- | ---- | ------------------ |
| 1  | sys\_exit              | 2    | 结束执行               |
| 2  | sys\_write             | 2/6  | (2) 输出字符串 /(6) 写文件 |
| 3  | sys\_yield             | 3    | 暂时放弃执行             |
| 4  | sys\_get\_time         | 3    | 获取当前时间             |
| 5  | sys\_getpid            | 5    | 获取进程 id            |
| 6  | sys\_fork              | 5    | 创建子进程              |
| 7  | sys\_exec              | 5    | 执行新程序              |
| 8  | sys\_waitpid           | 5    | 等待子进程结束            |
| 9  | sys\_read              | 5/6  | (5) 读取字符串 /(6) 读文件 |
| 10 | sys\_open              | 6    | 打开 / 创建文件          |
| 11 | sys\_close             | 6    | 关闭文件               |
| 12 | sys\_dup               | 7    | 复制文件描述符            |
| 13 | sys\_pipe              | 7    | 创建管道               |
| 14 | sys\_kill              | 7    | 发送信号给某进程           |
| 15 | sys\_sigaction         | 7    | 设立信号处理例程           |
| 16 | sys\_sigprocmask       | 7    | 设置要阻止的信号           |
| 17 | sys\_sigreturn         | 7    | 从信号处理例程返回          |
| 18 | sys\_sleep             | 8    | 进程休眠一段时间           |
| 19 | sys\_thread\_create    | 8    | 创建线程               |
| 20 | sys\_gettid            | 8    | 获取线程 id            |
| 21 | sys\_waittid           | 8    | 等待线程结束             |
| 22 | sys\_mutex\_create     | 8    | 创建锁                |
| 23 | sys\_mutex\_lock       | 8    | 获取锁                |
| 24 | sys\_mutex\_unlock     | 8    | 释放锁                |
| 25 | sys\_semaphore\_create | 8    | 创建信号量              |
| 26 | sys\_semaphore\_up     | 8    | 减少信号量的计数           |
| 27 | sys\_semaphore\_down   | 8    | 增加信号量的计数           |
| 28 | sys\_condvar\_create   | 8    | 创建条件变量             |
| 29 | sys\_condvar\_signal   | 8    | 唤醒阻塞在条件变量上的线程      |
| 30 | sys\_condvar\_wait     | 8    | 阻塞与此条件变量关联的当前线程    |

### Day 7-10 (1~4/4/2025)

在其他同学的帮助下完成 ch3 的学习和提交