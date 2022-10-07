#!/bin/bash

curl http://reverse.blackfoot.io:8080/ELF_01 > ELF_FAST_01
flag=$(./solve.py | tail -n 1)

echo $flag

url="http://reverse.blackfoot.io:8080/validate/ELF_01/$flag"

curl $url
