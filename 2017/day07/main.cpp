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
#include <vector>
#include <map>
#include <set>
#include <cmath>


/* #define TEST */

#ifndef TEST
#define INPUT "./input"
/* #define INPUT "./input_test" */
#else
#define INPUT "./input_test"
#endif

std::vector<std::string> parse(std::string line){
    // we are going to asssume that we always get valid input,
    // and we shoulf this should always return a class
    std::vector<std::string> res;
    int32_t len = line.length();
    char *st = (char*) malloc(len+1);
#define splitters "(),-> "
    memset(st, 0, len+1);
    strcpy(st, line.c_str());
    char* p = strtok(st, splitters);
    while(p){
        /* printf ("%s\n",p); */
        res.push_back(std::string(p));
        p = strtok(NULL, splitters);
    }
    return res;
}

std::string part1(std::string fname) {
    std::ifstream inp(fname);
    std::string line;
    std::set<std::string> children;
    std::set<std::string> parents;

    while (std::getline(inp,line)) {
        std::vector<std::string> parsed = parse(line);
        parents.insert(parsed[0]);
        for (auto i = 2; i<parsed.size();i++) {
            children.insert(parsed[i]);
        }
    }
    for (auto c:children){
        if (parents.count(c)) {
            parents.erase(c);
        }
    }
    return *parents.begin();
}

class Tree{
    private:
        std::map<std::string,std::pair<std::pair<int32_t,int32_t>,std::vector<std::string>>> &tree;
        bool balance_tree;
    public:
        int32_t unbalanced_weight;

    Tree(std::map<std::string,std::pair<std::pair<int32_t,int32_t>,std::vector<std::string>>> &inp,bool balance_tree):tree(inp),balance_tree(balance_tree) {}

    int32_t eval_subree_weight(std::string &root_node) {
        auto this_node  = this->tree.find(root_node);
        int32_t res = this_node->second.first.first ;;

        std::vector<std::pair<int32_t,int32_t>> costs;
        for (auto i = 0; i < this_node->second.second.size();i++) {
            auto cost = this->eval_subree_weight(this_node->second.second[i]);
            costs.push_back({cost, this->tree.find(this_node->second.second[i])->second.first.first});
            res += cost;
        }

        if (this->balance_tree) {
            auto max_elem = std::max_element(costs.begin(),costs.end());
            auto min_elem = std::min_element(costs.begin(),costs.end());

            if (max_elem != costs.end() && min_elem != costs.end() && max_elem->first != min_elem->first) {
                auto count_max = 0;
                auto count_min = 0;
                std::for_each(costs.begin(),costs.end(),[&](std::pair<int32_t,int32_t> n) {
                        if (n.first == max_elem->first) count_max += 1;
                        });
                std::for_each(costs.begin(),costs.end(),[&](std::pair<int32_t,int32_t> n) {
                        if (n.first == min_elem->first) count_min += 1;
                        });
                // result would have to be adjusted here, otherwise unbalance
                // will cascade through the upper levels and get overwritten
                if (count_max > count_min) {
                    this->unbalanced_weight = min_elem->second + std::abs(max_elem->first - min_elem->first);
                    res += std::abs(max_elem->first - min_elem->first);
                } else {
                    this->unbalanced_weight = max_elem->second - std::abs(max_elem->first - min_elem->first);
                    res -= std::abs(max_elem->first - min_elem->first);
                }
                /* printf("\n%d - %d  -> %d",max_elem->first,min_elem->first,min_elem->second); */
                /* printf("\n%d - %d  -> %d\n",max_elem->first,min_elem->first,max_elem->second); */
            }
        }
        this_node->second.first.second = res;
        return  res;
    }
};

size_t part2(std::string fname) {
    std::string root = part1(fname);
    std::ifstream inp(fname);
    std::string line;

    std::map<std::string,std::pair<std::pair<int32_t,int32_t>,std::vector<std::string>>> tree;
    while (std::getline(inp,line)) {
        std::vector<std::string> parsed = parse(line);
        std::string node = parsed[0];
        int32_t weight = 0;
        sscanf(parsed[1].c_str(), "%d",&weight);
        parsed.erase(parsed.begin(),parsed.begin() +2 );
        tree.insert({node,{{weight,0},parsed}});
    }
    auto trav = Tree(tree,true);
    trav.eval_subree_weight(root);
    return  trav.unbalanced_weight;
}

int main (int argc, char *argv[]) {
    std::cout << "Part1 "<<part1(INPUT) << std::endl;
    std::cout << "Part2 "<<part2(INPUT) << std::endl;
    return 0;
}
