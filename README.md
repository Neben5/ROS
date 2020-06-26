# ROS
Basic operating system, written in rust, by a beginner. Follows Phillip Opperman's blog os to some extent (namely entrypoint, a portion of text mode, testing framework, and paging)
Bootimage is put into target/x86target/debug/bootimage-ros.bin
## Dependencies

* Xbuild
* C++ Build tools (on windows)

Must be built with cargo nightly, QEMU x86 is supported, other x86 platforms may work
Note that there are a huge amount of version compatibility issues. With an older nightly, none of the packages will compile. With a newer nightly, an older xbuild won't install. In general finding combinations of xbuild and nightly that will install *and* cross compile is like finding waldo in a christmas special edition.

On Windows, this is what I know works:
* rust nightly 2020-06-24
* Visual Studio Build Tools 2019 __with__:
  * MSVC VS 2019 C++ x64/x86 build tools
  * Windows 10 SDK
* xbuild 0.5.29
## Usage

To build, use 
```shell
cargo xbuild
```
To run, use
```shell
cargo xrun
```
or
```shell
qemu-system-x86_64 -drive format=raw,file=target/x86target/debug/bootimage-ros.bin
```