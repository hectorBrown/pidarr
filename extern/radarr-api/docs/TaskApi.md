# \TaskApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_system_task_get**](TaskApi.md#api_v3_system_task_get) | **GET** /api/v3/system/task | 
[**api_v3_system_task_id_get**](TaskApi.md#api_v3_system_task_id_get) | **GET** /api/v3/system/task/{id} | 



## api_v3_system_task_get

> Vec<models::TaskResource> api_v3_system_task_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::TaskResource>**](TaskResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_system_task_id_get

> models::TaskResource api_v3_system_task_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::TaskResource**](TaskResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

