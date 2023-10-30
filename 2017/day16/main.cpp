/* #include <stdio.h> */
#include <bits/types/struct_tm.h>
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

std::string spin(std::string inp,int cycle){
    int idx = 0;
    std::string res(inp);
    for (auto a: inp ){
        res[(idx+cycle)%inp.length()] = a;
        idx++;
    }
    return res;
}

std::string part1 (std::string fname,std::string perm){
    std::ifstream in (fname);
    std::string ln;
    std::string res(perm);

    while (std::getline(in,ln,'\n')) {
        std::stringstream line(ln);
        std::string tok;
        while (std::getline(line,tok,',')) {
            std::cout<< tok<< "\n";
            switch (tok[0]) {
                case 's':{
                    int cycle;
                    sscanf(tok.c_str(),"s%d",&cycle);
                    res = spin(res,cycle);
                    break;
                    }
                case 'x':{
                    int x1;
                    int x2;
                    sscanf(tok.c_str(),"x%d/%d",&x1,&x2);
                    char tmp = res[x1];
                    res[x1]= res[x2];
                    res[x2]= tmp;
                    break;}
                case 'p':{
                    char c1;
                    char c2;
                    sscanf(tok.c_str(),"x%c/%c",&c1,&c2);
                    int swch_pos_c1 = res.find(c1);
                    int swch_pos_c2 = res.find(c2);
                    res[swch_pos_c1]= c2;
                    res[swch_pos_c2]= c1;
                    break;
                         }
            }
        }
    }
    return res;
}

int main (int argc, char *argv[]) {
part1("./test","abcde");
    std::cout << "Part 1 -> "<< part1("./test","abcde") << std::endl;
    /* std::cout << "Part 1 -> "<< part1("./input") << std::endl; */
    return 0;
}
