# \MovieApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_movie_get**](MovieApi.md#api_v3_movie_get) | **GET** /api/v3/movie | 
[**api_v3_movie_id_delete**](MovieApi.md#api_v3_movie_id_delete) | **DELETE** /api/v3/movie/{id} | 
[**api_v3_movie_id_get**](MovieApi.md#api_v3_movie_id_get) | **GET** /api/v3/movie/{id} | 
[**api_v3_movie_id_put**](MovieApi.md#api_v3_movie_id_put) | **PUT** /api/v3/movie/{id} | 
[**api_v3_movie_post**](MovieApi.md#api_v3_movie_post) | **POST** /api/v3/movie | 



## api_v3_movie_get

> Vec<models::MovieResource> api_v3_movie_get(tmdb_id, exclude_local_covers, language_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tmdb_id** | Option<**i32**> |  |  |
**exclude_local_covers** | Option<**bool**> |  |  |[default to false]
**language_id** | Option<**i32**> |  |  |

### Return type

[**Vec<models::MovieResource>**](MovieResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_movie_id_delete

> api_v3_movie_id_delete(id, delete_files, add_import_exclusion)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**delete_files** | Option<**bool**> |  |  |[default to false]
**add_import_exclusion** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_movie_id_get

> models::MovieResource api_v3_movie_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::MovieResource**](MovieResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_movie_id_put

> models::MovieResource api_v3_movie_id_put(id, move_files, movie_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**move_files** | Option<**bool**> |  |  |[default to false]
**movie_resource** | Option<[**MovieResource**](MovieResource.md)> |  |  |

### Return type

[**models::MovieResource**](MovieResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_movie_post

> models::MovieResource api_v3_movie_post(movie_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_resource** | Option<[**MovieResource**](MovieResource.md)> |  |  |

### Return type

[**models::MovieResource**](MovieResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

