/* #include <stdio.h> */
#include <cctype>
#include <cstddef>
#include <ios>
#include <iterator>
#include <ostream>
#include <random>
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
#include <utility>
#include <valarray>
#include <vector>
#include <map>
#include <queue>
#include <set>
#include <cmath>

enum Direction {
    Up=0,
    Left,
    Down,
    Right,
};

std::pair<int, int> dirs[4] {
//    x, y
    { 0,  -1}, // Up=0,
    { -1, 0}, // Left,
    { 0,  1}, // Down,
    { 1,  0}, // Right,
};


int part1(std::string fname,int iterations){
    std::ifstream in(fname);
    std::string line;
    std::vector<std::string> primitive_map;
    std::map<std::pair<int,int>,char> grid;
    int act = 0;

    while(std::getline(in,line)) {
        primitive_map.push_back(line);
    }
    int dx,dy;
    dx = primitive_map[0].size()/2;
    dy = primitive_map.size()/2;
    for(int y = 0;y<primitive_map.size();y++) {
        for(int x = 0;x<primitive_map[0].size();x++) {
            grid[{x-dx,y-dy}] = primitive_map[y][x];
        }
    }

    std::pair<int, int> inf = {0,0};
    int dir = Direction::Up;
    for(int i=0; i < iterations; i++) {
        if(grid[inf] == '#') {
            grid[inf] = '.';
            dir = (dir+3)%4;
            inf.first += dirs[dir].first;
            inf.second += dirs[dir].second;
        } else {
            // is empty
            grid[inf] = '#';
            dir = (dir+1)%4;
            inf.first += dirs[dir].first;
            inf.second += dirs[dir].second;
            act ++;
        }
    }

    return act;
}


int part2(std::string fname,uint64_t iterations){
    std::ifstream in(fname);
    std::string line;
    std::vector<std::string> primitive_map;
    std::map<std::pair<uint64_t,uint64_t>,char> grid;
    int act = 0;

    while(std::getline(in,line)) {
        primitive_map.push_back(line);
    }
    int dx,dy;
    dx = primitive_map[0].size()/2;
    dy = primitive_map.size()/2;
    for(int y = 0;y<primitive_map.size();y++) {
        for(int x = 0;x<primitive_map[0].size();x++) {
            grid[{x-dx,y-dy}] = primitive_map[y][x];
        }
    }

    std::pair<uint64_t, uint64_t> inf = {0,0};
    int dir = Direction::Up;
    for(uint64_t i=0; i < iterations; i++) {
        switch (grid[inf]) {
            case '#':
                {
                    dir = (dir+3)%4;
                    grid[inf] = 'F';
                    break;
                }
            case 'W':
                {
                    act ++;
                    grid[inf] = '#';
                    break;
                }
            case 'F':
                {
                    dir = (dir+2)%4; // goes back
                    grid[inf] = '.';
                    break;
                }
            default:
                grid[inf] = 'W';
                dir = (dir+1)%4; // left
        }
        inf.first += dirs[dir].first;
        inf.second += dirs[dir].second;
    }

    return act;
}

int main (int argc, char *argv[]) {
    std::cout << "Part 1 (test) -> 5 == "<< part1("./test",7) << std::endl;
    std::cout << "Part 1 (test) -> 41 == "<< part1("./test",70) << std::endl;
    std::cout << "Part 1 (test) -> 5587 == "<< part1("./test",10000) << std::endl;
    std::cout << "Part 1 -> "<< part1("./input",10000) << std::endl;
    std::cout << "Part 2 (test) -> 26 == "<< part2("./test",100) << std::endl;
    std::cout << "Part 2 (test) -> 2511944 == "<< part2("./test",10000000) << std::endl;
    std::cout << "Part 2 -> "<< part2("./input",10000000) << std::endl;
    return 0;
}
