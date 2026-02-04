# 结构体

```markdown
和 C Golang 这些语言一样,Rust 也支持结构体并也采用关键字 struct
来声明结构体,接着,在大括号中,定义每一部分数据的名字和类型
```

```rust
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("{:?}", user1);
    let mut user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    user2.email = String::from("another@example.com");
    println!("{:?}", user2);
}
```

```markdown
为了从结构体中获取某个特定的值,可以使用点号.如果我们只想要用户的邮箱地址,可以用
user1.email.要更改结构体中的值,如果结构体的实例是可变的,我们可以使用点号并为
对应的字段赋值.注意整个实例必须是可变的,Rust 并不允许只将某个字段标记为可变.
```

## 变量与字段同名时的字段初始化简写语法

```markdown
当变量与字段同名时,我们可以使用简写语法,省略字段名,如下:
```

```rust
fn bind_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

## 使用结构体更新语法从其他实例创建实例

```markdown
使用旧实例的大部分值但改变其部分值来创建一个新的结构体实例通常很有用.
这可以通过结构体更新语法 (struct update syntax) 实现. .. 语法
指定了剩余未显式设置值的字段应有与给定实例对应字段相同的值. ..user1
必须放在最后,以指定其余的字段应从 user1 的相应字段中获取其值,但可以
选择以任何顺序为任意字段指定值,而不用考虑结构体定义中字段的顺序.
```

```rust
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("{:?}", user2);
}
```

```markdown
请注意,结构更新语法就像带有 = 的赋值,因为它移动了数据,就像移动部分讲到的一样.在这个例子中,
在创建 user2 后不能再使用 user1,因为 user1 的 username 字段中的 String 被移到 user2 中.
如果给 user2 的 email 和 username 都赋予新的 String 值,从而只使用 user1 的 active 和
sign_in_count 值,那么 user1 在创建 user2 后仍然有效.active 和 sign_in_count 的类型是
实现 Copy trait 的类型,所以克隆部分讨论的行为同样适用.
```

## 使用没有命名字段的元组结构体来创建不同的类型

```markdown
元组结构体有着结构体名称提供的含义,但没有具体的字段名,只有字段的类型.当你想给整个元组取一个名字,
并使元组成为与其他元组不同的类型时,元组结构体是很有用的
```

```rust
use std::any::type_name_of_val;

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Point: {}, {}, {}", origin.0, origin.1, origin.2);
    println!("Color: {}, {}, {}", black.0, black.1, black.2);
    println!("{}", type_name_of_val(&origin)); // structures::Point
    println!("{}", type_name_of_val(&black))  // structures::Color
}

```

```markdown
black 和 origin 值的类型不同,因为它们是不同的元组结构体的实例.定义的每一个结构体有其自己的类型,
即使结构体中的字段有着相同的类型.Color 类型参数的函数不能接受 Point 作为参数,即便这两个类型都由
三个 i32 值组成.在其他方面.元组结构体实例类似于元组,可以将其解构为单独的部分,也可以使用 . 后跟索
引来访问单独的值,等等.
```

## 没有任何字段的类单元结构体

```markdown
类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用
有点类似元组类型里的 unit 类型 ()
```

```rust
use std::type_name_of_val;
#[derive(Debug)]
struct AlwaysEqual;
fn main() {
    let subject = AlwaysEqual;
    println!("{:?}", subject);
    println!("{}", type_name_of_val(&subject));
}
```

## 结构体数据的所有权

```markdown
示例 中的 User 结构体的定义中,使用了自身拥有所有权的 String 类型而不是 &str 字符串 slice 类型.
这是一个有意而为之的选择,因为我们想要这个结构体拥有它所有的数据,为此只要整个结构体是有效的话其数据
也是有效的.也可以使结构体存储被其他对象拥有的数据的引用,不过这么做的话需要用上生命周期(lifetime)
生命周期确保结构体引用的数据有效性跟结构体本身保持一致.如果你尝试在结构体中存储一个引用而不指定生命
周期将是无效的,比如这样:
```

```rust
struct User {
    active: bool,
    username: &str, // error : lifetime required
    email: &str, // error : lifetime required
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
```

## 方法语法

```markdown
方法是一种定义在结构体或枚举类型中的函数,方法与普通函数定义的唯一区别是方法第一个参数是 self,
因为它在结构体的上下文中被定义 (或者是枚举或 trait 对象的上下文),self 代表调用该方法的结构体实例
为了使函数定义于 Rectangle 的上下文中,我们开始了一个 impl 块 (impl 是 implementation 的缩写),
这个 impl 块中的所有内容都将与 Rectangle 类型相关联.
```

```rust
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}
fn main() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    rect1.set_width(10);
    println!("The width of the rectangle is {}", rect1.width);
    println!("The area of the rectangle is {} square pixels.", rect1.area());
}
```

```markdown
在 area 的签名中,使用 &self 来替代 rectangle: &Rectangle,&self 实际上是 self: &Self 的缩写.
在一个 impl 块中,Self 类型是 impl 块的类型的别名.方法的第一个参数必须有一个名为 self 的 Self 类
型的参数,所以 Rust 让你在第一个参数位置上只用 self 这个名字来缩写.注意,仍然需要在 self 前面使用 &
来表示这个方法借用了 Self 实例,就像在 rectangle: &Rectangle 中做的那样.方法可以选择获得 self 的
所有权,或者像这里一样不可变地借用 self,或者可变地借用 self,就跟其他参数一样. 这里选择 &self 的理由
跟在函数版本中使用 &Rectangle 是相同的: 并不想获取所有权,只希望能够读取结构体中的数据,而不是写入.如
果想要在方法中改变调用方法的实例,需要将第一个参数改为 &mut self.通过仅仅使用 self 作为第一个参数来使
方法获取实例的所有权是很少见的,这种技术通常用在当方法将 self 转换成别的实例的时候,这时我们想要防止调用
者在转换之后使用原始的实例. 使用方法替代函数,除了可使用方法语法和不需要在每个函数签名中重复 self 的类
型之外,其主要好处在于组织性,将某个类型实例能做的所有事情都一起放入 impl 块中,而不是让将来的用户在库中
到处寻找 Rectangle 的功能.
```

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
#[derive(Debug)]
struct XXXX {
    width: u32,
    height: u32,
    length: u32,
}

impl Rectangle {
    fn turn_xxx(self) -> XXXX {
        XXXX {
            width: self.width,
            height: self.height,
            length: self.width,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 20,
        height: 20,
    };
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:?}", rect1);
    println!("area is {:#?}", rect1.area());
    println!("rect1 can hold rect is {:?}", rect1.can_hold(&rect));
    let xxx = rect.turn_xxx();
    println!("rect turn_xxx is {:#?}", xxx);
    println!("{:#?}", rect) // error 这里rect的所有权已经被转移了防止继续使用rect
}

```

```markdown
与字段同名的方法将被定义为只返回字段中的值,而不做其他事情.这样的方法被称为 getters,
Rust 并不像其他一些语言那样为结构字段自动实现它们.Getters 很有用,因为你可以把字段
变成私有的,但方法是公共的,这样就可以把对字段的只读访问作为该类型公共 API 的一部分.
```

## 自动解引用

```rust
#![allow(unused)]
fn main() {
    #[derive(Debug, Copy, Clone)]
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        fn distance(&self, other: &Point) -> f64 {
            let x_squared = f64::powi(other.x - self.x, 2);
            let y_squared = f64::powi(other.y - self.y, 2);

            f64::sqrt(x_squared + y_squared)
        }
    }
    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 5.0, y: 6.5 };
    p1.distance(&p2);
    (&p1).distance(&p2);
}
```

```markdown
当使用 object.something() 调用方法时,Rust 会自动为 object 添加 &、&mut 或 *
以便使 object 与方法签名匹配.也就是说上面代码段的 p1.distance(&p2) 等价于
(&p1).distance(&p2). 这种自动引用的行为之所以有效,是因为方法有一个明确的接收者
self 的类型.在给出接收者和方法名的前提下,Rust 可以明确地计算出方法是仅仅读取 (&self)
做出修改 (&mut self) 或者是获取所有权 (self)
```

## 关联函数

```markdown
所有在 impl 块中定义的函数被称为关联函数 (associated function),因为它们与 impl
后面命名的类型相关.我们可以定义不以 self 为第一参数的关联函数 (因此不是方法),因为它
们并不作用于一个结构体的实例.之前已经使用了一个这样的函数,String::from 函数,它是在
String 类型上定义的.关联函数经常被用作返回一个结构体新实例的构造函数,相当于 java 的
new.使用结构体名和 :: 语法来调用这个关联函数:比如 let sq = Rectangle::square(3);
这个方法位于结构体的命名空间中 :: 语法用于关联函数和模块创建的命名空间.
```

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let sq = Rectangle::square(3);
    println!("{:#?}", sq)
}
```

## 多 impl 块
```markdown
每个结构体都允许拥有多个 impl 块.但每个方法有其自己的 impl 块.
```
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

# 总结
```markdown
结构体让你可以创建出在你的领域中有意义的自定义类型.通过结构体,可以将相关联的数据片段联系起来并命名它们,
这样可以使得代码更加清晰.在 impl 块中.你可以定义与你的类型相关联的函数,而方法是一种相关联的函数,让你指
定结构体的实例所具有的行为.但结构体并不是创建自定义类型的唯一方法,你也可以使用枚举来创建自定义类型.
```