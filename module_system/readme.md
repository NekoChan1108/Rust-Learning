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

## 定义模块来控制作用域与私有性

```markdown
模块让我们可以将一个 crate 中的代码进行分组,以提高可读性与重用性.模块还可以控制项的私有性,
即项是可以被外部代码使用的 (public) ,还是作为一个内部实现的内容,不能被外部代码使用 (private).
可以采用 cargo new --lib restaurant 创建一个库 crate. 用关键字 mod 定义一个模块,指定模块的名字
,并用大括号包围模块的主体,可以在模块中包含其他模块,就像本示例中的 hosting 和 serving 模块.模块中也
可以包含其他项,比如结构体、枚举、常量、trait或者像一样包含函数.之前提到,src/main.rs 和 src/lib.rs
被称为 crate 根.如此称呼的原因是,这两个文件中任意一个的内容会构成名为 crate 的模块,且该模块被称为位于
crate 的模块树的模块结构的根部 ("at the root of the crate’s module structure"),简单来说就是把代
码解耦,提升可读性.
```

## 路径用于引用模块树中的项

```markdown
Rust 在模块中找到项的位置,并使用路径来引用模块树中的项.路径分为两种:一种是绝对路径,另一种是相对路径.
绝对路径和相对路径都后跟一个或多个由双冒号::分割的标识符

1. 绝对路径 (absolute path) 从 crate 根部开始,以 crate 名或者字面量 crate 开头.
2. 相对路径 (relative path) 从当前模块开始,以 self、super 或当前模块的标识符开头.

```

```markdown
Rust 的模块系统通过私有性边界来控制代码的可见性和访问权限,默认情况下所有项 (函数、结构体、模块等)
都是私有的.

1. 私有性规则:
   父模块无法访问子模块中的私有项. 子模块可以访问父模块中的项,因为子模块能看到其定义的上下文.
2. 封装与隐藏:
   子模块封装并隐藏实现细节,类似餐馆的后台办公室——对外不可见,但内部可自由操作.
3. 公共暴露:
   使用 pub 关键字可将子模块的内部项暴露给上级模块,实现可控的公开.
   这种设计让开发者能安全地修改内部实现而不影响外部代码,同时通过 pub 精确控制接口的可见性.
```

### 使用 super 起始的相对路径

```markdown
模块树中的项可以通过 super 关键字引用当前模块的父模块,类似于 shell 的 .. 命令
```

### 创建公有的结构体和枚举

```markdown
我们还可以使用 pub 来设计公有的结构体和枚举,不过有一些额外的细节需要注意.如果我们
在一个结构体定义的前面使用了 pub ,这个结构体会变成公有的,但是这个结构体的字段仍然
是私有的,我们可以根据情况决定每个字段是否公有.由于将结构体的某些字段设置为了私有,
所以必须定义一个公有的构造函数,这个构造函数会返回一个 pub 的结构体实例.与之相反,
如果我们将枚举设为公有,则它的所有成员都将变为公有.
```

## 使用 use 关键字将名称引入作用域

```markdown
使用 use 引入作用域,将模块树中的项引入当前作用域,也可以在后面加上一个别名防止名称冲突.
当使用 use 关键字将名称导入作用域时,在新作用域中可用的名称是私有的.如果为了让调用你编写
的代码的代码能够像在自己的作用域内引用这些类型,可以结合 pub 和 use.这个技术被称为重导出
(re-exporting),因为这样做将项引入作用域并同时使其可供其他代码引入自己的作用域.例如我在
main.rs 里引入了 lib.rs 中 pub use 的枚举.对于外部包的使用我们可以去 Crates.io 搜索
模块并查看其文档将其导入项目的 Cargo.toml 文件中.最后也可以通过 glob use * 将一个路径下
所有公有项引入作用域,例如 use std::collections::*;使用 glob 运算符时要多加小心,Glob
会使得我们难以推导作用域中有什么名称和它们是在何处定义的,这就有点像 C++ 的 using 一样 .
glob 运算符经常用于测试模块 tests 中.
```

## 将模块划分为不同的文件

```markdown
模块可以定义在单独的文件中,这样我们就可以在不同的文件中定义模块,并让模块的代码保持一致性,
解耦代码同时提升可读性. Rust 会在同一个目录下寻找模块定义的文件,并使用文件名作为模块名.
此时我们采用 mod front_of_house; 以分号结束,告诉 Rust 在另一个与模块同名的文件中加载
模块的内容.例如,如果模块 front_of_house,那么 Rust 会在同级目录下的 front_of_house.rs
中寻找 或者 在同级目录下新建一个 front_of_house 目录并放入两个文件: mod.rs 和 hosting.rs,
并将 mod front_of_house; 改为 pub mod front_of_house; 使得模块可被其他模块引用. mod.rs
用于表示其所处的文件夹是一个模块目录,然后也要在 mod.rs 中声明对应的模块,例如 pub mod hosting
hosting.rs 用于定义同名模块内容.
```