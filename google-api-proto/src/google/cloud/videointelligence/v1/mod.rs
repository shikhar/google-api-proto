/// Video annotation request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateVideoRequest {
    /// Input video location. Currently, only
    /// [Cloud Storage](<https://cloud.google.com/storage/>) URIs are
    /// supported. URIs must be specified in the following format:
    /// `gs://bucket-id/object-id` (other URI formats return
    /// \[google.rpc.Code.INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT\]). For
    /// more information, see [Request
    /// URIs](<https://cloud.google.com/storage/docs/request-endpoints>). To identify
    /// multiple videos, a video URI may include wildcards in the `object-id`.
    /// Supported wildcards: '*' to match 0 or more characters;
    /// '?' to match 1 character. If unset, the input video should be embedded
    /// in the request as `input_content`. If set, `input_content` must be unset.
    #[prost(string, tag="1")]
    pub input_uri: ::prost::alloc::string::String,
    /// The video data bytes.
    /// If unset, the input video(s) should be specified via the `input_uri`.
    /// If set, `input_uri` must be unset.
    #[prost(bytes="bytes", tag="6")]
    pub input_content: ::prost::bytes::Bytes,
    /// Required. Requested video annotation features.
    #[prost(enumeration="Feature", repeated, packed="false", tag="2")]
    pub features: ::prost::alloc::vec::Vec<i32>,
    /// Additional video context and/or feature-specific parameters.
    #[prost(message, optional, tag="3")]
    pub video_context: ::core::option::Option<VideoContext>,
    /// Optional. Location where the output (in JSON format) should be stored.
    /// Currently, only [Cloud Storage](<https://cloud.google.com/storage/>)
    /// URIs are supported. These must be specified in the following format:
    /// `gs://bucket-id/object-id` (other URI formats return
    /// \[google.rpc.Code.INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT\]). For
    /// more information, see [Request
    /// URIs](<https://cloud.google.com/storage/docs/request-endpoints>).
    #[prost(string, tag="4")]
    pub output_uri: ::prost::alloc::string::String,
    /// Optional. Cloud region where annotation should take place. Supported cloud
    /// regions are: `us-east1`, `us-west1`, `europe-west1`, `asia-east1`. If no
    /// region is specified, the region will be determined based on video file
    /// location.
    #[prost(string, tag="5")]
    pub location_id: ::prost::alloc::string::String,
}
/// Video context and/or feature-specific parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoContext {
    /// Video segments to annotate. The segments may overlap and are not required
    /// to be contiguous or span the whole video. If unspecified, each video is
    /// treated as a single segment.
    #[prost(message, repeated, tag="1")]
    pub segments: ::prost::alloc::vec::Vec<VideoSegment>,
    /// Config for LABEL_DETECTION.
    #[prost(message, optional, tag="2")]
    pub label_detection_config: ::core::option::Option<LabelDetectionConfig>,
    /// Config for SHOT_CHANGE_DETECTION.
    #[prost(message, optional, tag="3")]
    pub shot_change_detection_config: ::core::option::Option<ShotChangeDetectionConfig>,
    /// Config for EXPLICIT_CONTENT_DETECTION.
    #[prost(message, optional, tag="4")]
    pub explicit_content_detection_config: ::core::option::Option<ExplicitContentDetectionConfig>,
    /// Config for FACE_DETECTION.
    #[prost(message, optional, tag="5")]
    pub face_detection_config: ::core::option::Option<FaceDetectionConfig>,
    /// Config for SPEECH_TRANSCRIPTION.
    #[prost(message, optional, tag="6")]
    pub speech_transcription_config: ::core::option::Option<SpeechTranscriptionConfig>,
    /// Config for TEXT_DETECTION.
    #[prost(message, optional, tag="8")]
    pub text_detection_config: ::core::option::Option<TextDetectionConfig>,
    /// Config for PERSON_DETECTION.
    #[prost(message, optional, tag="11")]
    pub person_detection_config: ::core::option::Option<PersonDetectionConfig>,
    /// Config for OBJECT_TRACKING.
    #[prost(message, optional, tag="13")]
    pub object_tracking_config: ::core::option::Option<ObjectTrackingConfig>,
}
/// Config for LABEL_DETECTION.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelDetectionConfig {
    /// What labels should be detected with LABEL_DETECTION, in addition to
    /// video-level labels or segment-level labels.
    /// If unspecified, defaults to `SHOT_MODE`.
    #[prost(enumeration="LabelDetectionMode", tag="1")]
    pub label_detection_mode: i32,
    /// Whether the video has been shot from a stationary (i.e., non-moving)
    /// camera. When set to true, might improve detection accuracy for moving
    /// objects. Should be used with `SHOT_AND_FRAME_MODE` enabled.
    #[prost(bool, tag="2")]
    pub stationary_camera: bool,
    /// Model to use for label detection.
    /// Supported values: "builtin/stable" (the default if unset) and
    /// "builtin/latest".
    #[prost(string, tag="3")]
    pub model: ::prost::alloc::string::String,
    /// The confidence threshold we perform filtering on the labels from
    /// frame-level detection. If not set, it is set to 0.4 by default. The valid
    /// range for this threshold is [0.1, 0.9]. Any value set outside of this
    /// range will be clipped.
    /// Note: For best results, follow the default threshold. We will update
    /// the default threshold everytime when we release a new model.
    #[prost(float, tag="4")]
    pub frame_confidence_threshold: f32,
    /// The confidence threshold we perform filtering on the labels from
    /// video-level and shot-level detections. If not set, it's set to 0.3 by
    /// default. The valid range for this threshold is [0.1, 0.9]. Any value set
    /// outside of this range will be clipped.
    /// Note: For best results, follow the default threshold. We will update
    /// the default threshold everytime when we release a new model.
    #[prost(float, tag="5")]
    pub video_confidence_threshold: f32,
}
/// Config for SHOT_CHANGE_DETECTION.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShotChangeDetectionConfig {
    /// Model to use for shot change detection.
    /// Supported values: "builtin/stable" (the default if unset) and
    /// "builtin/latest".
    #[prost(string, tag="1")]
    pub model: ::prost::alloc::string::String,
}
/// Config for OBJECT_TRACKING.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectTrackingConfig {
    /// Model to use for object tracking.
    /// Supported values: "builtin/stable" (the default if unset) and
    /// "builtin/latest".
    #[prost(string, tag="1")]
    pub model: ::prost::alloc::string::String,
}
/// Config for FACE_DETECTION.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FaceDetectionConfig {
    /// Model to use for face detection.
    /// Supported values: "builtin/stable" (the default if unset) and
    /// "builtin/latest".
    #[prost(string, tag="1")]
    pub model: ::prost::alloc::string::String,
    /// Whether bounding boxes are included in the face annotation output.
    #[prost(bool, tag="2")]
    pub include_bounding_boxes: bool,
    /// Whether to enable face attributes detection, such as glasses, dark_glasses,
    /// mouth_open etc. Ignored if 'include_bounding_boxes' is set to false.
    #[prost(bool, tag="5")]
    pub include_attributes: bool,
}
/// Config for PERSON_DETECTION.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersonDetectionConfig {
    /// Whether bounding boxes are included in the person detection annotation
    /// output.
    #[prost(bool, tag="1")]
    pub include_bounding_boxes: bool,
    /// Whether to enable pose landmarks detection. Ignored if
    /// 'include_bounding_boxes' is set to false.
    #[prost(bool, tag="2")]
    pub include_pose_landmarks: bool,
    /// Whether to enable person attributes detection, such as cloth color (black,
    /// blue, etc), type (coat, dress, etc), pattern (plain, floral, etc), hair,
    /// etc.
    /// Ignored if 'include_bounding_boxes' is set to false.
    #[prost(bool, tag="3")]
    pub include_attributes: bool,
}
/// Config for EXPLICIT_CONTENT_DETECTION.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplicitContentDetectionConfig {
    /// Model to use for explicit content detection.
    /// Supported values: "builtin/stable" (the default if unset) and
    /// "builtin/latest".
    #[prost(string, tag="1")]
    pub model: ::prost::alloc::string::String,
}
/// Config for TEXT_DETECTION.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextDetectionConfig {
    /// Language hint can be specified if the language to be detected is known a
    /// priori. It can increase the accuracy of the detection. Language hint must
    /// be language code in BCP-47 format.
    ///
    /// Automatic language detection is performed if no hint is provided.
    #[prost(string, repeated, tag="1")]
    pub language_hints: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Model to use for text detection.
    /// Supported values: "builtin/stable" (the default if unset) and
    /// "builtin/latest".
    #[prost(string, tag="2")]
    pub model: ::prost::alloc::string::String,
}
/// Video segment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoSegment {
    /// Time-offset, relative to the beginning of the video,
    /// corresponding to the start of the segment (inclusive).
    #[prost(message, optional, tag="1")]
    pub start_time_offset: ::core::option::Option<::prost_types::Duration>,
    /// Time-offset, relative to the beginning of the video,
    /// corresponding to the end of the segment (inclusive).
    #[prost(message, optional, tag="2")]
    pub end_time_offset: ::core::option::Option<::prost_types::Duration>,
}
/// Video segment level annotation results for label detection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelSegment {
    /// Video segment where a label was detected.
    #[prost(message, optional, tag="1")]
    pub segment: ::core::option::Option<VideoSegment>,
    /// Confidence that the label is accurate. Range: [0, 1].
    #[prost(float, tag="2")]
    pub confidence: f32,
}
/// Video frame level annotation results for label detection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelFrame {
    /// Time-offset, relative to the beginning of the video, corresponding to the
    /// video frame for this location.
    #[prost(message, optional, tag="1")]
    pub time_offset: ::core::option::Option<::prost_types::Duration>,
    /// Confidence that the label is accurate. Range: [0, 1].
    #[prost(float, tag="2")]
    pub confidence: f32,
}
/// Detected entity from video analysis.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entity {
    /// Opaque entity ID. Some IDs may be available in
    /// [Google Knowledge Graph Search
    /// API](<https://developers.google.com/knowledge-graph/>).
    #[prost(string, tag="1")]
    pub entity_id: ::prost::alloc::string::String,
    /// Textual description, e.g., `Fixed-gear bicycle`.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// Language code for `description` in BCP-47 format.
    #[prost(string, tag="3")]
    pub language_code: ::prost::alloc::string::String,
}
/// Label annotation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelAnnotation {
    /// Detected entity.
    #[prost(message, optional, tag="1")]
    pub entity: ::core::option::Option<Entity>,
    /// Common categories for the detected entity.
    /// For example, when the label is `Terrier`, the category is likely `dog`. And
    /// in some cases there might be more than one categories e.g., `Terrier` could
    /// also be a `pet`.
    #[prost(message, repeated, tag="2")]
    pub category_entities: ::prost::alloc::vec::Vec<Entity>,
    /// All video segments where a label was detected.
    #[prost(message, repeated, tag="3")]
    pub segments: ::prost::alloc::vec::Vec<LabelSegment>,
    /// All video frames where a label was detected.
    #[prost(message, repeated, tag="4")]
    pub frames: ::prost::alloc::vec::Vec<LabelFrame>,
    /// Feature version.
    #[prost(string, tag="5")]
    pub version: ::prost::alloc::string::String,
}
/// Video frame level annotation results for explicit content.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplicitContentFrame {
    /// Time-offset, relative to the beginning of the video, corresponding to the
    /// video frame for this location.
    #[prost(message, optional, tag="1")]
    pub time_offset: ::core::option::Option<::prost_types::Duration>,
    /// Likelihood of the pornography content..
    #[prost(enumeration="Likelihood", tag="2")]
    pub pornography_likelihood: i32,
}
/// Explicit content annotation (based on per-frame visual signals only).
/// If no explicit content has been detected in a frame, no annotations are
/// present for that frame.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplicitContentAnnotation {
    /// All video frames where explicit content was detected.
    #[prost(message, repeated, tag="1")]
    pub frames: ::prost::alloc::vec::Vec<ExplicitContentFrame>,
    /// Feature version.
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
}
/// Normalized bounding box.
/// The normalized vertex coordinates are relative to the original image.
/// Range: [0, 1].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NormalizedBoundingBox {
    /// Left X coordinate.
    #[prost(float, tag="1")]
    pub left: f32,
    /// Top Y coordinate.
    #[prost(float, tag="2")]
    pub top: f32,
    /// Right X coordinate.
    #[prost(float, tag="3")]
    pub right: f32,
    /// Bottom Y coordinate.
    #[prost(float, tag="4")]
    pub bottom: f32,
}
/// Face detection annotation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FaceDetectionAnnotation {
    /// The face tracks with attributes.
    #[prost(message, repeated, tag="3")]
    pub tracks: ::prost::alloc::vec::Vec<Track>,
    /// The thumbnail of a person's face.
    #[prost(bytes="bytes", tag="4")]
    pub thumbnail: ::prost::bytes::Bytes,
    /// Feature version.
    #[prost(string, tag="5")]
    pub version: ::prost::alloc::string::String,
}
/// Person detection annotation per video.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersonDetectionAnnotation {
    /// The detected tracks of a person.
    #[prost(message, repeated, tag="1")]
    pub tracks: ::prost::alloc::vec::Vec<Track>,
    /// Feature version.
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
}
/// Video segment level annotation results for face detection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FaceSegment {
    /// Video segment where a face was detected.
    #[prost(message, optional, tag="1")]
    pub segment: ::core::option::Option<VideoSegment>,
}
/// Deprecated. No effect.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FaceFrame {
    /// Normalized Bounding boxes in a frame.
    /// There can be more than one boxes if the same face is detected in multiple
    /// locations within the current frame.
    #[prost(message, repeated, tag="1")]
    pub normalized_bounding_boxes: ::prost::alloc::vec::Vec<NormalizedBoundingBox>,
    /// Time-offset, relative to the beginning of the video,
    /// corresponding to the video frame for this location.
    #[prost(message, optional, tag="2")]
    pub time_offset: ::core::option::Option<::prost_types::Duration>,
}
/// Deprecated. No effect.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FaceAnnotation {
    /// Thumbnail of a representative face view (in JPEG format).
    #[prost(bytes="bytes", tag="1")]
    pub thumbnail: ::prost::bytes::Bytes,
    /// All video segments where a face was detected.
    #[prost(message, repeated, tag="2")]
    pub segments: ::prost::alloc::vec::Vec<FaceSegment>,
    /// All video frames where a face was detected.
    #[prost(message, repeated, tag="3")]
    pub frames: ::prost::alloc::vec::Vec<FaceFrame>,
}
/// For tracking related features.
/// An object at time_offset with attributes, and located with
/// normalized_bounding_box.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimestampedObject {
    /// Normalized Bounding box in a frame, where the object is located.
    #[prost(message, optional, tag="1")]
    pub normalized_bounding_box: ::core::option::Option<NormalizedBoundingBox>,
    /// Time-offset, relative to the beginning of the video,
    /// corresponding to the video frame for this object.
    #[prost(message, optional, tag="2")]
    pub time_offset: ::core::option::Option<::prost_types::Duration>,
    /// Optional. The attributes of the object in the bounding box.
    #[prost(message, repeated, tag="3")]
    pub attributes: ::prost::alloc::vec::Vec<DetectedAttribute>,
    /// Optional. The detected landmarks.
    #[prost(message, repeated, tag="4")]
    pub landmarks: ::prost::alloc::vec::Vec<DetectedLandmark>,
}
/// A track of an object instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Track {
    /// Video segment of a track.
    #[prost(message, optional, tag="1")]
    pub segment: ::core::option::Option<VideoSegment>,
    /// The object with timestamp and attributes per frame in the track.
    #[prost(message, repeated, tag="2")]
    pub timestamped_objects: ::prost::alloc::vec::Vec<TimestampedObject>,
    /// Optional. Attributes in the track level.
    #[prost(message, repeated, tag="3")]
    pub attributes: ::prost::alloc::vec::Vec<DetectedAttribute>,
    /// Optional. The confidence score of the tracked object.
    #[prost(float, tag="4")]
    pub confidence: f32,
}
/// A generic detected attribute represented by name in string format.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetectedAttribute {
    /// The name of the attribute, for example, glasses, dark_glasses, mouth_open.
    /// A full list of supported type names will be provided in the document.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Detected attribute confidence. Range [0, 1].
    #[prost(float, tag="2")]
    pub confidence: f32,
    /// Text value of the detection result. For example, the value for "HairColor"
    /// can be "black", "blonde", etc.
    #[prost(string, tag="3")]
    pub value: ::prost::alloc::string::String,
}
/// A generic detected landmark represented by name in string format and a 2D
/// location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetectedLandmark {
    /// The name of this landmark, for example, left_hand, right_shoulder.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The 2D point of the detected landmark using the normalized image
    /// coordindate system. The normalized coordinates have the range from 0 to 1.
    #[prost(message, optional, tag="2")]
    pub point: ::core::option::Option<NormalizedVertex>,
    /// The confidence score of the detected landmark. Range [0, 1].
    #[prost(float, tag="3")]
    pub confidence: f32,
}
/// Annotation results for a single video.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoAnnotationResults {
    /// Video file location in
    /// [Cloud Storage](<https://cloud.google.com/storage/>).
    #[prost(string, tag="1")]
    pub input_uri: ::prost::alloc::string::String,
    /// Video segment on which the annotation is run.
    #[prost(message, optional, tag="10")]
    pub segment: ::core::option::Option<VideoSegment>,
    /// Topical label annotations on video level or user-specified segment level.
    /// There is exactly one element for each unique label.
    #[prost(message, repeated, tag="2")]
    pub segment_label_annotations: ::prost::alloc::vec::Vec<LabelAnnotation>,
    /// Presence label annotations on video level or user-specified segment level.
    /// There is exactly one element for each unique label. Compared to the
    /// existing topical `segment_label_annotations`, this field presents more
    /// fine-grained, segment-level labels detected in video content and is made
    /// available only when the client sets `LabelDetectionConfig.model` to
    /// "builtin/latest" in the request.
    #[prost(message, repeated, tag="23")]
    pub segment_presence_label_annotations: ::prost::alloc::vec::Vec<LabelAnnotation>,
    /// Topical label annotations on shot level.
    /// There is exactly one element for each unique label.
    #[prost(message, repeated, tag="3")]
    pub shot_label_annotations: ::prost::alloc::vec::Vec<LabelAnnotation>,
    /// Presence label annotations on shot level. There is exactly one element for
    /// each unique label. Compared to the existing topical
    /// `shot_label_annotations`, this field presents more fine-grained, shot-level
    /// labels detected in video content and is made available only when the client
    /// sets `LabelDetectionConfig.model` to "builtin/latest" in the request.
    #[prost(message, repeated, tag="24")]
    pub shot_presence_label_annotations: ::prost::alloc::vec::Vec<LabelAnnotation>,
    /// Label annotations on frame level.
    /// There is exactly one element for each unique label.
    #[prost(message, repeated, tag="4")]
    pub frame_label_annotations: ::prost::alloc::vec::Vec<LabelAnnotation>,
    /// Deprecated. Please use `face_detection_annotations` instead.
    #[deprecated]
    #[prost(message, repeated, tag="5")]
    pub face_annotations: ::prost::alloc::vec::Vec<FaceAnnotation>,
    /// Face detection annotations.
    #[prost(message, repeated, tag="13")]
    pub face_detection_annotations: ::prost::alloc::vec::Vec<FaceDetectionAnnotation>,
    /// Shot annotations. Each shot is represented as a video segment.
    #[prost(message, repeated, tag="6")]
    pub shot_annotations: ::prost::alloc::vec::Vec<VideoSegment>,
    /// Explicit content annotation.
    #[prost(message, optional, tag="7")]
    pub explicit_annotation: ::core::option::Option<ExplicitContentAnnotation>,
    /// Speech transcription.
    #[prost(message, repeated, tag="11")]
    pub speech_transcriptions: ::prost::alloc::vec::Vec<SpeechTranscription>,
    /// OCR text detection and tracking.
    /// Annotations for list of detected text snippets. Each will have list of
    /// frame information associated with it.
    #[prost(message, repeated, tag="12")]
    pub text_annotations: ::prost::alloc::vec::Vec<TextAnnotation>,
    /// Annotations for list of objects detected and tracked in video.
    #[prost(message, repeated, tag="14")]
    pub object_annotations: ::prost::alloc::vec::Vec<ObjectTrackingAnnotation>,
    /// Annotations for list of logos detected, tracked and recognized in video.
    #[prost(message, repeated, tag="19")]
    pub logo_recognition_annotations: ::prost::alloc::vec::Vec<LogoRecognitionAnnotation>,
    /// Person detection annotations.
    #[prost(message, repeated, tag="20")]
    pub person_detection_annotations: ::prost::alloc::vec::Vec<PersonDetectionAnnotation>,
    /// If set, indicates an error. Note that for a single `AnnotateVideoRequest`
    /// some videos may succeed and some may fail.
    #[prost(message, optional, tag="9")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
}
/// Video annotation response. Included in the `response`
/// field of the `Operation` returned by the `GetOperation`
/// call of the `google::longrunning::Operations` service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateVideoResponse {
    /// Annotation results for all videos specified in `AnnotateVideoRequest`.
    #[prost(message, repeated, tag="1")]
    pub annotation_results: ::prost::alloc::vec::Vec<VideoAnnotationResults>,
}
/// Annotation progress for a single video.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoAnnotationProgress {
    /// Video file location in
    /// [Cloud Storage](<https://cloud.google.com/storage/>).
    #[prost(string, tag="1")]
    pub input_uri: ::prost::alloc::string::String,
    /// Approximate percentage processed thus far. Guaranteed to be
    /// 100 when fully processed.
    #[prost(int32, tag="2")]
    pub progress_percent: i32,
    /// Time when the request was received.
    #[prost(message, optional, tag="3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Time of the most recent update.
    #[prost(message, optional, tag="4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Specifies which feature is being tracked if the request contains more than
    /// one feature.
    #[prost(enumeration="Feature", tag="5")]
    pub feature: i32,
    /// Specifies which segment is being tracked if the request contains more than
    /// one segment.
    #[prost(message, optional, tag="6")]
    pub segment: ::core::option::Option<VideoSegment>,
}
/// Video annotation progress. Included in the `metadata`
/// field of the `Operation` returned by the `GetOperation`
/// call of the `google::longrunning::Operations` service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateVideoProgress {
    /// Progress metadata for all videos specified in `AnnotateVideoRequest`.
    #[prost(message, repeated, tag="1")]
    pub annotation_progress: ::prost::alloc::vec::Vec<VideoAnnotationProgress>,
}
/// Config for SPEECH_TRANSCRIPTION.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeechTranscriptionConfig {
    /// Required. *Required* The language of the supplied audio as a
    /// \[BCP-47\](<https://www.rfc-editor.org/rfc/bcp/bcp47.txt>) language tag.
    /// Example: "en-US".
    /// See [Language Support](<https://cloud.google.com/speech/docs/languages>)
    /// for a list of the currently supported language codes.
    #[prost(string, tag="1")]
    pub language_code: ::prost::alloc::string::String,
    /// Optional. Maximum number of recognition hypotheses to be returned.
    /// Specifically, the maximum number of `SpeechRecognitionAlternative` messages
    /// within each `SpeechTranscription`. The server may return fewer than
    /// `max_alternatives`. Valid values are `0`-`30`. A value of `0` or `1` will
    /// return a maximum of one. If omitted, will return a maximum of one.
    #[prost(int32, tag="2")]
    pub max_alternatives: i32,
    /// Optional. If set to `true`, the server will attempt to filter out
    /// profanities, replacing all but the initial character in each filtered word
    /// with asterisks, e.g. "f***". If set to `false` or omitted, profanities
    /// won't be filtered out.
    #[prost(bool, tag="3")]
    pub filter_profanity: bool,
    /// Optional. A means to provide context to assist the speech recognition.
    #[prost(message, repeated, tag="4")]
    pub speech_contexts: ::prost::alloc::vec::Vec<SpeechContext>,
    /// Optional. If 'true', adds punctuation to recognition result hypotheses.
    /// This feature is only available in select languages. Setting this for
    /// requests in other languages has no effect at all. The default 'false' value
    /// does not add punctuation to result hypotheses. NOTE: "This is currently
    /// offered as an experimental service, complimentary to all users. In the
    /// future this may be exclusively available as a premium feature."
    #[prost(bool, tag="5")]
    pub enable_automatic_punctuation: bool,
    /// Optional. For file formats, such as MXF or MKV, supporting multiple audio
    /// tracks, specify up to two tracks. Default: track 0.
    #[prost(int32, repeated, packed="false", tag="6")]
    pub audio_tracks: ::prost::alloc::vec::Vec<i32>,
    /// Optional. If 'true', enables speaker detection for each recognized word in
    /// the top alternative of the recognition result using a speaker_tag provided
    /// in the WordInfo.
    /// Note: When this is true, we send all the words from the beginning of the
    /// audio for the top alternative in every consecutive response.
    /// This is done in order to improve our speaker tags as our models learn to
    /// identify the speakers in the conversation over time.
    #[prost(bool, tag="7")]
    pub enable_speaker_diarization: bool,
    /// Optional. If set, specifies the estimated number of speakers in the
    /// conversation. If not set, defaults to '2'. Ignored unless
    /// enable_speaker_diarization is set to true.
    #[prost(int32, tag="8")]
    pub diarization_speaker_count: i32,
    /// Optional. If `true`, the top result includes a list of words and the
    /// confidence for those words. If `false`, no word-level confidence
    /// information is returned. The default is `false`.
    #[prost(bool, tag="9")]
    pub enable_word_confidence: bool,
}
/// Provides "hints" to the speech recognizer to favor specific words and phrases
/// in the results.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeechContext {
    /// Optional. A list of strings containing words and phrases "hints" so that
    /// the speech recognition is more likely to recognize them. This can be used
    /// to improve the accuracy for specific words and phrases, for example, if
    /// specific commands are typically spoken by the user. This can also be used
    /// to add additional words to the vocabulary of the recognizer. See
    /// [usage limits](<https://cloud.google.com/speech/limits#content>).
    #[prost(string, repeated, tag="1")]
    pub phrases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A speech recognition result corresponding to a portion of the audio.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeechTranscription {
    /// May contain one or more recognition hypotheses (up to the maximum specified
    /// in `max_alternatives`).  These alternatives are ordered in terms of
    /// accuracy, with the top (first) alternative being the most probable, as
    /// ranked by the recognizer.
    #[prost(message, repeated, tag="1")]
    pub alternatives: ::prost::alloc::vec::Vec<SpeechRecognitionAlternative>,
    /// Output only. The \[BCP-47\](<https://www.rfc-editor.org/rfc/bcp/bcp47.txt>)
    /// language tag of the language in this result. This language code was
    /// detected to have the most likelihood of being spoken in the audio.
    #[prost(string, tag="2")]
    pub language_code: ::prost::alloc::string::String,
}
/// Alternative hypotheses (a.k.a. n-best list).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeechRecognitionAlternative {
    /// Transcript text representing the words that the user spoke.
    #[prost(string, tag="1")]
    pub transcript: ::prost::alloc::string::String,
    /// Output only. The confidence estimate between 0.0 and 1.0. A higher number
    /// indicates an estimated greater likelihood that the recognized words are
    /// correct. This field is set only for the top alternative.
    /// This field is not guaranteed to be accurate and users should not rely on it
    /// to be always provided.
    /// The default of 0.0 is a sentinel value indicating `confidence` was not set.
    #[prost(float, tag="2")]
    pub confidence: f32,
    /// Output only. A list of word-specific information for each recognized word.
    /// Note: When `enable_speaker_diarization` is set to true, you will see all
    /// the words from the beginning of the audio.
    #[prost(message, repeated, tag="3")]
    pub words: ::prost::alloc::vec::Vec<WordInfo>,
}
/// Word-specific information for recognized words. Word information is only
/// included in the response when certain request parameters are set, such
/// as `enable_word_time_offsets`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WordInfo {
    /// Time offset relative to the beginning of the audio, and
    /// corresponding to the start of the spoken word. This field is only set if
    /// `enable_word_time_offsets=true` and only in the top hypothesis. This is an
    /// experimental feature and the accuracy of the time offset can vary.
    #[prost(message, optional, tag="1")]
    pub start_time: ::core::option::Option<::prost_types::Duration>,
    /// Time offset relative to the beginning of the audio, and
    /// corresponding to the end of the spoken word. This field is only set if
    /// `enable_word_time_offsets=true` and only in the top hypothesis. This is an
    /// experimental feature and the accuracy of the time offset can vary.
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<::prost_types::Duration>,
    /// The word corresponding to this set of information.
    #[prost(string, tag="3")]
    pub word: ::prost::alloc::string::String,
    /// Output only. The confidence estimate between 0.0 and 1.0. A higher number
    /// indicates an estimated greater likelihood that the recognized words are
    /// correct. This field is set only for the top alternative.
    /// This field is not guaranteed to be accurate and users should not rely on it
    /// to be always provided.
    /// The default of 0.0 is a sentinel value indicating `confidence` was not set.
    #[prost(float, tag="4")]
    pub confidence: f32,
    /// Output only. A distinct integer value is assigned for every speaker within
    /// the audio. This field specifies which one of those speakers was detected to
    /// have spoken this word. Value ranges from 1 up to diarization_speaker_count,
    /// and is only set if speaker diarization is enabled.
    #[prost(int32, tag="5")]
    pub speaker_tag: i32,
}
/// A vertex represents a 2D point in the image.
/// NOTE: the normalized vertex coordinates are relative to the original image
/// and range from 0 to 1.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NormalizedVertex {
    /// X coordinate.
    #[prost(float, tag="1")]
    pub x: f32,
    /// Y coordinate.
    #[prost(float, tag="2")]
    pub y: f32,
}
/// Normalized bounding polygon for text (that might not be aligned with axis).
/// Contains list of the corner points in clockwise order starting from
/// top-left corner. For example, for a rectangular bounding box:
/// When the text is horizontal it might look like:
///         0----1
///         |    |
///         3----2
///
/// When it's clockwise rotated 180 degrees around the top-left corner it
/// becomes:
///         2----3
///         |    |
///         1----0
///
/// and the vertex order will still be (0, 1, 2, 3). Note that values can be less
/// than 0, or greater than 1 due to trignometric calculations for location of
/// the box.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NormalizedBoundingPoly {
    /// Normalized vertices of the bounding polygon.
    #[prost(message, repeated, tag="1")]
    pub vertices: ::prost::alloc::vec::Vec<NormalizedVertex>,
}
/// Video segment level annotation results for text detection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextSegment {
    /// Video segment where a text snippet was detected.
    #[prost(message, optional, tag="1")]
    pub segment: ::core::option::Option<VideoSegment>,
    /// Confidence for the track of detected text. It is calculated as the highest
    /// over all frames where OCR detected text appears.
    #[prost(float, tag="2")]
    pub confidence: f32,
    /// Information related to the frames where OCR detected text appears.
    #[prost(message, repeated, tag="3")]
    pub frames: ::prost::alloc::vec::Vec<TextFrame>,
}
/// Video frame level annotation results for text annotation (OCR).
/// Contains information regarding timestamp and bounding box locations for the
/// frames containing detected OCR text snippets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextFrame {
    /// Bounding polygon of the detected text for this frame.
    #[prost(message, optional, tag="1")]
    pub rotated_bounding_box: ::core::option::Option<NormalizedBoundingPoly>,
    /// Timestamp of this frame.
    #[prost(message, optional, tag="2")]
    pub time_offset: ::core::option::Option<::prost_types::Duration>,
}
/// Annotations related to one detected OCR text snippet. This will contain the
/// corresponding text, confidence value, and frame level information for each
/// detection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextAnnotation {
    /// The detected text.
    #[prost(string, tag="1")]
    pub text: ::prost::alloc::string::String,
    /// All video segments where OCR detected text appears.
    #[prost(message, repeated, tag="2")]
    pub segments: ::prost::alloc::vec::Vec<TextSegment>,
    /// Feature version.
    #[prost(string, tag="3")]
    pub version: ::prost::alloc::string::String,
}
/// Video frame level annotations for object detection and tracking. This field
/// stores per frame location, time offset, and confidence.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectTrackingFrame {
    /// The normalized bounding box location of this object track for the frame.
    #[prost(message, optional, tag="1")]
    pub normalized_bounding_box: ::core::option::Option<NormalizedBoundingBox>,
    /// The timestamp of the frame in microseconds.
    #[prost(message, optional, tag="2")]
    pub time_offset: ::core::option::Option<::prost_types::Duration>,
}
/// Annotations corresponding to one tracked object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectTrackingAnnotation {
    /// Entity to specify the object category that this track is labeled as.
    #[prost(message, optional, tag="1")]
    pub entity: ::core::option::Option<Entity>,
    /// Object category's labeling confidence of this track.
    #[prost(float, tag="4")]
    pub confidence: f32,
    /// Information corresponding to all frames where this object track appears.
    /// Non-streaming batch mode: it may be one or multiple ObjectTrackingFrame
    /// messages in frames.
    /// Streaming mode: it can only be one ObjectTrackingFrame message in frames.
    #[prost(message, repeated, tag="2")]
    pub frames: ::prost::alloc::vec::Vec<ObjectTrackingFrame>,
    /// Feature version.
    #[prost(string, tag="6")]
    pub version: ::prost::alloc::string::String,
    /// Different representation of tracking info in non-streaming batch
    /// and streaming modes.
    #[prost(oneof="object_tracking_annotation::TrackInfo", tags="3, 5")]
    pub track_info: ::core::option::Option<object_tracking_annotation::TrackInfo>,
}
/// Nested message and enum types in `ObjectTrackingAnnotation`.
pub mod object_tracking_annotation {
    /// Different representation of tracking info in non-streaming batch
    /// and streaming modes.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TrackInfo {
        /// Non-streaming batch mode ONLY.
        /// Each object track corresponds to one video segment where it appears.
        #[prost(message, tag="3")]
        Segment(super::VideoSegment),
        /// Streaming mode ONLY.
        /// In streaming mode, we do not know the end time of a tracked object
        /// before it is completed. Hence, there is no VideoSegment info returned.
        /// Instead, we provide a unique identifiable integer track_id so that
        /// the customers can correlate the results of the ongoing
        /// ObjectTrackAnnotation of the same track_id over time.
        #[prost(int64, tag="5")]
        TrackId(i64),
    }
}
/// Annotation corresponding to one detected, tracked and recognized logo class.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogoRecognitionAnnotation {
    /// Entity category information to specify the logo class that all the logo
    /// tracks within this LogoRecognitionAnnotation are recognized as.
    #[prost(message, optional, tag="1")]
    pub entity: ::core::option::Option<Entity>,
    /// All logo tracks where the recognized logo appears. Each track corresponds
    /// to one logo instance appearing in consecutive frames.
    #[prost(message, repeated, tag="2")]
    pub tracks: ::prost::alloc::vec::Vec<Track>,
    /// All video segments where the recognized logo appears. There might be
    /// multiple instances of the same logo class appearing in one VideoSegment.
    #[prost(message, repeated, tag="3")]
    pub segments: ::prost::alloc::vec::Vec<VideoSegment>,
}
/// Video annotation feature.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Feature {
    /// Unspecified.
    Unspecified = 0,
    /// Label detection. Detect objects, such as dog or flower.
    LabelDetection = 1,
    /// Shot change detection.
    ShotChangeDetection = 2,
    /// Explicit content detection.
    ExplicitContentDetection = 3,
    /// Human face detection.
    FaceDetection = 4,
    /// Speech transcription.
    SpeechTranscription = 6,
    /// OCR text detection and tracking.
    TextDetection = 7,
    /// Object detection and tracking.
    ObjectTracking = 9,
    /// Logo detection, tracking, and recognition.
    LogoRecognition = 12,
    /// Person detection.
    PersonDetection = 14,
}
/// Label detection mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LabelDetectionMode {
    /// Unspecified.
    Unspecified = 0,
    /// Detect shot-level labels.
    ShotMode = 1,
    /// Detect frame-level labels.
    FrameMode = 2,
    /// Detect both shot-level and frame-level labels.
    ShotAndFrameMode = 3,
}
/// Bucketized representation of likelihood.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Likelihood {
    /// Unspecified likelihood.
    Unspecified = 0,
    /// Very unlikely.
    VeryUnlikely = 1,
    /// Unlikely.
    Unlikely = 2,
    /// Possible.
    Possible = 3,
    /// Likely.
    Likely = 4,
    /// Very likely.
    VeryLikely = 5,
}
/// Generated client implementations.
pub mod video_intelligence_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Service that implements the Video Intelligence API.
    #[derive(Debug, Clone)]
    pub struct VideoIntelligenceServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> VideoIntelligenceServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Default + Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> VideoIntelligenceServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            VideoIntelligenceServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// Performs asynchronous video annotation. Progress and results can be
        /// retrieved through the `google.longrunning.Operations` interface.
        /// `Operation.metadata` contains `AnnotateVideoProgress` (progress).
        /// `Operation.response` contains `AnnotateVideoResponse` (results).
        pub async fn annotate_video(
            &mut self,
            request: impl tonic::IntoRequest<super::AnnotateVideoRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::longrunning::Operation>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.videointelligence.v1.VideoIntelligenceService/AnnotateVideo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
