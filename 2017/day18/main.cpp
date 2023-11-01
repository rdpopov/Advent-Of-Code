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
#include <vector>
#include <map>
#include <set>
#include <cmath>

enum Kind {
    Constant,
    Register,
    Empty,
};

enum Command {
    Add,
    Jgz,
    Mod,
    Mul,
    Rcv,
    Set,
    Snd,
    Non,
};

class Token{
    public:
        Kind tag;
        union {
            int32_t cnst;
            std::string reg;
        };
// =========================
        Token(): tag(Kind::Empty) {};

        Token(std::string inp) {
            if (inp.length() > 0 && ('0' <= inp[0] && inp[0] <= '9' || inp[0] == '-')){
                this->tag = Kind::Constant;
                sscanf(inp.c_str(),"%d", &(this->cnst));
                if (errno != 0) { 
                    std::cerr << "Failiure to parse inp"; 
                    this->tag = Kind::Empty;
                }
            } else {
                this->tag = Kind::Register;
                this->reg = inp;
            }
        }
};

class Instruction {
    public:
    Command cmd;
    Token left;
    Token right;
    Instruction(): cmd(Command::Non){};
};




int part1(std::string fname){
    std::ifstream in(fname);
    std::string line;
    while(std::getline(in,line)){
        std::stringstream line_itr(line);
        while(std::getline(line_itr,line,' ')){
            std::cout<<line<<"\n";
        }
    }
    return 0;
}

int main (int argc, char *argv[]) {
    std::cout << "Part 1 -> "<< part1("./test") << std::endl;

/*     std::cout << "Part 1 -> "<< part1("./input") << std::endl; */
/*     std::cout << "Part 2 -> "<< part2("./input") << std::endl; */
/*     std::cout << "Part 2 fast -> "<< part2_fast("./input") << std::endl; */
    return 0;
}
