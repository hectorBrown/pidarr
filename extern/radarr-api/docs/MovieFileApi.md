# \MovieFileApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_moviefile_bulk_delete**](MovieFileApi.md#api_v3_moviefile_bulk_delete) | **DELETE** /api/v3/moviefile/bulk | 
[**api_v3_moviefile_bulk_put**](MovieFileApi.md#api_v3_moviefile_bulk_put) | **PUT** /api/v3/moviefile/bulk | 
[**api_v3_moviefile_editor_put**](MovieFileApi.md#api_v3_moviefile_editor_put) | **PUT** /api/v3/moviefile/editor | 
[**api_v3_moviefile_get**](MovieFileApi.md#api_v3_moviefile_get) | **GET** /api/v3/moviefile | 
[**api_v3_moviefile_id_delete**](MovieFileApi.md#api_v3_moviefile_id_delete) | **DELETE** /api/v3/moviefile/{id} | 
[**api_v3_moviefile_id_get**](MovieFileApi.md#api_v3_moviefile_id_get) | **GET** /api/v3/moviefile/{id} | 
[**api_v3_moviefile_id_put**](MovieFileApi.md#api_v3_moviefile_id_put) | **PUT** /api/v3/moviefile/{id} | 



## api_v3_moviefile_bulk_delete

> api_v3_moviefile_bulk_delete(movie_file_list_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_file_list_resource** | Option<[**MovieFileListResource**](MovieFileListResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_moviefile_bulk_put

> api_v3_moviefile_bulk_put(movie_file_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_file_resource** | Option<[**Vec<models::MovieFileResource>**](MovieFileResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_moviefile_editor_put

> api_v3_moviefile_editor_put(movie_file_list_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_file_list_resource** | Option<[**MovieFileListResource**](MovieFileListResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_moviefile_get

> Vec<models::MovieFileResource> api_v3_moviefile_get(movie_id, movie_file_ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**movie_file_ids** | Option<[**Vec<i32>**](i32.md)> |  |  |

### Return type

[**Vec<models::MovieFileResource>**](MovieFileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_moviefile_id_delete

> api_v3_moviefile_id_delete(id)


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


## api_v3_moviefile_id_get

> models::MovieFileResource api_v3_moviefile_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::MovieFileResource**](MovieFileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_moviefile_id_put

> models::MovieFileResource api_v3_moviefile_id_put(id, movie_file_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**movie_file_resource** | Option<[**MovieFileResource**](MovieFileResource.md)> |  |  |

### Return type

[**models::MovieFileResource**](MovieFileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

