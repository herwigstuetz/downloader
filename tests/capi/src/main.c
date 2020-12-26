#include "stdio.h"
#include "downloader.h"

int main() {
    char * file = dl_download("http://orf.at", "/tmp");
    printf("file: %s", file);
    dl_free(file);
    return 0;
}
