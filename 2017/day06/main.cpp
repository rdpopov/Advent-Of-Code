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

size_t max_index(std::vector<int32_t> arr) {
    size_t res = 0;
    for(size_t i =0;i< arr.size();i++) {
        if (arr[res] < arr[i]) {
            res = i;
        }
    }
    return res;
}

int32_t times(std::vector<int32_t> nums) {
    int32_t res = 0;
    std::set<std::string> visited;
    std::vector<int32_t> crnt(nums);
    while(visited.count(std::str(crnt.begin(), crnt.end()) == 0){
        visited.insert(std::str(crnt.begin(), crnt.end());
        auto idx = max_index(crnt);
        auto add_to_all = crnt[idx] / crnt.size();
        auto set_at_idx = crnt[idx] % crnt.size();
        // we can just add to the index we are going to override it anyway
        for(size_t i = 0;i < crnt.size();i++) {
            crnt[i] += add_to_all;
        }
        crnt[idx] = set_at_idx;
        res += 1;
    }
    return res;
}

int32_t part1(std::string fname) {
    std::ifstream inp(fname);
    std::string line;
    std::vector <int32_t>numbers = {};

    while (std::getline(inp,line)) {
        char * to_parse = (char *)line.c_str();
        printf("%s\n",to_parse);
        auto offs = 0;
        int32_t cmd;
        while(sscanf(to_parse,"%d%n",&cmd,&offs)) {
            to_parse += offs;
            numbers.push_back(cmd);
            /* printf("%d\n",cmd); */
        }
        break;
    }

    return times(numbers);
}

size_t part2(std::string fname) {
    return 0;
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
    /* std::cout << "Part2 "<<part2(INPUT) << std::endl; */
#endif

    return 0;
}
