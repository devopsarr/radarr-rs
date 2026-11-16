# MovieCollection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**title** | Option<**String**> |  | [optional]
**clean_title** | Option<**String**> |  | [optional]
**sort_title** | Option<**String**> |  | [optional]
**tmdb_id** | Option<**i32**> |  | [optional]
**overview** | Option<**String**> |  | [optional]
**monitored** | Option<**bool**> |  | [optional]
**quality_profile_id** | Option<**i32**> |  | [optional]
**root_folder_path** | Option<**String**> |  | [optional]
**search_on_add** | Option<**bool**> |  | [optional]
**minimum_availability** | Option<[**models::MovieStatusType**](MovieStatusType.md)> |  | [optional]
**last_info_sync** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**images** | Option<[**Vec<models::MediaCover>**](MediaCover.md)> |  | [optional]
**added** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**movies** | Option<[**Vec<models::MovieMetadata>**](MovieMetadata.md)> |  | [optional]
**tags** | Option<**HashSet<i32>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


