/// Type of information detected by the API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoType {
    /// Name of the information type. Either a name of your choosing when
    /// creating a CustomInfoType, or one of the names listed
    /// at <https://cloud.google.com/dlp/docs/infotypes-reference> when specifying
    /// a built-in type.  When sending Cloud DLP results to Data Catalog, infoType
    /// names should conform to the pattern `\[A-Za-z0-9$-_\]{1,64}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional version name for this InfoType.
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
}
/// Score is a summary of all elements in the data profile.
/// A higher number means more sensitive.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SensitivityScore {
    /// The score applied to the resource.
    #[prost(enumeration="sensitivity_score::SensitivityScoreLevel", tag="1")]
    pub score: i32,
}
/// Nested message and enum types in `SensitivityScore`.
pub mod sensitivity_score {
    /// Various score levels for resources.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SensitivityScoreLevel {
        /// Unused.
        SensitivityScoreUnspecified = 0,
        /// No sensitive information detected. Limited access.
        SensitivityLow = 10,
        /// Medium risk - PII, potentially sensitive data, or fields with free-text
        /// data that are at higher risk of having intermittent sensitive data.
        /// Consider limiting access.
        SensitivityModerate = 20,
        /// High risk – SPII may be present. Exfiltration of data may lead to user
        /// data loss. Re-identification of users may be possible. Consider limiting
        /// usage and or removing SPII.
        SensitivityHigh = 30,
    }
}
/// A reference to a StoredInfoType to use with scanning.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoredType {
    /// Resource name of the requested `StoredInfoType`, for example
    /// `organizations/433245324/storedInfoTypes/432452342` or
    /// `projects/project-id/storedInfoTypes/432452342`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Timestamp indicating when the version of the `StoredInfoType` used for
    /// inspection was created. Output-only field, populated by the system.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Custom information type provided by the user. Used to find domain-specific
/// sensitive information configurable to the data in question.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomInfoType {
    /// CustomInfoType can either be a new infoType, or an extension of built-in
    /// infoType, when the name matches one of existing infoTypes and that infoType
    /// is specified in `InspectContent.info_types` field. Specifying the latter
    /// adds findings to the one detected by the system. If built-in info type is
    /// not specified in `InspectContent.info_types` list then the name is treated
    /// as a custom info type.
    #[prost(message, optional, tag="1")]
    pub info_type: ::core::option::Option<InfoType>,
    /// Likelihood to return for this CustomInfoType. This base value can be
    /// altered by a detection rule if the finding meets the criteria specified by
    /// the rule. Defaults to `VERY_LIKELY` if not specified.
    #[prost(enumeration="Likelihood", tag="6")]
    pub likelihood: i32,
    /// Set of detection rules to apply to all findings of this CustomInfoType.
    /// Rules are applied in order that they are specified. Not supported for the
    /// `surrogate_type` CustomInfoType.
    #[prost(message, repeated, tag="7")]
    pub detection_rules: ::prost::alloc::vec::Vec<custom_info_type::DetectionRule>,
    /// If set to EXCLUSION_TYPE_EXCLUDE this infoType will not cause a finding
    /// to be returned. It still can be used for rules matching.
    #[prost(enumeration="custom_info_type::ExclusionType", tag="8")]
    pub exclusion_type: i32,
    #[prost(oneof="custom_info_type::Type", tags="2, 3, 4, 5")]
    pub r#type: ::core::option::Option<custom_info_type::Type>,
}
/// Nested message and enum types in `CustomInfoType`.
pub mod custom_info_type {
    /// Custom information type based on a dictionary of words or phrases. This can
    /// be used to match sensitive information specific to the data, such as a list
    /// of employee IDs or job titles.
    ///
    /// Dictionary words are case-insensitive and all characters other than letters
    /// and digits in the unicode [Basic Multilingual
    /// Plane](<https://en.wikipedia.org/wiki/Plane_%28Unicode%29#Basic_Multilingual_Plane>)
    /// will be replaced with whitespace when scanning for matches, so the
    /// dictionary phrase "Sam Johnson" will match all three phrases "sam johnson",
    /// "Sam, Johnson", and "Sam (Johnson)". Additionally, the characters
    /// surrounding any match must be of a different type than the adjacent
    /// characters within the word, so letters must be next to non-letters and
    /// digits next to non-digits. For example, the dictionary word "jen" will
    /// match the first three letters of the text "jen123" but will return no
    /// matches for "jennifer".
    ///
    /// Dictionary words containing a large number of characters that are not
    /// letters or digits may result in unexpected findings because such characters
    /// are treated as whitespace. The
    /// \[limits\](<https://cloud.google.com/dlp/limits>) page contains details about
    /// the size limits of dictionaries. For dictionaries that do not fit within
    /// these constraints, consider using `LargeCustomDictionaryConfig` in the
    /// `StoredInfoType` API.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Dictionary {
        #[prost(oneof="dictionary::Source", tags="1, 3")]
        pub source: ::core::option::Option<dictionary::Source>,
    }
    /// Nested message and enum types in `Dictionary`.
    pub mod dictionary {
        /// Message defining a list of words or phrases to search for in the data.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct WordList {
            /// Words or phrases defining the dictionary. The dictionary must contain
            /// at least one phrase and every phrase must contain at least 2 characters
            /// that are letters or digits. \[required\]
            #[prost(string, repeated, tag="1")]
            pub words: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        }
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Source {
            /// List of words or phrases to search for.
            #[prost(message, tag="1")]
            WordList(WordList),
            /// Newline-delimited file of words in Cloud Storage. Only a single file
            /// is accepted.
            #[prost(message, tag="3")]
            CloudStoragePath(super::super::CloudStoragePath),
        }
    }
    /// Message defining a custom regular expression.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Regex {
        /// Pattern defining the regular expression. Its syntax
        /// (<https://github.com/google/re2/wiki/Syntax>) can be found under the
        /// google/re2 repository on GitHub.
        #[prost(string, tag="1")]
        pub pattern: ::prost::alloc::string::String,
        /// The index of the submatch to extract as findings. When not
        /// specified, the entire match is returned. No more than 3 may be included.
        #[prost(int32, repeated, tag="2")]
        pub group_indexes: ::prost::alloc::vec::Vec<i32>,
    }
    /// Message for detecting output from deidentification transformations
    /// such as
    /// \[`CryptoReplaceFfxFpeConfig`\](<https://cloud.google.com/dlp/docs/reference/rest/v2/organizations.deidentifyTemplates#cryptoreplaceffxfpeconfig>).
    /// These types of transformations are
    /// those that perform pseudonymization, thereby producing a "surrogate" as
    /// output. This should be used in conjunction with a field on the
    /// transformation such as `surrogate_info_type`. This CustomInfoType does
    /// not support the use of `detection_rules`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SurrogateType {
    }
    /// Deprecated; use `InspectionRuleSet` instead. Rule for modifying a
    /// `CustomInfoType` to alter behavior under certain circumstances, depending
    /// on the specific details of the rule. Not supported for the `surrogate_type`
    /// custom infoType.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DetectionRule {
        #[prost(oneof="detection_rule::Type", tags="1")]
        pub r#type: ::core::option::Option<detection_rule::Type>,
    }
    /// Nested message and enum types in `DetectionRule`.
    pub mod detection_rule {
        /// Message for specifying a window around a finding to apply a detection
        /// rule.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Proximity {
            /// Number of characters before the finding to consider. For tabular data,
            /// if you want to modify the likelihood of an entire column of findngs,
            /// set this to 1. For more information, see
            /// [Hotword example: Set the match likelihood of a table column]
            /// (<https://cloud.google.com/dlp/docs/creating-custom-infotypes-likelihood#match-column-values>).
            #[prost(int32, tag="1")]
            pub window_before: i32,
            /// Number of characters after the finding to consider.
            #[prost(int32, tag="2")]
            pub window_after: i32,
        }
        /// Message for specifying an adjustment to the likelihood of a finding as
        /// part of a detection rule.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct LikelihoodAdjustment {
            #[prost(oneof="likelihood_adjustment::Adjustment", tags="1, 2")]
            pub adjustment: ::core::option::Option<likelihood_adjustment::Adjustment>,
        }
        /// Nested message and enum types in `LikelihoodAdjustment`.
        pub mod likelihood_adjustment {
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Adjustment {
                /// Set the likelihood of a finding to a fixed value.
                #[prost(enumeration="super::super::super::Likelihood", tag="1")]
                FixedLikelihood(i32),
                /// Increase or decrease the likelihood by the specified number of
                /// levels. For example, if a finding would be `POSSIBLE` without the
                /// detection rule and `relative_likelihood` is 1, then it is upgraded to
                /// `LIKELY`, while a value of -1 would downgrade it to `UNLIKELY`.
                /// Likelihood may never drop below `VERY_UNLIKELY` or exceed
                /// `VERY_LIKELY`, so applying an adjustment of 1 followed by an
                /// adjustment of -1 when base likelihood is `VERY_LIKELY` will result in
                /// a final likelihood of `LIKELY`.
                #[prost(int32, tag="2")]
                RelativeLikelihood(i32),
            }
        }
        /// The rule that adjusts the likelihood of findings within a certain
        /// proximity of hotwords.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct HotwordRule {
            /// Regular expression pattern defining what qualifies as a hotword.
            #[prost(message, optional, tag="1")]
            pub hotword_regex: ::core::option::Option<super::Regex>,
            /// Range of characters within which the entire hotword must reside.
            /// The total length of the window cannot exceed 1000 characters.
            /// The finding itself will be included in the window, so that hotwords can
            /// be used to match substrings of the finding itself. Suppose you
            /// want Cloud DLP to promote the likelihood of the phone number
            /// regex "\(\d{3}\) \d{3}-\d{4}" if the area code is known to be the
            /// area code of a company's office. In this case, use the hotword regex
            /// "\(xxx\)", where "xxx" is the area code in question.
            ///
            /// For tabular data, if you want to modify the likelihood of an entire
            /// column of findngs, see
            /// [Hotword example: Set the match likelihood of a table column]
            /// (<https://cloud.google.com/dlp/docs/creating-custom-infotypes-likelihood#match-column-values>).
            #[prost(message, optional, tag="2")]
            pub proximity: ::core::option::Option<Proximity>,
            /// Likelihood adjustment to apply to all matching findings.
            #[prost(message, optional, tag="3")]
            pub likelihood_adjustment: ::core::option::Option<LikelihoodAdjustment>,
        }
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Type {
            /// Hotword-based detection rule.
            #[prost(message, tag="1")]
            HotwordRule(HotwordRule),
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ExclusionType {
        /// A finding of this custom info type will not be excluded from results.
        Unspecified = 0,
        /// A finding of this custom info type will be excluded from final results,
        /// but can still affect rule execution.
        Exclude = 1,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// A list of phrases to detect as a CustomInfoType.
        #[prost(message, tag="2")]
        Dictionary(Dictionary),
        /// Regular expression based CustomInfoType.
        #[prost(message, tag="3")]
        Regex(Regex),
        /// Message for detecting output from deidentification transformations that
        /// support reversing.
        #[prost(message, tag="4")]
        SurrogateType(SurrogateType),
        /// Load an existing `StoredInfoType` resource for use in
        /// `InspectDataSource`. Not currently supported in `InspectContent`.
        #[prost(message, tag="5")]
        StoredType(super::StoredType),
    }
}
/// General identifier of a data field in a storage service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldId {
    /// Name describing the field.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Datastore partition ID.
/// A partition ID identifies a grouping of entities. The grouping is always
/// by project and namespace, however the namespace ID may be empty.
///
/// A partition ID contains several dimensions:
/// project ID and namespace ID.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartitionId {
    /// The ID of the project to which the entities belong.
    #[prost(string, tag="2")]
    pub project_id: ::prost::alloc::string::String,
    /// If not empty, the ID of the namespace to which the entities belong.
    #[prost(string, tag="4")]
    pub namespace_id: ::prost::alloc::string::String,
}
/// A representation of a Datastore kind.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KindExpression {
    /// The name of the kind.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Options defining a data set within Google Cloud Datastore.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatastoreOptions {
    /// A partition ID identifies a grouping of entities. The grouping is always
    /// by project and namespace, however the namespace ID may be empty.
    #[prost(message, optional, tag="1")]
    pub partition_id: ::core::option::Option<PartitionId>,
    /// The kind to process.
    #[prost(message, optional, tag="2")]
    pub kind: ::core::option::Option<KindExpression>,
}
/// Message representing a set of files in a Cloud Storage bucket. Regular
/// expressions are used to allow fine-grained control over which files in the
/// bucket to include.
///
/// Included files are those that match at least one item in `include_regex` and
/// do not match any items in `exclude_regex`. Note that a file that matches
/// items from both lists will _not_ be included. For a match to occur, the
/// entire file path (i.e., everything in the url after the bucket name) must
/// match the regular expression.
///
/// For example, given the input `{bucket_name: "mybucket", include_regex:
/// \["directory1/.*"\], exclude_regex:
/// \["directory1/excluded.*"\]}`:
///
/// * `gs://mybucket/directory1/myfile` will be included
/// * `gs://mybucket/directory1/directory2/myfile` will be included (`.*` matches
/// across `/`)
/// * `gs://mybucket/directory0/directory1/myfile` will _not_ be included (the
/// full path doesn't match any items in `include_regex`)
/// * `gs://mybucket/directory1/excludedfile` will _not_ be included (the path
/// matches an item in `exclude_regex`)
///
/// If `include_regex` is left empty, it will match all files by default
/// (this is equivalent to setting `include_regex: \[".*"\]`).
///
/// Some other common use cases:
///
/// * `{bucket_name: "mybucket", exclude_regex: \[".*\.pdf"\]}` will include all
/// files in `mybucket` except for .pdf files
/// * `{bucket_name: "mybucket", include_regex: \["directory/[^/]+"\]}` will
/// include all files directly under `gs://mybucket/directory/`, without matching
/// across `/`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudStorageRegexFileSet {
    /// The name of a Cloud Storage bucket. Required.
    #[prost(string, tag="1")]
    pub bucket_name: ::prost::alloc::string::String,
    /// A list of regular expressions matching file paths to include. All files in
    /// the bucket that match at least one of these regular expressions will be
    /// included in the set of files, except for those that also match an item in
    /// `exclude_regex`. Leaving this field empty will match all files by default
    /// (this is equivalent to including `.*` in the list).
    ///
    /// Regular expressions use RE2
    /// \[syntax\](<https://github.com/google/re2/wiki/Syntax>); a guide can be found
    /// under the google/re2 repository on GitHub.
    #[prost(string, repeated, tag="2")]
    pub include_regex: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of regular expressions matching file paths to exclude. All files in
    /// the bucket that match at least one of these regular expressions will be
    /// excluded from the scan.
    ///
    /// Regular expressions use RE2
    /// \[syntax\](<https://github.com/google/re2/wiki/Syntax>); a guide can be found
    /// under the google/re2 repository on GitHub.
    #[prost(string, repeated, tag="3")]
    pub exclude_regex: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Options defining a file or a set of files within a Cloud Storage
/// bucket.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudStorageOptions {
    /// The set of one or more files to scan.
    #[prost(message, optional, tag="1")]
    pub file_set: ::core::option::Option<cloud_storage_options::FileSet>,
    /// Max number of bytes to scan from a file. If a scanned file's size is bigger
    /// than this value then the rest of the bytes are omitted. Only one
    /// of bytes_limit_per_file and bytes_limit_per_file_percent can be specified.
    /// Cannot be set if de-identification is requested.
    #[prost(int64, tag="4")]
    pub bytes_limit_per_file: i64,
    /// Max percentage of bytes to scan from a file. The rest are omitted. The
    /// number of bytes scanned is rounded down. Must be between 0 and 100,
    /// inclusively. Both 0 and 100 means no limit. Defaults to 0. Only one
    /// of bytes_limit_per_file and bytes_limit_per_file_percent can be specified.
    /// Cannot be set if de-identification is requested.
    #[prost(int32, tag="8")]
    pub bytes_limit_per_file_percent: i32,
    /// List of file type groups to include in the scan.
    /// If empty, all files are scanned and available data format processors
    /// are applied. In addition, the binary content of the selected files
    /// is always scanned as well.
    /// Images are scanned only as binary if the specified region
    /// does not support image inspection and no file_types were specified.
    /// Image inspection is restricted to 'global', 'us', 'asia', and 'europe'.
    #[prost(enumeration="FileType", repeated, tag="5")]
    pub file_types: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration="cloud_storage_options::SampleMethod", tag="6")]
    pub sample_method: i32,
    /// Limits the number of files to scan to this percentage of the input FileSet.
    /// Number of files scanned is rounded down. Must be between 0 and 100,
    /// inclusively. Both 0 and 100 means no limit. Defaults to 0.
    #[prost(int32, tag="7")]
    pub files_limit_percent: i32,
}
/// Nested message and enum types in `CloudStorageOptions`.
pub mod cloud_storage_options {
    /// Set of files to scan.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FileSet {
        /// The Cloud Storage url of the file(s) to scan, in the format
        /// `gs://<bucket>/<path>`. Trailing wildcard in the path is allowed.
        ///
        /// If the url ends in a trailing slash, the bucket or directory represented
        /// by the url will be scanned non-recursively (content in sub-directories
        /// will not be scanned). This means that `gs://mybucket/` is equivalent to
        /// `gs://mybucket/*`, and `gs://mybucket/directory/` is equivalent to
        /// `gs://mybucket/directory/*`.
        ///
        /// Exactly one of `url` or `regex_file_set` must be set.
        #[prost(string, tag="1")]
        pub url: ::prost::alloc::string::String,
        /// The regex-filtered set of files to scan. Exactly one of `url` or
        /// `regex_file_set` must be set.
        #[prost(message, optional, tag="2")]
        pub regex_file_set: ::core::option::Option<super::CloudStorageRegexFileSet>,
    }
    /// How to sample bytes if not all bytes are scanned. Meaningful only when used
    /// in conjunction with bytes_limit_per_file. If not specified, scanning would
    /// start from the top.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SampleMethod {
        Unspecified = 0,
        /// Scan from the top (default).
        Top = 1,
        /// For each file larger than bytes_limit_per_file, randomly pick the offset
        /// to start scanning. The scanned bytes are contiguous.
        RandomStart = 2,
    }
}
/// Message representing a set of files in Cloud Storage.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudStorageFileSet {
    /// The url, in the format `gs://<bucket>/<path>`. Trailing wildcard in the
    /// path is allowed.
    #[prost(string, tag="1")]
    pub url: ::prost::alloc::string::String,
}
/// Message representing a single file or path in Cloud Storage.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudStoragePath {
    /// A url representing a file or path (no wildcards) in Cloud Storage.
    /// Example: gs://\[BUCKET_NAME\]/dictionary.txt
    #[prost(string, tag="1")]
    pub path: ::prost::alloc::string::String,
}
/// Options defining BigQuery table and row identifiers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryOptions {
    /// Complete BigQuery table reference.
    #[prost(message, optional, tag="1")]
    pub table_reference: ::core::option::Option<BigQueryTable>,
    /// Table fields that may uniquely identify a row within the table. When
    /// `actions.saveFindings.outputConfig.table` is specified, the values of
    /// columns specified here are available in the output table under
    /// `location.content_locations.record_location.record_key.id_values`. Nested
    /// fields such as `person.birthdate.year` are allowed.
    #[prost(message, repeated, tag="2")]
    pub identifying_fields: ::prost::alloc::vec::Vec<FieldId>,
    /// Max number of rows to scan. If the table has more rows than this value, the
    /// rest of the rows are omitted. If not set, or if set to 0, all rows will be
    /// scanned. Only one of rows_limit and rows_limit_percent can be specified.
    /// Cannot be used in conjunction with TimespanConfig.
    #[prost(int64, tag="3")]
    pub rows_limit: i64,
    /// Max percentage of rows to scan. The rest are omitted. The number of rows
    /// scanned is rounded down. Must be between 0 and 100, inclusively. Both 0 and
    /// 100 means no limit. Defaults to 0. Only one of rows_limit and
    /// rows_limit_percent can be specified. Cannot be used in conjunction with
    /// TimespanConfig.
    #[prost(int32, tag="6")]
    pub rows_limit_percent: i32,
    #[prost(enumeration="big_query_options::SampleMethod", tag="4")]
    pub sample_method: i32,
    /// References to fields excluded from scanning. This allows you to skip
    /// inspection of entire columns which you know have no findings.
    #[prost(message, repeated, tag="5")]
    pub excluded_fields: ::prost::alloc::vec::Vec<FieldId>,
    /// Limit scanning only to these fields.
    #[prost(message, repeated, tag="7")]
    pub included_fields: ::prost::alloc::vec::Vec<FieldId>,
}
/// Nested message and enum types in `BigQueryOptions`.
pub mod big_query_options {
    /// How to sample rows if not all rows are scanned. Meaningful only when used
    /// in conjunction with either rows_limit or rows_limit_percent. If not
    /// specified, rows are scanned in the order BigQuery reads them.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SampleMethod {
        Unspecified = 0,
        /// Scan groups of rows in the order BigQuery provides (default). Multiple
        /// groups of rows may be scanned in parallel, so results may not appear in
        /// the same order the rows are read.
        Top = 1,
        /// Randomly pick groups of rows to scan.
        RandomStart = 2,
    }
}
/// Shared message indicating Cloud storage type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageConfig {
    #[prost(message, optional, tag="6")]
    pub timespan_config: ::core::option::Option<storage_config::TimespanConfig>,
    #[prost(oneof="storage_config::Type", tags="2, 3, 4, 9")]
    pub r#type: ::core::option::Option<storage_config::Type>,
}
/// Nested message and enum types in `StorageConfig`.
pub mod storage_config {
    /// Configuration of the timespan of the items to include in scanning.
    /// Currently only supported when inspecting Cloud Storage and BigQuery.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TimespanConfig {
        /// Exclude files, tables, or rows older than this value.
        /// If not set, no lower time limit is applied.
        #[prost(message, optional, tag="1")]
        pub start_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Exclude files, tables, or rows newer than this value.
        /// If not set, no upper time limit is applied.
        #[prost(message, optional, tag="2")]
        pub end_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Specification of the field containing the timestamp of scanned items.
        /// Used for data sources like Datastore and BigQuery.
        ///
        /// <b>For BigQuery</b>
        ///
        /// If this value is not specified and the table was modified between the
        /// given start and end times, the entire table will be scanned. If this
        /// value is specified, then rows are filtered based on the given start and
        /// end times. Rows with a `NULL` value in the provided BigQuery column are
        /// skipped.
        /// Valid data types of the provided BigQuery column are: `INTEGER`, `DATE`,
        /// `TIMESTAMP`, and `DATETIME`.
        ///
        /// If your BigQuery table is [partitioned at ingestion
        /// time](<https://cloud.google.com/bigquery/docs/partitioned-tables#ingestion_time>),
        /// you can use any of the following pseudo-columns as your timestamp field.
        /// When used with Cloud DLP, these pseudo-column names are case sensitive.
        ///
        /// <ul>
        /// <li><code>_PARTITIONTIME</code></li>
        /// <li><code>_PARTITIONDATE</code></li>
        /// <li><code>_PARTITION_LOAD_TIME</code></li>
        /// </ul>
        ///
        /// <b>For Datastore</b>
        ///
        /// If this value is specified, then entities are filtered based on the given
        /// start and end times. If an entity does not contain the provided timestamp
        /// property or contains empty or invalid values, then it is included.
        /// Valid data types of the provided timestamp property are: `TIMESTAMP`.
        ///
        /// See the
        /// [known issue](<https://cloud.google.com/dlp/docs/known-issues#bq-timespan>)
        /// related to this operation.
        #[prost(message, optional, tag="3")]
        pub timestamp_field: ::core::option::Option<super::FieldId>,
        /// When the job is started by a JobTrigger we will automatically figure out
        /// a valid start_time to avoid scanning files that have not been modified
        /// since the last time the JobTrigger executed. This will be based on the
        /// time of the execution of the last run of the JobTrigger or the timespan
        /// end_time used in the last run of the JobTrigger.
        #[prost(bool, tag="4")]
        pub enable_auto_population_of_timespan_config: bool,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// Google Cloud Datastore options.
        #[prost(message, tag="2")]
        DatastoreOptions(super::DatastoreOptions),
        /// Cloud Storage options.
        #[prost(message, tag="3")]
        CloudStorageOptions(super::CloudStorageOptions),
        /// BigQuery options.
        #[prost(message, tag="4")]
        BigQueryOptions(super::BigQueryOptions),
        /// Hybrid inspection options.
        #[prost(message, tag="9")]
        HybridOptions(super::HybridOptions),
    }
}
/// Configuration to control jobs where the content being inspected is outside
/// of Google Cloud Platform.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HybridOptions {
    /// A short description of where the data is coming from. Will be stored once
    /// in the job. 256 max length.
    #[prost(string, tag="1")]
    pub description: ::prost::alloc::string::String,
    /// These are labels that each inspection request must include within their
    /// 'finding_labels' map. Request may contain others, but any missing one of
    /// these will be rejected.
    ///
    /// Label keys must be between 1 and 63 characters long and must conform
    /// to the following regular expression: `\[a-z]([-a-z0-9]*[a-z0-9\])?`.
    ///
    /// No more than 10 keys can be required.
    #[prost(string, repeated, tag="2")]
    pub required_finding_label_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// To organize findings, these labels will be added to each finding.
    ///
    /// Label keys must be between 1 and 63 characters long and must conform
    /// to the following regular expression: `\[a-z]([-a-z0-9]*[a-z0-9\])?`.
    ///
    /// Label values must be between 0 and 63 characters long and must conform
    /// to the regular expression `(\[a-z]([-a-z0-9]*[a-z0-9\])?)?`.
    ///
    /// No more than 10 labels can be associated with a given finding.
    ///
    /// Examples:
    /// * `"environment" : "production"`
    /// * `"pipeline" : "etl"`
    #[prost(btree_map="string, string", tag="3")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// If the container is a table, additional information to make findings
    /// meaningful such as the columns that are primary keys.
    #[prost(message, optional, tag="4")]
    pub table_options: ::core::option::Option<TableOptions>,
}
/// Row key for identifying a record in BigQuery table.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryKey {
    /// Complete BigQuery table reference.
    #[prost(message, optional, tag="1")]
    pub table_reference: ::core::option::Option<BigQueryTable>,
    /// Row number inferred at the time the table was scanned. This value is
    /// nondeterministic, cannot be queried, and may be null for inspection
    /// jobs. To locate findings within a table, specify
    /// `inspect_job.storage_config.big_query_options.identifying_fields` in
    /// `CreateDlpJobRequest`.
    #[prost(int64, tag="2")]
    pub row_number: i64,
}
/// Record key for a finding in Cloud Datastore.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatastoreKey {
    /// Datastore entity key.
    #[prost(message, optional, tag="1")]
    pub entity_key: ::core::option::Option<Key>,
}
/// A unique identifier for a Datastore entity.
/// If a key's partition ID or any of its path kinds or names are
/// reserved/read-only, the key is reserved/read-only.
/// A reserved/read-only key is forbidden in certain documented contexts.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Key {
    /// Entities are partitioned into subsets, currently identified by a project
    /// ID and namespace ID.
    /// Queries are scoped to a single partition.
    #[prost(message, optional, tag="1")]
    pub partition_id: ::core::option::Option<PartitionId>,
    /// The entity path.
    /// An entity path consists of one or more elements composed of a kind and a
    /// string or numerical identifier, which identify entities. The first
    /// element identifies a _root entity_, the second element identifies
    /// a _child_ of the root entity, the third element identifies a child of the
    /// second entity, and so forth. The entities identified by all prefixes of
    /// the path are called the element's _ancestors_.
    ///
    /// A path can never be empty, and a path can have at most 100 elements.
    #[prost(message, repeated, tag="2")]
    pub path: ::prost::alloc::vec::Vec<key::PathElement>,
}
/// Nested message and enum types in `Key`.
pub mod key {
    /// A (kind, ID/name) pair used to construct a key path.
    ///
    /// If either name or ID is set, the element is complete.
    /// If neither is set, the element is incomplete.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PathElement {
        /// The kind of the entity.
        /// A kind matching regex `__.*__` is reserved/read-only.
        /// A kind must not contain more than 1500 bytes when UTF-8 encoded.
        /// Cannot be `""`.
        #[prost(string, tag="1")]
        pub kind: ::prost::alloc::string::String,
        /// The type of ID.
        #[prost(oneof="path_element::IdType", tags="2, 3")]
        pub id_type: ::core::option::Option<path_element::IdType>,
    }
    /// Nested message and enum types in `PathElement`.
    pub mod path_element {
        /// The type of ID.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum IdType {
            /// The auto-allocated ID of the entity.
            /// Never equal to zero. Values less than zero are discouraged and may not
            /// be supported in the future.
            #[prost(int64, tag="2")]
            Id(i64),
            /// The name of the entity.
            /// A name matching regex `__.*__` is reserved/read-only.
            /// A name must not be more than 1500 bytes when UTF-8 encoded.
            /// Cannot be `""`.
            #[prost(string, tag="3")]
            Name(::prost::alloc::string::String),
        }
    }
}
/// Message for a unique key indicating a record that contains a finding.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordKey {
    /// Values of identifying columns in the given row. Order of values matches
    /// the order of `identifying_fields` specified in the scanning request.
    #[prost(string, repeated, tag="5")]
    pub id_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(oneof="record_key::Type", tags="2, 3")]
    pub r#type: ::core::option::Option<record_key::Type>,
}
/// Nested message and enum types in `RecordKey`.
pub mod record_key {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag="2")]
        DatastoreKey(super::DatastoreKey),
        #[prost(message, tag="3")]
        BigQueryKey(super::BigQueryKey),
    }
}
/// Message defining the location of a BigQuery table. A table is uniquely
/// identified  by its project_id, dataset_id, and table_name. Within a query
/// a table is often referenced with a string in the format of:
/// `<project_id>:<dataset_id>.<table_id>` or
/// `<project_id>.<dataset_id>.<table_id>`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryTable {
    /// The Google Cloud Platform project ID of the project containing the table.
    /// If omitted, project ID is inferred from the API call.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// Dataset ID of the table.
    #[prost(string, tag="2")]
    pub dataset_id: ::prost::alloc::string::String,
    /// Name of the table.
    #[prost(string, tag="3")]
    pub table_id: ::prost::alloc::string::String,
}
/// Message defining a field of a BigQuery table.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryField {
    /// Source table of the field.
    #[prost(message, optional, tag="1")]
    pub table: ::core::option::Option<BigQueryTable>,
    /// Designated field in the BigQuery table.
    #[prost(message, optional, tag="2")]
    pub field: ::core::option::Option<FieldId>,
}
/// An entity in a dataset is a field or set of fields that correspond to a
/// single person. For example, in medical records the `EntityId` might be a
/// patient identifier, or for financial records it might be an account
/// identifier. This message is used when generalizations or analysis must take
/// into account that multiple rows correspond to the same entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityId {
    /// Composite key indicating which field contains the entity identifier.
    #[prost(message, optional, tag="1")]
    pub field: ::core::option::Option<FieldId>,
}
/// Instructions regarding the table content being inspected.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableOptions {
    /// The columns that are the primary keys for table objects included in
    /// ContentItem. A copy of this cell's value will stored alongside alongside
    /// each finding so that the finding can be traced to the specific row it came
    /// from. No more than 3 may be provided.
    #[prost(message, repeated, tag="1")]
    pub identifying_fields: ::prost::alloc::vec::Vec<FieldId>,
}
/// Categorization of results based on how likely they are to represent a match,
/// based on the number of elements they contain which imply a match.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Likelihood {
    /// Default value; same as POSSIBLE.
    Unspecified = 0,
    /// Few matching elements.
    VeryUnlikely = 1,
    Unlikely = 2,
    /// Some matching elements.
    Possible = 3,
    Likely = 4,
    /// Many matching elements.
    VeryLikely = 5,
}
/// Definitions of file type groups to scan. New types will be added to this
/// list.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FileType {
    /// Includes all files.
    Unspecified = 0,
    /// Includes all file extensions not covered by another entry. Binary
    /// scanning attempts to convert the content of the file to utf_8 to scan
    /// the file.
    /// If you wish to avoid this fall back, specify one or more of the other
    /// FileType's in your storage scan.
    BinaryFile = 1,
    /// Included file extensions:
    ///   asc,asp, aspx, brf, c, cc,cfm, cgi, cpp, csv, cxx, c++, cs, css, dart,
    ///   dat, dot, eml,, epbub, ged, go, h, hh, hpp, hxx, h++, hs, html, htm,
    ///   mkd, markdown, m, ml, mli, perl, pl, plist, pm, php, phtml, pht,
    ///   properties, py, pyw, rb, rbw, rs, rss,  rc, scala, sh, sql, swift, tex,
    ///   shtml, shtm, xhtml, lhs, ics, ini, java, js, json, kix, kml, ocaml, md,
    ///   txt, text, tsv, vb, vcard, vcs, wml, xcodeproj, xml, xsl, xsd, yml, yaml.
    TextFile = 2,
    /// Included file extensions:
    ///   bmp, gif, jpg, jpeg, jpe, png.
    /// bytes_limit_per_file has no effect on image files.
    /// Image inspection is restricted to 'global', 'us', 'asia', and 'europe'.
    Image = 3,
    /// Word files >30 MB will be scanned as binary files.
    /// Included file extensions:
    ///   docx, dotx, docm, dotm
    Word = 5,
    /// PDF files >30 MB will be scanned as binary files.
    /// Included file extensions:
    ///   pdf
    Pdf = 6,
    /// Included file extensions:
    ///   avro
    Avro = 7,
    /// Included file extensions:
    ///   csv
    Csv = 8,
    /// Included file extensions:
    ///   tsv
    Tsv = 9,
    /// Powerpoint files >30 MB will be scanned as binary files.
    /// Included file extensions:
    ///   pptx, pptm, potx, potm, pot
    Powerpoint = 11,
    /// Excel files >30 MB will be scanned as binary files.
    /// Included file extensions:
    ///   xlsx, xlsm, xltx, xltm
    Excel = 12,
}
/// List of excluded infoTypes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExcludeInfoTypes {
    /// InfoType list in ExclusionRule rule drops a finding when it overlaps or
    /// contained within with a finding of an infoType from this list. For
    /// example, for `InspectionRuleSet.info_types` containing "PHONE_NUMBER"` and
    /// `exclusion_rule` containing `exclude_info_types.info_types` with
    /// "EMAIL_ADDRESS" the phone number findings are dropped if they overlap
    /// with EMAIL_ADDRESS finding.
    /// That leads to "555-222-2222@example.org" to generate only a single
    /// finding, namely email address.
    #[prost(message, repeated, tag="1")]
    pub info_types: ::prost::alloc::vec::Vec<InfoType>,
}
/// The rule that specifies conditions when findings of infoTypes specified in
/// `InspectionRuleSet` are removed from results.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExclusionRule {
    /// How the rule is applied, see MatchingType documentation for details.
    #[prost(enumeration="MatchingType", tag="4")]
    pub matching_type: i32,
    /// Exclusion rule types.
    #[prost(oneof="exclusion_rule::Type", tags="1, 2, 3")]
    pub r#type: ::core::option::Option<exclusion_rule::Type>,
}
/// Nested message and enum types in `ExclusionRule`.
pub mod exclusion_rule {
    /// Exclusion rule types.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// Dictionary which defines the rule.
        #[prost(message, tag="1")]
        Dictionary(super::custom_info_type::Dictionary),
        /// Regular expression which defines the rule.
        #[prost(message, tag="2")]
        Regex(super::custom_info_type::Regex),
        /// Set of infoTypes for which findings would affect this rule.
        #[prost(message, tag="3")]
        ExcludeInfoTypes(super::ExcludeInfoTypes),
    }
}
/// A single inspection rule to be applied to infoTypes, specified in
/// `InspectionRuleSet`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InspectionRule {
    /// Inspection rule types.
    #[prost(oneof="inspection_rule::Type", tags="1, 2")]
    pub r#type: ::core::option::Option<inspection_rule::Type>,
}
/// Nested message and enum types in `InspectionRule`.
pub mod inspection_rule {
    /// Inspection rule types.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// Hotword-based detection rule.
        #[prost(message, tag="1")]
        HotwordRule(super::custom_info_type::detection_rule::HotwordRule),
        /// Exclusion rule.
        #[prost(message, tag="2")]
        ExclusionRule(super::ExclusionRule),
    }
}
/// Rule set for modifying a set of infoTypes to alter behavior under certain
/// circumstances, depending on the specific details of the rules within the set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InspectionRuleSet {
    /// List of infoTypes this rule set is applied to.
    #[prost(message, repeated, tag="1")]
    pub info_types: ::prost::alloc::vec::Vec<InfoType>,
    /// Set of rules to be applied to infoTypes. The rules are applied in order.
    #[prost(message, repeated, tag="2")]
    pub rules: ::prost::alloc::vec::Vec<InspectionRule>,
}
/// Configuration description of the scanning process.
/// When used with redactContent only info_types and min_likelihood are currently
/// used.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InspectConfig {
    /// Restricts what info_types to look for. The values must correspond to
    /// InfoType values returned by ListInfoTypes or listed at
    /// <https://cloud.google.com/dlp/docs/infotypes-reference.>
    ///
    /// When no InfoTypes or CustomInfoTypes are specified in a request, the
    /// system may automatically choose what detectors to run. By default this may
    /// be all types, but may change over time as detectors are updated.
    ///
    /// If you need precise control and predictability as to what detectors are
    /// run you should specify specific InfoTypes listed in the reference,
    /// otherwise a default list will be used, which may change over time.
    #[prost(message, repeated, tag="1")]
    pub info_types: ::prost::alloc::vec::Vec<InfoType>,
    /// Only returns findings equal or above this threshold. The default is
    /// POSSIBLE.
    /// See <https://cloud.google.com/dlp/docs/likelihood> to learn more.
    #[prost(enumeration="Likelihood", tag="2")]
    pub min_likelihood: i32,
    /// Configuration to control the number of findings returned.
    /// This is not used for data profiling.
    ///
    /// When redacting sensitive data from images, finding limits don't apply. They
    /// can cause unexpected or inconsistent results, where only some data is
    /// redacted. Don't include finding limits in
    /// \[RedactImage][google.privacy.dlp.v2.DlpService.RedactImage\]
    /// requests. Otherwise, Cloud DLP returns an error.
    #[prost(message, optional, tag="3")]
    pub limits: ::core::option::Option<inspect_config::FindingLimits>,
    /// When true, a contextual quote from the data that triggered a finding is
    /// included in the response; see \[Finding.quote][google.privacy.dlp.v2.Finding.quote\].
    /// This is not used for data profiling.
    #[prost(bool, tag="4")]
    pub include_quote: bool,
    /// When true, excludes type information of the findings.
    /// This is not used for data profiling.
    #[prost(bool, tag="5")]
    pub exclude_info_types: bool,
    /// CustomInfoTypes provided by the user. See
    /// <https://cloud.google.com/dlp/docs/creating-custom-infotypes> to learn more.
    #[prost(message, repeated, tag="6")]
    pub custom_info_types: ::prost::alloc::vec::Vec<CustomInfoType>,
    /// Deprecated and unused.
    #[prost(enumeration="ContentOption", repeated, tag="8")]
    pub content_options: ::prost::alloc::vec::Vec<i32>,
    /// Set of rules to apply to the findings for this InspectConfig.
    /// Exclusion rules, contained in the set are executed in the end, other
    /// rules are executed in the order they are specified for each info type.
    #[prost(message, repeated, tag="10")]
    pub rule_set: ::prost::alloc::vec::Vec<InspectionRuleSet>,
}
/// Nested message and enum types in `InspectConfig`.
pub mod inspect_config {
    /// Configuration to control the number of findings returned for inspection.
    /// This is not used for de-identification or data profiling.
    ///
    /// When redacting sensitive data from images, finding limits don't apply. They
    /// can cause unexpected or inconsistent results, where only some data is
    /// redacted. Don't include finding limits in
    /// \[RedactImage][google.privacy.dlp.v2.DlpService.RedactImage\]
    /// requests. Otherwise, Cloud DLP returns an error.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FindingLimits {
        /// Max number of findings that will be returned for each item scanned.
        /// When set within `InspectJobConfig`,
        /// the maximum returned is 2000 regardless if this is set higher.
        /// When set within `InspectContentRequest`, this field is ignored.
        #[prost(int32, tag="1")]
        pub max_findings_per_item: i32,
        /// Max number of findings that will be returned per request/job.
        /// When set within `InspectContentRequest`, the maximum returned is 2000
        /// regardless if this is set higher.
        #[prost(int32, tag="2")]
        pub max_findings_per_request: i32,
        /// Configuration of findings limit given for specified infoTypes.
        #[prost(message, repeated, tag="3")]
        pub max_findings_per_info_type: ::prost::alloc::vec::Vec<finding_limits::InfoTypeLimit>,
    }
    /// Nested message and enum types in `FindingLimits`.
    pub mod finding_limits {
        /// Max findings configuration per infoType, per content item or long
        /// running DlpJob.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct InfoTypeLimit {
            /// Type of information the findings limit applies to. Only one limit per
            /// info_type should be provided. If InfoTypeLimit does not have an
            /// info_type, the DLP API applies the limit against all info_types that
            /// are found but not specified in another InfoTypeLimit.
            #[prost(message, optional, tag="1")]
            pub info_type: ::core::option::Option<super::super::InfoType>,
            /// Max findings limit for the given infoType.
            #[prost(int32, tag="2")]
            pub max_findings: i32,
        }
    }
}
/// Container for bytes to inspect or redact.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ByteContentItem {
    /// The type of data stored in the bytes string. Default will be TEXT_UTF8.
    #[prost(enumeration="byte_content_item::BytesType", tag="1")]
    pub r#type: i32,
    /// Content data to inspect or redact.
    #[prost(bytes="bytes", tag="2")]
    pub data: ::prost::bytes::Bytes,
}
/// Nested message and enum types in `ByteContentItem`.
pub mod byte_content_item {
    /// The type of data being sent for inspection. To learn more, see
    /// [Supported file
    /// types](<https://cloud.google.com/dlp/docs/supported-file-types>).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BytesType {
        /// Unused
        Unspecified = 0,
        /// Any image type.
        Image = 6,
        /// jpeg
        ImageJpeg = 1,
        /// bmp
        ImageBmp = 2,
        /// png
        ImagePng = 3,
        /// svg
        ImageSvg = 4,
        /// plain text
        TextUtf8 = 5,
        /// docx, docm, dotx, dotm
        WordDocument = 7,
        /// pdf
        Pdf = 8,
        /// pptx, pptm, potx, potm, pot
        PowerpointDocument = 9,
        /// xlsx, xlsm, xltx, xltm
        ExcelDocument = 10,
        /// avro
        Avro = 11,
        /// csv
        Csv = 12,
        /// tsv
        Tsv = 13,
    }
}
/// Container structure for the content to inspect.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentItem {
    /// Data of the item either in the byte array or UTF-8 string form, or table.
    #[prost(oneof="content_item::DataItem", tags="3, 4, 5")]
    pub data_item: ::core::option::Option<content_item::DataItem>,
}
/// Nested message and enum types in `ContentItem`.
pub mod content_item {
    /// Data of the item either in the byte array or UTF-8 string form, or table.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DataItem {
        /// String data to inspect or redact.
        #[prost(string, tag="3")]
        Value(::prost::alloc::string::String),
        /// Structured content for inspection. See
        /// <https://cloud.google.com/dlp/docs/inspecting-text#inspecting_a_table> to
        /// learn more.
        #[prost(message, tag="4")]
        Table(super::Table),
        /// Content data to inspect or redact. Replaces `type` and `data`.
        #[prost(message, tag="5")]
        ByteItem(super::ByteContentItem),
    }
}
/// Structured content to inspect. Up to 50,000 `Value`s per request allowed. See
/// <https://cloud.google.com/dlp/docs/inspecting-structured-text#inspecting_a_table>
/// to learn more.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Table {
    /// Headers of the table.
    #[prost(message, repeated, tag="1")]
    pub headers: ::prost::alloc::vec::Vec<FieldId>,
    /// Rows of the table.
    #[prost(message, repeated, tag="2")]
    pub rows: ::prost::alloc::vec::Vec<table::Row>,
}
/// Nested message and enum types in `Table`.
pub mod table {
    /// Values of the row.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Row {
        /// Individual cells.
        #[prost(message, repeated, tag="1")]
        pub values: ::prost::alloc::vec::Vec<super::Value>,
    }
}
/// All the findings for a single scanned item.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InspectResult {
    /// List of findings for an item.
    #[prost(message, repeated, tag="1")]
    pub findings: ::prost::alloc::vec::Vec<Finding>,
    /// If true, then this item might have more findings than were returned,
    /// and the findings returned are an arbitrary subset of all findings.
    /// The findings list might be truncated because the input items were too
    /// large, or because the server reached the maximum amount of resources
    /// allowed for a single API call. For best results, divide the input into
    /// smaller batches.
    #[prost(bool, tag="2")]
    pub findings_truncated: bool,
}
/// Represents a piece of potentially sensitive content.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Finding {
    /// Resource name in format
    /// projects/{project}/locations/{location}/findings/{finding} Populated only
    /// when viewing persisted findings.
    #[prost(string, tag="14")]
    pub name: ::prost::alloc::string::String,
    /// The content that was found. Even if the content is not textual, it
    /// may be converted to a textual representation here.
    /// Provided if `include_quote` is true and the finding is
    /// less than or equal to 4096 bytes long. If the finding exceeds 4096 bytes
    /// in length, the quote may be omitted.
    #[prost(string, tag="1")]
    pub quote: ::prost::alloc::string::String,
    /// The type of content that might have been found.
    /// Provided if `excluded_types` is false.
    #[prost(message, optional, tag="2")]
    pub info_type: ::core::option::Option<InfoType>,
    /// Confidence of how likely it is that the `info_type` is correct.
    #[prost(enumeration="Likelihood", tag="3")]
    pub likelihood: i32,
    /// Where the content was found.
    #[prost(message, optional, tag="4")]
    pub location: ::core::option::Option<Location>,
    /// Timestamp when finding was detected.
    #[prost(message, optional, tag="6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Contains data parsed from quotes. Only populated if include_quote was set
    /// to true and a supported infoType was requested. Currently supported
    /// infoTypes: DATE, DATE_OF_BIRTH and TIME.
    #[prost(message, optional, tag="7")]
    pub quote_info: ::core::option::Option<QuoteInfo>,
    /// The job that stored the finding.
    #[prost(string, tag="8")]
    pub resource_name: ::prost::alloc::string::String,
    /// Job trigger name, if applicable, for this finding.
    #[prost(string, tag="9")]
    pub trigger_name: ::prost::alloc::string::String,
    /// The labels associated with this `Finding`.
    ///
    /// Label keys must be between 1 and 63 characters long and must conform
    /// to the following regular expression: `\[a-z]([-a-z0-9]*[a-z0-9\])?`.
    ///
    /// Label values must be between 0 and 63 characters long and must conform
    /// to the regular expression `(\[a-z]([-a-z0-9]*[a-z0-9\])?)?`.
    ///
    /// No more than 10 labels can be associated with a given finding.
    ///
    /// Examples:
    /// * `"environment" : "production"`
    /// * `"pipeline" : "etl"`
    #[prost(btree_map="string, string", tag="10")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Time the job started that produced this finding.
    #[prost(message, optional, tag="11")]
    pub job_create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The job that stored the finding.
    #[prost(string, tag="13")]
    pub job_name: ::prost::alloc::string::String,
    /// The unique finding id.
    #[prost(string, tag="15")]
    pub finding_id: ::prost::alloc::string::String,
}
/// Specifies the location of the finding.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    /// Zero-based byte offsets delimiting the finding.
    /// These are relative to the finding's containing element.
    /// Note that when the content is not textual, this references
    /// the UTF-8 encoded textual representation of the content.
    /// Omitted if content is an image.
    #[prost(message, optional, tag="1")]
    pub byte_range: ::core::option::Option<Range>,
    /// Unicode character offsets delimiting the finding.
    /// These are relative to the finding's containing element.
    /// Provided when the content is text.
    #[prost(message, optional, tag="2")]
    pub codepoint_range: ::core::option::Option<Range>,
    /// List of nested objects pointing to the precise location of the finding
    /// within the file or record.
    #[prost(message, repeated, tag="7")]
    pub content_locations: ::prost::alloc::vec::Vec<ContentLocation>,
    /// Information about the container where this finding occurred, if available.
    #[prost(message, optional, tag="8")]
    pub container: ::core::option::Option<Container>,
}
/// Precise location of the finding within a document, record, image, or metadata
/// container.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentLocation {
    /// Name of the container where the finding is located.
    /// The top level name is the source file name or table name. Names of some
    /// common storage containers are formatted as follows:
    ///
    /// * BigQuery tables:  `{project_id}:{dataset_id}.{table_id}`
    /// * Cloud Storage files: `gs://{bucket}/{path}`
    /// * Datastore namespace: {namespace}
    ///
    /// Nested names could be absent if the embedded object has no string
    /// identifier (for example, an image contained within a document).
    #[prost(string, tag="1")]
    pub container_name: ::prost::alloc::string::String,
    /// Finding container modification timestamp, if applicable. For Cloud Storage,
    /// this field contains the last file modification timestamp. For a BigQuery
    /// table, this field contains the last_modified_time property. For Datastore,
    /// this field isn't populated.
    #[prost(message, optional, tag="6")]
    pub container_timestamp: ::core::option::Option<::prost_types::Timestamp>,
    /// Finding container version, if available
    /// ("generation" for Cloud Storage).
    #[prost(string, tag="7")]
    pub container_version: ::prost::alloc::string::String,
    /// Type of the container within the file with location of the finding.
    #[prost(oneof="content_location::Location", tags="2, 3, 5, 8")]
    pub location: ::core::option::Option<content_location::Location>,
}
/// Nested message and enum types in `ContentLocation`.
pub mod content_location {
    /// Type of the container within the file with location of the finding.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Location {
        /// Location within a row or record of a database table.
        #[prost(message, tag="2")]
        RecordLocation(super::RecordLocation),
        /// Location within an image's pixels.
        #[prost(message, tag="3")]
        ImageLocation(super::ImageLocation),
        /// Location data for document files.
        #[prost(message, tag="5")]
        DocumentLocation(super::DocumentLocation),
        /// Location within the metadata for inspected content.
        #[prost(message, tag="8")]
        MetadataLocation(super::MetadataLocation),
    }
}
/// Metadata Location
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataLocation {
    /// Type of metadata containing the finding.
    #[prost(enumeration="MetadataType", tag="1")]
    pub r#type: i32,
    /// Label of the piece of metadata containing the finding, for example -
    /// latitude, author, caption.
    #[prost(oneof="metadata_location::Label", tags="3")]
    pub label: ::core::option::Option<metadata_location::Label>,
}
/// Nested message and enum types in `MetadataLocation`.
pub mod metadata_location {
    /// Label of the piece of metadata containing the finding, for example -
    /// latitude, author, caption.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Label {
        /// Storage metadata.
        #[prost(message, tag="3")]
        StorageLabel(super::StorageMetadataLabel),
    }
}
/// Storage metadata label to indicate which metadata entry contains findings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageMetadataLabel {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
}
/// Location of a finding within a document.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentLocation {
    /// Offset of the line, from the beginning of the file, where the finding
    /// is located.
    #[prost(int64, tag="1")]
    pub file_offset: i64,
}
/// Location of a finding within a row or record.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordLocation {
    /// Key of the finding.
    #[prost(message, optional, tag="1")]
    pub record_key: ::core::option::Option<RecordKey>,
    /// Field id of the field containing the finding.
    #[prost(message, optional, tag="2")]
    pub field_id: ::core::option::Option<FieldId>,
    /// Location within a `ContentItem.Table`.
    #[prost(message, optional, tag="3")]
    pub table_location: ::core::option::Option<TableLocation>,
}
/// Location of a finding within a table.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableLocation {
    /// The zero-based index of the row where the finding is located. Only
    /// populated for resources that have a natural ordering, not BigQuery. In
    /// BigQuery, to identify the row a finding came from, populate
    /// BigQueryOptions.identifying_fields with your primary key column names and
    /// when you store the findings the value of those columns will be stored
    /// inside of Finding.
    #[prost(int64, tag="1")]
    pub row_index: i64,
}
/// Represents a container that may contain DLP findings.
/// Examples of a container include a file, table, or database record.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Container {
    /// Container type, for example BigQuery or Cloud Storage.
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    /// Project where the finding was found.
    /// Can be different from the project that owns the finding.
    #[prost(string, tag="2")]
    pub project_id: ::prost::alloc::string::String,
    /// A string representation of the full container name.
    /// Examples:
    /// - BigQuery: 'Project:DataSetId.TableId'
    /// - Cloud Storage: 'gs://Bucket/folders/filename.txt'
    #[prost(string, tag="3")]
    pub full_path: ::prost::alloc::string::String,
    /// The root of the container.
    /// Examples:
    ///
    /// - For BigQuery table `project_id:dataset_id.table_id`, the root is
    ///  `dataset_id`
    /// - For Cloud Storage file `gs://bucket/folder/filename.txt`, the root
    ///  is `gs://bucket`
    #[prost(string, tag="4")]
    pub root_path: ::prost::alloc::string::String,
    /// The rest of the path after the root.
    /// Examples:
    ///
    /// - For BigQuery table `project_id:dataset_id.table_id`, the relative path is
    ///  `table_id`
    /// - For Cloud Storage file `gs://bucket/folder/filename.txt`, the relative
    ///  path is `folder/filename.txt`
    #[prost(string, tag="5")]
    pub relative_path: ::prost::alloc::string::String,
    /// Findings container modification timestamp, if applicable. For Cloud
    /// Storage, this field contains the last file modification timestamp. For a
    /// BigQuery table, this field contains the last_modified_time property. For
    /// Datastore, this field isn't populated.
    #[prost(message, optional, tag="6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Findings container version, if available
    /// ("generation" for Cloud Storage).
    #[prost(string, tag="7")]
    pub version: ::prost::alloc::string::String,
}
/// Generic half-open interval [start, end)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Range {
    /// Index of the first character of the range (inclusive).
    #[prost(int64, tag="1")]
    pub start: i64,
    /// Index of the last character of the range (exclusive).
    #[prost(int64, tag="2")]
    pub end: i64,
}
/// Location of the finding within an image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageLocation {
    /// Bounding boxes locating the pixels within the image containing the finding.
    #[prost(message, repeated, tag="1")]
    pub bounding_boxes: ::prost::alloc::vec::Vec<BoundingBox>,
}
/// Bounding box encompassing detected text within an image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoundingBox {
    /// Top coordinate of the bounding box. (0,0) is upper left.
    #[prost(int32, tag="1")]
    pub top: i32,
    /// Left coordinate of the bounding box. (0,0) is upper left.
    #[prost(int32, tag="2")]
    pub left: i32,
    /// Width of the bounding box in pixels.
    #[prost(int32, tag="3")]
    pub width: i32,
    /// Height of the bounding box in pixels.
    #[prost(int32, tag="4")]
    pub height: i32,
}
/// Request to search for potentially sensitive info in an image and redact it
/// by covering it with a colored rectangle.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedactImageRequest {
    /// Parent resource name.
    ///
    /// The format of this value varies depending on whether you have [specified a
    /// processing
    /// location](<https://cloud.google.com/dlp/docs/specifying-location>):
    ///
    /// + Projects scope, location specified:<br/>
    ///   `projects/`<var>PROJECT_ID</var>`/locations/`<var>LOCATION_ID</var>
    /// + Projects scope, no location specified (defaults to global):<br/>
    ///   `projects/`<var>PROJECT_ID</var>
    ///
    /// The following example `parent` string specifies a parent project with the
    /// identifier `example-project`, and specifies the `europe-west3` location
    /// for processing data:
    ///
    ///     parent=projects/example-project/locations/europe-west3
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Deprecated. This field has no effect.
    #[prost(string, tag="8")]
    pub location_id: ::prost::alloc::string::String,
    /// Configuration for the inspector.
    #[prost(message, optional, tag="2")]
    pub inspect_config: ::core::option::Option<InspectConfig>,
    /// The configuration for specifying what content to redact from images.
    #[prost(message, repeated, tag="5")]
    pub image_redaction_configs: ::prost::alloc::vec::Vec<redact_image_request::ImageRedactionConfig>,
    /// Whether the response should include findings along with the redacted
    /// image.
    #[prost(bool, tag="6")]
    pub include_findings: bool,
    /// The content must be PNG, JPEG, SVG or BMP.
    #[prost(message, optional, tag="7")]
    pub byte_item: ::core::option::Option<ByteContentItem>,
}
/// Nested message and enum types in `RedactImageRequest`.
pub mod redact_image_request {
    /// Configuration for determining how redaction of images should occur.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ImageRedactionConfig {
        /// The color to use when redacting content from an image. If not specified,
        /// the default is black.
        #[prost(message, optional, tag="3")]
        pub redaction_color: ::core::option::Option<super::Color>,
        /// Type of information to redact from images.
        #[prost(oneof="image_redaction_config::Target", tags="1, 2")]
        pub target: ::core::option::Option<image_redaction_config::Target>,
    }
    /// Nested message and enum types in `ImageRedactionConfig`.
    pub mod image_redaction_config {
        /// Type of information to redact from images.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Target {
            /// Only one per info_type should be provided per request. If not
            /// specified, and redact_all_text is false, the DLP API will redact all
            /// text that it matches against all info_types that are found, but not
            /// specified in another ImageRedactionConfig.
            #[prost(message, tag="1")]
            InfoType(super::super::InfoType),
            /// If true, all text found in the image, regardless whether it matches an
            /// info_type, is redacted. Only one should be provided.
            #[prost(bool, tag="2")]
            RedactAllText(bool),
        }
    }
}
/// Represents a color in the RGB color space.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Color {
    /// The amount of red in the color as a value in the interval [0, 1].
    #[prost(float, tag="1")]
    pub red: f32,
    /// The amount of green in the color as a value in the interval [0, 1].
    #[prost(float, tag="2")]
    pub green: f32,
    /// The amount of blue in the color as a value in the interval [0, 1].
    #[prost(float, tag="3")]
    pub blue: f32,
}
/// Results of redacting an image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedactImageResponse {
    /// The redacted image. The type will be the same as the original image.
    #[prost(bytes="bytes", tag="1")]
    pub redacted_image: ::prost::bytes::Bytes,
    /// If an image was being inspected and the InspectConfig's include_quote was
    /// set to true, then this field will include all text, if any, that was found
    /// in the image.
    #[prost(string, tag="2")]
    pub extracted_text: ::prost::alloc::string::String,
    /// The findings. Populated when include_findings in the request is true.
    #[prost(message, optional, tag="3")]
    pub inspect_result: ::core::option::Option<InspectResult>,
}
/// Request to de-identify a ContentItem.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeidentifyContentRequest {
    /// Parent resource name.
    ///
    /// The format of this value varies depending on whether you have [specified a
    /// processing
    /// location](<https://cloud.google.com/dlp/docs/specifying-location>):
    ///
    /// + Projects scope, location specified:<br/>
    ///   `projects/`<var>PROJECT_ID</var>`/locations/`<var>LOCATION_ID</var>
    /// + Projects scope, no location specified (defaults to global):<br/>
    ///   `projects/`<var>PROJECT_ID</var>
    ///
    /// The following example `parent` string specifies a parent project with the
    /// identifier `example-project`, and specifies the `europe-west3` location
    /// for processing data:
    ///
    ///     parent=projects/example-project/locations/europe-west3
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Configuration for the de-identification of the content item.
    /// Items specified here will override the template referenced by the
    /// deidentify_template_name argument.
    #[prost(message, optional, tag="2")]
    pub deidentify_config: ::core::option::Option<DeidentifyConfig>,
    /// Configuration for the inspector.
    /// Items specified here will override the template referenced by the
    /// inspect_template_name argument.
    #[prost(message, optional, tag="3")]
    pub inspect_config: ::core::option::Option<InspectConfig>,
    /// The item to de-identify. Will be treated as text.
    #[prost(message, optional, tag="4")]
    pub item: ::core::option::Option<ContentItem>,
    /// Template to use. Any configuration directly specified in
    /// inspect_config will override those set in the template. Singular fields
    /// that are set in this request will replace their corresponding fields in the
    /// template. Repeated fields are appended. Singular sub-messages and groups
    /// are recursively merged.
    #[prost(string, tag="5")]
    pub inspect_template_name: ::prost::alloc::string::String,
    /// Template to use. Any configuration directly specified in
    /// deidentify_config will override those set in the template. Singular fields
    /// that are set in this request will replace their corresponding fields in the
    /// template. Repeated fields are appended. Singular sub-messages and groups
    /// are recursively merged.
    #[prost(string, tag="6")]
    pub deidentify_template_name: ::prost::alloc::string::String,
    /// Deprecated. This field has no effect.
    #[prost(string, tag="7")]
    pub location_id: ::prost::alloc::string::String,
}
/// Results of de-identifying a ContentItem.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeidentifyContentResponse {
    /// The de-identified item.
    #[prost(message, optional, tag="1")]
    pub item: ::core::option::Option<ContentItem>,
    /// An overview of the changes that were made on the `item`.
    #[prost(message, optional, tag="2")]
    pub overview: ::core::option::Option<TransformationOverview>,
}
/// Request to re-identify an item.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReidentifyContentRequest {
    /// Required. Parent resource name.
    ///
    /// The format of this value varies depending on whether you have [specified a
    /// processing
    /// location](<https://cloud.google.com/dlp/docs/specifying-location>):
    ///
    /// + Projects scope, location specified:<br/>
    ///   `projects/`<var>PROJECT_ID</var>`/locations/`<var>LOCATION_ID</var>
    /// + Projects scope, no location specified (defaults to global):<br/>
    ///   `projects/`<var>PROJECT_ID</var>
    ///
    /// The following example `parent` string specifies a parent project with the
    /// identifier `example-project`, and specifies the `europe-west3` location
    /// for processing data:
    ///
    ///     parent=projects/example-project/locations/europe-west3
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Configuration for the re-identification of the content item.
    /// This field shares the same proto message type that is used for
    /// de-identification, however its usage here is for the reversal of the
    /// previous de-identification. Re-identification is performed by examining
    /// the transformations used to de-identify the items and executing the
    /// reverse. This requires that only reversible transformations
    /// be provided here. The reversible transformations are:
    ///
    ///  - `CryptoDeterministicConfig`
    ///  - `CryptoReplaceFfxFpeConfig`
    #[prost(message, optional, tag="2")]
    pub reidentify_config: ::core::option::Option<DeidentifyConfig>,
    /// Configuration for the inspector.
    #[prost(message, optional, tag="3")]
    pub inspect_config: ::core::option::Option<InspectConfig>,
    /// The item to re-identify. Will be treated as text.
    #[prost(message, optional, tag="4")]
    pub item: ::core::option::Option<ContentItem>,
    /// Template to use. Any configuration directly specified in
    /// `inspect_config` will override those set in the template. Singular fields
    /// that are set in this request will replace their corresponding fields in the
    /// template. Repeated fields are appended. Singular sub-messages and groups
    /// are recursively merged.
    #[prost(string, tag="5")]
    pub inspect_template_name: ::prost::alloc::string::String,
    /// Template to use. References an instance of `DeidentifyTemplate`.
    /// Any configuration directly specified in `reidentify_config` or
    /// `inspect_config` will override those set in the template. The
    /// `DeidentifyTemplate` used must include only reversible transformations.
    /// Singular fields that are set in this request will replace their
    /// corresponding fields in the template. Repeated fields are appended.
    /// Singular sub-messages and groups are recursively merged.
    #[prost(string, tag="6")]
    pub reidentify_template_name: ::prost::alloc::string::String,
    /// Deprecated. This field has no effect.
    #[prost(string, tag="7")]
    pub location_id: ::prost::alloc::string::String,
}
/// Results of re-identifying an item.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReidentifyContentResponse {
    /// The re-identified item.
    #[prost(message, optional, tag="1")]
    pub item: ::core::option::Option<ContentItem>,
    /// An overview of the changes that were made to the `item`.
    #[prost(message, optional, tag="2")]
    pub overview: ::core::option::Option<TransformationOverview>,
}
/// Request to search for potentially sensitive info in a ContentItem.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InspectContentRequest {
    /// Parent resource name.
    ///
    /// The format of this value varies depending on whether you have [specified a
    /// processing
    /// location](<https://cloud.google.com/dlp/docs/specifying-location>):
    ///
    /// + Projects scope, location specified:<br/>
    ///   `projects/`<var>PROJECT_ID</var>`/locations/`<var>LOCATION_ID</var>
    /// + Projects scope, no location specified (defaults to global):<br/>
    ///   `projects/`<var>PROJECT_ID</var>
    ///
    /// The following example `parent` string specifies a parent project with the
    /// identifier `example-project`, and specifies the `europe-west3` location
    /// for processing data:
    ///
    ///     parent=projects/example-project/locations/europe-west3
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Configuration for the inspector. What specified here will override
    /// the template referenced by the inspect_template_name argument.
    #[prost(message, optional, tag="2")]
    pub inspect_config: ::core::option::Option<InspectConfig>,
    /// The item to inspect.
    #[prost(message, optional, tag="3")]
    pub item: ::core::option::Option<ContentItem>,
    /// Template to use. Any configuration directly specified in
    /// inspect_config will override those set in the template. Singular fields
    /// that are set in this request will replace their corresponding fields in the
    /// template. Repeated fields are appended. Singular sub-messages and groups
    /// are recursively merged.
    #[prost(string, tag="4")]
    pub inspect_template_name: ::prost::alloc::string::String,
    /// Deprecated. This field has no effect.
    #[prost(string, tag="5")]
    pub location_id: ::prost::alloc::string::String,
}
/// Results of inspecting an item.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InspectContentResponse {
    /// The findings.
    #[prost(message, optional, tag="1")]
    pub result: ::core::option::Option<InspectResult>,
}
/// Cloud repository for storing output.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputStorageConfig {
    /// Schema used for writing the findings for Inspect jobs. This field is only
    /// used for Inspect and must be unspecified for Risk jobs. Columns are derived
    /// from the `Finding` object. If appending to an existing table, any columns
    /// from the predefined schema that are missing will be added. No columns in
    /// the existing table will be deleted.
    ///
    /// If unspecified, then all available columns will be used for a new table or
    /// an (existing) table with no schema, and no changes will be made to an
    /// existing table that has a schema.
    /// Only for use with external storage.
    #[prost(enumeration="output_storage_config::OutputSchema", tag="3")]
    pub output_schema: i32,
    /// Output storage types.
    #[prost(oneof="output_storage_config::Type", tags="1")]
    pub r#type: ::core::option::Option<output_storage_config::Type>,
}
/// Nested message and enum types in `OutputStorageConfig`.
pub mod output_storage_config {
    /// Predefined schemas for storing findings.
    /// Only for use with external storage.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OutputSchema {
        /// Unused.
        Unspecified = 0,
        /// Basic schema including only `info_type`, `quote`, `certainty`, and
        /// `timestamp`.
        BasicColumns = 1,
        /// Schema tailored to findings from scanning Cloud Storage.
        GcsColumns = 2,
        /// Schema tailored to findings from scanning Google Datastore.
        DatastoreColumns = 3,
        /// Schema tailored to findings from scanning Google BigQuery.
        BigQueryColumns = 4,
        /// Schema containing all columns.
        AllColumns = 5,
    }
    /// Output storage types.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// Store findings in an existing table or a new table in an existing
        /// dataset. If table_id is not set a new one will be generated
        /// for you with the following format:
        /// dlp_googleapis_yyyy_mm_dd_\[dlp_job_id\]. Pacific time zone will be used
        /// for generating the date details.
        ///
        /// For Inspect, each column in an existing output table must have the same
        /// name, type, and mode of a field in the `Finding` object.
        ///
        /// For Risk, an existing output table should be the output of a previous
        /// Risk analysis job run on the same source table, with the same privacy
        /// metric and quasi-identifiers. Risk jobs that analyze the same table but
        /// compute a different privacy metric, or use different sets of
        /// quasi-identifiers, cannot store their results in the same table.
        #[prost(message, tag="1")]
        Table(super::BigQueryTable),
    }
}
/// Statistics regarding a specific InfoType.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoTypeStats {
    /// The type of finding this stat is for.
    #[prost(message, optional, tag="1")]
    pub info_type: ::core::option::Option<InfoType>,
    /// Number of findings for this infoType.
    #[prost(int64, tag="2")]
    pub count: i64,
}
/// The results of an inspect DataSource job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InspectDataSourceDetails {
    /// The configuration used for this job.
    #[prost(message, optional, tag="2")]
    pub requested_options: ::core::option::Option<inspect_data_source_details::RequestedOptions>,
    /// A summary of the outcome of this inspection job.
    #[prost(message, optional, tag="3")]
    pub result: ::core::option::Option<inspect_data_source_details::Result>,
}
/// Nested message and enum types in `InspectDataSourceDetails`.
pub mod inspect_data_source_details {
    /// Snapshot of the inspection configuration.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RequestedOptions {
        /// If run with an InspectTemplate, a snapshot of its state at the time of
        /// this run.
        #[prost(message, optional, tag="1")]
        pub snapshot_inspect_template: ::core::option::Option<super::InspectTemplate>,
        /// Inspect config.
        #[prost(message, optional, tag="3")]
        pub job_config: ::core::option::Option<super::InspectJobConfig>,
    }
    /// All result fields mentioned below are updated while the job is processing.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Result {
        /// Total size in bytes that were processed.
        #[prost(int64, tag="1")]
        pub processed_bytes: i64,
        /// Estimate of the number of bytes to process.
        #[prost(int64, tag="2")]
        pub total_estimated_bytes: i64,
        /// Statistics of how many instances of each info type were found during
        /// inspect job.
        #[prost(message, repeated, tag="3")]
        pub info_type_stats: ::prost::alloc::vec::Vec<super::InfoTypeStats>,
        /// Statistics related to the processing of hybrid inspect.
        #[prost(message, optional, tag="7")]
        pub hybrid_stats: ::core::option::Option<super::HybridInspectStatistics>,
    }
}
/// Statistics related to processing hybrid inspect requests.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HybridInspectStatistics {
    /// The number of hybrid inspection requests processed within this job.
    #[prost(int64, tag="1")]
    pub processed_count: i64,
    /// The number of hybrid inspection requests aborted because the job ran
    /// out of quota or was ended before they could be processed.
    #[prost(int64, tag="2")]
    pub aborted_count: i64,
    /// The number of hybrid requests currently being processed. Only populated
    /// when called via method `getDlpJob`.
    /// A burst of traffic may cause hybrid inspect requests to be enqueued.
    /// Processing will take place as quickly as possible, but resource limitations
    /// may impact how long a request is enqueued for.
    #[prost(int64, tag="3")]
    pub pending_count: i64,
}
/// InfoType description.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoTypeDescription {
    /// Internal name of the infoType.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Human readable form of the infoType name.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    /// Which parts of the API supports this InfoType.
    #[prost(enumeration="InfoTypeSupportedBy", repeated, tag="3")]
    pub supported_by: ::prost::alloc::vec::Vec<i32>,
    /// Description of the infotype. Translated when language is provided in the
    /// request.
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
    /// A list of available versions for the infotype.
    #[prost(message, repeated, tag="9")]
    pub versions: ::prost::alloc::vec::Vec<VersionDescription>,
    /// The category of the infoType.
    #[prost(message, repeated, tag="10")]
    pub categories: ::prost::alloc::vec::Vec<InfoTypeCategory>,
}
/// Classification of infoTypes to organize them according to geographic
/// location, industry, and data type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoTypeCategory {
    #[prost(oneof="info_type_category::Category", tags="1, 2, 3")]
    pub category: ::core::option::Option<info_type_category::Category>,
}
/// Nested message and enum types in `InfoTypeCategory`.
pub mod info_type_category {
    /// Enum of the current locations.
    /// We might add more locations in the future.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LocationCategory {
        /// Unused location
        LocationUnspecified = 0,
        /// The infoType is not issued by or tied to a specific region, but is used
        /// almost everywhere.
        Global = 1,
        /// The infoType is typically used in Argentina.
        Argentina = 2,
        /// The infoType is typically used in Australia.
        Australia = 3,
        /// The infoType is typically used in Belgium.
        Belgium = 4,
        /// The infoType is typically used in Brazil.
        Brazil = 5,
        /// The infoType is typically used in Canada.
        Canada = 6,
        /// The infoType is typically used in Chile.
        Chile = 7,
        /// The infoType is typically used in China.
        China = 8,
        /// The infoType is typically used in Colombia.
        Colombia = 9,
        /// The infoType is typically used in Denmark.
        Denmark = 10,
        /// The infoType is typically used in France.
        France = 11,
        /// The infoType is typically used in Finland.
        Finland = 12,
        /// The infoType is typically used in Germany.
        Germany = 13,
        /// The infoType is typically used in Hong Kong.
        HongKong = 14,
        /// The infoType is typically used in India.
        India = 15,
        /// The infoType is typically used in Indonesia.
        Indonesia = 16,
        /// The infoType is typically used in Ireland.
        Ireland = 17,
        /// The infoType is typically used in Israel.
        Israel = 18,
        /// The infoType is typically used in Italy.
        Italy = 19,
        /// The infoType is typically used in Japan.
        Japan = 20,
        /// The infoType is typically used in Korea.
        Korea = 21,
        /// The infoType is typically used in Mexico.
        Mexico = 22,
        /// The infoType is typically used in the Netherlands.
        TheNetherlands = 23,
        /// The infoType is typically used in Norway.
        Norway = 24,
        /// The infoType is typically used in Paraguay.
        Paraguay = 25,
        /// The infoType is typically used in Peru.
        Peru = 26,
        /// The infoType is typically used in Poland.
        Poland = 27,
        /// The infoType is typically used in Portugal.
        Portugal = 28,
        /// The infoType is typically used in Singapore.
        Singapore = 29,
        /// The infoType is typically used in South Africa.
        SouthAfrica = 30,
        /// The infoType is typically used in Spain.
        Spain = 31,
        /// The infoType is typically used in Sweden.
        Sweden = 32,
        /// The infoType is typically used in Taiwan.
        Taiwan = 33,
        /// The infoType is typically used in Thailand.
        Thailand = 34,
        /// The infoType is typically used in Turkey.
        Turkey = 35,
        /// The infoType is typically used in the United Kingdom.
        UnitedKingdom = 36,
        /// The infoType is typically used in the United States.
        UnitedStates = 37,
        /// The infoType is typically used in Uruguay.
        Uruguay = 38,
        /// The infoType is typically used in Venezuela.
        Venezuela = 39,
        /// The infoType is typically used in Google internally.
        Internal = 40,
    }
    /// Enum of the current industries in the category.
    /// We might add more industries in the future.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum IndustryCategory {
        /// Unused industry
        IndustryUnspecified = 0,
        /// The infoType is typically used in the finance industry.
        Finance = 1,
        /// The infoType is typically used in the health industry.
        Health = 2,
        /// The infoType is typically used in the telecommunications industry.
        Telecommunications = 3,
    }
    /// Enum of the current types in the category.
    /// We might add more types in the future.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TypeCategory {
        /// Unused type
        TypeUnspecified = 0,
        /// Personally identifiable information, for example, a
        /// name or phone number
        Pii = 1,
        /// Personally identifiable information that is especially sensitive, for
        /// example, a passport number.
        Spii = 2,
        /// Attributes that can partially identify someone, especially in
        /// combination with other attributes, like age, height, and gender.
        Demographic = 3,
        /// Confidential or secret information, for example, a password.
        Credential = 4,
        /// An identification document issued by a government.
        GovernmentId = 5,
        /// A document, for example, a resume or source code.
        Document = 6,
        /// Information that is not sensitive on its own, but provides details about
        /// the circumstances surrounding an entity or an event.
        ContextualInformation = 7,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Category {
        /// The region or country that issued the ID or document represented by the
        /// infoType.
        #[prost(enumeration="LocationCategory", tag="1")]
        LocationCategory(i32),
        /// The group of relevant businesses where this infoType is commonly used
        #[prost(enumeration="IndustryCategory", tag="2")]
        IndustryCategory(i32),
        /// The class of identifiers where this infoType belongs
        #[prost(enumeration="TypeCategory", tag="3")]
        TypeCategory(i32),
    }
}
/// Details about each available version for an infotype.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionDescription {
    /// Name of the version
    #[prost(string, tag="1")]
    pub version: ::prost::alloc::string::String,
    /// Description of the version.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
}
/// Request for the list of infoTypes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInfoTypesRequest {
    /// The parent resource name.
    ///
    /// The format of this value is as follows:
    ///
    ///     locations/<var>LOCATION_ID</var>
    #[prost(string, tag="4")]
    pub parent: ::prost::alloc::string::String,
    /// BCP-47 language code for localized infoType friendly
    /// names. If omitted, or if localized strings are not available,
    /// en-US strings will be returned.
    #[prost(string, tag="1")]
    pub language_code: ::prost::alloc::string::String,
    /// filter to only return infoTypes supported by certain parts of the
    /// API. Defaults to supported_by=INSPECT.
    #[prost(string, tag="2")]
    pub filter: ::prost::alloc::string::String,
    /// Deprecated. This field has no effect.
    #[prost(string, tag="3")]
    pub location_id: ::prost::alloc::string::String,
}
/// Response to the ListInfoTypes request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInfoTypesResponse {
    /// Set of sensitive infoTypes.
    #[prost(message, repeated, tag="1")]
    pub info_types: ::prost::alloc::vec::Vec<InfoTypeDescription>,
}
/// Configuration for a risk analysis job. See
/// <https://cloud.google.com/dlp/docs/concepts-risk-analysis> to learn more.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RiskAnalysisJobConfig {
    /// Privacy metric to compute.
    #[prost(message, optional, tag="1")]
    pub privacy_metric: ::core::option::Option<PrivacyMetric>,
    /// Input dataset to compute metrics over.
    #[prost(message, optional, tag="2")]
    pub source_table: ::core::option::Option<BigQueryTable>,
    /// Actions to execute at the completion of the job. Are executed in the order
    /// provided.
    #[prost(message, repeated, tag="3")]
    pub actions: ::prost::alloc::vec::Vec<Action>,
}
/// A column with a semantic tag attached.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuasiId {
    /// Required. Identifies the column.
    #[prost(message, optional, tag="1")]
    pub field: ::core::option::Option<FieldId>,
    /// Semantic tag that identifies what a column contains, to determine which
    /// statistical model to use to estimate the reidentifiability of each
    /// value. \[required\]
    #[prost(oneof="quasi_id::Tag", tags="2, 3, 4")]
    pub tag: ::core::option::Option<quasi_id::Tag>,
}
/// Nested message and enum types in `QuasiId`.
pub mod quasi_id {
    /// Semantic tag that identifies what a column contains, to determine which
    /// statistical model to use to estimate the reidentifiability of each
    /// value. \[required\]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Tag {
        /// A column can be tagged with a InfoType to use the relevant public
        /// dataset as a statistical model of population, if available. We
        /// currently support US ZIP codes, region codes, ages and genders.
        /// To programmatically obtain the list of supported InfoTypes, use
        /// ListInfoTypes with the supported_by=RISK_ANALYSIS filter.
        #[prost(message, tag="2")]
        InfoType(super::InfoType),
        /// A column can be tagged with a custom tag. In this case, the user must
        /// indicate an auxiliary table that contains statistical information on
        /// the possible values of this column (below).
        #[prost(string, tag="3")]
        CustomTag(::prost::alloc::string::String),
        /// If no semantic tag is indicated, we infer the statistical model from
        /// the distribution of values in the input data
        #[prost(message, tag="4")]
        Inferred(()),
    }
}
/// An auxiliary table containing statistical information on the relative
/// frequency of different quasi-identifiers values. It has one or several
/// quasi-identifiers columns, and one column that indicates the relative
/// frequency of each quasi-identifier tuple.
/// If a tuple is present in the data but not in the auxiliary table, the
/// corresponding relative frequency is assumed to be zero (and thus, the
/// tuple is highly reidentifiable).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatisticalTable {
    /// Required. Auxiliary table location.
    #[prost(message, optional, tag="3")]
    pub table: ::core::option::Option<BigQueryTable>,
    /// Required. Quasi-identifier columns.
    #[prost(message, repeated, tag="1")]
    pub quasi_ids: ::prost::alloc::vec::Vec<statistical_table::QuasiIdentifierField>,
    /// Required. The relative frequency column must contain a floating-point number
    /// between 0 and 1 (inclusive). Null values are assumed to be zero.
    #[prost(message, optional, tag="2")]
    pub relative_frequency: ::core::option::Option<FieldId>,
}
/// Nested message and enum types in `StatisticalTable`.
pub mod statistical_table {
    /// A quasi-identifier column has a custom_tag, used to know which column
    /// in the data corresponds to which column in the statistical model.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QuasiIdentifierField {
        /// Identifies the column.
        #[prost(message, optional, tag="1")]
        pub field: ::core::option::Option<super::FieldId>,
        /// A column can be tagged with a custom tag. In this case, the user must
        /// indicate an auxiliary table that contains statistical information on
        /// the possible values of this column (below).
        #[prost(string, tag="2")]
        pub custom_tag: ::prost::alloc::string::String,
    }
}
/// Privacy metric to compute for reidentification risk analysis.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivacyMetric {
    /// Types of analysis.
    #[prost(oneof="privacy_metric::Type", tags="1, 2, 3, 4, 5, 6")]
    pub r#type: ::core::option::Option<privacy_metric::Type>,
}
/// Nested message and enum types in `PrivacyMetric`.
pub mod privacy_metric {
    /// Compute numerical stats over an individual column, including
    /// min, max, and quantiles.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NumericalStatsConfig {
        /// Field to compute numerical stats on. Supported types are
        /// integer, float, date, datetime, timestamp, time.
        #[prost(message, optional, tag="1")]
        pub field: ::core::option::Option<super::FieldId>,
    }
    /// Compute numerical stats over an individual column, including
    /// number of distinct values and value count distribution.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CategoricalStatsConfig {
        /// Field to compute categorical stats on. All column types are
        /// supported except for arrays and structs. However, it may be more
        /// informative to use NumericalStats when the field type is supported,
        /// depending on the data.
        #[prost(message, optional, tag="1")]
        pub field: ::core::option::Option<super::FieldId>,
    }
    /// k-anonymity metric, used for analysis of reidentification risk.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KAnonymityConfig {
        /// Set of fields to compute k-anonymity over. When multiple fields are
        /// specified, they are considered a single composite key. Structs and
        /// repeated data types are not supported; however, nested fields are
        /// supported so long as they are not structs themselves or nested within
        /// a repeated field.
        #[prost(message, repeated, tag="1")]
        pub quasi_ids: ::prost::alloc::vec::Vec<super::FieldId>,
        /// Message indicating that multiple rows might be associated to a
        /// single individual. If the same entity_id is associated to multiple
        /// quasi-identifier tuples over distinct rows, we consider the entire
        /// collection of tuples as the composite quasi-identifier. This collection
        /// is a multiset: the order in which the different tuples appear in the
        /// dataset is ignored, but their frequency is taken into account.
        ///
        /// Important note: a maximum of 1000 rows can be associated to a single
        /// entity ID. If more rows are associated with the same entity ID, some
        /// might be ignored.
        #[prost(message, optional, tag="2")]
        pub entity_id: ::core::option::Option<super::EntityId>,
    }
    /// l-diversity metric, used for analysis of reidentification risk.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LDiversityConfig {
        /// Set of quasi-identifiers indicating how equivalence classes are
        /// defined for the l-diversity computation. When multiple fields are
        /// specified, they are considered a single composite key.
        #[prost(message, repeated, tag="1")]
        pub quasi_ids: ::prost::alloc::vec::Vec<super::FieldId>,
        /// Sensitive field for computing the l-value.
        #[prost(message, optional, tag="2")]
        pub sensitive_attribute: ::core::option::Option<super::FieldId>,
    }
    /// Reidentifiability metric. This corresponds to a risk model similar to what
    /// is called "journalist risk" in the literature, except the attack dataset is
    /// statistically modeled instead of being perfectly known. This can be done
    /// using publicly available data (like the US Census), or using a custom
    /// statistical model (indicated as one or several BigQuery tables), or by
    /// extrapolating from the distribution of values in the input dataset.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KMapEstimationConfig {
        /// Required. Fields considered to be quasi-identifiers. No two columns can have the
        /// same tag.
        #[prost(message, repeated, tag="1")]
        pub quasi_ids: ::prost::alloc::vec::Vec<k_map_estimation_config::TaggedField>,
        /// ISO 3166-1 alpha-2 region code to use in the statistical modeling.
        /// Set if no column is tagged with a region-specific InfoType (like
        /// US_ZIP_5) or a region code.
        #[prost(string, tag="2")]
        pub region_code: ::prost::alloc::string::String,
        /// Several auxiliary tables can be used in the analysis. Each custom_tag
        /// used to tag a quasi-identifiers column must appear in exactly one column
        /// of one auxiliary table.
        #[prost(message, repeated, tag="3")]
        pub auxiliary_tables: ::prost::alloc::vec::Vec<k_map_estimation_config::AuxiliaryTable>,
    }
    /// Nested message and enum types in `KMapEstimationConfig`.
    pub mod k_map_estimation_config {
        /// A column with a semantic tag attached.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TaggedField {
            /// Required. Identifies the column.
            #[prost(message, optional, tag="1")]
            pub field: ::core::option::Option<super::super::FieldId>,
            /// Semantic tag that identifies what a column contains, to determine which
            /// statistical model to use to estimate the reidentifiability of each
            /// value. \[required\]
            #[prost(oneof="tagged_field::Tag", tags="2, 3, 4")]
            pub tag: ::core::option::Option<tagged_field::Tag>,
        }
        /// Nested message and enum types in `TaggedField`.
        pub mod tagged_field {
            /// Semantic tag that identifies what a column contains, to determine which
            /// statistical model to use to estimate the reidentifiability of each
            /// value. \[required\]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Tag {
                /// A column can be tagged with a InfoType to use the relevant public
                /// dataset as a statistical model of population, if available. We
                /// currently support US ZIP codes, region codes, ages and genders.
                /// To programmatically obtain the list of supported InfoTypes, use
                /// ListInfoTypes with the supported_by=RISK_ANALYSIS filter.
                #[prost(message, tag="2")]
                InfoType(super::super::super::InfoType),
                /// A column can be tagged with a custom tag. In this case, the user must
                /// indicate an auxiliary table that contains statistical information on
                /// the possible values of this column (below).
                #[prost(string, tag="3")]
                CustomTag(::prost::alloc::string::String),
                /// If no semantic tag is indicated, we infer the statistical model from
                /// the distribution of values in the input data
                #[prost(message, tag="4")]
                Inferred(()),
            }
        }
        /// An auxiliary table contains statistical information on the relative
        /// frequency of different quasi-identifiers values. It has one or several
        /// quasi-identifiers columns, and one column that indicates the relative
        /// frequency of each quasi-identifier tuple.
        /// If a tuple is present in the data but not in the auxiliary table, the
        /// corresponding relative frequency is assumed to be zero (and thus, the
        /// tuple is highly reidentifiable).
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct AuxiliaryTable {
            /// Required. Auxiliary table location.
            #[prost(message, optional, tag="3")]
            pub table: ::core::option::Option<super::super::BigQueryTable>,
            /// Required. Quasi-identifier columns.
            #[prost(message, repeated, tag="1")]
            pub quasi_ids: ::prost::alloc::vec::Vec<auxiliary_table::QuasiIdField>,
            /// Required. The relative frequency column must contain a floating-point number
            /// between 0 and 1 (inclusive). Null values are assumed to be zero.
            #[prost(message, optional, tag="2")]
            pub relative_frequency: ::core::option::Option<super::super::FieldId>,
        }
        /// Nested message and enum types in `AuxiliaryTable`.
        pub mod auxiliary_table {
            /// A quasi-identifier column has a custom_tag, used to know which column
            /// in the data corresponds to which column in the statistical model.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct QuasiIdField {
                /// Identifies the column.
                #[prost(message, optional, tag="1")]
                pub field: ::core::option::Option<super::super::super::FieldId>,
                /// A auxiliary field.
                #[prost(string, tag="2")]
                pub custom_tag: ::prost::alloc::string::String,
            }
        }
    }
    /// δ-presence metric, used to estimate how likely it is for an attacker to
    /// figure out that one given individual appears in a de-identified dataset.
    /// Similarly to the k-map metric, we cannot compute δ-presence exactly without
    /// knowing the attack dataset, so we use a statistical model instead.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeltaPresenceEstimationConfig {
        /// Required. Fields considered to be quasi-identifiers. No two fields can have the
        /// same tag.
        #[prost(message, repeated, tag="1")]
        pub quasi_ids: ::prost::alloc::vec::Vec<super::QuasiId>,
        /// ISO 3166-1 alpha-2 region code to use in the statistical modeling.
        /// Set if no column is tagged with a region-specific InfoType (like
        /// US_ZIP_5) or a region code.
        #[prost(string, tag="2")]
        pub region_code: ::prost::alloc::string::String,
        /// Several auxiliary tables can be used in the analysis. Each custom_tag
        /// used to tag a quasi-identifiers field must appear in exactly one
        /// field of one auxiliary table.
        #[prost(message, repeated, tag="3")]
        pub auxiliary_tables: ::prost::alloc::vec::Vec<super::StatisticalTable>,
    }
    /// Types of analysis.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// Numerical stats
        #[prost(message, tag="1")]
        NumericalStatsConfig(NumericalStatsConfig),
        /// Categorical stats
        #[prost(message, tag="2")]
        CategoricalStatsConfig(CategoricalStatsConfig),
        /// K-anonymity
        #[prost(message, tag="3")]
        KAnonymityConfig(KAnonymityConfig),
        /// l-diversity
        #[prost(message, tag="4")]
        LDiversityConfig(LDiversityConfig),
        /// k-map
        #[prost(message, tag="5")]
        KMapEstimationConfig(KMapEstimationConfig),
        /// delta-presence
        #[prost(message, tag="6")]
        DeltaPresenceEstimationConfig(DeltaPresenceEstimationConfig),
    }
}
/// Result of a risk analysis operation request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeDataSourceRiskDetails {
    /// Privacy metric to compute.
    #[prost(message, optional, tag="1")]
    pub requested_privacy_metric: ::core::option::Option<PrivacyMetric>,
    /// Input dataset to compute metrics over.
    #[prost(message, optional, tag="2")]
    pub requested_source_table: ::core::option::Option<BigQueryTable>,
    /// The configuration used for this job.
    #[prost(message, optional, tag="10")]
    pub requested_options: ::core::option::Option<analyze_data_source_risk_details::RequestedRiskAnalysisOptions>,
    /// Values associated with this metric.
    #[prost(oneof="analyze_data_source_risk_details::Result", tags="3, 4, 5, 6, 7, 9")]
    pub result: ::core::option::Option<analyze_data_source_risk_details::Result>,
}
/// Nested message and enum types in `AnalyzeDataSourceRiskDetails`.
pub mod analyze_data_source_risk_details {
    /// Result of the numerical stats computation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NumericalStatsResult {
        /// Minimum value appearing in the column.
        #[prost(message, optional, tag="1")]
        pub min_value: ::core::option::Option<super::Value>,
        /// Maximum value appearing in the column.
        #[prost(message, optional, tag="2")]
        pub max_value: ::core::option::Option<super::Value>,
        /// List of 99 values that partition the set of field values into 100 equal
        /// sized buckets.
        #[prost(message, repeated, tag="4")]
        pub quantile_values: ::prost::alloc::vec::Vec<super::Value>,
    }
    /// Result of the categorical stats computation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CategoricalStatsResult {
        /// Histogram of value frequencies in the column.
        #[prost(message, repeated, tag="5")]
        pub value_frequency_histogram_buckets: ::prost::alloc::vec::Vec<categorical_stats_result::CategoricalStatsHistogramBucket>,
    }
    /// Nested message and enum types in `CategoricalStatsResult`.
    pub mod categorical_stats_result {
        /// Histogram of value frequencies in the column.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CategoricalStatsHistogramBucket {
            /// Lower bound on the value frequency of the values in this bucket.
            #[prost(int64, tag="1")]
            pub value_frequency_lower_bound: i64,
            /// Upper bound on the value frequency of the values in this bucket.
            #[prost(int64, tag="2")]
            pub value_frequency_upper_bound: i64,
            /// Total number of values in this bucket.
            #[prost(int64, tag="3")]
            pub bucket_size: i64,
            /// Sample of value frequencies in this bucket. The total number of
            /// values returned per bucket is capped at 20.
            #[prost(message, repeated, tag="4")]
            pub bucket_values: ::prost::alloc::vec::Vec<super::super::ValueFrequency>,
            /// Total number of distinct values in this bucket.
            #[prost(int64, tag="5")]
            pub bucket_value_count: i64,
        }
    }
    /// Result of the k-anonymity computation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KAnonymityResult {
        /// Histogram of k-anonymity equivalence classes.
        #[prost(message, repeated, tag="5")]
        pub equivalence_class_histogram_buckets: ::prost::alloc::vec::Vec<k_anonymity_result::KAnonymityHistogramBucket>,
    }
    /// Nested message and enum types in `KAnonymityResult`.
    pub mod k_anonymity_result {
        /// The set of columns' values that share the same ldiversity value
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct KAnonymityEquivalenceClass {
            /// Set of values defining the equivalence class. One value per
            /// quasi-identifier column in the original KAnonymity metric message.
            /// The order is always the same as the original request.
            #[prost(message, repeated, tag="1")]
            pub quasi_ids_values: ::prost::alloc::vec::Vec<super::super::Value>,
            /// Size of the equivalence class, for example number of rows with the
            /// above set of values.
            #[prost(int64, tag="2")]
            pub equivalence_class_size: i64,
        }
        /// Histogram of k-anonymity equivalence classes.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct KAnonymityHistogramBucket {
            /// Lower bound on the size of the equivalence classes in this bucket.
            #[prost(int64, tag="1")]
            pub equivalence_class_size_lower_bound: i64,
            /// Upper bound on the size of the equivalence classes in this bucket.
            #[prost(int64, tag="2")]
            pub equivalence_class_size_upper_bound: i64,
            /// Total number of equivalence classes in this bucket.
            #[prost(int64, tag="3")]
            pub bucket_size: i64,
            /// Sample of equivalence classes in this bucket. The total number of
            /// classes returned per bucket is capped at 20.
            #[prost(message, repeated, tag="4")]
            pub bucket_values: ::prost::alloc::vec::Vec<KAnonymityEquivalenceClass>,
            /// Total number of distinct equivalence classes in this bucket.
            #[prost(int64, tag="5")]
            pub bucket_value_count: i64,
        }
    }
    /// Result of the l-diversity computation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LDiversityResult {
        /// Histogram of l-diversity equivalence class sensitive value frequencies.
        #[prost(message, repeated, tag="5")]
        pub sensitive_value_frequency_histogram_buckets: ::prost::alloc::vec::Vec<l_diversity_result::LDiversityHistogramBucket>,
    }
    /// Nested message and enum types in `LDiversityResult`.
    pub mod l_diversity_result {
        /// The set of columns' values that share the same ldiversity value.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct LDiversityEquivalenceClass {
            /// Quasi-identifier values defining the k-anonymity equivalence
            /// class. The order is always the same as the original request.
            #[prost(message, repeated, tag="1")]
            pub quasi_ids_values: ::prost::alloc::vec::Vec<super::super::Value>,
            /// Size of the k-anonymity equivalence class.
            #[prost(int64, tag="2")]
            pub equivalence_class_size: i64,
            /// Number of distinct sensitive values in this equivalence class.
            #[prost(int64, tag="3")]
            pub num_distinct_sensitive_values: i64,
            /// Estimated frequencies of top sensitive values.
            #[prost(message, repeated, tag="4")]
            pub top_sensitive_values: ::prost::alloc::vec::Vec<super::super::ValueFrequency>,
        }
        /// Histogram of l-diversity equivalence class sensitive value frequencies.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct LDiversityHistogramBucket {
            /// Lower bound on the sensitive value frequencies of the equivalence
            /// classes in this bucket.
            #[prost(int64, tag="1")]
            pub sensitive_value_frequency_lower_bound: i64,
            /// Upper bound on the sensitive value frequencies of the equivalence
            /// classes in this bucket.
            #[prost(int64, tag="2")]
            pub sensitive_value_frequency_upper_bound: i64,
            /// Total number of equivalence classes in this bucket.
            #[prost(int64, tag="3")]
            pub bucket_size: i64,
            /// Sample of equivalence classes in this bucket. The total number of
            /// classes returned per bucket is capped at 20.
            #[prost(message, repeated, tag="4")]
            pub bucket_values: ::prost::alloc::vec::Vec<LDiversityEquivalenceClass>,
            /// Total number of distinct equivalence classes in this bucket.
            #[prost(int64, tag="5")]
            pub bucket_value_count: i64,
        }
    }
    /// Result of the reidentifiability analysis. Note that these results are an
    /// estimation, not exact values.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KMapEstimationResult {
        /// The intervals [min_anonymity, max_anonymity] do not overlap. If a value
        /// doesn't correspond to any such interval, the associated frequency is
        /// zero. For example, the following records:
        ///   {min_anonymity: 1, max_anonymity: 1, frequency: 17}
        ///   {min_anonymity: 2, max_anonymity: 3, frequency: 42}
        ///   {min_anonymity: 5, max_anonymity: 10, frequency: 99}
        /// mean that there are no record with an estimated anonymity of 4, 5, or
        /// larger than 10.
        #[prost(message, repeated, tag="1")]
        pub k_map_estimation_histogram: ::prost::alloc::vec::Vec<k_map_estimation_result::KMapEstimationHistogramBucket>,
    }
    /// Nested message and enum types in `KMapEstimationResult`.
    pub mod k_map_estimation_result {
        /// A tuple of values for the quasi-identifier columns.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct KMapEstimationQuasiIdValues {
            /// The quasi-identifier values.
            #[prost(message, repeated, tag="1")]
            pub quasi_ids_values: ::prost::alloc::vec::Vec<super::super::Value>,
            /// The estimated anonymity for these quasi-identifier values.
            #[prost(int64, tag="2")]
            pub estimated_anonymity: i64,
        }
        /// A KMapEstimationHistogramBucket message with the following values:
        ///   min_anonymity: 3
        ///   max_anonymity: 5
        ///   frequency: 42
        /// means that there are 42 records whose quasi-identifier values correspond
        /// to 3, 4 or 5 people in the overlying population. An important particular
        /// case is when min_anonymity = max_anonymity = 1: the frequency field then
        /// corresponds to the number of uniquely identifiable records.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct KMapEstimationHistogramBucket {
            /// Always positive.
            #[prost(int64, tag="1")]
            pub min_anonymity: i64,
            /// Always greater than or equal to min_anonymity.
            #[prost(int64, tag="2")]
            pub max_anonymity: i64,
            /// Number of records within these anonymity bounds.
            #[prost(int64, tag="5")]
            pub bucket_size: i64,
            /// Sample of quasi-identifier tuple values in this bucket. The total
            /// number of classes returned per bucket is capped at 20.
            #[prost(message, repeated, tag="6")]
            pub bucket_values: ::prost::alloc::vec::Vec<KMapEstimationQuasiIdValues>,
            /// Total number of distinct quasi-identifier tuple values in this bucket.
            #[prost(int64, tag="7")]
            pub bucket_value_count: i64,
        }
    }
    /// Result of the δ-presence computation. Note that these results are an
    /// estimation, not exact values.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeltaPresenceEstimationResult {
        /// The intervals [min_probability, max_probability) do not overlap. If a
        /// value doesn't correspond to any such interval, the associated frequency
        /// is zero. For example, the following records:
        ///   {min_probability: 0, max_probability: 0.1, frequency: 17}
        ///   {min_probability: 0.2, max_probability: 0.3, frequency: 42}
        ///   {min_probability: 0.3, max_probability: 0.4, frequency: 99}
        /// mean that there are no record with an estimated probability in [0.1, 0.2)
        /// nor larger or equal to 0.4.
        #[prost(message, repeated, tag="1")]
        pub delta_presence_estimation_histogram: ::prost::alloc::vec::Vec<delta_presence_estimation_result::DeltaPresenceEstimationHistogramBucket>,
    }
    /// Nested message and enum types in `DeltaPresenceEstimationResult`.
    pub mod delta_presence_estimation_result {
        /// A tuple of values for the quasi-identifier columns.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DeltaPresenceEstimationQuasiIdValues {
            /// The quasi-identifier values.
            #[prost(message, repeated, tag="1")]
            pub quasi_ids_values: ::prost::alloc::vec::Vec<super::super::Value>,
            /// The estimated probability that a given individual sharing these
            /// quasi-identifier values is in the dataset. This value, typically
            /// called δ, is the ratio between the number of records in the dataset
            /// with these quasi-identifier values, and the total number of individuals
            /// (inside *and* outside the dataset) with these quasi-identifier values.
            /// For example, if there are 15 individuals in the dataset who share the
            /// same quasi-identifier values, and an estimated 100 people in the entire
            /// population with these values, then δ is 0.15.
            #[prost(double, tag="2")]
            pub estimated_probability: f64,
        }
        /// A DeltaPresenceEstimationHistogramBucket message with the following
        /// values:
        ///   min_probability: 0.1
        ///   max_probability: 0.2
        ///   frequency: 42
        /// means that there are 42 records for which δ is in [0.1, 0.2). An
        /// important particular case is when min_probability = max_probability = 1:
        /// then, every individual who shares this quasi-identifier combination is in
        /// the dataset.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DeltaPresenceEstimationHistogramBucket {
            /// Between 0 and 1.
            #[prost(double, tag="1")]
            pub min_probability: f64,
            /// Always greater than or equal to min_probability.
            #[prost(double, tag="2")]
            pub max_probability: f64,
            /// Number of records within these probability bounds.
            #[prost(int64, tag="5")]
            pub bucket_size: i64,
            /// Sample of quasi-identifier tuple values in this bucket. The total
            /// number of classes returned per bucket is capped at 20.
            #[prost(message, repeated, tag="6")]
            pub bucket_values: ::prost::alloc::vec::Vec<DeltaPresenceEstimationQuasiIdValues>,
            /// Total number of distinct quasi-identifier tuple values in this bucket.
            #[prost(int64, tag="7")]
            pub bucket_value_count: i64,
        }
    }
    /// Risk analysis options.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RequestedRiskAnalysisOptions {
        /// The job config for the risk job.
        #[prost(message, optional, tag="1")]
        pub job_config: ::core::option::Option<super::RiskAnalysisJobConfig>,
    }
    /// Values associated with this metric.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        /// Numerical stats result
        #[prost(message, tag="3")]
        NumericalStatsResult(NumericalStatsResult),
        /// Categorical stats result
        #[prost(message, tag="4")]
        CategoricalStatsResult(CategoricalStatsResult),
        /// K-anonymity result
        #[prost(message, tag="5")]
        KAnonymityResult(KAnonymityResult),
        /// L-divesity result
        #[prost(message, tag="6")]
        LDiversityResult(LDiversityResult),
        /// K-map result
        #[prost(message, tag="7")]
        KMapEstimationResult(KMapEstimationResult),
        /// Delta-presence result
        #[prost(message, tag="9")]
        DeltaPresenceEstimationResult(DeltaPresenceEstimationResult),
    }
}
/// A value of a field, including its frequency.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueFrequency {
    /// A value contained in the field in question.
    #[prost(message, optional, tag="1")]
    pub value: ::core::option::Option<Value>,
    /// How many times the value is contained in the field.
    #[prost(int64, tag="2")]
    pub count: i64,
}
/// Set of primitive values supported by the system.
/// Note that for the purposes of inspection or transformation, the number
/// of bytes considered to comprise a 'Value' is based on its representation
/// as a UTF-8 encoded string. For example, if 'integer_value' is set to
/// 123456789, the number of bytes would be counted as 9, even though an
/// int64 only holds up to 8 bytes of data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    /// Value types
    #[prost(oneof="value::Type", tags="1, 2, 3, 4, 5, 6, 7, 8")]
    pub r#type: ::core::option::Option<value::Type>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    /// Value types
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// integer
        #[prost(int64, tag="1")]
        IntegerValue(i64),
        /// float
        #[prost(double, tag="2")]
        FloatValue(f64),
        /// string
        #[prost(string, tag="3")]
        StringValue(::prost::alloc::string::String),
        /// boolean
        #[prost(bool, tag="4")]
        BooleanValue(bool),
        /// timestamp
        #[prost(message, tag="5")]
        TimestampValue(::prost_types::Timestamp),
        /// time of day
        #[prost(message, tag="6")]
        TimeValue(super::super::super::super::r#type::TimeOfDay),
        /// date
        #[prost(message, tag="7")]
        DateValue(super::super::super::super::r#type::Date),
        /// day of week
        #[prost(enumeration="super::super::super::super::r#type::DayOfWeek", tag="8")]
        DayOfWeekValue(i32),
    }
}
/// Message for infoType-dependent details parsed from quote.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuoteInfo {
    /// Object representation of the quote.
    #[prost(oneof="quote_info::ParsedQuote", tags="2")]
    pub parsed_quote: ::core::option::Option<quote_info::ParsedQuote>,
}
/// Nested message and enum types in `QuoteInfo`.
pub mod quote_info {
    /// Object representation of the quote.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ParsedQuote {
        /// The date time indicated by the quote.
        #[prost(message, tag="2")]
        DateTime(super::DateTime),
    }
}
/// Message for a date time object.
/// e.g. 2018-01-01, 5th August.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateTime {
    /// One or more of the following must be set.
    /// Must be a valid date or time value.
    #[prost(message, optional, tag="1")]
    pub date: ::core::option::Option<super::super::super::r#type::Date>,
    /// Day of week
    #[prost(enumeration="super::super::super::r#type::DayOfWeek", tag="2")]
    pub day_of_week: i32,
    /// Time of day
    #[prost(message, optional, tag="3")]
    pub time: ::core::option::Option<super::super::super::r#type::TimeOfDay>,
    /// Time zone
    #[prost(message, optional, tag="4")]
    pub time_zone: ::core::option::Option<date_time::TimeZone>,
}
/// Nested message and enum types in `DateTime`.
pub mod date_time {
    /// Time zone of the date time object.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TimeZone {
        /// Set only if the offset can be determined. Positive for time ahead of UTC.
        /// E.g. For "UTC-9", this value is -540.
        #[prost(int32, tag="1")]
        pub offset_minutes: i32,
    }
}
/// The configuration that controls how the data will change.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeidentifyConfig {
    /// Mode for handling transformation errors. If left unspecified, the default
    /// mode is `TransformationErrorHandling.ThrowError`.
    #[prost(message, optional, tag="3")]
    pub transformation_error_handling: ::core::option::Option<TransformationErrorHandling>,
    #[prost(oneof="deidentify_config::Transformation", tags="1, 2, 4")]
    pub transformation: ::core::option::Option<deidentify_config::Transformation>,
}
/// Nested message and enum types in `DeidentifyConfig`.
pub mod deidentify_config {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Transformation {
        /// Treat the dataset as free-form text and apply the same free text
        /// transformation everywhere.
        #[prost(message, tag="1")]
        InfoTypeTransformations(super::InfoTypeTransformations),
        /// Treat the dataset as structured. Transformations can be applied to
        /// specific locations within structured datasets, such as transforming
        /// a column within a table.
        #[prost(message, tag="2")]
        RecordTransformations(super::RecordTransformations),
        /// Treat the dataset as an image and redact.
        #[prost(message, tag="4")]
        ImageTransformations(super::ImageTransformations),
    }
}
/// A type of transformation that is applied over images.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageTransformations {
    #[prost(message, repeated, tag="2")]
    pub transforms: ::prost::alloc::vec::Vec<image_transformations::ImageTransformation>,
}
/// Nested message and enum types in `ImageTransformations`.
pub mod image_transformations {
    /// Configuration for determining how redaction of images should occur.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ImageTransformation {
        /// The color to use when redacting content from an image. If not
        /// specified, the default is black.
        #[prost(message, optional, tag="3")]
        pub redaction_color: ::core::option::Option<super::Color>,
        #[prost(oneof="image_transformation::Target", tags="4, 5, 6")]
        pub target: ::core::option::Option<image_transformation::Target>,
    }
    /// Nested message and enum types in `ImageTransformation`.
    pub mod image_transformation {
        /// Apply transformation to the selected info_types.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SelectedInfoTypes {
            /// Required. InfoTypes to apply the transformation to. Required. Provided InfoType
            /// must be unique within the ImageTransformations message.
            #[prost(message, repeated, tag="5")]
            pub info_types: ::prost::alloc::vec::Vec<super::super::InfoType>,
        }
        /// Apply transformation to all findings.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct AllInfoTypes {
        }
        /// Apply to all text.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct AllText {
        }
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Target {
            /// Apply transformation to the selected info_types.
            #[prost(message, tag="4")]
            SelectedInfoTypes(SelectedInfoTypes),
            /// Apply transformation to all findings not specified in other
            /// ImageTransformation's selected_info_types. Only one instance is allowed
            /// within the ImageTransformations message.
            #[prost(message, tag="5")]
            AllInfoTypes(AllInfoTypes),
            /// Apply transformation to all text that doesn't match an infoType. Only
            /// one instance is allowed within the ImageTransformations message.
            #[prost(message, tag="6")]
            AllText(AllText),
        }
    }
}
/// How to handle transformation errors during de-identification. A
/// transformation error occurs when the requested transformation is incompatible
/// with the data. For example, trying to de-identify an IP address using a
/// `DateShift` transformation would result in a transformation error, since date
/// info cannot be extracted from an IP address.
/// Information about any incompatible transformations, and how they were
/// handled, is returned in the response as part of the
/// `TransformationOverviews`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransformationErrorHandling {
    /// How transformation errors should be handled.
    #[prost(oneof="transformation_error_handling::Mode", tags="1, 2")]
    pub mode: ::core::option::Option<transformation_error_handling::Mode>,
}
/// Nested message and enum types in `TransformationErrorHandling`.
pub mod transformation_error_handling {
    /// Throw an error and fail the request when a transformation error occurs.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ThrowError {
    }
    /// Skips the data without modifying it if the requested transformation would
    /// cause an error. For example, if a `DateShift` transformation were applied
    /// an an IP address, this mode would leave the IP address unchanged in the
    /// response.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LeaveUntransformed {
    }
    /// How transformation errors should be handled.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Mode {
        /// Throw an error
        #[prost(message, tag="1")]
        ThrowError(ThrowError),
        /// Ignore errors
        #[prost(message, tag="2")]
        LeaveUntransformed(LeaveUntransformed),
    }
}
/// A rule for transforming a value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrimitiveTransformation {
    #[prost(oneof="primitive_transformation::Transformation", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 13")]
    pub transformation: ::core::option::Option<primitive_transformation::Transformation>,
}
/// Nested message and enum types in `PrimitiveTransformation`.
pub mod primitive_transformation {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Transformation {
        /// Replace with a specified value.
        #[prost(message, tag="1")]
        ReplaceConfig(super::ReplaceValueConfig),
        /// Redact
        #[prost(message, tag="2")]
        RedactConfig(super::RedactConfig),
        /// Mask
        #[prost(message, tag="3")]
        CharacterMaskConfig(super::CharacterMaskConfig),
        /// Ffx-Fpe
        #[prost(message, tag="4")]
        CryptoReplaceFfxFpeConfig(super::CryptoReplaceFfxFpeConfig),
        /// Fixed size bucketing
        #[prost(message, tag="5")]
        FixedSizeBucketingConfig(super::FixedSizeBucketingConfig),
        /// Bucketing
        #[prost(message, tag="6")]
        BucketingConfig(super::BucketingConfig),
        /// Replace with infotype
        #[prost(message, tag="7")]
        ReplaceWithInfoTypeConfig(super::ReplaceWithInfoTypeConfig),
        /// Time extraction
        #[prost(message, tag="8")]
        TimePartConfig(super::TimePartConfig),
        /// Crypto
        #[prost(message, tag="9")]
        CryptoHashConfig(super::CryptoHashConfig),
        /// Date Shift
        #[prost(message, tag="11")]
        DateShiftConfig(super::DateShiftConfig),
        /// Deterministic Crypto
        #[prost(message, tag="12")]
        CryptoDeterministicConfig(super::CryptoDeterministicConfig),
        /// Replace with a value randomly drawn (with replacement) from a dictionary.
        #[prost(message, tag="13")]
        ReplaceDictionaryConfig(super::ReplaceDictionaryConfig),
    }
}
/// For use with `Date`, `Timestamp`, and `TimeOfDay`, extract or preserve a
/// portion of the value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimePartConfig {
    /// The part of the time to keep.
    #[prost(enumeration="time_part_config::TimePart", tag="1")]
    pub part_to_extract: i32,
}
/// Nested message and enum types in `TimePartConfig`.
pub mod time_part_config {
    /// Components that make up time.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TimePart {
        /// Unused
        Unspecified = 0,
        /// \[0-9999\]
        Year = 1,
        /// \[1-12\]
        Month = 2,
        /// \[1-31\]
        DayOfMonth = 3,
        /// \[1-7\]
        DayOfWeek = 4,
        /// \[1-53\]
        WeekOfYear = 5,
        /// \[0-23\]
        HourOfDay = 6,
    }
}
/// Pseudonymization method that generates surrogates via cryptographic hashing.
/// Uses SHA-256.
/// The key size must be either 32 or 64 bytes.
/// Outputs a base64 encoded representation of the hashed output
/// (for example, L7k0BHmF1ha5U3NfGykjro4xWi1MPVQPjhMAZbSV9mM=).
/// Currently, only string and integer values can be hashed.
/// See <https://cloud.google.com/dlp/docs/pseudonymization> to learn more.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptoHashConfig {
    /// The key used by the hash function.
    #[prost(message, optional, tag="1")]
    pub crypto_key: ::core::option::Option<CryptoKey>,
}
/// Pseudonymization method that generates deterministic encryption for the given
/// input. Outputs a base64 encoded representation of the encrypted output.
/// Uses AES-SIV based on the RFC <https://tools.ietf.org/html/rfc5297.>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptoDeterministicConfig {
    /// The key used by the encryption function. For deterministic encryption
    /// using AES-SIV, the provided key is internally expanded to 64 bytes prior to
    /// use.
    #[prost(message, optional, tag="1")]
    pub crypto_key: ::core::option::Option<CryptoKey>,
    /// The custom info type to annotate the surrogate with.
    /// This annotation will be applied to the surrogate by prefixing it with
    /// the name of the custom info type followed by the number of
    /// characters comprising the surrogate. The following scheme defines the
    /// format: {info type name}({surrogate character count}):{surrogate}
    ///
    /// For example, if the name of custom info type is 'MY_TOKEN_INFO_TYPE' and
    /// the surrogate is 'abc', the full replacement value
    /// will be: 'MY_TOKEN_INFO_TYPE(3):abc'
    ///
    /// This annotation identifies the surrogate when inspecting content using the
    /// custom info type 'Surrogate'. This facilitates reversal of the
    /// surrogate when it occurs in free text.
    ///
    /// Note: For record transformations where the entire cell in a table is being
    /// transformed, surrogates are not mandatory. Surrogates are used to denote
    /// the location of the token and are necessary for re-identification in free
    /// form text.
    ///
    /// In order for inspection to work properly, the name of this info type must
    /// not occur naturally anywhere in your data; otherwise, inspection may either
    ///
    /// - reverse a surrogate that does not correspond to an actual identifier
    /// - be unable to parse the surrogate and result in an error
    ///
    /// Therefore, choose your custom info type name carefully after considering
    /// what your data looks like. One way to select a name that has a high chance
    /// of yielding reliable detection is to include one or more unicode characters
    /// that are highly improbable to exist in your data.
    /// For example, assuming your data is entered from a regular ASCII keyboard,
    /// the symbol with the hex code point 29DD might be used like so:
    /// ⧝MY_TOKEN_TYPE.
    #[prost(message, optional, tag="2")]
    pub surrogate_info_type: ::core::option::Option<InfoType>,
    /// A context may be used for higher security and maintaining
    /// referential integrity such that the same identifier in two different
    /// contexts will be given a distinct surrogate. The context is appended to
    /// plaintext value being encrypted. On decryption the provided context is
    /// validated against the value used during encryption. If a context was
    /// provided during encryption, same context must be provided during decryption
    /// as well.
    ///
    /// If the context is not set, plaintext would be used as is for encryption.
    /// If the context is set but:
    ///
    /// 1. there is no record present when transforming a given value or
    /// 2. the field is not present when transforming a given value,
    ///
    /// plaintext would be used as is for encryption.
    ///
    /// Note that case (1) is expected when an `InfoTypeTransformation` is
    /// applied to both structured and unstructured `ContentItem`s.
    #[prost(message, optional, tag="3")]
    pub context: ::core::option::Option<FieldId>,
}
/// Replace each input value with a given `Value`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplaceValueConfig {
    /// Value to replace it with.
    #[prost(message, optional, tag="1")]
    pub new_value: ::core::option::Option<Value>,
}
/// Replace each input value with a value randomly selected from the dictionary.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplaceDictionaryConfig {
    #[prost(oneof="replace_dictionary_config::Type", tags="1")]
    pub r#type: ::core::option::Option<replace_dictionary_config::Type>,
}
/// Nested message and enum types in `ReplaceDictionaryConfig`.
pub mod replace_dictionary_config {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// A list of words to select from for random replacement. The
        /// \[limits\](<https://cloud.google.com/dlp/limits>) page contains details about
        /// the size limits of dictionaries.
        #[prost(message, tag="1")]
        WordList(super::custom_info_type::dictionary::WordList),
    }
}
/// Replace each matching finding with the name of the info_type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplaceWithInfoTypeConfig {
}
/// Redact a given value. For example, if used with an `InfoTypeTransformation`
/// transforming PHONE_NUMBER, and input 'My phone number is 206-555-0123', the
/// output would be 'My phone number is '.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedactConfig {
}
/// Characters to skip when doing deidentification of a value. These will be left
/// alone and skipped.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CharsToIgnore {
    #[prost(oneof="chars_to_ignore::Characters", tags="1, 2")]
    pub characters: ::core::option::Option<chars_to_ignore::Characters>,
}
/// Nested message and enum types in `CharsToIgnore`.
pub mod chars_to_ignore {
    /// Convenience enum for indicating common characters to not transform.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CommonCharsToIgnore {
        /// Unused.
        Unspecified = 0,
        /// 0-9
        Numeric = 1,
        /// A-Z
        AlphaUpperCase = 2,
        /// a-z
        AlphaLowerCase = 3,
        /// US Punctuation, one of !"#$%&'()*+,-./:;<=>?@\[\\]^_`{|}~
        Punctuation = 4,
        /// Whitespace character, one of [ \t\n\x0B\f\r]
        Whitespace = 5,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Characters {
        /// Characters to not transform when masking.
        #[prost(string, tag="1")]
        CharactersToSkip(::prost::alloc::string::String),
        /// Common characters to not transform when masking. Useful to avoid removing
        /// punctuation.
        #[prost(enumeration="CommonCharsToIgnore", tag="2")]
        CommonCharactersToIgnore(i32),
    }
}
/// Partially mask a string by replacing a given number of characters with a
/// fixed character. Masking can start from the beginning or end of the string.
/// This can be used on data of any type (numbers, longs, and so on) and when
/// de-identifying structured data we'll attempt to preserve the original data's
/// type. (This allows you to take a long like 123 and modify it to a string like
/// **3.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CharacterMaskConfig {
    /// Character to use to mask the sensitive values&mdash;for example, `*` for an
    /// alphabetic string such as a name, or `0` for a numeric string such as ZIP
    /// code or credit card number. This string must have a length of 1. If not
    /// supplied, this value defaults to `*` for strings, and `0` for digits.
    #[prost(string, tag="1")]
    pub masking_character: ::prost::alloc::string::String,
    /// Number of characters to mask. If not set, all matching chars will be
    /// masked. Skipped characters do not count towards this tally.
    ///
    /// If `number_to_mask` is negative, this denotes inverse masking. Cloud DLP
    /// masks all but a number of characters.
    /// For example, suppose you have the following values:
    ///
    /// - `masking_character` is `*`
    /// - `number_to_mask` is `-4`
    /// - `reverse_order` is `false`
    /// - `CharsToIgnore` includes `-`
    /// - Input string is `1234-5678-9012-3456`
    ///
    /// The resulting de-identified string is
    /// `****-****-****-3456`. Cloud DLP masks all but the last four characters.
    /// If `reverse_order` is `true`, all but the first four characters are masked
    /// as `1234-****-****-****`.
    #[prost(int32, tag="2")]
    pub number_to_mask: i32,
    /// Mask characters in reverse order. For example, if `masking_character` is
    /// `0`, `number_to_mask` is `14`, and `reverse_order` is `false`, then the
    /// input string `1234-5678-9012-3456` is masked as `00000000000000-3456`.
    /// If `masking_character` is `*`, `number_to_mask` is `3`, and `reverse_order`
    /// is `true`, then the string `12345` is masked as `12***`.
    #[prost(bool, tag="3")]
    pub reverse_order: bool,
    /// When masking a string, items in this list will be skipped when replacing
    /// characters. For example, if the input string is `555-555-5555` and you
    /// instruct Cloud DLP to skip `-` and mask 5 characters with `*`, Cloud DLP
    /// returns `***-**5-5555`.
    #[prost(message, repeated, tag="4")]
    pub characters_to_ignore: ::prost::alloc::vec::Vec<CharsToIgnore>,
}
/// Buckets values based on fixed size ranges. The
/// Bucketing transformation can provide all of this functionality,
/// but requires more configuration. This message is provided as a convenience to
/// the user for simple bucketing strategies.
///
/// The transformed value will be a hyphenated string of
/// {lower_bound}-{upper_bound}. For example, if lower_bound = 10 and upper_bound
/// = 20, all values that are within this bucket will be replaced with "10-20".
///
/// This can be used on data of type: double, long.
///
/// If the bound Value type differs from the type of data
/// being transformed, we will first attempt converting the type of the data to
/// be transformed to match the type of the bound before comparing.
///
/// See <https://cloud.google.com/dlp/docs/concepts-bucketing> to learn more.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FixedSizeBucketingConfig {
    /// Required. Lower bound value of buckets. All values less than `lower_bound` are
    /// grouped together into a single bucket; for example if `lower_bound` = 10,
    /// then all values less than 10 are replaced with the value "-10".
    #[prost(message, optional, tag="1")]
    pub lower_bound: ::core::option::Option<Value>,
    /// Required. Upper bound value of buckets. All values greater than upper_bound are
    /// grouped together into a single bucket; for example if `upper_bound` = 89,
    /// then all values greater than 89 are replaced with the value "89+".
    #[prost(message, optional, tag="2")]
    pub upper_bound: ::core::option::Option<Value>,
    /// Required. Size of each bucket (except for minimum and maximum buckets). So if
    /// `lower_bound` = 10, `upper_bound` = 89, and `bucket_size` = 10, then the
    /// following buckets would be used: -10, 10-20, 20-30, 30-40, 40-50, 50-60,
    /// 60-70, 70-80, 80-89, 89+. Precision up to 2 decimals works.
    #[prost(double, tag="3")]
    pub bucket_size: f64,
}
/// Generalization function that buckets values based on ranges. The ranges and
/// replacement values are dynamically provided by the user for custom behavior,
/// such as 1-30 -> LOW 31-65 -> MEDIUM 66-100 -> HIGH
/// This can be used on
/// data of type: number, long, string, timestamp.
/// If the bound `Value` type differs from the type of data being transformed, we
/// will first attempt converting the type of the data to be transformed to match
/// the type of the bound before comparing.
/// See <https://cloud.google.com/dlp/docs/concepts-bucketing> to learn more.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BucketingConfig {
    /// Set of buckets. Ranges must be non-overlapping.
    #[prost(message, repeated, tag="1")]
    pub buckets: ::prost::alloc::vec::Vec<bucketing_config::Bucket>,
}
/// Nested message and enum types in `BucketingConfig`.
pub mod bucketing_config {
    /// Bucket is represented as a range, along with replacement values.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Bucket {
        /// Lower bound of the range, inclusive. Type should be the same as max if
        /// used.
        #[prost(message, optional, tag="1")]
        pub min: ::core::option::Option<super::Value>,
        /// Upper bound of the range, exclusive; type must match min.
        #[prost(message, optional, tag="2")]
        pub max: ::core::option::Option<super::Value>,
        /// Required. Replacement value for this bucket.
        #[prost(message, optional, tag="3")]
        pub replacement_value: ::core::option::Option<super::Value>,
    }
}
/// Replaces an identifier with a surrogate using Format Preserving Encryption
/// (FPE) with the FFX mode of operation; however when used in the
/// `ReidentifyContent` API method, it serves the opposite function by reversing
/// the surrogate back into the original identifier. The identifier must be
/// encoded as ASCII. For a given crypto key and context, the same identifier
/// will be replaced with the same surrogate. Identifiers must be at least two
/// characters long. In the case that the identifier is the empty string, it will
/// be skipped. See <https://cloud.google.com/dlp/docs/pseudonymization> to learn
/// more.
///
/// Note: We recommend using  CryptoDeterministicConfig for all use cases which
/// do not require preserving the input alphabet space and size, plus warrant
/// referential integrity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptoReplaceFfxFpeConfig {
    /// Required. The key used by the encryption algorithm.
    #[prost(message, optional, tag="1")]
    pub crypto_key: ::core::option::Option<CryptoKey>,
    /// The 'tweak', a context may be used for higher security since the same
    /// identifier in two different contexts won't be given the same surrogate. If
    /// the context is not set, a default tweak will be used.
    ///
    /// If the context is set but:
    ///
    /// 1. there is no record present when transforming a given value or
    /// 1. the field is not present when transforming a given value,
    ///
    /// a default tweak will be used.
    ///
    /// Note that case (1) is expected when an `InfoTypeTransformation` is
    /// applied to both structured and unstructured `ContentItem`s.
    /// Currently, the referenced field may be of value type integer or string.
    ///
    /// The tweak is constructed as a sequence of bytes in big endian byte order
    /// such that:
    ///
    /// - a 64 bit integer is encoded followed by a single byte of value 1
    /// - a string is encoded in UTF-8 format followed by a single byte of value 2
    #[prost(message, optional, tag="2")]
    pub context: ::core::option::Option<FieldId>,
    /// The custom infoType to annotate the surrogate with.
    /// This annotation will be applied to the surrogate by prefixing it with
    /// the name of the custom infoType followed by the number of
    /// characters comprising the surrogate. The following scheme defines the
    /// format: info_type_name(surrogate_character_count):surrogate
    ///
    /// For example, if the name of custom infoType is 'MY_TOKEN_INFO_TYPE' and
    /// the surrogate is 'abc', the full replacement value
    /// will be: 'MY_TOKEN_INFO_TYPE(3):abc'
    ///
    /// This annotation identifies the surrogate when inspecting content using the
    /// custom infoType
    /// \[`SurrogateType`\](<https://cloud.google.com/dlp/docs/reference/rest/v2/InspectConfig#surrogatetype>).
    /// This facilitates reversal of the surrogate when it occurs in free text.
    ///
    /// In order for inspection to work properly, the name of this infoType must
    /// not occur naturally anywhere in your data; otherwise, inspection may
    /// find a surrogate that does not correspond to an actual identifier.
    /// Therefore, choose your custom infoType name carefully after considering
    /// what your data looks like. One way to select a name that has a high chance
    /// of yielding reliable detection is to include one or more unicode characters
    /// that are highly improbable to exist in your data.
    /// For example, assuming your data is entered from a regular ASCII keyboard,
    /// the symbol with the hex code point 29DD might be used like so:
    /// ⧝MY_TOKEN_TYPE
    #[prost(message, optional, tag="8")]
    pub surrogate_info_type: ::core::option::Option<InfoType>,
    /// Choose an alphabet which the data being transformed will be made up of.
    #[prost(oneof="crypto_replace_ffx_fpe_config::Alphabet", tags="4, 5, 6")]
    pub alphabet: ::core::option::Option<crypto_replace_ffx_fpe_config::Alphabet>,
}
/// Nested message and enum types in `CryptoReplaceFfxFpeConfig`.
pub mod crypto_replace_ffx_fpe_config {
    /// These are commonly used subsets of the alphabet that the FFX mode
    /// natively supports. In the algorithm, the alphabet is selected using
    /// the "radix". Therefore each corresponds to a particular radix.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FfxCommonNativeAlphabet {
        /// Unused.
        Unspecified = 0,
        /// `\[0-9\]` (radix of 10)
        Numeric = 1,
        /// `\[0-9A-F\]` (radix of 16)
        Hexadecimal = 2,
        /// `\[0-9A-Z\]` (radix of 36)
        UpperCaseAlphaNumeric = 3,
        /// `\[0-9A-Za-z\]` (radix of 62)
        AlphaNumeric = 4,
    }
    /// Choose an alphabet which the data being transformed will be made up of.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Alphabet {
        /// Common alphabets.
        #[prost(enumeration="FfxCommonNativeAlphabet", tag="4")]
        CommonAlphabet(i32),
        /// This is supported by mapping these to the alphanumeric characters
        /// that the FFX mode natively supports. This happens before/after
        /// encryption/decryption.
        /// Each character listed must appear only once.
        /// Number of characters must be in the range [2, 95].
        /// This must be encoded as ASCII.
        /// The order of characters does not matter.
        /// The full list of allowed characters is:
        /// <code>0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz
        /// ~`!@#$%^&*()_-+={\[}\]|\:;"'<,>.?/</code>
        #[prost(string, tag="5")]
        CustomAlphabet(::prost::alloc::string::String),
        /// The native way to select the alphabet. Must be in the range [2, 95].
        #[prost(int32, tag="6")]
        Radix(i32),
    }
}
/// This is a data encryption key (DEK) (as opposed to
/// a key encryption key (KEK) stored by Cloud Key Management Service
/// (Cloud KMS).
/// When using Cloud KMS to wrap or unwrap a DEK, be sure to set an appropriate
/// IAM policy on the KEK to ensure an attacker cannot
/// unwrap the DEK.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptoKey {
    /// Sources of crypto keys.
    #[prost(oneof="crypto_key::Source", tags="1, 2, 3")]
    pub source: ::core::option::Option<crypto_key::Source>,
}
/// Nested message and enum types in `CryptoKey`.
pub mod crypto_key {
    /// Sources of crypto keys.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Transient crypto key
        #[prost(message, tag="1")]
        Transient(super::TransientCryptoKey),
        /// Unwrapped crypto key
        #[prost(message, tag="2")]
        Unwrapped(super::UnwrappedCryptoKey),
        /// Key wrapped using Cloud KMS
        #[prost(message, tag="3")]
        KmsWrapped(super::KmsWrappedCryptoKey),
    }
}
/// Use this to have a random data crypto key generated.
/// It will be discarded after the request finishes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransientCryptoKey {
    /// Required. Name of the key.
    /// This is an arbitrary string used to differentiate different keys.
    /// A unique key is generated per name: two separate `TransientCryptoKey`
    /// protos share the same generated key if their names are the same.
    /// When the data crypto key is generated, this name is not used in any way
    /// (repeating the api call will result in a different key being generated).
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Using raw keys is prone to security risks due to accidentally
/// leaking the key. Choose another type of key if possible.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnwrappedCryptoKey {
    /// Required. A 128/192/256 bit key.
    #[prost(bytes="bytes", tag="1")]
    pub key: ::prost::bytes::Bytes,
}
/// Include to use an existing data crypto key wrapped by KMS.
/// The wrapped key must be a 128-, 192-, or 256-bit key.
/// Authorization requires the following IAM permissions when sending a request
/// to perform a crypto transformation using a KMS-wrapped crypto key:
/// dlp.kms.encrypt
///
/// For more information, see [Creating a wrapped key]
/// (<https://cloud.google.com/dlp/docs/create-wrapped-key>).
///
/// Note: When you use Cloud KMS for cryptographic operations,
/// [charges apply](<https://cloud.google.com/kms/pricing>).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KmsWrappedCryptoKey {
    /// Required. The wrapped data crypto key.
    #[prost(bytes="bytes", tag="1")]
    pub wrapped_key: ::prost::bytes::Bytes,
    /// Required. The resource name of the KMS CryptoKey to use for unwrapping.
    #[prost(string, tag="2")]
    pub crypto_key_name: ::prost::alloc::string::String,
}
/// Shifts dates by random number of days, with option to be consistent for the
/// same context. See <https://cloud.google.com/dlp/docs/concepts-date-shifting>
/// to learn more.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateShiftConfig {
    /// Required. Range of shift in days. Actual shift will be selected at random within this
    /// range (inclusive ends). Negative means shift to earlier in time. Must not
    /// be more than 365250 days (1000 years) each direction.
    ///
    /// For example, 3 means shift date to at most 3 days into the future.
    #[prost(int32, tag="1")]
    pub upper_bound_days: i32,
    /// Required. For example, -5 means shift date to at most 5 days back in the past.
    #[prost(int32, tag="2")]
    pub lower_bound_days: i32,
    /// Points to the field that contains the context, for example, an entity id.
    /// If set, must also set cryptoKey. If set, shift will be consistent for the
    /// given context.
    #[prost(message, optional, tag="3")]
    pub context: ::core::option::Option<FieldId>,
    /// Method for calculating shift that takes context into consideration. If
    /// set, must also set context. Can only be applied to table items.
    #[prost(oneof="date_shift_config::Method", tags="4")]
    pub method: ::core::option::Option<date_shift_config::Method>,
}
/// Nested message and enum types in `DateShiftConfig`.
pub mod date_shift_config {
    /// Method for calculating shift that takes context into consideration. If
    /// set, must also set context. Can only be applied to table items.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Method {
        /// Causes the shift to be computed based on this key and the context. This
        /// results in the same shift for the same context and crypto_key. If
        /// set, must also set context. Can only be applied to table items.
        #[prost(message, tag="4")]
        CryptoKey(super::CryptoKey),
    }
}
/// A type of transformation that will scan unstructured text and
/// apply various `PrimitiveTransformation`s to each finding, where the
/// transformation is applied to only values that were identified as a specific
/// info_type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoTypeTransformations {
    /// Required. Transformation for each infoType. Cannot specify more than one
    /// for a given infoType.
    #[prost(message, repeated, tag="1")]
    pub transformations: ::prost::alloc::vec::Vec<info_type_transformations::InfoTypeTransformation>,
}
/// Nested message and enum types in `InfoTypeTransformations`.
pub mod info_type_transformations {
    /// A transformation to apply to text that is identified as a specific
    /// info_type.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InfoTypeTransformation {
        /// InfoTypes to apply the transformation to. An empty list will cause
        /// this transformation to apply to all findings that correspond to
        /// infoTypes that were requested in `InspectConfig`.
        #[prost(message, repeated, tag="1")]
        pub info_types: ::prost::alloc::vec::Vec<super::InfoType>,
        /// Required. Primitive transformation to apply to the infoType.
        #[prost(message, optional, tag="2")]
        pub primitive_transformation: ::core::option::Option<super::PrimitiveTransformation>,
    }
}
/// The transformation to apply to the field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldTransformation {
    /// Required. Input field(s) to apply the transformation to.
    /// When you have columns that reference their position within a list,
    /// omit the index from the FieldId. FieldId name matching ignores the index.
    /// For example, instead of "contact.nums\[0\].type", use "contact.nums.type".
    #[prost(message, repeated, tag="1")]
    pub fields: ::prost::alloc::vec::Vec<FieldId>,
    /// Only apply the transformation if the condition evaluates to true for the
    /// given `RecordCondition`. The conditions are allowed to reference fields
    /// that are not used in the actual transformation.
    ///
    /// Example Use Cases:
    ///
    /// - Apply a different bucket transformation to an age column if the zip code
    /// column for the same record is within a specific range.
    /// - Redact a field if the date of birth field is greater than 85.
    #[prost(message, optional, tag="3")]
    pub condition: ::core::option::Option<RecordCondition>,
    /// Transformation to apply. \[required\]
    #[prost(oneof="field_transformation::Transformation", tags="4, 5")]
    pub transformation: ::core::option::Option<field_transformation::Transformation>,
}
/// Nested message and enum types in `FieldTransformation`.
pub mod field_transformation {
    /// Transformation to apply. \[required\]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Transformation {
        /// Apply the transformation to the entire field.
        #[prost(message, tag="4")]
        PrimitiveTransformation(super::PrimitiveTransformation),
        /// Treat the contents of the field as free text, and selectively
        /// transform content that matches an `InfoType`.
        #[prost(message, tag="5")]
        InfoTypeTransformations(super::InfoTypeTransformations),
    }
}
/// A type of transformation that is applied over structured data such as a
/// table.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordTransformations {
    /// Transform the record by applying various field transformations.
    #[prost(message, repeated, tag="1")]
    pub field_transformations: ::prost::alloc::vec::Vec<FieldTransformation>,
    /// Configuration defining which records get suppressed entirely. Records that
    /// match any suppression rule are omitted from the output.
    #[prost(message, repeated, tag="2")]
    pub record_suppressions: ::prost::alloc::vec::Vec<RecordSuppression>,
}
/// Configuration to suppress records whose suppression conditions evaluate to
/// true.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordSuppression {
    /// A condition that when it evaluates to true will result in the record being
    /// evaluated to be suppressed from the transformed content.
    #[prost(message, optional, tag="1")]
    pub condition: ::core::option::Option<RecordCondition>,
}
/// A condition for determining whether a transformation should be applied to
/// a field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordCondition {
    /// An expression.
    #[prost(message, optional, tag="3")]
    pub expressions: ::core::option::Option<record_condition::Expressions>,
}
/// Nested message and enum types in `RecordCondition`.
pub mod record_condition {
    /// The field type of `value` and `field` do not need to match to be
    /// considered equal, but not all comparisons are possible.
    /// EQUAL_TO and NOT_EQUAL_TO attempt to compare even with incompatible types,
    /// but all other comparisons are invalid with incompatible types.
    /// A `value` of type:
    ///
    /// - `string` can be compared against all other types
    /// - `boolean` can only be compared against other booleans
    /// - `integer` can be compared against doubles or a string if the string value
    /// can be parsed as an integer.
    /// - `double` can be compared against integers or a string if the string can
    /// be parsed as a double.
    /// - `Timestamp` can be compared against strings in RFC 3339 date string
    /// format.
    /// - `TimeOfDay` can be compared against timestamps and strings in the format
    /// of 'HH:mm:ss'.
    ///
    /// If we fail to compare do to type mismatch, a warning will be given and
    /// the condition will evaluate to false.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Condition {
        /// Required. Field within the record this condition is evaluated against.
        #[prost(message, optional, tag="1")]
        pub field: ::core::option::Option<super::FieldId>,
        /// Required. Operator used to compare the field or infoType to the value.
        #[prost(enumeration="super::RelationalOperator", tag="3")]
        pub operator: i32,
        /// Value to compare against. [Mandatory, except for `EXISTS` tests.]
        #[prost(message, optional, tag="4")]
        pub value: ::core::option::Option<super::Value>,
    }
    /// A collection of conditions.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Conditions {
        /// A collection of conditions.
        #[prost(message, repeated, tag="1")]
        pub conditions: ::prost::alloc::vec::Vec<Condition>,
    }
    /// An expression, consisting of an operator and conditions.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Expressions {
        /// The operator to apply to the result of conditions. Default and currently
        /// only supported value is `AND`.
        #[prost(enumeration="expressions::LogicalOperator", tag="1")]
        pub logical_operator: i32,
        /// Expression types.
        #[prost(oneof="expressions::Type", tags="3")]
        pub r#type: ::core::option::Option<expressions::Type>,
    }
    /// Nested message and enum types in `Expressions`.
    pub mod expressions {
        /// Logical operators for conditional checks.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum LogicalOperator {
            /// Unused
            Unspecified = 0,
            /// Conditional AND
            And = 1,
        }
        /// Expression types.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Type {
            /// Conditions to apply to the expression.
            #[prost(message, tag="3")]
            Conditions(super::Conditions),
        }
    }
}
/// Overview of the modifications that occurred.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransformationOverview {
    /// Total size in bytes that were transformed in some way.
    #[prost(int64, tag="2")]
    pub transformed_bytes: i64,
    /// Transformations applied to the dataset.
    #[prost(message, repeated, tag="3")]
    pub transformation_summaries: ::prost::alloc::vec::Vec<TransformationSummary>,
}
/// Summary of a single transformation.
/// Only one of 'transformation', 'field_transformation', or 'record_suppress'
/// will be set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransformationSummary {
    /// Set if the transformation was limited to a specific InfoType.
    #[prost(message, optional, tag="1")]
    pub info_type: ::core::option::Option<InfoType>,
    /// Set if the transformation was limited to a specific FieldId.
    #[prost(message, optional, tag="2")]
    pub field: ::core::option::Option<FieldId>,
    /// The specific transformation these stats apply to.
    #[prost(message, optional, tag="3")]
    pub transformation: ::core::option::Option<PrimitiveTransformation>,
    /// The field transformation that was applied.
    /// If multiple field transformations are requested for a single field,
    /// this list will contain all of them; otherwise, only one is supplied.
    #[prost(message, repeated, tag="5")]
    pub field_transformations: ::prost::alloc::vec::Vec<FieldTransformation>,
    /// The specific suppression option these stats apply to.
    #[prost(message, optional, tag="6")]
    pub record_suppress: ::core::option::Option<RecordSuppression>,
    /// Collection of all transformations that took place or had an error.
    #[prost(message, repeated, tag="4")]
    pub results: ::prost::alloc::vec::Vec<transformation_summary::SummaryResult>,
    /// Total size in bytes that were transformed in some way.
    #[prost(int64, tag="7")]
    pub transformed_bytes: i64,
}
/// Nested message and enum types in `TransformationSummary`.
pub mod transformation_summary {
    /// A collection that informs the user the number of times a particular
    /// `TransformationResultCode` and error details occurred.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SummaryResult {
        /// Number of transformations counted by this result.
        #[prost(int64, tag="1")]
        pub count: i64,
        /// Outcome of the transformation.
        #[prost(enumeration="TransformationResultCode", tag="2")]
        pub code: i32,
        /// A place for warnings or errors to show up if a transformation didn't
        /// work as expected.
        #[prost(string, tag="3")]
        pub details: ::prost::alloc::string::String,
    }
    /// Possible outcomes of transformations.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TransformationResultCode {
        /// Unused
        Unspecified = 0,
        /// Transformation completed without an error.
        Success = 1,
        /// Transformation had an error.
        Error = 2,
    }
}
/// A flattened description of a `PrimitiveTransformation` or
/// `RecordSuppression`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransformationDescription {
    /// The transformation type.
    #[prost(enumeration="TransformationType", tag="1")]
    pub r#type: i32,
    /// A description of the transformation. This is empty for a
    /// RECORD_SUPPRESSION, or is the output of calling toString() on the
    /// `PrimitiveTransformation` protocol buffer message for any other type of
    /// transformation.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// A human-readable string representation of the `RecordCondition`
    /// corresponding to this transformation. Set if a `RecordCondition` was used
    /// to determine whether or not to apply this transformation.
    ///
    /// Examples:
    ///     * (age_field > 85)
    ///     * (age_field <= 18)
    ///     * (zip_field exists)
    ///     * (zip_field == 01234) && (city_field != "Springville")
    ///     * (zip_field == 01234) && (age_field <= 18) && (city_field exists)
    #[prost(string, tag="3")]
    pub condition: ::prost::alloc::string::String,
    /// Set if the transformation was limited to a specific `InfoType`.
    #[prost(message, optional, tag="4")]
    pub info_type: ::core::option::Option<InfoType>,
}
/// Details about a single transformation. This object contains a description of
/// the transformation, information about whether the transformation was
/// successfully applied, and the precise location where the transformation
/// occurred. These details are stored in a user-specified BigQuery table.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransformationDetails {
    /// The name of the job that completed the transformation.
    #[prost(string, tag="1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The top level name of the container where the transformation is located
    /// (this will be the source file name or table name).
    #[prost(string, tag="2")]
    pub container_name: ::prost::alloc::string::String,
    /// Description of transformation. This would only contain more than one
    /// element if there were multiple matching transformations and which one to
    /// apply was ambiguous. Not set for states that contain no transformation,
    /// currently only state that contains no transformation is
    /// TransformationResultStateType.METADATA_UNRETRIEVABLE.
    #[prost(message, repeated, tag="3")]
    pub transformation: ::prost::alloc::vec::Vec<TransformationDescription>,
    /// Status of the transformation, if transformation was not successful, this
    /// will specify what caused it to fail, otherwise it will show that the
    /// transformation was successful.
    #[prost(message, optional, tag="4")]
    pub status_details: ::core::option::Option<TransformationResultStatus>,
    /// The number of bytes that were transformed. If transformation was
    /// unsuccessful or did not take place because there was no content to
    /// transform, this will be zero.
    #[prost(int64, tag="5")]
    pub transformed_bytes: i64,
    /// The precise location of the transformed content in the original container.
    #[prost(message, optional, tag="6")]
    pub transformation_location: ::core::option::Option<TransformationLocation>,
}
/// Specifies the location of a transformation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransformationLocation {
    /// Information about the functionality of the container where this finding
    /// occurred, if available.
    #[prost(enumeration="TransformationContainerType", tag="3")]
    pub container_type: i32,
    #[prost(oneof="transformation_location::LocationType", tags="1, 2")]
    pub location_type: ::core::option::Option<transformation_location::LocationType>,
}
/// Nested message and enum types in `TransformationLocation`.
pub mod transformation_location {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LocationType {
        /// For infotype transformations, link to the corresponding findings ID so
        /// that location information does not need to be duplicated. Each findings
        /// ID correlates to an entry in the findings output table, this table only
        /// gets created when users specify to save findings (add the save findings
        /// action to the request).
        #[prost(string, tag="1")]
        FindingId(::prost::alloc::string::String),
        /// For record transformations, provide a field and container information.
        #[prost(message, tag="2")]
        RecordTransformation(super::RecordTransformation),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordTransformation {
    /// For record transformations, provide a field.
    #[prost(message, optional, tag="1")]
    pub field_id: ::core::option::Option<FieldId>,
    /// Findings container modification timestamp, if applicable.
    #[prost(message, optional, tag="2")]
    pub container_timestamp: ::core::option::Option<::prost_types::Timestamp>,
    /// Container version, if available ("generation" for Cloud Storage).
    #[prost(string, tag="3")]
    pub container_version: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransformationResultStatus {
    /// Transformation result status type, this will be either SUCCESS, or it will
    /// be the reason for why the transformation was not completely successful.
    #[prost(enumeration="TransformationResultStatusType", tag="1")]
    pub result_status_type: i32,
    /// Detailed error codes and messages
    #[prost(message, optional, tag="2")]
    pub details: ::core::option::Option<super::super::super::rpc::Status>,
}
/// Config for storing transformation details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransformationDetailsStorageConfig {
    /// Location to store the transformation summary.
    #[prost(oneof="transformation_details_storage_config::Type", tags="1")]
    pub r#type: ::core::option::Option<transformation_details_storage_config::Type>,
}
/// Nested message and enum types in `TransformationDetailsStorageConfig`.
pub mod transformation_details_storage_config {
    /// Location to store the transformation summary.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// The BigQuery table in which to store the output. This may be an existing
        /// table or in a new table in an existing dataset.
        /// If table_id is not set a new one will be generated for you with the
        /// following format:
        /// dlp_googleapis_transformation_details_yyyy_mm_dd_\[dlp_job_id\]. Pacific
        /// time zone will be used for generating the date details.
        #[prost(message, tag="1")]
        Table(super::BigQueryTable),
    }
}
/// Schedule for inspect job triggers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schedule {
    #[prost(oneof="schedule::Option", tags="1")]
    pub option: ::core::option::Option<schedule::Option>,
}
/// Nested message and enum types in `Schedule`.
pub mod schedule {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Option {
        /// With this option a job is started on a regular periodic basis. For
        /// example: every day (86400 seconds).
        ///
        /// A scheduled start time will be skipped if the previous
        /// execution has not ended when its scheduled time occurs.
        ///
        /// This value must be set to a time duration greater than or equal
        /// to 1 day and can be no longer than 60 days.
        #[prost(message, tag="1")]
        RecurrencePeriodDuration(::prost_types::Duration),
    }
}
/// Job trigger option for hybrid jobs. Jobs must be manually created
/// and finished.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Manual {
}
/// The inspectTemplate contains a configuration (set of types of sensitive data
/// to be detected) to be used anywhere you otherwise would normally specify
/// InspectConfig. See <https://cloud.google.com/dlp/docs/concepts-templates>
/// to learn more.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InspectTemplate {
    /// Output only. The template name.
    ///
    /// The template will have one of the following formats:
    /// `projects/PROJECT_ID/inspectTemplates/TEMPLATE_ID` OR
    /// `organizations/ORGANIZATION_ID/inspectTemplates/TEMPLATE_ID`;
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Display name (max 256 chars).
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    /// Short description (max 256 chars).
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The creation timestamp of an inspectTemplate.
    #[prost(message, optional, tag="4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last update timestamp of an inspectTemplate.
    #[prost(message, optional, tag="5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The core content of the template. Configuration of the scanning process.
    #[prost(message, optional, tag="6")]
    pub inspect_config: ::core::option::Option<InspectConfig>,
}
/// DeidentifyTemplates contains instructions on how to de-identify content.
/// See <https://cloud.google.com/dlp/docs/concepts-templates> to learn more.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeidentifyTemplate {
    /// Output only. The template name.
    ///
    /// The template will have one of the following formats:
    /// `projects/PROJECT_ID/deidentifyTemplates/TEMPLATE_ID` OR
    /// `organizations/ORGANIZATION_ID/deidentifyTemplates/TEMPLATE_ID`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Display name (max 256 chars).
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    /// Short description (max 256 chars).
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The creation timestamp of an inspectTemplate.
    #[prost(message, optional, tag="4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last update timestamp of an inspectTemplate.
    #[prost(message, optional, tag="5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The core content of the template.
    #[prost(message, optional, tag="6")]
    pub deidentify_config: ::core::option::Option<DeidentifyConfig>,
}
/// Details information about an error encountered during job execution or
/// the results of an unsuccessful activation of the JobTrigger.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    /// Detailed error codes and messages.
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::super::rpc::Status>,
    /// The times the error occurred.
    #[prost(message, repeated, tag="2")]
    pub timestamps: ::prost::alloc::vec::Vec<::prost_types::Timestamp>,
}
/// Contains a configuration to make dlp api calls on a repeating basis.
/// See <https://cloud.google.com/dlp/docs/concepts-job-triggers> to learn more.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobTrigger {
    /// Unique resource name for the triggeredJob, assigned by the service when the
    /// triggeredJob is created, for example
    /// `projects/dlp-test-project/jobTriggers/53234423`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Display name (max 100 chars)
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    /// User provided description (max 256 chars)
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    /// A list of triggers which will be OR'ed together. Only one in the list
    /// needs to trigger for a job to be started. The list may contain only
    /// a single Schedule trigger and must have at least one object.
    #[prost(message, repeated, tag="5")]
    pub triggers: ::prost::alloc::vec::Vec<job_trigger::Trigger>,
    /// Output only. A stream of errors encountered when the trigger was activated. Repeated
    /// errors may result in the JobTrigger automatically being paused.
    /// Will return the last 100 errors. Whenever the JobTrigger is modified
    /// this list will be cleared.
    #[prost(message, repeated, tag="6")]
    pub errors: ::prost::alloc::vec::Vec<Error>,
    /// Output only. The creation timestamp of a triggeredJob.
    #[prost(message, optional, tag="7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last update timestamp of a triggeredJob.
    #[prost(message, optional, tag="8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp of the last time this trigger executed.
    #[prost(message, optional, tag="9")]
    pub last_run_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. A status for this trigger.
    #[prost(enumeration="job_trigger::Status", tag="10")]
    pub status: i32,
    /// The configuration details for the specific type of job to run.
    #[prost(oneof="job_trigger::Job", tags="4")]
    pub job: ::core::option::Option<job_trigger::Job>,
}
/// Nested message and enum types in `JobTrigger`.
pub mod job_trigger {
    /// What event needs to occur for a new job to be started.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Trigger {
        #[prost(oneof="trigger::Trigger", tags="1, 2")]
        pub trigger: ::core::option::Option<trigger::Trigger>,
    }
    /// Nested message and enum types in `Trigger`.
    pub mod trigger {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Trigger {
            /// Create a job on a repeating basis based on the elapse of time.
            #[prost(message, tag="1")]
            Schedule(super::super::Schedule),
            /// For use with hybrid jobs. Jobs must be manually created and finished.
            #[prost(message, tag="2")]
            Manual(super::super::Manual),
        }
    }
    /// Whether the trigger is currently active. If PAUSED or CANCELLED, no jobs
    /// will be created with this configuration. The service may automatically
    /// pause triggers experiencing frequent errors. To restart a job, set the
    /// status to HEALTHY after correcting user errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        /// Unused.
        Unspecified = 0,
        /// Trigger is healthy.
        Healthy = 1,
        /// Trigger is temporarily paused.
        Paused = 2,
        /// Trigger is cancelled and can not be resumed.
        Cancelled = 3,
    }
    /// The configuration details for the specific type of job to run.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Job {
        /// For inspect jobs, a snapshot of the configuration.
        #[prost(message, tag="4")]
        InspectJob(super::InspectJobConfig),
    }
}
/// A task to execute on the completion of a job.
/// See <https://cloud.google.com/dlp/docs/concepts-actions> to learn more.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Action {
    #[prost(oneof="action::Action", tags="1, 2, 3, 5, 7, 8, 9")]
    pub action: ::core::option::Option<action::Action>,
}
/// Nested message and enum types in `Action`.
pub mod action {
    /// If set, the detailed findings will be persisted to the specified
    /// OutputStorageConfig. Only a single instance of this action can be
    /// specified.
    /// Compatible with: Inspect, Risk
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SaveFindings {
        /// Location to store findings outside of DLP.
        #[prost(message, optional, tag="1")]
        pub output_config: ::core::option::Option<super::OutputStorageConfig>,
    }
    /// Publish a message into a given Pub/Sub topic when DlpJob has completed. The
    /// message contains a single field, `DlpJobName`, which is equal to the
    /// finished job's
    /// \[`DlpJob.name`\](<https://cloud.google.com/dlp/docs/reference/rest/v2/projects.dlpJobs#DlpJob>).
    /// Compatible with: Inspect, Risk
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PublishToPubSub {
        /// Cloud Pub/Sub topic to send notifications to. The topic must have given
        /// publishing access rights to the DLP API service account executing
        /// the long running DlpJob sending the notifications.
        /// Format is projects/{project}/topics/{topic}.
        #[prost(string, tag="1")]
        pub topic: ::prost::alloc::string::String,
    }
    /// Publish the result summary of a DlpJob to the Cloud Security
    /// Command Center (CSCC Alpha).
    /// This action is only available for projects which are parts of
    /// an organization and whitelisted for the alpha Cloud Security Command
    /// Center.
    /// The action will publish the count of finding instances and their info
    /// types. The summary of findings will be persisted in CSCC and are governed
    /// by CSCC service-specific policy, see
    /// <https://cloud.google.com/terms/service-terms> Only a single instance of this
    /// action can be specified. Compatible with: Inspect
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PublishSummaryToCscc {
    }
    /// Publish findings of a DlpJob to Data Catalog. In Data Catalog, tag
    /// templates are applied to the resource that Cloud DLP scanned. Data
    /// Catalog tag templates are stored in the same project and region where the
    /// BigQuery table exists. For Cloud DLP to create and apply the tag template,
    /// the Cloud DLP service agent must have the
    /// `roles/datacatalog.tagTemplateOwner` permission on the project. The tag
    /// template contains fields summarizing the results of the DlpJob. Any field
    /// values previously written by another DlpJob are deleted. [InfoType naming
    /// patterns]\[google.privacy.dlp.v2.InfoType\] are strictly enforced when using
    /// this feature.
    ///
    /// Findings are persisted in Data Catalog storage and are governed by
    /// service-specific policies for Data Catalog. For more information, see
    /// [Service Specific Terms](<https://cloud.google.com/terms/service-terms>).
    ///
    /// Only a single instance of this action can be specified. This action is
    /// allowed only if all resources being scanned are BigQuery tables.
    /// Compatible with: Inspect
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PublishFindingsToCloudDataCatalog {
    }
    /// Create a de-identified copy of the requested table or files.
    ///
    /// A TransformationDetail will be created for each transformation.
    ///
    /// If any rows in BigQuery are skipped during de-identification
    /// (transformation errors or row size exceeds BigQuery insert API limits) they
    /// are placed in the failure output table. If the original row exceeds
    /// the BigQuery insert API limit it will be truncated when written to the
    /// failure output table. The failure output table can be set in the
    /// action.deidentify.output.big_query_output.deidentified_failure_output_table
    /// field, if no table is set, a table will be automatically created in the
    /// same project and dataset as the original table.
    ///
    /// Compatible with: Inspect
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Deidentify {
        /// User specified deidentify templates and configs for structured,
        /// unstructured, and image files.
        #[prost(message, optional, tag="7")]
        pub transformation_config: ::core::option::Option<super::TransformationConfig>,
        /// Config for storing transformation details. This is separate from the
        /// de-identified content, and contains metadata about the successful
        /// transformations and/or failures that occurred while de-identifying. This
        /// needs to be set in order for users to access information about the status
        /// of each transformation (see
        /// \[TransformationDetails][google.privacy.dlp.v2.TransformationDetails\]
        /// message for more information about what is noted).
        #[prost(message, optional, tag="3")]
        pub transformation_details_storage_config: ::core::option::Option<super::TransformationDetailsStorageConfig>,
        /// List of user-specified file type groups to transform. If specified, only
        /// the files with these filetypes will be transformed. If empty, all
        /// supported files will be transformed. Supported types may be automatically
        /// added over time. If a file type is set in this field that isn't supported
        /// by the Deidentify action then the job will fail and will not be
        /// successfully created/started. Currently the only filetypes supported are:
        /// IMAGES, TEXT_FILES, CSV, TSV.
        #[prost(enumeration="super::FileType", repeated, tag="8")]
        pub file_types_to_transform: ::prost::alloc::vec::Vec<i32>,
        #[prost(oneof="deidentify::Output", tags="9")]
        pub output: ::core::option::Option<deidentify::Output>,
    }
    /// Nested message and enum types in `Deidentify`.
    pub mod deidentify {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Output {
            /// Required. User settable Cloud Storage bucket and folders to store de-identified
            /// files. This field must be set for cloud storage deidentification. The
            /// output Cloud Storage bucket must be different from the input bucket.
            /// De-identified files will overwrite files in the output path.
            ///
            /// Form of: gs://bucket/folder/ or gs://bucket
            #[prost(string, tag="9")]
            CloudStorageOutput(::prost::alloc::string::String),
        }
    }
    /// Sends an email when the job completes. The email goes to IAM project owners
    /// and technical [Essential
    /// Contacts](<https://cloud.google.com/resource-manager/docs/managing-notification-contacts>).
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct JobNotificationEmails {
    }
    /// Enable Stackdriver metric dlp.googleapis.com/finding_count. This
    /// will publish a metric to stack driver on each infotype requested and
    /// how many findings were found for it. CustomDetectors will be bucketed
    /// as 'Custom' under the Stackdriver label 'info_type'.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PublishToStackdriver {
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        /// Save resulting findings in a provided location.
        #[prost(message, tag="1")]
        SaveFindings(SaveFindings),
        /// Publish a notification to a Pub/Sub topic.
        #[prost(message, tag="2")]
        PubSub(PublishToPubSub),
        /// Publish summary to Cloud Security Command Center (Alpha).
        #[prost(message, tag="3")]
        PublishSummaryToCscc(PublishSummaryToCscc),
        /// Publish findings to Cloud Datahub.
        #[prost(message, tag="5")]
        PublishFindingsToCloudDataCatalog(PublishFindingsToCloudDataCatalog),
        /// Create a de-identified copy of the input data.
        #[prost(message, tag="7")]
        Deidentify(Deidentify),
        /// Enable email notification for project owners and editors on job's
        /// completion/failure.
        #[prost(message, tag="8")]
        JobNotificationEmails(JobNotificationEmails),
        /// Enable Stackdriver metric dlp.googleapis.com/finding_count.
        #[prost(message, tag="9")]
        PublishToStackdriver(PublishToStackdriver),
    }
}
/// User specified templates and configs for how to deidentify structured,
/// unstructures, and image files. User must provide either a unstructured
/// deidentify template or at least one redact image config.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransformationConfig {
    /// De-identify template.
    /// If this template is specified, it will serve as the default de-identify
    /// template. This template cannot contain `record_transformations` since it
    /// can be used for unstructured content such as free-form text files. If this
    /// template is not set, a default `ReplaceWithInfoTypeConfig` will be used to
    /// de-identify unstructured content.
    #[prost(string, tag="1")]
    pub deidentify_template: ::prost::alloc::string::String,
    /// Structured de-identify template.
    /// If this template is specified, it will serve as the de-identify template
    /// for structured content such as delimited files and tables. If this template
    /// is not set but the `deidentify_template` is set, then `deidentify_template`
    /// will also apply to the structured content. If neither template is set, a
    /// default `ReplaceWithInfoTypeConfig` will be used to de-identify structured
    /// content.
    #[prost(string, tag="2")]
    pub structured_deidentify_template: ::prost::alloc::string::String,
    /// Image redact template.
    /// If this template is specified, it will serve as the de-identify template
    /// for images. If this template is not set, all findings in the image will be
    /// redacted with a black box.
    #[prost(string, tag="4")]
    pub image_redact_template: ::prost::alloc::string::String,
}
/// Request message for CreateInspectTemplate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInspectTemplateRequest {
    /// Required. Parent resource name.
    ///
    /// The format of this value varies depending on the scope of the request
    /// (project or organization) and whether you have [specified a processing
    /// location](<https://cloud.google.com/dlp/docs/specifying-location>):
    ///
    /// + Projects scope, location specified:<br/>
    ///   `projects/`<var>PROJECT_ID</var>`/locations/`<var>LOCATION_ID</var>
    /// + Projects scope, no location specified (defaults to global):<br/>
    ///   `projects/`<var>PROJECT_ID</var>
    /// + Organizations scope, location specified:<br/>
    ///   `organizations/`<var>ORG_ID</var>`/locations/`<var>LOCATION_ID</var>
    /// + Organizations scope, no location specified (defaults to global):<br/>
    ///   `organizations/`<var>ORG_ID</var>
    ///
    /// The following example `parent` string specifies a parent project with the
    /// identifier `example-project`, and specifies the `europe-west3` location
    /// for processing data:
    ///
    ///     parent=projects/example-project/locations/europe-west3
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The InspectTemplate to create.
    #[prost(message, optional, tag="2")]
    pub inspect_template: ::core::option::Option<InspectTemplate>,
    /// The template id can contain uppercase and lowercase letters,
    /// numbers, and hyphens; that is, it must match the regular
    /// expression: `\[a-zA-Z\d-_\]+`. The maximum length is 100
    /// characters. Can be empty to allow the system to generate one.
    #[prost(string, tag="3")]
    pub template_id: ::prost::alloc::string::String,
    /// Deprecated. This field has no effect.
    #[prost(string, tag="4")]
    pub location_id: ::prost::alloc::string::String,
}
/// Request message for UpdateInspectTemplate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInspectTemplateRequest {
    /// Required. Resource name of organization and inspectTemplate to be updated, for
    /// example `organizations/433245324/inspectTemplates/432452342` or
    /// projects/project-id/inspectTemplates/432452342.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// New InspectTemplate value.
    #[prost(message, optional, tag="2")]
    pub inspect_template: ::core::option::Option<InspectTemplate>,
    /// Mask to control which fields get updated.
    #[prost(message, optional, tag="3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for GetInspectTemplate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInspectTemplateRequest {
    /// Required. Resource name of the organization and inspectTemplate to be read, for
    /// example `organizations/433245324/inspectTemplates/432452342` or
    /// projects/project-id/inspectTemplates/432452342.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListInspectTemplates.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInspectTemplatesRequest {
    /// Required. Parent resource name.
    ///
    /// The format of this value varies depending on the scope of the request
    /// (project or organization) and whether you have [specified a processing
    /// location](<https://cloud.google.com/dlp/docs/specifying-location>):
    ///
    /// + Projects scope, location specified:<br/>
    ///   `projects/`<var>PROJECT_ID</var>`/locations/`<var>LOCATION_ID</var>
    /// + Projects scope, no location specified (defaults to global):<br/>
    ///   `projects/`<var>PROJECT_ID</var>
    /// + Organizations scope, location specified:<br/>
    ///   `organizations/`<var>ORG_ID</var>`/locations/`<var>LOCATION_ID</var>
    /// + Organizations scope, no location specified (defaults to global):<br/>
    ///   `organizations/`<var>ORG_ID</var>
    ///
    /// The following example `parent` string specifies a parent project with the
    /// identifier `example-project`, and specifies the `europe-west3` location
    /// for processing data:
    ///
    ///     parent=projects/example-project/locations/europe-west3
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Page token to continue retrieval. Comes from previous call
    /// to `ListInspectTemplates`.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// Size of the page, can be limited by the server. If zero server returns
    /// a page of max size 100.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// Comma separated list of fields to order by,
    /// followed by `asc` or `desc` postfix. This list is case-insensitive,
    /// default sorting order is ascending, redundant space characters are
    /// insignificant.
    ///
    /// Example: `name asc,update_time, create_time desc`
    ///
    /// Supported fields are:
    ///
    /// - `create_time`: corresponds to the time the template was created.
    /// - `update_time`: corresponds to the time the template was last updated.
    /// - `name`: corresponds to the template's name.
    /// - `display_name`: corresponds to the template's display name.
    #[prost(string, tag="4")]
    pub order_by: ::prost::alloc::string::String,
    /// Deprecated. This field has no effect.
    #[prost(string, tag="5")]
    pub location_id: ::prost::alloc::string::String,
}
/// Response message for ListInspectTemplates.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInspectTemplatesResponse {
    /// List of inspectTemplates, up to page_size in ListInspectTemplatesRequest.
    #[prost(message, repeated, tag="1")]
    pub inspect_templates: ::prost::alloc::vec::Vec<InspectTemplate>,
    /// If the next page is available then the next page token to be used
    /// in following ListInspectTemplates request.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for DeleteInspectTemplate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInspectTemplateRequest {
    /// Required. Resource name of the organization and inspectTemplate to be deleted, for
    /// example `organizations/433245324/inspectTemplates/432452342` or
    /// projects/project-id/inspectTemplates/432452342.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateJobTrigger.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateJobTriggerRequest {
    /// Required. Parent resource name.
    ///
    /// The format of this value varies depending on whether you have [specified a
    /// processing
    /// location](<https://cloud.google.com/dlp/docs/specifying-location>):
    ///
    /// + Projects scope, location specified:<br/>
    ///   `projects/`<var>PROJECT_ID</var>`/locations/`<var>LOCATION_ID</var>
    /// + Projects scope, no location specified (defaults to global):<br/>
    ///   `projects/`<var>PROJECT_ID</var>
    ///
    /// The following example `parent` string specifies a parent project with the
    /// identifier `example-project`, and specifies the `europe-west3` location
    /// for processing data:
    ///
    ///     parent=projects/example-project/locations/europe-west3
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The JobTrigger to create.
    #[prost(message, optional, tag="2")]
    pub job_trigger: ::core::option::Option<JobTrigger>,
    /// The trigger id can contain uppercase and lowercase letters,
    /// numbers, and hyphens; that is, it must match the regular
    /// expression: `\[a-zA-Z\d-_\]+`. The maximum length is 100
    /// characters. Can be empty to allow the system to generate one.
    #[prost(string, tag="3")]
    pub trigger_id: ::prost::alloc::string::String,
    /// Deprecated. This field has no effect.
    #[prost(string, tag="4")]
    pub location_id: ::prost::alloc::string::String,
}
/// Request message for ActivateJobTrigger.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateJobTriggerRequest {
    /// Required. Resource name of the trigger to activate, for example
    /// `projects/dlp-test-project/jobTriggers/53234423`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateJobTrigger.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateJobTriggerRequest {
    /// Required. Resource name of the project and the triggeredJob, for example
    /// `projects/dlp-test-project/jobTriggers/53234423`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// New JobTrigger value.
    #[prost(message, optional, tag="2")]
    pub job_trigger: ::core::option::Option<JobTrigger>,
    /// Mask to control which fields get updated.
    #[prost(message, optional, tag="3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for GetJobTrigger.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJobTriggerRequest {
    /// Required. Resource name of the project and the triggeredJob, for example
    /// `projects/dlp-test-project/jobTriggers/53234423`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateDlpJobRequest. Used to initiate long running
/// jobs such as calculating risk metrics or inspecting Google Cloud
/// Storage.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDlpJobRequest {
    /// Required. Parent resource name.
    ///
    /// The format of this value varies depending on whether you have [specified a
    /// processing
    /// location](<https://cloud.google.com/dlp/docs/specifying-location>):
    ///
    /// + Projects scope, location specified:<br/>
    ///   `projects/`<var>PROJECT_ID</var>`/locations/`<var>LOCATION_ID</var>
    /// + Projects scope, no location specified (defaults to global):<br/>
    ///   `projects/`<var>PROJECT_ID</var>
    ///
    /// The following example `parent` string specifies a parent project with the
    /// identifier `example-project`, and specifies the `europe-west3` location
    /// for processing data:
    ///
    ///     parent=projects/example-project/locations/europe-west3
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The job id can contain uppercase and lowercase letters,
    /// numbers, and hyphens; that is, it must match the regular
    /// expression: `\[a-zA-Z\d-_\]+`. The maximum length is 100
    /// characters. Can be empty to allow the system to generate one.
    #[prost(string, tag="4")]
    pub job_id: ::prost::alloc::string::String,
    /// Deprecated. This field has no effect.
    #[prost(string, tag="5")]
    pub location_id: ::prost::alloc::string::String,
    /// The configuration details for the specific type of job to run.
    #[prost(oneof="create_dlp_job_request::Job", tags="2, 3")]
    pub job: ::core::option::Option<create_dlp_job_request::Job>,
}
/// Nested message and enum types in `CreateDlpJobRequest`.
pub mod create_dlp_job_request {
    /// The configuration details for the specific type of job to run.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Job {
        /// An inspection job scans a storage repository for InfoTypes.
        #[prost(message, tag="2")]
        InspectJob(super::InspectJobConfig),
        /// A risk analysis job calculates re-identification risk metrics for a
        /// BigQuery table.
        #[prost(message, tag="3")]
        RiskJob(super::RiskAnalysisJobConfig),
    }
}
/// Request message for ListJobTriggers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobTriggersRequest {
    /// Required. Parent resource name.
    ///
    /// The format of this value varies depending on whether you have [specified a
    /// processing
    /// location](<https://cloud.google.com/dlp/docs/specifying-location>):
    ///
    /// + Projects scope, location specified:<br/>
    ///   `projects/`<var>PROJECT_ID</var>`/locations/`<var>LOCATION_ID</var>
    /// + Projects scope, no location specified (defaults to global):<br/>
    ///   `projects/`<var>PROJECT_ID</var>
    ///
    /// The following example `parent` string specifies a parent project with the
    /// identifier `example-project`, and specifies the `europe-west3` location
    /// for processing data:
    ///
    ///     parent=projects/example-project/locations/europe-west3
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Page token to continue retrieval. Comes from previous call
    /// to ListJobTriggers. `order_by` field must not
    /// change for subsequent calls.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// Size of the page, can be limited by a server.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// Comma separated list of triggeredJob fields to order by,
    /// followed by `asc` or `desc` postfix. This list is case-insensitive,
    /// default sorting order is ascending, redundant space characters are
    /// insignificant.
    ///
    /// Example: `name asc,update_time, create_time desc`
    ///
    /// Supported fields are:
    ///
    /// - `create_time`: corresponds to the time the JobTrigger was created.
    /// - `update_time`: corresponds to the time the JobTrigger was last updated.
    /// - `last_run_time`: corresponds to the last time the JobTrigger ran.
    /// - `name`: corresponds to the JobTrigger's name.
    /// - `display_name`: corresponds to the JobTrigger's display name.
    /// - `status`: corresponds to JobTrigger's status.
    #[prost(string, tag="4")]
    pub order_by: ::prost::alloc::string::String,
    /// Allows filtering.
    ///
    /// Supported syntax:
    ///
    /// * Filter expressions are made up of one or more restrictions.
    /// * Restrictions can be combined by `AND` or `OR` logical operators. A
    /// sequence of restrictions implicitly uses `AND`.
    /// * A restriction has the form of `{field} {operator} {value}`.
    /// * Supported fields/values for inspect triggers:
    ///     - `status` - HEALTHY|PAUSED|CANCELLED
    ///     - `inspected_storage` - DATASTORE|CLOUD_STORAGE|BIGQUERY
    ///     - 'last_run_time` - RFC 3339 formatted timestamp, surrounded by
    ///     quotation marks. Nanoseconds are ignored.
    ///     - 'error_count' - Number of errors that have occurred while running.
    /// * The operator must be `=` or `!=` for status and inspected_storage.
    ///
    /// Examples:
    ///
    /// * inspected_storage = cloud_storage AND status = HEALTHY
    /// * inspected_storage = cloud_storage OR inspected_storage = bigquery
    /// * inspected_storage = cloud_storage AND (state = PAUSED OR state = HEALTHY)
    /// * last_run_time > \"2017-12-12T00:00:00+00:00\"
    ///
    /// The length of this field should be no more than 500 characters.
    #[prost(string, tag="5")]
    pub filter: ::prost::alloc::string::String,
    /// The type of jobs. Will use `DlpJobType.INSPECT` if not set.
    #[prost(enumeration="DlpJobType", tag="6")]
    pub r#type: i32,
    /// Deprecated. This field has no effect.
    #[prost(string, tag="7")]
    pub location_id: ::prost::alloc::string::String,
}
/// Response message for ListJobTriggers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobTriggersResponse {
    /// List of triggeredJobs, up to page_size in ListJobTriggersRequest.
    #[prost(message, repeated, tag="1")]
    pub job_triggers: ::prost::alloc::vec::Vec<JobTrigger>,
    /// If the next page is available then the next page token to be used
    /// in following ListJobTriggers request.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for DeleteJobTrigger.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteJobTriggerRequest {
    /// Required. Resource name of the project and the triggeredJob, for example
    /// `projects/dlp-test-project/jobTriggers/53234423`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Controls what and how to inspect for findings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InspectJobConfig {
    /// The data to scan.
    #[prost(message, optional, tag="1")]
    pub storage_config: ::core::option::Option<StorageConfig>,
    /// How and what to scan for.
    #[prost(message, optional, tag="2")]
    pub inspect_config: ::core::option::Option<InspectConfig>,
    /// If provided, will be used as the default for all values in InspectConfig.
    /// `inspect_config` will be merged into the values persisted as part of the
    /// template.
    #[prost(string, tag="3")]
    pub inspect_template_name: ::prost::alloc::string::String,
    /// Actions to execute at the completion of the job.
    #[prost(message, repeated, tag="4")]
    pub actions: ::prost::alloc::vec::Vec<Action>,
}
/// A task to execute when a data profile has been generated.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataProfileAction {
    #[prost(oneof="data_profile_action::Action", tags="1, 2")]
    pub action: ::core::option::Option<data_profile_action::Action>,
}
/// Nested message and enum types in `DataProfileAction`.
pub mod data_profile_action {
    /// If set, the detailed data profiles will be persisted to the location
    /// of your choice whenever updated.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Export {
        /// Store all table and column profiles in an existing table or a new table
        /// in an existing dataset. Each re-generation will result in a new row in
        /// BigQuery.
        #[prost(message, optional, tag="1")]
        pub profile_table: ::core::option::Option<super::BigQueryTable>,
    }
    /// Send a Pub/Sub message into the given Pub/Sub topic to connect other
    /// systems to data profile generation. The message payload data will
    /// be the byte serialization of `DataProfilePubSubMessage`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PubSubNotification {
        /// Cloud Pub/Sub topic to send notifications to.
        /// Format is projects/{project}/topics/{topic}.
        #[prost(string, tag="1")]
        pub topic: ::prost::alloc::string::String,
        /// The type of event that triggers a Pub/Sub. At most one
        /// `PubSubNotification` per EventType is permitted.
        #[prost(enumeration="EventType", tag="2")]
        pub event: i32,
        /// Conditions (e.g., data risk or sensitivity level) for triggering a
        /// Pub/Sub.
        #[prost(message, optional, tag="3")]
        pub pubsub_condition: ::core::option::Option<super::DataProfilePubSubCondition>,
        /// How much data to include in the Pub/Sub message. If the user wishes to
        /// limit the size of the message, they can use resource_name and fetch the
        /// profile fields they wish to. Per table profile (not per column).
        #[prost(enumeration="pub_sub_notification::DetailLevel", tag="4")]
        pub detail_of_message: i32,
    }
    /// Nested message and enum types in `PubSubNotification`.
    pub mod pub_sub_notification {
        /// The levels of detail that can be included in the Pub/Sub message.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum DetailLevel {
            /// Unused.
            Unspecified = 0,
            /// The full table data profile.
            TableProfile = 1,
            /// The resource name of the table.
            ResourceName = 2,
        }
    }
    /// Types of event that can trigger an action.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EventType {
        /// Unused.
        Unspecified = 0,
        /// New profile (not a re-profile).
        NewProfile = 1,
        /// Changed one of the following profile metrics:
        /// * Table data risk score
        /// * Table sensitivity score
        /// * Table resource visibility
        /// * Table encryption type
        /// * Table predicted infoTypes
        /// * Table other infoTypes
        ChangedProfile = 2,
        /// Table data risk score or sensitivity score increased.
        ScoreIncreased = 3,
        /// A user (non-internal) error occurred.
        ErrorChanged = 4,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        /// Export data profiles into a provided location.
        #[prost(message, tag="1")]
        ExportData(Export),
        /// Publish a message into the Pub/Sub topic.
        #[prost(message, tag="2")]
        PubSubNotification(PubSubNotification),
    }
}
/// Configuration for setting up a job to scan resources for profile generation.
/// Only one data profile configuration may exist per organization, folder,
/// or project.
///
/// The generated data profiles are retained according to the
/// [data retention policy]
/// (<https://cloud.google.com/dlp/docs/data-profiles#retention>).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataProfileJobConfig {
    /// The data to scan.
    #[prost(message, optional, tag="1")]
    pub location: ::core::option::Option<DataProfileLocation>,
    /// The project that will run the scan. The DLP service
    /// account that exists within this project must have access to all resources
    /// that are profiled, and the Cloud DLP API must be enabled.
    #[prost(string, tag="5")]
    pub project_id: ::prost::alloc::string::String,
    /// Detection logic for profile generation.
    ///
    /// Not all template features are used by profiles. FindingLimits,
    /// include_quote and exclude_info_types have no impact on
    /// data profiling.
    ///
    /// Multiple templates may be provided if there is data in multiple regions.
    /// At most one template must be specified per-region (including "global").
    /// Each region is scanned using the applicable template. If no region-specific
    /// template is specified, but a "global" template is specified, it will be
    /// copied to that region and used instead. If no global or region-specific
    /// template is provided for a region with data, that region's data will not be
    /// scanned.
    ///
    /// For more information, see
    /// <https://cloud.google.com/dlp/docs/data-profiles#data_residency.>
    #[prost(string, repeated, tag="7")]
    pub inspect_templates: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Actions to execute at the completion of the job.
    #[prost(message, repeated, tag="6")]
    pub data_profile_actions: ::prost::alloc::vec::Vec<DataProfileAction>,
}
/// The data that will be profiled.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataProfileLocation {
    /// The location to be scanned.
    #[prost(oneof="data_profile_location::Location", tags="1, 2")]
    pub location: ::core::option::Option<data_profile_location::Location>,
}
/// Nested message and enum types in `DataProfileLocation`.
pub mod data_profile_location {
    /// The location to be scanned.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Location {
        /// The ID of an organization to scan.
        #[prost(int64, tag="1")]
        OrganizationId(i64),
        /// The ID of the Folder within an organization to scan.
        #[prost(int64, tag="2")]
        FolderId(i64),
    }
}
/// Combines all of the information about a DLP job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DlpJob {
    /// The server-assigned name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The type of job.
    #[prost(enumeration="DlpJobType", tag="2")]
    pub r#type: i32,
    /// State of a job.
    #[prost(enumeration="dlp_job::JobState", tag="3")]
    pub state: i32,
    /// Time when the job was created.
    #[prost(message, optional, tag="6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Time when the job started.
    #[prost(message, optional, tag="7")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Time when the job finished.
    #[prost(message, optional, tag="8")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// If created by a job trigger, the resource name of the trigger that
    /// instantiated the job.
    #[prost(string, tag="10")]
    pub job_trigger_name: ::prost::alloc::string::String,
    /// A stream of errors encountered running the job.
    #[prost(message, repeated, tag="11")]
    pub errors: ::prost::alloc::vec::Vec<Error>,
    #[prost(oneof="dlp_job::Details", tags="4, 5")]
    pub details: ::core::option::Option<dlp_job::Details>,
}
/// Nested message and enum types in `DlpJob`.
pub mod dlp_job {
    /// Possible states of a job. New items may be added.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum JobState {
        /// Unused.
        Unspecified = 0,
        /// The job has not yet started.
        Pending = 1,
        /// The job is currently running. Once a job has finished it will transition
        /// to FAILED or DONE.
        Running = 2,
        /// The job is no longer running.
        Done = 3,
        /// The job was canceled before it could be completed.
        Canceled = 4,
        /// The job had an error and did not complete.
        Failed = 5,
        /// The job is currently accepting findings via hybridInspect.
        /// A hybrid job in ACTIVE state may continue to have findings added to it
        /// through the calling of hybridInspect. After the job has finished no more
        /// calls to hybridInspect may be made. ACTIVE jobs can transition to DONE.
        Active = 6,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Details {
        /// Results from analyzing risk of a data source.
        #[prost(message, tag="4")]
        RiskDetails(super::AnalyzeDataSourceRiskDetails),
        /// Results from inspecting a data source.
        #[prost(message, tag="5")]
        InspectDetails(super::InspectDataSourceDetails),
    }
}
/// The request message for \[DlpJobs.GetDlpJob][\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDlpJobRequest {
    /// Required. The name of the DlpJob resource.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for listing DLP jobs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDlpJobsRequest {
    /// Required. Parent resource name.
    ///
    /// The format of this value varies depending on whether you have [specified a
    /// processing
    /// location](<https://cloud.google.com/dlp/docs/specifying-location>):
    ///
    /// + Projects scope, location specified:<br/>
    ///   `projects/`<var>PROJECT_ID</var>`/locations/`<var>LOCATION_ID</var>
    /// + Projects scope, no location specified (defaults to global):<br/>
    ///   `projects/`<var>PROJECT_ID</var>
    ///
    /// The following example `parent` string specifies a parent project with the
    /// identifier `example-project`, and specifies the `europe-west3` location
    /// for processing data:
    ///
    ///     parent=projects/example-project/locations/europe-west3
    #[prost(string, tag="4")]
    pub parent: ::prost::alloc::string::String,
    /// Allows filtering.
    ///
    /// Supported syntax:
    ///
    /// * Filter expressions are made up of one or more restrictions.
    /// * Restrictions can be combined by `AND` or `OR` logical operators. A
    /// sequence of restrictions implicitly uses `AND`.
    /// * A restriction has the form of `{field} {operator} {value}`.
    /// * Supported fields/values for inspect jobs:
    ///     - `state` - PENDING|RUNNING|CANCELED|FINISHED|FAILED
    ///     - `inspected_storage` - DATASTORE|CLOUD_STORAGE|BIGQUERY
    ///     - `trigger_name` - The name of the trigger that created the job.
    ///     - 'end_time` - Corresponds to the time the job finished.
    ///     - 'start_time` - Corresponds to the time the job finished.
    /// * Supported fields for risk analysis jobs:
    ///     - `state` - RUNNING|CANCELED|FINISHED|FAILED
    ///     - 'end_time` - Corresponds to the time the job finished.
    ///     - 'start_time` - Corresponds to the time the job finished.
    /// * The operator must be `=` or `!=`.
    ///
    /// Examples:
    ///
    /// * inspected_storage = cloud_storage AND state = done
    /// * inspected_storage = cloud_storage OR inspected_storage = bigquery
    /// * inspected_storage = cloud_storage AND (state = done OR state = canceled)
    /// * end_time > \"2017-12-12T00:00:00+00:00\"
    ///
    /// The length of this field should be no more than 500 characters.
    #[prost(string, tag="1")]
    pub filter: ::prost::alloc::string::String,
    /// The standard list page size.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// The standard list page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// The type of job. Defaults to `DlpJobType.INSPECT`
    #[prost(enumeration="DlpJobType", tag="5")]
    pub r#type: i32,
    /// Comma separated list of fields to order by,
    /// followed by `asc` or `desc` postfix. This list is case-insensitive,
    /// default sorting order is ascending, redundant space characters are
    /// insignificant.
    ///
    /// Example: `name asc, end_time asc, create_time desc`
    ///
    /// Supported fields are:
    ///
    /// - `create_time`: corresponds to the time the job was created.
    /// - `end_time`: corresponds to the time the job ended.
    /// - `name`: corresponds to the job's name.
    /// - `state`: corresponds to `state`
    #[prost(string, tag="6")]
    pub order_by: ::prost::alloc::string::String,
    /// Deprecated. This field has no effect.
    #[prost(string, tag="7")]
    pub location_id: ::prost::alloc::string::String,
}
/// The response message for listing DLP jobs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDlpJobsResponse {
    /// A list of DlpJobs that matches the specified filter in the request.
    #[prost(message, repeated, tag="1")]
    pub jobs: ::prost::alloc::vec::Vec<DlpJob>,
    /// The standard List next-page token.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for canceling a DLP job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelDlpJobRequest {
    /// Required. The name of the DlpJob resource to be cancelled.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for finishing a DLP hybrid job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishDlpJobRequest {
    /// Required. The name of the DlpJob resource to be cancelled.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for deleting a DLP job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDlpJobRequest {
    /// Required. The name of the DlpJob resource to be deleted.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateDeidentifyTemplate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDeidentifyTemplateRequest {
    /// Required. Parent resource name.
    ///
    /// The format of this value varies depending on the scope of the request
    /// (project or organization) and whether you have [specified a processing
    /// location](<https://cloud.google.com/dlp/docs/specifying-location>):
    ///
    /// + Projects scope, location specified:<br/>
    ///   `projects/`<var>PROJECT_ID</var>`/locations/`<var>LOCATION_ID</var>
    /// + Projects scope, no location specified (defaults to global):<br/>
    ///   `projects/`<var>PROJECT_ID</var>
    /// + Organizations scope, location specified:<br/>
    ///   `organizations/`<var>ORG_ID</var>`/locations/`<var>LOCATION_ID</var>
    /// + Organizations scope, no location specified (defaults to global):<br/>
    ///   `organizations/`<var>ORG_ID</var>
    ///
    /// The following example `parent` string specifies a parent project with the
    /// identifier `example-project`, and specifies the `europe-west3` location
    /// for processing data:
    ///
    ///     parent=projects/example-project/locations/europe-west3
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The DeidentifyTemplate to create.
    #[prost(message, optional, tag="2")]
    pub deidentify_template: ::core::option::Option<DeidentifyTemplate>,
    /// The template id can contain uppercase and lowercase letters,
    /// numbers, and hyphens; that is, it must match the regular
    /// expression: `\[a-zA-Z\d-_\]+`. The maximum length is 100
    /// characters. Can be empty to allow the system to generate one.
    #[prost(string, tag="3")]
    pub template_id: ::prost::alloc::string::String,
    /// Deprecated. This field has no effect.
    #[prost(string, tag="4")]
    pub location_id: ::prost::alloc::string::String,
}
/// Request message for UpdateDeidentifyTemplate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDeidentifyTemplateRequest {
    /// Required. Resource name of organization and deidentify template to be updated, for
    /// example `organizations/433245324/deidentifyTemplates/432452342` or
    /// projects/project-id/deidentifyTemplates/432452342.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// New DeidentifyTemplate value.
    #[prost(message, optional, tag="2")]
    pub deidentify_template: ::core::option::Option<DeidentifyTemplate>,
    /// Mask to control which fields get updated.
    #[prost(message, optional, tag="3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for GetDeidentifyTemplate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDeidentifyTemplateRequest {
    /// Required. Resource name of the organization and deidentify template to be read, for
    /// example `organizations/433245324/deidentifyTemplates/432452342` or
    /// projects/project-id/deidentifyTemplates/432452342.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListDeidentifyTemplates.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeidentifyTemplatesRequest {
    /// Required. Parent resource name.
    ///
    /// The format of this value varies depending on the scope of the request
    /// (project or organization) and whether you have [specified a processing
    /// location](<https://cloud.google.com/dlp/docs/specifying-location>):
    ///
    /// + Projects scope, location specified:<br/>
    ///   `projects/`<var>PROJECT_ID</var>`/locations/`<var>LOCATION_ID</var>
    /// + Projects scope, no location specified (defaults to global):<br/>
    ///   `projects/`<var>PROJECT_ID</var>
    /// + Organizations scope, location specified:<br/>
    ///   `organizations/`<var>ORG_ID</var>`/locations/`<var>LOCATION_ID</var>
    /// + Organizations scope, no location specified (defaults to global):<br/>
    ///   `organizations/`<var>ORG_ID</var>
    ///
    /// The following example `parent` string specifies a parent project with the
    /// identifier `example-project`, and specifies the `europe-west3` location
    /// for processing data:
    ///
    ///     parent=projects/example-project/locations/europe-west3
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Page token to continue retrieval. Comes from previous call
    /// to `ListDeidentifyTemplates`.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// Size of the page, can be limited by the server. If zero server returns
    /// a page of max size 100.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// Comma separated list of fields to order by,
    /// followed by `asc` or `desc` postfix. This list is case-insensitive,
    /// default sorting order is ascending, redundant space characters are
    /// insignificant.
    ///
    /// Example: `name asc,update_time, create_time desc`
    ///
    /// Supported fields are:
    ///
    /// - `create_time`: corresponds to the time the template was created.
    /// - `update_time`: corresponds to the time the template was last updated.
    /// - `name`: corresponds to the template's name.
    /// - `display_name`: corresponds to the template's display name.
    #[prost(string, tag="4")]
    pub order_by: ::prost::alloc::string::String,
    /// Deprecated. This field has no effect.
    #[prost(string, tag="5")]
    pub location_id: ::prost::alloc::string::String,
}
/// Response message for ListDeidentifyTemplates.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeidentifyTemplatesResponse {
    /// List of deidentify templates, up to page_size in
    /// ListDeidentifyTemplatesRequest.
    #[prost(message, repeated, tag="1")]
    pub deidentify_templates: ::prost::alloc::vec::Vec<DeidentifyTemplate>,
    /// If the next page is available then the next page token to be used
    /// in following ListDeidentifyTemplates request.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for DeleteDeidentifyTemplate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDeidentifyTemplateRequest {
    /// Required. Resource name of the organization and deidentify template to be deleted,
    /// for example `organizations/433245324/deidentifyTemplates/432452342` or
    /// projects/project-id/deidentifyTemplates/432452342.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Configuration for a custom dictionary created from a data source of any size
/// up to the maximum size defined in the
/// \[limits\](<https://cloud.google.com/dlp/limits>) page. The artifacts of
/// dictionary creation are stored in the specified Cloud Storage
/// location. Consider using `CustomInfoType.Dictionary` for smaller dictionaries
/// that satisfy the size requirements.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LargeCustomDictionaryConfig {
    /// Location to store dictionary artifacts in Cloud Storage. These files
    /// will only be accessible by project owners and the DLP API. If any of these
    /// artifacts are modified, the dictionary is considered invalid and can no
    /// longer be used.
    #[prost(message, optional, tag="1")]
    pub output_path: ::core::option::Option<CloudStoragePath>,
    #[prost(oneof="large_custom_dictionary_config::Source", tags="2, 3")]
    pub source: ::core::option::Option<large_custom_dictionary_config::Source>,
}
/// Nested message and enum types in `LargeCustomDictionaryConfig`.
pub mod large_custom_dictionary_config {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Set of files containing newline-delimited lists of dictionary phrases.
        #[prost(message, tag="2")]
        CloudStorageFileSet(super::CloudStorageFileSet),
        /// Field in a BigQuery table where each cell represents a dictionary phrase.
        #[prost(message, tag="3")]
        BigQueryField(super::BigQueryField),
    }
}
/// Summary statistics of a custom dictionary.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LargeCustomDictionaryStats {
    /// Approximate number of distinct phrases in the dictionary.
    #[prost(int64, tag="1")]
    pub approx_num_phrases: i64,
}
/// Configuration for stored infoTypes. All fields and subfield are provided
/// by the user. For more information, see
/// <https://cloud.google.com/dlp/docs/creating-custom-infotypes.>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoredInfoTypeConfig {
    /// Display name of the StoredInfoType (max 256 characters).
    #[prost(string, tag="1")]
    pub display_name: ::prost::alloc::string::String,
    /// Description of the StoredInfoType (max 256 characters).
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// Stored infotype types.
    #[prost(oneof="stored_info_type_config::Type", tags="3, 4, 5")]
    pub r#type: ::core::option::Option<stored_info_type_config::Type>,
}
/// Nested message and enum types in `StoredInfoTypeConfig`.
pub mod stored_info_type_config {
    /// Stored infotype types.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// StoredInfoType where findings are defined by a dictionary of phrases.
        #[prost(message, tag="3")]
        LargeCustomDictionary(super::LargeCustomDictionaryConfig),
        /// Store dictionary-based CustomInfoType.
        #[prost(message, tag="4")]
        Dictionary(super::custom_info_type::Dictionary),
        /// Store regular expression-based StoredInfoType.
        #[prost(message, tag="5")]
        Regex(super::custom_info_type::Regex),
    }
}
/// Statistics for a StoredInfoType.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoredInfoTypeStats {
    /// Stat types
    #[prost(oneof="stored_info_type_stats::Type", tags="1")]
    pub r#type: ::core::option::Option<stored_info_type_stats::Type>,
}
/// Nested message and enum types in `StoredInfoTypeStats`.
pub mod stored_info_type_stats {
    /// Stat types
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// StoredInfoType where findings are defined by a dictionary of phrases.
        #[prost(message, tag="1")]
        LargeCustomDictionary(super::LargeCustomDictionaryStats),
    }
}
/// Version of a StoredInfoType, including the configuration used to build it,
/// create timestamp, and current state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoredInfoTypeVersion {
    /// StoredInfoType configuration.
    #[prost(message, optional, tag="1")]
    pub config: ::core::option::Option<StoredInfoTypeConfig>,
    /// Create timestamp of the version. Read-only, determined by the system
    /// when the version is created.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Stored info type version state. Read-only, updated by the system
    /// during dictionary creation.
    #[prost(enumeration="StoredInfoTypeState", tag="3")]
    pub state: i32,
    /// Errors that occurred when creating this storedInfoType version, or
    /// anomalies detected in the storedInfoType data that render it unusable. Only
    /// the five most recent errors will be displayed, with the most recent error
    /// appearing first.
    ///
    /// For example, some of the data for stored custom dictionaries is put in
    /// the user's Cloud Storage bucket, and if this data is modified or
    /// deleted by the user or another system, the dictionary becomes invalid.
    ///
    /// If any errors occur, fix the problem indicated by the error message and
    /// use the UpdateStoredInfoType API method to create another version of the
    /// storedInfoType to continue using it, reusing the same `config` if it was
    /// not the source of the error.
    #[prost(message, repeated, tag="4")]
    pub errors: ::prost::alloc::vec::Vec<Error>,
    /// Statistics about this storedInfoType version.
    #[prost(message, optional, tag="5")]
    pub stats: ::core::option::Option<StoredInfoTypeStats>,
}
/// StoredInfoType resource message that contains information about the current
/// version and any pending updates.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoredInfoType {
    /// Resource name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Current version of the stored info type.
    #[prost(message, optional, tag="2")]
    pub current_version: ::core::option::Option<StoredInfoTypeVersion>,
    /// Pending versions of the stored info type. Empty if no versions are
    /// pending.
    #[prost(message, repeated, tag="3")]
    pub pending_versions: ::prost::alloc::vec::Vec<StoredInfoTypeVersion>,
}
/// Request message for CreateStoredInfoType.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStoredInfoTypeRequest {
    /// Required. Parent resource name.
    ///
    /// The format of this value varies depending on the scope of the request
    /// (project or organization) and whether you have [specified a processing
    /// location](<https://cloud.google.com/dlp/docs/specifying-location>):
    ///
    /// + Projects scope, location specified:<br/>
    ///   `projects/`<var>PROJECT_ID</var>`/locations/`<var>LOCATION_ID</var>
    /// + Projects scope, no location specified (defaults to global):<br/>
    ///   `projects/`<var>PROJECT_ID</var>
    /// + Organizations scope, location specified:<br/>
    ///   `organizations/`<var>ORG_ID</var>`/locations/`<var>LOCATION_ID</var>
    /// + Organizations scope, no location specified (defaults to global):<br/>
    ///   `organizations/`<var>ORG_ID</var>
    ///
    /// The following example `parent` string specifies a parent project with the
    /// identifier `example-project`, and specifies the `europe-west3` location
    /// for processing data:
    ///
    ///     parent=projects/example-project/locations/europe-west3
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Configuration of the storedInfoType to create.
    #[prost(message, optional, tag="2")]
    pub config: ::core::option::Option<StoredInfoTypeConfig>,
    /// The storedInfoType ID can contain uppercase and lowercase letters,
    /// numbers, and hyphens; that is, it must match the regular
    /// expression: `\[a-zA-Z\d-_\]+`. The maximum length is 100
    /// characters. Can be empty to allow the system to generate one.
    #[prost(string, tag="3")]
    pub stored_info_type_id: ::prost::alloc::string::String,
    /// Deprecated. This field has no effect.
    #[prost(string, tag="4")]
    pub location_id: ::prost::alloc::string::String,
}
/// Request message for UpdateStoredInfoType.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateStoredInfoTypeRequest {
    /// Required. Resource name of organization and storedInfoType to be updated, for
    /// example `organizations/433245324/storedInfoTypes/432452342` or
    /// projects/project-id/storedInfoTypes/432452342.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Updated configuration for the storedInfoType. If not provided, a new
    /// version of the storedInfoType will be created with the existing
    /// configuration.
    #[prost(message, optional, tag="2")]
    pub config: ::core::option::Option<StoredInfoTypeConfig>,
    /// Mask to control which fields get updated.
    #[prost(message, optional, tag="3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for GetStoredInfoType.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoredInfoTypeRequest {
    /// Required. Resource name of the organization and storedInfoType to be read, for
    /// example `organizations/433245324/storedInfoTypes/432452342` or
    /// projects/project-id/storedInfoTypes/432452342.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListStoredInfoTypes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStoredInfoTypesRequest {
    /// Required. Parent resource name.
    ///
    /// The format of this value varies depending on the scope of the request
    /// (project or organization) and whether you have [specified a processing
    /// location](<https://cloud.google.com/dlp/docs/specifying-location>):
    ///
    /// + Projects scope, location specified:<br/>
    ///   `projects/`<var>PROJECT_ID</var>`/locations/`<var>LOCATION_ID</var>
    /// + Projects scope, no location specified (defaults to global):<br/>
    ///   `projects/`<var>PROJECT_ID</var>
    ///
    /// The following example `parent` string specifies a parent project with the
    /// identifier `example-project`, and specifies the `europe-west3` location
    /// for processing data:
    ///
    ///     parent=projects/example-project/locations/europe-west3
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Page token to continue retrieval. Comes from previous call
    /// to `ListStoredInfoTypes`.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// Size of the page, can be limited by the server. If zero server returns
    /// a page of max size 100.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// Comma separated list of fields to order by,
    /// followed by `asc` or `desc` postfix. This list is case-insensitive,
    /// default sorting order is ascending, redundant space characters are
    /// insignificant.
    ///
    /// Example: `name asc, display_name, create_time desc`
    ///
    /// Supported fields are:
    ///
    /// - `create_time`: corresponds to the time the most recent version of the
    /// resource was created.
    /// - `state`: corresponds to the state of the resource.
    /// - `name`: corresponds to resource name.
    /// - `display_name`: corresponds to info type's display name.
    #[prost(string, tag="4")]
    pub order_by: ::prost::alloc::string::String,
    /// Deprecated. This field has no effect.
    #[prost(string, tag="5")]
    pub location_id: ::prost::alloc::string::String,
}
/// Response message for ListStoredInfoTypes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStoredInfoTypesResponse {
    /// List of storedInfoTypes, up to page_size in ListStoredInfoTypesRequest.
    #[prost(message, repeated, tag="1")]
    pub stored_info_types: ::prost::alloc::vec::Vec<StoredInfoType>,
    /// If the next page is available then the next page token to be used
    /// in following ListStoredInfoTypes request.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for DeleteStoredInfoType.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteStoredInfoTypeRequest {
    /// Required. Resource name of the organization and storedInfoType to be deleted, for
    /// example `organizations/433245324/storedInfoTypes/432452342` or
    /// projects/project-id/storedInfoTypes/432452342.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to search for potentially sensitive info in a custom location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HybridInspectJobTriggerRequest {
    /// Required. Resource name of the trigger to execute a hybrid inspect on, for example
    /// `projects/dlp-test-project/jobTriggers/53234423`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The item to inspect.
    #[prost(message, optional, tag="3")]
    pub hybrid_item: ::core::option::Option<HybridContentItem>,
}
/// Request to search for potentially sensitive info in a custom location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HybridInspectDlpJobRequest {
    /// Required. Resource name of the job to execute a hybrid inspect on, for example
    /// `projects/dlp-test-project/dlpJob/53234423`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The item to inspect.
    #[prost(message, optional, tag="3")]
    pub hybrid_item: ::core::option::Option<HybridContentItem>,
}
/// An individual hybrid item to inspect. Will be stored temporarily during
/// processing.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HybridContentItem {
    /// The item to inspect.
    #[prost(message, optional, tag="1")]
    pub item: ::core::option::Option<ContentItem>,
    /// Supplementary information that will be added to each finding.
    #[prost(message, optional, tag="2")]
    pub finding_details: ::core::option::Option<HybridFindingDetails>,
}
/// Populate to associate additional data with each finding.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HybridFindingDetails {
    /// Details about the container where the content being inspected is from.
    #[prost(message, optional, tag="1")]
    pub container_details: ::core::option::Option<Container>,
    /// Offset in bytes of the line, from the beginning of the file, where the
    /// finding  is located. Populate if the item being scanned is only part of a
    /// bigger item, such as a shard of a file and you want to track the absolute
    /// position of the finding.
    #[prost(int64, tag="2")]
    pub file_offset: i64,
    /// Offset of the row for tables. Populate if the row(s) being scanned are
    /// part of a bigger dataset and you want to keep track of their absolute
    /// position.
    #[prost(int64, tag="3")]
    pub row_offset: i64,
    /// If the container is a table, additional information to make findings
    /// meaningful such as the columns that are primary keys. If not known ahead
    /// of time, can also be set within each inspect hybrid call and the two
    /// will be merged. Note that identifying_fields will only be stored to
    /// BigQuery, and only if the BigQuery action has been included.
    #[prost(message, optional, tag="4")]
    pub table_options: ::core::option::Option<TableOptions>,
    /// Labels to represent user provided metadata about the data being inspected.
    /// If configured by the job, some key values may be required.
    /// The labels associated with `Finding`'s produced by hybrid
    /// inspection.
    ///
    /// Label keys must be between 1 and 63 characters long and must conform
    /// to the following regular expression: `\[a-z]([-a-z0-9]*[a-z0-9\])?`.
    ///
    /// Label values must be between 0 and 63 characters long and must conform
    /// to the regular expression `(\[a-z]([-a-z0-9]*[a-z0-9\])?)?`.
    ///
    /// No more than 10 labels can be associated with a given finding.
    ///
    /// Examples:
    /// * `"environment" : "production"`
    /// * `"pipeline" : "etl"`
    #[prost(btree_map="string, string", tag="5")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Quota exceeded errors will be thrown once quota has been met.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HybridInspectResponse {
}
/// Score is a summary of all elements in the data profile.
/// A higher number means more risk.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataRiskLevel {
    /// The score applied to the resource.
    #[prost(enumeration="data_risk_level::DataRiskLevelScore", tag="1")]
    pub score: i32,
}
/// Nested message and enum types in `DataRiskLevel`.
pub mod data_risk_level {
    /// Various score levels for resources.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DataRiskLevelScore {
        /// Unused.
        RiskScoreUnspecified = 0,
        /// Low risk - Lower indication of sensitive data that appears to have
        /// additional access restrictions in place or no indication of sensitive
        /// data found.
        RiskLow = 10,
        /// Medium risk - Sensitive data may be present but additional access or fine
        /// grain access restrictions appear to be present.  Consider limiting
        /// access even further or transform data to mask.
        RiskModerate = 20,
        /// High risk – SPII may be present. Access controls may include public
        /// ACLs. Exfiltration of data may lead to user data loss. Re-identification
        /// of users may be possible. Consider limiting usage and or removing SPII.
        RiskHigh = 30,
    }
}
/// Snapshot of the configurations used to generate the profile.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataProfileConfigSnapshot {
    /// A copy of the inspection config used to generate this profile. This
    /// is a copy of the inspect_template specified in `DataProfileJobConfig`.
    #[prost(message, optional, tag="2")]
    pub inspect_config: ::core::option::Option<InspectConfig>,
    /// A copy of the configuration used to generate this profile.
    #[prost(message, optional, tag="3")]
    pub data_profile_job: ::core::option::Option<DataProfileJobConfig>,
}
/// The profile for a scanned table.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableDataProfile {
    /// The name of the profile.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The resource name to the project data profile for this table.
    #[prost(string, tag="2")]
    pub project_data_profile: ::prost::alloc::string::String,
    /// The GCP project ID that owns the BigQuery dataset.
    #[prost(string, tag="24")]
    pub dataset_project_id: ::prost::alloc::string::String,
    /// The BigQuery location where the dataset's data is stored.
    /// See <https://cloud.google.com/bigquery/docs/locations> for supported
    /// locations.
    #[prost(string, tag="29")]
    pub dataset_location: ::prost::alloc::string::String,
    /// The BigQuery dataset ID.
    #[prost(string, tag="25")]
    pub dataset_id: ::prost::alloc::string::String,
    /// The BigQuery table ID.
    #[prost(string, tag="26")]
    pub table_id: ::prost::alloc::string::String,
    /// The resource name of the table.
    /// <https://cloud.google.com/apis/design/resource_names#full_resource_name>
    #[prost(string, tag="3")]
    pub full_resource: ::prost::alloc::string::String,
    /// Success or error status from the most recent profile generation attempt.
    /// May be empty if the profile is still being generated.
    #[prost(message, optional, tag="21")]
    pub profile_status: ::core::option::Option<ProfileStatus>,
    /// State of a profile.
    #[prost(enumeration="table_data_profile::State", tag="22")]
    pub state: i32,
    /// The sensitivity score of this table.
    #[prost(message, optional, tag="5")]
    pub sensitivity_score: ::core::option::Option<SensitivityScore>,
    /// The data risk level of this table.
    #[prost(message, optional, tag="6")]
    pub data_risk_level: ::core::option::Option<DataRiskLevel>,
    /// The infoTypes predicted from this table's data.
    #[prost(message, repeated, tag="27")]
    pub predicted_info_types: ::prost::alloc::vec::Vec<InfoTypeSummary>,
    /// Other infoTypes found in this table's data.
    #[prost(message, repeated, tag="28")]
    pub other_info_types: ::prost::alloc::vec::Vec<OtherInfoTypeSummary>,
    /// The snapshot of the configurations used to generate the profile.
    #[prost(message, optional, tag="7")]
    pub config_snapshot: ::core::option::Option<DataProfileConfigSnapshot>,
    /// The time when this table was last modified
    #[prost(message, optional, tag="8")]
    pub last_modified_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. The time when this table expires.
    #[prost(message, optional, tag="9")]
    pub expiration_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The number of columns profiled in the table.
    #[prost(int64, tag="10")]
    pub scanned_column_count: i64,
    /// The number of columns skipped in the table because of an error.
    #[prost(int64, tag="11")]
    pub failed_column_count: i64,
    /// The size of the table when the profile was generated.
    #[prost(int64, tag="12")]
    pub table_size_bytes: i64,
    /// Number of rows in the table when the profile was generated.
    #[prost(int64, tag="13")]
    pub row_count: i64,
    /// How the table is encrypted.
    #[prost(enumeration="EncryptionStatus", tag="14")]
    pub encryption_status: i32,
    /// How broadly a resource has been shared.
    #[prost(enumeration="ResourceVisibility", tag="15")]
    pub resource_visibility: i32,
    /// The last time the profile was generated.
    #[prost(message, optional, tag="16")]
    pub profile_last_generated: ::core::option::Option<::prost_types::Timestamp>,
    /// The labels applied to the resource at the time the profile was generated.
    #[prost(btree_map="string, string", tag="17")]
    pub resource_labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// The time at which the table was created.
    #[prost(message, optional, tag="23")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `TableDataProfile`.
pub mod table_data_profile {
    /// Possible states of a profile. New items may be added.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unused.
        Unspecified = 0,
        /// The profile is currently running. Once a profile has finished it will
        /// transition to DONE.
        Running = 1,
        /// The profile is no longer generating.
        /// If profile_status.status.code is 0, the profile succeeded, otherwise, it
        /// failed.
        Done = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProfileStatus {
    /// Profiling status code and optional message
    #[prost(message, optional, tag="1")]
    pub status: ::core::option::Option<super::super::super::rpc::Status>,
    /// Time when the profile generation status was updated
    #[prost(message, optional, tag="3")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
/// The infoType details for this column.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoTypeSummary {
    /// The infoType.
    #[prost(message, optional, tag="1")]
    pub info_type: ::core::option::Option<InfoType>,
    /// Approximate percentage of non-null rows that contained data detected by
    /// this infotype.
    #[prost(int32, tag="2")]
    pub estimated_prevalence: i32,
}
/// Infotype details for other infoTypes found within a column.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OtherInfoTypeSummary {
    /// The other infoType.
    #[prost(message, optional, tag="1")]
    pub info_type: ::core::option::Option<InfoType>,
    /// Approximate percentage of non-null rows that contained data detected by
    /// this infotype.
    #[prost(int32, tag="2")]
    pub estimated_prevalence: i32,
}
/// A condition for determining whether a Pub/Sub should be triggered.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataProfilePubSubCondition {
    /// An expression.
    #[prost(message, optional, tag="1")]
    pub expressions: ::core::option::Option<data_profile_pub_sub_condition::PubSubExpressions>,
}
/// Nested message and enum types in `DataProfilePubSubCondition`.
pub mod data_profile_pub_sub_condition {
    /// A condition consisting of a value.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PubSubCondition {
        /// The value for the condition to trigger.
        #[prost(oneof="pub_sub_condition::Value", tags="1, 2")]
        pub value: ::core::option::Option<pub_sub_condition::Value>,
    }
    /// Nested message and enum types in `PubSubCondition`.
    pub mod pub_sub_condition {
        /// The value for the condition to trigger.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Value {
            /// The minimum data risk score that triggers the condition.
            #[prost(enumeration="super::ProfileScoreBucket", tag="1")]
            MinimumRiskScore(i32),
            /// The minimum sensitivity level that triggers the condition.
            #[prost(enumeration="super::ProfileScoreBucket", tag="2")]
            MinimumSensitivityScore(i32),
        }
    }
    /// An expression, consisting of an operator and conditions.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PubSubExpressions {
        /// The operator to apply to the collection of conditions.
        #[prost(enumeration="pub_sub_expressions::PubSubLogicalOperator", tag="1")]
        pub logical_operator: i32,
        /// Conditions to apply to the expression.
        #[prost(message, repeated, tag="2")]
        pub conditions: ::prost::alloc::vec::Vec<PubSubCondition>,
    }
    /// Nested message and enum types in `PubSubExpressions`.
    pub mod pub_sub_expressions {
        /// Logical operators for conditional checks.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum PubSubLogicalOperator {
            /// Unused.
            LogicalOperatorUnspecified = 0,
            /// Conditional OR.
            Or = 1,
            /// Conditional AND.
            And = 2,
        }
    }
    /// Various score levels for resources.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ProfileScoreBucket {
        /// Unused.
        Unspecified = 0,
        /// High risk/sensitivity detected.
        High = 1,
        /// Medium or high risk/sensitivity detected.
        MediumOrHigh = 2,
    }
}
/// Pub/Sub topic message for a DataProfileAction.PubSubNotification event.
/// To receive a message of protocol buffer schema type, convert the message data
/// to an object of this proto class.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataProfilePubSubMessage {
    /// If `DetailLevel` is `TABLE_PROFILE` this will be fully populated.
    /// Otherwise, if `DetailLevel` is `RESOURCE_NAME`, then only `name` and
    /// `full_resource` will be populated.
    #[prost(message, optional, tag="1")]
    pub profile: ::core::option::Option<TableDataProfile>,
    /// The event that caused the Pub/Sub message to be sent.
    #[prost(enumeration="data_profile_action::EventType", tag="2")]
    pub event: i32,
}
/// Enum of possible outcomes of transformations. SUCCESS if transformation and
/// storing of transformation was successful, otherwise, reason for not
/// transforming.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransformationResultStatusType {
    StateTypeUnspecified = 0,
    /// This will be set when a finding could not be transformed (i.e. outside user
    /// set bucket range).
    InvalidTransform = 1,
    /// This will be set when a BigQuery transformation was successful but could
    /// not be stored back in BigQuery because the transformed row exceeds
    /// BigQuery's max row size.
    BigqueryMaxRowSizeExceeded = 2,
    /// This will be set when there is a finding in the custom metadata of a file,
    /// but at the write time of the transformed file, this key / value pair is
    /// unretrievable.
    MetadataUnretrievable = 3,
    /// This will be set when the transformation and storing of it is successful.
    Success = 4,
}
/// Describes functionality of a given container in its original format.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransformationContainerType {
    TransformUnknownContainer = 0,
    TransformBody = 1,
    TransformMetadata = 2,
    TransformTable = 3,
}
/// An enum of rules that can be used to transform a value. Can be a
/// record suppression, or one of the transformation rules specified under
/// `PrimitiveTransformation`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransformationType {
    /// Unused
    Unspecified = 0,
    /// Record suppression
    RecordSuppression = 1,
    /// Replace value
    ReplaceValue = 2,
    /// Replace value using a dictionary.
    ReplaceDictionary = 15,
    /// Redact
    Redact = 3,
    /// Character mask
    CharacterMask = 4,
    /// FFX-FPE
    CryptoReplaceFfxFpe = 5,
    /// Fixed size bucketing
    FixedSizeBucketing = 6,
    /// Bucketing
    Bucketing = 7,
    /// Replace with info type
    ReplaceWithInfoType = 8,
    /// Time part
    TimePart = 9,
    /// Crypto hash
    CryptoHash = 10,
    /// Date shift
    DateShift = 12,
    /// Deterministic crypto
    CryptoDeterministicConfig = 13,
    /// Redact image
    RedactImage = 14,
}
/// Operators available for comparing the value of fields.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RelationalOperator {
    /// Unused
    Unspecified = 0,
    /// Equal. Attempts to match even with incompatible types.
    EqualTo = 1,
    /// Not equal to. Attempts to match even with incompatible types.
    NotEqualTo = 2,
    /// Greater than.
    GreaterThan = 3,
    /// Less than.
    LessThan = 4,
    /// Greater than or equals.
    GreaterThanOrEquals = 5,
    /// Less than or equals.
    LessThanOrEquals = 6,
    /// Exists
    Exists = 7,
}
/// Type of the match which can be applied to different ways of matching, like
/// Dictionary, regular expression and intersecting with findings of another
/// info type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MatchingType {
    /// Invalid.
    Unspecified = 0,
    /// Full match.
    ///
    /// - Dictionary: join of Dictionary results matched complete finding quote
    /// - Regex: all regex matches fill a finding quote start to end
    /// - Exclude info type: completely inside affecting info types findings
    FullMatch = 1,
    /// Partial match.
    ///
    /// - Dictionary: at least one of the tokens in the finding matches
    /// - Regex: substring of the finding matches
    /// - Exclude info type: intersects with affecting info types findings
    PartialMatch = 2,
    /// Inverse match.
    ///
    /// - Dictionary: no tokens in the finding match the dictionary
    /// - Regex: finding doesn't match the regex
    /// - Exclude info type: no intersection with affecting info types findings
    InverseMatch = 3,
}
/// Deprecated and unused.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContentOption {
    /// Includes entire content of a file or a data stream.
    ContentUnspecified = 0,
    /// Text content within the data, excluding any metadata.
    ContentText = 1,
    /// Images found in the data.
    ContentImage = 2,
}
/// Type of metadata containing the finding.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MetadataType {
    /// Unused
    MetadatatypeUnspecified = 0,
    /// General file metadata provided by Cloud Storage.
    StorageMetadata = 2,
}
/// Parts of the APIs which use certain infoTypes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InfoTypeSupportedBy {
    /// Unused.
    EnumTypeUnspecified = 0,
    /// Supported by the inspect operations.
    Inspect = 1,
    /// Supported by the risk analysis operations.
    RiskAnalysis = 2,
}
/// An enum to represent the various types of DLP jobs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DlpJobType {
    /// Defaults to INSPECT_JOB.
    Unspecified = 0,
    /// The job inspected Google Cloud for sensitive data.
    InspectJob = 1,
    /// The job executed a Risk Analysis computation.
    RiskAnalysisJob = 2,
}
/// State of a StoredInfoType version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StoredInfoTypeState {
    /// Unused
    Unspecified = 0,
    /// StoredInfoType version is being created.
    Pending = 1,
    /// StoredInfoType version is ready for use.
    Ready = 2,
    /// StoredInfoType creation failed. All relevant error messages are returned in
    /// the `StoredInfoTypeVersion` message.
    Failed = 3,
    /// StoredInfoType is no longer valid because artifacts stored in
    /// user-controlled storage were modified. To fix an invalid StoredInfoType,
    /// use the `UpdateStoredInfoType` method to create a new version.
    Invalid = 4,
}
/// How broadly a resource has been shared. New items may be added over time.
/// A higher number means more restricted.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResourceVisibility {
    /// Unused.
    Unspecified = 0,
    /// Visible to any user.
    Public = 10,
    /// Visible only to specific users.
    Restricted = 20,
}
/// How a resource is encrypted.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EncryptionStatus {
    /// Unused.
    Unspecified = 0,
    /// Google manages server-side encryption keys on your behalf.
    EncryptionGoogleManaged = 1,
    /// Customer provides the key.
    EncryptionCustomerManaged = 2,
}
/// Generated client implementations.
pub mod dlp_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// The Cloud Data Loss Prevention (DLP) API is a service that allows clients
    /// to detect the presence of Personally Identifiable Information (PII) and other
    /// privacy-sensitive data in user-supplied, unstructured data streams, like text
    /// blocks or images.
    /// The service also includes methods for sensitive data redaction and
    /// scheduling of data scans on Google Cloud Platform based data sets.
    ///
    /// To learn more about concepts and find how-to guides see
    /// https://cloud.google.com/dlp/docs/.
    #[derive(Debug, Clone)]
    pub struct DlpServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DlpServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DlpServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
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
            DlpServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Finds potentially sensitive info in content.
        /// This method has limits on input size, processing time, and output size.
        ///
        /// When no InfoTypes or CustomInfoTypes are specified in this request, the
        /// system will automatically choose what detectors to run. By default this may
        /// be all types, but may change over time as detectors are updated.
        ///
        /// For how to guides, see https://cloud.google.com/dlp/docs/inspecting-images
        /// and https://cloud.google.com/dlp/docs/inspecting-text,
        pub async fn inspect_content(
            &mut self,
            request: impl tonic::IntoRequest<super::InspectContentRequest>,
        ) -> Result<tonic::Response<super::InspectContentResponse>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/InspectContent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Redacts potentially sensitive info from an image.
        /// This method has limits on input size, processing time, and output size.
        /// See https://cloud.google.com/dlp/docs/redacting-sensitive-data-images to
        /// learn more.
        ///
        /// When no InfoTypes or CustomInfoTypes are specified in this request, the
        /// system will automatically choose what detectors to run. By default this may
        /// be all types, but may change over time as detectors are updated.
        pub async fn redact_image(
            &mut self,
            request: impl tonic::IntoRequest<super::RedactImageRequest>,
        ) -> Result<tonic::Response<super::RedactImageResponse>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/RedactImage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// De-identifies potentially sensitive info from a ContentItem.
        /// This method has limits on input size and output size.
        /// See https://cloud.google.com/dlp/docs/deidentify-sensitive-data to
        /// learn more.
        ///
        /// When no InfoTypes or CustomInfoTypes are specified in this request, the
        /// system will automatically choose what detectors to run. By default this may
        /// be all types, but may change over time as detectors are updated.
        pub async fn deidentify_content(
            &mut self,
            request: impl tonic::IntoRequest<super::DeidentifyContentRequest>,
        ) -> Result<tonic::Response<super::DeidentifyContentResponse>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/DeidentifyContent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Re-identifies content that has been de-identified.
        /// See
        /// https://cloud.google.com/dlp/docs/pseudonymization#re-identification_in_free_text_code_example
        /// to learn more.
        pub async fn reidentify_content(
            &mut self,
            request: impl tonic::IntoRequest<super::ReidentifyContentRequest>,
        ) -> Result<tonic::Response<super::ReidentifyContentResponse>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/ReidentifyContent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns a list of the sensitive information types that DLP API
        /// supports. See https://cloud.google.com/dlp/docs/infotypes-reference to
        /// learn more.
        pub async fn list_info_types(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInfoTypesRequest>,
        ) -> Result<tonic::Response<super::ListInfoTypesResponse>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/ListInfoTypes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates an InspectTemplate for reusing frequently used configuration
        /// for inspecting content, images, and storage.
        /// See https://cloud.google.com/dlp/docs/creating-templates to learn more.
        pub async fn create_inspect_template(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInspectTemplateRequest>,
        ) -> Result<tonic::Response<super::InspectTemplate>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/CreateInspectTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the InspectTemplate.
        /// See https://cloud.google.com/dlp/docs/creating-templates to learn more.
        pub async fn update_inspect_template(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateInspectTemplateRequest>,
        ) -> Result<tonic::Response<super::InspectTemplate>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/UpdateInspectTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets an InspectTemplate.
        /// See https://cloud.google.com/dlp/docs/creating-templates to learn more.
        pub async fn get_inspect_template(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInspectTemplateRequest>,
        ) -> Result<tonic::Response<super::InspectTemplate>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/GetInspectTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists InspectTemplates.
        /// See https://cloud.google.com/dlp/docs/creating-templates to learn more.
        pub async fn list_inspect_templates(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInspectTemplatesRequest>,
        ) -> Result<
            tonic::Response<super::ListInspectTemplatesResponse>,
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
                "/google.privacy.dlp.v2.DlpService/ListInspectTemplates",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes an InspectTemplate.
        /// See https://cloud.google.com/dlp/docs/creating-templates to learn more.
        pub async fn delete_inspect_template(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteInspectTemplateRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/DeleteInspectTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a DeidentifyTemplate for reusing frequently used configuration
        /// for de-identifying content, images, and storage.
        /// See https://cloud.google.com/dlp/docs/creating-templates-deid to learn
        /// more.
        pub async fn create_deidentify_template(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDeidentifyTemplateRequest>,
        ) -> Result<tonic::Response<super::DeidentifyTemplate>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/CreateDeidentifyTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the DeidentifyTemplate.
        /// See https://cloud.google.com/dlp/docs/creating-templates-deid to learn
        /// more.
        pub async fn update_deidentify_template(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDeidentifyTemplateRequest>,
        ) -> Result<tonic::Response<super::DeidentifyTemplate>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/UpdateDeidentifyTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a DeidentifyTemplate.
        /// See https://cloud.google.com/dlp/docs/creating-templates-deid to learn
        /// more.
        pub async fn get_deidentify_template(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDeidentifyTemplateRequest>,
        ) -> Result<tonic::Response<super::DeidentifyTemplate>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/GetDeidentifyTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists DeidentifyTemplates.
        /// See https://cloud.google.com/dlp/docs/creating-templates-deid to learn
        /// more.
        pub async fn list_deidentify_templates(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDeidentifyTemplatesRequest>,
        ) -> Result<
            tonic::Response<super::ListDeidentifyTemplatesResponse>,
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
                "/google.privacy.dlp.v2.DlpService/ListDeidentifyTemplates",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a DeidentifyTemplate.
        /// See https://cloud.google.com/dlp/docs/creating-templates-deid to learn
        /// more.
        pub async fn delete_deidentify_template(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDeidentifyTemplateRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/DeleteDeidentifyTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a job trigger to run DLP actions such as scanning storage for
        /// sensitive information on a set schedule.
        /// See https://cloud.google.com/dlp/docs/creating-job-triggers to learn more.
        pub async fn create_job_trigger(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateJobTriggerRequest>,
        ) -> Result<tonic::Response<super::JobTrigger>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/CreateJobTrigger",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a job trigger.
        /// See https://cloud.google.com/dlp/docs/creating-job-triggers to learn more.
        pub async fn update_job_trigger(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateJobTriggerRequest>,
        ) -> Result<tonic::Response<super::JobTrigger>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/UpdateJobTrigger",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Inspect hybrid content and store findings to a trigger. The inspection
        /// will be processed asynchronously. To review the findings monitor the
        /// jobs within the trigger.
        pub async fn hybrid_inspect_job_trigger(
            &mut self,
            request: impl tonic::IntoRequest<super::HybridInspectJobTriggerRequest>,
        ) -> Result<tonic::Response<super::HybridInspectResponse>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/HybridInspectJobTrigger",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a job trigger.
        /// See https://cloud.google.com/dlp/docs/creating-job-triggers to learn more.
        pub async fn get_job_trigger(
            &mut self,
            request: impl tonic::IntoRequest<super::GetJobTriggerRequest>,
        ) -> Result<tonic::Response<super::JobTrigger>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/GetJobTrigger",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists job triggers.
        /// See https://cloud.google.com/dlp/docs/creating-job-triggers to learn more.
        pub async fn list_job_triggers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListJobTriggersRequest>,
        ) -> Result<tonic::Response<super::ListJobTriggersResponse>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/ListJobTriggers",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a job trigger.
        /// See https://cloud.google.com/dlp/docs/creating-job-triggers to learn more.
        pub async fn delete_job_trigger(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteJobTriggerRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/DeleteJobTrigger",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Activate a job trigger. Causes the immediate execute of a trigger
        /// instead of waiting on the trigger event to occur.
        pub async fn activate_job_trigger(
            &mut self,
            request: impl tonic::IntoRequest<super::ActivateJobTriggerRequest>,
        ) -> Result<tonic::Response<super::DlpJob>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/ActivateJobTrigger",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new job to inspect storage or calculate risk metrics.
        /// See https://cloud.google.com/dlp/docs/inspecting-storage and
        /// https://cloud.google.com/dlp/docs/compute-risk-analysis to learn more.
        ///
        /// When no InfoTypes or CustomInfoTypes are specified in inspect jobs, the
        /// system will automatically choose what detectors to run. By default this may
        /// be all types, but may change over time as detectors are updated.
        pub async fn create_dlp_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDlpJobRequest>,
        ) -> Result<tonic::Response<super::DlpJob>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/CreateDlpJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists DlpJobs that match the specified filter in the request.
        /// See https://cloud.google.com/dlp/docs/inspecting-storage and
        /// https://cloud.google.com/dlp/docs/compute-risk-analysis to learn more.
        pub async fn list_dlp_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDlpJobsRequest>,
        ) -> Result<tonic::Response<super::ListDlpJobsResponse>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/ListDlpJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the latest state of a long-running DlpJob.
        /// See https://cloud.google.com/dlp/docs/inspecting-storage and
        /// https://cloud.google.com/dlp/docs/compute-risk-analysis to learn more.
        pub async fn get_dlp_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDlpJobRequest>,
        ) -> Result<tonic::Response<super::DlpJob>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/GetDlpJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a long-running DlpJob. This method indicates that the client is
        /// no longer interested in the DlpJob result. The job will be canceled if
        /// possible.
        /// See https://cloud.google.com/dlp/docs/inspecting-storage and
        /// https://cloud.google.com/dlp/docs/compute-risk-analysis to learn more.
        pub async fn delete_dlp_job(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDlpJobRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/DeleteDlpJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Starts asynchronous cancellation on a long-running DlpJob. The server
        /// makes a best effort to cancel the DlpJob, but success is not
        /// guaranteed.
        /// See https://cloud.google.com/dlp/docs/inspecting-storage and
        /// https://cloud.google.com/dlp/docs/compute-risk-analysis to learn more.
        pub async fn cancel_dlp_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelDlpJobRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/CancelDlpJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a pre-built stored infoType to be used for inspection.
        /// See https://cloud.google.com/dlp/docs/creating-stored-infotypes to
        /// learn more.
        pub async fn create_stored_info_type(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateStoredInfoTypeRequest>,
        ) -> Result<tonic::Response<super::StoredInfoType>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/CreateStoredInfoType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the stored infoType by creating a new version. The existing version
        /// will continue to be used until the new version is ready.
        /// See https://cloud.google.com/dlp/docs/creating-stored-infotypes to
        /// learn more.
        pub async fn update_stored_info_type(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateStoredInfoTypeRequest>,
        ) -> Result<tonic::Response<super::StoredInfoType>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/UpdateStoredInfoType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a stored infoType.
        /// See https://cloud.google.com/dlp/docs/creating-stored-infotypes to
        /// learn more.
        pub async fn get_stored_info_type(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStoredInfoTypeRequest>,
        ) -> Result<tonic::Response<super::StoredInfoType>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/GetStoredInfoType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists stored infoTypes.
        /// See https://cloud.google.com/dlp/docs/creating-stored-infotypes to
        /// learn more.
        pub async fn list_stored_info_types(
            &mut self,
            request: impl tonic::IntoRequest<super::ListStoredInfoTypesRequest>,
        ) -> Result<tonic::Response<super::ListStoredInfoTypesResponse>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/ListStoredInfoTypes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a stored infoType.
        /// See https://cloud.google.com/dlp/docs/creating-stored-infotypes to
        /// learn more.
        pub async fn delete_stored_info_type(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteStoredInfoTypeRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/DeleteStoredInfoType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Inspect hybrid content and store findings to a job.
        /// To review the findings, inspect the job. Inspection will occur
        /// asynchronously.
        pub async fn hybrid_inspect_dlp_job(
            &mut self,
            request: impl tonic::IntoRequest<super::HybridInspectDlpJobRequest>,
        ) -> Result<tonic::Response<super::HybridInspectResponse>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/HybridInspectDlpJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Finish a running hybrid DlpJob. Triggers the finalization steps and running
        /// of any enabled actions that have not yet run.
        pub async fn finish_dlp_job(
            &mut self,
            request: impl tonic::IntoRequest<super::FinishDlpJobRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.privacy.dlp.v2.DlpService/FinishDlpJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
