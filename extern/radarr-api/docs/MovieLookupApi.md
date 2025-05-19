# \MovieLookupApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_movie_lookup_get**](MovieLookupApi.md#api_v3_movie_lookup_get) | **GET** /api/v3/movie/lookup | 
[**api_v3_movie_lookup_imdb_get**](MovieLookupApi.md#api_v3_movie_lookup_imdb_get) | **GET** /api/v3/movie/lookup/imdb | 
[**api_v3_movie_lookup_tmdb_get**](MovieLookupApi.md#api_v3_movie_lookup_tmdb_get) | **GET** /api/v3/movie/lookup/tmdb | 



## api_v3_movie_lookup_get

> Vec<models::MovieResource> api_v3_movie_lookup_get(term)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term** | Option<**String**> |  |  |

### Return type

[**Vec<models::MovieResource>**](MovieResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_movie_lookup_imdb_get

> models::MovieResource api_v3_movie_lookup_imdb_get(imdb_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**imdb_id** | Option<**String**> |  |  |

### Return type

[**models::MovieResource**](MovieResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_movie_lookup_tmdb_get

> models::MovieResource api_v3_movie_lookup_tmdb_get(tmdb_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tmdb_id** | Option<**i32**> |  |  |

### Return type

[**models::MovieResource**](MovieResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

