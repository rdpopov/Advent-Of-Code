/* #include <stdio.h> */
#include <cctype>
#include <cstddef>
#include <iterator>
#include <ostream>
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

class Vec3 {
    public: // dont care
        std::valarray<int> c;
        Vec3() {
            this->c = {0,0,0};
        };

        Vec3(std::string toPrs) : Vec3() {
            char Vlet;
            sscanf(toPrs.c_str(), "%c=<%d,%d,%d>", &Vlet, &(this->c[0]), &(this->c[1]), &(this->c[2]));
        };
        friend std::ostream& operator<<(std::ostream& os, Vec3& p);
};

std::ostream& operator<<(std::ostream& os, Vec3& p){
    os <<"("<< p.c[0] << " " << p.c[1] << " " << p.c[2] << ")";
    return os;
}

class Particle {
    public:
        Vec3 initial;
        Vec3 place;
        Vec3 velocity;
        Vec3 accel;
        int dist;

        Particle(): place(),initial(), velocity(), accel(),dist(0) {};
        Particle(std::string _line){
            std::stringstream line(_line);
            std::string toPrs;
            std::getline(line,toPrs,' ');
            this->place = Vec3(toPrs);
            this->initial.c = this->place.c;
            std::getline(line,toPrs,' ');
            this->velocity = Vec3(toPrs);
            std::getline(line,toPrs,' ');
            this->accel = Vec3(toPrs);
            this->dist = (this->initial.c).apply(abs).sum();
        };

        // returns manhatan distance from zero
        int nxt() {
            velocity.c = velocity.c + accel.c;
            place.c    = place.c    + velocity.c;
            return this->place.c.apply(abs).sum();
        }

        int delta() {
            return (initial.c - place.c).apply(abs).sum();
        }

        friend std::ostream& operator<<(std::ostream& os, Particle& p);
};

std::ostream& operator<<(std::ostream& os, Particle& p){
    os << "place:" << p.place << " vel:" << p.velocity << " acc:" << p.accel;
    return os;
}

// this worked by chance
int part1(std::string fname){
    std::ifstream in(fname);
    std::string line;
    std::vector<Particle> particles;

    int iterations = 1000;
    while(std::getline(in,line)){
        particles.push_back(Particle(line));
    }

    int mindist = particles[0].dist;
    int minidx = 0;
    while(iterations--) {
        mindist = particles[0].nxt();;
        minidx = 0;

        for(int n=1; n < particles.size();n++) {
            int dist = particles[n].nxt();
            if (dist <= mindist) {
                mindist = dist;
                minidx = n;
            }
        }
    }

    return minidx;
}

int part2(std::string fname){
    return 0;
}

int main (int argc, char *argv[]) {
    /* std::cout << "Part 1 -> \n"<< part1("./test") << std::endl; */
    std::cout << "Part 1 -> "<< part1("./input") << std::endl;

    return 0;
}
