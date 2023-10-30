/* #include <stdio.h> */
#include <cstddef>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
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
#include <tuple>
#include <utility>
#include <vector>
#include <map>
#include <set>
#include <cmath>

#define CALC(tmp,coef,div) (tmp * coef)%div
#define CALC_MULT(tmp,coef,div,mult) (tmp = (tmp * coef)%div)%mult

uint64_t part1(uint64_t iA,uint64_t iB,uint64_t div_by,uint64_t itr){
    uint64_t A = iA;
    uint64_t B = iB;
    uint64_t res = 0;

    for (uint64_t i=0;i<itr;i++){
         if((A & 0xffff)==(B &0xffff)){
              res++;
         }
         A = CALC(A,16807,div_by);
         B = CALC(B,48271,div_by);
    }
    return res;
}

uint64_t part2(uint64_t iA,uint64_t iB,uint64_t div_by,uint64_t itr) {
    uint64_t A = iA;
    uint64_t B = iB;
    uint64_t res = 0;

    for (uint64_t i=0;i<itr;i++){
        if((A & 0xffff)==(B &0xffff)){
            res++;
        }
        while (CALC_MULT(A,16807,div_by,4));
        while (CALC_MULT(B,48271,div_by,8));
    }
    return res;
}

int main (int argc, char *argv[]) {
    std::cout<<"Part 1 -> " <<part1(65,8921,2147483647,40000000)<<std::endl;
    std::cout<<"Part 1 -> " <<part1(703,516,2147483647,40000000)<<std::endl;
    std::cout<<"Part 2 -> " <<part2(65,8921,2147483647,5000000)<<std::endl;
    std::cout<<"Part 2 -> " <<part2(703,516,2147483647,5000000)<<std::endl;
    return 0;
}
