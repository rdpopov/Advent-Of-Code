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

#define TEST

#ifdef TEST
#define INPUT "test"
#else
#define INPUT "input"
#endif

int32_t part1(std::string inp) {
    std::map<std::string, std::vector<int>> swc={};
    swc["ne"] = {0,0,1};
    swc["nw"] = {1,0,0};
    swc["n"]  = {0,1,0};
    swc["se"] = {0,0,-1};
    swc["sw"] = {-1,0,0};
    swc["s"]  = {0,-1,0};

    std::vector<int> pos={0,0,0};

    std::ifstream in (inp);
    std::string c;
    while (std::getline(in,c,',')) {
        if (c.back() == '\n'){
            c.resize(c.length()-1);
        }
        auto mv = swc[c];
        pos[0] += mv[0];
        pos[1] += mv[1];
        pos[2] += mv[2];
    }

    // minimum of the length of the complement?
    for (auto a: pos){std::cout<< a <<" "; }
    return 0;
}

int main (int argc, char *argv[]) {
    std::cout << part1("test") << std::endl;
    std::cout << part1("test1") << std::endl;
    std::cout << part1("test2") << std::endl;
    return 0;
}
