# \LibrariesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v2_add_audio_codec_exclude_post**](LibrariesApi.md#api_v2_add_audio_codec_exclude_post) | **POST** /api/v2/add-audio-codec-exclude | 
[**api_v2_add_plugin_include_post**](LibrariesApi.md#api_v2_add_plugin_include_post) | **POST** /api/v2/add-plugin-include | 
[**api_v2_add_video_codec_exclude_post**](LibrariesApi.md#api_v2_add_video_codec_exclude_post) | **POST** /api/v2/add-video-codec-exclude | 
[**api_v2_get_filescanner_status_post**](LibrariesApi.md#api_v2_get_filescanner_status_post) | **POST** /api/v2/get-filescanner-status | 
[**api_v2_get_subdirectories_post**](LibrariesApi.md#api_v2_get_subdirectories_post) | **POST** /api/v2/get-subdirectories | 
[**api_v2_kill_file_scanner_post**](LibrariesApi.md#api_v2_kill_file_scanner_post) | **POST** /api/v2/kill-file-scanner | 
[**api_v2_remove_audio_codec_exclude_post**](LibrariesApi.md#api_v2_remove_audio_codec_exclude_post) | **POST** /api/v2/remove-audio-codec-exclude | 
[**api_v2_remove_library_files_post**](LibrariesApi.md#api_v2_remove_library_files_post) | **POST** /api/v2/remove-library-files | 
[**api_v2_remove_plugin_include_post**](LibrariesApi.md#api_v2_remove_plugin_include_post) | **POST** /api/v2/remove-plugin-include | 
[**api_v2_remove_video_codec_exclude_post**](LibrariesApi.md#api_v2_remove_video_codec_exclude_post) | **POST** /api/v2/remove-video-codec-exclude | 
[**api_v2_scan_files_post**](LibrariesApi.md#api_v2_scan_files_post) | **POST** /api/v2/scan-files | 
[**api_v2_toggle_folder_watch_post**](LibrariesApi.md#api_v2_toggle_folder_watch_post) | **POST** /api/v2/toggle-folder-watch | 
[**api_v2_toggle_schedule_post**](LibrariesApi.md#api_v2_toggle_schedule_post) | **POST** /api/v2/toggle-schedule | 
[**api_v2_update_audio_codec_exclude_post**](LibrariesApi.md#api_v2_update_audio_codec_exclude_post) | **POST** /api/v2/update-audio-codec-exclude | 
[**api_v2_update_plugin_include_post**](LibrariesApi.md#api_v2_update_plugin_include_post) | **POST** /api/v2/update-plugin-include | 
[**api_v2_update_schedule_block_post**](LibrariesApi.md#api_v2_update_schedule_block_post) | **POST** /api/v2/update-schedule-block | 
[**api_v2_update_video_codec_exclude_post**](LibrariesApi.md#api_v2_update_video_codec_exclude_post) | **POST** /api/v2/update-video-codec-exclude | 
[**api_v2_verify_folder_exists_post**](LibrariesApi.md#api_v2_verify_folder_exists_post) | **POST** /api/v2/verify-folder-exists | 
[**api_v2_verify_plugin_post**](LibrariesApi.md#api_v2_verify_plugin_post) | **POST** /api/v2/verify-plugin | 



## api_v2_add_audio_codec_exclude_post

> String api_v2_add_audio_codec_exclude_post(body)


For adding an audio codec to be excluded/included in basic audio transcoding settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2AddAudioCodecExcludePostRequest**](ApiV2AddAudioCodecExcludePostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_add_plugin_include_post

> String api_v2_add_plugin_include_post(body)


For adding a plugin to a classic plugin stack

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2AddPluginIncludePostRequest**](ApiV2AddPluginIncludePostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_add_video_codec_exclude_post

> String api_v2_add_video_codec_exclude_post(body)


For adding an video codec to be excluded/included in basic video transcoding settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2AddAudioCodecExcludePostRequest**](ApiV2AddAudioCodecExcludePostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_get_filescanner_status_post

> String api_v2_get_filescanner_status_post(body)


For getting the status of a file scanner in progress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2GetFilescannerStatusPostRequest**](ApiV2GetFilescannerStatusPostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_get_subdirectories_post

> models::ApiV2GetSubdirectoriesPost200Response api_v2_get_subdirectories_post(body)


For getting subdirectories of a folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2GetSubdirectoriesPostRequest**](ApiV2GetSubdirectoriesPostRequest.md)> |  |  |

### Return type

[**models::ApiV2GetSubdirectoriesPost200Response**](_api_v2_get_subdirectories_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_kill_file_scanner_post

> String api_v2_kill_file_scanner_post(body)


For killing a file scanner in progress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2GetFilescannerStatusPostRequest**](ApiV2GetFilescannerStatusPostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_remove_audio_codec_exclude_post

> String api_v2_remove_audio_codec_exclude_post(body)


For removing an audio codec to be excluded/included in basic audio transcoding settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2AddAudioCodecExcludePostRequest**](ApiV2AddAudioCodecExcludePostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_remove_library_files_post

> String api_v2_remove_library_files_post(body)


For removing all files from a Tdarr library DB, files on disk aren't removed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2RemoveLibraryFilesPostRequest**](ApiV2RemoveLibraryFilesPostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_remove_plugin_include_post

> String api_v2_remove_plugin_include_post(body)


For removing a plugin from a classic plugin stack

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2AddAudioCodecExcludePostRequest**](ApiV2AddAudioCodecExcludePostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_remove_video_codec_exclude_post

> String api_v2_remove_video_codec_exclude_post(body)


For removing an video codec to be excluded/included in basic video transcoding settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2AddAudioCodecExcludePostRequest**](ApiV2AddAudioCodecExcludePostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_scan_files_post

> String api_v2_scan_files_post(body)


   For running a scanFresh, scanFindNew or scanFolderWatcher on a library   scanFresh & scanFindNew require a single string directory path   scanFolderWatcher requires an array of file paths to scanned   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2ScanFilesPostRequest**](ApiV2ScanFilesPostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_toggle_folder_watch_post

> String api_v2_toggle_folder_watch_post(body)


For enabling/disabling folder watching on a library

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2ToggleFolderWatchPostRequest**](ApiV2ToggleFolderWatchPostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_toggle_schedule_post

> String api_v2_toggle_schedule_post(body)


For updating the schedule of a library

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2ToggleSchedulePostRequest**](ApiV2ToggleSchedulePostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_update_audio_codec_exclude_post

> String api_v2_update_audio_codec_exclude_post(body)


For updating an audio codec to be excluded/included in basic audio transcoding settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2UpdateAudioCodecExcludePostRequest**](ApiV2UpdateAudioCodecExcludePostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_update_plugin_include_post

> String api_v2_update_plugin_include_post(body)


For enabling/disabling a plugin in a classic plugin stack

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2UpdateAudioCodecExcludePostRequest**](ApiV2UpdateAudioCodecExcludePostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_update_schedule_block_post

> String api_v2_update_schedule_block_post(body)


For updating a block in a library schedule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2UpdateAudioCodecExcludePostRequest**](ApiV2UpdateAudioCodecExcludePostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_update_video_codec_exclude_post

> String api_v2_update_video_codec_exclude_post(body)


For updating an video codec to be excluded/included in basic video transcoding settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2UpdateAudioCodecExcludePostRequest**](ApiV2UpdateAudioCodecExcludePostRequest.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_verify_folder_exists_post

> bool api_v2_verify_folder_exists_post(body)


For verifying if a folder exists

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2GetSubdirectoriesPostRequest**](ApiV2GetSubdirectoriesPostRequest.md)> |  |  |

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_verify_plugin_post

> bool api_v2_verify_plugin_post(body)


For verifying if a classic plugin exists

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ApiV2VerifyPluginPostRequest**](ApiV2VerifyPluginPostRequest.md)> |  |  |

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

