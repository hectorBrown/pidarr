# \DownloadClientConfigApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_config_downloadclient_get**](DownloadClientConfigApi.md#api_v3_config_downloadclient_get) | **GET** /api/v3/config/downloadclient | 
[**api_v3_config_downloadclient_id_get**](DownloadClientConfigApi.md#api_v3_config_downloadclient_id_get) | **GET** /api/v3/config/downloadclient/{id} | 
[**api_v3_config_downloadclient_id_put**](DownloadClientConfigApi.md#api_v3_config_downloadclient_id_put) | **PUT** /api/v3/config/downloadclient/{id} | 



## api_v3_config_downloadclient_get

> models::DownloadClientConfigResource api_v3_config_downloadclient_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::DownloadClientConfigResource**](DownloadClientConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_config_downloadclient_id_get

> models::DownloadClientConfigResource api_v3_config_downloadclient_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::DownloadClientConfigResource**](DownloadClientConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_config_downloadclient_id_put

> models::DownloadClientConfigResource api_v3_config_downloadclient_id_put(id, download_client_config_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**download_client_config_resource** | Option<[**DownloadClientConfigResource**](DownloadClientConfigResource.md)> |  |  |

### Return type

[**models::DownloadClientConfigResource**](DownloadClientConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

