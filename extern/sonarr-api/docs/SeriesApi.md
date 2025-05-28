# \SeriesApi

All URIs are relative to *http://localhost:8989*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_series_get**](SeriesApi.md#api_v3_series_get) | **GET** /api/v3/series | 
[**api_v3_series_id_delete**](SeriesApi.md#api_v3_series_id_delete) | **DELETE** /api/v3/series/{id} | 
[**api_v3_series_id_get**](SeriesApi.md#api_v3_series_id_get) | **GET** /api/v3/series/{id} | 
[**api_v3_series_id_put**](SeriesApi.md#api_v3_series_id_put) | **PUT** /api/v3/series/{id} | 
[**api_v3_series_post**](SeriesApi.md#api_v3_series_post) | **POST** /api/v3/series | 



## api_v3_series_get

> Vec<models::SeriesResource> api_v3_series_get(tvdb_id, include_season_images)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tvdb_id** | Option<**i32**> |  |  |
**include_season_images** | Option<**bool**> |  |  |[default to false]

### Return type

[**Vec<models::SeriesResource>**](SeriesResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_series_id_delete

> api_v3_series_id_delete(id, delete_files, add_import_list_exclusion)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**delete_files** | Option<**bool**> |  |  |[default to false]
**add_import_list_exclusion** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_series_id_get

> models::SeriesResource api_v3_series_id_get(id, include_season_images)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**include_season_images** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::SeriesResource**](SeriesResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_series_id_put

> models::SeriesResource api_v3_series_id_put(id, move_files, series_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**move_files** | Option<**bool**> |  |  |[default to false]
**series_resource** | Option<[**SeriesResource**](SeriesResource.md)> |  |  |

### Return type

[**models::SeriesResource**](SeriesResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_series_post

> models::SeriesResource api_v3_series_post(series_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_resource** | Option<[**SeriesResource**](SeriesResource.md)> |  |  |

### Return type

[**models::SeriesResource**](SeriesResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

