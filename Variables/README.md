标量（scalar）类型代表一个单独的值。Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字 
符类型。你可能在其他语言中见过它们。让我们深入了解它们在 Rust 中是如何工作的。
整型
整数 是一个没有小数部分的数字。我们在第二章使用过 u32 整数类型。该类型声明表明，它关联的 
值应该是一个占据 32 比特位的无符号整数（有符号整数类型以 i 开头而不是 u ）。表格 3-1 展示 
了 Rust 内建的整数类型。我们可以使用其中的任一个来声明一个整数值的类型。

整型溢出
比方说有一个 u8 ，它可以存放从零到 255 的值。那么当你将其修改为 256 时会发生什么 
呢？这被称为 “整型溢出”（“integer overﬂow” ），这会导致以下两种行为之一的发生。当在
debug 模式编译时，Rust 检查这类问题并使程序  panic，这个术语被 Rust 用来表明程序因错误 
而退出。第九章  “ panic! 与不可恢复的错误” 部分会详细介绍 panic。
在 release 构建中，Rust 不检测溢出，相反会进行一种被称为二进制补码包装（two’s 
complement wrapping）的操作。简而言之，值    256 变成    0 ，值    257 变成    1 ，依此类推。依 
赖整型溢出被认为是一种错误，即便可能出现这种行为。如果你确实需要这种行为，标准库中 
有一个类型显式提供此功能，  Wrapping 。  为了显式地处理溢出的可能性，你可以使用标准库 
在原生数值类型上提供的以下方法:
所有模式下都可以使用    
1.wrapping_* 方法进行包装，如wrapping_add 
2.如果check_* 方法出现溢出，则返回None 值
3.用overflowing_* 方法返回值和一个布尔值，表示是否出现溢出 
4.用saturating_* 方法在值的最小值或最大值处进行饱和处理