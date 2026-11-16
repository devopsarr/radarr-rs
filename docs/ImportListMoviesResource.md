# ImportListMoviesResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**title** | Option<**String**> |  | [optional]
**sort_title** | Option<**String**> |  | [optional]
**original_language** | Option<[**models::Language**](Language.md)> |  | [optional]
**status** | Option<[**models::MovieStatusType**](MovieStatusType.md)> |  | [optional]
**overview** | Option<**String**> |  | [optional]
**in_cinemas** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**physical_release** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**digital_release** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**images** | Option<[**Vec<models::MediaCover>**](MediaCover.md)> |  | [optional]
**website** | Option<**String**> |  | [optional]
**remote_poster** | Option<**String**> |  | [optional]
**year** | Option<**i32**> |  | [optional]
**you_tube_trailer_id** | Option<**String**> |  | [optional]
**studio** | Option<**String**> |  | [optional]
**runtime** | Option<**i32**> |  | [optional]
**imdb_id** | Option<**String**> |  | [optional]
**tmdb_id** | Option<**i32**> |  | [optional]
**folder** | Option<**String**> |  | [optional]
**certification** | Option<**String**> |  | [optional]
**popularity** | Option<**f32**> |  | [optional]
**genres** | Option<**Vec<String>**> |  | [optional]
**ratings** | Option<[**models::Ratings**](Ratings.md)> |  | [optional]
**collection** | Option<[**models::MovieCollection**](MovieCollection.md)> |  | [optional]
**is_excluded** | Option<**bool**> |  | [optional]
**is_existing** | Option<**bool**> |  | [optional]
**is_trending** | Option<**bool**> |  | [optional]
**is_popular** | Option<**bool**> |  | [optional]
**is_recommendation** | Option<**bool**> |  | [optional]
**lists** | Option<**HashSet<i32>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


