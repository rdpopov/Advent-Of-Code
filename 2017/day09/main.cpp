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

#ifndef TEST
#define INPUT "./input"
#else
#define INPUT "./input_test"
#endif

class Parser{
    private:
        std::string  program;
        enum State{
            Garbage=1,
            SkipNext=2
        };
    public:


        Parser(std::string line): program(line) {}

        int32_t parse_p1() {
            auto score = 0;
            auto depth = 0;
            auto skip=0;
            auto garb=0;

            for(auto a: this->program){
                if (!skip){
                    switch (a) {
                        case '{': { if (!garb) depth++; } break;
                        case '}': { if (!garb) score += depth--; } break; 
                        case '!': skip = 2; break;
                        case '<': { garb = 1; } break;
                        case '>': { if (garb) { garb = 0; } } break;
                        default: break;
                    }
                }
                skip = skip ? skip - 1 : 0;
            }
            return score;
        };
        int32_t parse_p2() {
            auto score = 0;
            auto depth = 0;
            int32_t skip=0;
            int32_t garb=0;
            int32_t garb_chars = 0;

            for(auto a: this->program){
                if (skip == 0){
                    switch (a) {
                        case '{': { if (!garb) depth++; } break;
                        case '}': { if (!garb) score += depth--;} break;
                        case '!': skip = 2;  break;
                        case '<': {if (!garb) {garb_chars--;garb=1;}} break;
                        case '>': {garb = 0;} break;
                        default: break;
                    }
                    /* printf("chars: %c score: %d\n",a ,garb); */
                    if (skip == 0) garb_chars += garb;
                }
                skip = skip ? skip - 1 : 0;
            }
            return garb_chars;
        }

};

int32_t part1(std::string fname) {
    std::ifstream inp(fname);
    std::string line;
    std::map<std::string,int32_t> registers;
    while (std::getline(inp,line)) {
        auto res = Parser(line);
        return res.parse_p1();
    }
    return  0;
}
int32_t part2(std::string fname) {
    std::ifstream inp(fname);
    std::string line;
    std::map<std::string,int32_t> registers;
    while (std::getline(inp,line)) {
        auto res = Parser(line);
        return res.parse_p2();
    }
    return  0;
}

int main (int argc, char *argv[]) {
    std::cout << "Part1 "<<part1(INPUT) << std::endl;
    std::cout << "Part2 "<<part2(INPUT) << std::endl;
    return 0;
}
