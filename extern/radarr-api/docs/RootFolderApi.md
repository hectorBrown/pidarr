# \RootFolderApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_rootfolder_get**](RootFolderApi.md#api_v3_rootfolder_get) | **GET** /api/v3/rootfolder | 
[**api_v3_rootfolder_id_delete**](RootFolderApi.md#api_v3_rootfolder_id_delete) | **DELETE** /api/v3/rootfolder/{id} | 
[**api_v3_rootfolder_id_get**](RootFolderApi.md#api_v3_rootfolder_id_get) | **GET** /api/v3/rootfolder/{id} | 
[**api_v3_rootfolder_post**](RootFolderApi.md#api_v3_rootfolder_post) | **POST** /api/v3/rootfolder | 



## api_v3_rootfolder_get

> Vec<models::RootFolderResource> api_v3_rootfolder_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::RootFolderResource>**](RootFolderResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_rootfolder_id_delete

> api_v3_rootfolder_id_delete(id)


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


## api_v3_rootfolder_id_get

> models::RootFolderResource api_v3_rootfolder_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::RootFolderResource**](RootFolderResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_rootfolder_post

> models::RootFolderResource api_v3_rootfolder_post(root_folder_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**root_folder_resource** | Option<[**RootFolderResource**](RootFolderResource.md)> |  |  |

### Return type

[**models::RootFolderResource**](RootFolderResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

