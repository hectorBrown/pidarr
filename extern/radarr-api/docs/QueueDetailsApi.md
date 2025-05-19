# \QueueDetailsApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_queue_details_get**](QueueDetailsApi.md#api_v3_queue_details_get) | **GET** /api/v3/queue/details | 



## api_v3_queue_details_get

> Vec<models::QueueResource> api_v3_queue_details_get(movie_id, include_movie)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | Option<**i32**> |  |  |
**include_movie** | Option<**bool**> |  |  |[default to false]

### Return type

[**Vec<models::QueueResource>**](QueueResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

