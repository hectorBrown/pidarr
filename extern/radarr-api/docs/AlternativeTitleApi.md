# \AlternativeTitleApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_alttitle_get**](AlternativeTitleApi.md#api_v3_alttitle_get) | **GET** /api/v3/alttitle | 
[**api_v3_alttitle_id_get**](AlternativeTitleApi.md#api_v3_alttitle_id_get) | **GET** /api/v3/alttitle/{id} | 



## api_v3_alttitle_get

> Vec<models::AlternativeTitleResource> api_v3_alttitle_get(movie_id, movie_metadata_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | Option<**i32**> |  |  |
**movie_metadata_id** | Option<**i32**> |  |  |

### Return type

[**Vec<models::AlternativeTitleResource>**](AlternativeTitleResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_alttitle_id_get

> models::AlternativeTitleResource api_v3_alttitle_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::AlternativeTitleResource**](AlternativeTitleResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

