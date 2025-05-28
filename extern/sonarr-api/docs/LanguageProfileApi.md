# \LanguageProfileApi

All URIs are relative to *http://localhost:8989*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_languageprofile_get**](LanguageProfileApi.md#api_v3_languageprofile_get) | **GET** /api/v3/languageprofile | 
[**api_v3_languageprofile_id_delete**](LanguageProfileApi.md#api_v3_languageprofile_id_delete) | **DELETE** /api/v3/languageprofile/{id} | 
[**api_v3_languageprofile_id_get**](LanguageProfileApi.md#api_v3_languageprofile_id_get) | **GET** /api/v3/languageprofile/{id} | 
[**api_v3_languageprofile_id_put**](LanguageProfileApi.md#api_v3_languageprofile_id_put) | **PUT** /api/v3/languageprofile/{id} | 
[**api_v3_languageprofile_post**](LanguageProfileApi.md#api_v3_languageprofile_post) | **POST** /api/v3/languageprofile | 



## api_v3_languageprofile_get

> Vec<models::LanguageProfileResource> api_v3_languageprofile_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::LanguageProfileResource>**](LanguageProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_languageprofile_id_delete

> api_v3_languageprofile_id_delete(id)


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


## api_v3_languageprofile_id_get

> models::LanguageProfileResource api_v3_languageprofile_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::LanguageProfileResource**](LanguageProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_languageprofile_id_put

> models::LanguageProfileResource api_v3_languageprofile_id_put(id, language_profile_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**language_profile_resource** | Option<[**LanguageProfileResource**](LanguageProfileResource.md)> |  |  |

### Return type

[**models::LanguageProfileResource**](LanguageProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_languageprofile_post

> models::LanguageProfileResource api_v3_languageprofile_post(language_profile_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language_profile_resource** | Option<[**LanguageProfileResource**](LanguageProfileResource.md)> |  |  |

### Return type

[**models::LanguageProfileResource**](LanguageProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

