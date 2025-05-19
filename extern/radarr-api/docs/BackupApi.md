# \BackupApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_system_backup_get**](BackupApi.md#api_v3_system_backup_get) | **GET** /api/v3/system/backup | 
[**api_v3_system_backup_id_delete**](BackupApi.md#api_v3_system_backup_id_delete) | **DELETE** /api/v3/system/backup/{id} | 
[**api_v3_system_backup_restore_id_post**](BackupApi.md#api_v3_system_backup_restore_id_post) | **POST** /api/v3/system/backup/restore/{id} | 
[**api_v3_system_backup_restore_upload_post**](BackupApi.md#api_v3_system_backup_restore_upload_post) | **POST** /api/v3/system/backup/restore/upload | 



## api_v3_system_backup_get

> Vec<models::BackupResource> api_v3_system_backup_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::BackupResource>**](BackupResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_system_backup_id_delete

> api_v3_system_backup_id_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_system_backup_restore_id_post

> api_v3_system_backup_restore_id_post(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_system_backup_restore_upload_post

> api_v3_system_backup_restore_upload_post()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

