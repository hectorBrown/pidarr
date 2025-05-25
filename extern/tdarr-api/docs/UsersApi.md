# \UsersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v2_admin_register_post**](UsersApi.md#api_v2_admin_register_post) | **POST** /api/v2/admin/register | 
[**api_v2_admin_reset_password_post**](UsersApi.md#api_v2_admin_reset_password_post) | **POST** /api/v2/admin/reset-password | 
[**api_v2_auth_logout_post**](UsersApi.md#api_v2_auth_logout_post) | **POST** /api/v2/auth/logout | 
[**api_v2_auth_reset_password_post**](UsersApi.md#api_v2_auth_reset_password_post) | **POST** /api/v2/auth/reset-password | 
[**api_v2_auth_verify_token_get**](UsersApi.md#api_v2_auth_verify_token_get) | **GET** /api/v2/auth/verify-token | 
[**api_v2_public_auth_login_post**](UsersApi.md#api_v2_public_auth_login_post) | **POST** /api/v2/public/auth/login | 
[**api_v2_public_auth_register_post**](UsersApi.md#api_v2_public_auth_register_post) | **POST** /api/v2/public/auth/register | 



## api_v2_admin_register_post

> models::ApiV2AdminRegisterPost201Response api_v2_admin_register_post(body)


For admin registering a new user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2AdminRegisterPostRequest**](ApiV2AdminRegisterPostRequest.md)> |  |  |

### Return type

[**models::ApiV2AdminRegisterPost201Response**](_api_v2_admin_register_post_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_admin_reset_password_post

> models::ApiV2AdminRegisterPost201Response api_v2_admin_reset_password_post(body)


For admin resetting a user password

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2AdminResetPasswordPostRequest**](ApiV2AdminResetPasswordPostRequest.md)> |  |  |

### Return type

[**models::ApiV2AdminRegisterPost201Response**](_api_v2_admin_register_post_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_auth_logout_post

> String api_v2_auth_logout_post()


For logging out a user

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


## api_v2_auth_reset_password_post

> models::ApiV2AuthResetPasswordPost201Response api_v2_auth_reset_password_post(body)


For resetting a user password

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2AuthResetPasswordPostRequest**](ApiV2AuthResetPasswordPostRequest.md)> |  |  |

### Return type

[**models::ApiV2AuthResetPasswordPost201Response**](_api_v2_auth_reset_password_post_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_auth_verify_token_get

> models::ApiV2AuthVerifyTokenGet200Response api_v2_auth_verify_token_get()


For verifying a user token

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiV2AuthVerifyTokenGet200Response**](_api_v2_auth_verify_token_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_public_auth_login_post

> models::ApiV2AuthResetPasswordPost201Response api_v2_public_auth_login_post(body)


For logging in a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2PublicAuthLoginPostRequest**](ApiV2PublicAuthLoginPostRequest.md)> |  |  |

### Return type

[**models::ApiV2AuthResetPasswordPost201Response**](_api_v2_auth_reset_password_post_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_public_auth_register_post

> models::ApiV2AuthResetPasswordPost201Response api_v2_public_auth_register_post(body)


For registering a new user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2PublicAuthLoginPostRequest**](ApiV2PublicAuthLoginPostRequest.md)> |  |  |

### Return type

[**models::ApiV2AuthResetPasswordPost201Response**](_api_v2_auth_reset_password_post_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

