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
    Mod,
    Mul,
    Rcv,
    Set,
    Snd,
    Non,
};

class State {
    friend class Instr;
    std::map<std::string,int64_t> memory;
    std::queue<int64_t> lastEmited;
    bool waiting;
    int sends ;
    int ip;
    public:
    State(): memory({}), lastEmited(), ip(0), sends(0){};
    int IP(){
        return this->ip;
    }
    int Sends(){
        return this->sends;
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

    bool isWaiting(){
        return this->waiting;
    }

    void chanSend(State *other,uint64_t v){
        if (this == other) { // we are sending to self
            while (!this->lastEmited.empty()) {
                this->lastEmited.pop();
            }
            this->lastEmited.push(v);
        }
        else { // we are sending from another 'thread'
            this->sends++;
            other->lastEmited.push(v);
            other->waiting=false;
            /* printf("size %d of queue in ptr %p\n",(int)lastEmited.size(),other); */
        }
    }

    bool chanRecv(uint64_t *v){
        if (!this->lastEmited.empty()){
            *v = this ->lastEmited.front();
            this->lastEmited.pop();
            return true;
        } else {
            this->waiting=true;
            return false;
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
            case Add:
                s->setToken(this->left, s->getToken(this->left) + s->getToken(this->right));
                break;
            case Jgz:
                if (s->getToken(this->left)>0) {
                    int jmp = s->getToken(this->right); 
                    s->ip += s->getToken(this->right);
                    return 0 ;
                }
                break;
            case Mod:
                s->setToken(this->left, s->getToken(this->left) % s->getToken(this->right));
                break;
            case Mul:
                s->setToken(this->left, s->getToken(this->left) * s->getToken(this->right));
                break;
            case Rcv:
                do {
                    s->dumpMem();
                    /* std::cout << "try recieve  pred " << s->getToken(this->left) <<" "<<this->left <<"\n"; */
                    uint64_t ret = 0;
                    s->chanRecv(&ret);
                    if (s->getToken(this->left) != 0) {
                        return ret;
                    }
                    return ret;
                } while(0);
                break;
            case Set:
                s->setToken(this->left, s->getToken(this->right));
                break;
            case Snd:
                {
                    /* std::cout << "sendig " << s->getToken(this->left) <<"\n"; */
                    s->chanSend(s, s->getToken(this->left));
                }
                break;
            case Non:
                /* throw "Panic"; */
                break;
        }
        s->ip++;
        return NZrecieved;
    }

    int exec_part2(State *ths,State *other, int nr ){
        int NZrecieved = 0 ;
        /* std::cout<< ths->ip << "\t" << this->cmd <<"\t"<<this->left <<"\t"<<this->right  << std::endl; */
        switch (this->cmd) {
            case Add:
                ths->setToken(this->left, ths->getToken(this->left) + ths->getToken(this->right));
                break;
            case Jgz:
                if (ths->getToken(this->left)>0) {
                    int jmp = ths->getToken(this->right); 
                    ths->ip += ths->getToken(this->right);
                    return 0 ;
                }
                break;
            case Mod:
                ths->setToken(this->left, ths->getToken(this->left) % ths->getToken(this->right));
                break;
            case Mul:
                ths->setToken(this->left, ths->getToken(this->left) * ths->getToken(this->right));
                break;
            case Rcv:
                do {
                    std::cout << "try recieve in "<<nr<<" pred in " <<  this->left <<"\n";
                    uint64_t ret = 0;
                    if (ths->chanRecv(&ret)) {
                        std::cout << "before ";
                        ths->dumpMem();
                        ths -> setToken(this->left, ret);
                        ths -> waiting = false;
                        std::cout << "after ";
                        ths->dumpMem();
                    } else {
                        return 0;
                    }
                } while(0);
                break;
            case Set:
                ths->setToken(this->left, ths->getToken(this->right));
                break;
            case Snd:
                {
                    printf("sending %d from %d \n",(int)ths->getToken(this->left),nr);
                    ths->chanSend(other, ths->getToken(this->left));
                }
                break;
            case Non:
                break;
        }
        ths->ip++;
        return 0;
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

    while(-1 <state.IP() && state.IP() < tape.size()){
        int emmited = tape[state.IP()].exec_part1(&state);
        if (emmited) return  emmited;
    };
    return 0;
}

int part2(std::string fname){
    std::ifstream in(fname);
    std::string line;
    std::vector<Instr> tape = {};
    State thr0;
    State thr1;
    thr0.setToken("p", 0);
    thr1.setToken("p", 1);

    while(std::getline(in,line)){
        tape.push_back(Instr(line));
    }

    auto is_not_finished =[&](State *t){
        return -1 <t->IP() && t->IP() < tape.size() && !t->isWaiting();
    };
    /* return 0; */
    while( is_not_finished(&thr1) || is_not_finished(&thr0) ){
        tape[thr0.IP()].exec_part2(&thr0,&thr1,0);
        /* while (is_not_finished(&thr1)) { */
        tape[thr1.IP()].exec_part2(&thr1,&thr0,1);
        /* } */
        if (thr1.isWaiting() && thr0.isWaiting()) {
            printf("deadlock %d %d\n",thr0.Sends(),thr1.Sends());
        }
    };
    return thr1.Sends();
}

int main (int argc, char *argv[]) {
    /* std::cout << "Part 1 -> "<< part1("./test") << std::endl; */
    /* std::cout << "Part 1 -> "<< part1("./input") << std::endl; */

    /* std::cout << "Part 2 -> "<< part2("./test2") << std::endl; */
    std::cout << "Part 2 -> "<< part2("./input") << std::endl;

/*     std::cout << "Part 1 -> "<< part1("./input") << std::endl; */
/*     std::cout << "Part 2 -> "<< part2("./input") << std::endl; */
/*     std::cout << "Part 2 fast -> "<< part2_fast("./input") << std::endl; */
    return 0;
}
