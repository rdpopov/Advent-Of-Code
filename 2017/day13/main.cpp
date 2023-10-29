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

#define TEST

#ifdef TEST
#define INPUT "test"
#else
#define INPUT "input"
#endif

std::vector<std::pair<int, int>> parse_file (std::string fname){
    std::ifstream in (fname);
    std::string ln;
    std::vector<std::pair<int, int>> res;

    while (std::getline(in,ln,'\n')) {
        ln.erase(remove(ln.begin(),ln.end(),':'),ln.end());
        std::stringstream line(ln);
        std::string tok;
        std::vector<int> nums;

        while (std::getline(line,tok,' ')) {
            int prs;
            sscanf(tok.c_str(),"%d",&prs);
            nums.push_back(prs);
        }
        res.push_back({nums[0],nums[1]}); // no point in removing the first number
                                        // since it shoud contain itself
    }
    return res;
}


int32_t part1(std::string inp) {
    std::vector<std::pair<int, int>>  contents=parse_file(inp);
    int res = 0;
    for (auto &a: contents ){
        int divisor = a.second * 2 - 2;
        int idx = a.first;
        if (idx % divisor == 0){
            printf("( %d %d + %d )",a.first , a.second,a.first * a.second);
            res += a.first * a.second;
        }
    }
    return res;
}

int32_t part1_dumb(std::vector<std::pair<int, int>> &contents,int offs) {
    int res = 0;
    for (auto &a: contents ){
        int divisor = a.second * 2 - 2;
        int idx = a.first+offs;
        if (idx % divisor == 0){
            res ++;
        }
    }
    return res;
}

int32_t part2(std::string inp) {
    std::vector<std::pair<int, int>>  contents=parse_file(inp);
    for (int i=0;i<10000000;i++){
        if (part1_dumb(contents,i)==0) return i;
    }
    return -1;
}


int main (int argc, char *argv[]) {
    std::cout << "Part 1 -> "<< part1("./test") << std::endl;
    std::cout << "Part 1 -> "<< part1("./input") << std::endl;
    std::cout << "Part 2 -> "<< part2("./test") << std::endl;
    std::cout << "Part 2 -> "<< part2("./input") << std::endl;
    return 0;
}
