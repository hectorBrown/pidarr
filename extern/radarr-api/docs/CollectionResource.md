# CollectionResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**title** | Option<**String**> |  | [optional]
**sort_title** | Option<**String**> |  | [optional]
**tmdb_id** | Option<**i32**> |  | [optional]
**images** | Option<[**Vec<models::MediaCover>**](MediaCover.md)> |  | [optional]
**overview** | Option<**String**> |  | [optional]
**monitored** | Option<**bool**> |  | [optional]
**root_folder_path** | Option<**String**> |  | [optional]
**quality_profile_id** | Option<**i32**> |  | [optional]
**search_on_add** | Option<**bool**> |  | [optional]
**minimum_availability** | Option<[**models::MovieStatusType**](MovieStatusType.md)> |  | [optional]
**movies** | Option<[**Vec<models::CollectionMovieResource>**](CollectionMovieResource.md)> |  | [optional]
**missing_movies** | Option<**i32**> |  | [optional]
**tags** | Option<**Vec<i32>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


