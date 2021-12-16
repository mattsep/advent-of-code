#pragma once

#include <fstream>
#include <vector>
#include <fmt/format.h>

class Advent {
public:
  Advent(const char* input_file) : input{input_file}, values{} {}

  void part_one() {
    auto line = std::string();
    while (std::getline(input, line)) {
        values.push_back(std::stoi(line));
    }

    int cnt = 0;
    for (size_t i = 1; i < values.size(); ++i) {
        cnt += values[i] > values[i-1];
    }

    // Part 1 output
    fmt::print("{}\n", cnt);
  }
  
  void part_two() {
    auto sums = std::vector<int>();
    for (size_t i = 2; i < values.size(); ++i) {
        sums.push_back(values[i-2] + values[i-1] + values[i]);
    }

    int cnt = 0;
    for (size_t i = 1; i < sums.size(); ++i) {
        cnt += sums[i] > sums[i-1];
    }

    // Part 2 output
    fmt::print("{}\n", cnt);
  }

private:
    std::ifstream input;
    std::vector<int> values;    
};