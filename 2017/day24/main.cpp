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

#define pr(V) { for(auto a:V) std::cout<< a << " "; std::cout<< "\n"; }

int calc(std::vector<int> &route, std::map<int,std::vector<int>> &nodes){
    int ret = 0;
    std::set<int> loops;
    if (route.size() == 1){
        return 0;
    };
    for (auto a:route){
        ret += 2*a;
        for(auto o: nodes[a]){
            if(o == a){
                // loops
                loops.insert(a);
                break;
            }
        }
    };
    for (auto a: loops){
        ret += 2*a;
    }
    ret -= route[0];
    ret -= route[route.size()-1];
    return ret;
}

bool in_vec(int n, std::vector<int> &route){
    for(auto a:route){
        if (n == a ){
            return true;
        }
    }
    return false;
}

int traverse(int start,std::map<int,std::vector<int>> &nodes){
    std::queue<std::vector<int>> que;
    std::vector<int> max_path;
    int max_cost = 0;
    que.push({start});
    while (que.size()) {
        std::vector<int> crnt = que.front();
        /* pr(crnt); */
        //see all the children of the last elemnt visited
        bool final = true;
        for(auto a: nodes[crnt[crnt.size()-1]]) {
            if (!in_vec(a,crnt)){
                std::vector<int> new_path(crnt);
                new_path.push_back(a);
                que.push(new_path);
                final = false;
            }
        }
        if(final) {
            max_cost = std::max(max_cost,calc(crnt,nodes));
            std::cout<<max_cost << " -> ";
            pr(crnt);
        }
        que.pop();
    }
    return max_cost;
}

int part1(std::string fname) {
    std::ifstream in(fname);
    std::string line;
    std::map<int,std::vector<int>> nodes;
    int max_cost = 0;

    while(std::getline(in,line)){
        int from;
        int to;
        sscanf(line.c_str(),"%d/%d" ,&from,&to);
        if (nodes.find(to) == nodes.end()){
            nodes[to] = {};
        }
        if (nodes.find(from) == nodes.end()){
            nodes[from] = {};
        }

        if (from == to){
            nodes[to].push_back(to);
        } else {
            nodes[to].push_back(from);
            nodes[from].push_back(to);
        }
    }

    for (auto a : nodes) {
        std::cout << a.first << " -> ";
        pr(a.second);
    }
    return traverse(0, nodes);
}




int main (int argc, char *argv[]) {
    /* std::cout << "Part 1 -> "<< part1("./test") << std::endl; */
    std::cout << "Part 1 -> "<< part1("./input") << std::endl;

    return 0;
}
