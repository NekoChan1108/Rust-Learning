# 枚举

```markdown
枚举允许你通过列举可能的成员 (variants) 来定义一个类型,例如:
```

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

```markdown
现在 IpAddrKind 就是一个可以在代码中使用的自定义数据类型了,
注意枚举的成员位于其标识符的命名空间中,并使用两个冒号分开.
这么设计的益处是现在 IpAddrKind::V4 和 IpAddrKind::V6
都是 IpAddrKind 类型的.
```

```rust
use std::any::type_name_of_val;

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("{:#?}", home);
    println!("{:#?}", loopback);
}
```

```markdown
可以使用一种更简洁的方式来表达相同的概念,仅仅使用枚举并将数据
直接放进每一个枚举成员而不是将枚举作为结构体的一部分
```

```rust
#[allow(unused)]
enum IpAddr {
    V4(String),
    V6(String),
}
fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
}
```

```markdown
用枚举替代结构体还有另一个优势:每个成员可以处理不同类型和数量的数据.
IPv4 版本的 IP 地址总是含有四个值在 0 和 255 之间的数字部分.如果
我们想要将 V4 地址存储为四个 u8 值而 V6 地址仍然表现为一个 String,
这就不能使用结构体了,枚举则可以轻易地处理这个情况
```

```rust
use std::fmt;

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddrKind {
    fn get_url(&self) -> String {
        match self {
            IpAddrKind::V4(a, b, c, d) => fmt::format(format_args!("{}.{}.{}.{}", a, b, c, d)),
            IpAddrKind::V6(s) => s.to_string(),
        }
    }
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
    println!("{:#?}", home);
    println!("{:#?}", loopback);
    println!("{}", home.get_url());
    println!("{}", loopback.get_url());
}
```

## Option 枚举和其相对于空值的优势

```markdown
Option 是标准库定义的另一个枚举.之所以应用广泛是因为它编码了一个非常普遍的场景,
即一个值要么有值要么没值.从类型系统的角度来表达这个概念就意味着编译器需要检查是否
处理了所有应该处理的情况,这样就可以避免在其他编程语言中非常常见的 bug.编程语言的
设计经常要考虑包含哪些功能,但考虑排除哪些功能也很重要.Rust 并没有很多其他语言中有
的空值功能,空值 Null 是一个值,它代表没有值.在有空值的语言中,变量总是这两种状态之
一:空值和非空值.空值的问题在于当你尝试像一个非空值那样使用一个空值,会出现某种形式的
错误,因为空和非空的属性无处不在,非常容易出现这类错误.然而,空值尝试表达的概念仍然是
有意义的:空值是一个因为某种原因目前无效或缺失的值.问题不在于概念而在于具体的实现.
为此,Rust 并没有空值,不过它确实拥有一个可以编码存在或不存在概念的枚举 Option<T>.
Option<T> 由于其可用性直接被预包含在了 prelude 中,因此你可以直接使用它而不需要引用.
Option 枚举有两个成员: Some 和 None.
```

```rust
#![allow(unused)]
fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None; //
}
```

[Option](https://rustwiki.org/zh-CN/std/option/enum.Option.html)

```markdown 
如果使用 None 而不是 Some,需要告诉 Rust Option<T> 是什么类型的,因为编译器只通过
None 值无法推断出 Some 成员保存的值的类型. 当有一个 Some 值时,我们就知道存在一个值
而这个值保存在 Some 中.当有个 None 值时,在某种意义上,它跟空值具有相同的意义:并没有一
个有效的值,要取出 Some 值,必须先检查它是否有值,然后采取 unwrap 操作来获取其值,但是官
方不推荐使用 unwrap 操作因为它在没有检查时会 panic ,可以退而求其次使用 unwrap_or系
列方法.
```

```rust
fn main() {
    let x: Option<i8> = Some(5);
    if x.is_some() {
        println!("{}", x.unwrap());
    }
    let y: Option<i8> = None;
    println!("{}", y.unwrap_or(0)) // 0
}
```

## Option 小结

```markdown
在对 Option<T> 进行 T 的运算之前必须将其转换为 T.通常这能帮助我们捕获到空值最常见的问题之一:
假设某值不为空但实际上为空的情况. 不再担心会错误地假设一个非空值,为了拥有一个可能为空的值,必须要
显式地将其放入对应类型的 Option<T> 中.当使用这个值时,必须明确地处理值为空的情况.只要一个值不是
Option<T> 类型,你就可以安全地认定它的值不为空,这是 Rust 的一个经过深思熟虑的设计决策,来限制空
值的泛滥以增加 Rust 代码的安全性.
```

## match 控制流

### 匹配 Option<T>

```markdown
在之前的部分中使用 Option<T> 时,是为了从 Some 中取出其内部的 T 值,还可以使用 match 处理
Option<T> 只不过这回比较是 Option<T> 的成员,但 match 表达式的工作方式保持不变.
```

```rust
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", six); // Some(6)
    println!("{:?}", none); // None
}
```

```markdown
match 语句的每个分支都必须处理所有可能情况,否则编译器会报错.
也就是匹配是穷尽的不能单单匹配一个值,Rust 中的匹配是穷举式的
(exhaustive): 必须穷举到最后的可能性来使代码有效.
```

```rust

#[allow(unused)]
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {    // The `match` expression does not cover all possible cases [E0004]
            Some(i) => Some(i + 1),
            // None => None,
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

### 通配模式和 _ 占位符

```markdown
如果希望对一些特定的值采取特殊操作,而对其他的值采取默认操作,那么可以使用通配模式
来匹配这些值.
```

```rust
#[allow(unused)]
#[allow(unused_variables)]
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}
```

```markdown
Rust 还提供了一个模式,当我们不想使用通配模式获取的值时,可以使用 _ ,
这是一个特殊的模式,可以匹配任意值而不绑定到该值.这告诉 Rust 我们不会
使用这个值,所以 Rust 也不会警告我们存在未使用的变量.
```

```rust
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // 匹配任意值 如果忽视处理可以使用 _ => (), 来忽略处理
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
}
```

## if let 控制流

```markdown
如果 match 语句的每个分支都必须处理所有可能情况过于麻烦,那么可以使用 if let 语法.
if let 语法让我们以一种不那么冗长的方式结合 if 和 let,来处理只匹配一个模式的值而
忽略其他模式的情况.
```

```rust
#![allow(unused)]
fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // 等价于
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
```