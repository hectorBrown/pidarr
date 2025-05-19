# \AutoTaggingApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_autotagging_get**](AutoTaggingApi.md#api_v3_autotagging_get) | **GET** /api/v3/autotagging | 
[**api_v3_autotagging_id_delete**](AutoTaggingApi.md#api_v3_autotagging_id_delete) | **DELETE** /api/v3/autotagging/{id} | 
[**api_v3_autotagging_id_get**](AutoTaggingApi.md#api_v3_autotagging_id_get) | **GET** /api/v3/autotagging/{id} | 
[**api_v3_autotagging_id_put**](AutoTaggingApi.md#api_v3_autotagging_id_put) | **PUT** /api/v3/autotagging/{id} | 
[**api_v3_autotagging_post**](AutoTaggingApi.md#api_v3_autotagging_post) | **POST** /api/v3/autotagging | 
[**api_v3_autotagging_schema_get**](AutoTaggingApi.md#api_v3_autotagging_schema_get) | **GET** /api/v3/autotagging/schema | 



## api_v3_autotagging_get

> Vec<models::AutoTaggingResource> api_v3_autotagging_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::AutoTaggingResource>**](AutoTaggingResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_autotagging_id_delete

> api_v3_autotagging_id_delete(id)


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


## api_v3_autotagging_id_get

> models::AutoTaggingResource api_v3_autotagging_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::AutoTaggingResource**](AutoTaggingResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_autotagging_id_put

> models::AutoTaggingResource api_v3_autotagging_id_put(id, auto_tagging_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**auto_tagging_resource** | Option<[**AutoTaggingResource**](AutoTaggingResource.md)> |  |  |

### Return type

[**models::AutoTaggingResource**](AutoTaggingResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_autotagging_post

> models::AutoTaggingResource api_v3_autotagging_post(auto_tagging_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auto_tagging_resource** | Option<[**AutoTaggingResource**](AutoTaggingResource.md)> |  |  |

### Return type

[**models::AutoTaggingResource**](AutoTaggingResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_autotagging_schema_get

> api_v3_autotagging_schema_get()


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

