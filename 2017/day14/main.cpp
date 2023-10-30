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
#include <tuple>
#include <utility>
#include <vector>
#include <map>
#include <set>
#include <cmath>

#define UP_BITS(t) (!!(t & 0x1) + !!(t & 0x2) + !!(t & 0x4) + !!(t & 0x8))

using vec = std::vector<int>;
using mat =std::vector<std::vector<int>>;
using pairset = std::set<std::pair<int,int>>;
void swap_part1 (std::vector<int> *tape, int low, int size){
    auto norm = [&](int a){
        a += tape->size();
        a %= tape->size();
        return  a;
    };
    std::vector<int> store = {};
    for (int i = 0 ; i < size; i ++){
        int tmp = (*tape)[norm(low+i)];
        store.push_back(tmp);
    }
    int a = 0;
    for (auto i = store.rbegin() ; i != store.rend(); i++,a++ ){
        (*tape)[norm(low+a)] = *i ;
    }
}

int32_t hash(vec *tape,vec *instructions, int *idx, int *skip  ) {
    int length = tape->size();
    for (auto a: *instructions) {
        swap_part1(tape,*idx,a);
        *idx = (*idx + a + *skip)%length;
        (*skip)++;
    }
    return  (*tape)[0] * (*tape)[1];
}

std::string hex_code_impl(vec instructions, size_t length) {
    vec tape;
    int idx = 0;
    tape.resize(length);
    for (auto &t: tape){
        t = idx++;
    }
    idx = 0;
    int skip = 0;

    // 64 times the hash
    for (int i=0; i<64; i++)
        hash(&tape, &instructions, &idx, &skip);

    std::stringbuf out ;
    char res[16*2 + 1 ];
    res [16*2] = 0;
    char* tmp = res;
    for(int x=0; x<16;x++) {
        int dense = tape[x*16];
        for (int i=1; i<16; i++)
            dense = dense ^ tape[x*16 + i];
        sprintf(tmp, "%02x", dense); // ol c rules
        tmp += 2;
    }
    return std::string(res);
}

std::string hex_code(std::string input){
    std::string word;
    std::vector<int> res = {};
    for(auto a: input)
        res.push_back(a);
    res.push_back(17);
    res.push_back(31);
    res.push_back(73);
    res.push_back(47);
    res.push_back(23);
    return hex_code_impl(res,256);
}

int32_t part1(std::string inp) {
    /* std::cout<<hex_code(inp); */
    int res = 0;
    for(int i= 0; i<128;i++){
        std::string to_hash = inp + "-" + std::to_string(i);
        std::string hsh = hex_code(to_hash);
        for (auto a: hsh){
            int value ;
            switch (a) {
                case 'a':
                case 'b':
                case 'c':
                case 'd':
                case 'e':
                case 'f':
                    value = a -'a' + 10;
                    break;
                default:
                    value = a -'0';
            }
            res += UP_BITS(value);
        }
    }
    return res;
}
#define VALID(p) (0<=p.first && p.first <=127  && 0<= p.second && p.second < 128)
void bfs(mat *mesh,std::pair<int,int> origin) {
    pairset visited;
    pairset to_visit{origin};
    while (!to_visit.empty()){
        std::pair<int, int> cn = *to_visit.begin();
        visited.insert(cn);
        std::vector<std::pair<int,int>> neigh = {
            {cn.first+0,cn.second+1},
            {cn.first+1,cn.second+0},
            {cn.first+0,cn.second-1},
            {cn.first-1,cn.second+0},
        };
        for (auto a: neigh){
            if (VALID(a) && (visited.find(a)==visited.end()) && (*mesh)[a.first][a.second]!=0){
                to_visit.insert(a);
            }
        }
        to_visit.erase(cn);
    }
    // clean up current group
    for (auto a: visited){
       (*mesh)[a.first][a.second] = 0;
    }
}

int32_t part2(std::string inp) {
    int res=0;
    // prepare mesh
    mat mesh = {};
    mesh.reserve(128);
    for(int i= 0; i<128;i++){
        std::string to_hash = inp + "-" + std::to_string(i);
        std::string hsh = hex_code(to_hash);
        std::vector<int> crnt_line;
        crnt_line.reserve(128);
        for (auto a: hsh){
            int value ;
            switch (a) {
                case 'a':
                case 'b':
                case 'c':
                case 'd':
                case 'e':
                case 'f':
                    value = a -'a' + 10;
                    break;
                default:
                    value = a -'0';
            }
            if (value & 0x8) crnt_line.push_back(1);
            else  crnt_line.push_back(0);
            if (value & 0x4) crnt_line.push_back(1);
            else  crnt_line.push_back(0);
            if (value & 0x2) crnt_line.push_back(1);
            else  crnt_line.push_back(0);
            if (value & 0x1) crnt_line.push_back(1);
            else  crnt_line.push_back(0);
        }

        mesh.push_back(crnt_line);
    }

    // kind of bfs
    for (int y=0; y<128;y++){
        for (int x=0; x<128;x++){
            if (mesh[y][x] == 1){
                bfs(&mesh,{y,x});
                res++;
            }
        }
    }

    return res;
}

int main (int argc, char *argv[]) {
    std::cout << "Part 1 -> "<< part1("flqrgnkx") << std::endl;
    std::cout << "Part 1 -> "<< part1("uugsqrei") << std::endl;
    std::cout << "Part 2 -> "<< part2("flqrgnkx") << std::endl;
    std::cout << "Part 2 -> "<< part2("uugsqrei") << std::endl;
    return 0;
}
