# \ImportListExclusionApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_exclusions**](ImportListExclusionApi.md#create_exclusions) | **POST** /api/v3/exclusions | 
[**create_exclusions_bulk**](ImportListExclusionApi.md#create_exclusions_bulk) | **POST** /api/v3/exclusions/bulk | 
[**delete_exclusions**](ImportListExclusionApi.md#delete_exclusions) | **DELETE** /api/v3/exclusions/{id} | 
[**delete_exclusions_bulk**](ImportListExclusionApi.md#delete_exclusions_bulk) | **DELETE** /api/v3/exclusions/bulk | 
[**get_exclusions_by_id**](ImportListExclusionApi.md#get_exclusions_by_id) | **GET** /api/v3/exclusions/{id} | 
[**get_exclusions_paged**](ImportListExclusionApi.md#get_exclusions_paged) | **GET** /api/v3/exclusions/paged | 
[**list_exclusions**](ImportListExclusionApi.md#list_exclusions) | **GET** /api/v3/exclusions | 
[**update_exclusions**](ImportListExclusionApi.md#update_exclusions) | **PUT** /api/v3/exclusions/{id} | 



## create_exclusions

> models::ImportListExclusionResource create_exclusions(import_list_exclusion_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**import_list_exclusion_resource** | Option<[**ImportListExclusionResource**](ImportListExclusionResource.md)> |  |  |

### Return type

[**models::ImportListExclusionResource**](ImportListExclusionResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_exclusions_bulk

> create_exclusions_bulk(import_list_exclusion_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**import_list_exclusion_resource** | Option<[**Vec<models::ImportListExclusionResource>**](ImportListExclusionResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_exclusions

> delete_exclusions(id)


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


## delete_exclusions_bulk

> delete_exclusions_bulk(import_list_exclusion_bulk_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**import_list_exclusion_bulk_resource** | Option<[**ImportListExclusionBulkResource**](ImportListExclusionBulkResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_exclusions_by_id

> models::ImportListExclusionResource get_exclusions_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::ImportListExclusionResource**](ImportListExclusionResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_exclusions_paged

> models::ImportListExclusionResourcePagingResource get_exclusions_paged(page, page_size, sort_key, sort_direction)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**page_size** | Option<**i32**> |  |  |[default to 10]
**sort_key** | Option<**String**> |  |  |
**sort_direction** | Option<[**SortDirection**](.md)> |  |  |

### Return type

[**models::ImportListExclusionResourcePagingResource**](ImportListExclusionResourcePagingResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_exclusions

> Vec<models::ImportListExclusionResource> list_exclusions()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ImportListExclusionResource>**](ImportListExclusionResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_exclusions

> models::ImportListExclusionResource update_exclusions(id, import_list_exclusion_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**import_list_exclusion_resource** | Option<[**ImportListExclusionResource**](ImportListExclusionResource.md)> |  |  |

### Return type

[**models::ImportListExclusionResource**](ImportListExclusionResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

