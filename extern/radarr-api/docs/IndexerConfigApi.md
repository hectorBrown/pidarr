# \IndexerConfigApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_config_indexer_get**](IndexerConfigApi.md#api_v3_config_indexer_get) | **GET** /api/v3/config/indexer | 
[**api_v3_config_indexer_id_get**](IndexerConfigApi.md#api_v3_config_indexer_id_get) | **GET** /api/v3/config/indexer/{id} | 
[**api_v3_config_indexer_id_put**](IndexerConfigApi.md#api_v3_config_indexer_id_put) | **PUT** /api/v3/config/indexer/{id} | 



## api_v3_config_indexer_get

> models::IndexerConfigResource api_v3_config_indexer_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::IndexerConfigResource**](IndexerConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_config_indexer_id_get

> models::IndexerConfigResource api_v3_config_indexer_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::IndexerConfigResource**](IndexerConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_config_indexer_id_put

> models::IndexerConfigResource api_v3_config_indexer_id_put(id, indexer_config_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**indexer_config_resource** | Option<[**IndexerConfigResource**](IndexerConfigResource.md)> |  |  |

### Return type

[**models::IndexerConfigResource**](IndexerConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

