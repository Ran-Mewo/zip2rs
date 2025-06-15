#ifndef __ZIP4J_ABI_H
#define __ZIP4J_ABI_H

#include <graal_isolate_dynamic.h>


#if defined(__cplusplus)
extern "C" {
#endif

typedef int (*zip4j_init_fn_t)(graal_isolatethread_t*);

typedef int (*zip4j_cleanup_fn_t)(graal_isolatethread_t*);

typedef int (*zip4j_create_fn_t)(graal_isolatethread_t*, char*, long long*);

typedef int (*zip4j_create_with_password_fn_t)(graal_isolatethread_t*, char*, char*, long long*);

typedef int (*zip4j_set_password_fn_t)(graal_isolatethread_t*, long long int, char*);

typedef int (*zip4j_close_fn_t)(graal_isolatethread_t*, long long int);

typedef int (*zip4j_is_valid_fn_t)(graal_isolatethread_t*, long long int, int*);

typedef int (*zip4j_is_encrypted_fn_t)(graal_isolatethread_t*, long long int, int*);

typedef int (*zip4j_is_split_archive_fn_t)(graal_isolatethread_t*, long long int, int*);

typedef int (*zip4j_get_file_path_fn_t)(graal_isolatethread_t*, long long int, char*, int, int*);

typedef int (*zip4j_get_comment_fn_t)(graal_isolatethread_t*, long long int, char*, int, int*);

typedef int (*zip4j_set_comment_fn_t)(graal_isolatethread_t*, long long int, char*);

typedef int (*zip4j_get_entry_count_fn_t)(graal_isolatethread_t*, long long int, long long*);

typedef int (*zip4j_get_entry_by_index_fn_t)(graal_isolatethread_t*, long long int, long long int, long long*);

typedef int (*zip4j_get_entry_by_name_fn_t)(graal_isolatethread_t*, long long int, char*, long long*);

typedef int (*zip4j_release_entry_fn_t)(graal_isolatethread_t*, long long int);

typedef int (*zip4j_entry_get_name_fn_t)(graal_isolatethread_t*, long long int, char*, int, int*);

typedef int (*zip4j_entry_get_size_fn_t)(graal_isolatethread_t*, long long int, long long*);

typedef int (*zip4j_entry_get_compressed_size_fn_t)(graal_isolatethread_t*, long long int, long long*);

typedef int (*zip4j_entry_is_directory_fn_t)(graal_isolatethread_t*, long long int, int*);

typedef int (*zip4j_entry_is_encrypted_fn_t)(graal_isolatethread_t*, long long int, int*);

typedef int (*zip4j_entry_get_crc_fn_t)(graal_isolatethread_t*, long long int, long long*);

typedef int (*zip4j_entry_get_last_modified_time_fn_t)(graal_isolatethread_t*, long long int, long long*);

typedef int (*zip4j_add_file_fn_t)(graal_isolatethread_t*, long long int, char*);

typedef int (*zip4j_add_file_with_params_fn_t)(graal_isolatethread_t*, long long int, char*, int, int, int, int, char*);

typedef int (*zip4j_add_directory_fn_t)(graal_isolatethread_t*, long long int, char*);

typedef int (*zip4j_add_directory_with_params_fn_t)(graal_isolatethread_t*, long long int, char*, int, int, int, int, char*);

typedef int (*zip4j_extract_all_fn_t)(graal_isolatethread_t*, long long int, char*);

typedef int (*zip4j_extract_file_fn_t)(graal_isolatethread_t*, long long int, char*, char*);

typedef int (*zip4j_extract_entry_fn_t)(graal_isolatethread_t*, long long int, long long int, char*);

typedef int (*zip4j_remove_file_fn_t)(graal_isolatethread_t*, long long int, char*);

typedef int (*zip4j_remove_entry_fn_t)(graal_isolatethread_t*, long long int, long long int);

typedef int (*zip4j_create_input_stream_fn_t)(graal_isolatethread_t*, long long int, long long int, long long*);

typedef int (*zip4j_stream_read_fn_t)(graal_isolatethread_t*, long long int, char*, int, int*);

typedef int (*zip4j_close_input_stream_fn_t)(graal_isolatethread_t*, long long int);

typedef int (*zip4j_get_progress_monitor_fn_t)(graal_isolatethread_t*, long long int, long long*);

typedef int (*zip4j_get_progress_percentage_fn_t)(graal_isolatethread_t*, long long int, int*);

typedef int (*zip4j_is_operation_finished_fn_t)(graal_isolatethread_t*, long long int, int*);

typedef int (*zip4j_cancel_operation_fn_t)(graal_isolatethread_t*, long long int);

typedef int (*zip4j_get_last_error_fn_t)(graal_isolatethread_t*, long long int, char*, int, int*);

typedef int (*zip4j_create_split_zip_fn_t)(graal_isolatethread_t*, char*, long long int, long long*);

typedef int (*zip4j_merge_split_files_fn_t)(graal_isolatethread_t*, long long int, char*);

typedef int (*zip4j_rename_entry_fn_t)(graal_isolatethread_t*, long long int, long long int, char*);

typedef int (*zip4j_add_data_fn_t)(graal_isolatethread_t*, long long int, char*, char*, int, int, int, int, int, char*);

typedef int (*zip4j_extract_data_fn_t)(graal_isolatethread_t*, long long int, long long int, char*, int, int*);

typedef int (*zip4j_entry_get_compression_method_fn_t)(graal_isolatethread_t*, long long int, int*);

typedef int (*zip4j_entry_get_encryption_method_fn_t)(graal_isolatethread_t*, long long int, int*);

#if defined(__cplusplus)
}
#endif
#endif
