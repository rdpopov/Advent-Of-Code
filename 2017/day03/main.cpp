/* #include <stdio.h> */
#include <cstdint>
#include <iostream>
#include <stdint.h>
#include <assert.h>
#include <sstream>
#include <fstream>
#include <string>
#include <vector>
#include <map>
#include <cmath>



#ifndef TEST
#define INPUT 265149
#endif

typedef std::vector<std::vector<int32_t>> Matr;
const std::vector<std::pair<int32_t,int32_t>> dirs = {{1,0},{0,1},{-1,0},{0,-1}};;

class SquareIter {
	public:
		int32_t x;
		int32_t y;
		int32_t ring;
		size_t dir;

		SquareIter(): x(0), y(0), ring(0), dir(0) {
		}

		std::pair<int32_t,int32_t> next(){
			std::pair<int32_t,int32_t> crnt = {x,y};
			if (std::abs(dirs[dir].first * x + dirs[dir].second * y) > ring) {
				dir += 1;
				if (dir % 4 == 0) {
					ring += 1;
					dir = 0;
				}
			}
			x+=dirs[dir].first;
			y+=dirs[dir].second;
			/* std::cout << this->str(); */
			return {x,y};
		}

		std::string str() {
			std::stringstream out;
			out << "SquareIter {";
			out << " x: " << this->x;
			out << " y: " << this->y;
			out << " dir: " << this->dir;
			out << " ring: " << this->ring;
			out << " } ";
			return out.str();
		}
};

int32_t sum_moore_neigh(
		std::map<std::pair<int32_t,int32_t>,int32_t> matr,
		std::pair<int32_t,int32_t> cell) {
	int32_t res = 0;
	for(int32_t x=-1;x<2;x++) {
		for(int32_t y=-1;y<2;y++) {
			if (x || y) {
				auto cell_value = matr.find({cell.first + x,cell.second +y});
				if (cell_value != matr.end()) {
					res += cell_value->second;
				}
			}
		}
	}
	return res;
}


int32_t part1(int32_t pos) {
    auto ring  = (int32_t)(std::sqrt(pos-1) + 1) >> 1;
    int32_t ring_sq = (2*ring-1) *(2*ring-1);
    int32_t offset_of_ring = pos;
    std::vector<int32_t> cross = {
        std::abs( pos - ( ring_sq + 1 * ring) ),
        std::abs( pos - ( ring_sq + 3 * ring) ),
        std::abs( pos - ( ring_sq + 5 * ring) ),
        std::abs( pos - ( ring_sq + 7 * ring) ),
    };
    for(auto& s: cross) {
        /* std::cout << s << " "; */
        offset_of_ring = std::min(offset_of_ring,s);
    }

    /* std::cout << ring  << std::endl; */
    return ring + offset_of_ring;
}

std::pair<int32_t,int32_t> get_coords_of_pos(int32_t pos) {
    auto ring  = (int32_t)(std::sqrt(pos-1) + 1) >> 1;
    int32_t ring_sq = (2*ring-1) *(2*ring-1);
    return std::pair<int32_t,int32_t>(ring,ring);
}

int32_t part2(int32_t pos) {
	SquareIter cell;
	std::map<std::pair<int32_t,int32_t>,int32_t> matr;
	matr.insert({{0,0},1});
	int32_t last_sum = 1;

	while (last_sum <= pos){
		auto crnt = cell.next();
		last_sum = sum_moore_neigh(matr,crnt);
        /* printf("%d %d -> %d\n",crnt.first, crnt.second,last_sum); */
        matr.insert({crnt,last_sum});
	}
    return last_sum;
}

int main (int argc, char *argv[]) {
#ifdef TEST
    auto test_part1_lambda = [](int32_t pos,int32_t expected){
        int32_t fn_res = part1(pos);
        std::stringstream msg;
        msg << "part1( "<<pos << " ) != "<< fn_res;
        if (fn_res != expected) throw std::logic_error(msg.str());
        return 1;
    };
    int32_t passed_tests = 0;
    std::vector<std::pair<int32_t,int32_t>> tests = {
        { 1    ,0  },
        { 12   ,3  },
        { 23   ,2  },
        { 1024 ,31 }
        }; 
    for( auto &t: tests ) {
        try {
            passed_tests += test_part1_lambda(t.first,t.second);
        } catch(const std::exception& e) {
            std::cout << e.what() << std::endl;
        }
    }

    auto test_part2_lambda = [](int32_t pos,int32_t expected){
        int32_t fn_res = part2(pos);
        std::stringstream msg;
        msg << "part2( "<<pos << " ) != "<< fn_res;
        if (fn_res != expected) throw std::logic_error(msg.str());
        return 1;
    };
    std::vector<std::pair<int32_t,int32_t>> tests2 = {
        { 0  ,1  },
        { 10 ,11 },
        { 24 ,25 },
        { 3  ,4  },
        };
    for( auto &t: tests2 ) {
        try {
            passed_tests += test_part2_lambda(t.first,t.second);
        } catch(const std::exception& e) {
            std::cout << e.what() << std::endl;
        }
    }
    std::cout << "passed tests: "<< passed_tests << "/" << tests.size() + tests2.size();

#else
    std::cout << "Part1 "<<part1(INPUT) << std::endl;
    std::cout << "Part2 "<<part2(INPUT) << std::endl;
#endif

    return 0;
}
