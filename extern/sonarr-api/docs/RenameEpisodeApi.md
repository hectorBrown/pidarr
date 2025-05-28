# \RenameEpisodeApi

All URIs are relative to *http://localhost:8989*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_rename_get**](RenameEpisodeApi.md#api_v3_rename_get) | **GET** /api/v3/rename | 



## api_v3_rename_get

> Vec<models::RenameEpisodeResource> api_v3_rename_get(series_id, season_number)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_id** | Option<**i32**> |  |  |
**season_number** | Option<**i32**> |  |  |

### Return type

[**Vec<models::RenameEpisodeResource>**](RenameEpisodeResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

