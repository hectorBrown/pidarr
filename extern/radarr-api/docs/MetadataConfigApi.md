# \MetadataConfigApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_config_metadata_get**](MetadataConfigApi.md#api_v3_config_metadata_get) | **GET** /api/v3/config/metadata | 
[**api_v3_config_metadata_id_get**](MetadataConfigApi.md#api_v3_config_metadata_id_get) | **GET** /api/v3/config/metadata/{id} | 
[**api_v3_config_metadata_id_put**](MetadataConfigApi.md#api_v3_config_metadata_id_put) | **PUT** /api/v3/config/metadata/{id} | 



## api_v3_config_metadata_get

> models::MetadataConfigResource api_v3_config_metadata_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MetadataConfigResource**](MetadataConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_config_metadata_id_get

> models::MetadataConfigResource api_v3_config_metadata_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::MetadataConfigResource**](MetadataConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_config_metadata_id_put

> models::MetadataConfigResource api_v3_config_metadata_id_put(id, metadata_config_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**metadata_config_resource** | Option<[**MetadataConfigResource**](MetadataConfigResource.md)> |  |  |

### Return type

[**models::MetadataConfigResource**](MetadataConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

