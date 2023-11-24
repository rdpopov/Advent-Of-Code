/* #include <stdio.h> */
#include <cctype>
#include <cstddef>
#include <iterator>
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
#include <utility>
#include <vector>
#include <map>
#include <queue>
#include <set>
#include <cmath>


enum States {
    A = 0,
    B,
    C,
    D,
    E,
    F
};

typedef struct Operation {
    int write;
    int fp_increase;
    States next;
} Operation;

Operation test_program[][2] = {
    { // A
        {.write = 1, .fp_increase = 1,  .next = B }, // 0
        {.write = 0, .fp_increase = -1, .next = B }, // 1
    },
    { // B
        {.write = 1, .fp_increase = -1, .next = A }, // 0
        {.write = 1, .fp_increase = 1,  .next = A }, // 1
    },
};

Operation real_program[][2] = {
    { // A
        {.write = 1, .fp_increase =  1, .next = B }, // 0
        {.write = 0, .fp_increase = -1, .next = E }, // 1
    },
    { // B
        {.write = 1, .fp_increase = -1, .next = C }, // 0
        {.write = 0, .fp_increase =  1, .next = A }, // 1
    },
    { // C
        {.write = 1, .fp_increase = -1, .next = D }, // 0
        {.write = 0, .fp_increase = 1,  .next = C }, // 1
    },
    { // D
        {.write = 1, .fp_increase = -1, .next = E }, // 0
        {.write = 0, .fp_increase = -1, .next = F }, // 1
    },
    { // E
        {.write = 1, .fp_increase = -1, .next = A }, // 0
        {.write = 1, .fp_increase = -1, .next = C }, // 1
    },
    { // F
        {.write = 1, .fp_increase = -1, .next = E }, // 0
        {.write = 1, .fp_increase = 1,  .next = A }, // 1
    },
};

uint64_t part1(Operation  instructions[][2], uint64_t itr){
    uint64_t ret=0;
    int64_t instr_ptr;
    States instr = A;
    std::map<int64_t, int8_t> tape;
    tape[0] = 0;

    for(int i=0; i < itr;i++) {
        Operation crnt = instructions[instr][tape[instr_ptr]];
        tape[instr_ptr]  = crnt.write;
        instr            = crnt.next;
        instr_ptr       += crnt.fp_increase;
    }

    for (auto a : tape){
        ret += a.second;
    }
    return ret;
}


int main (int argc, char *argv[]) {
    std::cout << "Part 1 -> " << part1(test_program, 6) <<"\n" ;
    std::cout << "Part 1 -> " << part1(real_program, 12386363) <<"\n" ;
    return 0;
}
