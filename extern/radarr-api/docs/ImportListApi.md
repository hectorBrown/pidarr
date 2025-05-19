# \ImportListApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_importlist_action_name_post**](ImportListApi.md#api_v3_importlist_action_name_post) | **POST** /api/v3/importlist/action/{name} | 
[**api_v3_importlist_bulk_delete**](ImportListApi.md#api_v3_importlist_bulk_delete) | **DELETE** /api/v3/importlist/bulk | 
[**api_v3_importlist_bulk_put**](ImportListApi.md#api_v3_importlist_bulk_put) | **PUT** /api/v3/importlist/bulk | 
[**api_v3_importlist_get**](ImportListApi.md#api_v3_importlist_get) | **GET** /api/v3/importlist | 
[**api_v3_importlist_id_delete**](ImportListApi.md#api_v3_importlist_id_delete) | **DELETE** /api/v3/importlist/{id} | 
[**api_v3_importlist_id_get**](ImportListApi.md#api_v3_importlist_id_get) | **GET** /api/v3/importlist/{id} | 
[**api_v3_importlist_id_put**](ImportListApi.md#api_v3_importlist_id_put) | **PUT** /api/v3/importlist/{id} | 
[**api_v3_importlist_post**](ImportListApi.md#api_v3_importlist_post) | **POST** /api/v3/importlist | 
[**api_v3_importlist_schema_get**](ImportListApi.md#api_v3_importlist_schema_get) | **GET** /api/v3/importlist/schema | 
[**api_v3_importlist_test_post**](ImportListApi.md#api_v3_importlist_test_post) | **POST** /api/v3/importlist/test | 
[**api_v3_importlist_testall_post**](ImportListApi.md#api_v3_importlist_testall_post) | **POST** /api/v3/importlist/testall | 



## api_v3_importlist_action_name_post

> api_v3_importlist_action_name_post(name, import_list_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**import_list_resource** | Option<[**ImportListResource**](ImportListResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_importlist_bulk_delete

> api_v3_importlist_bulk_delete(import_list_bulk_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**import_list_bulk_resource** | Option<[**ImportListBulkResource**](ImportListBulkResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_importlist_bulk_put

> models::ImportListResource api_v3_importlist_bulk_put(import_list_bulk_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**import_list_bulk_resource** | Option<[**ImportListBulkResource**](ImportListBulkResource.md)> |  |  |

### Return type

[**models::ImportListResource**](ImportListResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_importlist_get

> Vec<models::ImportListResource> api_v3_importlist_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ImportListResource>**](ImportListResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_importlist_id_delete

> api_v3_importlist_id_delete(id)


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


## api_v3_importlist_id_get

> models::ImportListResource api_v3_importlist_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::ImportListResource**](ImportListResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_importlist_id_put

> models::ImportListResource api_v3_importlist_id_put(id, force_save, import_list_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**force_save** | Option<**bool**> |  |  |[default to false]
**import_list_resource** | Option<[**ImportListResource**](ImportListResource.md)> |  |  |

### Return type

[**models::ImportListResource**](ImportListResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_importlist_post

> models::ImportListResource api_v3_importlist_post(force_save, import_list_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_save** | Option<**bool**> |  |  |[default to false]
**import_list_resource** | Option<[**ImportListResource**](ImportListResource.md)> |  |  |

### Return type

[**models::ImportListResource**](ImportListResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_importlist_schema_get

> Vec<models::ImportListResource> api_v3_importlist_schema_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ImportListResource>**](ImportListResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_importlist_test_post

> api_v3_importlist_test_post(force_test, import_list_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_test** | Option<**bool**> |  |  |[default to false]
**import_list_resource** | Option<[**ImportListResource**](ImportListResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_importlist_testall_post

> api_v3_importlist_testall_post()


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

