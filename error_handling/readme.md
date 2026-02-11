# 错误处理

```markdown
Rust 将错误组合成两个主要类别: 可恢复错误 (recoverable) 和 不可恢复错误 (unrecoverable).
可恢复错误通常代表向用户报告错误和重试操作是合理的情况,比如未找到文件.不可恢复错误通常是 bug 的
同义词,比如尝试访问超过数组结尾的位置. 大部分语言并不区分这两类错误,并采用类似异常这样方式统一处
理他们.Rust 并没有异常,但是,有可恢复错误 Result<T, E> ,和不可恢复(遇到错误时停止程序执行)错误
panic!.
```

## 对应 panic 时栈展开或终止

```markdown
当出现 panic 时,程序默认会开始展开 (unwinding) ,这意味着 Rust 会回溯栈并清理它遇到的每一个函数
的数据,不过这个回溯并清理的过程有很多工作.另一种选择是直接终止 (abort) ,这会不清理数据就退出程序.
那么程序所使用的内存需要由操作系统来清理.如果你需要项目的最终二进制文件越小越好,panic 时通过在
Cargo.toml 的 [profile] 部分增加 panic = 'abort',可以由展开切换为终止.例如,如果你想要在release
模式中 panic 时直接终止：
```

```toml
[profile.release]
panic = 'abort'
```

## 使用 panic! 的 backtrace

```markdown
Rust 提供了 RUST_BACKTRACE 环境变量,当这个变量被设置为 1 时,Rust 会打印 panic! 所调用的函数的
错误栈的展开详细信息,允许你检查错误并找出导致 panic 的函数.
```

```shell
RUST_BACKTRACE=1 cargo run
```

```markdown
当然,你也可以不配置这些错误打印就像前面的 abort 一样,通过在 Cargo.toml 的 [profile] 部分增加
panic = 'abort',选择你的开发模式: dev、release、bench、 test
```

```shell
cargo run --release
```

## Result 与可恢复的错误

```markdown
大部分错误并没有严重到需要程序完全停止执行.有时,一个函数会因为一个容易理解并做出反应的原因失败.
例如,如果因为打开一个并不存在的文件而失败,此时我们可能想要创建这个文件,而不是终止进程.此时就可以
使用 Result<T, E> 来处理错误.
```

````rust
#[allow(unused)]
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        }
    };
}
````

## 匹配不同的错误

```markdown
前面的例子针对所有错误都返回一个 panic! ,但是某些错误我们可能希望处理,比如文件打开失败,我们希望
创建这个文件而不是直接终止退出.
```

```rust
use std::fs::File;
use std::io::ErrorKind;
#[allow(unused)]
fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => {
            println!("File opened successfully");
            file
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(f) => {
                    println!("File created successfully");
                    f
                }
                Err(e) => panic!("Error creating file: {:?}", e),
            },
            other_error => panic!("Error opening file: {:?}", other_error),
        },
    };
}
```

```markdown
对于上述代码里的错误处理采取了大量的 match 表达式,我们可以通过 unwrap_or_else 函数来处理错
误减少不必要的和冗余的匹配.
```

```rust
use std::fs::File;
use std::io::ErrorKind;
#[allow(unused)]
fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|err| {
                panic!("Error creating file: {:?}", err);
            })
        } else {
            panic!("Error opening file: {:?}", err);
        }
    });
}
```

## 失败时 panic 的简写：unwrap 和 expect

```markdown
Rust 提供了 unwrap 和 expect 方法,它们都返回结果中的值,但是当它们失败时,它们会返回一个错误
信息.虽然遇到错误时都会调用 panic! ,但是 expect 提供了自定义错误描述的语法.这样就很方便在开
发时检查并定位错误.
```

```rust
use std::fs::File;
#[allow(unused)]
fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    println!("File opened successfully");
    let f = File::open("hello.txt").unwrap();
    println!("File opened successfully");
}
 ```

## 采用 ? 运算符来传播错误

```markdown
当编写一个需要先调用一些可能会失败的操作的函数时,除了在这个函数中处理错误外,还可以选择让调用者
知道这个错误并决定该如何处理.这被称为传播 (propagating) 错误,这样能更好地控制代码调用,因为比
起你代码所拥有的上下文,调用者可能拥有更多信息或逻辑来决定应该如何处理错误.
```

```rust
use std::fs::File;
use std::io;
use std::io::Read;

#[allow(unused)]
fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}
fn main() {
    println!("{:?}", read_file("Cargo.toml"))
}
```

```markdown
match 表达式与问号运算符所做的有一点不同: ? 运算符所使用的错误值被传递给了 from 函数,它定义于
标准库的 From trait 中,其用来将错误从一种类型转换为另一种类型.当 ? 运算符调用 from 函数时,
收到的错误类型被转换为由当前函数返回类型所指定的错误类型.这在当函数返回单个错误类型来代表所有可
能失败的方式时很有用,即使其可能会因很多种原因失败.只要每一个错误类型都实现了 from 函数来定义如
何将自身转换为返回的错误类型, ? 运算符会自动处理这些转换.甚至可以在 ? 后采用链试调用的方式来进
一步简化代码.
```

```rust
use std::fs::File;
use std::io;
use std::io::Read;
#[allow(unused)]
fn read_file(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}
fn main() {
    println!("{:?}", read_file("Cargo.toml"))
}
```

```markdown
main 函数是特殊的,其必须返回什么类型是有限制的.main 函数的一个有效的返回值是 (),同时出于方便
,另一个有效的返回值是 Result<T, E>,如果想在 main 函数里使用 ? 运算符,则必须返回 Result<T, E>
,同时为了处理任何类型的错误,可以采用 Box<dyn Error>,Box<dyn Error> 被称为 "trait 对象"
(trait object),目前可以理解 Box<dyn Error> 为使用 ? 时 main 允许返回的 "任何类型的错误".
```

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{:?}", read_file("Cargo.toml"));
    File::open("Cargo.toml")?;
    Ok(())
}
```

# 总结

```markdown
Rust 的错误处理功能旨在帮助你编写更健壮的代码. panic! 宏代表一个程序无法处理的状态,并停止执行
而不是使用无效或不正确的值继续处理. Rust 类型系统的 Result 枚举代表操作可能会在一种可以恢复的
情况下失败.可以使用 Result 来告诉代码调用者他需要处理潜在的成功或失败.在适当的场景使用 panic!
和 Result 将会使你的代码在面对不可避免的错误时显得更加可靠.
```