# 泛型、trait以及生命周期

```markdown
每一个编程语言都有高效处理重复概念的工具.在 Rust 中其工具之一就是泛型 (generics).泛型是具体类型
或其他属性的抽象替代.我们可以表达泛型的属性,比如他们的行为或如何与其他泛型相关联,而不需要在编写和
编译代码时知道他们在这里实际上代表什么.trait,这是一个定义泛型行为的方法.trait 可以与泛型结合来将
泛型限制为拥有特定行为的类型,而不是任意类型.生命周期,它是一类允许我们向编译器提供引用如何相互关联的
泛型.Rust 的生命周期功能允许在很多场景下借用值的同时仍然使编译器能够检查这些引用的有效性.
```

## 泛型数据类型

```markdown
泛型数据类型的使用和其他语言例如 Golang 类似,可以在结构体、方法、枚举等当中使用.
```

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mix_up<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mix_up(p2);
    println!("p3.x = {:#?}, p3.y = {:#?}", p3.x, p3.y);
}
```

## 泛型代码的性能

```markdown
Rust 实现了泛型,使得使用泛型类型参数的代码相比使用具体类型并没有任何速度上的损失.Rust 通过在编
译时进行泛型代码的单态化 (monomorphization) 来保证效率.单态化是一个通过填充编译时使用的具体类
型,将通用代码转换为特定代码的过程.
```

```rust
#[allow(unused)]
fn main() {
    let integer = Some(5);
    let float = Some(5.0);
}
```

```markdown
当 Rust 编译这些代码的时候，它会进行单态化。编译器会读取传递给 Option<T> 的值并发现有两种
Option<T>: 一个对应 i32 另一个对应 f64.为此,它会将泛型定义 Option<T> 展开为 Option_i32
和 Option_f64,接着将泛型定义替换为这两个具体的定义.
``` 

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

## trait

```markdown
trait 告诉 Rust 编译器某个特定类型拥有可能与其他类型共享的功能.可以通过 trait 以一种抽象的方
式定义共享的行为.可以使用 trait bounds 指定泛型是任何拥有特定行为的类型.可以简单地将 trait
视为例如 Java Golang 里的接口 (interface).
```

### trait 定义

```markdown
一个类型的行为由其可供调用的方法构成.如果可以对不同类型调用相同的方法的话,这些类型就可以共享相同
的行为了.trait 定义是一种将方法签名组合起来的方法,目的是定义一个实现某些目的所必需的行为的集合.
```

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```

```markdown
在类型上实现 trait 类似于实现与 trait 无关的方法.区别在于 impl 关键字之后,我们提供需要实现
trait 的名称,接着是 for 和需要实现 trait 的类型的名称.在 impl 块中,使用 trait 定义中的方
法签名,不过不再后跟分号,而是需要在大括号中编写函数体来为特定类型实现 trait 方法所拥有的行为.就
和实现接口一样.实现 trait 时需要注意的一个限制是,只有当 trait 或者要实现 trait 的类型位于
crate 的本地作用域时,才能为该类型实现 trait.这个限制是被称为相干性 (coherence) 的程序属性
的一部分,或者更具体的说是孤儿规则 (orphan rule),其得名于不存在父类型.这条规则确保了其他人编
写的代码不会破坏你代码,反之亦然.没有这条规则的话,两个 crate 可以分别对相同类型实现相同的 trait
,而 Rust 将无从得知应该使用哪一个实现.
```

### trait bound

```markdown
对于方法重载以及默认方法以及 trait 作为函数参数这些和 Golang Java 都差不多就不过多赘述了.
唯一不同的是作为函数参数要在对应的 trait 名字前面加上 impl 关键字.
```

```rust
fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

```markdown
impl Trait 语法适用于直观的例子,它实际上是一种较长形式语法的语法糖.我们称为 trait bound,它
看起来像
```

```rust
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
```

```markdown
trait bound 与泛型参数声明在一起,位于尖括号中的冒号后面.impl Trait 很方便,适用于短小的例子.
trait bound 则适用于更复杂的场景.例如,可以获取两个实现了 Summary 的参数.使用 impl Trait
的语法看起来像这样:
```

```rust
fn notify(item1: impl Summary, item2: impl Summary) {
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize());
}
```

```markdown
这样适用于接收只要实现了 Summary 的参数的函数.但是想要获取两个参数,并且要求它们是相同的类型,
那么就需要 trait bound 了.
```

```rust
fn notify<T: Summary>(item1: T, item2: T) {
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize());
}
```

```markdown
更适合 trait bound 的场景是,当 trait bound 需要多个 trait 时. Rust 允许我们使用 + 运算
符来组合多个 trait bound 或者 impl Trait 语法.
```

```rust
use std::fmt::{Debug, Display};

fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    1
}
fn some_function(t: impl Display + Clone, u: impl Debug + Clone) -> i32 {
    1
}
```

```markdown
Rust 还为 trait bound 提供了更短的语法,使用 where 从句使得代码更简洁明了.这个函数签名就显
得不那么杂乱,函数名、参数列表和返回值类型都离得很近,看起来跟没有那么多 trait bounds 的函数
很像.
```

```rust
fn some_function<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{ 1 }
```

```markdown
当然,也可以使用 trait bound 有条件地实现方法
```

```rust
pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Pair<T>
where
    T: PartialOrd + Display,
{
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

```markdown
上述代码我只为可比较以及可打印的类型实现了 cmp_display 方法.也可以对任何实现了特定 trait 的
类型有条件地实现 trait.对任何满足特定 trait bound 的类型实现 trait 被称为 blanket implementations,
他们被广泛的用于 Rust 标准库中.例如,标准库为任何实现了 Display trait 的类型实现了 ToString
trait.这个 impl 块看起来像这样:
```

```rust
impl<T: Display> ToString for T {
    // --snip--
}
```

## 生命周期与引用有效性

```markdown
Rust 中的每一个引用都有其生命周期 (lifetime),也就是引用保持有效的作用域.大部分时候生命周期是
隐含并可以推断的,正如大部分时候类型也是可以推断的一样.类似于当因为有多种可能类型的时候不得不注明
类型,也会出现引用的生命周期以一些不同方式相关联的情况,所以 Rust 需要我们使用泛型生命周期参数来
注明他们的关系,这样就能确保运行时实际使用的引用绝对是有效的.生命周期的概念从某种程度上说不同于其
他语言中类似的工具,毫无疑问这是 Rust 最与众不同的功能.
```

### 生命周期避免了悬垂引用

```markdown
生命周期的主要目标是避免悬垂引用.它会导致程序引用了非预期引用的数据
```

```rust
#[allow(unused)]
fn main() {
    {
        let r;
        {
            let x = 5;
            r = &x;
        }
        println!("r: {}", r);
    }
}
```

```markdown
外部作用域声明了一个没有初值的变量 r,而内部作用域声明了一个初值为 5 的变量 x.在内部作用域中,我
们尝试将 r 的值设置为一个 x 的引用.接着在内部作用域结束后，尝试打印出 r 的值.这段代码不能编译
因为 r 引用的值在尝试使用之前就离开了作用域.
```

### 借用检查器

```markdown
Rust 编译器有一个借用检查器 (borrow checker),它比较作用域来确保所有的借用都是有效的.
```

```rust
fn main() {
    {
        let r;                // ---------+-- 'a
        //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
        //          |
        println!("r: {}", r); //          |
    }                         // ---------+
}
```

```markdown
这里将 r 的生命周期标记为 'a 并将 x 的生命周期标记为 'b.如你所见,内部的 'b 块要比外部的生命
周期 'a 小得多.在编译时,Rust 比较这两个生命周期的大小,并发现 r 拥有生命周期 'a,不过它引用了
一个拥有生命周期 'b 的对象.程序被拒绝编译,因为生命周期 'b 比生命周期 'a 要小:被引用的对象比它
的引用者存在的时间更短.反之言,引用者的生命周期需要比被引用的对象更短才能确保引用有效.
```

### 函数中的泛型生命周期

```markdown
下面代码是一个典型的生命周期错误,它在编译时就会报错.错误提示文本揭示了返回值需要一个泛型生命周
期参数,因为 Rust 并不知道将要返回的引用是指向 x 或 y.事实上我们也不知道,因为函数体中 if 块
返回一个 x 的引用而 else 块返回一个 y 的引用！当我们定义这个函数的时候,并不知道传递给函数的
具体值,所以也不知道到底是 if 还是 else 会被执行.我们也不知道传入的引用的具体生命周期,所以也
就不能通过观察作用域来确定返回的引用是否总是有效.借用检查器自身同样也无法确定,因为它不知道 x
和 y 的生命周期是如何与返回值的生命周期相关联的.为了修复这个错误,我们将增加泛型生命周期参数来
定义引用间的关系以便借用检查器可以进行分析
```

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

### 生命周期的标注

```markdown
生命周期标注并不改变任何引用的生命周期的长短.与当函数签名中指定了泛型类型参数后就可以接受任何类
型一样,当指定了泛型生命周期后函数也能接受任何生命周期的引用.生命周期标注描述了多个引用生命周期
相互的关系,而不影响其生命周期.生命周期标注有着一个不太常见的语法:生命周期参数名称必须以撇号'开
头,其名称通常全是小写,类似于泛型其名称非常短.'a 是大多数人默认使用的名称.生命周期参数标注位于
引用的 & 之后,并有一个空格来将引用类型与生命周期标注分隔开.
```

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

```markdown
单个生命周期标注本身没有多少意义,因为生命周期标注告诉 Rust 多个引用的泛型生命周期参数如何相互
联系的.例如如果函数有一个生命周期 'a 的 i32 的引用的参数 first.还有另一个同样是生命周期 'a
的 i32 的引用的参数 second.这两个生命周期标注意味着引用 first 和 second 必须与这泛型生命周
期存在得一样久.现在来看看 longest 函数的上下文中的生命周期.就像泛型类型参数,泛型生命周期参数
需要声明在函数名和参数列表间的尖括号中.这里我们想要告诉 Rust 关于参数中的引用和返回值之间的限
制是他们都必须拥有相同的生命周期,就像示例中在每个引用中都加上了 'a 那样,这就是函数签名中生命周
期参数的声明方式.
```

```markdown
当具体的引用被传递给 longest 时,被 'a 所替代的具体生命周期是 x 的作用域与 y 的作用域相重叠的
那一部分.换一种说法就是泛型生命周期 'a 的具体生命周期等同于 x 和 y 的生命周期中较小的那一个.因
为我们用相同的生命周期参数 'a 标注了返回的引用值,所以返回的引用值就能保证在 x 和 y 中较短的那个
生命周期结束之前保持有效.
```

```markdown
当从函数返回一个引用,返回值的生命周期参数需要与一个参数的生命周期参数相匹配.如果返回的引用没有
指向任何一个参数,那么唯一的可能就是它指向一个函数内部创建的值,它将会是一个悬垂引用,因为它将会在
函数结束时离开作用域.
```

### 结构体中的生命周期标注

```markdown
类似于泛型参数类型,必须在结构体名称后面的尖括号中声明泛型生命周期参数,以便在结构体定义中使用生命
周期参数.这个标注意味着对应的实例不能比其字段中的引用存在的更久,否则就会产生悬垂引用.
```

### 生命周期省略

```rust

#![allow(unused)]
fn main() {
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }
}
```

```markdown
上述代码并没有显示声明生命周期参数,但是编译仍然是成功的.早期的 Rust 版本确实是需要显示声明生命
周期参数的,但是 Rust 1.0 版本开始,编译器会自动为生命周期参数进行省略.被编码进 Rust 引用分析
的模式被称为生命周期省略规则 (lifetime elision rules).这并不是需要开发者遵守的规则,这些规则
是一系列特定的场景,此时编译器会考虑,如果代码符合这些场景,就无需明确指定生命周期.
```

```markdown
省略规则并不提供完整的推断:如果 Rust 在明确遵守这些规则的前提下变量的生命周期仍然是模棱两可的
话,它不会猜测剩余引用的生命周期应该是什么.在这种情况,编译器会给出一个错误,这可以通过增加对应引
用之间相联系的生命周期标注来解决.函数或方法的参数的生命周期被称为输入生命周期 (input lifetimes)
,而返回值的生命周期被称为输出生命周期 (output lifetimes).编译器采用三条规则来判断引用何时不
需要明确的标注.第一条规则适用于输入生命周期,后两条规则适用于输出生命周期.如果编译器检查完这三条
规则后仍然存在没有计算出生命周期的引用,编译器将会停止并生成错误.这些规则适用于 fn 定义,以及
impl 块.
```

```markdown
1. 第一条规则是每一个是引用的参数都有它自己的生命周期参数.换句话说就是,有一个引用参数的函数有一
   个生命周期参数: fn foo<'a>(x: &'a i32),有两个引用参数的函数有两个不同的生命周期参数,fn
   foo<'a,'b>(x: &'a i32, y: &'b i32),依此类推.
2. 第二条规则是如果只有一个输入生命周期参数,那么它被赋予所有输出生命周期参数: fn foo<'a>(x:
   &'a i32) -> &'a i32.
3. 第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self,说明是个
   对象的方法(method), 那么所有输出生命周期参数被赋予 self 的生命周期.第三条规则使得方法更容
   易读写,因为只需更少的符号.
```

### 方法定义中生命周期标注

```markdown
当为带有生命周期的结构体实现方法时,其语法依然类似泛型类型参数的语法.声明和使用生命周期参数的位置
依赖于生命周期参数是否同结构体字段或方法参数和返回值相关.(实现方法时)结构体字段的生命周期必须总是
在 impl 关键字之后声明并在结构体名称之后被使用,因为这些生命周期是结构体类型的一部分.impl 块里的
方法签名中,引用可能与结构体字段中的引用相关联,也可能是独立的.另外,生命周期省略规则也经常让我们无需
在方法签名中使用生命周期标注.
```

```rust

#![allow(unused)]
fn main() {
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }

    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }
}
```

```markdown
对于第一个实现,我们为 ImportantExcerpt 结构体的 level 方法声明了生命周期参数 'a.这个生命周
期参数,其唯一参数是 self 引用但是返回值是一个 i32 不存在引用.因此,编译器会自动为 level 方法的
声明省略生命周期参数,符合第一条生命周期省略规则.第二个实现依照同样的规则一为方法的两个输入参数声
明了各自的生命周期参数,并返回一个 &str.而第一个参数是 self 引用,因此生命周期参数 'a 被赋予给
返回值,这样符合第三条生命周期省略规则并且针对输入与输出生命周期参数都确定了.
```

### 静态生命周期

```markdown
静态生命周期 'static 表示该引用的生命周期将比所有其他可能的生命周期更长.它会持续在程序运行期
间.所有的字符串字面量都拥有 'static 生命周期.IDE可能在错误信息的帮助文本中提示 'static 生
命周期的建议，不过将引用指定为 'static 之前需思考一下这个引用是否真的在整个程序的生命周期里都
有效.也许要考虑是否希望它存在得这么久,即使这是可能的但大部分情况,代码中的问题是尝试创建一个悬垂
引用或者可用的生命周期不匹配,请解决这些问题而不是指定一个 'static 的生命周期.
```

### 结合泛型类型参数、trait bounds 和生命周期

```rust
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() { x } else { y }
}
```