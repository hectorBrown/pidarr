# \StatsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v2_stats_get_pies_post**](StatsApi.md#api_v2_stats_get_pies_post) | **POST** /api/v2/stats/get-pies | 
[**api_v2_stats_get_res_hist_post**](StatsApi.md#api_v2_stats_get_res_hist_post) | **POST** /api/v2/stats/get-res-hist | 
[**api_v2_stats_get_running_worker_hist_post**](StatsApi.md#api_v2_stats_get_running_worker_hist_post) | **POST** /api/v2/stats/get-running-worker-hist | 
[**api_v2_stats_get_space_saved_post**](StatsApi.md#api_v2_stats_get_space_saved_post) | **POST** /api/v2/stats/get-space-saved | 
[**api_v2_stats_get_streams_post**](StatsApi.md#api_v2_stats_get_streams_post) | **POST** /api/v2/stats/get-streams | 
[**api_v2_stats_get_worker_verdict_hist_post**](StatsApi.md#api_v2_stats_get_worker_verdict_hist_post) | **POST** /api/v2/stats/get-worker-verdict-hist | 
[**api_v2_stats_space_saved_add_post**](StatsApi.md#api_v2_stats_space_saved_add_post) | **POST** /api/v2/stats/space-saved-add | 



## api_v2_stats_get_pies_post

> models::ApiV2StatsGetPiesPost200Response api_v2_stats_get_pies_post(body)


For getting all or library pie stats

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2StatsGetPiesPostRequest**](ApiV2StatsGetPiesPostRequest.md)> |  |  |

### Return type

[**models::ApiV2StatsGetPiesPost200Response**](_api_v2_stats_get_pies_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_stats_get_res_hist_post

> models::ApiV2StatsGetResHistPost200Response api_v2_stats_get_res_hist_post(body)


For getting server resource history

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2StatsGetResHistPostRequest**](ApiV2StatsGetResHistPostRequest.md)> |  |  |

### Return type

[**models::ApiV2StatsGetResHistPost200Response**](_api_v2_stats_get_res_hist_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_stats_get_running_worker_hist_post

> models::ApiV2StatsGetRunningWorkerHistPost200Response api_v2_stats_get_running_worker_hist_post(body)


For getting running worker history

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2StatsGetResHistPostRequest**](ApiV2StatsGetResHistPostRequest.md)> |  |  |

### Return type

[**models::ApiV2StatsGetRunningWorkerHistPost200Response**](_api_v2_stats_get_running_worker_hist_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_stats_get_space_saved_post

> models::ApiV2StatsGetSpaceSavedPost200Response api_v2_stats_get_space_saved_post(body)


For getting space saved history

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2StatsGetSpaceSavedPostRequest**](ApiV2StatsGetSpaceSavedPostRequest.md)> |  |  |

### Return type

[**models::ApiV2StatsGetSpaceSavedPost200Response**](_api_v2_stats_get_space_saved_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_stats_get_streams_post

> models::ApiV2StatsGetStreamsPost200Response api_v2_stats_get_streams_post(body)


For getting stream stats info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2StatsGetPiesPostRequest**](ApiV2StatsGetPiesPostRequest.md)> |  |  |

### Return type

[**models::ApiV2StatsGetStreamsPost200Response**](_api_v2_stats_get_streams_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_stats_get_worker_verdict_hist_post

> models::ApiV2StatsGetWorkerVerdictHistPost200Response api_v2_stats_get_worker_verdict_hist_post(body)


For getting worker verdict history

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2StatsGetWorkerVerdictHistPostRequest**](ApiV2StatsGetWorkerVerdictHistPostRequest.md)> |  |  |

### Return type

[**models::ApiV2StatsGetWorkerVerdictHistPost200Response**](_api_v2_stats_get_worker_verdict_hist_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_stats_space_saved_add_post

> String api_v2_stats_space_saved_add_post(body)


For adding space saved record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2StatsSpaceSavedAddPostRequest**](ApiV2StatsSpaceSavedAddPostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

