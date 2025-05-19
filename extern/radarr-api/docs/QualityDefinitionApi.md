# \QualityDefinitionApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_qualitydefinition_get**](QualityDefinitionApi.md#api_v3_qualitydefinition_get) | **GET** /api/v3/qualitydefinition | 
[**api_v3_qualitydefinition_id_get**](QualityDefinitionApi.md#api_v3_qualitydefinition_id_get) | **GET** /api/v3/qualitydefinition/{id} | 
[**api_v3_qualitydefinition_id_put**](QualityDefinitionApi.md#api_v3_qualitydefinition_id_put) | **PUT** /api/v3/qualitydefinition/{id} | 
[**api_v3_qualitydefinition_update_put**](QualityDefinitionApi.md#api_v3_qualitydefinition_update_put) | **PUT** /api/v3/qualitydefinition/update | 



## api_v3_qualitydefinition_get

> Vec<models::QualityDefinitionResource> api_v3_qualitydefinition_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::QualityDefinitionResource>**](QualityDefinitionResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_qualitydefinition_id_get

> models::QualityDefinitionResource api_v3_qualitydefinition_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::QualityDefinitionResource**](QualityDefinitionResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_qualitydefinition_id_put

> models::QualityDefinitionResource api_v3_qualitydefinition_id_put(id, quality_definition_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**quality_definition_resource** | Option<[**QualityDefinitionResource**](QualityDefinitionResource.md)> |  |  |

### Return type

[**models::QualityDefinitionResource**](QualityDefinitionResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_qualitydefinition_update_put

> api_v3_qualitydefinition_update_put(quality_definition_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quality_definition_resource** | Option<[**Vec<models::QualityDefinitionResource>**](QualityDefinitionResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

