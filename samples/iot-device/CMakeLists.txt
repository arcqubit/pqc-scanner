cmake_minimum_required(VERSION 3.10)
project(iot_device_vulnerable)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find OpenSSL
find_package(OpenSSL REQUIRED)

# Include directories
include_directories(
    ${CMAKE_SOURCE_DIR}/src
    ${OPENSSL_INCLUDE_DIR}
)

# Source files
set(SOURCES
    src/crypto/device_auth.cpp
    src/crypto/firmware_verify.cpp
    src/crypto/message_crypto.cpp
    src/network/mqtt_client.cpp
    src/storage/key_manager.cpp
)

# Executable
add_executable(iot_device ${SOURCES})

# Link libraries
target_link_libraries(iot_device
    ${OPENSSL_LIBRARIES}
)

# Tests
enable_testing()
add_subdirectory(tests)
