#!/usr/bin/env bash

green=$(tput setaf 2)
blue=$(tput setaf 4)
normal=$(tput sgr0)

SCRIPT=`realpath $0`
SCRIPTPATH=`dirname $SCRIPT`

echo "${green}Cloning Submodule${normal}"
git -C ${SCRIPTPATH} submodule update --init --recursive

echo "${green}Setting up isolate's package${normal}"
apt install make gcc libcap-dev

echo "Setting up isolate..."
echo 0 > /proc/sys/kernel/randomize_va_space
echo never > /sys/kernel/mm/transparent_hugepage/enabled
echo never > /sys/kernel/mm/transparent_hugepage/defrag
echo 0 > /sys/kernel/mm/transparent_hugepage/khugepaged/defrag

make -C ${SCRIPTPATH}/isolate isolate
cp ${SCRIPTPATH}/isolate/isolate /usr/local/bin/isolate
cp ${SCRIPTPATH}/isolate/default.cf /usr/local/etc/isolate

echo "${green}Setting up C++ compiler and Rust's cargo${normal}"
apt install g++
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. $HOME/.cargo/env

echo "${green}Setting up .env${normal}"

echo ISOLATE_PATH=\"/usr/local/bin/isolate\" >> .env
echo ALTERNATIVE_PATH=\"/etc/alternatives\" >> .env
echo TEMPORARY_PATH=\"/tmp\" >> .env
echo BASE_PATH=\"$(pwd)/example\" >> .env

echo "${green}Compiling checkers${normal}\n"

CHECKER_PATH=${SCRIPTPATH}/example/scripts/checker_scripts

mkdir -p ${CHECKER_PATH}

for file in ${SCRIPTPATH}/testlib/*
do
  filename_ex=${file##*/}
  ex=${filename_ex#*.}
  if [ "${ex}" = "cpp" ];
  then
    echo "${blue}Compiling ${filename_ex}${normal}"
    filename=${filename_ex%.*}
    g++ -std=c++11 ${file} -O2 -o ${CHECKER_PATH}/${filename} -I ${SCRIPTPATH}/testlib
  fi
done