#!/bin/sh
echo "Setting up isolate..."
echo 0 > /proc/sys/kernel/randomize_va_space
echo never > /sys/kernel/mm/transparent_hugepage/enabled
echo never > /sys/kernel/mm/transparent_hugepage/defrag
echo 0 > /sys/kernel/mm/transparent_hugepage/khugepaged/defrag

apt install make gcc libcap-dev

cd isolate
make isolate
cp isolate /usr/local/bin/isolate
cp default.cf /usr/local/etc/isolate
cd ..

echo "Setting up C++ compiler and Rust's cargo"
apt install g++ cargo

echo "Compiling checker"
git submodule update --init --recursive

mkdir -p example/scripts/checkers

for file in testlib/checkers/*
do
  filename_ex=${file##*/}
  filename=${filename_ex%.*}
  echo ${filename}
  g++ -std=c++11 ${file} -O2 -o example/scripts/checkers/${filename} -I testlib/
done