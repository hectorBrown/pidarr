# \BackupsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v2_create_backup_post**](BackupsApi.md#api_v2_create_backup_post) | **POST** /api/v2/create-backup | 
[**api_v2_delete_backup_post**](BackupsApi.md#api_v2_delete_backup_post) | **POST** /api/v2/delete-backup | 
[**api_v2_get_backup_status_post**](BackupsApi.md#api_v2_get_backup_status_post) | **POST** /api/v2/get-backup-status | 
[**api_v2_get_backups_post**](BackupsApi.md#api_v2_get_backups_post) | **POST** /api/v2/get-backups | 
[**api_v2_reset_backup_status_post**](BackupsApi.md#api_v2_reset_backup_status_post) | **POST** /api/v2/reset-backup-status | 



## api_v2_create_backup_post

> bool api_v2_create_backup_post()


For creating a backup of the Tdarr database

### Parameters

This endpoint does not need any parameter.

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_delete_backup_post

> serde_json::Value api_v2_delete_backup_post(body)


For deleting a backup of the Tdarr database

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2DeleteBackupPostRequest**](ApiV2DeleteBackupPostRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_get_backup_status_post

> Vec<models::ApiV2GetBackupStatusPost200ResponseInner> api_v2_get_backup_status_post()


For getting the status of a Tdarr backup in progress

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ApiV2GetBackupStatusPost200ResponseInner>**](_api_v2_get_backup_status_post_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_get_backups_post

> Vec<models::ApiV2GetBackupsPost200ResponseInner> api_v2_get_backups_post()


For getting a list of backups of the Tdarr database

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ApiV2GetBackupsPost200ResponseInner>**](_api_v2_get_backups_post_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_reset_backup_status_post

> String api_v2_reset_backup_status_post()


For resetting the backup status

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

