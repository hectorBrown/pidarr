# \EpisodeApi

All URIs are relative to *http://localhost:8989*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_episode_get**](EpisodeApi.md#api_v3_episode_get) | **GET** /api/v3/episode | 
[**api_v3_episode_id_get**](EpisodeApi.md#api_v3_episode_id_get) | **GET** /api/v3/episode/{id} | 
[**api_v3_episode_id_put**](EpisodeApi.md#api_v3_episode_id_put) | **PUT** /api/v3/episode/{id} | 
[**api_v3_episode_monitor_put**](EpisodeApi.md#api_v3_episode_monitor_put) | **PUT** /api/v3/episode/monitor | 



## api_v3_episode_get

> Vec<models::EpisodeResource> api_v3_episode_get(series_id, season_number, episode_ids, episode_file_id, include_series, include_episode_file, include_images)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_id** | Option<**i32**> |  |  |
**season_number** | Option<**i32**> |  |  |
**episode_ids** | Option<[**Vec<i32>**](i32.md)> |  |  |
**episode_file_id** | Option<**i32**> |  |  |
**include_series** | Option<**bool**> |  |  |[default to false]
**include_episode_file** | Option<**bool**> |  |  |[default to false]
**include_images** | Option<**bool**> |  |  |[default to false]

### Return type

[**Vec<models::EpisodeResource>**](EpisodeResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_episode_id_get

> models::EpisodeResource api_v3_episode_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::EpisodeResource**](EpisodeResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_episode_id_put

> models::EpisodeResource api_v3_episode_id_put(id, episode_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**episode_resource** | Option<[**EpisodeResource**](EpisodeResource.md)> |  |  |

### Return type

[**models::EpisodeResource**](EpisodeResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_episode_monitor_put

> api_v3_episode_monitor_put(include_images, episodes_monitored_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_images** | Option<**bool**> |  |  |[default to false]
**episodes_monitored_resource** | Option<[**EpisodesMonitoredResource**](EpisodesMonitoredResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

