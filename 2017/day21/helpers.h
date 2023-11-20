/* #include <stdio.h> */
#include <cctype>
#include <cstddef>
#include <ios>
#include <iterator>
#include <math.h>
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
using lattice = std::vector<std::string>;

std::string removeslsh(std::string inp){
        std::string res("");
        for(auto a:inp){
            if (a != '/'){
                res.push_back(a);
            }
        }
        return res;
}

inline std::string rotate(std::string inp){
    if(inp.size() == 4){
        std::string res(inp);
        res[0]= inp[2];
        res[1]= inp[0];
        res[2]= inp[3];
        res[3]= inp[1];
        return  res;
    }
    if(inp.size() == 9){
        std::string res(inp);
        res[0]= inp[6];
        res[2]= inp[0];
        res[8]= inp[2];
        res[6]= inp[8];
        res[1]= inp[3];
        res[3]= inp[7];
        res[5]= inp[1];
        res[7]= inp[5];
        return res;
    }
    return "";
}

inline std::string flipv(std::string inp){
    if(inp.size() == 4){
        std::string res(inp);
        res[0]= inp[1];
        res[1]= inp[0];
        res[2]= inp[3];
        res[3]= inp[2];
        return  res;
    }
    if(inp.size() == 9){
        std::string res(inp);
        res[0]= inp[2];
        res[2]= inp[0];
        res[3]= inp[5];
        res[5]= inp[3];
        res[6]= inp[8];
        res[8]= inp[6];
        return res;
    }
    return "";
}

inline std::string fliph(std::string inp){
    if(inp.size() == 4){
        std::string res(inp);
        res[0]= inp[2];
        res[2]= inp[0];
        res[1]= inp[3];
        res[3]= inp[1];
        return  res;
    }
    if(inp.size() == 9){
        std::string res(inp);
        res[0]= inp[6];
        res[6]= inp[0];
        res[1]= inp[7];
        res[7]= inp[1];
        res[2]= inp[8];
        res[8]= inp[2];
        return res;
    }
    return "";
}


inline std::vector<std::string> rotations(std::string init){
    std::vector<std::string> ret = {init};
    ret.push_back(rotate(ret[ret.size()-1]));
    ret.push_back(rotate(ret[ret.size()-1]));
    ret.push_back(rotate(ret[ret.size()-1]));
    for(int i=0; i<4; i++) {
        ret.push_back(fliph(ret[i]));
        ret.push_back(flipv(ret[i]));
    }
    return ret;
}

inline lattice to_latt(std::string init){
    lattice ret;
    int size = sqrt(init.size());
    for(int y = 0;y < size ;y ++){
        ret.push_back("");
        for(int x = 0;x < size ;x ++){
            ret[y].push_back(init[y*size+x]);
        }
    }
    return ret;
}

