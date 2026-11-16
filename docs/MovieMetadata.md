# MovieMetadata

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**tmdb_id** | Option<**i32**> |  | [optional]
**images** | Option<[**Vec<models::MediaCover>**](MediaCover.md)> |  | [optional]
**genres** | Option<**Vec<String>**> |  | [optional]
**keywords** | Option<**Vec<String>**> |  | [optional]
**in_cinemas** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**physical_release** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**digital_release** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**certification** | Option<**String**> |  | [optional]
**year** | Option<**i32**> |  | [optional]
**ratings** | Option<[**models::Ratings**](Ratings.md)> |  | [optional]
**collection_tmdb_id** | Option<**i32**> |  | [optional]
**collection_title** | Option<**String**> |  | [optional]
**last_info_sync** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**runtime** | Option<**i32**> |  | [optional]
**website** | Option<**String**> |  | [optional]
**imdb_id** | Option<**String**> |  | [optional]
**title** | Option<**String**> |  | [optional]
**clean_title** | Option<**String**> |  | [optional]
**sort_title** | Option<**String**> |  | [optional]
**status** | Option<[**models::MovieStatusType**](MovieStatusType.md)> |  | [optional]
**overview** | Option<**String**> |  | [optional]
**alternative_titles** | Option<[**Vec<models::AlternativeTitle>**](AlternativeTitle.md)> |  | [optional]
**translations** | Option<[**Vec<models::MovieTranslation>**](MovieTranslation.md)> |  | [optional]
**secondary_year** | Option<**i32**> |  | [optional]
**you_tube_trailer_id** | Option<**String**> |  | [optional]
**studio** | Option<**String**> |  | [optional]
**original_title** | Option<**String**> |  | [optional]
**clean_original_title** | Option<**String**> |  | [optional]
**original_language** | Option<[**models::Language**](Language.md)> |  | [optional]
**recommendations** | Option<**Vec<i32>**> |  | [optional]
**popularity** | Option<**f32**> |  | [optional]
**is_recent_movie** | Option<**bool**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


