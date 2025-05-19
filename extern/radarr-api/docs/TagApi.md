# \TagApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_tag_get**](TagApi.md#api_v3_tag_get) | **GET** /api/v3/tag | 
[**api_v3_tag_id_delete**](TagApi.md#api_v3_tag_id_delete) | **DELETE** /api/v3/tag/{id} | 
[**api_v3_tag_id_get**](TagApi.md#api_v3_tag_id_get) | **GET** /api/v3/tag/{id} | 
[**api_v3_tag_id_put**](TagApi.md#api_v3_tag_id_put) | **PUT** /api/v3/tag/{id} | 
[**api_v3_tag_post**](TagApi.md#api_v3_tag_post) | **POST** /api/v3/tag | 



## api_v3_tag_get

> Vec<models::TagResource> api_v3_tag_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::TagResource>**](TagResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_tag_id_delete

> api_v3_tag_id_delete(id)


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


## api_v3_tag_id_get

> models::TagResource api_v3_tag_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::TagResource**](TagResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_tag_id_put

> models::TagResource api_v3_tag_id_put(id, tag_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**tag_resource** | Option<[**TagResource**](TagResource.md)> |  |  |

### Return type

[**models::TagResource**](TagResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_tag_post

> models::TagResource api_v3_tag_post(tag_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_resource** | Option<[**TagResource**](TagResource.md)> |  |  |

### Return type

[**models::TagResource**](TagResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

