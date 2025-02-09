# \CalendarApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_calendar**](CalendarApi.md#list_calendar) | **GET** /api/v3/calendar | 



## list_calendar

> Vec<models::MovieResource> list_calendar(start, end, unmonitored, tags)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**String**> |  |  |
**end** | Option<**String**> |  |  |
**unmonitored** | Option<**bool**> |  |  |[default to false]
**tags** | Option<**String**> |  |  |[default to ]

### Return type

[**Vec<models::MovieResource>**](MovieResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

