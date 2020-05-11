# ROS
Basic operating system, written in rust, by a beginner. Follows Phillip Opperman's blog os
Bootimage is put into target/x86target/debug/bootimage-ros.bin
## Dependencies

Bootloader
Volatile
Spin
Xbuild

Must be built with cargo nightly, QEMU x86 is supported, other x86 platforms may work

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