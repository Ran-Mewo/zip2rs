#ifndef __ZIP4J_ABI_H
#define __ZIP4J_ABI_H

/* Additional header includes */
#include "zip4j-extras.h"


#include <graal_isolate.h>


#if defined(__cplusplus)
extern "C" {
#endif

int zip4j_init(graal_isolatethread_t*);

int zip4j_cleanup(graal_isolatethread_t*);

int zip4j_create(graal_isolatethread_t*, char*, long long*);

int zip4j_create_with_password(graal_isolatethread_t*, char*, char*, long long*);

int zip4j_set_password(graal_isolatethread_t*, long long int, char*);

int zip4j_close(graal_isolatethread_t*, long long int);

int zip4j_is_valid(graal_isolatethread_t*, long long int, int*);

int zip4j_is_encrypted(graal_isolatethread_t*, long long int, int*);

int zip4j_is_split_archive(graal_isolatethread_t*, long long int, int*);

int zip4j_get_file_path(graal_isolatethread_t*, long long int, char*, int, int*);

int zip4j_get_comment(graal_isolatethread_t*, long long int, char*, int, int*);

int zip4j_set_comment(graal_isolatethread_t*, long long int, char*);

int zip4j_get_entry_count(graal_isolatethread_t*, long long int, long long*);

int zip4j_get_entry_by_index(graal_isolatethread_t*, long long int, long long int, long long*);

int zip4j_get_entry_by_name(graal_isolatethread_t*, long long int, char*, long long*);

int zip4j_release_entry(graal_isolatethread_t*, long long int);

int zip4j_entry_get_name(graal_isolatethread_t*, long long int, char*, int, int*);

int zip4j_entry_get_size(graal_isolatethread_t*, long long int, long long*);

int zip4j_entry_get_compressed_size(graal_isolatethread_t*, long long int, long long*);

int zip4j_entry_is_directory(graal_isolatethread_t*, long long int, int*);

int zip4j_entry_is_encrypted(graal_isolatethread_t*, long long int, int*);

int zip4j_entry_get_crc(graal_isolatethread_t*, long long int, long long*);

int zip4j_entry_get_last_modified_time(graal_isolatethread_t*, long long int, long long*);

int zip4j_add_file(graal_isolatethread_t*, long long int, char*);

int zip4j_add_file_with_params(graal_isolatethread_t*, long long int, char*, int, int, int, int, char*);

int zip4j_add_directory(graal_isolatethread_t*, long long int, char*);

int zip4j_add_directory_with_params(graal_isolatethread_t*, long long int, char*, int, int, int, int, char*);

int zip4j_extract_all(graal_isolatethread_t*, long long int, char*);

int zip4j_extract_file(graal_isolatethread_t*, long long int, char*, char*);

int zip4j_extract_entry(graal_isolatethread_t*, long long int, long long int, char*);

int zip4j_remove_file(graal_isolatethread_t*, long long int, char*);

int zip4j_remove_entry(graal_isolatethread_t*, long long int, long long int);

int zip4j_create_input_stream(graal_isolatethread_t*, long long int, long long int, long long*);

int zip4j_stream_read(graal_isolatethread_t*, long long int, char*, int, int*);

int zip4j_close_input_stream(graal_isolatethread_t*, long long int);

int zip4j_get_progress_monitor(graal_isolatethread_t*, long long int, long long*);

int zip4j_get_progress_percentage(graal_isolatethread_t*, long long int, int*);

int zip4j_is_operation_finished(graal_isolatethread_t*, long long int, int*);

int zip4j_cancel_operation(graal_isolatethread_t*, long long int);

int zip4j_get_last_error(graal_isolatethread_t*, long long int, char*, int, int*);

int zip4j_create_split_zip(graal_isolatethread_t*, char*, long long int, long long*);

int zip4j_merge_split_files(graal_isolatethread_t*, long long int, char*);

int zip4j_rename_entry(graal_isolatethread_t*, long long int, long long int, char*);

int zip4j_add_data(graal_isolatethread_t*, long long int, char*, char*, int, int, int, int, int, char*);

int zip4j_extract_data(graal_isolatethread_t*, long long int, long long int, char*, int, int*);

int zip4j_entry_get_compression_method(graal_isolatethread_t*, long long int, int*);

int zip4j_entry_get_encryption_method(graal_isolatethread_t*, long long int, int*);

#if defined(__cplusplus)
}
#endif
#endif