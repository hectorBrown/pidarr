# \ImportListConfigApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_config_importlist_get**](ImportListConfigApi.md#api_v3_config_importlist_get) | **GET** /api/v3/config/importlist | 
[**api_v3_config_importlist_id_get**](ImportListConfigApi.md#api_v3_config_importlist_id_get) | **GET** /api/v3/config/importlist/{id} | 
[**api_v3_config_importlist_id_put**](ImportListConfigApi.md#api_v3_config_importlist_id_put) | **PUT** /api/v3/config/importlist/{id} | 



## api_v3_config_importlist_get

> models::ImportListConfigResource api_v3_config_importlist_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ImportListConfigResource**](ImportListConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_config_importlist_id_get

> models::ImportListConfigResource api_v3_config_importlist_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::ImportListConfigResource**](ImportListConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_config_importlist_id_put

> models::ImportListConfigResource api_v3_config_importlist_id_put(id, import_list_config_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**import_list_config_resource** | Option<[**ImportListConfigResource**](ImportListConfigResource.md)> |  |  |

### Return type

[**models::ImportListConfigResource**](ImportListConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

