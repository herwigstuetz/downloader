# See https://stackoverflow.com/a/49857853 for details
add_library(downloader STATIC IMPORTED GLOBAL)

set_target_properties(downloader PROPERTIES
    IMPORTED_LOCATION "${CMAKE_CURRENT_LIST_DIR}/lib/libdownloader.a"
    IMPORTED_LINK_INTERFACE_LIBRARIES "ssl;crypto;dl;pthread;m"
    INTERFACE_INCLUDE_DIRECTORIES "${CMAKE_CURRENT_LIST_DIR}/include")
