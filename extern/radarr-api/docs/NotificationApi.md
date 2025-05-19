# \NotificationApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_notification_action_name_post**](NotificationApi.md#api_v3_notification_action_name_post) | **POST** /api/v3/notification/action/{name} | 
[**api_v3_notification_get**](NotificationApi.md#api_v3_notification_get) | **GET** /api/v3/notification | 
[**api_v3_notification_id_delete**](NotificationApi.md#api_v3_notification_id_delete) | **DELETE** /api/v3/notification/{id} | 
[**api_v3_notification_id_get**](NotificationApi.md#api_v3_notification_id_get) | **GET** /api/v3/notification/{id} | 
[**api_v3_notification_id_put**](NotificationApi.md#api_v3_notification_id_put) | **PUT** /api/v3/notification/{id} | 
[**api_v3_notification_post**](NotificationApi.md#api_v3_notification_post) | **POST** /api/v3/notification | 
[**api_v3_notification_schema_get**](NotificationApi.md#api_v3_notification_schema_get) | **GET** /api/v3/notification/schema | 
[**api_v3_notification_test_post**](NotificationApi.md#api_v3_notification_test_post) | **POST** /api/v3/notification/test | 
[**api_v3_notification_testall_post**](NotificationApi.md#api_v3_notification_testall_post) | **POST** /api/v3/notification/testall | 



## api_v3_notification_action_name_post

> api_v3_notification_action_name_post(name, notification_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**notification_resource** | Option<[**NotificationResource**](NotificationResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_notification_get

> Vec<models::NotificationResource> api_v3_notification_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::NotificationResource>**](NotificationResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_notification_id_delete

> api_v3_notification_id_delete(id)


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


## api_v3_notification_id_get

> models::NotificationResource api_v3_notification_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::NotificationResource**](NotificationResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_notification_id_put

> models::NotificationResource api_v3_notification_id_put(id, force_save, notification_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**force_save** | Option<**bool**> |  |  |[default to false]
**notification_resource** | Option<[**NotificationResource**](NotificationResource.md)> |  |  |

### Return type

[**models::NotificationResource**](NotificationResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_notification_post

> models::NotificationResource api_v3_notification_post(force_save, notification_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_save** | Option<**bool**> |  |  |[default to false]
**notification_resource** | Option<[**NotificationResource**](NotificationResource.md)> |  |  |

### Return type

[**models::NotificationResource**](NotificationResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_notification_schema_get

> Vec<models::NotificationResource> api_v3_notification_schema_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::NotificationResource>**](NotificationResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_notification_test_post

> api_v3_notification_test_post(force_test, notification_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_test** | Option<**bool**> |  |  |[default to false]
**notification_resource** | Option<[**NotificationResource**](NotificationResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_notification_testall_post

> api_v3_notification_testall_post()


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

