# \NamingConfigApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_naming_config**](NamingConfigApi.md#get_naming_config) | **GET** /api/v3/config/naming | 
[**get_naming_config_by_id**](NamingConfigApi.md#get_naming_config_by_id) | **GET** /api/v3/config/naming/{id} | 
[**get_naming_config_examples**](NamingConfigApi.md#get_naming_config_examples) | **GET** /api/v3/config/naming/examples | 
[**update_naming_config**](NamingConfigApi.md#update_naming_config) | **PUT** /api/v3/config/naming/{id} | 



## get_naming_config

> models::NamingConfigResource get_naming_config()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::NamingConfigResource**](NamingConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_naming_config_by_id

> models::NamingConfigResource get_naming_config_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::NamingConfigResource**](NamingConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_naming_config_examples

> get_naming_config_examples(rename_movies, replace_illegal_characters, colon_replacement_format, standard_movie_format, movie_folder_format, id, resource_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rename_movies** | Option<**bool**> |  |  |
**replace_illegal_characters** | Option<**bool**> |  |  |
**colon_replacement_format** | Option<[**ColonReplacementFormat**](.md)> |  |  |
**standard_movie_format** | Option<**String**> |  |  |
**movie_folder_format** | Option<**String**> |  |  |
**id** | Option<**i32**> |  |  |
**resource_name** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_naming_config

> models::NamingConfigResource update_naming_config(id, naming_config_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**naming_config_resource** | Option<[**NamingConfigResource**](NamingConfigResource.md)> |  |  |

### Return type

[**models::NamingConfigResource**](NamingConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

