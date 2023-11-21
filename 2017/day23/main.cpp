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
#include <queue>
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
    Jnz,
    Mod,
    Mul,
    Rcv,
    Set,
    Snd,
    Sub,
    Non,
};

class State {
    friend class Instr;
    std::map<std::string,int64_t> memory;
    std::queue<int64_t> lastEmited;
    bool waiting;
    int sends ;
    int muls ;
    int ip;
    public:
    State(): memory({}), lastEmited(), ip(0), sends(0), muls(0){};
    int IP(){
        return this->ip;
    }
    int Sends(){
        return this->sends;
    }

    int Muls(){
        return this->muls;
    }
    int64_t getToken(std::string tok_key) {
        if (tok_key.length()>0 && (std::isdigit(tok_key[0]) || tok_key[0] == '-' )){
            int ret;
            sscanf(tok_key.c_str(), "%d",&ret);
            /* printf("parsed %s -> %d\n",tok_key.c_str(),ret); */
            return ret;
        } else if(tok_key.length()>0) {
            if (this->memory.find(tok_key) == memory.end()){
                this->memory.insert({tok_key,0});
            }
            return this->memory[tok_key];
        } else {
            throw "Wtf";
        }
    }

    void setToken(std::string tok_key,int64_t val) {
        if (tok_key.length() && (std::isdigit(tok_key[0]) || tok_key[0] == '-' )){
            return;
        } else if(tok_key.length()>0) {
            this->memory[tok_key] = val;
        } else {
            throw "Wtf";
        }
    }

    void dumpMem(){
        std ::cout<<"memmory"<<std::endl;
        for (auto a:this->memory) {
            std::cout <<"("<< a.first <<" "<<a.second<<") ";
        }
        std ::cout<<std::endl;
        // impl dumping of mem
    }
};


class Instr {
    private:
    public:
    static std::map<std::string,Command> cmd_translate;
    Command cmd;
    std::string left;
    std::string right;
    Instr(): cmd(Command::Non) ,left(""), right(""){};
    ~Instr(){};
    Instr(std::string inp) {
        this->cmd = Command::Non;
        std::stringstream iinp(inp);
        std::string tmp;
        if (iinp >> tmp) {
            this->cmd = cmd_translate[tmp];
        }

        if (iinp >> tmp) {
            this->left  = tmp;
        } else {
            this->left="";
        }

        if (iinp >> tmp) {
            this->right = tmp;
        } else {
            this->right="";
        }
    };

    int exec_part1(State *s){
        int NZrecieved = 0 ;
        /* std::cout<< s->ip << "\t" << this->cmd <<"\t"<<this->left <<"\t"<<this->right  << std::endl; */
        switch (this->cmd) {
            case Sub:
                s->setToken(this->left, s->getToken(this->left) - s->getToken(this->right));
                break;
            case Jnz:
                if (s->getToken(this->left)!=0) {
                    int jmp = s->getToken(this->right); 
                    s->ip += s->getToken(this->right);
                    return 0 ;
                }
                break;

            case Mul:
                s->muls++;
                s->setToken(this->left, s->getToken(this->left) * s->getToken(this->right));
                break;
            case Set:
                s->setToken(this->left, s->getToken(this->right));
                break;
            case Non:
                break;
        }
        s->ip++;
        return NZrecieved;
    }

};

std::map<std::string,Command> Instr::cmd_translate = {
            {"non",Non},
            {"add",Add},
            {"sub",Sub},
            {"jgz",Jgz},
            {"jnz",Jnz},
            {"mod",Mod},
            {"mul",Mul},
            {"rcv",Rcv},
            {"set",Set},
            {"snd",Snd},
        };

int part1(std::string fname) {
    std::ifstream in(fname);
    std::string line;
    std::vector<Instr> tape = {};
    State state;

    while(std::getline(in,line)){
        tape.push_back(Instr(line));
    }

    while(-1 <state.IP() && state.IP() < tape.size()){
       tape[state.IP()].exec_part1(&state);
    };
    return state.Muls();
}

int part2(std::string fname) {
    std::ifstream in(fname);
    std::string line;
    std::vector<Instr> tape = {};
    State state;

    state.setToken("a", 1);

    while(std::getline(in,line)){
        tape.push_back(Instr(line));
    }

    while(-1 <state.IP() && state.IP() < tape.size()){
       tape[state.IP()].exec_part1(&state);
    };
    return state.getToken("h");
}
int primenumber(int64_t number)
{
    for(int i=1; i<number; i++)
    {
       if(number%i==0)
           return 0;
    }  
    return 1;
}
int tmp(){
    int64_t b = (57 * 100) + 100000;
    int64_t c = b + 17000;
    int h = 0;
    do{
        if(primenumber(b)) h++;
        if (b-c == 0){
            return h;
        } else {
            b += 17;
        }
    } while (true);
}

int main (int argc, char *argv[]) {
    std::cout << "Part 1 -> "<< part1("./input") << std::endl;
    std::cout << "Part 2 -> "<< part2("./input2") << std::endl;
    std::cout << tmp();

    return 0;
}
