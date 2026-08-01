cmake_minimum_required(VERSION 3.20)
project(test-opencv)

find_package(OpenCV REQUIRED)

add_executable(test_opencv src/main.cpp)
target_link_libraries(test_opencv PRIVATE opencv_core)
