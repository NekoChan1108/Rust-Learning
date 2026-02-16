# minigrep

```markdown
一个简单的 I/O 项目,实现 grep 命令,用来复习之前的内容
```

## 错误的输出

```markdown
大部分终端都提供了两种输出: 标准输出 (standard output,stdout) 对应一般信息,标准错误
(standard error,stderr）则用于错误信息.这种区别允许用户选择将程序正常输出定向到一个文
件中并仍将错误信息打印到屏幕上.但是 println! 函数只能够打印到标准输出,所以我们必须使用其
他方法来打印到标准错误.
```
```shell
cargo run > log.txt
```
```markdown
运行 cargo run > log.txt 会将标准输出重定向到一个文件中,但是标准错误不会.如果我们将 
println! 函数替换为 eprintln! 函数,那么错误信息就会打印到标准错误中而不会重定向输出
到 log.txt 输出,相反正确的结果会重定向到 log.txt 中.
```