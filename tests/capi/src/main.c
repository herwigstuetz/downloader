#include "stdio.h"
#include "bindings.h"

int main() {
    char * file = dl_download("http://orf.at", "/tmp");
    printf("file: %s", file);
    dl_free(file);
    return 0;
}
