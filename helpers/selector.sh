#!/bin/bash
filelang=$1
n=$(wc -l $filelang|cut -d" " -f1)
rand_lang=$(shuf -i 1-$n|head -n 1)
awk 'NR=='$rand_lang'{print$0}' $filelang
