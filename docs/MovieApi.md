# \MovieApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_movie**](MovieApi.md#create_movie) | **POST** /api/v3/movie | 
[**delete_movie**](MovieApi.md#delete_movie) | **DELETE** /api/v3/movie/{id} | 
[**get_movie_by_id**](MovieApi.md#get_movie_by_id) | **GET** /api/v3/movie/{id} | 
[**list_movie**](MovieApi.md#list_movie) | **GET** /api/v3/movie | 
[**update_movie**](MovieApi.md#update_movie) | **PUT** /api/v3/movie/{id} | 



## create_movie

> models::MovieResource create_movie(movie_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_resource** | Option<[**MovieResource**](MovieResource.md)> |  |  |

### Return type

[**models::MovieResource**](MovieResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_movie

> delete_movie(id, delete_files, add_import_exclusion)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**delete_files** | Option<**bool**> |  |  |[default to false]
**add_import_exclusion** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_by_id

> models::MovieResource get_movie_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::MovieResource**](MovieResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_movie

> Vec<models::MovieResource> list_movie(tmdb_id, exclude_local_covers, language_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tmdb_id** | Option<**i32**> |  |  |
**exclude_local_covers** | Option<**bool**> |  |  |[default to false]
**language_id** | Option<**i32**> |  |  |

### Return type

[**Vec<models::MovieResource>**](MovieResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_movie

> models::MovieResource update_movie(id, move_files, movie_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**move_files** | Option<**bool**> |  |  |[default to false]
**movie_resource** | Option<[**MovieResource**](MovieResource.md)> |  |  |

### Return type

[**models::MovieResource**](MovieResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

