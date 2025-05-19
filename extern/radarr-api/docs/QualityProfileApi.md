# \QualityProfileApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_qualityprofile_get**](QualityProfileApi.md#api_v3_qualityprofile_get) | **GET** /api/v3/qualityprofile | 
[**api_v3_qualityprofile_id_delete**](QualityProfileApi.md#api_v3_qualityprofile_id_delete) | **DELETE** /api/v3/qualityprofile/{id} | 
[**api_v3_qualityprofile_id_get**](QualityProfileApi.md#api_v3_qualityprofile_id_get) | **GET** /api/v3/qualityprofile/{id} | 
[**api_v3_qualityprofile_id_put**](QualityProfileApi.md#api_v3_qualityprofile_id_put) | **PUT** /api/v3/qualityprofile/{id} | 
[**api_v3_qualityprofile_post**](QualityProfileApi.md#api_v3_qualityprofile_post) | **POST** /api/v3/qualityprofile | 



## api_v3_qualityprofile_get

> Vec<models::QualityProfileResource> api_v3_qualityprofile_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::QualityProfileResource>**](QualityProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_qualityprofile_id_delete

> api_v3_qualityprofile_id_delete(id)


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


## api_v3_qualityprofile_id_get

> models::QualityProfileResource api_v3_qualityprofile_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::QualityProfileResource**](QualityProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_qualityprofile_id_put

> models::QualityProfileResource api_v3_qualityprofile_id_put(id, quality_profile_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**quality_profile_resource** | Option<[**QualityProfileResource**](QualityProfileResource.md)> |  |  |

### Return type

[**models::QualityProfileResource**](QualityProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_qualityprofile_post

> models::QualityProfileResource api_v3_qualityprofile_post(quality_profile_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quality_profile_resource** | Option<[**QualityProfileResource**](QualityProfileResource.md)> |  |  |

### Return type

[**models::QualityProfileResource**](QualityProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

