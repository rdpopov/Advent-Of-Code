/* #include <stdio.h> */
#include <cstddef>
#include <list>
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
#include <vector>
#include <map>
#include <set>
#include <cmath>

uint64_t part1(uint64_t push_jump,uint64_t upper = 2017){
    std::list<uint64_t> tape{0};
    auto itr = tape.begin();
    for(uint64_t i = 1 ; i <= upper;i++){
        uint64_t skip = push_jump;
        while(skip--){
            if (itr == tape.end()) itr = tape.begin();
            itr++;
        }
        tape.insert(itr,i);
        /* auto dbg = tape.begin(); */
        /* while(dbg!=tape.end()){ */
        /*     std::cout<<*dbg<<" "; */
        /*     dbg++; */
        /* } */
        /* std::cout<<"\n"; */
    }
    return *(itr);
}

uint64_t part2(uint64_t push_jump,uint64_t upper){
    uint64_t next = 0;
    uint64_t ret = 0;
    for(uint64_t i = 1 ; i <= upper;i++){
        next = ((push_jump + next) % i) + 1;
        if(next == 1) {
            ret = i;
        }
    }
    return ret;
}

int main (int argc, char *argv[]) {
    std::cout << "Part 1 " << part1(3,10)<<"\n";
    std::cout << "Part 1 " << part1(329,2017)<<"\n";
    std::cout<< part2(329,50000000)<<"\n";
    return 0;
}
