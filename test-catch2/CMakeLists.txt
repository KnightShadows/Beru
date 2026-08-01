cmake_minimum_required(VERSION 3.20)
project(test-catch2)

# Beru automatically injects the toolchain file which provides find_package paths
find_package(Catch2 REQUIRED)

add_executable(test_catch2 src/main.cpp)
target_link_libraries(test_catch2 PRIVATE Catch2::Catch2WithMain)
