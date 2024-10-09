cmake_minimum_required(VERSION 3.29)
project(etsi014_client_c_test C)

set(CMAKE_C_STANDARD 17)

add_executable(etsi014-client-c-test src/main.c)
target_link_libraries(etsi014-client-c-test etsi014_client)
