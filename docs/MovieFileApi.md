# \MovieFileApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_movie_file**](MovieFileApi.md#delete_movie_file) | **DELETE** /api/v3/moviefile/{id} | 
[**delete_movie_file_bulk**](MovieFileApi.md#delete_movie_file_bulk) | **DELETE** /api/v3/moviefile/bulk | 
[**get_movie_file_by_id**](MovieFileApi.md#get_movie_file_by_id) | **GET** /api/v3/moviefile/{id} | 
[**list_movie_file**](MovieFileApi.md#list_movie_file) | **GET** /api/v3/moviefile | 
[**put_movie_file_editor**](MovieFileApi.md#put_movie_file_editor) | **PUT** /api/v3/moviefile/editor | 
[**update_movie_file**](MovieFileApi.md#update_movie_file) | **PUT** /api/v3/moviefile/{id} | 



## delete_movie_file

> delete_movie_file(id)


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


## delete_movie_file_bulk

> delete_movie_file_bulk(movie_file_list_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_file_list_resource** | Option<[**MovieFileListResource**](MovieFileListResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_file_by_id

> models::MovieFileResource get_movie_file_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::MovieFileResource**](MovieFileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_movie_file

> Vec<models::MovieFileResource> list_movie_file(movie_id, movie_file_ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**movie_file_ids** | Option<[**Vec<i32>**](i32.md)> |  |  |

### Return type

[**Vec<models::MovieFileResource>**](MovieFileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_movie_file_editor

> put_movie_file_editor(movie_file_list_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_file_list_resource** | Option<[**MovieFileListResource**](MovieFileListResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_movie_file

> models::MovieFileResource update_movie_file(id, movie_file_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**movie_file_resource** | Option<[**MovieFileResource**](MovieFileResource.md)> |  |  |

### Return type

[**models::MovieFileResource**](MovieFileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

