# MOPS
Modern operating path system (MOPS)
## C++
Код на плюсах изолирован в src/corelib/cpp.
Там же для запуска отдельно выделен файл с заголовками stdc++.h, т.к. он нестандартный.
Чтобы сделать функцию доступной извне, нужно добавить extern "C" перед ней.
## Building
In package dir: `cargo run` or `cargo build` if you want only to build. In this process build.rs will compile c++ into object using vs compiler (cc crate). File src/corelib/mod.rs externs c++.
## Links

* https://github.com/BrianPeek/legoev3

* https://sourceforge.net/projects/mingw-w64/

* http://proghouse.ru/programming/60-ev3-remote-controlhttp://proghouse.ru/programming/60-ev3-remote-control

* https://www.python.org/

* http://ev3directcommands.blogspot.com/

## TODO
System commands:

- [ ] BEGIN_DOWNLOAD                0x92    // Begin file download
- [ ] CONTINUE_DOWNLOAD             0x93    // Continue file download
- [ ] BEGIN_UPLOAD                  0x94    // Begin file upload
- [ ] CONTINUE_UPLOAD               0x95    // Continue file upload
- [ ] BEGIN_GETFILE                 0x96    // Begin get bytes from a file (while writing to the file)
- [ ] CONTINUE_GETFILE              0x97    // Continue get byte from a file (while writing to the file)
- [ ] CLOSE_FILEHANDLE              0x98    // Close file handle
- [ ] LIST_FILES                    0x99    // List files
- [ ] CONTINUE_LIST_FILES           0x9A    // Continue list files
- [ ] CREATE_DIR                    0x9B    // Create directory
- [ ] DELETE_FILE                   0x9C    // Delete file
- [ ] LIST_OPEN_HANDLES             0x9D    // List handles
- [x] WRITEMAILBOX                  0x9E    // Write to mailbox
- [ ] BLUETOOTHPIN                  0x9F    // Transfer trusted pin code to brick
- [ ] ENTERFWUPDATE                 0xA0    // Restart the brick in Firmware update mode
