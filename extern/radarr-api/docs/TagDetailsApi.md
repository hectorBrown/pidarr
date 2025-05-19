# \TagDetailsApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_tag_detail_get**](TagDetailsApi.md#api_v3_tag_detail_get) | **GET** /api/v3/tag/detail | 
[**api_v3_tag_detail_id_get**](TagDetailsApi.md#api_v3_tag_detail_id_get) | **GET** /api/v3/tag/detail/{id} | 



## api_v3_tag_detail_get

> Vec<models::TagDetailsResource> api_v3_tag_detail_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::TagDetailsResource>**](TagDetailsResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_tag_detail_id_get

> models::TagDetailsResource api_v3_tag_detail_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::TagDetailsResource**](TagDetailsResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

