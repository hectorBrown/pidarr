# \MetadataApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_metadata_action_name_post**](MetadataApi.md#api_v3_metadata_action_name_post) | **POST** /api/v3/metadata/action/{name} | 
[**api_v3_metadata_get**](MetadataApi.md#api_v3_metadata_get) | **GET** /api/v3/metadata | 
[**api_v3_metadata_id_delete**](MetadataApi.md#api_v3_metadata_id_delete) | **DELETE** /api/v3/metadata/{id} | 
[**api_v3_metadata_id_get**](MetadataApi.md#api_v3_metadata_id_get) | **GET** /api/v3/metadata/{id} | 
[**api_v3_metadata_id_put**](MetadataApi.md#api_v3_metadata_id_put) | **PUT** /api/v3/metadata/{id} | 
[**api_v3_metadata_post**](MetadataApi.md#api_v3_metadata_post) | **POST** /api/v3/metadata | 
[**api_v3_metadata_schema_get**](MetadataApi.md#api_v3_metadata_schema_get) | **GET** /api/v3/metadata/schema | 
[**api_v3_metadata_test_post**](MetadataApi.md#api_v3_metadata_test_post) | **POST** /api/v3/metadata/test | 
[**api_v3_metadata_testall_post**](MetadataApi.md#api_v3_metadata_testall_post) | **POST** /api/v3/metadata/testall | 



## api_v3_metadata_action_name_post

> api_v3_metadata_action_name_post(name, metadata_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**metadata_resource** | Option<[**MetadataResource**](MetadataResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_metadata_get

> Vec<models::MetadataResource> api_v3_metadata_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::MetadataResource>**](MetadataResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_metadata_id_delete

> api_v3_metadata_id_delete(id)


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


## api_v3_metadata_id_get

> models::MetadataResource api_v3_metadata_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::MetadataResource**](MetadataResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_metadata_id_put

> models::MetadataResource api_v3_metadata_id_put(id, force_save, metadata_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**force_save** | Option<**bool**> |  |  |[default to false]
**metadata_resource** | Option<[**MetadataResource**](MetadataResource.md)> |  |  |

### Return type

[**models::MetadataResource**](MetadataResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_metadata_post

> models::MetadataResource api_v3_metadata_post(force_save, metadata_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_save** | Option<**bool**> |  |  |[default to false]
**metadata_resource** | Option<[**MetadataResource**](MetadataResource.md)> |  |  |

### Return type

[**models::MetadataResource**](MetadataResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_metadata_schema_get

> Vec<models::MetadataResource> api_v3_metadata_schema_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::MetadataResource>**](MetadataResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_metadata_test_post

> api_v3_metadata_test_post(force_test, metadata_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_test** | Option<**bool**> |  |  |[default to false]
**metadata_resource** | Option<[**MetadataResource**](MetadataResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_metadata_testall_post

> api_v3_metadata_testall_post()


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

