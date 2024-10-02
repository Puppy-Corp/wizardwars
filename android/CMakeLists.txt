# Sets the minimum CMake version required for this project.
cmake_minimum_required(VERSION 3.22.1)

# Declares the project name.
project("wizardwars")

# Define the path to native_app_glue
set(NATIVE_APP_GLUE ${ANDROID_NDK}/sources/android/native_app_glue)

# Add native_app_glue as a static library
add_library(native_app_glue STATIC ${NATIVE_APP_GLUE}/android_native_app_glue.c)

# Include directories for native_app_glue
include_directories(${NATIVE_APP_GLUE})

# Find and link OpenXR loader
find_package(OpenXR REQUIRED)

# Gather all source files recursively from the cpp directory
file(GLOB_RECURSE SRC_FILES
        cpp/*.c
        cpp/*.cpp
)

# Create a shared library with the project name and the gathered source files
add_library(${PROJECT_NAME} SHARED ${SRC_FILES})

# Include directories for your project
target_include_directories(${PROJECT_NAME} PUBLIC
        ${NATIVE_APP_GLUE}
        Src  # Assuming 'Src' is a directory containing additional headers
)

# Link to the native_app_glue, android, OpenXR, and log libraries
find_library(log-lib log)

target_link_libraries(${PROJECT_NAME}
        native_app_glue
        android
        OpenXR::openxr_loader
        ${log-lib})

# Optional: If you still face issues, you can explicitly set the linker flag
set_target_properties(${PROJECT_NAME} PROPERTIES LINK_FLAGS "-u ANativeActivity_onCreate")
