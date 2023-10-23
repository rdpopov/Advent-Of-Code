/* #include <stdio.h> */
#include <cstddef>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <stdint.h>

#include <sstream>
#include <algorithm>
#include <fstream>
#include <istream>
#include <string>
#include <sys/types.h>
#include <vector>
#include <map>
#include <set>
#include <cmath>

/* #define TEST */

#ifdef TEST
#define INPUT std::vector<int>{3, 4, 1, 5}
#define LIST_SIZE 5
#else
#define INPUT std::vector<int>{83,0,193,1,254,237,187,40,88,27,2,255,149,29,42,100}
#define LIST_SIZE 256
#endif


void swap (std::vector<int> &tape, int low, int high){
    auto norm = [&](int a){
        a += tape.size();
        a %= tape.size();
        return  a;
    };
    auto incr= [&](){
        low += 1;
        low += tape.size();
        low %= tape.size();
    };
    auto decr= [&](){
        high -= 1;
        high += tape.size();
        high %= tape.size();
    };
    if (low == high) return;
    while(true) {
        std::swap(tape[low],tape[high]);
        decr();
        if (low == high) return;
        incr();
        if (low == high) return;
    }
}

int32_t part1(std::vector<int> instructions, size_t length  ) {
    std::vector<int> tape;
    int idx = 0;
    tape.resize(length);
    for (auto &t: tape){
        t = idx++;
    }
    std::vector<int> tmp(tape);
    idx = 0;
    int skip = 0;
    for (auto a:instructions) {
        int nxt = (idx + a) % length;
        int crnt = 0;
        swap(tape,idx,nxt);
        crnt =0;
        std::cout<< idx << " " << nxt << " ";
        std::cout << "[";
        idx = (nxt + skip)%length;
        skip++;
        for (auto t: tape){
            if (t == tape[tape.size()-1]){
                std::cout<< t << "]";
            } else {
                std::cout<< t << ", ";
            }
        }
        std::cout << std::endl;
    }
    return  tape[0]*tape[1];
}

int main (int argc, char *argv[]) {
    std::cout << part1(INPUT, LIST_SIZE) << std::endl;
    return 0;

}
