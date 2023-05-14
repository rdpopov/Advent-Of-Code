#include <cstdint>
#include <iostream>
#include <fstream>
#include <stdint.h>
#include <vector>

#ifndef TEST
#define INPUT "./input"
#define INPUT2 "./input"
#else
#define INPUT "./input_test"
#define INPUT2 "./input_test_2"
#endif //TEST

int32_t part1() {
    std::ifstream inp(INPUT);
    std::string line;
    int32_t sum = 0;

    while (std::getline(inp,line)) {
        int32_t line_min = ~0;
        int32_t line_max = 0;
        int32_t tok;
        int32_t offset;
        char* begin = (char*)line.c_str(); // wtf
        while (sscanf(begin,"%d %n",&tok,&offset) > 0){
            begin += offset;
            line_max = std::max(line_max,tok);
            if (line_min == -1) {
                line_min = line_max;
            }
            line_min = std::min(line_min,tok);
        }
        sum += line_max - line_min; 
    }
    return sum;
}

int32_t part2() {
    std::ifstream inp(INPUT2);
    std::string line;
    int32_t sum = 0;

    while (std::getline(inp,line)) {
        int32_t tok;
        int32_t offset;
        std::vector<int32_t> row;
        char* begin = (char*)line.c_str(); // wtf
        [&]{
            while (sscanf(begin,"%d %n",&tok,&offset) > 0){
                begin += offset;
                for (size_t i=0;i < row.size();i++){
                    if (row.size() > 1 && (row[i]%tok == 0 || tok%row[i] == 0)) { 
                        sum += tok/row[i] +row[i]/tok;
                        return;
                    }
                }
                row.push_back(tok);
            }
        }();
    }
    return sum;
}

int main (int argc, char *argv[]) {
    /* int32_t tok; */
    /* int32_t offset; */
    /* std::cout << " sscanf : " << sscanf("","%d %n",&tok,&offset) << "\n"; // cool sscanf */
    std::cout<<"Part 1 "<<part1()<<std::endl;
    std::cout<<"Part 2 "<<part2()<<std::endl;
    return 0;
}
