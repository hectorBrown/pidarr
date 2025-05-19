# \FileSystemApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_filesystem_get**](FileSystemApi.md#api_v3_filesystem_get) | **GET** /api/v3/filesystem | 
[**api_v3_filesystem_mediafiles_get**](FileSystemApi.md#api_v3_filesystem_mediafiles_get) | **GET** /api/v3/filesystem/mediafiles | 
[**api_v3_filesystem_type_get**](FileSystemApi.md#api_v3_filesystem_type_get) | **GET** /api/v3/filesystem/type | 



## api_v3_filesystem_get

> api_v3_filesystem_get(path, include_files, allow_folders_without_trailing_slashes)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | Option<**String**> |  |  |
**include_files** | Option<**bool**> |  |  |[default to false]
**allow_folders_without_trailing_slashes** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_filesystem_mediafiles_get

> api_v3_filesystem_mediafiles_get(path)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_filesystem_type_get

> api_v3_filesystem_type_get(path)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

