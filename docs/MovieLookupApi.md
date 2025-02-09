# \MovieLookupApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_movie_lookup**](MovieLookupApi.md#list_movie_lookup) | **GET** /api/v3/movie/lookup | 
[**list_movie_lookup_imdb**](MovieLookupApi.md#list_movie_lookup_imdb) | **GET** /api/v3/movie/lookup/imdb | 
[**list_movie_lookup_tmdb**](MovieLookupApi.md#list_movie_lookup_tmdb) | **GET** /api/v3/movie/lookup/tmdb | 



## list_movie_lookup

> Vec<models::MovieResource> list_movie_lookup(term)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term** | Option<**String**> |  |  |

### Return type

[**Vec<models::MovieResource>**](MovieResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_movie_lookup_imdb

> Vec<models::MovieResource> list_movie_lookup_imdb(imdb_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**imdb_id** | Option<**String**> |  |  |

### Return type

[**Vec<models::MovieResource>**](MovieResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_movie_lookup_tmdb

> Vec<models::MovieResource> list_movie_lookup_tmdb(tmdb_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tmdb_id** | Option<**i32**> |  |  |

### Return type

[**Vec<models::MovieResource>**](MovieResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

