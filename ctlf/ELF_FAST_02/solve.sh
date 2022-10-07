#!/bin/bash

curl http://reverse.blackfoot.io:8080/ELF_02 > ELF_FAST_02
flag=$(./solve.py | tail -n 1)

echo $flag

url="http://reverse.blackfoot.io:8080/validate/ELF_02/$flag"

curl $url
