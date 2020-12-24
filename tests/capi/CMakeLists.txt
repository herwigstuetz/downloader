project( downloader-c )
cmake_minimum_required( VERSION 2.6 )
add_executable( ${PROJECT_NAME} src/main.c )

target_include_directories ( ${PROJECT_NAME}
  PRIVATE ../..)
target_link_directories ( ${PROJECT_NAME}
  PRIVATE
  src)
#  ../../target/debug
#  /home/herwig/downloader/target/debug)
target_link_libraries ( ${PROJECT_NAME}
  PRIVATE downloader ssl crypto dl pthread m)

install(TARGETS ${PROJECT_NAME} )
