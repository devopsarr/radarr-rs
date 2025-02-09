# \AlternativeTitleApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_alttitle_by_id**](AlternativeTitleApi.md#get_alttitle_by_id) | **GET** /api/v3/alttitle/{id} | 
[**list_alttitle**](AlternativeTitleApi.md#list_alttitle) | **GET** /api/v3/alttitle | 



## get_alttitle_by_id

> models::AlternativeTitleResource get_alttitle_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::AlternativeTitleResource**](AlternativeTitleResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_alttitle

> Vec<models::AlternativeTitleResource> list_alttitle(movie_id, movie_metadata_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | Option<**i32**> |  |  |
**movie_metadata_id** | Option<**i32**> |  |  |

### Return type

[**Vec<models::AlternativeTitleResource>**](AlternativeTitleResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

