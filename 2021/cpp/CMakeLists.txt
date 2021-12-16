cmake_minimum_required(VERSION 3.20)

project(AdventOfCode CXX)

find_package(fmt REQUIRED)

file(DOWNLOAD https://adventofcode.com/2021/day/1/input ${CMAKE_BINARY_DIR}/input.txt)

add_executable(main main.cpp)
target_compile_features(main PRIVATE cxx_std_20)
target_include_directories(main PRIVATE include)
target_link_libraries(main PRIVATE fmt::fmt)

