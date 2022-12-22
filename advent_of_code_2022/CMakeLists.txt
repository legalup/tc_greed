project(aoc)
cmake_minimum_required(VERSION 3.1)
set (CMAKE_CXX_STANDARD 17)
add_executable(aoc aoc.cpp )
add_definitions(-g -Wreturn-type -Wcomment -Wunused-variable
-Wunused-parameter -Wno-sign-compare)
