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

using pnt = std::pair<int,int>;
using vecpnt = std::vector<pnt>;

enum Dirs{
    UP = 0,
    LEFT,
    DOWN,
    RIGHT,
};

vecpnt tr_vec ={
    {-1,0}, /* UP = 0, */
    {0,-1}, /* LEFT, */
    {1,0}, /* DOWN, */
    {0,1}, /* RIGHT, */
};

Dirs turnLeft[] = {LEFT,DOWN,RIGHT,UP};
Dirs turnRight[] = {RIGHT,UP,LEFT,DOWN};


std::pair<std::string,int> traverse_part1(std::vector<std::string> &matr) {
    int idx=0;
    int iterations=0;
    std::string res = "";
    for(;idx < matr[0].length() && matr[0][idx]!='|'; idx++ );
    pnt crnt = {0,idx}; // y x
    Dirs d = Dirs::DOWN;

    auto legal = [&](pnt crnt){
        return
            0 <= crnt.first && crnt.first < matr.size() &&
            0 <= crnt.second && crnt.second < matr[0].size();
    };

    auto move = [&](pnt crnt,Dirs dir){
        return pnt{crnt.first + tr_vec[dir].first, crnt.second + tr_vec[dir].second};
    };

    while (1) {
        switch (matr[crnt.first][crnt.second]) {
            case ' ':
                return {res,iterations};
            case 'A'... 'Z':
                {
                    res.push_back(matr[crnt.first][crnt.second]);
                } // fallthrough
            case '-':
            case '|':
                crnt = move(crnt, d);
                break;
            case '+':
                {
                    pnt nextLeft = move(crnt, turnLeft[d]);
                    pnt nextRight = move(crnt, turnRight[d]);

                    if (legal(nextLeft) && matr[nextLeft.first][nextLeft.second] !=' '){
                        crnt = nextLeft;
                        d = turnLeft[d];
                        break;
                    }

                    if (legal(nextRight) && matr[nextRight.first][nextRight.second] !=' '){
                        crnt = nextRight;
                        d = turnRight[d];
                        break;
                    }
                }
            default:
                break;
        }
        /* printf("{%d,%d}\n",crnt.first,crnt.second); */
        iterations++;
    }
    return {res,iterations};
}

std::string part1(std::string fname) {
    std::ifstream inp(fname);
    std::string tmp;
    std::vector<std::string> matr;

    while (std::getline(inp,tmp,'\n')) {
        matr.push_back(tmp);
    }

    for (auto &a : matr){
        /* std::cout << a << "\n"; */
    }

    return traverse_part1(matr).first;
}

int part2(std::string fname) {
    std::ifstream inp(fname);
    std::string tmp;
    std::vector<std::string> matr;

    while (std::getline(inp,tmp,'\n')) {
        matr.push_back(tmp);
    }

    for (auto &a : matr){
        /* std::cout << a << "\n"; */
    }

    return traverse_part1(matr).second;
}

int main (int argc, char *argv[]) {
    std::cout << "Part 1 -> "<< part1("./test") << std::endl;
    std::cout << "Part 1 -> "<< part1("./input") << std::endl;
    std::cout << "Part 2 -> "<< part2("./test") << std::endl;
    std::cout << "Part 2 -> "<< part2("./input") << std::endl;
    return 0;
}
