# 集合

```markdown
大部分其他数据类型都代表一个特定的值,不过集合可以包含多个值.
不同于内建的数组和元组类型,这些集合指向的数据是储存在堆上的,
这意味着数据的数量不必在编译时就已知,并且还可以随着程序的运
行增长或缩小.每种集合都有着不同功能和成本,而根据当前情况选择
合适的集合.以下是三个在 Rust 程序中被广泛使用的集合:

1. vector 允许我们一个挨着一个地储存一系列数量可变的值
2. 字符串 (string) 字符的集合
3. 哈希 map (hash map) 允许我们将值与一个特定的键 (key) 相关联
```

## vector

```markdown
vector 允许我们在一个单独的数据结构中储存多个值,所有值在内存中彼此相邻排列,
vector 只能储存相同类型的值.
```

### 新建 vector

```markdown
vector 可以通过两种方式来创建: 一种是使用 Vec::new() 创建一个空的 vector,
可以指定 vector 的元素类型,另一种是使用 vec! 宏创建一个 vector,宏会根据第一
个元素的类型来推导出 vector 的元素类型.
```

```rust
#[allow(unused)]
fn main() {
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
}
```

### 更新 vector

```markdown
vector 的元素可以通过索引访问和修改.也可以通过 push(element) 方法来添加元素,或
通过 pop() 方法来移除尾部元素.
```

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);
    v[1] = 6;
    v.pop();
    println!("{:?}", v); // [1, 6, 3, 4]
}
```

### 丢弃 vector

```markdown
类似于任何其他的 struct,vector 在其离开作用域时会被释放.
```

### 读取 vector

```markdown
vector 的元素读取方式有两种,索引语法或者 get 方法:
```

```rust
#![allow(unused)]
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(no3) => println!("The third element is {}", no3),
        None => println!("There is no third element."),
    }
}
```

```markdown
两个不同的获取第三个元素的方式分别为: 使用 & 和 [] 返回一个引用;或者使用 get 方法
以索引作为参数来返回一个 Option<&T>. Rust 有两个引用元素的方法的原因是程序可以选
择如何处理当索引值在 vector 中没有对应值的情况
```

```rust
#![allow(unused)]
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let does_not_exist = v.get(100);
    println!("{:?}", does_not_exist); // None
    let does_not_exist = &v[100];
    println!("{}", does_not_exist); // panic
}
```

```markdown
一旦程序获取了一个有效的引用,借用检查器将会执行所有权和借用规则
来确保 vector 内容的这个引用和任何其他引用保持有效.下面的代码
尝试在 vector 的元素被添加到 vector 之后获取 vector 的第一
个元素.在 vector 的结尾增加新元素时,在没有足够空间将所有所有元
素依次相邻存放的情况下,可能会要求分配新内存并将老的元素拷贝到新的
空间中.这时,第一个元素的引用就指向了被释放的内存,借用规则阻止程序
陷入这种状况,就像之前章节里的不可同时出现可变和不可变引用一样.
```

```rust
#[allow(unused)]
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);  // error
    println!("The first element is: {}", first);
}
```

### 遍历 vector 中的元素

```markdown
如果想要依次访问 vector 中的每一个元素,我们可以遍历其所有的元素
的引用而无需通过索引一次一个的访问
```

```rust
fn main() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    let mut v1 = vec![100, 32, 57];
    for i in &mut v1 {
        *i += 50;
    }
    println!("{:?}", v1); // [150, 82, 107]
}
```

### 使用枚举来存储多种类型

```markdown
vector 只能储存相同类型的值,这是很不方便的,绝对会有需要储存
一系列不同类型的值的用例.幸运的是,枚举的成员都被定义为相同的
枚举类型,所以当需要在 vector 中储存不同类型值时,我们可以定
义并使用一个枚举.
```

```rust
#![allow(unused)]
fn main() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for cell in row {
        match cell {
            SpreadsheetCell::Int(i) => println!("{}", i),
            SpreadsheetCell::Float(f) => println!("{}", f),
            SpreadsheetCell::Text(s) => println!("{}", s),
        }
    }
}
```

```markdown
Rust 在编译时就必须准确的知道 vector 中类型的原因在于它需要知道
储存每个元素到底需要多少内存.第二个好处是可以准确的知道这个 vector
中允许什么类型.如果 Rust 允许 vector 存放任意类型,那么当对 vector
元素执行操作时一个或多个类型的值就有可能会造成错误.使用枚举外加 match
意味着 Rust 能在编译时就保证总是会处理所有可能的情况.
```