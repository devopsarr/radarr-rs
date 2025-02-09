# \MovieImportApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_movie_import**](MovieImportApi.md#create_movie_import) | **POST** /api/v3/movie/import | 



## create_movie_import

> Vec<models::MovieResource> create_movie_import(movie_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_resource** | Option<[**Vec<models::MovieResource>**](MovieResource.md)> |  |  |

### Return type

[**Vec<models::MovieResource>**](MovieResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

