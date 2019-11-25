# Modern operating path system (MOPS)
## C++
Код на плюсах изолирован в src/corelib/cpp.
Там же для запуска отдельно выделен файл с загаловками stdc++.h, т.к. он не стандартный.
Чтобы сделать функцию доступной из вне, нужно добавить extern "C" перед ней.
## Building
Чтобы запуститься, в package выполняем cargo run;
Файл build.rs компилирует c++ код через vs compiler (cc crate) в object файл.
Файл src/corelib/mod.rs линкует c++ к расту.
## Links
*https://github.com/BrianPeek/legoev3
*https://sourceforge.net/projects/mingw-w64/
*http://proghouse.ru/programming/60-ev3-remote-controlhttp://proghouse.ru/programming/60-ev3-remote-control
*https://www.python.org/
*http://ev3directcommands.blogspot.com/
