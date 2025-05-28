# \HistoryApi

All URIs are relative to *http://localhost:8989*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_history_failed_id_post**](HistoryApi.md#api_v3_history_failed_id_post) | **POST** /api/v3/history/failed/{id} | 
[**api_v3_history_get**](HistoryApi.md#api_v3_history_get) | **GET** /api/v3/history | 
[**api_v3_history_series_get**](HistoryApi.md#api_v3_history_series_get) | **GET** /api/v3/history/series | 
[**api_v3_history_since_get**](HistoryApi.md#api_v3_history_since_get) | **GET** /api/v3/history/since | 



## api_v3_history_failed_id_post

> api_v3_history_failed_id_post(id)


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


## api_v3_history_get

> models::HistoryResourcePagingResource api_v3_history_get(page, page_size, sort_key, sort_direction, include_series, include_episode, event_type, episode_id, download_id, series_ids, languages, quality)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**page_size** | Option<**i32**> |  |  |[default to 10]
**sort_key** | Option<**String**> |  |  |
**sort_direction** | Option<[**SortDirection**](.md)> |  |  |
**include_series** | Option<**bool**> |  |  |
**include_episode** | Option<**bool**> |  |  |
**event_type** | Option<[**Vec<i32>**](i32.md)> |  |  |
**episode_id** | Option<**i32**> |  |  |
**download_id** | Option<**String**> |  |  |
**series_ids** | Option<[**Vec<i32>**](i32.md)> |  |  |
**languages** | Option<[**Vec<i32>**](i32.md)> |  |  |
**quality** | Option<[**Vec<i32>**](i32.md)> |  |  |

### Return type

[**models::HistoryResourcePagingResource**](HistoryResourcePagingResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_history_series_get

> Vec<models::HistoryResource> api_v3_history_series_get(series_id, season_number, event_type, include_series, include_episode)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_id** | Option<**i32**> |  |  |
**season_number** | Option<**i32**> |  |  |
**event_type** | Option<[**EpisodeHistoryEventType**](.md)> |  |  |
**include_series** | Option<**bool**> |  |  |[default to false]
**include_episode** | Option<**bool**> |  |  |[default to false]

### Return type

[**Vec<models::HistoryResource>**](HistoryResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_history_since_get

> Vec<models::HistoryResource> api_v3_history_since_get(date, event_type, include_series, include_episode)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date** | Option<**String**> |  |  |
**event_type** | Option<[**EpisodeHistoryEventType**](.md)> |  |  |
**include_series** | Option<**bool**> |  |  |[default to false]
**include_episode** | Option<**bool**> |  |  |[default to false]

### Return type

[**Vec<models::HistoryResource>**](HistoryResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

