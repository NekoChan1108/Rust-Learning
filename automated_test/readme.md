# 编写自动化测试

```markdown
编写自动化测试很简单,只需要在测试函数上添加 # [test] 注解,并且保证函数不接收参数.测试用例如下 :
```

```rust
#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3);
}
```

```shell
cargo test
```

```markdown
只需要运行 cargo test 就可以运行测试了这会测试当前终端的路径下的所有使用 #[test] 注解的函数,同
时可以采用 assert_eq! 等断言宏来断言测试结果或者自定义断言错误信息.
```

```rust
#[test]
fn test_add() {
    assert_eq!(
        crate::add(1, 2),
        2,
        "1+2 should equals to 3 not {}",
        2
    );
}
```

```markdown
也可以采用 #[should_panic] 注解来判断测试结果是否正常 panic.这个宏需要放在测试函数上并且在
#[test] 注解之后.
```

```rust
#[test]
#[should_panic]
fn test_panic() {
    panic!("this should panic");
}
```

```markdown
通常上述方式的返回结果会过于模糊,这时我们可以使用 #[should_panic(expected = "panic message")]
注解来指定 panic 的错误信息.
```

```rust
#[test]
#[should_panic(expected = "this should panic")]
fn test_panic() {
    panic!("this should panic");
}
```

```markdown
这样如果测试结果不按照预期 expected 输出,测试就会失败.这样就能更好的定位测试结果而不是模糊的
panic 显示 ok. 参考之前的 Result 类型,我们也可以将 Result<T, E> 错误类型返回给测试函数,
那这样就不能使用 #[should_panic] 注解了,因为该注解不接受返回值,换成 Result<T, E> 的好处是
可以获取错误信息而不是直接 panic. 要断言操作返回 Err 值,不要在 Result<T, E> 值上使用问号运
算符.相反,请使用 assert!(value.is_err()),例子如下 :
```

```rust
 #[test]
fn it_works() -> Result<(), String> {
    if 2 + 2 == 3 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}
```

## 额外的常用测试参数

```shell
cargo test -- --test-threads=n #运行测试时指定线程数 n 是线程的数量
cargo test -- --show-output #显示测试结果
cargo test -- --ignored #运行所有被 #[ignore] 注解忽略的测试
cargo test fn_name/mod_name #运行指定测试 例如 cargo test test_add
cargo test some_string #运行包含指定内容的测试
cargo test --help #查看 cargo test 命令的帮助信息
```

## 测试的组织结构

```markdown
Rust 社区倾向于根据测试的两个主要分类来考虑问题: 单元测试 (unit tests) 与 集成测试
(integration tests).单元测试倾向于更小而更集中,在隔离的环境中一次测试一个模块,或者
是测试私有接口.而集成测试对于你的库来说则完全是外部的.它们与其他外部代码一样,通过相同的
方式使用你的代码,只测试公有接口而且每个测试都有可能会测试多个模块.
```

### 单元测试

```markdown
单元测试的目的是在与其他部分隔离的环境中测试每一个单元的代码,以便于快速而准确的某个单元的代码功
能是否符合预期.单元测试与他们要测试的代码共同存放在位于 src 目录下相同的文件中.规范是在每个文件
中创建包含测试函数的 tests 模块,并使用 #[cfg(test)] 标注模块.测试模块的 #[cfg(test)] 标注
告诉 Rust 只在执行 cargo test 时才编译和运行测试代码,而在运行 cargo build 时不这么做.这在
只希望构建库的时候可以节省编译时间,并且因为它们并没有包含测试,所以能减少编译产生的文件的大小.与
之对应的集成测试因为位于另一个文件夹,所以它们并不需要 #[cfg(test)] 标注.然而单元测试位于与源
码相同的文件中,所以你需要使用 #[cfg(test)] 来指定他们不应该被包含进编译结果中.
```

### 集成测试

```markdown
在 Rust 中,集成测试对于你需要测试的库来说完全是外部的.同其他使用库的代码一样使用库文件,也就是
说它们只能调用一部分库中的公有 API .集成测试的目的是测试库的多个部分能否一起正常工作.一些单独
能正确运行的代码单元集成在一起也可能会出现问题,所以集成测试的覆盖率也是很重要的.为了编写集成测
试,需要在项目根目录创建一个 tests 目录,与 src 同级.Cargo 知道如何去寻找这个目录中的集成测试
文件.接着可以随意在这个目录中创建任意多的测试文件,Cargo 会将每一个文件当作单独的 crate 来编译.
与单元测试不同,我们需要在文件顶部添加引用。这是因为每一个 tests 目录中的测试文件都是完全独立的
crate,所以需要在每一个文件中导入库.
```

```rust
use adder;
#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

```markdown
随着集成测试的增加,你可能希望在 tests 目录增加更多文件以便更好的组织他们,例如根据测试的功能来
将测试分组.正如我们之前提到的,每一个 tests 目录中的文件都被编译为单独的 crate. 将每个集成测
试文件当作其自己的 crate 来对待,这更有助于创建单独的作用域,这种单独的作用域能提供更类似与最终
使用者使用 crate 的环境.tests 目录中的文件不能像 src 中的文件那样共享相同的行为.当你有一些
在多个集成测试文件都会用到的帮助函数,将他们提取到一个通用的模块中时,tests 目录中不同文件的行
为就会显得很明显.例如,如果我们可以创建一个tests/common.rs 文件并创建一个名叫 setup 的函数
,我们希望这个函数能被多个测试文件的测试函数调用:
```

```rust
#[allow(unused)]
pub fn setup() {}
 ```

```markdown
进一步将其封装到一个模块中,例如 tests/common/mod.rs,就会发现这个模块中的所有内容都会被 tests
目录中的所有文件共享.然后不参与到测试的文件中了,因为 Rust 只认 tests 下的文件为测试文件,这些文
件并不包括子文件夹里的文件.如果项目是二进制 crate 并且只包含 src/main.rs 而没有 src/lib.rs,
这样就不可能在 tests 目录创建集成测试并使用 use 语句导入 src/main.rs 中定义的函数.只有库
crate 才会向其他 crate 暴露了可供调用和使用的函数;二进制 crate 只意在单独运行.这就是 Rust
二进制项目明确采用 src/main.rs 调用 src/lib.rs 中的逻辑的原因之一.通过这种结构,集成测试就
可以通过 use 测试库 crate 中的主要功能了,而如果这些重要的功能没有问题的话,src/main.rs 中的
少量代码也就会正常工作且不需要测试.
```