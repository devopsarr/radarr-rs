# \BlocklistApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_blocklist**](BlocklistApi.md#delete_blocklist) | **DELETE** /api/v3/blocklist/{id} | 
[**delete_blocklist_bulk**](BlocklistApi.md#delete_blocklist_bulk) | **DELETE** /api/v3/blocklist/bulk | 
[**get_blocklist**](BlocklistApi.md#get_blocklist) | **GET** /api/v3/blocklist | 
[**list_blocklist_movie**](BlocklistApi.md#list_blocklist_movie) | **GET** /api/v3/blocklist/movie | 



## delete_blocklist

> delete_blocklist(id)


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


## delete_blocklist_bulk

> delete_blocklist_bulk(blocklist_bulk_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blocklist_bulk_resource** | Option<[**BlocklistBulkResource**](BlocklistBulkResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blocklist

> models::BlocklistResourcePagingResource get_blocklist(page, page_size, sort_key, sort_direction, movie_ids, protocols)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**page_size** | Option<**i32**> |  |  |[default to 10]
**sort_key** | Option<**String**> |  |  |
**sort_direction** | Option<[**SortDirection**](.md)> |  |  |
**movie_ids** | Option<[**Vec<i32>**](i32.md)> |  |  |
**protocols** | Option<[**Vec<models::DownloadProtocol>**](models::DownloadProtocol.md)> |  |  |

### Return type

[**models::BlocklistResourcePagingResource**](BlocklistResourcePagingResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_blocklist_movie

> Vec<models::BlocklistResource> list_blocklist_movie(movie_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | Option<**i32**> |  |  |

### Return type

[**Vec<models::BlocklistResource>**](BlocklistResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

