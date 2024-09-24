#define GCC_COMPILER (defined(__GNUC__))
#define CLANG_COMPILER (defined(__clang__))
#define MSVC_COMPILER (defined(_MSC_VER))

//** CLANG ONLY**/
#define __aarch64__ (defined(__aarch64__))
#define __arm__ (defined(__arm__))
//** CLANG ONLY**/

#if defined(__x86_64__)
/* 64 bit detected */
#endif
#if defined(__i386__)
/* 32 bit x86 detected */
#endif
#if defined(__arm__)
/* 32 bit arm detected */
#endif
#if defined(__aarch64__)
/* 64 bit arm detected */
#endif