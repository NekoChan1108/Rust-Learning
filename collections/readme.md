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

## String 字符串

```markdown
Rust 的核心语言中只有一种字符串类型 str,字符串 slice,它通常
以被借用的形式 &str 出现.Rust 标准库中还包含一系列其他字符串
类型,比如 OsString、OsStr、CString 和 CStr.
```

### 新建字符串

```markdown
字符串的创建方式可以分为一下几种:
```

```rust
#[allow(unused)]
fn main() {
    let mut s = String::new(); // 创建一个空字符串
    let data = "initial contents"; // 创建一个字符串字面量
    let s = data.to_string(); // 字符串字面量转换为 String
    let s = String::new("hello"); // 通过String::new结合字符串面量创建一个字符串
}
```

### 更新字符串

```markdown
可以使用 push_str 方法来将字符串追加到已有的字符串中.
或者使用 push 方法来将单个字符追加到字符串中.采用 pop
方法来移除字符串末尾的字符.
```

```rust
#![allow(unused)]
fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");
    s.push('!');
    println!("{}", s); // hello, world!
    s.pop();
    println!("{}", s); // hello, world
}
```

```markdown
另外也可以使用 + 运算符或者 format! 宏来将字符串连接起来.
```

```rust
#![allow(unused)]

use std::fmt::format;
use std::ops::Add;

fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    println!("{}", s1 + &s2); // println!("{}", String::add(s1, &s2))
    let s1 = String::from("hello");
    let s3 = format!("{}---{}", s1, s2);
    println!("{:?}", s3); // "hello---world"
}
```

```markdown
对于 + 运算符,Rust 会将 s1 的所有权移动给 s1 + &s2,因为 + 的底层
逻辑是 String 下的 add 方法,add 方法的参数 s1 是 self 不带引用,
所以 Rust 会将其的所有权移动给 add 方法.之所以能够在 add 调用中使用
&s2 是因为 &String 可以被强转 (coerced) 成 &str.当 add 函数被调用时
Rust 使用了一个被称为 解引用强制转换 (deref coercion) 的技术,可以将其
理解为它把 &s2 变成了 &s2[..]. 与 println! 宏一样, format! 宏也是接收
引用的,所以在这里传 s1、s2 参数时,Rust 不会将 s1 和 s2 的所有权移动给
format!
```

### 字符串索引

```markdown
在很多语言中,通过索引来引用字符串中的单独字符是有效且常见的操作.然而在 Rust 中,
如果你尝试使用索引语法访问 String 的一部分,会出现一个错误.String 是一个 Vec<u8>
的封装,Vec<u8> 中的元素是字节,而 String 中的元素是字符,对于字符串中的某个字符,
其不一定是一个字节,所以索引语法无法提供有效的结果,从 Rust 的角度来讲,事实上有三
种相关方式可以理解字符串:字节、标量值和字形簇(最接近人们眼中 字母 的概念).
```

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("Здравствуйте"); // 24字节 因为每个 Unicode 标量值需要 2 个字节存储
    println!("{} {}", s1.len(), s2.len()); // 5 24
}
 ```

```markdown
最后一个 Rust 不允许使用索引获取 String 字符的原因是索引操作预期总是需要
常数时间 O(1).但是对于 String 不可能保证这样的性能,因为 Rust 必须从开头
到索引位置遍历来确定有多少有效的字符.
```

### 字符串 slice

```markdown
索引字符串通常是一个坏点子,因为字符串索引应该返回的类型是不明确的:字节值、
字符、字形簇或者字符串 slice.因此,如果真的希望使用索引创建字符串 slice 时
,Rust 会要求你更明确一些.为了更明确索引并表明你需要一个字符串 slice,相比使
用 [] 和单个值的索引,可以使用 [] 和一个 range 来创建含特定字节的字符串 slice
不过对于之前的非单字符 1 字节的字符串,传入错误的索引范围,Rust 会 panic .
```

```rust
fn main() {
    let s = String::from("hello world");
    let s1 = String::from("Здравствуйте");
    println!("{}", &s[0..5]);
    println!("{}", &s1[0..1]); // panic
}
```

### 遍历字符串的几种方法

```markdown
如果你需要操作单独的 Unicode 标量值,最好的选择是使用 chars 方法,
也可以使用 bytes 方法返回每一个原始字节,不过请记住有效的 Unicode
标量值可能会由不止一个字节组成
```

```rust
fn main() {
    for c in "Здравствуйте".chars() {
        println!("{}", c);
    }
    for b in "Здравствуйте".bytes() {
        println!("{}", b);
    }
}
```

## 哈希 map

```markdown
HashMap<K, V> 类型储存了一个键类型 K 对应一个值类型 V 的映射.它通过一个
哈希函数 (hashing function) 来实现映射,决定如何将键和值放入内存中.
```

### 新建哈希 map

```markdown
有两种方法来新建一个哈希 map,一种是使用 HashMap::new 函数,另一个构建哈希 map
的方法是使用一个元组的 vector 的 collect 方法,其中每个元组包含一个键值对.collect
方法可以将数据收集进一系列的集合类型,包括 HashMap.例如,如果队伍的名字和初始分数分别
在两个 vector 中,可以使用 zip 方法来创建一个元组的 vector,接着就可以使用 collect
方法将这个元组 vector 转换成一个 HashMap
```

```rust
use std::collections::HashMap;

fn main() {
    let mut hash_map: HashMap<String, String> = HashMap::new();
    hash_map.insert("key".to_string(), "value".to_string());
    println!("{:#?}", hash_map);
    println!("{:#?}", hash_map.get("key"));

    println!("************************************************");

    let team: Vec<String> = vec!["Alice".to_string(), "Bob".to_string()];
    let team_score: Vec<i32> = vec![10, 20];
    let team_score_map: HashMap<_, _> = team.iter().zip(team_score.iter()).collect();
    println!("{:#?}", team_score_map);
    println!("{:#?}", team_score_map.get(&"Alice".to_string()));
    println!("{:#?}", team_score_map.get(&"Bob".to_string()));
}
```

```markdown
这里 HashMap<_, _> 类型标注是必要的,因为 collect 有可能当成多种不同的数据结构,
而除非显式指定否则 Rust 无从得知你需要的类型.但是对于键和值的类型参数来说,可以使
用下划线占位,而 Rust 能够根据 vector 中数据的类型推断出 HashMap 所包含的类型.
```

### 访问哈希 map 中的值

```markdown
可以通过 get 方法并提供对应的键来从哈希 map 中获取值,如果键不存在则返回 None
也可以通过遍历 map 来获取所有的键值对
```

```rust
use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    if score.is_some() {
        println!("{:?}", score.unwrap_or(&(0i32)));
    }

    let mut idx = 0;
    for (k, v) in &scores {
        idx += 1;
        println!("kv{}:{:?} {:?}", idx, k, v); // 和其他语言一样, map 的 kv 遍历是随机的
    }
}
```

### 哈希 map 所有权

```markdown
对于像 i32 这样的实现了 Copy trait 的类型,其值可以拷贝进哈希 map.对于像 String
这样拥有所有权的值,其值将被移动而哈希 map 会成为这些值的所有者,如果后续想要再次访问
请插入克隆 (clone) 或者把 map 的 kv 类型参数改为 &String 或者其他引用.
```

```rust
use std::collections::HashMap;

fn main() {
    let mut hash_map: HashMap<&String, &String> = HashMap::new();
    let key = "key".to_string();
    let value = "value".to_string();
    hash_map.insert(&key, &value);
    println!("{:?}", hash_map);
    println!("{:?}", hash_map.get(&key));
    println!("key: {} value: {}", key, value);
    println!("************************************");
    let mut hash_map1: HashMap<String, String> = HashMap::new();
    hash_map1.insert(key, value);
    println!("{:?}", hash_map1);
    println!("{:?}", hash_map1.get(&key)); // error
    println!("key: {} value: {}", key, value); // error
}
```

### 更新哈希 map

```markdown
可以使用 insert 方法来更新哈希 map 中的值,如果键已经存在则更新,否则插入.
这种方式称为覆盖,或者也可以采取判断键是否存在来决定是插入还是更新,如果键不
存在则插入,否则什么也不做.
```

```rust
use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // scores.insert(String::from("Blue"), 100);
    scores.entry(String::from("Blue")).or_insert(100); // or_insert 函数返回一个该键对应值的 mut 引用
    let mut idx = 0;
    for (k, v) in &scores {
        idx += 1;
        println!("kv{}:{:?} {:?}", idx, k, v);
    }
}
```

```markdown
也可以根据旧值更新值,比如在键对应的值加 1,以下这段代码会打印出
{"world": 2, "hello": 1, "wonderful": 1}，or_insert 方法
事实上会返回这个键的值的一个可变引用 (&mut V) .这里将这个可变
引用储存在 count 变量中,所以为了赋值必须首先使用星号 * 解引用
count .这个可变引用在 for 循环的结尾离开作用域,这样所有这些改
变都是安全的并符合借用规则.
```

```rust
use std::collections::HashMap;
fn main() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_ascii_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map);
}
```

### 哈希函数

```markdown
HashMap 默认使用一种 "密码学安全的" (cryptographically strong) 哈希函数,
它可以抵抗拒绝服务 (Denial of Service, DoS) 攻击.然而这并不是可用的最快的
算法,不过为了更高的安全性值得付出一些性能的代价.如果性能监测显示此哈希函数非
常慢,以致于你无法接受,你可以指定一个不同的 hasher 来切换为其它函数.
```