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

void swap_part1 (std::vector<int> *tape, int low, int size){
    auto norm = [&](int a){
        a += tape->size();
        a %= tape->size();
        return  a;
    };
    std::vector<int> store = {};
    for (int i = 0 ; i < size; i ++){
        int tmp = (*tape)[norm(low+i)];
        store.push_back(tmp);
    }
    int a = 0;
    for (auto i = store.rbegin() ; i != store.rend(); i++,a++ ){
        (*tape)[norm(low+a)] = *i ;
    }
}

using vec = std::vector<int>;

int32_t hash(vec *tape,vec *instructions, int *idx, int *skip  ) {
    int length = tape->size();
    for (auto a: *instructions) {
        swap_part1(tape,*idx,a);
        *idx = (*idx + a + *skip)%length;
        (*skip)++;
    }
    return  (*tape)[0] * (*tape)[1];
}


int32_t part1(vec instructions, size_t length) {
    vec tape;
    int idx = 0;
    tape.resize(length);
    for (auto &t: tape){
        t = idx++;
    }
    idx = 0;
    int skip = 0;
    return hash(&tape,&instructions,&idx,&skip);
}

std::string part2(vec instructions, size_t length) {
    vec tape;
    int idx = 0;
    tape.resize(length);
    for (auto &t: tape){
        t = idx++;
    }
    idx = 0;
    int skip = 0;

    // 64 times the hash
    for (int i=0; i<64; i++)
        hash(&tape, &instructions, &idx, &skip);

    std::stringbuf out ;
    char res[16*2 + 1 ];
    res [16*2] = 0;
    char* tmp = res;
    for(int x=0; x<16;x++) {
        int dense = tape[x*16];
        for (int i=1; i<16; i++)
            dense = dense ^ tape[x*16 + i];
        sprintf(tmp, "%02x", dense); // ol c rules
        tmp += 2;
    }
    return std::string(res);
}

std::vector<int> parse_part2(std::string fname) {
    std::ifstream in(fname);
    std::string word;
    std::vector<int> res = {};

    char c = in.get();
    while (in.good()) {
        if ( c!=10 ) res.push_back(c);
        c = in.get();
    }
    res.push_back(17);
    res.push_back(31);
    res.push_back(73);
    res.push_back(47);
    res.push_back(23);

    /* for (auto a: res) std::cout<< a<< " "; */
    /* std::cout<<std::endl; */
    return res;
}


int main (int argc, char *argv[]) {
    std::cout << part1(INPUT, LIST_SIZE) << std::endl;
    std::cout << part2(parse_part2("input"), LIST_SIZE) <<std::endl;
    std::cout << part2(parse_part2("test"), LIST_SIZE) << std::endl;
    /* parse_part2("test"); */
    return 0;
}
