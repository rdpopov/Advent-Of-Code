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

#define TEST

#ifdef TEST
#define INPUT "test"
#else
#define INPUT "input"
#endif

std::map<int, std::set<int>> parse_file (std::string fname){
    std::ifstream in (fname);
    std::string ln;
    std::map<int, std::set<int>> trees;

    while (std::getline(in,ln,'\n')) {
        ln.erase(remove(ln.begin(),ln.end(),','),ln.end());
        std::stringstream line(ln);
        std::string tok;
        std::vector<int> nums;

        while (std::getline(line,tok,' ')) {
            if (tok== "<->") continue;
            int prs;
            sscanf(tok.c_str(),"%d",&prs);
            nums.push_back(prs);
        }

        int head = nums[0];
        std::set<int> items_set({});
        items_set.insert(nums.begin(),nums.end());
        trees.insert({head,items_set}); // no point in removing the first number
                                        // since it shoud contain itself
    }
    return trees;
}

int32_t part1(std::string inp) {
    std::map<int, std::set<int>> trees=parse_file(inp);
    int set_size = 0;
    std::set<int> visited({0});
    visited.insert(trees[0].begin(),trees[0].end());

    while (set_size!= visited.size()){
        set_size = visited.size();
        for (auto &a: trees ){
            if (visited.find(a.first)== visited.end()){
                for (auto &xs: a.second ){
                    if (visited.find(xs)!= visited.end()){
                        visited.insert(a.second.begin(),a.second.end());
                        break;
                    }
                }
            }
        }
    }
    return visited.size();
}

int32_t part2(std::string inp) {
    std::map<int, std::set<int>> trees=parse_file(inp);

    int count;
    for (count = 0; trees.size()>0;count++){
        int set_size = 0;
        std::set<int> visited({});
        int start = trees.begin()->first;
        if (trees[start].size() == 1){ // if the group is magnitute 1 then it
                                       // just contains itself
                                       // so we can just exclude it and skip
            trees.erase(trees.find(start));
            continue;
        }
        visited.insert(trees[start].begin(),trees[start].end());
        while (set_size!= visited.size()){
            set_size = visited.size();
            for (auto &a: trees ){
                if (visited.find(a.first)== visited.end()){
                    for (auto &xs: a.second ){
                        if (visited.find(xs)!= visited.end()){
                            visited.insert(a.second.begin(),a.second.end());
                            break;
                        }
                    }
                }
            }
        }
        // inneficient af but who cares
        // pc fan go brr
        for (auto er: visited){
            trees.erase(trees.find(er));
        }
    }
    return count;
}


int main (int argc, char *argv[]) {
    std::cout << "Part 1 -> "<< part1("./test") << std::endl;
    std::cout << "Part 1 -> "<< part1("./input") << std::endl;
    std::cout << "Part 2 -> "<< part2("./test") << std::endl;
    std::cout << "Part 2 -> "<< part2("./input") << std::endl;
    return 0;
}
