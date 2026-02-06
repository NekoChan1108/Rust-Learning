# 模块系统

```markdown
Rust 有许多功能可以让你管理代码的组织,包括哪些内容可以被公开,哪些内容作为私有部分,
以及程序每个作用域中的名字,这些功能.这有时被称为模块系统 the module system 包括
```

* 包 (Packages): Cargo 的一个功能,它允许你构建、测试和分享 crate
* Crates: 一个模块的树形结构,它形成了库或二进制项目
* 模块 (Modules) 和 use: 允许你控制作用域和路径的私有性
* 路径 (path): 一个命名例如结构体、函数或模块等项的方式

## 包和 crate

```markdown
crate 是一个二进制项或者库,crate root 是一个源文件,Rust 编译器以它为起始点,
并构成你的 crate 的根模块.包 (package) 是提供一系列功能的一个或者多个 crate.
一个包会包含有一个 Cargo.toml 文件,阐述如何去构建这些 crate.一个包中至多只能
包含一个库 crate (library crate) , 包中可以包含任意多个二进制 crate (binary
crate) , 包中至少包含一个 crate,无论是库的还是二进制的.
```

### crate 规则

#### 单个二进制 crate

```markdown
如果包目录中只有 src/main.rs 文件,则 Cargo 会认为这是一个二进制 crate (可执行程序),
并且它的名字与包名相同. src/main.rs 被视为这个二进制 crate 的根文件 (crate root),
Cargo 会将其传递给 rustc 编译器来构建可执行文件,如下文件结构:
my-project/
├── Cargo.toml
└── src/
└── main.rs
在这个例子中包名是 my-project. src/main.rs 是唯一的源文件,因此 Cargo 会构建一个名为
my-project 的二进制可执行文件. 运行命令 cargo build 会在 target/debug 目录下生成可
执行文件 my-project.
```

#### 二进制和库 crate

```markdown
如果包目录中同时存在 src/main.rs 和 src/lib.rs,则 Cargo 会认为这个包包含两个 crate:
一个库 crate (由 src/lib.rs 定义) 和一个二进制 crate (由 src/main.rs 定义).两者的
名字都与包名相同,如下文件结构:
my-project/
├── Cargo.toml
└── src/
├── main.rs
└── lib.rs
在这个例子中 src/lib.rs 定义了一个库 crate,可以通过其他项目依赖使用. src/main.rs 定义了
一个二进制 crate,可以作为可执行程序运行. 运行命令 cargo build 会在 target/debug 目录下
生成两个文件: my-project 和 libmy-project.rlib.
```

#### 多个二进制 crate

```markdown
如果你想在一个包中定义多个二进制 crate,可以将每个二进制 crate 的入口文件放在 src/bin/
目录下.每个 src/bin/ 下的 .rs 文件都会被编译为一个独立的二进制 crate,其名称与文件名相同.
如下文件结构:
my-project/
├── Cargo.toml
└── src/
├── bin/
│ ├── server.rs
│ └── client.rs
└── lib.rs
src/lib.rs 定义了一个库 crate. src/bin/server.rs 和 src/bin/client.rs 分别定义了
两个独立的二进制 crate. server.rs 对应可执行文件 server.client.rs 对应可执行文件 client.
运行命令 cargo build 会在 target/debug 目录下生成四个文件: my-project, server, client,
my-project.rlib . cargo run --bin 后面接要的二进制 crate 的名称,可以运行指定的二进制 crate.
```

### crate 作用域隔离

```markdown
每个 crate 都有自己的作用域,这意味着即使不同的 crate 中有同名的项 (如结构体、函数、特性等),
它们也不会相互干扰.例如,在你的 crate 中定义了一个名为 Rng 的结构体,而 rand crate 也提供了
一个名为 Rng 的特性 (trait).由于作用域隔离,编译器可以明确区分这两者: 你自己的 Rng 结构体可
以直接通过 Rng 访问. rand crate 中的 Rng 特性则需要通过 rand::Rng 来访问.
```

```rust
// 自定义的 Rng 结构体
struct Rng {
    seed: u64,
}

impl Rng {
    fn new(seed: u64) -> Self {
        Rng { seed }
    }
}

// 使用 rand crate 的 Rng 特性
use rand::Rng as RandRng; // 重命名避免冲突

fn main() {
    let mut rng = rand::thread_rng(); // 创建 rand 的随机数生成器
    let random_number: u32 = rng.gen(); // 使用 rand::Rng 特性的 gen 方法
    println!("Random number: {}", random_number);

    let my_rng = Rng::new(42); // 使用自定义的 Rng 结构体
    println!("My RNG seed: {}", my_rng.seed);
}
```