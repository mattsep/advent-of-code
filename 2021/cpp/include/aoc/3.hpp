#pragma once

#include <fstream>
#include <stdexcept>
#include <vector>
#include <string_view>
#include <algorithm>

#include <fmt/format.h>

class Advent {
public:
  Advent(const char* input_file)
    : raw{}
    , ones{}
  {
    std::ifstream input{input_file};
    auto line = std::string();
    while (std::getline(input, line)) {
      if (line.size() > ones.size()) ones.resize(line.size());

      raw.push_back(line);
      for (size_t i = 0; i < line.size(); ++i) {
        ones[i] += line[i] - '0';
      }
    }
  }

  void part_one() {
    unsigned half = raw.size() / 2;
    unsigned gamma = 0;
    unsigned epsilon = 0;
    for (auto cnt : ones) {
      gamma = (gamma << 1) | (cnt >= half);
      epsilon = (epsilon << 1) | (cnt < half);
    }
    
    fmt::print("{}\n", gamma * epsilon);
  }
  
  void part_two() {
    fmt::print("{}\n", do_filter(true) * do_filter(false));
  }

private:
    std::vector<std::string> raw;
    std::vector<unsigned> ones;

    // returns:
    //  '0' if 0 is more common
    //  '1' if 1 is more common
    //  '-' if they're equally likely
    char common_bit(std::vector<std::string> const& values, unsigned pos) {
      int cnt = 0;
      for (auto line : values) {
        cnt += line[pos] == '1' ? 1 : -1;
      }
      if (cnt < 0) return '0';
      if (cnt > 0) return '1';
      return '-';
    }

    int do_filter(bool oxygen) {
      auto values = raw;
      for (size_t bit = 0; bit < ones.size(); ++bit) {
        char c = common_bit(values, bit);
        char keep;
        if (oxygen) {
          if (c == '-') keep = '1';
          else keep = c;
        } else {
          if (c == '-') keep = '0';
          else  keep = (c == '0') ? '1' : '0';
        }

        auto pred = [&](std::string const& s) {
          return s[bit] != keep;
        };
        auto it = std::remove_if(values.begin(), values.end(), pred);
        values.erase(it, values.end());

        if (values.size() == 1) return std::stoi(values.front(), 0, 2);
      }

      return 0;
    }
};