# \ManualImportApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_manualimport_get**](ManualImportApi.md#api_v3_manualimport_get) | **GET** /api/v3/manualimport | 
[**api_v3_manualimport_post**](ManualImportApi.md#api_v3_manualimport_post) | **POST** /api/v3/manualimport | 



## api_v3_manualimport_get

> Vec<models::ManualImportResource> api_v3_manualimport_get(folder, download_id, movie_id, filter_existing_files)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder** | Option<**String**> |  |  |
**download_id** | Option<**String**> |  |  |
**movie_id** | Option<**i32**> |  |  |
**filter_existing_files** | Option<**bool**> |  |  |[default to true]

### Return type

[**Vec<models::ManualImportResource>**](ManualImportResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_manualimport_post

> api_v3_manualimport_post(manual_import_reprocess_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**manual_import_reprocess_resource** | Option<[**Vec<models::ManualImportReprocessResource>**](ManualImportReprocessResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

