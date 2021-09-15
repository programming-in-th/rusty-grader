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

echo "Setting up C++ compiler and Rust's cargo"
apt install g++ cargo