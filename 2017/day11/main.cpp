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

#define TEST

#ifdef TEST
#define INPUT "test"
#else
#define INPUT "input"
#endif
int32_t distance(std::pair<int,int> fst,std::pair<int,int> sec){

  int dx = abs(fst.first - sec.first);
  int dy = abs(fst.second - sec.second);

  if (dx == 0) {
    return dy;
  } else if (dy == 0) {
    return dx;
  } else {
    if (sec.first < fst.first && sec.second < fst.second) { // empty Corner
      return dx + dy;
    } else if (sec.first < fst.first &&
               sec.second > fst.second) { // Filled Corner
      return std::max(dx, dy);
    } else if (sec.first > fst.first &&
               sec.second < fst.second) { // Filled Corner
      return std::max(dx, dy);
    } else if (sec.first > fst.first &&
               sec.second > fst.second) { // empty Corner
      return dx + dy;
    } else
      return 0;
  }
}

int32_t part1(std::string inp) {
    std::map<std::string, std::vector<int>> swc={};
    swc["n"]  = {0,1};
    swc["s"]  = {0,-1};

    swc["ne"] = {1,0};
    swc["nw"] = {-1,1};
    swc["se"] = {1,-1};
    swc["sw"] = {-1,0};

    std::vector<int> pos={0,0};

    std::ifstream in (inp);
    std::string c;
    while (std::getline(in,c,',')) {
        if (c.back() == '\n'){
            c.resize(c.length()-1);
        }
        auto mv = swc[c];
        pos[0] += mv[0];
        pos[1] += mv[1];
    }

    // minimum of the length of the complement?
    for (auto a: pos){std::cout<< a <<" "; }
    return distance({0,0}, {pos[0],pos[1]});
}

int32_t part2(std::string inp) {
    std::map<std::string, std::vector<int>> swc={};
    swc["n"]  = {0,1};
    swc["s"]  = {0,-1};

    swc["ne"] = {1,0};
    swc["nw"] = {-1,1};
    swc["se"] = {1,-1};
    swc["sw"] = {-1,0};

    std::vector<int> pos={0,0};

    std::ifstream in (inp);
    std::string c;
    int res;
    while (std::getline(in,c,',')) {
        if (c.back() == '\n'){
            c.resize(c.length()-1);
        }
        auto mv = swc[c];
        pos[0] += mv[0];
        pos[1] += mv[1];
        res = std::max(res,distance({0,0}, {pos[0],pos[1]}));
    }

    // minimum of the length of the complement?
    for (auto a: pos){std::cout<< a <<" "; }
    return res;
}
int main (int argc, char *argv[]) {
    std::cout << part1("./input") << std::endl;
    std::cout << part2("./input") << std::endl;
    return 0;
}
