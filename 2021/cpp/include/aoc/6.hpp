#pragma once

#include <algorithm>
#include <array>
#include <charconv>
#include <cmath>
#include <fstream>
#include <numeric>
#include <ranges>
#include <vector>

#include <fmt/format.h>

static constexpr auto MAX_DAYS = 256;
static constexpr auto REPRODUCTION_TIME = 6;
static constexpr auto NEW_FISH_LIFETIME = 8;

class Advent {
public:
  Advent(const char *filepath)
    : fish{} // zero-initialized
  {
    auto file = std::fstream(filepath);
    auto line = std::string();
    while (std::getline(file, line, ',')) {
      auto time = line.front() - '0';
      fish[time] += 1;
    }
  };

  void part_one() {
    for (auto day : std::views::iota(0, MAX_DAYS)) {      
      auto num_new = fish[0];
      for (int i = 1; i < fish.size(); ++i) {
        fish[i - 1] = fish[i];
      }
      
      fish.back() = 0;
      fish[REPRODUCTION_TIME] += num_new;
      fish[NEW_FISH_LIFETIME] += num_new;

    }

    fmt::print("{}\n", std::accumulate(fish.begin(), fish.end(), 0ull));
  }

  void part_two() {}

private:
  std::array<size_t, NEW_FISH_LIFETIME + 1> fish;
};