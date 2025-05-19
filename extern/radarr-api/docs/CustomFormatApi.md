# \CustomFormatApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_customformat_bulk_delete**](CustomFormatApi.md#api_v3_customformat_bulk_delete) | **DELETE** /api/v3/customformat/bulk | 
[**api_v3_customformat_bulk_put**](CustomFormatApi.md#api_v3_customformat_bulk_put) | **PUT** /api/v3/customformat/bulk | 
[**api_v3_customformat_get**](CustomFormatApi.md#api_v3_customformat_get) | **GET** /api/v3/customformat | 
[**api_v3_customformat_id_delete**](CustomFormatApi.md#api_v3_customformat_id_delete) | **DELETE** /api/v3/customformat/{id} | 
[**api_v3_customformat_id_get**](CustomFormatApi.md#api_v3_customformat_id_get) | **GET** /api/v3/customformat/{id} | 
[**api_v3_customformat_id_put**](CustomFormatApi.md#api_v3_customformat_id_put) | **PUT** /api/v3/customformat/{id} | 
[**api_v3_customformat_post**](CustomFormatApi.md#api_v3_customformat_post) | **POST** /api/v3/customformat | 
[**api_v3_customformat_schema_get**](CustomFormatApi.md#api_v3_customformat_schema_get) | **GET** /api/v3/customformat/schema | 



## api_v3_customformat_bulk_delete

> api_v3_customformat_bulk_delete(custom_format_bulk_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**custom_format_bulk_resource** | Option<[**CustomFormatBulkResource**](CustomFormatBulkResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_customformat_bulk_put

> models::CustomFormatResource api_v3_customformat_bulk_put(custom_format_bulk_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**custom_format_bulk_resource** | Option<[**CustomFormatBulkResource**](CustomFormatBulkResource.md)> |  |  |

### Return type

[**models::CustomFormatResource**](CustomFormatResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_customformat_get

> Vec<models::CustomFormatResource> api_v3_customformat_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::CustomFormatResource>**](CustomFormatResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_customformat_id_delete

> api_v3_customformat_id_delete(id)


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


## api_v3_customformat_id_get

> models::CustomFormatResource api_v3_customformat_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::CustomFormatResource**](CustomFormatResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_customformat_id_put

> models::CustomFormatResource api_v3_customformat_id_put(id, custom_format_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**custom_format_resource** | Option<[**CustomFormatResource**](CustomFormatResource.md)> |  |  |

### Return type

[**models::CustomFormatResource**](CustomFormatResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_customformat_post

> models::CustomFormatResource api_v3_customformat_post(custom_format_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**custom_format_resource** | Option<[**CustomFormatResource**](CustomFormatResource.md)> |  |  |

### Return type

[**models::CustomFormatResource**](CustomFormatResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_customformat_schema_get

> api_v3_customformat_schema_get()


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

