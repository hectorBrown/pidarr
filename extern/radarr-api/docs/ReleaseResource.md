# ReleaseResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**guid** | Option<**String**> |  | [optional]
**quality** | Option<[**models::QualityModel**](QualityModel.md)> |  | [optional]
**custom_formats** | Option<[**Vec<models::CustomFormatResource>**](CustomFormatResource.md)> |  | [optional]
**custom_format_score** | Option<**i32**> |  | [optional]
**quality_weight** | Option<**i32**> |  | [optional]
**age** | Option<**i32**> |  | [optional]
**age_hours** | Option<**f64**> |  | [optional]
**age_minutes** | Option<**f64**> |  | [optional]
**size** | Option<**i64**> |  | [optional]
**indexer_id** | Option<**i32**> |  | [optional]
**indexer** | Option<**String**> |  | [optional]
**release_group** | Option<**String**> |  | [optional]
**sub_group** | Option<**String**> |  | [optional]
**release_hash** | Option<**String**> |  | [optional]
**title** | Option<**String**> |  | [optional]
**scene_source** | Option<**bool**> |  | [optional]
**movie_titles** | Option<**Vec<String>**> |  | [optional]
**languages** | Option<[**Vec<models::Language>**](Language.md)> |  | [optional]
**mapped_movie_id** | Option<**i32**> |  | [optional]
**approved** | Option<**bool**> |  | [optional]
**temporarily_rejected** | Option<**bool**> |  | [optional]
**rejected** | Option<**bool**> |  | [optional]
**tmdb_id** | Option<**i32**> |  | [optional]
**imdb_id** | Option<**i32**> |  | [optional]
**rejections** | Option<**Vec<String>**> |  | [optional]
**publish_date** | Option<**String**> |  | [optional]
**comment_url** | Option<**String**> |  | [optional]
**download_url** | Option<**String**> |  | [optional]
**info_url** | Option<**String**> |  | [optional]
**movie_requested** | Option<**bool**> |  | [optional]
**download_allowed** | Option<**bool**> |  | [optional]
**release_weight** | Option<**i32**> |  | [optional]
**edition** | Option<**String**> |  | [optional]
**magnet_url** | Option<**String**> |  | [optional]
**info_hash** | Option<**String**> |  | [optional]
**seeders** | Option<**i32**> |  | [optional]
**leechers** | Option<**i32**> |  | [optional]
**protocol** | Option<[**models::DownloadProtocol**](DownloadProtocol.md)> |  | [optional]
**indexer_flags** | Option<[**serde_json::Value**](.md)> |  | [optional]
**movie_id** | Option<**i32**> |  | [optional]
**download_client_id** | Option<**i32**> |  | [optional]
**download_client** | Option<**String**> |  | [optional]
**should_override** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


