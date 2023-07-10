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

class Expr{
    private:
        std::string target_token;
        int32_t delta;
        std::string variable_check;
        std::string variable_predicate;
        int32_t variable_value;
        enum Operation {
            NotEqual,
            Equal,
            Less,
            More,
            MoreOrEqual,
            LessOrEqual,
        };

    public:

        Expr(std::string _line) {
            std::vector<std::string> tokens;
            std::stringstream line(_line);
            std::string tmp;
            while(getline(line, tmp, ' ')) tokens.push_back(tmp);
            this->target_token = tokens[0];
            this->delta = atoi(tokens[2].c_str());
            if (tokens[1] == "dec") delta = -delta;
            this->variable_check = tokens[4];
            this->variable_value = atoi(tokens[6].c_str());
            this->variable_predicate = tokens[5];
        }
        bool predicate(std::map<std::string,int32_t> &registers) {
            static const std::map<std::string,Operation> translation_table = {
                {"!=",Operation::NotEqual},
                {"==",Operation::Equal},
                {"<",Operation::Less},
                {">",Operation::More},
                {"<=",Operation::LessOrEqual},
                {">=",Operation::MoreOrEqual},
            };
            switch(translation_table.find(this->variable_predicate)->second){
                case NotEqual:
                    return registers.find(this->variable_check)->second != this->variable_value;
                case Equal:
                    return registers.find(this->variable_check)->second == this->variable_value;
                case Less:
                    return registers.find(this->variable_check)->second < this->variable_value;
                case More:
                    return registers.find(this->variable_check)->second > this->variable_value;
                case MoreOrEqual:
                    return registers.find(this->variable_check)->second >= this->variable_value;
                case LessOrEqual:
                    return registers.find(this->variable_check)->second <= this->variable_value;
            }

            return  true;
        };

        void execute(std::map<std::string,int32_t> &registers){
            if (this->predicate(registers)) {
                if(registers.find(this->target_token ) == registers.end()){
                    registers.insert({this->target_token,0});
                }
                registers.at(this->target_token) += this->delta;
            }
        };



        std::string str(){
            std::stringstream out;
            out << this->target_token << " + ";
            out << this->delta << " | if ";
            out << this->variable_check << " ";
            out << this->variable_predicate << " ";
            out << this->variable_value;
            return out.str();
        }

};

int32_t part1(std::string fname) {
    std::ifstream inp(fname);
    std::string line;
    std::map<std::string,int32_t> registers;
    while (std::getline(inp,line)) {
        Expr(line).execute(registers);
    }
    auto max_reg = 0;
    for(auto a: registers) {
        max_reg = std::max(max_reg,a.second);
    }
    return max_reg;
}



int32_t part2(std::string fname) {
    std::ifstream inp(fname);
    std::string line;
    std::map<std::string,int32_t> registers;
    auto max_reg = 0;
    while (std::getline(inp,line)) {
        Expr(line).execute(registers);
        for(auto a: registers) {
            max_reg = std::max(max_reg,a.second);
        }
    }
    return max_reg;
}

int main (int argc, char *argv[]) {
    std::cout << "Part1 "<<part1(INPUT) << std::endl;
    std::cout << "Part2 "<<part2(INPUT) << std::endl;
    return 0;
}
