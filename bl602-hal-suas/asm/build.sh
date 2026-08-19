#!/bin/bash

# Empty directory
rm -rf ../bin
mkdir -p ../bin

# Compile
../../../../../toolchain/compiler/bin/riscv32-unknown-elf-gcc -ggdb3 -c -mabi=ilp32f -march=rv32imfc assembly.S -o ../bin/assembly.o
../../../../../toolchain/compiler/bin/riscv32-unknown-elf-ar crs ../bin/assembly.a ../bin/assembly.o

# Cleanup object file
rm ../bin/*.o