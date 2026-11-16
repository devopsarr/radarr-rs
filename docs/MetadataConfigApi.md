# \MetadataConfigApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_metadata_config**](MetadataConfigApi.md#get_metadata_config) | **GET** /api/v3/config/metadata | 
[**get_metadata_config_by_id**](MetadataConfigApi.md#get_metadata_config_by_id) | **GET** /api/v3/config/metadata/{id} | 
[**update_metadata_config**](MetadataConfigApi.md#update_metadata_config) | **PUT** /api/v3/config/metadata/{id} | 



## get_metadata_config

> models::MetadataConfigResource get_metadata_config()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MetadataConfigResource**](MetadataConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metadata_config_by_id

> models::MetadataConfigResource get_metadata_config_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::MetadataConfigResource**](MetadataConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_metadata_config

> models::MetadataConfigResource update_metadata_config(id, metadata_config_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**metadata_config_resource** | Option<[**MetadataConfigResource**](MetadataConfigResource.md)> |  |  |

### Return type

[**models::MetadataConfigResource**](MetadataConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

