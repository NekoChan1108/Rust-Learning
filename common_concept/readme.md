# 一些 Rust 基本概念

## 1.变量和可变性

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

```markdown
这段代码在编译的时候就会报错,因为 x 是不可变的,不能被赋值
正确的代码如下:
```

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

### 1.1 常量声明

```markdown
常量声明和变量声明一样,但是常量的值不能被修改,常量的值必须在声明
的时候赋值并且明确类型,采用 const 关键字
```

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

### 1.2 变量遮蔽 (shadowing)

```markdown
变量遮蔽允许我们在声明变量的时候,将一个变量的值修改为另一个
不同类型的值或者不改变类型只改变其值
```

```rust
use std::any::type_name_of_val;
fn main() {
    let x = 5;
    println!("{} {}", x, type_name_of_val(&x));
    let x = x + 1;
    println!("{} {}", x, type_name_of_val(&x));
    let x: bool = true;
    println!("{} {}", x, type_name_of_val(&x));
    let x: i32 = 42;
    println!("{} {}", x, type_name_of_val(&x));
}
 ```

```markdown
输出: 5 i32
6 i32
true bool
42 i32
```

```markdown
变量遮蔽不跨越块级作用域,仅限一个块级作用域内,例如:
```

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = "hello";
        println!("{}", x); // hello
    }
    println!("{}", x); // 6
}
```

```markdown
变量遮蔽不要求原来的变量是可变的,相反,如果在执行变量遮蔽时
按照可变变量的方式尝试修改一个可变的变量为其他类型的值,编译器在编译时就会警告
```

```rust
fn main() {
    let mut spaces = "   ";
    spaces = spaces.len(); // compile error  here : type mismatch expected `&str`, found `usize`
    println!("{}", spaces);
}
```

## 2.数据类型 [Rust 类型官方文档](https://rustwiki.org/zh-CN/book/ch03-02-data-types.html "Rust 类型官方文档")

```markdown
首先要明确一点,Rust 是强类型语言,类型在编译时就要被确定了,不能在运行时再改变类型
下面代码在编译时就会报错
```

```rust
use std::any::type_name_of_val;

fn main() {
    let guess = "42".parse().expect("Not a number!"); // compile error type annotations needed type must be known at this point
    println!("Guess: {} Type: {}", guess, type_name_of_val(&guess))
}
```

```markdown
正确的做法是给 guess 变量一个类型 例如 i32
```

### 2.1 标量类型

```markdown
标量类型是基本数据类型,Rust 中有 4 种标量类型:
整数(无符号整数、有符号整数)、浮点数、布尔值、字符
这里主要说一下字符型占 4 Bytes 
```

### 2.2 复合类型

```markdown
复合类型是多个标量类型的组合,Rust 中有 2 种复合类型:
元组(tuple)、数组(array),下面是示例:
```

```rust
fn main() {
    let tup: (i32, i32, bool, f64) = (10, 20, true, 10.5); // tuple
    println!("{:?}", tup); // (10, 20, true, 10.5)
    println!("{}", tup.0); // tuple index ".index" index>=0&&index<len
    println!("*************************");
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // len is 5 
    println!("{:?}", arr); // [1, 2, 3, 4, 5]
    println!("{}", arr[0]); // array index "[index]" index>=0&&index<len
    let arr = [5.5; 5];
    println!("{:?}", arr); // [5.5, 5.5, 5.5, 5.5, 5.5]
}
```

```markdown
值得注意的是如果是采用 let arr = [val;len] 的语法,那么 val 的类型会
被推导为数组的元素类型;其次就是数组下标索引的数据类型必须是 usize 类型
元组下表索引不可以用赋值给整数类型的变量只能是 0 1 2 len-1 的固定值
```

## 3. 函数

```markdown
函数的定义采用 fn 关键字,函数的名字采用小写加下划线的方式,例如 another_function
,返回值由箭头 -> 指定,函数的定义不需要一定在 main 函数前,只要定义了就可以调用,同名函数
不支持重载也即不能被多次定义
```

```rust
fn another_function() {
    println!("Another function.");
}
fn another_function1(x: i32) -> i32 {
    return x + 1;
}
```

### 3.1 语句和表达式

```markdown
表达式和语句很相似,但是表达式计算产生值并可以返回值,语句只能执行相关操作不能返回值,这两者最大的
区别是表达式结尾没有分号,语句后面带有分号,因此返回值返回时如果采取省略 return 关键字的话,
那么就必须去掉变量后面的分号,带上分号的话编译器就会报错将其当作语句执行而不是直接返回,例如:
```

```rust
fn main() {
    println!("{}", test_with_return(1));
    println!("{}", test_without_return(2));
}

fn test_with_return(x: i32) -> i32 {
    return x;
}
fn test_without_return(x: i32) -> i32 {
    x; // error[E0308]: mismatched types  implicitly returns `()` as its body has no tail or `return` expression
}
```

```markdown
Rust 不像其它语言一样支持 let x = (let y = 5); 这种语法将 y 的值赋给 x;
因为 Rust 是一个表达式语言,因此 Rust 在前述情况下会将 let y = 5; 当作一个
表达式执行而表达式不返回值,因此 x 是拿不到 y 的值的, 要解决上述问题可以采取 {}
形成一个块级作用域或者说一个块级表达式,例如:
```

```rust
fn main() {
    let x = {
        let y = 5;
        y + 1
    };
    println!("{}", x);
}
```

```markdown
输出: 6 块级作用域的表达式会返回块级作用域的最后一个表达式的值,因此 x 的值为 6
```

## 4. 注释

```markdown
注释语法和多数的语言一样,单行注释使用 // 或者 使用 /* */,
块级注释使用 /** */ 且必须在不为空的代码上方进行使用,下面是一些示例:
```

```rust
/**
 * This is a multi-line comment,and it must be used above a function or some valid code not blank
 */
fn main() {
    // This is a comment
    /* This is also a comment */
    // println!("Hello, world!"); 
}
```

```markdown
另外还有两种文档注释语法,一个是使用 ///, 当函数或作用域的
文档注释被使用时,文档注释会生成一个 HTML 文档,用于生成文档
的命令是 cargo doc,生成文档的 HTML 文件会保存在 target/doc
目录下, 这常被用于开发者将自己的包发布到 crates.io 供他人使用;
另一个是 //! 注释,这是一种包含注释项的文档注释语法,简单来说 //!
描述了它所在的整个文件而不是一个函数或者作用域,位于项之中的文档注释
//! 对于描述 crate 和模块特别有用,使用他们描述其容器整体的目的来
帮助 crate 用户理解你的代码组织
```

```rust
//!
//! this is a doc comment for the crate
//!
fn main() {
    println!("{}", example(1, 2));
}

///
/// # Example
/// ```
/// let answer = example(1,2);
/// assert_eq!(answer,3);
/// ```
///
fn example(a: i32, b: i32) -> i32 {
    a + b
}
```

## 5. 控制流

```markdown
Rust 中有条件判断和循环两种控制流,下面分别介绍:
```

### 5.1 if 条件判断控制流

```rust
// if
fn main() {
    let a = 5;
    if a > 10 {
        println!("a is greater than 10");
    } else if a < 10 {
        println!("a is less than 10");
    } else {
        println!("a is equal to 10");
    }
    // wrong code
    let number = 3;
    if number { // condition should be an expression that can be implicitly converted to `bool`
        println!("number was three");
    }
}
```

```markdown
在使用 if 语句来实现赋值的时候,两种条件下的值必须是相同的类型,
否则在编译时会报错,且这种情况下以前面的类型为准,例如:
```

```rust
fn main() {
    let condition = true;

    // 这里 else 的类型必须与 if 块的返回值类型相同 因为 number 的类型在编译时由 if 块的返回值类型决定
    let number = if condition { 5 } else { "six" }; // error[E0308]: `if` and `else` have incompatible types 

    println!("The value of number is: {number}");
}
```

### 5.2 循环控制流

```markdown
循环有三种: loop, while, for,可以采取 'xxx 来标记循环,
可以采用关键字 break 和 continue 来控制循环
```

#### 5.2.1 loop 循环

```rust
// infinite loop to stop with ctrl+c
fn main() {
    loop {
        println!("loop");
    }
}
// break to stop and  continue to  skip
fn main() {
    let mut count = 0;
    'counting_up: loop {
        count += 1;
        if count == 2 {
            continue; // skip
        }
        println!("x = {count}"); // 2 is not printed due to continue
        if count == 10 {
            break 'counting_up; // stop  loop
        }
    }
    // you can also apply the loop as a value to a variable
    count = 0;
    let mut result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };
    println!("The result is {result}"); // The result is 20

    count = 0;
    result = 'val_loop: loop {
        count += 1;
        if count == 10 {
            break 'val_loop  count * 2;
        }
    };
    println!("The result is {result}"); // The result is 20
}
```

#### 5.2.2 while 循环

```markdown
while 循环和其他语言一样,采用循环条件判断
```

```rust
fn main() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
 ```

#### 5.2.3 for 循环

```markdown
for 循环和 while 循环一样,但是 for 循环会自动处理索引变量
```

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```
