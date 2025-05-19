# \MediaManagementConfigApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_config_mediamanagement_get**](MediaManagementConfigApi.md#api_v3_config_mediamanagement_get) | **GET** /api/v3/config/mediamanagement | 
[**api_v3_config_mediamanagement_id_get**](MediaManagementConfigApi.md#api_v3_config_mediamanagement_id_get) | **GET** /api/v3/config/mediamanagement/{id} | 
[**api_v3_config_mediamanagement_id_put**](MediaManagementConfigApi.md#api_v3_config_mediamanagement_id_put) | **PUT** /api/v3/config/mediamanagement/{id} | 



## api_v3_config_mediamanagement_get

> models::MediaManagementConfigResource api_v3_config_mediamanagement_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MediaManagementConfigResource**](MediaManagementConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_config_mediamanagement_id_get

> models::MediaManagementConfigResource api_v3_config_mediamanagement_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::MediaManagementConfigResource**](MediaManagementConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_config_mediamanagement_id_put

> models::MediaManagementConfigResource api_v3_config_mediamanagement_id_put(id, media_management_config_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**media_management_config_resource** | Option<[**MediaManagementConfigResource**](MediaManagementConfigResource.md)> |  |  |

### Return type

[**models::MediaManagementConfigResource**](MediaManagementConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

