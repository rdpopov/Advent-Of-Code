/* #include <stdio.h> */
#include <iostream>
#include <stdint.h>
#include <assert.h>
#include <sstream>
#include <fstream>
#include <string>
#include <vector>
#include <cmath>



#ifndef TEST
#define INPUT 265149
#endif


int32_t part1(int32_t pos) {
    auto ring  = (int32_t)(std::sqrt(pos-1) + 1) >> 1;
    int32_t ring_sq = (2*ring-1) *(2*ring-1);
    int32_t offset_of_ring = pos;
    std::vector<int32_t> cross = {
        std::abs( pos - ( ring_sq + 1 * ring) ),
        std::abs( pos - ( ring_sq + 3 * ring) ),
        std::abs( pos - ( ring_sq + 5 * ring) ),
        std::abs( pos - ( ring_sq + 7 * ring) ),
    };
    for(auto& s: cross) {
        std::cout << s << " ";
        offset_of_ring = std::min(offset_of_ring,s);
    }

    /* std::cout << ring  << std::endl; */
    return ring + offset_of_ring;
}

int main (int argc, char *argv[]) {
#ifdef TEST
    auto test_part1_lambda = [](int32_t pos,int32_t expected){
        int32_t fn_res = part1(pos);
        std::stringstream msg;
        msg << "part1( "<<pos << " ) != "<< fn_res;
        if (fn_res != expected) throw std::logic_error(msg.str());
        return 1;
    };
    int32_t passed_tests = 0;
    std::vector<std::pair<int32_t,int32_t>> tests = {
        { 1    ,0  },
        { 12   ,3  },
        { 23   ,2  },
        { 1024 ,31 }
        }; 
    for( auto &t: tests ) {
        try {
            passed_tests += test_part1_lambda(t.first,t.second);
        } catch(const std::exception& e) {
            std::cout << e.what() << std::endl;
        }
    }
#endif
    std::cout << part1(INPUT);
    return 0;
}
