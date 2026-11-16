# \FileSystemApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_file_system**](FileSystemApi.md#get_file_system) | **GET** /api/v3/filesystem | 
[**get_file_system_type**](FileSystemApi.md#get_file_system_type) | **GET** /api/v3/filesystem/type | 
[**list_file_system_mediafiles**](FileSystemApi.md#list_file_system_mediafiles) | **GET** /api/v3/filesystem/mediafiles | 



## get_file_system

> models::FileSystemResult get_file_system(path, include_files, allow_folders_without_trailing_slashes)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | Option<**String**> |  |  |
**include_files** | Option<**bool**> |  |  |[default to false]
**allow_folders_without_trailing_slashes** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::FileSystemResult**](FileSystemResult.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file_system_type

> models::FileSystemTypeResource get_file_system_type(path)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | Option<**String**> |  |  |

### Return type

[**models::FileSystemTypeResource**](FileSystemTypeResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_file_system_mediafiles

> Vec<models::FileSystemMediaFileResource> list_file_system_mediafiles(path)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | Option<**String**> |  |  |

### Return type

[**Vec<models::FileSystemMediaFileResource>**](FileSystemMediaFileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

