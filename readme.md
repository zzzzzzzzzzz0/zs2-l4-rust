+ 式微，式微！胡不归？
+ 不行了，不行了，你怎么还在写程序？

如中文脚本 c++ 版一般用脚本直接调用动态库

有所简化，如对 linux 系统基础标准库 libstdc++ 的接口脚本由

```
模块“标准库”。

赋予“句柄”【私】以函数集
“libstdc++.so.6.0.13”、
“libstdc++.so.6”、
“libstdc++-libc6.1-1.so.2”、
“libstdc++.dylib”。

函数“格输出”以“printf”、‘句柄’、“i-c*”。
（……）
```
改为

```
模块“标准库”。

函数集
“libstdc++.so.6.0.13”、
“libstdc++.so.6”、
“libstdc++-libc6.1-1.so.2”、
“libstdc++.dylib”。

函数“格输出”以“printf”、“i-c*”。
（……）
```

使内嵌的 clpars （命令行解析）也如 c++ 版一般支持正则表达式作 flag 名，并有属性 flag、rem、type、argc、code、tag、no（独有，匹配中的序号）

纯库，供公用，目前是 l4-rust 和 zsp2-rust