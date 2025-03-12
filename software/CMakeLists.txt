# The following lines of boilerplate have to be in your project's
# CMakeLists in this exact order for cmake to work correctly
cmake_minimum_required(VERSION 3.31)
set(CMAKE_CXX_STANDARD 20)
set(EXCLUDE_COMPONENTS "mbedtls" "wpa_supplicant" "wifi" "bt" "esp_wifi" "esp_event" "esp_netif" "lwip" "mdns" "esp_mesh" "esp_smartconfig" "esp_now" "esp_eth" "esp_pm")
set(COMPONENTS
    main
    driver
    hal
    freertos
    log
    newlib
    onewire
    nvs_flash
    console
    esp_phy
)
include($ENV{IDF_PATH}/tools/cmake/project.cmake)
project(eisenbahn)
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -Wl,--gc-sections")
