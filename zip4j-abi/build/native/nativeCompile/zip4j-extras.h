#ifndef ZIP4J_ABI_H
#define ZIP4J_ABI_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// Error codes
#define ZIP4J_SUCCESS 0
#define ZIP4J_ERROR_INVALID_HANDLE -1
#define ZIP4J_ERROR_FILE_NOT_FOUND -2
#define ZIP4J_ERROR_ZIP_EXCEPTION -3
#define ZIP4J_ERROR_IO_EXCEPTION -4
#define ZIP4J_ERROR_INVALID_PARAMETER -5
#define ZIP4J_ERROR_OUT_OF_MEMORY -6
#define ZIP4J_ERROR_ENTRY_NOT_FOUND -7
#define ZIP4J_ERROR_UNKNOWN -999

// Encryption methods
#define ZIP4J_ENCRYPTION_NONE 0
#define ZIP4J_ENCRYPTION_STANDARD 1
#define ZIP4J_ENCRYPTION_AES_128 2
#define ZIP4J_ENCRYPTION_AES_256 3

#ifdef __cplusplus
}
#endif

#endif // ZIP4J_ABI_H