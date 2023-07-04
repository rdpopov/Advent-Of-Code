/* #include <stdio.h> */
#include <cstdint>
#include <iostream>
#include <stdint.h>

#include <sstream>
#include <algorithm>
#include <fstream>
#include <istream>
#include <string>
#include <vector>
#include <map>
#include <set>
#include <cmath>



#ifndef TEST
#define INPUT "./input"
/* #define INPUT "./input_test" */
#else
#define INPUT "./input_test"
#endif


int32_t play_commands_part1(std::vector<int32_t> commands) {
    int32_t idx = 0;
    int32_t iterations = 0;

    while (idx != commands.size()){
        auto idx_old = commands[idx];
        commands[idx] += 1;
        idx += idx_old;
        iterations += 1;
    }
    return iterations;
}

size_t play_commands_part2(std::vector<int32_t> commands) {
    int32_t idx = 0;
    size_t iterations = 0;

    while (idx != commands.size()){
        auto idx_old = commands[idx];
        if (commands[idx] < 3) {
            commands[idx] += 1; 
        } else {
            commands[idx] -= 1; 
        }
        idx += idx_old;
        iterations += 1;
    }
    return iterations;
}

int32_t part1(std::string fname) {
    std::ifstream inp(fname);
    std::string line;
    auto res = 0;
    std::vector<int32_t> commands = {};

    while (std::getline(inp,line)) {
        int32_t cmd = 0;
        sscanf(line.c_str(),"%d",&cmd);
        commands.push_back(cmd);
    }
    return play_commands_part1(commands);
}

size_t part2(std::string fname) {
    std::ifstream inp(fname);
    std::string line;
    auto res = 0;
    std::vector<int32_t> commands = {};

    while (std::getline(inp,line)) {
        int32_t cmd = 0;
        sscanf(line.c_str(),"%d",&cmd);
        commands.push_back(cmd);
    }
    return play_commands_part2(commands);
}

int main (int argc, char *argv[]) {
#ifdef TEST
    auto test_part1_lambda = [](std::string fname,int32_t expected){
        int32_t fn_res = part1(fname);
        std::stringstream msg;
        msg << "part1( "<< fname << " ) != "<< fn_res;
        if (fn_res != expected) throw std::logic_error(msg.str());
        return 1;
    };
    int32_t passed_tests = 0;
    std::vector<std::pair<std::string,int32_t>> tests = {
        { "./input_test"    ,5  },
        }; 
    for( auto &t: tests ) {
        try {
            passed_tests += test_part1_lambda(t.first,t.second);
        } catch(const std::exception& e) {
            std::cout << e.what() << std::endl;
        }
    }

    auto test_part2_lambda = [](std::string fname,int32_t expected){
        int32_t fn_res = part2(fname);
        std::stringstream msg;
        msg << "part2( "<< fname << " ) != "<< fn_res;
        if (fn_res != expected) throw std::logic_error(msg.str());
        return 1;
    };
    std::vector<std::pair<std::string,int32_t>> tests2 = {
        { "./input_test2"    ,3  },
        }; 
    for( auto &t: tests2 ) {
        try {
            passed_tests += test_part2_lambda(t.first,t.second);
        } catch(const std::exception& e) {
            std::cout << e.what() << std::endl;
        }
    }
#else
    std::cout << "Part1 "<<part1(INPUT) << std::endl;
    std::cout << "Part2 "<<part2(INPUT) << std::endl;
#endif

    return 0;
}
