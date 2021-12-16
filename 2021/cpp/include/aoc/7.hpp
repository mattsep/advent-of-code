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

auto median(auto const& xs) {
  auto len = xs.size();
  auto mid = len / 2;

  if (len % 2 == 0) {
    return (xs[mid - 1] + xs[mid]) / 2;
  }

  return xs[mid];
}

auto mean(auto const& xs) {
  return std::accumulate(xs.begin(), xs.end(), 0.0) / xs.size();
}

class Advent {
public:
  Advent(const char *filepath)
    : xs{} // zero-initialized
  {
    auto file = std::fstream(filepath);
    auto line = std::string();
    while (std::getline(file, line, ',')) {
      xs.push_back(std::stoull(line));
    }

    std::ranges::sort(xs);
  };

  void part_one() {
    auto m = median(xs);
    auto fuel = long{};
    for (auto x : xs) {
      fuel += std::abs(x - m);
    }
    fmt::print("{}\n", fuel);
  }

  void part_two() {
    auto m = static_cast<long>(std::round(mean(xs)));
    
    long fuel, n, l, c, r; // left, right, center;
    do {
      l = c = r = 0;
      for (auto x : xs) {
        n = std::abs(x - m + 1);
        l += n * (n + 1) / 2;
        
        n = std::abs(x - m);
        c += n * (n + 1) / 2;
        
        n = std::abs(x - m - 1);
        r += n * (n + 1) / 2;
      }

      m += (r < c) - (l < c);
    } while (not (c < r and c < l));
    
    fmt::print("{}\n", c);
  }

private:
  std::vector<long> xs;
};