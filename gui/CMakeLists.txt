cmake_minimum_required(VERSION 3.16)
project(arch_manager_gui LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt6 REQUIRED COMPONENTS Widgets)

qt_standard_project_setup()

add_executable(arch-manager-gui
    src/main.cpp
    src/main_window.cpp
    src/main_window.h
)

target_link_libraries(arch-manager-gui PRIVATE Qt6::Widgets)
