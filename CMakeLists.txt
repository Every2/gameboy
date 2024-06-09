cmake_minimum_required(VERSION 3.28)
project(gameboy)

set(CMAKE_CXX_STANDARD 20)

set(SOURCES
        src/main.cpp
        include/main.hpp
)
add_executable(gameboy ${SOURCES})
