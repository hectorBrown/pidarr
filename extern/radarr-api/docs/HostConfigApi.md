# \HostConfigApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_config_host_get**](HostConfigApi.md#api_v3_config_host_get) | **GET** /api/v3/config/host | 
[**api_v3_config_host_id_get**](HostConfigApi.md#api_v3_config_host_id_get) | **GET** /api/v3/config/host/{id} | 
[**api_v3_config_host_id_put**](HostConfigApi.md#api_v3_config_host_id_put) | **PUT** /api/v3/config/host/{id} | 



## api_v3_config_host_get

> models::HostConfigResource api_v3_config_host_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HostConfigResource**](HostConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_config_host_id_get

> models::HostConfigResource api_v3_config_host_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::HostConfigResource**](HostConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_config_host_id_put

> models::HostConfigResource api_v3_config_host_id_put(id, host_config_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**host_config_resource** | Option<[**HostConfigResource**](HostConfigResource.md)> |  |  |

### Return type

[**models::HostConfigResource**](HostConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

