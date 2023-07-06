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
    for(size_t i =1;i< arr.size();i++) {
        if (arr[res] < arr[i]) {
            res = i;
        }
    }
    return res;
}

std::vector<int32_t> spread(std::vector<int32_t> &nums) {
    int32_t idx = max_index(nums);
    int32_t value = nums[idx];
    nums[idx] = 0;
    for(int i = (idx+1)%nums.size(); value > 0; i++,i%=nums.size(),value-- ) {
        nums[i] += 1;
    }
    return nums;
}

std::string hsh_state(std::vector<int32_t> nums) {
    std::stringstream res;
    for(auto i : nums){
        res << i << " ";
    }
    return res.str();
}

int32_t times(std::vector<int32_t> nums) {
    int32_t res = 0;
    std::set<std::string> visited;
    std::vector<int32_t> crnt(nums);
    int32_t to = 20;
    visited.insert(hsh_state(crnt));

    while(1){
        res += 1;
        spread(crnt);
        std::cout << hsh_state(crnt) <<  std::endl;
        if (visited.find(hsh_state(crnt)) != visited.end() ) break;
        visited.insert(hsh_state(crnt));
    }
    return res;
}

// TODO: investigate this further, why tf does this work
std::pair<int32_t,int32_t> detect_cycle(std::vector<int32_t> seed) {
    std::vector<int32_t> hare(seed);
    std::vector<int32_t> turt(seed);
    size_t mu = 0;
    size_t lambda = 1;
    spread(hare);
    spread(hare);
    spread(turt);
    while(turt != hare) {
        spread(hare);
        spread(hare);
        spread(turt);
    }
    // we have found at which value thecycle happens
    // a loop may have many entries but once you enter its a loop
    std::vector<int32_t> turtle_2(seed);
    while(turtle_2 != hare) {
        spread(turtle_2);
        spread(hare);
        mu++;
    }

    std::vector<int32_t> hare_2(hare);
    spread(hare_2);
    while(hare_2 != hare) {
        spread(hare_2);
        lambda+=1;
    }

    return {mu,lambda};
}

int32_t part1(std::string fname) {
    std::ifstream inp(fname);
    std::string line;
    std::vector <int32_t>numbers = {};

    while (std::getline(inp,line)) {
        char * to_parse = (char *)line.c_str();
        auto offs = 0;
        int32_t cmd;
        while(sscanf(to_parse,"%d%n",&cmd,&offs) > 0) {
            to_parse += offs;
            numbers.push_back(cmd);
        }
        break;
    }
    auto res= detect_cycle(numbers);
    return res.first + res.second;
}

size_t part2(std::string fname) {
    std::ifstream inp(fname);
    std::string line;
    std::vector <int32_t>numbers = {};

    while (std::getline(inp,line)) {
        char * to_parse = (char *)line.c_str();
        auto offs = 0;
        int32_t cmd;
        while(sscanf(to_parse,"%d%n",&cmd,&offs) > 0) {
            to_parse += offs;
            numbers.push_back(cmd);
        }
        break;
    }
    auto res= detect_cycle(numbers);
    return res.second;
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
    /* part1(INPUT); */
    std::cout << "Part2 "<<part2(INPUT) << std::endl;
#endif

    return 0;
}
