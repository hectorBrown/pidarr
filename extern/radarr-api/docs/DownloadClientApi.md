# \DownloadClientApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_downloadclient_action_name_post**](DownloadClientApi.md#api_v3_downloadclient_action_name_post) | **POST** /api/v3/downloadclient/action/{name} | 
[**api_v3_downloadclient_bulk_delete**](DownloadClientApi.md#api_v3_downloadclient_bulk_delete) | **DELETE** /api/v3/downloadclient/bulk | 
[**api_v3_downloadclient_bulk_put**](DownloadClientApi.md#api_v3_downloadclient_bulk_put) | **PUT** /api/v3/downloadclient/bulk | 
[**api_v3_downloadclient_get**](DownloadClientApi.md#api_v3_downloadclient_get) | **GET** /api/v3/downloadclient | 
[**api_v3_downloadclient_id_delete**](DownloadClientApi.md#api_v3_downloadclient_id_delete) | **DELETE** /api/v3/downloadclient/{id} | 
[**api_v3_downloadclient_id_get**](DownloadClientApi.md#api_v3_downloadclient_id_get) | **GET** /api/v3/downloadclient/{id} | 
[**api_v3_downloadclient_id_put**](DownloadClientApi.md#api_v3_downloadclient_id_put) | **PUT** /api/v3/downloadclient/{id} | 
[**api_v3_downloadclient_post**](DownloadClientApi.md#api_v3_downloadclient_post) | **POST** /api/v3/downloadclient | 
[**api_v3_downloadclient_schema_get**](DownloadClientApi.md#api_v3_downloadclient_schema_get) | **GET** /api/v3/downloadclient/schema | 
[**api_v3_downloadclient_test_post**](DownloadClientApi.md#api_v3_downloadclient_test_post) | **POST** /api/v3/downloadclient/test | 
[**api_v3_downloadclient_testall_post**](DownloadClientApi.md#api_v3_downloadclient_testall_post) | **POST** /api/v3/downloadclient/testall | 



## api_v3_downloadclient_action_name_post

> api_v3_downloadclient_action_name_post(name, download_client_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**download_client_resource** | Option<[**DownloadClientResource**](DownloadClientResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_downloadclient_bulk_delete

> api_v3_downloadclient_bulk_delete(download_client_bulk_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**download_client_bulk_resource** | Option<[**DownloadClientBulkResource**](DownloadClientBulkResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_downloadclient_bulk_put

> models::DownloadClientResource api_v3_downloadclient_bulk_put(download_client_bulk_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**download_client_bulk_resource** | Option<[**DownloadClientBulkResource**](DownloadClientBulkResource.md)> |  |  |

### Return type

[**models::DownloadClientResource**](DownloadClientResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_downloadclient_get

> Vec<models::DownloadClientResource> api_v3_downloadclient_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::DownloadClientResource>**](DownloadClientResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_downloadclient_id_delete

> api_v3_downloadclient_id_delete(id)


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


## api_v3_downloadclient_id_get

> models::DownloadClientResource api_v3_downloadclient_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::DownloadClientResource**](DownloadClientResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_downloadclient_id_put

> models::DownloadClientResource api_v3_downloadclient_id_put(id, force_save, download_client_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**force_save** | Option<**bool**> |  |  |[default to false]
**download_client_resource** | Option<[**DownloadClientResource**](DownloadClientResource.md)> |  |  |

### Return type

[**models::DownloadClientResource**](DownloadClientResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_downloadclient_post

> models::DownloadClientResource api_v3_downloadclient_post(force_save, download_client_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_save** | Option<**bool**> |  |  |[default to false]
**download_client_resource** | Option<[**DownloadClientResource**](DownloadClientResource.md)> |  |  |

### Return type

[**models::DownloadClientResource**](DownloadClientResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_downloadclient_schema_get

> Vec<models::DownloadClientResource> api_v3_downloadclient_schema_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::DownloadClientResource>**](DownloadClientResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_downloadclient_test_post

> api_v3_downloadclient_test_post(force_test, download_client_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_test** | Option<**bool**> |  |  |[default to false]
**download_client_resource** | Option<[**DownloadClientResource**](DownloadClientResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_downloadclient_testall_post

> api_v3_downloadclient_testall_post()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

