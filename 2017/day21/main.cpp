#include "helpers.h"
#include <iostream>
#include <string>
#include <sys/types.h>

// A macro to treat string as a matrix
// it saves on translations between strings and a vector of strings
// so no additional allocations
#define MATR(STR,X,Y) STR[Y*(int(sqrt(STR.size()))) + X]
#define SIDE(STR) (int(sqrt(STR.size())))


void print_sq(std::string input){
    for(int kY = 0;kY < SIDE(input);kY++) {
        for(int kX = 0;kX < SIDE(input);kX++) {
            std::cout<<MATR(input, kX, kY);
        }
        std::cout<<"\n";
    }
    std::cout<<"\n";
}

std::string upscale(std::string input,std::map<std::string,std::string> & translation){
    if (SIDE(input) % 2 == 0) {
        int new_size        = (SIDE(input) / 2) * 3;
        new_size           *= new_size;
        std::string out     = std::string(new_size,'.');
        int qdrants         = SIDE(input) / 2;
        int source_qd_size  = 2;
        int dest_qd_size    = 3;

        for(int qdY = 0;qdY < qdrants;qdY++) {
            for(int qdX = 0;qdX < qdrants;qdX++){
                // collect string for key
                std::string key = "";
                for(int kY = 0;kY < source_qd_size;kY++) {
                for(int kX = 0;kX < source_qd_size;kX++) {
                        int x = qdX * source_qd_size + kX ;
                        int y = qdY * source_qd_size + kY ;
                        key.push_back(MATR(input,x,y));
                    }
                }
                // get the info for the other side
                std::string repl = translation[key];
                // fill the other side
                for(int kY = 0;kY < dest_qd_size;kY++) {
                for(int kX = 0;kX < dest_qd_size;kX++) {
                        int x = qdX * dest_qd_size + kX ;
                        int y = qdY * dest_qd_size + kY ;
                        MATR(out,x,y) = MATR(repl,kX,kY);
                    }
                }
            }
        }
        return out;
    } else {
        int new_size        = (SIDE(input) / 3) * 4;
        new_size           *= new_size;
        std::string out     = std::string(new_size,'.');
        int qdrants         = SIDE(input) / 3;
        int source_qd_size  = 3;
        int dest_qd_size    = 4;

        for(int qdY = 0;qdY < qdrants;qdY++) {
            for(int qdX = 0;qdX < qdrants;qdX++){
                // collect string for key
                std::string key = "";
                for(int kY = 0;kY < source_qd_size;kY++) {
                for(int kX = 0;kX < source_qd_size;kX++) {
                        int x = qdX * source_qd_size + kX ;
                        int y = qdY * source_qd_size + kY ;
                        key.push_back(MATR(input,x,y));
                    }
                }
                // get the info for the other side
                std::string repl = translation[key];
                // fill the other side
                for(int kY = 0;kY < dest_qd_size;kY++) {
                for(int kX = 0;kX < dest_qd_size;kX++) {
                        int x = qdX * dest_qd_size + kX ;
                        int y = qdY * dest_qd_size + kY ;
                        MATR(out,x,y) = MATR(repl,kX,kY);
                    }
                }
            }
        }
        return out;
    }
}

int part1(std::string fname,std::string start,int iterations){
    std::ifstream in(fname);
    std::string line;
    std::map<std::string,std::string> translation;

    while(std::getline(in,line)){
        std::stringstream ln (line);
        std::string fr;
        std::string to;
        std::getline(ln,fr,' ');
        fr = removeslsh(fr);
        std::getline(ln,to,' ');
        std::getline(ln,to,' '); // we need the third token
        to = removeslsh(to);
        for(auto a: rotations(fr)){ // put all the rotations in the hash map
                                    // that way we dont have to individially
                                    // rotate and check for each square.
                                    // some shapes are symetrical in some way,
                                    // so that will save on space
                                    // and performance, we keep dynamic
                                    // allocations to a minimum-ish
            translation[a] = to;
        }
    }

    for (int i =0;i< iterations ;i++){
        start =  upscale(start, translation);
    }
    int res =0;
    for (auto a: start){
        if (a == '#') res++;
    }
    return res;

}

int main (int argc, char *argv[]) {
    std::cout << "Part 1 ->\n"<< part1("./test",".#...####",2) << std::endl;
    std::cout << "Part 1 ->\n"<< part1("./input",".#...####",5) << std::endl;
    std::cout << "Part 2 ->\n"<< part1("./input",".#...####",18) << std::endl;

    return 0;
}
