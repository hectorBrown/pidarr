# \CustomFilterApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_customfilter_get**](CustomFilterApi.md#api_v3_customfilter_get) | **GET** /api/v3/customfilter | 
[**api_v3_customfilter_id_delete**](CustomFilterApi.md#api_v3_customfilter_id_delete) | **DELETE** /api/v3/customfilter/{id} | 
[**api_v3_customfilter_id_get**](CustomFilterApi.md#api_v3_customfilter_id_get) | **GET** /api/v3/customfilter/{id} | 
[**api_v3_customfilter_id_put**](CustomFilterApi.md#api_v3_customfilter_id_put) | **PUT** /api/v3/customfilter/{id} | 
[**api_v3_customfilter_post**](CustomFilterApi.md#api_v3_customfilter_post) | **POST** /api/v3/customfilter | 



## api_v3_customfilter_get

> Vec<models::CustomFilterResource> api_v3_customfilter_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::CustomFilterResource>**](CustomFilterResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_customfilter_id_delete

> api_v3_customfilter_id_delete(id)


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


## api_v3_customfilter_id_get

> models::CustomFilterResource api_v3_customfilter_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::CustomFilterResource**](CustomFilterResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_customfilter_id_put

> models::CustomFilterResource api_v3_customfilter_id_put(id, custom_filter_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**custom_filter_resource** | Option<[**CustomFilterResource**](CustomFilterResource.md)> |  |  |

### Return type

[**models::CustomFilterResource**](CustomFilterResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_customfilter_post

> models::CustomFilterResource api_v3_customfilter_post(custom_filter_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**custom_filter_resource** | Option<[**CustomFilterResource**](CustomFilterResource.md)> |  |  |

### Return type

[**models::CustomFilterResource**](CustomFilterResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

