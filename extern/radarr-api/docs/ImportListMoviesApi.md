# \ImportListMoviesApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_importlist_movie_get**](ImportListMoviesApi.md#api_v3_importlist_movie_get) | **GET** /api/v3/importlist/movie | 
[**api_v3_importlist_movie_post**](ImportListMoviesApi.md#api_v3_importlist_movie_post) | **POST** /api/v3/importlist/movie | 



## api_v3_importlist_movie_get

> api_v3_importlist_movie_get(include_recommendations, include_trending, include_popular)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_recommendations** | Option<**bool**> |  |  |[default to false]
**include_trending** | Option<**bool**> |  |  |[default to false]
**include_popular** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_importlist_movie_post

> api_v3_importlist_movie_post(movie_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_resource** | Option<[**Vec<models::MovieResource>**](MovieResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

