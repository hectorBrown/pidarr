# \EpisodeFileApi

All URIs are relative to *http://localhost:8989*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_episodefile_bulk_delete**](EpisodeFileApi.md#api_v3_episodefile_bulk_delete) | **DELETE** /api/v3/episodefile/bulk | 
[**api_v3_episodefile_bulk_put**](EpisodeFileApi.md#api_v3_episodefile_bulk_put) | **PUT** /api/v3/episodefile/bulk | 
[**api_v3_episodefile_editor_put**](EpisodeFileApi.md#api_v3_episodefile_editor_put) | **PUT** /api/v3/episodefile/editor | 
[**api_v3_episodefile_get**](EpisodeFileApi.md#api_v3_episodefile_get) | **GET** /api/v3/episodefile | 
[**api_v3_episodefile_id_delete**](EpisodeFileApi.md#api_v3_episodefile_id_delete) | **DELETE** /api/v3/episodefile/{id} | 
[**api_v3_episodefile_id_get**](EpisodeFileApi.md#api_v3_episodefile_id_get) | **GET** /api/v3/episodefile/{id} | 
[**api_v3_episodefile_id_put**](EpisodeFileApi.md#api_v3_episodefile_id_put) | **PUT** /api/v3/episodefile/{id} | 



## api_v3_episodefile_bulk_delete

> api_v3_episodefile_bulk_delete(episode_file_list_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**episode_file_list_resource** | Option<[**EpisodeFileListResource**](EpisodeFileListResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_episodefile_bulk_put

> api_v3_episodefile_bulk_put(episode_file_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**episode_file_resource** | Option<[**Vec<models::EpisodeFileResource>**](EpisodeFileResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_episodefile_editor_put

> api_v3_episodefile_editor_put(episode_file_list_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**episode_file_list_resource** | Option<[**EpisodeFileListResource**](EpisodeFileListResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_episodefile_get

> Vec<models::EpisodeFileResource> api_v3_episodefile_get(series_id, episode_file_ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_id** | Option<**i32**> |  |  |
**episode_file_ids** | Option<[**Vec<i32>**](i32.md)> |  |  |

### Return type

[**Vec<models::EpisodeFileResource>**](EpisodeFileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_episodefile_id_delete

> api_v3_episodefile_id_delete(id)


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


## api_v3_episodefile_id_get

> models::EpisodeFileResource api_v3_episodefile_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::EpisodeFileResource**](EpisodeFileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_episodefile_id_put

> models::EpisodeFileResource api_v3_episodefile_id_put(id, episode_file_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**episode_file_resource** | Option<[**EpisodeFileResource**](EpisodeFileResource.md)> |  |  |

### Return type

[**models::EpisodeFileResource**](EpisodeFileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

