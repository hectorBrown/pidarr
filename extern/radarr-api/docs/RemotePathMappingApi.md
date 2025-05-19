# \RemotePathMappingApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_remotepathmapping_get**](RemotePathMappingApi.md#api_v3_remotepathmapping_get) | **GET** /api/v3/remotepathmapping | 
[**api_v3_remotepathmapping_id_delete**](RemotePathMappingApi.md#api_v3_remotepathmapping_id_delete) | **DELETE** /api/v3/remotepathmapping/{id} | 
[**api_v3_remotepathmapping_id_get**](RemotePathMappingApi.md#api_v3_remotepathmapping_id_get) | **GET** /api/v3/remotepathmapping/{id} | 
[**api_v3_remotepathmapping_id_put**](RemotePathMappingApi.md#api_v3_remotepathmapping_id_put) | **PUT** /api/v3/remotepathmapping/{id} | 
[**api_v3_remotepathmapping_post**](RemotePathMappingApi.md#api_v3_remotepathmapping_post) | **POST** /api/v3/remotepathmapping | 



## api_v3_remotepathmapping_get

> Vec<models::RemotePathMappingResource> api_v3_remotepathmapping_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::RemotePathMappingResource>**](RemotePathMappingResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_remotepathmapping_id_delete

> api_v3_remotepathmapping_id_delete(id)


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


## api_v3_remotepathmapping_id_get

> models::RemotePathMappingResource api_v3_remotepathmapping_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::RemotePathMappingResource**](RemotePathMappingResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_remotepathmapping_id_put

> models::RemotePathMappingResource api_v3_remotepathmapping_id_put(id, remote_path_mapping_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**remote_path_mapping_resource** | Option<[**RemotePathMappingResource**](RemotePathMappingResource.md)> |  |  |

### Return type

[**models::RemotePathMappingResource**](RemotePathMappingResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_remotepathmapping_post

> models::RemotePathMappingResource api_v3_remotepathmapping_post(remote_path_mapping_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**remote_path_mapping_resource** | Option<[**RemotePathMappingResource**](RemotePathMappingResource.md)> |  |  |

### Return type

[**models::RemotePathMappingResource**](RemotePathMappingResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

