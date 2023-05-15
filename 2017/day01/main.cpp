/* #include <stdio.h> */
#include <iostream>
#include <stdint.h>
#include <assert.h>
#include <fstream>
#include <string>


#ifndef TEST
#define INPUT_FILE "input"
#define INPUT_FILE2 "input"
#else
#define INPUT_FILE "input_test"
#define INPUT_FILE2 "input_test2"
#endif

inline char get_with_delta(std::string line, size_t idx, int delta = 0) {
    return line[(line.length()+delta+idx) % line.length()];
}

void part1() {
    std::cout<< "\nPart 1\n";
    std::ifstream inp (INPUT_FILE);
    std::string line = ""; 
    int64_t res = 0;
    assert(inp.is_open());
    while(inp) {
        res = 0 ;
        std::getline(inp,line);
        if (!line.length()) continue; // empty line
        for (size_t i = 0;i < line.length();i++){
            if (get_with_delta(line,i,-1) == get_with_delta(line,i) ){
                res += line[i]-'0';
            }
        }
        std::cout << "for " << line << "\nres is :" << res<<"\n";
    }
    return;
}

void part2() {
    std::cout<< "\nPart 2\n";
    std::ifstream inp (INPUT_FILE2);
    std::string line = ""; 
    int64_t res = 0;
    assert(inp.is_open());
    while(inp) {
        res = 0 ;
        std::getline(inp,line);
        if (!line.length()) continue; // empty line 
        for (size_t i = 0;i < line.length();i++){
            if (get_with_delta(line,i,line.length()/2) == get_with_delta(line,i) ){
                res += line[i]-'0';
            }
        }
        std::cout << "for " << line << "\nres is :" << res<<"\n";
    }
    return;
}

int main (int argc, char *argv[]) {
    part1();
    part2();
    return 0;
}
