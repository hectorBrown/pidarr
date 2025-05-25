# \NodesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v2_alter_worker_limit_post**](NodesApi.md#api_v2_alter_worker_limit_post) | **POST** /api/v2/alter-worker-limit | 
[**api_v2_cancel_worker_item_post**](NodesApi.md#api_v2_cancel_worker_item_post) | **POST** /api/v2/cancel-worker-item | 
[**api_v2_disconnect_node_post**](NodesApi.md#api_v2_disconnect_node_post) | **POST** /api/v2/disconnect-node | 
[**api_v2_download_plugins_get**](NodesApi.md#api_v2_download_plugins_get) | **GET** /api/v2/download-plugins | 
[**api_v2_file_download_post**](NodesApi.md#api_v2_file_download_post) | **POST** /api/v2/file/download | 
[**api_v2_file_upload_post**](NodesApi.md#api_v2_file_upload_post) | **POST** /api/v2/file/upload | 
[**api_v2_get_new_task_post**](NodesApi.md#api_v2_get_new_task_post) | **POST** /api/v2/get-new-task | 
[**api_v2_get_node_log_post**](NodesApi.md#api_v2_get_node_log_post) | **POST** /api/v2/get-node-log | 
[**api_v2_get_nodes_get**](NodesApi.md#api_v2_get_nodes_get) | **GET** /api/v2/get-nodes | 
[**api_v2_item_proc_end_post**](NodesApi.md#api_v2_item_proc_end_post) | **POST** /api/v2/item-proc-end | 
[**api_v2_kill_worker_post**](NodesApi.md#api_v2_kill_worker_post) | **POST** /api/v2/kill-worker | 
[**api_v2_log_job_report_post**](NodesApi.md#api_v2_log_job_report_post) | **POST** /api/v2/log-job-report | 
[**api_v2_poll_worker_limits_post**](NodesApi.md#api_v2_poll_worker_limits_post) | **POST** /api/v2/poll-worker-limits | 
[**api_v2_read_plugin_post**](NodesApi.md#api_v2_read_plugin_post) | **POST** /api/v2/read-plugin | 
[**api_v2_restart_node_post**](NodesApi.md#api_v2_restart_node_post) | **POST** /api/v2/restart-node | 
[**api_v2_sync_plugins_post**](NodesApi.md#api_v2_sync_plugins_post) | **POST** /api/v2/sync-plugins | 
[**api_v2_update_node_post**](NodesApi.md#api_v2_update_node_post) | **POST** /api/v2/update-node | 
[**api_v2_update_node_relay_post**](NodesApi.md#api_v2_update_node_relay_post) | **POST** /api/v2/update-node-relay | 



## api_v2_alter_worker_limit_post

> String api_v2_alter_worker_limit_post(body)


For changing the number of running workers of a specific type on a specific node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2AlterWorkerLimitPostRequest**](ApiV2AlterWorkerLimitPostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_cancel_worker_item_post

> String api_v2_cancel_worker_item_post(body)


For cancelling a running worker item on a specific node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2CancelWorkerItemPostRequest**](ApiV2CancelWorkerItemPostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_disconnect_node_post

> String api_v2_disconnect_node_post(body)


For forcefully disconnecting a node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2DisconnectNodePostRequest**](ApiV2DisconnectNodePostRequest.md)> |  |  |

### Return type

**String**

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


## api_v2_file_download_post

> serde_json::Value api_v2_file_download_post(body)


For downloading a file

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2FileDownloadPostRequest**](ApiV2FileDownloadPostRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_file_upload_post

> serde_json::Value api_v2_file_upload_post()


For uploading a file

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_get_new_task_post

> Vec<serde_json::Value> api_v2_get_new_task_post(body)


For a node to request a new task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2GetNewTaskPostRequest**](ApiV2GetNewTaskPostRequest.md)> |  |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_get_node_log_post

> String api_v2_get_node_log_post(body)


For getting the log of a node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2DisconnectNodePostRequest**](ApiV2DisconnectNodePostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_get_nodes_get

> std::collections::HashMap<String, serde_json::Value> api_v2_get_nodes_get()


For getting connected nodes information

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_item_proc_end_post

> String api_v2_item_proc_end_post(body)


For when a node completes processing an item

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2ItemProcEndPostRequest**](ApiV2ItemProcEndPostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_kill_worker_post

> String api_v2_kill_worker_post(body)


For killing a worker on a node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2KillWorkerPostRequest**](ApiV2KillWorkerPostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_log_job_report_post

> String api_v2_log_job_report_post(body)


For updating a job report

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2LogJobReportPostRequest**](ApiV2LogJobReportPostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_poll_worker_limits_post

> models::ApiV2PollWorkerLimitsPost200Response api_v2_poll_worker_limits_post(body)


For a node to get its worker limits and check if there's anything in the queue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2DisconnectNodePostRequest**](ApiV2DisconnectNodePostRequest.md)> |  |  |

### Return type

[**models::ApiV2PollWorkerLimitsPost200Response**](_api_v2_poll_worker_limits_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_read_plugin_post

> models::ApiV2ReadPluginPost200Response api_v2_read_plugin_post(body)


For a node to read a plugin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2ReadPluginPostRequest**](ApiV2ReadPluginPostRequest.md)> |  |  |

### Return type

[**models::ApiV2ReadPluginPost200Response**](_api_v2_read_plugin_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_restart_node_post

> String api_v2_restart_node_post(body)


For restarting a specific node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2RestartNodePostRequest**](ApiV2RestartNodePostRequest.md)> |  |  |

### Return type

**String**

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


## api_v2_update_node_post

> String api_v2_update_node_post(body)


For the UI to update a connected node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2UpdateNodePostRequest**](ApiV2UpdateNodePostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_update_node_relay_post

> String api_v2_update_node_relay_post(body)


For nodes to update the server with their status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2UpdateNodeRelayPostRequest**](ApiV2UpdateNodeRelayPostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

