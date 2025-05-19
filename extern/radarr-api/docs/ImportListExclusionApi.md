# \ImportListExclusionApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_exclusions_bulk_delete**](ImportListExclusionApi.md#api_v3_exclusions_bulk_delete) | **DELETE** /api/v3/exclusions/bulk | 
[**api_v3_exclusions_bulk_post**](ImportListExclusionApi.md#api_v3_exclusions_bulk_post) | **POST** /api/v3/exclusions/bulk | 
[**api_v3_exclusions_get**](ImportListExclusionApi.md#api_v3_exclusions_get) | **GET** /api/v3/exclusions | 
[**api_v3_exclusions_id_delete**](ImportListExclusionApi.md#api_v3_exclusions_id_delete) | **DELETE** /api/v3/exclusions/{id} | 
[**api_v3_exclusions_id_get**](ImportListExclusionApi.md#api_v3_exclusions_id_get) | **GET** /api/v3/exclusions/{id} | 
[**api_v3_exclusions_id_put**](ImportListExclusionApi.md#api_v3_exclusions_id_put) | **PUT** /api/v3/exclusions/{id} | 
[**api_v3_exclusions_paged_get**](ImportListExclusionApi.md#api_v3_exclusions_paged_get) | **GET** /api/v3/exclusions/paged | 
[**api_v3_exclusions_post**](ImportListExclusionApi.md#api_v3_exclusions_post) | **POST** /api/v3/exclusions | 



## api_v3_exclusions_bulk_delete

> api_v3_exclusions_bulk_delete(import_list_exclusion_bulk_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**import_list_exclusion_bulk_resource** | Option<[**ImportListExclusionBulkResource**](ImportListExclusionBulkResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_exclusions_bulk_post

> api_v3_exclusions_bulk_post(import_list_exclusion_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**import_list_exclusion_resource** | Option<[**Vec<models::ImportListExclusionResource>**](ImportListExclusionResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_exclusions_get

> Vec<models::ImportListExclusionResource> api_v3_exclusions_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ImportListExclusionResource>**](ImportListExclusionResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_exclusions_id_delete

> api_v3_exclusions_id_delete(id)


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


## api_v3_exclusions_id_get

> models::ImportListExclusionResource api_v3_exclusions_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::ImportListExclusionResource**](ImportListExclusionResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_exclusions_id_put

> models::ImportListExclusionResource api_v3_exclusions_id_put(id, import_list_exclusion_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**import_list_exclusion_resource** | Option<[**ImportListExclusionResource**](ImportListExclusionResource.md)> |  |  |

### Return type

[**models::ImportListExclusionResource**](ImportListExclusionResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_exclusions_paged_get

> models::ImportListExclusionResourcePagingResource api_v3_exclusions_paged_get(page, page_size, sort_key, sort_direction)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**page_size** | Option<**i32**> |  |  |[default to 10]
**sort_key** | Option<**String**> |  |  |
**sort_direction** | Option<[**SortDirection**](.md)> |  |  |

### Return type

[**models::ImportListExclusionResourcePagingResource**](ImportListExclusionResourcePagingResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_exclusions_post

> models::ImportListExclusionResource api_v3_exclusions_post(import_list_exclusion_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**import_list_exclusion_resource** | Option<[**ImportListExclusionResource**](ImportListExclusionResource.md)> |  |  |

### Return type

[**models::ImportListExclusionResource**](ImportListExclusionResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

