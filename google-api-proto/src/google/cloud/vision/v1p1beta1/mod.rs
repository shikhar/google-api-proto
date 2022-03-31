/// A vertex represents a 2D point in the image.
/// NOTE: the vertex coordinates are in the same scale as the original image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vertex {
    /// X coordinate.
    #[prost(int32, tag = "1")]
    pub x: i32,
    /// Y coordinate.
    #[prost(int32, tag = "2")]
    pub y: i32,
}
/// A bounding polygon for the detected image annotation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoundingPoly {
    /// The bounding polygon vertices.
    #[prost(message, repeated, tag = "1")]
    pub vertices: ::prost::alloc::vec::Vec<Vertex>,
}
/// A 3D position in the image, used primarily for Face detection landmarks.
/// A valid Position must have both x and y coordinates.
/// The position coordinates are in the same scale as the original image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Position {
    /// X coordinate.
    #[prost(float, tag = "1")]
    pub x: f32,
    /// Y coordinate.
    #[prost(float, tag = "2")]
    pub y: f32,
    /// Z coordinate (or depth).
    #[prost(float, tag = "3")]
    pub z: f32,
}
/// Relevant information for the image from the Internet.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebDetection {
    /// Deduced entities from similar images on the Internet.
    #[prost(message, repeated, tag = "1")]
    pub web_entities: ::prost::alloc::vec::Vec<web_detection::WebEntity>,
    /// Fully matching images from the Internet.
    /// Can include resized copies of the query image.
    #[prost(message, repeated, tag = "2")]
    pub full_matching_images: ::prost::alloc::vec::Vec<web_detection::WebImage>,
    /// Partial matching images from the Internet.
    /// Those images are similar enough to share some key-point features. For
    /// example an original image will likely have partial matching for its crops.
    #[prost(message, repeated, tag = "3")]
    pub partial_matching_images: ::prost::alloc::vec::Vec<web_detection::WebImage>,
    /// Web pages containing the matching images from the Internet.
    #[prost(message, repeated, tag = "4")]
    pub pages_with_matching_images: ::prost::alloc::vec::Vec<web_detection::WebPage>,
    /// The visually similar image results.
    #[prost(message, repeated, tag = "6")]
    pub visually_similar_images: ::prost::alloc::vec::Vec<web_detection::WebImage>,
    /// Best guess text labels for the request image.
    #[prost(message, repeated, tag = "8")]
    pub best_guess_labels: ::prost::alloc::vec::Vec<web_detection::WebLabel>,
}
/// Nested message and enum types in `WebDetection`.
pub mod web_detection {
    /// Entity deduced from similar images on the Internet.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WebEntity {
        /// Opaque entity ID.
        #[prost(string, tag = "1")]
        pub entity_id: ::prost::alloc::string::String,
        /// Overall relevancy score for the entity.
        /// Not normalized and not comparable across different image queries.
        #[prost(float, tag = "2")]
        pub score: f32,
        /// Canonical description of the entity, in English.
        #[prost(string, tag = "3")]
        pub description: ::prost::alloc::string::String,
    }
    /// Metadata for online images.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WebImage {
        /// The result image URL.
        #[prost(string, tag = "1")]
        pub url: ::prost::alloc::string::String,
        /// (Deprecated) Overall relevancy score for the image.
        #[prost(float, tag = "2")]
        pub score: f32,
    }
    /// Metadata for web pages.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WebPage {
        /// The result web page URL.
        #[prost(string, tag = "1")]
        pub url: ::prost::alloc::string::String,
        /// (Deprecated) Overall relevancy score for the web page.
        #[prost(float, tag = "2")]
        pub score: f32,
        /// Title for the web page, may contain HTML markups.
        #[prost(string, tag = "3")]
        pub page_title: ::prost::alloc::string::String,
        /// Fully matching images on the page.
        /// Can include resized copies of the query image.
        #[prost(message, repeated, tag = "4")]
        pub full_matching_images: ::prost::alloc::vec::Vec<WebImage>,
        /// Partial matching images on the page.
        /// Those images are similar enough to share some key-point features. For
        /// example an original image will likely have partial matching for its
        /// crops.
        #[prost(message, repeated, tag = "5")]
        pub partial_matching_images: ::prost::alloc::vec::Vec<WebImage>,
    }
    /// Label to provide extra metadata for the web detection.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WebLabel {
        /// Label for extra metadata.
        #[prost(string, tag = "1")]
        pub label: ::prost::alloc::string::String,
        /// The BCP-47 language code for `label`, such as "en-US" or "sr-Latn".
        /// For more information, see
        /// <http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.>
        #[prost(string, tag = "2")]
        pub language_code: ::prost::alloc::string::String,
    }
}
/// TextAnnotation contains a structured representation of OCR extracted text.
/// The hierarchy of an OCR extracted text structure is like this:
///     TextAnnotation -> Page -> Block -> Paragraph -> Word -> Symbol
/// Each structural component, starting from Page, may further have their own
/// properties. Properties describe detected languages, breaks etc.. Please refer
/// to the
/// \[TextAnnotation.TextProperty][google.cloud.vision.v1p1beta1.TextAnnotation.TextProperty\]
/// message definition below for more detail.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextAnnotation {
    /// List of pages detected by OCR.
    #[prost(message, repeated, tag = "1")]
    pub pages: ::prost::alloc::vec::Vec<Page>,
    /// UTF-8 text detected on the pages.
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TextAnnotation`.
pub mod text_annotation {
    /// Detected language for a structural component.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DetectedLanguage {
        /// The BCP-47 language code, such as "en-US" or "sr-Latn". For more
        /// information, see
        /// <http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.>
        #[prost(string, tag = "1")]
        pub language_code: ::prost::alloc::string::String,
        /// Confidence of detected language. Range [0, 1].
        #[prost(float, tag = "2")]
        pub confidence: f32,
    }
    /// Detected start or end of a structural component.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DetectedBreak {
        /// Detected break type.
        #[prost(enumeration = "detected_break::BreakType", tag = "1")]
        pub r#type: i32,
        /// True if break prepends the element.
        #[prost(bool, tag = "2")]
        pub is_prefix: bool,
    }
    /// Nested message and enum types in `DetectedBreak`.
    pub mod detected_break {
        /// Enum to denote the type of break found. New line, space etc.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum BreakType {
            /// Unknown break label type.
            Unknown = 0,
            /// Regular space.
            Space = 1,
            /// Sure space (very wide).
            SureSpace = 2,
            /// Line-wrapping break.
            EolSureSpace = 3,
            /// End-line hyphen that is not present in text; does not co-occur with
            /// `SPACE`, `LEADER_SPACE`, or `LINE_BREAK`.
            Hyphen = 4,
            /// Line break that ends a paragraph.
            LineBreak = 5,
        }
    }
    /// Additional information detected on the structural component.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TextProperty {
        /// A list of detected languages together with confidence.
        #[prost(message, repeated, tag = "1")]
        pub detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
        /// Detected start or end of a text segment.
        #[prost(message, optional, tag = "2")]
        pub detected_break: ::core::option::Option<DetectedBreak>,
    }
}
/// Detected page from OCR.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Page {
    /// Additional information detected on the page.
    #[prost(message, optional, tag = "1")]
    pub property: ::core::option::Option<text_annotation::TextProperty>,
    /// Page width in pixels.
    #[prost(int32, tag = "2")]
    pub width: i32,
    /// Page height in pixels.
    #[prost(int32, tag = "3")]
    pub height: i32,
    /// List of blocks of text, images etc on this page.
    #[prost(message, repeated, tag = "4")]
    pub blocks: ::prost::alloc::vec::Vec<Block>,
    /// Confidence of the OCR results on the page. Range [0, 1].
    #[prost(float, tag = "5")]
    pub confidence: f32,
}
/// Logical element on the page.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    /// Additional information detected for the block.
    #[prost(message, optional, tag = "1")]
    pub property: ::core::option::Option<text_annotation::TextProperty>,
    /// The bounding box for the block.
    /// The vertices are in the order of top-left, top-right, bottom-right,
    /// bottom-left. When a rotation of the bounding box is detected the rotation
    /// is represented as around the top-left corner as defined when the text is
    /// read in the 'natural' orientation.
    /// For example:
    ///   * when the text is horizontal it might look like:
    ///      0----1
    ///      |    |
    ///      3----2
    ///   * when it's rotated 180 degrees around the top-left corner it becomes:
    ///      2----3
    ///      |    |
    ///      1----0
    ///   and the vertice order will still be (0, 1, 2, 3).
    #[prost(message, optional, tag = "2")]
    pub bounding_box: ::core::option::Option<BoundingPoly>,
    /// List of paragraphs in this block (if this blocks is of type text).
    #[prost(message, repeated, tag = "3")]
    pub paragraphs: ::prost::alloc::vec::Vec<Paragraph>,
    /// Detected block type (text, image etc) for this block.
    #[prost(enumeration = "block::BlockType", tag = "4")]
    pub block_type: i32,
    /// Confidence of the OCR results on the block. Range [0, 1].
    #[prost(float, tag = "5")]
    pub confidence: f32,
}
/// Nested message and enum types in `Block`.
pub mod block {
    /// Type of a block (text, image etc) as identified by OCR.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BlockType {
        /// Unknown block type.
        Unknown = 0,
        /// Regular text block.
        Text = 1,
        /// Table block.
        Table = 2,
        /// Image block.
        Picture = 3,
        /// Horizontal/vertical line box.
        Ruler = 4,
        /// Barcode block.
        Barcode = 5,
    }
}
/// Structural unit of text representing a number of words in certain order.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Paragraph {
    /// Additional information detected for the paragraph.
    #[prost(message, optional, tag = "1")]
    pub property: ::core::option::Option<text_annotation::TextProperty>,
    /// The bounding box for the paragraph.
    /// The vertices are in the order of top-left, top-right, bottom-right,
    /// bottom-left. When a rotation of the bounding box is detected the rotation
    /// is represented as around the top-left corner as defined when the text is
    /// read in the 'natural' orientation.
    /// For example:
    ///   * when the text is horizontal it might look like:
    ///      0----1
    ///      |    |
    ///      3----2
    ///   * when it's rotated 180 degrees around the top-left corner it becomes:
    ///      2----3
    ///      |    |
    ///      1----0
    ///   and the vertice order will still be (0, 1, 2, 3).
    #[prost(message, optional, tag = "2")]
    pub bounding_box: ::core::option::Option<BoundingPoly>,
    /// List of words in this paragraph.
    #[prost(message, repeated, tag = "3")]
    pub words: ::prost::alloc::vec::Vec<Word>,
    /// Confidence of the OCR results for the paragraph. Range [0, 1].
    #[prost(float, tag = "4")]
    pub confidence: f32,
}
/// A word representation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Word {
    /// Additional information detected for the word.
    #[prost(message, optional, tag = "1")]
    pub property: ::core::option::Option<text_annotation::TextProperty>,
    /// The bounding box for the word.
    /// The vertices are in the order of top-left, top-right, bottom-right,
    /// bottom-left. When a rotation of the bounding box is detected the rotation
    /// is represented as around the top-left corner as defined when the text is
    /// read in the 'natural' orientation.
    /// For example:
    ///   * when the text is horizontal it might look like:
    ///      0----1
    ///      |    |
    ///      3----2
    ///   * when it's rotated 180 degrees around the top-left corner it becomes:
    ///      2----3
    ///      |    |
    ///      1----0
    ///   and the vertice order will still be (0, 1, 2, 3).
    #[prost(message, optional, tag = "2")]
    pub bounding_box: ::core::option::Option<BoundingPoly>,
    /// List of symbols in the word.
    /// The order of the symbols follows the natural reading order.
    #[prost(message, repeated, tag = "3")]
    pub symbols: ::prost::alloc::vec::Vec<Symbol>,
    /// Confidence of the OCR results for the word. Range [0, 1].
    #[prost(float, tag = "4")]
    pub confidence: f32,
}
/// A single symbol representation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Symbol {
    /// Additional information detected for the symbol.
    #[prost(message, optional, tag = "1")]
    pub property: ::core::option::Option<text_annotation::TextProperty>,
    /// The bounding box for the symbol.
    /// The vertices are in the order of top-left, top-right, bottom-right,
    /// bottom-left. When a rotation of the bounding box is detected the rotation
    /// is represented as around the top-left corner as defined when the text is
    /// read in the 'natural' orientation.
    /// For example:
    ///   * when the text is horizontal it might look like:
    ///      0----1
    ///      |    |
    ///      3----2
    ///   * when it's rotated 180 degrees around the top-left corner it becomes:
    ///      2----3
    ///      |    |
    ///      1----0
    ///   and the vertice order will still be (0, 1, 2, 3).
    #[prost(message, optional, tag = "2")]
    pub bounding_box: ::core::option::Option<BoundingPoly>,
    /// The actual UTF-8 representation of the symbol.
    #[prost(string, tag = "3")]
    pub text: ::prost::alloc::string::String,
    /// Confidence of the OCR results for the symbol. Range [0, 1].
    #[prost(float, tag = "4")]
    pub confidence: f32,
}
/// Users describe the type of Google Cloud Vision API tasks to perform over
/// images by using *Feature*s. Each Feature indicates a type of image
/// detection task to perform. Features encode the Cloud Vision API
/// vertical to operate on and the number of top-scoring results to return.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feature {
    /// The feature type.
    #[prost(enumeration = "feature::Type", tag = "1")]
    pub r#type: i32,
    /// Maximum number of results of this type.
    #[prost(int32, tag = "2")]
    pub max_results: i32,
    /// Model to use for the feature.
    /// Supported values: "builtin/stable" (the default if unset) and
    /// "builtin/latest".
    #[prost(string, tag = "3")]
    pub model: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Feature`.
pub mod feature {
    /// Type of image feature.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Unspecified feature type.
        Unspecified = 0,
        /// Run face detection.
        FaceDetection = 1,
        /// Run landmark detection.
        LandmarkDetection = 2,
        /// Run logo detection.
        LogoDetection = 3,
        /// Run label detection.
        LabelDetection = 4,
        /// Run OCR.
        TextDetection = 5,
        /// Run dense text document OCR. Takes precedence when both
        /// DOCUMENT_TEXT_DETECTION and TEXT_DETECTION are present.
        DocumentTextDetection = 11,
        /// Run computer vision models to compute image safe-search properties.
        SafeSearchDetection = 6,
        /// Compute a set of image properties, such as the image's dominant colors.
        ImageProperties = 7,
        /// Run crop hints.
        CropHints = 9,
        /// Run web detection.
        WebDetection = 10,
    }
}
/// External image source (Google Cloud Storage image location).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageSource {
    /// NOTE: For new code `image_uri` below is preferred.
    /// Google Cloud Storage image URI, which must be in the following form:
    /// `gs://bucket_name/object_name` (for details, see
    /// [Google Cloud Storage Request
    /// URIs](<https://cloud.google.com/storage/docs/reference-uris>)).
    /// NOTE: Cloud Storage object versioning is not supported.
    #[prost(string, tag = "1")]
    pub gcs_image_uri: ::prost::alloc::string::String,
    /// Image URI which supports:
    /// 1) Google Cloud Storage image URI, which must be in the following form:
    /// `gs://bucket_name/object_name` (for details, see
    /// [Google Cloud Storage Request
    /// URIs](<https://cloud.google.com/storage/docs/reference-uris>)).
    /// NOTE: Cloud Storage object versioning is not supported.
    /// 2) Publicly accessible image HTTP/HTTPS URL.
    /// This is preferred over the legacy `gcs_image_uri` above. When both
    /// `gcs_image_uri` and `image_uri` are specified, `image_uri` takes
    /// precedence.
    #[prost(string, tag = "2")]
    pub image_uri: ::prost::alloc::string::String,
}
/// Client image to perform Google Cloud Vision API tasks over.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Image {
    /// Image content, represented as a stream of bytes.
    /// Note: as with all `bytes` fields, protobuffers use a pure binary
    /// representation, whereas JSON representations use base64.
    #[prost(bytes = "bytes", tag = "1")]
    pub content: ::prost::bytes::Bytes,
    /// Google Cloud Storage image location. If both `content` and `source`
    /// are provided for an image, `content` takes precedence and is
    /// used to perform the image annotation request.
    #[prost(message, optional, tag = "2")]
    pub source: ::core::option::Option<ImageSource>,
}
/// A face annotation object contains the results of face detection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FaceAnnotation {
    /// The bounding polygon around the face. The coordinates of the bounding box
    /// are in the original image's scale, as returned in `ImageParams`.
    /// The bounding box is computed to "frame" the face in accordance with human
    /// expectations. It is based on the landmarker results.
    /// Note that one or more x and/or y coordinates may not be generated in the
    /// `BoundingPoly` (the polygon will be unbounded) if only a partial face
    /// appears in the image to be annotated.
    #[prost(message, optional, tag = "1")]
    pub bounding_poly: ::core::option::Option<BoundingPoly>,
    /// The `fd_bounding_poly` bounding polygon is tighter than the
    /// `boundingPoly`, and encloses only the skin part of the face. Typically, it
    /// is used to eliminate the face from any image analysis that detects the
    /// "amount of skin" visible in an image. It is not based on the
    /// landmarker results, only on the initial face detection, hence
    /// the <code>fd</code> (face detection) prefix.
    #[prost(message, optional, tag = "2")]
    pub fd_bounding_poly: ::core::option::Option<BoundingPoly>,
    /// Detected face landmarks.
    #[prost(message, repeated, tag = "3")]
    pub landmarks: ::prost::alloc::vec::Vec<face_annotation::Landmark>,
    /// Roll angle, which indicates the amount of clockwise/anti-clockwise rotation
    /// of the face relative to the image vertical about the axis perpendicular to
    /// the face. Range \[-180,180\].
    #[prost(float, tag = "4")]
    pub roll_angle: f32,
    /// Yaw angle, which indicates the leftward/rightward angle that the face is
    /// pointing relative to the vertical plane perpendicular to the image. Range
    /// \[-180,180\].
    #[prost(float, tag = "5")]
    pub pan_angle: f32,
    /// Pitch angle, which indicates the upwards/downwards angle that the face is
    /// pointing relative to the image's horizontal plane. Range \[-180,180\].
    #[prost(float, tag = "6")]
    pub tilt_angle: f32,
    /// Detection confidence. Range [0, 1].
    #[prost(float, tag = "7")]
    pub detection_confidence: f32,
    /// Face landmarking confidence. Range [0, 1].
    #[prost(float, tag = "8")]
    pub landmarking_confidence: f32,
    /// Joy likelihood.
    #[prost(enumeration = "Likelihood", tag = "9")]
    pub joy_likelihood: i32,
    /// Sorrow likelihood.
    #[prost(enumeration = "Likelihood", tag = "10")]
    pub sorrow_likelihood: i32,
    /// Anger likelihood.
    #[prost(enumeration = "Likelihood", tag = "11")]
    pub anger_likelihood: i32,
    /// Surprise likelihood.
    #[prost(enumeration = "Likelihood", tag = "12")]
    pub surprise_likelihood: i32,
    /// Under-exposed likelihood.
    #[prost(enumeration = "Likelihood", tag = "13")]
    pub under_exposed_likelihood: i32,
    /// Blurred likelihood.
    #[prost(enumeration = "Likelihood", tag = "14")]
    pub blurred_likelihood: i32,
    /// Headwear likelihood.
    #[prost(enumeration = "Likelihood", tag = "15")]
    pub headwear_likelihood: i32,
}
/// Nested message and enum types in `FaceAnnotation`.
pub mod face_annotation {
    /// A face-specific landmark (for example, a face feature).
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Landmark {
        /// Face landmark type.
        #[prost(enumeration = "landmark::Type", tag = "3")]
        pub r#type: i32,
        /// Face landmark position.
        #[prost(message, optional, tag = "4")]
        pub position: ::core::option::Option<super::Position>,
    }
    /// Nested message and enum types in `Landmark`.
    pub mod landmark {
        /// Face landmark (feature) type.
        /// Left and right are defined from the vantage of the viewer of the image
        /// without considering mirror projections typical of photos. So, `LEFT_EYE`,
        /// typically, is the person's right eye.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Type {
            /// Unknown face landmark detected. Should not be filled.
            UnknownLandmark = 0,
            /// Left eye.
            LeftEye = 1,
            /// Right eye.
            RightEye = 2,
            /// Left of left eyebrow.
            LeftOfLeftEyebrow = 3,
            /// Right of left eyebrow.
            RightOfLeftEyebrow = 4,
            /// Left of right eyebrow.
            LeftOfRightEyebrow = 5,
            /// Right of right eyebrow.
            RightOfRightEyebrow = 6,
            /// Midpoint between eyes.
            MidpointBetweenEyes = 7,
            /// Nose tip.
            NoseTip = 8,
            /// Upper lip.
            UpperLip = 9,
            /// Lower lip.
            LowerLip = 10,
            /// Mouth left.
            MouthLeft = 11,
            /// Mouth right.
            MouthRight = 12,
            /// Mouth center.
            MouthCenter = 13,
            /// Nose, bottom right.
            NoseBottomRight = 14,
            /// Nose, bottom left.
            NoseBottomLeft = 15,
            /// Nose, bottom center.
            NoseBottomCenter = 16,
            /// Left eye, top boundary.
            LeftEyeTopBoundary = 17,
            /// Left eye, right corner.
            LeftEyeRightCorner = 18,
            /// Left eye, bottom boundary.
            LeftEyeBottomBoundary = 19,
            /// Left eye, left corner.
            LeftEyeLeftCorner = 20,
            /// Right eye, top boundary.
            RightEyeTopBoundary = 21,
            /// Right eye, right corner.
            RightEyeRightCorner = 22,
            /// Right eye, bottom boundary.
            RightEyeBottomBoundary = 23,
            /// Right eye, left corner.
            RightEyeLeftCorner = 24,
            /// Left eyebrow, upper midpoint.
            LeftEyebrowUpperMidpoint = 25,
            /// Right eyebrow, upper midpoint.
            RightEyebrowUpperMidpoint = 26,
            /// Left ear tragion.
            LeftEarTragion = 27,
            /// Right ear tragion.
            RightEarTragion = 28,
            /// Left eye pupil.
            LeftEyePupil = 29,
            /// Right eye pupil.
            RightEyePupil = 30,
            /// Forehead glabella.
            ForeheadGlabella = 31,
            /// Chin gnathion.
            ChinGnathion = 32,
            /// Chin left gonion.
            ChinLeftGonion = 33,
            /// Chin right gonion.
            ChinRightGonion = 34,
        }
    }
}
/// Detected entity location information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationInfo {
    /// lat/long location coordinates.
    #[prost(message, optional, tag = "1")]
    pub lat_lng: ::core::option::Option<super::super::super::r#type::LatLng>,
}
/// A `Property` consists of a user-supplied name/value pair.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Property {
    /// Name of the property.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Value of the property.
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
    /// Value of numeric properties.
    #[prost(uint64, tag = "3")]
    pub uint64_value: u64,
}
/// Set of detected entity features.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityAnnotation {
    /// Opaque entity ID. Some IDs may be available in
    /// [Google Knowledge Graph Search
    /// API](<https://developers.google.com/knowledge-graph/>).
    #[prost(string, tag = "1")]
    pub mid: ::prost::alloc::string::String,
    /// The language code for the locale in which the entity textual
    /// `description` is expressed.
    #[prost(string, tag = "2")]
    pub locale: ::prost::alloc::string::String,
    /// Entity textual description, expressed in its `locale` language.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Overall score of the result. Range [0, 1].
    #[prost(float, tag = "4")]
    pub score: f32,
    /// The accuracy of the entity detection in an image.
    /// For example, for an image in which the "Eiffel Tower" entity is detected,
    /// this field represents the confidence that there is a tower in the query
    /// image. Range [0, 1].
    #[prost(float, tag = "5")]
    pub confidence: f32,
    /// The relevancy of the ICA (Image Content Annotation) label to the
    /// image. For example, the relevancy of "tower" is likely higher to an image
    /// containing the detected "Eiffel Tower" than to an image containing a
    /// detected distant towering building, even though the confidence that
    /// there is a tower in each image may be the same. Range [0, 1].
    #[prost(float, tag = "6")]
    pub topicality: f32,
    /// Image region to which this entity belongs. Not produced
    /// for `LABEL_DETECTION` features.
    #[prost(message, optional, tag = "7")]
    pub bounding_poly: ::core::option::Option<BoundingPoly>,
    /// The location information for the detected entity. Multiple
    /// `LocationInfo` elements can be present because one location may
    /// indicate the location of the scene in the image, and another location
    /// may indicate the location of the place where the image was taken.
    /// Location information is usually present for landmarks.
    #[prost(message, repeated, tag = "8")]
    pub locations: ::prost::alloc::vec::Vec<LocationInfo>,
    /// Some entities may have optional user-supplied `Property` (name/value)
    /// fields, such a score or string that qualifies the entity.
    #[prost(message, repeated, tag = "9")]
    pub properties: ::prost::alloc::vec::Vec<Property>,
}
/// Set of features pertaining to the image, computed by computer vision
/// methods over safe-search verticals (for example, adult, spoof, medical,
/// violence).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SafeSearchAnnotation {
    /// Represents the adult content likelihood for the image. Adult content may
    /// contain elements such as nudity, pornographic images or cartoons, or
    /// sexual activities.
    #[prost(enumeration = "Likelihood", tag = "1")]
    pub adult: i32,
    /// Spoof likelihood. The likelihood that an modification
    /// was made to the image's canonical version to make it appear
    /// funny or offensive.
    #[prost(enumeration = "Likelihood", tag = "2")]
    pub spoof: i32,
    /// Likelihood that this is a medical image.
    #[prost(enumeration = "Likelihood", tag = "3")]
    pub medical: i32,
    /// Likelihood that this image contains violent content.
    #[prost(enumeration = "Likelihood", tag = "4")]
    pub violence: i32,
    /// Likelihood that the request image contains racy content. Racy content may
    /// include (but is not limited to) skimpy or sheer clothing, strategically
    /// covered nudity, lewd or provocative poses, or close-ups of sensitive
    /// body areas.
    #[prost(enumeration = "Likelihood", tag = "9")]
    pub racy: i32,
}
/// Rectangle determined by min and max `LatLng` pairs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LatLongRect {
    /// Min lat/long pair.
    #[prost(message, optional, tag = "1")]
    pub min_lat_lng: ::core::option::Option<super::super::super::r#type::LatLng>,
    /// Max lat/long pair.
    #[prost(message, optional, tag = "2")]
    pub max_lat_lng: ::core::option::Option<super::super::super::r#type::LatLng>,
}
/// Color information consists of RGB channels, score, and the fraction of
/// the image that the color occupies in the image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColorInfo {
    /// RGB components of the color.
    #[prost(message, optional, tag = "1")]
    pub color: ::core::option::Option<super::super::super::r#type::Color>,
    /// Image-specific score for this color. Value in range [0, 1].
    #[prost(float, tag = "2")]
    pub score: f32,
    /// The fraction of pixels the color occupies in the image.
    /// Value in range [0, 1].
    #[prost(float, tag = "3")]
    pub pixel_fraction: f32,
}
/// Set of dominant colors and their corresponding scores.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DominantColorsAnnotation {
    /// RGB color values with their score and pixel fraction.
    #[prost(message, repeated, tag = "1")]
    pub colors: ::prost::alloc::vec::Vec<ColorInfo>,
}
/// Stores image properties, such as dominant colors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageProperties {
    /// If present, dominant colors completed successfully.
    #[prost(message, optional, tag = "1")]
    pub dominant_colors: ::core::option::Option<DominantColorsAnnotation>,
}
/// Single crop hint that is used to generate a new crop when serving an image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CropHint {
    /// The bounding polygon for the crop region. The coordinates of the bounding
    /// box are in the original image's scale, as returned in `ImageParams`.
    #[prost(message, optional, tag = "1")]
    pub bounding_poly: ::core::option::Option<BoundingPoly>,
    /// Confidence of this being a salient region.  Range [0, 1].
    #[prost(float, tag = "2")]
    pub confidence: f32,
    /// Fraction of importance of this salient region with respect to the original
    /// image.
    #[prost(float, tag = "3")]
    pub importance_fraction: f32,
}
/// Set of crop hints that are used to generate new crops when serving images.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CropHintsAnnotation {
    /// Crop hint results.
    #[prost(message, repeated, tag = "1")]
    pub crop_hints: ::prost::alloc::vec::Vec<CropHint>,
}
/// Parameters for crop hints annotation request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CropHintsParams {
    /// Aspect ratios in floats, representing the ratio of the width to the height
    /// of the image. For example, if the desired aspect ratio is 4/3, the
    /// corresponding float value should be 1.33333.  If not specified, the
    /// best possible crop is returned. The number of provided aspect ratios is
    /// limited to a maximum of 16; any aspect ratios provided after the 16th are
    /// ignored.
    #[prost(float, repeated, tag = "1")]
    pub aspect_ratios: ::prost::alloc::vec::Vec<f32>,
}
/// Parameters for web detection request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebDetectionParams {
    /// Whether to include results derived from the geo information in the image.
    #[prost(bool, tag = "2")]
    pub include_geo_results: bool,
}
/// Parameters for text detections. This is used to control TEXT_DETECTION and
/// DOCUMENT_TEXT_DETECTION features.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextDetectionParams {
    /// By default, Cloud Vision API only includes confidence score for
    /// DOCUMENT_TEXT_DETECTION result. Set the flag to true to include confidence
    /// score for TEXT_DETECTION as well.
    #[prost(bool, tag = "9")]
    pub enable_text_detection_confidence_score: bool,
}
/// Image context and/or feature-specific parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageContext {
    /// lat/long rectangle that specifies the location of the image.
    #[prost(message, optional, tag = "1")]
    pub lat_long_rect: ::core::option::Option<LatLongRect>,
    /// List of languages to use for TEXT_DETECTION. In most cases, an empty value
    /// yields the best results since it enables automatic language detection. For
    /// languages based on the Latin alphabet, setting `language_hints` is not
    /// needed. In rare cases, when the language of the text in the image is known,
    /// setting a hint will help get better results (although it will be a
    /// significant hindrance if the hint is wrong). Text detection returns an
    /// error if one or more of the specified languages is not one of the
    /// [supported languages](<https://cloud.google.com/vision/docs/languages>).
    #[prost(string, repeated, tag = "2")]
    pub language_hints: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Parameters for crop hints annotation request.
    #[prost(message, optional, tag = "4")]
    pub crop_hints_params: ::core::option::Option<CropHintsParams>,
    /// Parameters for web detection.
    #[prost(message, optional, tag = "6")]
    pub web_detection_params: ::core::option::Option<WebDetectionParams>,
    /// Parameters for text detection and document text detection.
    #[prost(message, optional, tag = "12")]
    pub text_detection_params: ::core::option::Option<TextDetectionParams>,
}
/// Request for performing Google Cloud Vision API tasks over a user-provided
/// image, with user-requested features.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateImageRequest {
    /// The image to be processed.
    #[prost(message, optional, tag = "1")]
    pub image: ::core::option::Option<Image>,
    /// Requested features.
    #[prost(message, repeated, tag = "2")]
    pub features: ::prost::alloc::vec::Vec<Feature>,
    /// Additional context that may accompany the image.
    #[prost(message, optional, tag = "3")]
    pub image_context: ::core::option::Option<ImageContext>,
}
/// Response to an image annotation request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateImageResponse {
    /// If present, face detection has completed successfully.
    #[prost(message, repeated, tag = "1")]
    pub face_annotations: ::prost::alloc::vec::Vec<FaceAnnotation>,
    /// If present, landmark detection has completed successfully.
    #[prost(message, repeated, tag = "2")]
    pub landmark_annotations: ::prost::alloc::vec::Vec<EntityAnnotation>,
    /// If present, logo detection has completed successfully.
    #[prost(message, repeated, tag = "3")]
    pub logo_annotations: ::prost::alloc::vec::Vec<EntityAnnotation>,
    /// If present, label detection has completed successfully.
    #[prost(message, repeated, tag = "4")]
    pub label_annotations: ::prost::alloc::vec::Vec<EntityAnnotation>,
    /// If present, text (OCR) detection has completed successfully.
    #[prost(message, repeated, tag = "5")]
    pub text_annotations: ::prost::alloc::vec::Vec<EntityAnnotation>,
    /// If present, text (OCR) detection or document (OCR) text detection has
    /// completed successfully.
    /// This annotation provides the structural hierarchy for the OCR detected
    /// text.
    #[prost(message, optional, tag = "12")]
    pub full_text_annotation: ::core::option::Option<TextAnnotation>,
    /// If present, safe-search annotation has completed successfully.
    #[prost(message, optional, tag = "6")]
    pub safe_search_annotation: ::core::option::Option<SafeSearchAnnotation>,
    /// If present, image properties were extracted successfully.
    #[prost(message, optional, tag = "8")]
    pub image_properties_annotation: ::core::option::Option<ImageProperties>,
    /// If present, crop hints have completed successfully.
    #[prost(message, optional, tag = "11")]
    pub crop_hints_annotation: ::core::option::Option<CropHintsAnnotation>,
    /// If present, web detection has completed successfully.
    #[prost(message, optional, tag = "13")]
    pub web_detection: ::core::option::Option<WebDetection>,
    /// If set, represents the error message for the operation.
    /// Note that filled-in image annotations are guaranteed to be
    /// correct, even when `error` is set.
    #[prost(message, optional, tag = "9")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
}
/// Multiple image annotation requests are batched into a single service call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchAnnotateImagesRequest {
    /// Required. Individual image annotation requests for this batch.
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<AnnotateImageRequest>,
}
/// Response to a batch image annotation request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchAnnotateImagesResponse {
    /// Individual responses to image annotation requests within the batch.
    #[prost(message, repeated, tag = "1")]
    pub responses: ::prost::alloc::vec::Vec<AnnotateImageResponse>,
}
/// A bucketized representation of likelihood, which is intended to give clients
/// highly stable results across model upgrades.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Likelihood {
    /// Unknown likelihood.
    Unknown = 0,
    /// It is very unlikely that the image belongs to the specified vertical.
    VeryUnlikely = 1,
    /// It is unlikely that the image belongs to the specified vertical.
    Unlikely = 2,
    /// It is possible that the image belongs to the specified vertical.
    Possible = 3,
    /// It is likely that the image belongs to the specified vertical.
    Likely = 4,
    /// It is very likely that the image belongs to the specified vertical.
    VeryLikely = 5,
}
#[doc = r" Generated client implementations."]
pub mod image_annotator_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service that performs Google Cloud Vision API detection tasks over client"]
    #[doc = " images, such as face, landmark, logo, label, and text detection. The"]
    #[doc = " ImageAnnotator service returns detected entities from the images."]
    #[derive(Debug, Clone)]
    pub struct ImageAnnotatorClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ImageAnnotatorClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ImageAnnotatorClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            ImageAnnotatorClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Run image detection and annotation for a batch of images."]
        pub async fn batch_annotate_images(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchAnnotateImagesRequest>,
        ) -> Result<tonic::Response<super::BatchAnnotateImagesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p1beta1.ImageAnnotator/BatchAnnotateImages",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
