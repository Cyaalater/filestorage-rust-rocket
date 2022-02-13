#include <stdio.h>

#if defined(_WIN32) || defined(_WIN64)
        const char* os = "Windows";
#else
#ifdef __linux
        const char* os = "Linux";
#else
        const char* os = "Unknown";
#endif
#endif

int main(void)
{
   printf("os = %s\n", os);
   return 0;
}
