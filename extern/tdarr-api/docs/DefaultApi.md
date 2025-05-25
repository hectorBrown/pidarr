# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v2_auth_status_post**](DefaultApi.md#api_v2_auth_status_post) | **POST** /api/v2/auth-status | 
[**api_v2_client_client_type_post**](DefaultApi.md#api_v2_client_client_type_post) | **POST** /api/v2/client/{clientType} | 
[**api_v2_create_sample_post**](DefaultApi.md#api_v2_create_sample_post) | **POST** /api/v2/create-sample | 
[**api_v2_cruddb_post**](DefaultApi.md#api_v2_cruddb_post) | **POST** /api/v2/cruddb | 
[**api_v2_debug_get**](DefaultApi.md#api_v2_debug_get) | **GET** /api/v2/debug | 
[**api_v2_debug_vars_type_get**](DefaultApi.md#api_v2_debug_vars_type_get) | **GET** /api/v2/debug-vars/{type} | 
[**api_v2_delete_cache_file_post**](DefaultApi.md#api_v2_delete_cache_file_post) | **POST** /api/v2/delete-cache-file | 
[**api_v2_delete_file_post**](DefaultApi.md#api_v2_delete_file_post) | **POST** /api/v2/delete-file | 
[**api_v2_delete_unhealthy_files_post**](DefaultApi.md#api_v2_delete_unhealthy_files_post) | **POST** /api/v2/delete-unhealthy-files | 
[**api_v2_find_duplicates_post**](DefaultApi.md#api_v2_find_duplicates_post) | **POST** /api/v2/find-duplicates | 
[**api_v2_get_db_statuses_post**](DefaultApi.md#api_v2_get_db_statuses_post) | **POST** /api/v2/get-db-statuses | 
[**api_v2_get_res_stats_post**](DefaultApi.md#api_v2_get_res_stats_post) | **POST** /api/v2/get-res-stats | 
[**api_v2_get_server_log_get**](DefaultApi.md#api_v2_get_server_log_get) | **GET** /api/v2/get-server-log | 
[**api_v2_get_time_now_post**](DefaultApi.md#api_v2_get_time_now_post) | **POST** /api/v2/get-time-now | 
[**api_v2_is_server_alive_post**](DefaultApi.md#api_v2_is_server_alive_post) | **POST** /api/v2/is-server-alive | 
[**api_v2_list_footprint_id_reports_post**](DefaultApi.md#api_v2_list_footprint_id_reports_post) | **POST** /api/v2/list-footprintId-reports | 
[**api_v2_performance_stats_post**](DefaultApi.md#api_v2_performance_stats_post) | **POST** /api/v2/performance-stats | 
[**api_v2_read_job_file_post**](DefaultApi.md#api_v2_read_job_file_post) | **POST** /api/v2/read-job-file | 
[**api_v2_rescan_file_post**](DefaultApi.md#api_v2_rescan_file_post) | **POST** /api/v2/rescan-file | 
[**api_v2_restart_server_post**](DefaultApi.md#api_v2_restart_server_post) | **POST** /api/v2/restart-server | 
[**api_v2_restart_ui_get**](DefaultApi.md#api_v2_restart_ui_get) | **GET** /api/v2/restart-ui | 
[**api_v2_run_help_command_post**](DefaultApi.md#api_v2_run_help_command_post) | **POST** /api/v2/run-help-command | 
[**api_v2_scan_individual_file_post**](DefaultApi.md#api_v2_scan_individual_file_post) | **POST** /api/v2/scan-individual-file | 
[**api_v2_search_db_post**](DefaultApi.md#api_v2_search_db_post) | **POST** /api/v2/search-db | 
[**api_v2_search_job_reports_post**](DefaultApi.md#api_v2_search_job_reports_post) | **POST** /api/v2/search-job-reports | 
[**api_v2_set_all_status_post**](DefaultApi.md#api_v2_set_all_status_post) | **POST** /api/v2/set-all-status | 
[**api_v2_status_get**](DefaultApi.md#api_v2_status_get) | **GET** /api/v2/status | 
[**api_v2_stop_dedupe_get**](DefaultApi.md#api_v2_stop_dedupe_get) | **GET** /api/v2/stop-dedupe | 
[**api_v2_transcode_user_verdict_post**](DefaultApi.md#api_v2_transcode_user_verdict_post) | **POST** /api/v2/transcode-user-verdict | 
[**api_v2_updater_check_post**](DefaultApi.md#api_v2_updater_check_post) | **POST** /api/v2/updater/check | 
[**api_v2_updater_package_index_post**](DefaultApi.md#api_v2_updater_package_index_post) | **POST** /api/v2/updater/package-index | 
[**api_v2_updater_relaunch_post**](DefaultApi.md#api_v2_updater_relaunch_post) | **POST** /api/v2/updater/relaunch | 
[**api_v2_use_token_post**](DefaultApi.md#api_v2_use_token_post) | **POST** /api/v2/use-token | 



## api_v2_auth_status_post

> bool api_v2_auth_status_post(body)


For checking Tdarr Pro status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2AuthStatusPostRequest**](ApiV2AuthStatusPostRequest.md)> |  |  |

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_client_client_type_post

> std::collections::HashMap<String, serde_json::Value> api_v2_client_client_type_post(client_type, body)


For loading and updating data in various tables found around the Tdarr UI

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_type** | **String** |  | [required] |
**body** | Option<[**ApiV2ClientClientTypePostRequest**](ApiV2ClientClientTypePostRequest.md)> |  |  |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_create_sample_post

> models::ApiV2CreateSamplePost200Response api_v2_create_sample_post(body)


For creating a 30 second sample of a file

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2CreateSamplePostRequest**](ApiV2CreateSamplePostRequest.md)> |  |  |

### Return type

[**models::ApiV2CreateSamplePost200Response**](_api_v2_create_sample_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_cruddb_post

> serde_json::Value api_v2_cruddb_post(body)


 For interacting with the database  insert:    requires collection, docID, obj (with keys/values to insert) getById:   requires collection, docID getByIndex:requires collection, docID getAll:    requires collection update:    requires collection, docID, obj (with keys/values to update) removeOne: requires collection, docID removeAll: requires collection   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2CruddbPostRequest**](ApiV2CruddbPostRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_debug_get

> String api_v2_debug_get()


For getting a page with various debug info

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_debug_vars_type_get

> std::collections::HashMap<String, serde_json::Value> api_v2_debug_vars_type_get()


For getting various debug info

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_delete_cache_file_post

> String api_v2_delete_cache_file_post(body)


For deleting a cache file

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2DeleteCacheFilePostRequest**](ApiV2DeleteCacheFilePostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_delete_file_post

> String api_v2_delete_file_post(body)


For deleting a file on disk of a file in Tdarr DB

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2DeleteFilePostRequest**](ApiV2DeleteFilePostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_delete_unhealthy_files_post

> String api_v2_delete_unhealthy_files_post(body)


For deleting files which have failed to transcode (table3) or unhealthy files (table6)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2DeleteUnhealthyFilesPostRequest**](ApiV2DeleteUnhealthyFilesPostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_find_duplicates_post

> String api_v2_find_duplicates_post(body)


For starting the find duplicates process

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2FindDuplicatesPostRequest**](ApiV2FindDuplicatesPostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_get_db_statuses_post

> std::collections::HashMap<String, serde_json::Value> api_v2_get_db_statuses_post()


For getting the statuses of the Tdarr database

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_get_res_stats_post

> models::ApiV2GetResStatsPost200Response api_v2_get_res_stats_post()


For getting server resource information

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiV2GetResStatsPost200Response**](_api_v2_get_res_stats_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_get_server_log_get

> String api_v2_get_server_log_get()


For getting the server log

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_get_time_now_post

> i32 api_v2_get_time_now_post()


For getting the current time on the server

### Parameters

This endpoint does not need any parameter.

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_is_server_alive_post

> models::ApiV2IsServerAlivePost200Response api_v2_is_server_alive_post()


Old endpoint for checking if the server is alive (user 'status' instead)

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiV2IsServerAlivePost200Response**](_api_v2_is_server_alive_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_list_footprint_id_reports_post

> Vec<String> api_v2_list_footprint_id_reports_post(body)


For listing all job reports for a specific footprintId

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2ListFootprintIdReportsPostRequest**](ApiV2ListFootprintIdReportsPostRequest.md)> |  |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_performance_stats_post

> models::ApiV2PerformanceStatsPost200Response api_v2_performance_stats_post()


For various performance stat info

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiV2PerformanceStatsPost200Response**](_api_v2_performance_stats_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_read_job_file_post

> models::ApiV2ReadJobFilePost200Response api_v2_read_job_file_post(body)


For reading a job report

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2ReadJobFilePostRequest**](ApiV2ReadJobFilePostRequest.md)> |  |  |

### Return type

[**models::ApiV2ReadJobFilePost200Response**](_api_v2_read_job_file_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_rescan_file_post

> String api_v2_rescan_file_post(body)


For rescanning a file

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2RescanFilePostRequest**](ApiV2RescanFilePostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_restart_server_post

> String api_v2_restart_server_post()


For restarting Tdarr Server

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_restart_ui_get

> String api_v2_restart_ui_get()


For restarting the UI

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_run_help_command_post

> String api_v2_run_help_command_post(body)


For running an ffmpeg or handbrake help command on the Help tab

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2RunHelpCommandPostRequest**](ApiV2RunHelpCommandPostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_scan_individual_file_post

> models::ApiV2ScanIndividualFilePost200Response api_v2_scan_individual_file_post(body)


For scanning an individual file with various tools

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2ScanIndividualFilePostRequest**](ApiV2ScanIndividualFilePostRequest.md)> |  |  |

### Return type

[**models::ApiV2ScanIndividualFilePost200Response**](_api_v2_scan_individual_file_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_search_db_post

> Vec<std::collections::HashMap<String, serde_json::Value>> api_v2_search_db_post(body)


Old endpoint for searching the file database (use 'client' endpoint instead)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2SearchDbPostRequest**](ApiV2SearchDbPostRequest.md)> |  |  |

### Return type

[**Vec<std::collections::HashMap<String, serde_json::Value>>**](std::collections::HashMap.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_search_job_reports_post

> models::ApiV2SearchJobReportsPost200Response api_v2_search_job_reports_post(body)


For searching job reports

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2SearchJobReportsPostRequest**](ApiV2SearchJobReportsPostRequest.md)> |  |  |

### Return type

[**models::ApiV2SearchJobReportsPost200Response**](_api_v2_search_job_reports_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_set_all_status_post

> String api_v2_set_all_status_post(body)


For requeueing files for transcode or health check for a specific library

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2SetAllStatusPostRequest**](ApiV2SetAllStatusPostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_status_get

> models::ApiV2StatusGet200Response api_v2_status_get()


For checking server status

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiV2StatusGet200Response**](_api_v2_status_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_stop_dedupe_get

> String api_v2_stop_dedupe_get()


For stopping the dedupe process

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_transcode_user_verdict_post

> String api_v2_transcode_user_verdict_post(body)


For taking action on a staged item

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2TranscodeUserVerdictPostRequest**](ApiV2TranscodeUserVerdictPostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_updater_check_post

> models::ApiV2UpdaterCheckPost200Response api_v2_updater_check_post(body)


For checking if an update is available

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2UpdaterCheckPostRequest**](ApiV2UpdaterCheckPostRequest.md)> |  |  |

### Return type

[**models::ApiV2UpdaterCheckPost200Response**](_api_v2_updater_check_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_updater_package_index_post

> std::collections::HashMap<String, serde_json::Value> api_v2_updater_package_index_post()


For getting the package index

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_updater_relaunch_post

> String api_v2_updater_relaunch_post()


For relaunching Tdarr Server when an update is ready

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_use_token_post

> std::collections::HashMap<String, serde_json::Value> api_v2_use_token_post(body)


For using a token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2UseTokenPostRequest**](ApiV2UseTokenPostRequest.md)> |  |  |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

