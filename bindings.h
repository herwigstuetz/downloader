#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

char *dl_download(const char *url, const char *tmp);

void dl_free(char *s);
