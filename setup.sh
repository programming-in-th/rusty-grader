#!/bin/sh

green=$(tput setaf 2)
blue=$(tput setaf 4)
normal=$(tput sgr0)


echo "${green}Setting up isolate's package${normal}"
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

echo "${green}Setting up C++ compiler and Rust's cargo${normal}"
apt install g++ cargo

echo "${green}Compiling checkers${normal}"
git submodule update --init --recursive

mkdir -p example/scripts/checkers

for file in testlib/checkers/*
do
  filename_ex=${file##*/}
  echo "${blue}Compiling ${filename_ex}${normal}"
  filename=${filename_ex%.*}
  g++ -std=c++11 ${file} -O2 -o example/scripts/checkers/${filename} -I testlib/
done