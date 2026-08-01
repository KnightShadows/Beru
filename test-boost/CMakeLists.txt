cmake_minimum_required(VERSION 3.20)
project(test-boost)

# Boost headers might not use standard CMake packages if installed manually, but let's try find_package
find_package(Boost REQUIRED COMPONENTS system filesystem)

add_executable(test_boost src/main.cpp)
target_include_directories(test_boost PRIVATE ${Boost_INCLUDE_DIRS})
target_link_libraries(test_boost PRIVATE Boost::system Boost::filesystem)
