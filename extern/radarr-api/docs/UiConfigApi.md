# \UiConfigApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_config_ui_get**](UiConfigApi.md#api_v3_config_ui_get) | **GET** /api/v3/config/ui | 
[**api_v3_config_ui_id_get**](UiConfigApi.md#api_v3_config_ui_id_get) | **GET** /api/v3/config/ui/{id} | 
[**api_v3_config_ui_id_put**](UiConfigApi.md#api_v3_config_ui_id_put) | **PUT** /api/v3/config/ui/{id} | 



## api_v3_config_ui_get

> models::UiConfigResource api_v3_config_ui_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UiConfigResource**](UiConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_config_ui_id_get

> models::UiConfigResource api_v3_config_ui_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::UiConfigResource**](UiConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_config_ui_id_put

> models::UiConfigResource api_v3_config_ui_id_put(id, ui_config_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**ui_config_resource** | Option<[**UiConfigResource**](UiConfigResource.md)> |  |  |

### Return type

[**models::UiConfigResource**](UiConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

