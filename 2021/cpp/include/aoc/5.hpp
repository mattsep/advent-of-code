#pragma once

#include <cmath>
#include <fstream>
#include <stdexcept>
#include <vector>
#include <charconv>

#include <fmt/format.h>

struct Point {
  int x, y;
};

struct Line {
  Point p, q;

  auto is_vertical() -> bool {
    return p.x == q.x;
  }

  auto is_horizontal() -> bool {
    return p.y == q.y;
  }

  auto is_diagonal() -> bool {
    return std::abs(q.x - p.x) == std::abs(q.y - p.y);
  }
};

template<> struct fmt::formatter<Line> : fmt::formatter<int> {
  template <class FormatContext>
  auto format(Line const& line, FormatContext& ctx) -> decltype(ctx.out()) {
    return format_to(ctx.out(), "({:3d}, {:3d}) -> ({:3d}, {:3d})",
                     line.p.x, line.p.y, line.q.x, line.q.y);
  }
};


class Advent {
public:
  Advent(const char* filepath)
    : lines{}
    , grid(GRID_WIDTH * GRID_WIDTH, 0)
  {
    auto file = std::fstream(filepath);
    auto text = std::string();

    while (std::getline(file, text)) {
      auto line = parse(text);
      if (line.is_horizontal() or line.is_vertical() or line.is_diagonal()) {
        lines.push_back(line);
      }
    }
  };
  
  void part_one() {
    for (auto line : lines) {
      auto beg = line.p.x + GRID_WIDTH * line.p.y;
      auto end = line.q.x + GRID_WIDTH * line.q.y;
      if (beg > end) {
        std::swap(beg, end);
        std::swap(line.p, line.q);
      }

      auto inc = 1;
      if (line.is_vertical()) inc = GRID_WIDTH;
      if (line.is_diagonal()) {
        if (line.q.x < line.p.x) inc = GRID_WIDTH - 1;
        if (line.q.x > line.p.x) inc = GRID_WIDTH + 1;
      }
      
      while (beg <= end) {
        grid[beg]++;
        beg += inc;
      }
    }

    int cnt = 0;
    int i, j;
    i = j = 0;
    for (auto num : grid) {
      cnt += (num >= 2);
      
      // fmt::print("{}", num);
      // if (++i == GRID_WIDTH) { i = 0; j++; fmt::print("\n"); }
    }

    fmt::print("{}\n", cnt);
  }

  void part_two() {}

private:
  static constexpr auto GRID_WIDTH = 1000;
  std::vector<Line> lines;
  std::vector<int> grid;

  auto parse(std::string_view line) -> Line {
    Line result;
    size_t i;
    
    i = line.find_first_of(',');
    auto px = line.substr(0, i);
    line = line.substr(i + 1);
    
    i = line.find_first_of(' ');
    auto py = line.substr(0, i);
    line = line.substr(i + 4);

    i = line.find_first_of(',');
    auto qx = line.substr(0, i);
    line = line.substr(i + 1);
    
    i = line.find_first_of('\n');
    auto qy = line.substr(0, i);

    std::from_chars(px.begin(), px.end(), result.p.x);
    std::from_chars(py.begin(), py.end(), result.p.y);
    std::from_chars(qx.begin(), qx.end(), result.q.x);
    std::from_chars(qy.begin(), qy.end(), result.q.y);

    return result;
  }
};