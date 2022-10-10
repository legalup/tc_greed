cmake_minimum_required(VERSION 3.5)
set (CMAKE_CXX_STANDARD 17)


project(testy)
find_package(Threads REQUIRED)

set(THREADS_PREFER_PTHREAD_FLAG ON)

add_executable(testy test_package.cpp)
add_definitions(-g -Wreturn-type -Wcomment -Wunused-variable -Wunused-parameter -Wno-sign-compare)
target_link_libraries(testy pthread)
