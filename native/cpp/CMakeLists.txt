cmake_minimum_required(VERSION 3.16)
project(heartlight_cpp VERSION 0.3.1 LANGUAGES CXX)

include(GNUInstallDirs)

option(HEARTLIGHT_BUILD_TESTS "Build HEARTLIGHT C++ tests" ON)
option(HEARTLIGHT_BUILD_EXAMPLES "Build HEARTLIGHT C++ examples" ON)

set(HEARTLIGHT_SOURCES
    src/heartlight.cpp
    src/heartlight_c.cpp
    src/synapse.cpp
    src/synapse_c.cpp
)

add_library(heartlight STATIC ${HEARTLIGHT_SOURCES})
target_include_directories(heartlight PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    $<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}>
)
target_compile_features(heartlight PUBLIC cxx_std_17)
set_target_properties(heartlight PROPERTIES POSITION_INDEPENDENT_CODE ON)

add_library(heartlight_shared SHARED ${HEARTLIGHT_SOURCES})
target_include_directories(heartlight_shared PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    $<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}>
)
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

    add_executable(heartlight_synapse_test tests/test_synapse.cpp)
    target_link_libraries(heartlight_synapse_test PRIVATE heartlight)
    add_test(NAME heartlight_synapse COMMAND heartlight_synapse_test)
endif()

install(TARGETS heartlight heartlight_shared
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)
install(DIRECTORY include/ DESTINATION ${CMAKE_INSTALL_INCLUDEDIR})
