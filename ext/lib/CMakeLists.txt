cmake_minimum_required(VERSION 2.8.11)
project(librsqml)

set(CMAKE_BUILD_TYPE RELEASE)
set(CMAKE_INCLUDE_CURRENT_DIR ON)
set(CMAKE_AUTOMOC ON)

include(CheckCXXCompilerFlag)
CHECK_CXX_COMPILER_FLAG("-std=c++11" COMPILER_SUPPORTS_CXX11)
CHECK_CXX_COMPILER_FLAG("-std=c++0x" COMPILER_SUPPORTS_CXX0X)
if(COMPILER_SUPPORTS_CXX11)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")
elseif(COMPILER_SUPPORTS_CXX0X)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++0x")
else()
    message(STATUS "The compiler ${CMAKE_CXX_COMPILER} has no C++11 support. Please use a different C++ compiler.")
endif()

find_package(Qt5Core)
find_package(Qt5Quick)

set(librsqml_SRCS engine.cpp dynamicobject.cpp api.cpp)

add_library(librsqml STATIC ${librsqml_SRCS})
target_link_libraries(librsqml Qt5::Core Qt5::Quick)

install(TARGETS librsqml ARCHIVE DESTINATION lib)
