/* #include <stdio.h> */
#include <cctype>
#include <cstddef>
#include <iterator>
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
    Add=1,
    Jgz,
    Mod,
    Mul,
    Rcv,
    Set,
    Snd,
    Non,
};

class State {
    friend class Instr;
    std::map<std::string,int> memory;
    int lastEmited;
    int ip;
    public:
    State(): memory({}), lastEmited(0), ip(0){};
    int IP(){
        return this->ip;
    }
    int getToken(std::string tok_key) {
        if (tok_key.length()>0 && (std::isdigit(tok_key[0]) || tok_key[0] == '-' )){
            int ret;
            sscanf(tok_key.c_str(), "%d",&ret);
            /* printf("parsed %s -> %d\n",tok_key.c_str(),ret); */
            return ret;
        } else if(tok_key.length()>0) {
            if (this->memory.find(tok_key)!= memory.end()){
                this->memory.insert({tok_key,0});
            }
            return this->memory[tok_key];
        } else {
            throw "Wtf";
        }
    }

    void setToken(std::string tok_key,int val) {
        if (tok_key.length() && (std::isdigit(tok_key[0]) || tok_key[0] == '-' )){
            return;
        } else if(tok_key.length()>0) {
            if (this->memory.find(tok_key)!= memory.end()){
                this->memory.insert({tok_key,0});
            }
            this->memory[tok_key] = val;
        } else {
            throw "Wtf";
        }
    }
};


class Instr {
    private:
    public:
    static std::map<std::string,Command> cmd_translate;
    Command cmd;
    std::string left;
    std::string right;
    Instr(): cmd(Command::Non){};
    ~Instr(){};
    Instr(std::string inp) {
        this->cmd = Command::Non;
        std::stringstream iinp(inp);
        std::string tmp;
        if (iinp >> tmp) {
            this->cmd = cmd_translate[tmp];
        }

        if (iinp >> tmp) {
            this->left = tmp;
        }

        if (iinp >> tmp) {
            this->right = tmp;
        }
    };
    int exec(State *s){
        int ip_increment = 1;
        int NZrecieved = 0 ;
        /* std::cout<< s->ip << " " << this->cmd <<" "<<this->left <<" "<<this->right <<" "<<ip_increment<<" " << s->getToken("a") << std::endl; */
        switch (this->cmd) {
            case Add:
                try  {
                    s->setToken(this->left, s->getToken(this->left) + s->getToken(this->right));
                }
                catch (...){printf("in line : %d %s %s \n",__LINE__,this->left.c_str(),this->right.c_str()); }
                break;
            case Jgz:
                try {
                    if (s->getToken(this->left)>0)
                        ip_increment = s->getToken(this->right);
                }
                catch (...){printf("in line : %d \n",__LINE__); }
                break;
            case Mod:
                try {
                    s->setToken(this->left, s->getToken(this->left) % s->getToken(this->right));
                }
                catch (...){printf("in line : %d \n",__LINE__); }
                break;
            case Mul:
                try {
                    s->setToken(this->left, s->getToken(this->left) * s->getToken(this->right));
                }
                catch (...){printf("in line : %d \n",__LINE__); }
                break;
            case Rcv:
                try {
                    if (s->lastEmited > 0) {
                        NZrecieved = s->lastEmited;
                        /* std::cout << "value of rcv " << s->lastEmited << "\n"; */
                        s->setToken(this->left,s->lastEmited);
                    }
                }
                catch (...){printf("in line : %d \n",__LINE__); }
                break;
            case Set:
                try{
                   s->setToken(this->left, s->getToken(this->right));
                }
                catch (...){printf("in line : %d \n",__LINE__); }
                break;
            case Snd:
                try {
                    s->lastEmited = s->getToken(this->left);
                }
                catch (...) {printf("in line %d",__LINE__);}
                break;
            case Non:
                /* throw "Panic"; */
                break;
        }
        s->ip+=ip_increment;
        return NZrecieved;
    }
};

std::map<std::string,Command> Instr::cmd_translate = {
            {"non",Non},
            {"add",Add},
            {"jgz",Jgz},
            {"mod",Mod},
            {"mul",Mul},
            {"rcv",Rcv},
            {"set",Set},
            {"snd",Snd},
        };

int part1(std::string fname){
    std::ifstream in(fname);
    std::string line;
    std::vector<Instr> tape = {};
    State state;

    while(std::getline(in,line)){
        tape.push_back(Instr(line));
    }

    while(state.IP() < tape.size()){
        int emmited = tape[state.IP()].exec(&state);
        if (emmited) return  emmited;
    };
    return 0;
}

int main (int argc, char *argv[]) {
    std::cout << "Part 1 -> "<< part1("./test") << std::endl;
    std::cout << "Part 1 -> "<< part1("./input") << std::endl;

/*     std::cout << "Part 1 -> "<< part1("./input") << std::endl; */
/*     std::cout << "Part 2 -> "<< part2("./input") << std::endl; */
/*     std::cout << "Part 2 fast -> "<< part2_fast("./input") << std::endl; */
    return 0;
}
