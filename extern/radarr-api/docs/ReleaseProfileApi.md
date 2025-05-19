# \ReleaseProfileApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_releaseprofile_get**](ReleaseProfileApi.md#api_v3_releaseprofile_get) | **GET** /api/v3/releaseprofile | 
[**api_v3_releaseprofile_id_delete**](ReleaseProfileApi.md#api_v3_releaseprofile_id_delete) | **DELETE** /api/v3/releaseprofile/{id} | 
[**api_v3_releaseprofile_id_get**](ReleaseProfileApi.md#api_v3_releaseprofile_id_get) | **GET** /api/v3/releaseprofile/{id} | 
[**api_v3_releaseprofile_id_put**](ReleaseProfileApi.md#api_v3_releaseprofile_id_put) | **PUT** /api/v3/releaseprofile/{id} | 
[**api_v3_releaseprofile_post**](ReleaseProfileApi.md#api_v3_releaseprofile_post) | **POST** /api/v3/releaseprofile | 



## api_v3_releaseprofile_get

> Vec<models::ReleaseProfileResource> api_v3_releaseprofile_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ReleaseProfileResource>**](ReleaseProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_releaseprofile_id_delete

> api_v3_releaseprofile_id_delete(id)


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


## api_v3_releaseprofile_id_get

> models::ReleaseProfileResource api_v3_releaseprofile_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::ReleaseProfileResource**](ReleaseProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_releaseprofile_id_put

> models::ReleaseProfileResource api_v3_releaseprofile_id_put(id, release_profile_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**release_profile_resource** | Option<[**ReleaseProfileResource**](ReleaseProfileResource.md)> |  |  |

### Return type

[**models::ReleaseProfileResource**](ReleaseProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_releaseprofile_post

> models::ReleaseProfileResource api_v3_releaseprofile_post(release_profile_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**release_profile_resource** | Option<[**ReleaseProfileResource**](ReleaseProfileResource.md)> |  |  |

### Return type

[**models::ReleaseProfileResource**](ReleaseProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

