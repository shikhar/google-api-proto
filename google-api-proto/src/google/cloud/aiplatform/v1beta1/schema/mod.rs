#[cfg(
    any(
        feature = "google-cloud-aiplatform-v1beta1-schema-predict-instance",
        feature = "google-cloud-aiplatform-v1beta1-schema-predict-params",
        feature = "google-cloud-aiplatform-v1beta1-schema-predict-prediction",
    )
)]
pub mod predict;
#[cfg(any(feature = "google-cloud-aiplatform-v1beta1-schema-trainingjob-definition"))]
pub mod trainingjob;
/// An entry of mapping between color and AnnotationSpec. The mapping is used in
/// segmentation mask.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotationSpecColor {
    /// The color of the AnnotationSpec in a segmentation mask.
    #[prost(message, optional, tag="1")]
    pub color: ::core::option::Option<super::super::super::super::r#type::Color>,
    /// The display name of the AnnotationSpec represented by the color in the
    /// segmentation mask.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    /// The ID of the AnnotationSpec represented by the color in the segmentation
    /// mask.
    #[prost(string, tag="3")]
    pub id: ::prost::alloc::string::String,
}
/// Payload of Image DataItem.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageDataItem {
    /// Required. Google Cloud Storage URI points to the original image in user's bucket.
    /// The image is up to 30MB in size.
    #[prost(string, tag="1")]
    pub gcs_uri: ::prost::alloc::string::String,
    /// Output only. The mime type of the content of the image. Only the images in below listed
    /// mime types are supported.
    /// - image/jpeg
    /// - image/gif
    /// - image/png
    /// - image/webp
    /// - image/bmp
    /// - image/tiff
    /// - image/vnd.microsoft.icon
    #[prost(string, tag="2")]
    pub mime_type: ::prost::alloc::string::String,
}
/// Payload of Video DataItem.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoDataItem {
    /// Required. Google Cloud Storage URI points to the original video in user's bucket.
    /// The video is up to 50 GB in size and up to 3 hour in duration.
    #[prost(string, tag="1")]
    pub gcs_uri: ::prost::alloc::string::String,
    /// Output only. The mime type of the content of the video. Only the videos in below listed
    /// mime types are supported.
    /// Supported mime_type:
    /// - video/mp4
    /// - video/avi
    /// - video/quicktime
    #[prost(string, tag="2")]
    pub mime_type: ::prost::alloc::string::String,
}
/// Payload of Text DataItem.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextDataItem {
    /// Output only. Google Cloud Storage URI points to the original text in user's bucket.
    /// The text file is up to 10MB in size.
    #[prost(string, tag="1")]
    pub gcs_uri: ::prost::alloc::string::String,
}
/// The metadata of Datasets that contain Image DataItems.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageDatasetMetadata {
    /// Points to a YAML file stored on Google Cloud Storage describing payload of
    /// the Image DataItems that belong to this Dataset.
    #[prost(string, tag="1")]
    pub data_item_schema_uri: ::prost::alloc::string::String,
    /// Google Cloud Storage Bucket name that contains the blob data of this
    /// Dataset.
    #[prost(string, tag="2")]
    pub gcs_bucket: ::prost::alloc::string::String,
}
/// The metadata of Datasets that contain Text DataItems.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextDatasetMetadata {
    /// Points to a YAML file stored on Google Cloud Storage describing payload of
    /// the Text DataItems that belong to this Dataset.
    #[prost(string, tag="1")]
    pub data_item_schema_uri: ::prost::alloc::string::String,
    /// Google Cloud Storage Bucket name that contains the blob data of this
    /// Dataset.
    #[prost(string, tag="2")]
    pub gcs_bucket: ::prost::alloc::string::String,
}
/// The metadata of Datasets that contain Video DataItems.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoDatasetMetadata {
    /// Points to a YAML file stored on Google Cloud Storage describing payload of
    /// the Video DataItems that belong to this Dataset.
    #[prost(string, tag="1")]
    pub data_item_schema_uri: ::prost::alloc::string::String,
    /// Google Cloud Storage Bucket name that contains the blob data of this
    /// Dataset.
    #[prost(string, tag="2")]
    pub gcs_bucket: ::prost::alloc::string::String,
}
/// The metadata of Datasets that contain tables data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TablesDatasetMetadata {
    #[prost(message, optional, tag="1")]
    pub input_config: ::core::option::Option<tables_dataset_metadata::InputConfig>,
}
/// Nested message and enum types in `TablesDatasetMetadata`.
pub mod tables_dataset_metadata {
    /// The tables Dataset's data source. The Dataset doesn't store the data
    /// directly, but only pointer(s) to its data.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InputConfig {
        #[prost(oneof="input_config::Source", tags="1, 2")]
        pub source: ::core::option::Option<input_config::Source>,
    }
    /// Nested message and enum types in `InputConfig`.
    pub mod input_config {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Source {
            #[prost(message, tag="1")]
            GcsSource(super::GcsSource),
            #[prost(message, tag="2")]
            BigquerySource(super::BigQuerySource),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GcsSource {
        /// Cloud Storage URI of one or more files. Only CSV files are supported.
        /// The first line of the CSV file is used as the header.
        /// If there are multiple files, the header is the first line of
        /// the lexicographically first file, the other files must either
        /// contain the exact same header or omit the header.
        #[prost(string, repeated, tag="1")]
        pub uri: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BigQuerySource {
        /// The URI of a BigQuery table.
        /// e.g. bq://projectId.bqDatasetId.bqTableId
        #[prost(string, tag="1")]
        pub uri: ::prost::alloc::string::String,
    }
}
/// The metadata of Datasets that contain time series data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeSeriesDatasetMetadata {
    #[prost(message, optional, tag="1")]
    pub input_config: ::core::option::Option<time_series_dataset_metadata::InputConfig>,
    /// The column name of the time series identifier column that identifies the
    /// time series.
    #[prost(string, tag="2")]
    pub time_series_identifier_column: ::prost::alloc::string::String,
    /// The column name of the time column that identifies time order in the time
    /// series.
    #[prost(string, tag="3")]
    pub time_column: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TimeSeriesDatasetMetadata`.
pub mod time_series_dataset_metadata {
    /// The time series Dataset's data source. The Dataset doesn't store the data
    /// directly, but only pointer(s) to its data.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InputConfig {
        #[prost(oneof="input_config::Source", tags="1, 2")]
        pub source: ::core::option::Option<input_config::Source>,
    }
    /// Nested message and enum types in `InputConfig`.
    pub mod input_config {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Source {
            #[prost(message, tag="1")]
            GcsSource(super::GcsSource),
            #[prost(message, tag="2")]
            BigquerySource(super::BigQuerySource),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GcsSource {
        /// Cloud Storage URI of one or more files. Only CSV files are supported.
        /// The first line of the CSV file is used as the header.
        /// If there are multiple files, the header is the first line of
        /// the lexicographically first file, the other files must either
        /// contain the exact same header or omit the header.
        #[prost(string, repeated, tag="1")]
        pub uri: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BigQuerySource {
        /// The URI of a BigQuery table.
        #[prost(string, tag="1")]
        pub uri: ::prost::alloc::string::String,
    }
}
/// A vertex represents a 2D point in the image.
/// NOTE: the normalized vertex coordinates are relative to the original image
/// and range from 0 to 1.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vertex {
    /// X coordinate.
    #[prost(double, tag="1")]
    pub x: f64,
    /// Y coordinate.
    #[prost(double, tag="2")]
    pub y: f64,
}
/// Annotation details specific to image classification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageClassificationAnnotation {
    /// The resource Id of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag="1")]
    pub annotation_spec_id: ::prost::alloc::string::String,
    /// The display name of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
}
/// Annotation details specific to image object detection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageBoundingBoxAnnotation {
    /// The resource Id of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag="1")]
    pub annotation_spec_id: ::prost::alloc::string::String,
    /// The display name of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    /// The leftmost coordinate of the bounding box.
    #[prost(double, tag="3")]
    pub x_min: f64,
    /// The rightmost coordinate of the bounding box.
    #[prost(double, tag="4")]
    pub x_max: f64,
    /// The topmost coordinate of the bounding box.
    #[prost(double, tag="5")]
    pub y_min: f64,
    /// The bottommost coordinate of the bounding box.
    #[prost(double, tag="6")]
    pub y_max: f64,
}
/// Annotation details specific to image segmentation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageSegmentationAnnotation {
    #[prost(oneof="image_segmentation_annotation::Annotation", tags="3, 4, 5")]
    pub annotation: ::core::option::Option<image_segmentation_annotation::Annotation>,
}
/// Nested message and enum types in `ImageSegmentationAnnotation`.
pub mod image_segmentation_annotation {
    /// The mask based segmentation annotation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MaskAnnotation {
        /// Google Cloud Storage URI that points to the mask image. The image must be
        /// in PNG format. It must have the same size as the DataItem's image. Each
        /// pixel in the image mask represents the AnnotationSpec which the pixel in
        /// the image DataItem belong to. Each color is mapped to one AnnotationSpec
        /// based on annotation_spec_colors.
        #[prost(string, tag="1")]
        pub mask_gcs_uri: ::prost::alloc::string::String,
        /// The mapping between color and AnnotationSpec for this Annotation.
        #[prost(message, repeated, tag="2")]
        pub annotation_spec_colors: ::prost::alloc::vec::Vec<super::AnnotationSpecColor>,
    }
    /// Represents a polygon in image.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PolygonAnnotation {
        /// The vertexes are connected one by one and the last vertex is connected to
        /// the first one to represent a polygon.
        #[prost(message, repeated, tag="1")]
        pub vertexes: ::prost::alloc::vec::Vec<super::Vertex>,
        /// The resource Id of the AnnotationSpec that this Annotation pertains to.
        #[prost(string, tag="2")]
        pub annotation_spec_id: ::prost::alloc::string::String,
        /// The display name of the AnnotationSpec that this Annotation pertains to.
        #[prost(string, tag="3")]
        pub display_name: ::prost::alloc::string::String,
    }
    /// Represents a polyline in image.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PolylineAnnotation {
        /// The vertexes are connected one by one and the last vertex in not
        /// connected to the first one.
        #[prost(message, repeated, tag="1")]
        pub vertexes: ::prost::alloc::vec::Vec<super::Vertex>,
        /// The resource Id of the AnnotationSpec that this Annotation pertains to.
        #[prost(string, tag="2")]
        pub annotation_spec_id: ::prost::alloc::string::String,
        /// The display name of the AnnotationSpec that this Annotation pertains to.
        #[prost(string, tag="3")]
        pub display_name: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Annotation {
        /// Mask based segmentation annotation. Only one mask annotation can exist
        /// for one image.
        #[prost(message, tag="3")]
        MaskAnnotation(MaskAnnotation),
        /// Polygon annotation.
        #[prost(message, tag="4")]
        PolygonAnnotation(PolygonAnnotation),
        /// Polyline annotation.
        #[prost(message, tag="5")]
        PolylineAnnotation(PolylineAnnotation),
    }
}
/// Annotation details specific to text classification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextClassificationAnnotation {
    /// The resource Id of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag="1")]
    pub annotation_spec_id: ::prost::alloc::string::String,
    /// The display name of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
}
/// Annotation details specific to text extraction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextExtractionAnnotation {
    /// The segment of the text content.
    #[prost(message, optional, tag="1")]
    pub text_segment: ::core::option::Option<TextSegment>,
    /// The resource Id of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag="2")]
    pub annotation_spec_id: ::prost::alloc::string::String,
    /// The display name of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag="3")]
    pub display_name: ::prost::alloc::string::String,
}
/// The text segment inside of DataItem.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextSegment {
    /// Zero-based character index of the first character of the text
    /// segment (counting characters from the beginning of the text).
    #[prost(uint64, tag="1")]
    pub start_offset: u64,
    /// Zero-based character index of the first character past the end of
    /// the text segment (counting character from the beginning of the text).
    /// The character at the end_offset is NOT included in the text segment.
    #[prost(uint64, tag="2")]
    pub end_offset: u64,
    /// The text content in the segment for output only.
    #[prost(string, tag="3")]
    pub content: ::prost::alloc::string::String,
}
/// Annotation details specific to text sentiment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextSentimentAnnotation {
    /// The sentiment score for text.
    #[prost(int32, tag="1")]
    pub sentiment: i32,
    /// The sentiment max score for text.
    #[prost(int32, tag="2")]
    pub sentiment_max: i32,
    /// The resource Id of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag="3")]
    pub annotation_spec_id: ::prost::alloc::string::String,
    /// The display name of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag="4")]
    pub display_name: ::prost::alloc::string::String,
}
/// Annotation details specific to video classification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoClassificationAnnotation {
    /// This Annotation applies to the time period represented by the TimeSegment.
    /// If it's not set, the Annotation applies to the whole video.
    #[prost(message, optional, tag="1")]
    pub time_segment: ::core::option::Option<TimeSegment>,
    /// The resource Id of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag="2")]
    pub annotation_spec_id: ::prost::alloc::string::String,
    /// The display name of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag="3")]
    pub display_name: ::prost::alloc::string::String,
}
/// A time period inside of a DataItem that has a time dimension (e.g. video).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeSegment {
    /// Start of the time segment (inclusive), represented as the duration since
    /// the start of the DataItem.
    #[prost(message, optional, tag="1")]
    pub start_time_offset: ::core::option::Option<::prost_types::Duration>,
    /// End of the time segment (exclusive), represented as the duration since the
    /// start of the DataItem.
    #[prost(message, optional, tag="2")]
    pub end_time_offset: ::core::option::Option<::prost_types::Duration>,
}
/// Annotation details specific to video object tracking.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoObjectTrackingAnnotation {
    /// A time (frame) of a video to which this annotation pertains.
    /// Represented as the duration since the video's start.
    #[prost(message, optional, tag="1")]
    pub time_offset: ::core::option::Option<::prost_types::Duration>,
    /// The leftmost coordinate of the bounding box.
    #[prost(double, tag="2")]
    pub x_min: f64,
    /// The rightmost coordinate of the bounding box.
    #[prost(double, tag="3")]
    pub x_max: f64,
    /// The topmost coordinate of the bounding box.
    #[prost(double, tag="4")]
    pub y_min: f64,
    /// The bottommost coordinate of the bounding box.
    #[prost(double, tag="5")]
    pub y_max: f64,
    /// The instance of the object, expressed as a positive integer. Used to track
    /// the same object across different frames.
    #[prost(int64, tag="6")]
    pub instance_id: i64,
    /// The resource Id of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag="7")]
    pub annotation_spec_id: ::prost::alloc::string::String,
    /// The display name of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag="8")]
    pub display_name: ::prost::alloc::string::String,
}
/// Annotation details specific to video action recognition.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoActionRecognitionAnnotation {
    /// This Annotation applies to the time period represented by the TimeSegment.
    /// If it's not set, the Annotation applies to the whole video.
    #[prost(message, optional, tag="1")]
    pub time_segment: ::core::option::Option<TimeSegment>,
    /// The resource Id of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag="2")]
    pub annotation_spec_id: ::prost::alloc::string::String,
    /// The display name of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag="3")]
    pub display_name: ::prost::alloc::string::String,
}
/// Represents a line of JSONL in the batch prediction output file.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictionResult {
    /// The prediction result.
    /// Value is used here instead of Any so that JsonFormat does not append an
    /// extra "@type" field when we convert the proto to JSON and so we can
    /// represent array of objects.
    /// Do not set error if this is set.
    #[prost(message, optional, tag="3")]
    pub prediction: ::core::option::Option<::prost_types::Value>,
    /// The error result.
    /// Do not set prediction if this is set.
    #[prost(message, optional, tag="4")]
    pub error: ::core::option::Option<prediction_result::Error>,
    /// Some identifier from the input so that the prediction can be mapped back to
    /// the input instance.
    #[prost(oneof="prediction_result::Input", tags="1, 2")]
    pub input: ::core::option::Option<prediction_result::Input>,
}
/// Nested message and enum types in `PredictionResult`.
pub mod prediction_result {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Error {
        /// Error status. This will be serialized into the enum name e.g.
        /// "NOT_FOUND".
        #[prost(enumeration="super::super::super::super::super::rpc::Code", tag="1")]
        pub status: i32,
        /// Error message with additional details.
        #[prost(string, tag="2")]
        pub message: ::prost::alloc::string::String,
    }
    /// Some identifier from the input so that the prediction can be mapped back to
    /// the input instance.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Input {
        /// User's input instance.
        /// Struct is used here instead of Any so that JsonFormat does not append an
        /// extra "@type" field when we convert the proto to JSON.
        #[prost(message, tag="1")]
        Instance(::prost_types::Struct),
        /// Optional user-provided key from the input instance.
        #[prost(string, tag="2")]
        Key(::prost::alloc::string::String),
    }
}
