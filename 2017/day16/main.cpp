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

void spin(std::string * inp,int cycle){
    int idx = 0;
    std::string res(*inp);
    for (auto a: *inp ){
        res[(idx+cycle)%inp->length()] = a;
        idx++;
    }
    inp->replace(inp->begin(),inp->end(),res);
}

class CommandIntr{
    public:
        virtual void exec(std::string *perm) = 0;
};

class Spin : public CommandIntr {
/* class Spin { */
    public:
        void exec(std::string *perm) {
            spin(perm,this->cycle);
        }
        Spin(std::string tok){
            sscanf(tok.c_str(),"s%d",&(this->cycle));
        };
        ~Spin();
    private:
        int cycle;
};

class SwapIdx : public CommandIntr {
/* class SwapIdx { */
    public:
        void exec(std::string *perm) {
            int x1      = this->idx1 % perm->length();
            int x2      = this->idx2 % perm->length();
            char tmp    = (*perm)[x1];
            (*perm)[x1] = (*perm)[x2];
            (*perm)[x2] = tmp;
        }
        SwapIdx(std::string tok){
            sscanf(tok.c_str(),"x%d/%d",&(this->idx1),&(this->idx2));
        };
        ~SwapIdx();
    private:
        int idx1;
        int idx2;
};

class SwapChars : public CommandIntr {
/* class SwapChars { */
    public:
        void exec(std::string * perm) {
            int swch_pos_chc1 = perm->find(this->ch1);
            int swch_pos_chc2 = perm->find(this->ch2);
            (*perm)[swch_pos_chc1]= this->ch2;
            (*perm)[swch_pos_chc2]= this->ch1;
        }
        SwapChars(std::string &tok){
            sscanf(tok.c_str(),"p%c/%c",&(this->ch1),&(this->ch2));
        };
        ~SwapChars();
    private:
        char ch1;
        char ch2;
};

std::vector<CommandIntr*> input_to_commands(std::string fname){
    std::vector<CommandIntr*>  res;
    std::ifstream in (fname);
    std::string ln;
    std::stringstream line(ln);
    std::string tok;

    while (std::getline(in,ln,'\n')) {
        std::stringstream line(ln);
        std::string tok;
        while (std::getline(line,tok,',')) {
            switch (tok[0]){
                case 's':
                    res.push_back(new Spin(tok));
                    break;
                case 'x':
                    res.push_back(new SwapIdx(tok));
                    break;
                case 'p': 
                    res.push_back(new SwapChars(tok));
                    break;
            }
        }
    }
    return res;
}

std::string part1 (std::string fname,std::string _perm){
    auto cmds = input_to_commands(fname);
    std::string * perm = new std::string(_perm);
    auto exc = [&](){
    for(auto a: cmds){
        a->exec(perm);
    }};
    exc();
    return *perm;
}

std::string part2 (std::string fname,std::string _perm){
    auto cmds = input_to_commands(fname);
    std::string * perm = new std::string(_perm);
    auto exc = [&](){
    for(auto a: cmds){
        a->exec(perm);
    }};

    std::map<std::string,uint64_t> keep {};
    bool found_flag = true;
#define MAXITER 1000000000
    for(uint64_t i=0;i<MAXITER;i++){
        if (found_flag && keep.find(*perm)!=keep.end()){
            int period = i - keep[*perm];
            if(period == 0){
                // if period is 0 that means that processing achieved nothing ->
                // therefore we can return current state, as it will not change
                return *perm;
            }
            i = MAXITER - ((MAXITER - i)%period); // adjust i to be the start of
                                                  // the last loop
                                                  // it may be guaranteed
            found_flag = false;
        }
        if (found_flag){
            keep[*perm] = i;
        }
        exc();
    }
    return  *perm;
}

std::string part2_fast (std::string fname,std::string _perm){
    auto cmds = input_to_commands(fname);
    std::string * perm = new std::string(_perm);
    auto exc = [&](){
    for(auto a: cmds){
        a->exec(perm);
    }};

    bool found_flag = true;
#define MAXITER 1000000000
    uint64_t idx_stop = MAXITER;
    for(uint64_t i=1;i<idx_stop;i++){
        exc();
        if (found_flag && *perm == _perm){ // i think its guaranteed for the
            idx_stop = i ? (MAXITER % i+1) : 0; // change loop stop condition
            i = 0;
            found_flag = false;
        }
    }
    return  *perm;
}

int main (int argc, char *argv[]) {
    std::cout << "Part 1 -> "<< part1("./test","abcde") << std::endl;
    std::cout << "Part 2 -> "<< part2("./test","abcde") << std::endl;
    std::cout << "Part 2 fast -> "<< part2_fast("./test","abcde") << std::endl;

    std::cout << "Part 1 -> "<< part1("./input","abcdefghijklmnop") << std::endl;
    std::cout << "Part 2 -> "<< part2("./input","abcdefghijklmnop") << std::endl;
    std::cout << "Part 2 fast -> "<< part2_fast("./input","abcdefghijklmnop") << std::endl;
    return 0;
}
