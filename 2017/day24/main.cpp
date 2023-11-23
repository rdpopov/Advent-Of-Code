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
#include <utility>
#include <vector>
#include <map>
#include <queue>
#include <set>
#include <cmath>

#define pr(V) { for(auto a:V) std::cout<< a << " "; std::cout<< "\n"; }

std::pair<int,int> calc(std::vector<int> &route, std::map<int,std::vector<int>> &nodes){
    int ret = 0;
    std::set<int> loops;
    if (route.size() == 1){
        return {0,0};
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
    return {ret,route.size() + loops.size()};
}

bool in_vec(int n, std::vector<int> &route){
    for(auto a:route){
        if (n == a ){
            return true;
        }
    }
    return false;
}
bool pair_in_vec(std::pair<int,int> p, std::vector<int> &route){
    if(p.first == p.second) return true;
    for(int i = 1; i < route.size();i++)
    {
        if(p.first == route[i-1] &&
           p.second == route[i] ||
           p.first == route[i] &&
           p.second == route[i-1]) 
            return true;
    }
    return false;
}

int traverse_part1(int start,std::map<int,std::vector<int>> &nodes){
    std::queue<std::vector<int>> que;
    std::vector<int> max_path;
    int max_cost = 0;
    que.push({start});
    while (que.size()) {
        std::vector<int> crnt = que.front();
        //see all the children of the last elemnt visited
        bool final = true;
        int parent = crnt[crnt.size()-1];
        for(auto a: nodes[parent]) {
            if (!pair_in_vec({a,parent},crnt)){
                std::vector<int> new_path(crnt);
                new_path.push_back(a);
                que.push(new_path);
                final = false;
            }
        }
        if(final) {
            max_cost = std::max(max_cost,calc(crnt,nodes).first);
        }
        que.pop();
    }
    return max_cost;
}

int traverse_part2(int start,std::map<int,std::vector<int>> &nodes){
    std::queue<std::vector<int>> que;
    std::vector<int> max_path;
    std::pair<int,int> max_cost = {0,0};
    que.push({start});
    while (que.size()) {
        std::vector<int> crnt = que.front();
        bool final = true;
        int parent = crnt[crnt.size()-1];
        for(auto a: nodes[parent]) {
            if (!pair_in_vec({a,parent},crnt)){
                std::vector<int> new_path(crnt);
                new_path.push_back(a);
                que.push(new_path);
                final = false;
            }
        }
        if(final) {
            std::pair<int,int> metric = calc(crnt,nodes);
            if (metric.second > max_cost.second){
                max_cost = metric;
            } else if (metric.second == max_cost.second && metric.first > max_cost.first ){
                max_cost = metric;
            }
        }
        que.pop();
    }
    return max_cost.first;
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

    return traverse_part1(0, nodes);
}

int part2(std::string fname) {
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

    return traverse_part2(0, nodes);
}



int main (int argc, char *argv[]) {
    std::cout << "Part 1 -> "<< part1("./test") << std::endl;
    std::cout << "Part 1 -> "<< part1("./input") << std::endl;
    std::cout << "Part 2 -> "<< part2("./test") << std::endl;
    std::cout << "Part 2 -> "<< part2("./input") << std::endl;

    return 0;
}
