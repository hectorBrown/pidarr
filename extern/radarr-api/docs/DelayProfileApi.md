# \DelayProfileApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_delayprofile_get**](DelayProfileApi.md#api_v3_delayprofile_get) | **GET** /api/v3/delayprofile | 
[**api_v3_delayprofile_id_delete**](DelayProfileApi.md#api_v3_delayprofile_id_delete) | **DELETE** /api/v3/delayprofile/{id} | 
[**api_v3_delayprofile_id_get**](DelayProfileApi.md#api_v3_delayprofile_id_get) | **GET** /api/v3/delayprofile/{id} | 
[**api_v3_delayprofile_id_put**](DelayProfileApi.md#api_v3_delayprofile_id_put) | **PUT** /api/v3/delayprofile/{id} | 
[**api_v3_delayprofile_post**](DelayProfileApi.md#api_v3_delayprofile_post) | **POST** /api/v3/delayprofile | 
[**api_v3_delayprofile_reorder_id_put**](DelayProfileApi.md#api_v3_delayprofile_reorder_id_put) | **PUT** /api/v3/delayprofile/reorder/{id} | 



## api_v3_delayprofile_get

> Vec<models::DelayProfileResource> api_v3_delayprofile_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::DelayProfileResource>**](DelayProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_delayprofile_id_delete

> api_v3_delayprofile_id_delete(id)


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


## api_v3_delayprofile_id_get

> models::DelayProfileResource api_v3_delayprofile_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::DelayProfileResource**](DelayProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_delayprofile_id_put

> models::DelayProfileResource api_v3_delayprofile_id_put(id, delay_profile_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**delay_profile_resource** | Option<[**DelayProfileResource**](DelayProfileResource.md)> |  |  |

### Return type

[**models::DelayProfileResource**](DelayProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_delayprofile_post

> models::DelayProfileResource api_v3_delayprofile_post(delay_profile_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delay_profile_resource** | Option<[**DelayProfileResource**](DelayProfileResource.md)> |  |  |

### Return type

[**models::DelayProfileResource**](DelayProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_delayprofile_reorder_id_put

> Vec<models::DelayProfileResource> api_v3_delayprofile_reorder_id_put(id, after)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**after** | Option<**i32**> |  |  |

### Return type

[**Vec<models::DelayProfileResource>**](DelayProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

