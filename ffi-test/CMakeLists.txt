cmake_minimum_required(VERSION 3.26)
project(ffi_test C)

set(CMAKE_C_STANDARD 11)

add_executable(ffi_test main.c bindings.h)
target_link_libraries(ffi_test "${CMAKE_SOURCE_DIR}/libprocessing.a" m freetype fontconfig)