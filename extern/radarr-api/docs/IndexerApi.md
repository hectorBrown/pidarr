# \IndexerApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_indexer_action_name_post**](IndexerApi.md#api_v3_indexer_action_name_post) | **POST** /api/v3/indexer/action/{name} | 
[**api_v3_indexer_bulk_delete**](IndexerApi.md#api_v3_indexer_bulk_delete) | **DELETE** /api/v3/indexer/bulk | 
[**api_v3_indexer_bulk_put**](IndexerApi.md#api_v3_indexer_bulk_put) | **PUT** /api/v3/indexer/bulk | 
[**api_v3_indexer_get**](IndexerApi.md#api_v3_indexer_get) | **GET** /api/v3/indexer | 
[**api_v3_indexer_id_delete**](IndexerApi.md#api_v3_indexer_id_delete) | **DELETE** /api/v3/indexer/{id} | 
[**api_v3_indexer_id_get**](IndexerApi.md#api_v3_indexer_id_get) | **GET** /api/v3/indexer/{id} | 
[**api_v3_indexer_id_put**](IndexerApi.md#api_v3_indexer_id_put) | **PUT** /api/v3/indexer/{id} | 
[**api_v3_indexer_post**](IndexerApi.md#api_v3_indexer_post) | **POST** /api/v3/indexer | 
[**api_v3_indexer_schema_get**](IndexerApi.md#api_v3_indexer_schema_get) | **GET** /api/v3/indexer/schema | 
[**api_v3_indexer_test_post**](IndexerApi.md#api_v3_indexer_test_post) | **POST** /api/v3/indexer/test | 
[**api_v3_indexer_testall_post**](IndexerApi.md#api_v3_indexer_testall_post) | **POST** /api/v3/indexer/testall | 



## api_v3_indexer_action_name_post

> api_v3_indexer_action_name_post(name, indexer_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**indexer_resource** | Option<[**IndexerResource**](IndexerResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_indexer_bulk_delete

> api_v3_indexer_bulk_delete(indexer_bulk_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**indexer_bulk_resource** | Option<[**IndexerBulkResource**](IndexerBulkResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_indexer_bulk_put

> models::IndexerResource api_v3_indexer_bulk_put(indexer_bulk_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**indexer_bulk_resource** | Option<[**IndexerBulkResource**](IndexerBulkResource.md)> |  |  |

### Return type

[**models::IndexerResource**](IndexerResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_indexer_get

> Vec<models::IndexerResource> api_v3_indexer_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::IndexerResource>**](IndexerResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_indexer_id_delete

> api_v3_indexer_id_delete(id)


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


## api_v3_indexer_id_get

> models::IndexerResource api_v3_indexer_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::IndexerResource**](IndexerResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_indexer_id_put

> models::IndexerResource api_v3_indexer_id_put(id, force_save, indexer_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**force_save** | Option<**bool**> |  |  |[default to false]
**indexer_resource** | Option<[**IndexerResource**](IndexerResource.md)> |  |  |

### Return type

[**models::IndexerResource**](IndexerResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_indexer_post

> models::IndexerResource api_v3_indexer_post(force_save, indexer_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_save** | Option<**bool**> |  |  |[default to false]
**indexer_resource** | Option<[**IndexerResource**](IndexerResource.md)> |  |  |

### Return type

[**models::IndexerResource**](IndexerResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_indexer_schema_get

> Vec<models::IndexerResource> api_v3_indexer_schema_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::IndexerResource>**](IndexerResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_indexer_test_post

> api_v3_indexer_test_post(force_test, indexer_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_test** | Option<**bool**> |  |  |[default to false]
**indexer_resource** | Option<[**IndexerResource**](IndexerResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_indexer_testall_post

> api_v3_indexer_testall_post()


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

