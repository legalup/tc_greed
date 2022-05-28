cmake_minimum_required(VERSION 3.17)
set (CMAKE_CXX_STANDARD 17)
set(CMAKE_BUILD_TYPE Debug)


project(lc)
add_executable(lc conn_comps_undirected_graph.cpp)
add_definitions(-g -Wreturn-type -Wcomment -Wunused-variable -Wunused-parameter -Wno-sign-compare)
