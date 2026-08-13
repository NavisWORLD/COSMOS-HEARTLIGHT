cmake_minimum_required(VERSION 3.16)
project(heartlight_cpp VERSION 0.2.0 LANGUAGES CXX)

option(HEARTLIGHT_BUILD_TESTS "Build HEARTLIGHT C++ tests" ON)
option(HEARTLIGHT_BUILD_EXAMPLES "Build HEARTLIGHT C++ examples" ON)

add_library(heartlight STATIC
    src/heartlight.cpp
    src/heartlight_c.cpp
)
target_include_directories(heartlight PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)
target_compile_features(heartlight PUBLIC cxx_std_17)
set_target_properties(heartlight PROPERTIES POSITION_INDEPENDENT_CODE ON)

add_library(heartlight_shared SHARED
    src/heartlight.cpp
    src/heartlight_c.cpp
)
target_include_directories(heartlight_shared PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)
target_compile_features(heartlight_shared PUBLIC cxx_std_17)
set_target_properties(heartlight_shared PROPERTIES OUTPUT_NAME heartlight)

if(HEARTLIGHT_BUILD_EXAMPLES)
    add_executable(heartlight_demo examples/heartlight_demo.cpp)
    target_link_libraries(heartlight_demo PRIVATE heartlight)
endif()

if(HEARTLIGHT_BUILD_TESTS)
    enable_testing()
    add_executable(heartlight_test tests/test_heartlight.cpp)
    target_link_libraries(heartlight_test PRIVATE heartlight)
    add_test(NAME heartlight_core COMMAND heartlight_test)
endif()
