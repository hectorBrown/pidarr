# \LogFileApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_log_file_filename_get**](LogFileApi.md#api_v3_log_file_filename_get) | **GET** /api/v3/log/file/{filename} | 
[**api_v3_log_file_get**](LogFileApi.md#api_v3_log_file_get) | **GET** /api/v3/log/file | 



## api_v3_log_file_filename_get

> api_v3_log_file_filename_get(filename)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filename** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_log_file_get

> Vec<models::LogFileResource> api_v3_log_file_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::LogFileResource>**](LogFileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

