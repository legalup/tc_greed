project(codeforces)
cmake_minimum_required(VERSION 3.1)
set (CMAKE_CXX_STANDARD 11)
add_executable(cf codeforces.cpp )
add_definitions(-g -Wreturn-type -Wcomment -Wunused-variable
-Wunused-parameter -Wno-sign-compare)
