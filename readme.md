# MOPS
Modern operating path system (MOPS)
## C++
Код на плюсах изолирован в src/corelib/cpp.
Там же для запуска отдельно выделен файл с загаловками stdc++.h, т.к. он не стандартный.
Чтобы сделать функцию доступной из вне, нужно добавить extern "C" перед ней.
## Building
In package dir: `cargo run` or `cargo build` if you want only to build. In this process build.rs will compile c++ into object using vs compiler (cc crate). File src/corelib/mod.rs externs c++.
## Links

* https://github.com/BrianPeek/legoev3

* https://sourceforge.net/projects/mingw-w64/

* http://proghouse.ru/programming/60-ev3-remote-controlhttp://proghouse.ru/programming/60-ev3-remote-control

* https://www.python.org/

* http://ev3directcommands.blogspot.com/
