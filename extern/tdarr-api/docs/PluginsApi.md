# \PluginsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v2_copy_community_to_local_post**](PluginsApi.md#api_v2_copy_community_to_local_post) | **POST** /api/v2/copy-community-to-local | 
[**api_v2_create_plugin_post**](PluginsApi.md#api_v2_create_plugin_post) | **POST** /api/v2/create-plugin | 
[**api_v2_delete_plugin_post**](PluginsApi.md#api_v2_delete_plugin_post) | **POST** /api/v2/delete-plugin | 
[**api_v2_download_plugins_get**](PluginsApi.md#api_v2_download_plugins_get) | **GET** /api/v2/download-plugins | 
[**api_v2_read_plugin_text_post**](PluginsApi.md#api_v2_read_plugin_text_post) | **POST** /api/v2/read-plugin-text | 
[**api_v2_save_plugin_text_post**](PluginsApi.md#api_v2_save_plugin_text_post) | **POST** /api/v2/save-plugin-text | 
[**api_v2_search_flow_plugins_post**](PluginsApi.md#api_v2_search_flow_plugins_post) | **POST** /api/v2/search-flow-plugins | 
[**api_v2_search_flow_templates_post**](PluginsApi.md#api_v2_search_flow_templates_post) | **POST** /api/v2/search-flow-templates | 
[**api_v2_search_plugins_post**](PluginsApi.md#api_v2_search_plugins_post) | **POST** /api/v2/search-plugins | 
[**api_v2_sync_plugins_post**](PluginsApi.md#api_v2_sync_plugins_post) | **POST** /api/v2/sync-plugins | 
[**api_v2_update_plugins_post**](PluginsApi.md#api_v2_update_plugins_post) | **POST** /api/v2/update-plugins | 



## api_v2_copy_community_to_local_post

> Vec<serde_json::Value> api_v2_copy_community_to_local_post(body)


For copying a community plugin to local plugins

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2CopyCommunityToLocalPostRequest**](ApiV2CopyCommunityToLocalPostRequest.md)> |  |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_create_plugin_post

> bool api_v2_create_plugin_post(body)


For creating a basic classic plugin using the classic plugin creator

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2CreatePluginPostRequest**](ApiV2CreatePluginPostRequest.md)> |  |  |

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_delete_plugin_post

> Vec<serde_json::Value> api_v2_delete_plugin_post(body)


For deleting a plugin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2DeletePluginPostRequest**](ApiV2DeletePluginPostRequest.md)> |  |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_download_plugins_get

> String api_v2_download_plugins_get()


For nodes to download the latest plugins zip

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/zip

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_read_plugin_text_post

> Vec<serde_json::Value> api_v2_read_plugin_text_post(body)


For the classic plugin editor to read a plugin file

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2DeletePluginPostRequest**](ApiV2DeletePluginPostRequest.md)> |  |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_save_plugin_text_post

> Vec<serde_json::Value> api_v2_save_plugin_text_post(body)


For the classic plugin editor to save plugin text

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2SavePluginTextPostRequest**](ApiV2SavePluginTextPostRequest.md)> |  |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_search_flow_plugins_post

> Vec<serde_json::Value> api_v2_search_flow_plugins_post(body)


For searching flow plugins

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2SearchFlowPluginsPostRequest**](ApiV2SearchFlowPluginsPostRequest.md)> |  |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_search_flow_templates_post

> Vec<serde_json::Value> api_v2_search_flow_templates_post(body)


For searching flow templates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2SearchFlowPluginsPostRequest**](ApiV2SearchFlowPluginsPostRequest.md)> |  |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_search_plugins_post

> Vec<serde_json::Value> api_v2_search_plugins_post(body)


For searching classic plugins

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2SearchFlowPluginsPostRequest**](ApiV2SearchFlowPluginsPostRequest.md)> |  |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_sync_plugins_post

> String api_v2_sync_plugins_post()


For syncing plugins from server to all nodes

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_update_plugins_post

> String api_v2_update_plugins_post(body)


For requesting the server to update community plugins

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2UpdatePluginsPostRequest**](ApiV2UpdatePluginsPostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

