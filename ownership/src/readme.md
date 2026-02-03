# Rust 所有权

```markdown
所有运行的程序都必须管理其使用计算机内存的方式.一些语言中具有垃圾回收机制,
在程序运行时不断地寻找不再使用的内存;在另一些语言中，开发者必须亲自分配和释放内存.
Rust 则选择了第三种方式:通过所有权系统管理内存,编译器在编译时会根据一系列的规则进行检查.
在运行时,所有权系统的任何功能都不会减慢程序.
```

**跟踪哪部分代码正在使用堆上的哪些数据,最大限度地减少堆上的重复数据量,
以及清理堆上不再使用的数据确保不会耗尽空间,这些问题正是所有权系统要处理的.**

## 所有权规则

* Rust 中的每一个值都有一个被称为其所有者 (owner) 的变量
* 值在任一时刻有且只有一个所有者
* 当所有者 (变量) 离开作用域,这个值将被丢弃

## 内存与分配 以 String 为例

```markdown
对于 String 类型,为了支持一个可变,可增长的文本片段,所以需要在堆上分配
一块在编译时未知大小的内存来存放内容.这意味着必须在运行时向内存分配器请求内存,
还有就是需要一个处理完 String 时将内存返回给分配器的方法.第一步很简单,例如采用
String::from 申请一个字面量大小的内存
```

```rust
#![allow(unused)]
fn main() {
    {
        let s = String::from("hello");
    }
    // s 在这里离开了作用域,它的值将被丢弃.
}
```

```markdown
第二步 Rust 和其他 GC 语言自动记录并清除掉不再使用的内存或者手动记录
并调用清理函数来清理不再使用的内存的语言不一样.Rust 采取了一个不同的策略,
内存在拥有它的变量离开作用域后就被自动释放,当变量离开作用域,Rust 调用一个
特殊的函数 drop 来清理这个变量.
```

### 变量与数据交互的方式 (一) : 移动

```rust
fn main() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y); // x = 5, y = 5
}
 ```

```markdown
由于 x,y 的类型确定也即其所占的内存大小也确定,所以 x,y 存在栈上,
也即拷贝了 x 的值赋给 y,所以打印输出结果为 x = 5, y = 5
```

```rust
#![allow(unused)]
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s1 = {}, s2 = {}", s1, s2); // error Value used after being moved [E0382]
}
```

```markdown
走 String 底层的角度分析 s1 在创建时会向堆申请一块内存,并把 s1 的内容或者说字面量
写到申请的堆内存里,同时将堆内存的地址以及自身长度还有容量等信息保存在 s1 的栈内存里,
当我们将 s1 赋值给 s2,String 的数据被复制了,这意味着我们从栈上拷贝了它的指针、长度和容量
并没有复制指针指向的堆上数据.如果 Rust 连值也拷贝了的话,在 s1 的内容较大的时候就会对
运行效率产生影响.接着上文的 drop 函数,当 s1 离开作用域时,drop 函数会调用并清理 s1,
而 s2 也持有着与 s1 的同一个堆内存地址,所以理论上 s2 离开作用域时,drop 函数也会调用并清理 s2,
这就会引发一个 double free 错误也即释放了一个已经释放的内存导致内存污染.所以 Rust 在
let s2 = s1 时会认为 s1 已经移动到 s2 中了,也即 s1 无效,移动操作有点类似于其他语言的浅拷贝,
只拷贝了 s1 的指针、长度和容量,而不是复制指针指向的堆上的数据.
```

### 变量与数据交互的方式 (二) : 克隆

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2); // s1 = hello, s2 = hello
}
```

```markdown
如果说移动是一种类似于浅拷贝的操作,那么克隆是一种类似于深拷贝的操作,它不仅会拷贝
数据指针、长度和容量,还会将指针指向的堆上的数据也拷贝一份.到这里可能会疑惑为什么
之前 let x = 5; let y = x; 不需要克隆,原因是 x,y 的类型是 i32,这是一个基本类型,
大小是确定的,所以 x,y 存在栈上,也就表明没有理由在创建 y 之后使得 x 无效,换句话说
不存在 double free 的风险,在这里调用 clone() 是多余的,直接拷贝值就可以了.
```

```markdown
Rust 有一个叫做 Copy trait 的特殊标注,可以用在类似整型这样的存储在栈上的类型上.
如果一个类型实现了 Copy trait,那么一个旧的变量在将其赋值给其他变量后仍然可用.
Rust 不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait 比如 String.
如下是一些 Copy 的类型:

1. 所有整数类型,比如 u32
2. 布尔类型,bool,它的值是 true 和 false
3. 所有浮点数类型,比如 f64
4. 字符类型,char
5. 元组,当且仅当其包含的类型也都实现 Copy 的时候.比如,(i32, i32) 实现了 Copy,但 (i32, String) 就没有 (类比 golang 的
   comparable 类型 struct)
```

## 所有权和函数

```rust
fn main() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里,
    // 但 i32 是 Copy 的,所以在后面可继续使用 x

} // 这里, x 先移出了作用域,然后是 s.但因为 s 的值已被移走,
// 所以不会有特殊操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里,some_string 移出作用域并调用 `drop` 方法.占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里,some_integer 移出作用域.不会有特殊操作

```

```markdown
可以联想到前文的移动和克隆,这里 s 的所有权被移动到 takes_ownership 函数中,
换句话说先执行 let s = String::from("hello"); 再执行 takes_ownership(s);
相当于在函数内部隐式执行了 let some_string = s;也就是此时 s 被移动到 some_string 中,
也即 s 的所有权被移动到 some_string 中.然后 x 那部分逻辑一样,先执行 let x = 5; 再执行 makes_copy(x);
相当于在函数内部隐式执行了 let some_integer = x,但是由于 i32 是 Copy 的,所以 x 的值被复制给 some_integer,
在后续逻辑中 x 仍然有效.
```

## 返回值与作用域

```rust
fn main() {
    let s1 = gives_ownership(); // gives_ownership 将返回值
    // 移给 s1

    let s2 = String::from("hello"); // s2 进入作用域

    let s3 = takes_and_gives_back(s2); // s2 被移动到
    // takes_and_gives_back 中,
    // 它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
// 所以什么也不会发生。s1 移出作用域并被丢弃

fn gives_ownership() -> String {
    // gives_ownership 将返回值移动给
    // 调用它的函数

    let some_string = String::from("yours"); // some_string 进入作用域

    some_string // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域

    a_string // 返回 a_string 并移出给调用的函数
}

```

```markdown
变量的所有权总是遵循相同的模式:将值赋给另一个变量时移动它,当持有堆中数据值的变量离开作用域时,
其值将通过 drop 被清理掉,除非数据被移动为另一个变量所有.也就是说 drop 函数执行的三个条件是
一是这个变量是所有权持有者、第二是这个变量是堆数据、第三是这个变量出了作用域
```

## 引用与借用

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

```markdown
& 符号就是 引用,它允许你使用值但不获取其所有权,避免了上文的所有权多次转让的问题,
虽然 calculate_length 函数的执行之后 s 离开了作用域,但 s1 的所有权没有转移给
calculate_length 函数里的 s, 所以 s1 仍然有效,而 s 没有所有权所以作用域结束
后也不会被 drop .这种引用 的方式,称之为借用 (borrowing) ,有点类似于指针, 与
使用 & 引用相反的操作是解引用 (dereferencing) ,它使用解引用运算符 *.
```

```rust
fn main() {
    let s = String::from("hello");
    change(&s); // error 
}
fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

```markdown
正如变量在默认情况下是不可变的一样,引用默认也是不可变的,我们无法通过引用修改内容.
```

### 可变引用

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

```markdown
接着上面的例子,想要修改的那个变量本身必须是可变的,所以用 &mut 符号,
所以函数的接收参数也必须是一个可变引用这样才能在借用的变量上修改内容.
不过可变引用有一个很大的限制: 在同一时间,只能有一个对某一特定数据的
可变引用,尝试创建两个可变引用的代码将会失败 (想象一下有好几个人知道你的银行卡密码)
所以为了保证数据的安全,Rust 会禁止多个可变引用同时存在:
```

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s; // error second mutable borrow occurs here
    println!("{}, {}", r1, r2);
}
```

```markdown
这个限制的好处是 Rust 可以在编译时就避免数据竞争.数据竞争 (data race) 类似于竞态条件,
它由这三个行为造成: 两个或更多指针同时访问同一数据、至少有一个指针被用来写入数据、没有同步
数据访问的机制.以上三个行为同时发生才会造成数据竞争,而不是单一行为.如果需要创建多个可变引用,
那么这些引用必须通过一个作用域来限制来防止同时拥有多个可变引用.
```

```rust
#![allow[unused]]
fn main() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 在这里离开了作用域
    let r2 = &mut s;
}
```

```markdown
不能在拥有不可变引用的同时拥有可变引用.使用者可不希望不可变引用的值在他们的眼皮底下突然被改变了！
然而,多个不可变引用是可以的,因为没有哪个只能读取数据的人有能力影响其他人读取到的数据.注意一个引用
的作用域从声明的地方开始一直持续到最后一次使用为止.所以如下代码是可以编译的:
```

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);
    let r3 = &mut s;
    println!("{}", r3);
}
```

```markdown
不可变引用 r1 和 r2 的作用域在 println! 最后一次使用之后结束,这也是创建可变引用 r3 的地方
它们的作用域没有重叠,所以代码是可以编译的,编译器在作用域结束之前判断不再使用的引用的能力被称为
非词法作用域生命周期 (Non-Lexical Lifetimes),简称 NLL.
```

### 悬垂引用

```markdown
在具有指针的语言中,很容易通过释放内存时保留指向它的指针而错误地生成一个悬垂指针 (dangling pointer)
所谓悬垂指针是其指向的内存可能已经被分配给其它持有者.相比之下,在 Rust 中编译器确保引用永远也不会变成悬垂状态
当你拥有一些数据的引用,编译器确保数据不会在其引用之前离开作用域.如下示的代码会报错:
```

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
```

```markdown
dangle 函数返回了 s 的引用,但 s 已经离开作用域,所以返回的引用会变成悬垂引用.
因为 s 是在 dangle 函数内创建的,当 dangle 的代码执行完毕后,s 将被释放.因为 s
是堆变量并且持有所有权,执行完就出了作用域,但函数尝试返回它的引用,这意味着这个引用
会指向一个无效的 String,Rust 不会允许这么做,解决方法是直接返回 String,将 s 移动到函数的返回值中.
```

### 引用总结

* 在任意给定时间,要么只能有一个可变引用,要么只能有多个不可变引用
* 引用必须总是有效的

## 切片 slice 类型

```markdown
另一个没有所有权的数据类型是 slice, slice 允许你引用集合中一段连续的元素序列,而不用引用整个集合.
let s = String::from("hello world"); 引用方式:  &s[start..end]这种是最长见的方式就是截取
下标区间(左闭右开) 要想右闭就改为 [start..=end] 还有一些语法糖: &s[..] 表示从字符串开头到结尾
&s[i..] 表示从下标 i 到字符串结尾 &s[..i] 表示从字符串开头到下标 i-1 &s[..=i] 表示从字符串开头到下标 i
```

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word 的值为 5

    s.clear(); // 这清空了字符串，使其等于 ""

    // word 在此处的值仍然是 5，
    // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！
    println!("{:?}", word);
}
```

```markdown
上述例子虽然运行正常,但是 word 的值没有做到无效,因为 s 的内容被清空了,
目的是希望将 word 绑定到 s 上来保证 word 的值也跟着变化.可以将上述函数
改写如下:
```

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
fn main() {
    let my_string = String::from("hello world");
    let word = first_word(&my_string);
    println!("{:?}", word);
    my_string.clear(); // error 
    println!("{:?}", word);
}
```

```markdown
上述函数在编译时就会出错,因为在 first_word 函数中使用的是不可变引用,
而 clear 函数使用的是可变引用,在 clear 结束后引用使用了 word 需要
再次获取一个不可变引用,Rust 不允许 clear 中的可变引用和 word 中的
不可变引用同时存在,因此编译失败
```