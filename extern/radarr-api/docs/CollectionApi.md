# \CollectionApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_collection_get**](CollectionApi.md#api_v3_collection_get) | **GET** /api/v3/collection | 
[**api_v3_collection_id_get**](CollectionApi.md#api_v3_collection_id_get) | **GET** /api/v3/collection/{id} | 
[**api_v3_collection_id_put**](CollectionApi.md#api_v3_collection_id_put) | **PUT** /api/v3/collection/{id} | 
[**api_v3_collection_put**](CollectionApi.md#api_v3_collection_put) | **PUT** /api/v3/collection | 



## api_v3_collection_get

> Vec<models::CollectionResource> api_v3_collection_get(tmdb_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tmdb_id** | Option<**i32**> |  |  |

### Return type

[**Vec<models::CollectionResource>**](CollectionResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_collection_id_get

> models::CollectionResource api_v3_collection_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::CollectionResource**](CollectionResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_collection_id_put

> models::CollectionResource api_v3_collection_id_put(id, collection_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**collection_resource** | Option<[**CollectionResource**](CollectionResource.md)> |  |  |

### Return type

[**models::CollectionResource**](CollectionResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_collection_put

> api_v3_collection_put(collection_update_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_update_resource** | Option<[**CollectionUpdateResource**](CollectionUpdateResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

