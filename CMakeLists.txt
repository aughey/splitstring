cmake_policy(SET CMP0048 NEW)
project(Demo VERSION 0.0.1)
cmake_minimum_required(VERSION 3.18)

# set c++ standard to 17
set(CMAKE_CXX_STANDARD 17)

# use -Wall and -Wextra
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra")

# include all c++ files
file(GLOB SRCS src/*.cpp)
# include all header files
file(GLOB HEADERS src/*.h)
file(GLOB TEST_SOURCES "tests/*.cpp")


# Pull in google test 
include(FetchContent)
FetchContent_Declare(
  googletest
  GIT_REPOSITORY https://github.com/google/googletest.git
  GIT_TAG        main
  )

FetchContent_MakeAvailable(googletest)

# build app from all c++ files
add_library(applib ${SRCS} ${HEADERS})

# add src as an include path
include_directories(src)

enable_testing()

add_executable(unit_tests ${TEST_SOURCES})
target_link_libraries(unit_tests gtest gtest_main applib)
include(GoogleTest)
gtest_discover_tests(unit_tests)