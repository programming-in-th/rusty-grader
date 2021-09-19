#!/bin/sh
tput setaf 2; echo "Setting up isolate's package"
apt install make gcc libcap-dev

tput setaf 2; echo "Setting up isolate..."
echo 0 > /proc/sys/kernel/randomize_va_space
echo never > /sys/kernel/mm/transparent_hugepage/enabled
echo never > /sys/kernel/mm/transparent_hugepage/defrag
echo 0 > /sys/kernel/mm/transparent_hugepage/khugepaged/defrag

cd isolate
make isolate
cp isolate /usr/local/bin/isolate
cp default.cf /usr/local/etc/isolate
cd ..

tput setaf 2; echo "Setting up C++ compiler and Rust's cargo"
apt install g++ cargo

tput setaf 2; echo "Compiling checkers"
git submodule update --init --recursive

mkdir -p example/scripts/checkers

for file in testlib/checkers/*
do
  filename_ex=${file##*/}
  tput setaf 4; echo Compiling ${filename_ex}
  filename=${filename_ex%.*}
  g++ -std=c++11 ${file} -O2 -o example/scripts/checkers/${filename} -I testlib/
done