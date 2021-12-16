#pragma once

#include <fstream>
#include <stdexcept>
#include <vector>
#include <string_view>
#include <charconv>

#include <fmt/format.h>

struct Command {
  std::string direction;
  int distance;
};

struct Point {
  int x = 0;
  int y = 0;
  int aim = 0;
};

class Advent {
public:
  Advent(const char* input_file) : input{input_file}, commands{}, point{} {
    auto line = std::string();
    while (std::getline(input, line)) {
      commands.push_back(parse(line));
    }
  }

  void part_one() {
    for (auto command: commands) {
      execute_part_one(command);
    }

    // Part 1 output
    fmt::print("{}\n", point.x * point.y);
  }
  
  void part_two() {
    point = {};
    for (auto command : commands) {
      execute_part_two(command);
    }

    fmt::print("{}\n", point.x * point.y);
  }

private:
    std::ifstream input;
    std::vector<Command> commands;
    Point point;

    auto parse(std::string_view line) -> Command {
      Command command = {};
      auto i = line.find_first_of(' ');
      std::from_chars(line.begin() + i + 1, line.end(), command.distance);
      command.direction = line.substr(0, i);
      return command;
    }

    auto execute_part_one(Command command) -> void {
      if (command.direction == "forward") point.x += command.distance;
      else if (command.direction == "backward") point.x -= command.distance;
      else if (command.direction == "up") point.y -= command.distance;
      else if (command.direction == "down") point.y += command.distance;
      else throw std::runtime_error("invalid command!");
    }
    
    auto execute_part_two(Command command) -> void {
      if (command.direction == "forward") {
        point.x += command.distance;
        point.y += command.distance * point.aim;
      } else if (command.direction == "up") {
        point.aim -= command.distance;
      } else if (command.direction == "down") {
        point.aim += command.distance;
      } else {
        fmt::print("ERROR: invalid command '{} {}'", command.direction, command.distance);
      }
    }
};