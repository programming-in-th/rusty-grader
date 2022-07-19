#!/bin/sh

red=$(tput setaf 1)
green=$(tput setaf 2)
blue=$(tput setaf 4)
normal=$(tput sgr0)

SCRIPT=`realpath $0`
SCRIPTPATH=`dirname $SCRIPT`
ISOLATE_GROUPNAME="isolate"

echo "${green}Cloning Submodule${normal}"
git -C ${SCRIPTPATH} submodule update --init --recursive --depth 1

echo "${green}Setting up compilers and dependencies${normal}"
sudo apt-get update -y
sudo apt-get install --no-install-recommends -y build-essential cargo openjdk-17-jdk libcap-dev sysfsutils golang

echo "${green}Setting up isolate...${green}"
sudo sh -c "echo 0 > /proc/sys/kernel/randomize_va_space"
sudo sh -c "echo never > /sys/kernel/mm/transparent_hugepage/enabled"
sudo sh -c "echo never > /sys/kernel/mm/transparent_hugepage/defrag"
sudo sh -c "echo 0 > /sys/kernel/mm/transparent_hugepage/khugepaged/defrag"

if ! grep -Fxq "kernel.randomize_va_space = 0" /etc/sysctl.d/10-isolate.conf; then
  sudo sh -c 'echo "kernel.randomize_va_space = 0" >> /etc/sysctl.d/10-isolate.conf'
fi
if ! grep -Fxq "kernel/mm/transparent_hugepage/enabled = never" /etc/sysfs.conf; then
  sudo sh -c 'echo "kernel/mm/transparent_hugepage/enabled = never" >> /etc/sysfs.conf'
fi
if ! grep -Fxq "kernel/mm/transparent_hugepage/defrag = never" /etc/sysfs.conf; then
  sudo sh -c 'echo "kernel/mm/transparent_hugepage/defrag = never" >> /etc/sysfs.conf'
fi
if ! grep -Fxq "kernel/mm/transparent_hugepage/khugepaged/defrag = 0" /etc/sysfs.conf; then
  sudo sh -c 'echo "kernel/mm/transparent_hugepage/khugepaged/defrag = 0" >> /etc/sysfs.conf'
fi

sudo systemctl enable --now sysfsutils.service

make -C ${SCRIPTPATH}/isolate isolate
sudo make -C ${SCRIPTPATH}/isolate install

sudo groupadd ${ISOLATE_GROUPNAME} 
sudo chown root:${ISOLATE_GROUPNAME} /usr/local/bin/isolate
if [ -z ${GITHUB_ACTIONS} ]; then
  echo "${green}Adding ${USER} to ${ISOLATE_GROUPNAME} group${green}"
  sudo chmod 4750 /usr/local/bin/isolate
  sudo usermod -aG ${ISOLATE_GROUPNAME} ${USER}
else
  echo "${green}Setting isolate permissions to 777 for GitHub Actions${green}"
  sudo chmod 4777 /usr/local/bin/isolate
fi


echo "${green}Setting up .env${normal}"

echo ISOLATE_PATH=\"/usr/local/bin/isolate\" >> .env
echo ALTERNATIVE_PATH=\"/etc/alternatives\" >> .env
echo TEMPORARY_PATH=\"/tmp\" >> .env
echo BASE_PATH=\"$(pwd)/example\" >> .env

echo "${green}Compiling checkers${normal}"

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

echo "${red}You should reboot your system to apply the kernel paremeters change for isolate${normal}"
