#!/bin/bash

curl http://reverse.blackfoot.io:8080/ELF_03 > ELF_FAST_03
# res=$(./solve.py)
./solve.py > a

if [[ $? -ne 0 ]]; then
	exit 1
fi

flag=$(cat a | tail -n 1)

echo $res
echo $flag

url="http://reverse.blackfoot.io:8080/validate/ELF_03/$flag"

curl $url
