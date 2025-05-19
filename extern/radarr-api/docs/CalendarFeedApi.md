# \CalendarFeedApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**feed_v3_calendar_radarr_ics_get**](CalendarFeedApi.md#feed_v3_calendar_radarr_ics_get) | **GET** /feed/v3/calendar/radarr.ics | 



## feed_v3_calendar_radarr_ics_get

> feed_v3_calendar_radarr_ics_get(past_days, future_days, tags, unmonitored)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**past_days** | Option<**i32**> |  |  |[default to 7]
**future_days** | Option<**i32**> |  |  |[default to 28]
**tags** | Option<**String**> |  |  |[default to ]
**unmonitored** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

