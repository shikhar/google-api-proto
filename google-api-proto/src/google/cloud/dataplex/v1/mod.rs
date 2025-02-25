/// A lake is a centralized repository for managing enterprise data across the
/// organization distributed across many cloud projects, and stored in a variety
/// of storage services such as Google Cloud Storage and BigQuery. The resources
/// attached to a lake are referred to as managed resources. Data within these
/// managed resources can be structured or unstructured. A lake provides data
/// admins with tools to organize, secure and manage their data at scale, and
/// provides data scientists and data engineers an integrated experience to
/// easily search, discover, analyze and transform data and associated metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Lake {
    /// Output only. The relative resource name of the lake, of the form:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. User friendly display name.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. System generated globally unique ID for the lake. This ID will be
    /// different if the lake is deleted and re-created with the same name.
    #[prost(string, tag="3")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. The time when the lake was created.
    #[prost(message, optional, tag="4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the lake was last updated.
    #[prost(message, optional, tag="5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. User-defined labels for the lake.
    #[prost(btree_map="string, string", tag="6")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. Description of the lake.
    #[prost(string, tag="7")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Current state of the lake.
    #[prost(enumeration="State", tag="8")]
    pub state: i32,
    /// Output only. Service account associated with this lake. This service account must be
    /// authorized to access or operate on resources managed by the lake.
    #[prost(string, tag="9")]
    pub service_account: ::prost::alloc::string::String,
    /// Optional. Settings to manage lake and Dataproc Metastore service instance
    /// association.
    #[prost(message, optional, tag="102")]
    pub metastore: ::core::option::Option<lake::Metastore>,
    /// Output only. Aggregated status of the underlying assets of the lake.
    #[prost(message, optional, tag="103")]
    pub asset_status: ::core::option::Option<AssetStatus>,
    /// Output only. Metastore status of the lake.
    #[prost(message, optional, tag="104")]
    pub metastore_status: ::core::option::Option<lake::MetastoreStatus>,
}
/// Nested message and enum types in `Lake`.
pub mod lake {
    /// Settings to manage association of Dataproc Metastore with a lake.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Metastore {
        /// Optional. A relative reference to the Dataproc Metastore
        /// (<https://cloud.google.com/dataproc-metastore/docs>) service associated
        /// with the lake:
        /// `projects/{project_id}/locations/{location_id}/services/{service_id}`
        #[prost(string, tag="1")]
        pub service: ::prost::alloc::string::String,
    }
    /// Status of Lake and Dataproc Metastore service instance association.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MetastoreStatus {
        /// Current state of association.
        #[prost(enumeration="metastore_status::State", tag="1")]
        pub state: i32,
        /// Additional information about the current status.
        #[prost(string, tag="2")]
        pub message: ::prost::alloc::string::String,
        /// Last update time of the metastore status of the lake.
        #[prost(message, optional, tag="3")]
        pub update_time: ::core::option::Option<::prost_types::Timestamp>,
        /// The URI of the endpoint used to access the Metastore service.
        #[prost(string, tag="4")]
        pub endpoint: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `MetastoreStatus`.
    pub mod metastore_status {
        /// Current state of association.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum State {
            /// Unspecified.
            Unspecified = 0,
            /// A Metastore service instance is not associated with the lake.
            None = 1,
            /// A Metastore service instance is attached to the lake.
            Ready = 2,
            /// Attach/detach is in progress.
            Updating = 3,
            /// Attach/detach could not be done due to errors.
            Error = 4,
        }
    }
}
/// Aggregated status of the underlying assets of a lake or zone.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetStatus {
    /// Last update time of the status.
    #[prost(message, optional, tag="1")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Number of active assets.
    #[prost(int32, tag="2")]
    pub active_assets: i32,
    /// Number of assets that are in process of updating the security policy on
    /// attached resources.
    #[prost(int32, tag="3")]
    pub security_policy_applying_assets: i32,
}
/// A zone represents a logical group of related assets within a lake. A zone can
/// be used to map to organizational structure or represent stages of data
/// readiness from raw to curated. It provides managing behavior that is shared
/// or inherited by all contained assets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Zone {
    /// Output only. The relative resource name of the zone, of the form:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. User friendly display name.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. System generated globally unique ID for the zone. This ID will be
    /// different if the zone is deleted and re-created with the same name.
    #[prost(string, tag="3")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. The time when the zone was created.
    #[prost(message, optional, tag="4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the zone was last updated.
    #[prost(message, optional, tag="5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. User defined labels for the zone.
    #[prost(btree_map="string, string", tag="6")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. Description of the zone.
    #[prost(string, tag="7")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Current state of the zone.
    #[prost(enumeration="State", tag="8")]
    pub state: i32,
    /// Required. Immutable. The type of the zone.
    #[prost(enumeration="zone::Type", tag="9")]
    pub r#type: i32,
    /// Optional. Specification of the discovery feature applied to data in this zone.
    #[prost(message, optional, tag="103")]
    pub discovery_spec: ::core::option::Option<zone::DiscoverySpec>,
    /// Required. Specification of the resources that are referenced by the assets within
    /// this zone.
    #[prost(message, optional, tag="104")]
    pub resource_spec: ::core::option::Option<zone::ResourceSpec>,
    /// Output only. Aggregated status of the underlying assets of the zone.
    #[prost(message, optional, tag="105")]
    pub asset_status: ::core::option::Option<AssetStatus>,
}
/// Nested message and enum types in `Zone`.
pub mod zone {
    /// Settings for resources attached as assets within a zone.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResourceSpec {
        /// Required. Immutable. The location type of the resources that are allowed to be attached to the
        /// assets within this zone.
        #[prost(enumeration="resource_spec::LocationType", tag="1")]
        pub location_type: i32,
    }
    /// Nested message and enum types in `ResourceSpec`.
    pub mod resource_spec {
        /// Location type of the resources attached to a zone.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum LocationType {
            /// Unspecified location type.
            Unspecified = 0,
            /// Resources that are associated with a single region.
            SingleRegion = 1,
            /// Resources that are associated with a multi-region location.
            MultiRegion = 2,
        }
    }
    /// Settings to manage the metadata discovery and publishing in a zone.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DiscoverySpec {
        /// Required. Whether discovery is enabled.
        #[prost(bool, tag="1")]
        pub enabled: bool,
        /// Optional. The list of patterns to apply for selecting data to include during
        /// discovery if only a subset of the data should considered. For Cloud
        /// Storage bucket assets, these are interpreted as glob patterns used to
        /// match object names. For BigQuery dataset assets, these are
        /// interpreted as patterns to match table names.
        #[prost(string, repeated, tag="2")]
        pub include_patterns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Optional. The list of patterns to apply for selecting data to exclude during
        /// discovery.  For Cloud Storage bucket assets, these are interpreted as
        /// glob patterns used to match object names. For BigQuery dataset assets,
        /// these are interpreted as patterns to match table names.
        #[prost(string, repeated, tag="3")]
        pub exclude_patterns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Optional. Configuration for CSV data.
        #[prost(message, optional, tag="4")]
        pub csv_options: ::core::option::Option<discovery_spec::CsvOptions>,
        /// Optional. Configuration for Json data.
        #[prost(message, optional, tag="5")]
        pub json_options: ::core::option::Option<discovery_spec::JsonOptions>,
        /// Determines when discovery is triggered.
        #[prost(oneof="discovery_spec::Trigger", tags="10")]
        pub trigger: ::core::option::Option<discovery_spec::Trigger>,
    }
    /// Nested message and enum types in `DiscoverySpec`.
    pub mod discovery_spec {
        /// Describe CSV and similar semi-structured data formats.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CsvOptions {
            /// Optional. The number of rows to interpret as header rows that should be skipped
            /// when reading data rows.
            #[prost(int32, tag="1")]
            pub header_rows: i32,
            /// Optional. The delimiter being used to separate values. This defaults to ','.
            #[prost(string, tag="2")]
            pub delimiter: ::prost::alloc::string::String,
            /// Optional. The character encoding of the data. The default is UTF-8.
            #[prost(string, tag="3")]
            pub encoding: ::prost::alloc::string::String,
            /// Optional. Whether to disable the inference of data type for CSV data.
            /// If true, all columns will be registered as strings.
            #[prost(bool, tag="4")]
            pub disable_type_inference: bool,
        }
        /// Describe JSON data format.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct JsonOptions {
            /// Optional. The character encoding of the data. The default is UTF-8.
            #[prost(string, tag="1")]
            pub encoding: ::prost::alloc::string::String,
            /// Optional. Whether to disable the inference of data type for Json data.
            /// If true, all columns will be registered as their primitive types
            /// (strings, number or boolean).
            #[prost(bool, tag="2")]
            pub disable_type_inference: bool,
        }
        /// Determines when discovery is triggered.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Trigger {
            /// Optional. Cron schedule (<https://en.wikipedia.org/wiki/Cron>) for running
            /// discovery periodically. Successive discovery runs must be scheduled at
            /// least 60 minutes apart.
            /// The default value is to run discovery every 60 minutes.
            /// To explicitly set a timezone to the cron tab, apply a prefix in the
            /// cron tab: "CRON_TZ=${IANA_TIME_ZONE}" or TZ=${IANA_TIME_ZONE}".
            /// The ${IANA_TIME_ZONE} may only be a valid string from IANA time zone
            /// database. For example, "CRON_TZ=America/New_York 1 * * * *", or
            /// "TZ=America/New_York 1 * * * *".
            #[prost(string, tag="10")]
            Schedule(::prost::alloc::string::String),
        }
    }
    /// Type of zone.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Zone type not specified.
        Unspecified = 0,
        /// A zone that contains data that needs further processing before it is
        /// considered generally ready for consumption and analytics workloads.
        Raw = 1,
        /// A zone that contains data that is considered to be ready for broader
        /// consumption and analytics workloads. Curated structured data stored in
        /// Cloud Storage must conform to certain file formats (parquet, avro and
        /// orc) and organized in a hive-compatible directory layout.
        Curated = 2,
    }
}
/// Action represents an issue requiring administrator action for resolution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Action {
    /// The category of issue associated with the action.
    #[prost(enumeration="action::Category", tag="1")]
    pub category: i32,
    /// Detailed description of the issue requiring action.
    #[prost(string, tag="2")]
    pub issue: ::prost::alloc::string::String,
    /// The time that the issue was detected.
    #[prost(message, optional, tag="4")]
    pub detect_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The relative resource name of the action, of the form:
    /// `projects/{project}/locations/{location}/lakes/{lake}/actions/{action}`
    /// `projects/{project}/locations/{location}/lakes/{lake}/zones/{zone}/actions/{action}`
    /// `projects/{project}/locations/{location}/lakes/{lake}/zones/{zone}/assets/{asset}/actions/{action}`.
    #[prost(string, tag="5")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The relative resource name of the lake, of the form:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}`.
    #[prost(string, tag="6")]
    pub lake: ::prost::alloc::string::String,
    /// Output only. The relative resource name of the zone, of the form:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}`.
    #[prost(string, tag="7")]
    pub zone: ::prost::alloc::string::String,
    /// Output only. The relative resource name of the asset, of the form:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/assets/{asset_id}`.
    #[prost(string, tag="8")]
    pub asset: ::prost::alloc::string::String,
    /// The list of data locations associated with this action. Cloud Storage
    /// locations are represented as URI paths(E.g.
    /// `gs://bucket/table1/year=2020/month=Jan/`). BigQuery locations refer to
    /// resource names(E.g.
    /// `bigquery.googleapis.com/projects/project-id/datasets/dataset-id`).
    #[prost(string, repeated, tag="9")]
    pub data_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Additional details about the action based on the action category.
    #[prost(oneof="action::Details", tags="10, 11, 12, 13, 14, 15, 21, 22")]
    pub details: ::core::option::Option<action::Details>,
}
/// Nested message and enum types in `Action`.
pub mod action {
    /// Action details for resource references in assets that cannot be located.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MissingResource {
    }
    /// Action details for unauthorized resource issues raised to indicate that the
    /// service account associated with the lake instance is not authorized to
    /// access or manage the resource associated with an asset.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UnauthorizedResource {
    }
    /// Failed to apply security policy to the managed resource(s) under a
    /// lake, zone or an asset. For a lake or zone resource, one or more underlying
    /// assets has a failure applying security policy to the associated managed
    /// resource.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FailedSecurityPolicyApply {
        /// Resource name of one of the assets with failing security policy
        /// application. Populated for a lake or zone resource only.
        #[prost(string, tag="1")]
        pub asset: ::prost::alloc::string::String,
    }
    /// Action details for invalid or unsupported data files detected by discovery.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InvalidDataFormat {
        /// The list of data locations sampled and used for format/schema
        /// inference.
        #[prost(string, repeated, tag="1")]
        pub sampled_data_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The expected data format of the entity.
        #[prost(string, tag="2")]
        pub expected_format: ::prost::alloc::string::String,
        /// The new unexpected data format within the entity.
        #[prost(string, tag="3")]
        pub new_format: ::prost::alloc::string::String,
    }
    /// Action details for incompatible schemas detected by discovery.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IncompatibleDataSchema {
        /// The name of the table containing invalid data.
        #[prost(string, tag="1")]
        pub table: ::prost::alloc::string::String,
        /// The existing and expected schema of the table. The schema is provided as
        /// a JSON formatted structure listing columns and data types.
        #[prost(string, tag="2")]
        pub existing_schema: ::prost::alloc::string::String,
        /// The new and incompatible schema within the table. The schema is provided
        /// as a JSON formatted structured listing columns and data types.
        #[prost(string, tag="3")]
        pub new_schema: ::prost::alloc::string::String,
        /// The list of data locations sampled and used for format/schema
        /// inference.
        #[prost(string, repeated, tag="4")]
        pub sampled_data_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Whether the action relates to a schema that is incompatible or modified.
        #[prost(enumeration="incompatible_data_schema::SchemaChange", tag="5")]
        pub schema_change: i32,
    }
    /// Nested message and enum types in `IncompatibleDataSchema`.
    pub mod incompatible_data_schema {
        /// Whether the action relates to a schema that is incompatible or modified.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum SchemaChange {
            /// Schema change unspecified.
            Unspecified = 0,
            /// Newly discovered schema is incompatible with existing schema.
            Incompatible = 1,
            /// Newly discovered schema has changed from existing schema for data in a
            /// curated zone.
            Modified = 2,
        }
    }
    /// Action details for invalid or unsupported partitions detected by discovery.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InvalidDataPartition {
        /// The issue type of InvalidDataPartition.
        #[prost(enumeration="invalid_data_partition::PartitionStructure", tag="1")]
        pub expected_structure: i32,
    }
    /// Nested message and enum types in `InvalidDataPartition`.
    pub mod invalid_data_partition {
        /// The expected partition structure.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum PartitionStructure {
            /// PartitionStructure unspecified.
            Unspecified = 0,
            /// Consistent hive-style partition definition (both raw and curated zone).
            ConsistentKeys = 1,
            /// Hive style partition definition (curated zone only).
            HiveStyleKeys = 2,
        }
    }
    /// Action details for absence of data detected by discovery.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MissingData {
    }
    /// Action details for invalid data arrangement.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InvalidDataOrganization {
    }
    /// The category of issues.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Category {
        /// Unspecified category.
        Unspecified = 0,
        /// Resource management related issues.
        ResourceManagement = 1,
        /// Security policy related issues.
        SecurityPolicy = 2,
        /// Data and discovery related issues.
        DataDiscovery = 3,
    }
    /// Additional details about the action based on the action category.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Details {
        /// Details for issues related to invalid or unsupported data formats.
        #[prost(message, tag="10")]
        InvalidDataFormat(InvalidDataFormat),
        /// Details for issues related to incompatible schemas detected within data.
        #[prost(message, tag="11")]
        IncompatibleDataSchema(IncompatibleDataSchema),
        /// Details for issues related to invalid or unsupported data partition
        /// structure.
        #[prost(message, tag="12")]
        InvalidDataPartition(InvalidDataPartition),
        /// Details for issues related to absence of data within managed resources.
        #[prost(message, tag="13")]
        MissingData(MissingData),
        /// Details for issues related to absence of a managed resource.
        #[prost(message, tag="14")]
        MissingResource(MissingResource),
        /// Details for issues related to lack of permissions to access data
        /// resources.
        #[prost(message, tag="15")]
        UnauthorizedResource(UnauthorizedResource),
        /// Details for issues related to applying security policy.
        #[prost(message, tag="21")]
        FailedSecurityPolicyApply(FailedSecurityPolicyApply),
        /// Details for issues related to invalid data arrangement.
        #[prost(message, tag="22")]
        InvalidDataOrganization(InvalidDataOrganization),
    }
}
/// An asset represents a cloud resource that is being managed within a lake as a
/// member of a zone.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    /// Output only. The relative resource name of the asset, of the form:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/assets/{asset_id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. User friendly display name.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. System generated globally unique ID for the asset. This ID will be
    /// different if the asset is deleted and re-created with the same name.
    #[prost(string, tag="3")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. The time when the asset was created.
    #[prost(message, optional, tag="4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the asset was last updated.
    #[prost(message, optional, tag="5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. User defined labels for the asset.
    #[prost(btree_map="string, string", tag="6")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. Description of the asset.
    #[prost(string, tag="7")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Current state of the asset.
    #[prost(enumeration="State", tag="8")]
    pub state: i32,
    /// Required. Specification of the resource that is referenced by this asset.
    #[prost(message, optional, tag="100")]
    pub resource_spec: ::core::option::Option<asset::ResourceSpec>,
    /// Output only. Status of the resource referenced by this asset.
    #[prost(message, optional, tag="101")]
    pub resource_status: ::core::option::Option<asset::ResourceStatus>,
    /// Output only. Status of the security policy applied to resource referenced by this asset.
    #[prost(message, optional, tag="103")]
    pub security_status: ::core::option::Option<asset::SecurityStatus>,
    /// Optional. Specification of the discovery feature applied to data referenced by this
    /// asset.
    /// When this spec is left unset, the asset will use the spec set on the parent
    /// zone.
    #[prost(message, optional, tag="106")]
    pub discovery_spec: ::core::option::Option<asset::DiscoverySpec>,
    /// Output only. Status of the discovery feature applied to data referenced by this asset.
    #[prost(message, optional, tag="107")]
    pub discovery_status: ::core::option::Option<asset::DiscoveryStatus>,
}
/// Nested message and enum types in `Asset`.
pub mod asset {
    /// Security policy status of the asset. Data security policy, i.e., readers,
    /// writers & owners, should be specified in the lake/zone/asset IAM policy.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SecurityStatus {
        /// The current state of the security policy applied to the attached
        /// resource.
        #[prost(enumeration="security_status::State", tag="1")]
        pub state: i32,
        /// Additional information about the current state.
        #[prost(string, tag="2")]
        pub message: ::prost::alloc::string::String,
        /// Last update time of the status.
        #[prost(message, optional, tag="3")]
        pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    }
    /// Nested message and enum types in `SecurityStatus`.
    pub mod security_status {
        /// The state of the security policy.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum State {
            /// State unspecified.
            Unspecified = 0,
            /// Security policy has been successfully applied to the attached resource.
            Ready = 1,
            /// Security policy is in the process of being applied to the attached
            /// resource.
            Applying = 2,
            /// Security policy could not be applied to the attached resource due to
            /// errors.
            Error = 3,
        }
    }
    /// Settings to manage the metadata discovery and publishing for an asset.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DiscoverySpec {
        /// Optional. Whether discovery is enabled.
        #[prost(bool, tag="1")]
        pub enabled: bool,
        /// Optional. The list of patterns to apply for selecting data to include during
        /// discovery if only a subset of the data should considered.  For Cloud
        /// Storage bucket assets, these are interpreted as glob patterns used to
        /// match object names. For BigQuery dataset assets, these are interpreted as
        /// patterns to match table names.
        #[prost(string, repeated, tag="2")]
        pub include_patterns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Optional. The list of patterns to apply for selecting data to exclude during
        /// discovery.  For Cloud Storage bucket assets, these are interpreted as
        /// glob patterns used to match object names. For BigQuery dataset assets,
        /// these are interpreted as patterns to match table names.
        #[prost(string, repeated, tag="3")]
        pub exclude_patterns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Optional. Configuration for CSV data.
        #[prost(message, optional, tag="4")]
        pub csv_options: ::core::option::Option<discovery_spec::CsvOptions>,
        /// Optional. Configuration for Json data.
        #[prost(message, optional, tag="5")]
        pub json_options: ::core::option::Option<discovery_spec::JsonOptions>,
        /// Determines when discovery is triggered.
        #[prost(oneof="discovery_spec::Trigger", tags="10")]
        pub trigger: ::core::option::Option<discovery_spec::Trigger>,
    }
    /// Nested message and enum types in `DiscoverySpec`.
    pub mod discovery_spec {
        /// Describe CSV and similar semi-structured data formats.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CsvOptions {
            /// Optional. The number of rows to interpret as header rows that should be skipped
            /// when reading data rows.
            #[prost(int32, tag="1")]
            pub header_rows: i32,
            /// Optional. The delimiter being used to separate values. This defaults to ','.
            #[prost(string, tag="2")]
            pub delimiter: ::prost::alloc::string::String,
            /// Optional. The character encoding of the data. The default is UTF-8.
            #[prost(string, tag="3")]
            pub encoding: ::prost::alloc::string::String,
            /// Optional. Whether to disable the inference of data type for CSV data.
            /// If true, all columns will be registered as strings.
            #[prost(bool, tag="4")]
            pub disable_type_inference: bool,
        }
        /// Describe JSON data format.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct JsonOptions {
            /// Optional. The character encoding of the data. The default is UTF-8.
            #[prost(string, tag="1")]
            pub encoding: ::prost::alloc::string::String,
            /// Optional. Whether to disable the inference of data type for Json data.
            /// If true, all columns will be registered as their primitive types
            /// (strings, number or boolean).
            #[prost(bool, tag="2")]
            pub disable_type_inference: bool,
        }
        /// Determines when discovery is triggered.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Trigger {
            /// Optional. Cron schedule (<https://en.wikipedia.org/wiki/Cron>) for running
            /// discovery periodically. Successive discovery runs must be scheduled at
            /// least 60 minutes apart.
            /// The default value is to run discovery every 60 minutes.
            /// To explicitly set a timezone to the cron tab, apply a prefix in the
            /// cron tab: "CRON_TZ=${IANA_TIME_ZONE}" or TZ=${IANA_TIME_ZONE}".
            /// The ${IANA_TIME_ZONE} may only be a valid string from IANA time zone
            /// database. For example, "CRON_TZ=America/New_York 1 * * * *", or
            /// "TZ=America/New_York 1 * * * *".
            #[prost(string, tag="10")]
            Schedule(::prost::alloc::string::String),
        }
    }
    /// Identifies the cloud resource that is referenced by this asset.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResourceSpec {
        /// Immutable. Relative name of the cloud resource that contains the data that is
        /// being managed within a lake. For example:
        ///   `projects/{project_number}/buckets/{bucket_id}`
        ///   `projects/{project_number}/datasets/{dataset_id}`
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        /// Required. Immutable. Type of resource.
        #[prost(enumeration="resource_spec::Type", tag="2")]
        pub r#type: i32,
    }
    /// Nested message and enum types in `ResourceSpec`.
    pub mod resource_spec {
        /// Type of resource.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Type {
            /// Type not specified.
            Unspecified = 0,
            /// Cloud Storage bucket.
            StorageBucket = 1,
            /// BigQuery dataset.
            BigqueryDataset = 2,
        }
    }
    /// Status of the resource referenced by an asset.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResourceStatus {
        /// The current state of the managed resource.
        #[prost(enumeration="resource_status::State", tag="1")]
        pub state: i32,
        /// Additional information about the current state.
        #[prost(string, tag="2")]
        pub message: ::prost::alloc::string::String,
        /// Last update time of the status.
        #[prost(message, optional, tag="3")]
        pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    }
    /// Nested message and enum types in `ResourceStatus`.
    pub mod resource_status {
        /// The state of a resource.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum State {
            /// State unspecified.
            Unspecified = 0,
            /// Resource does not have any errors.
            Ready = 1,
            /// Resource has errors.
            Error = 2,
        }
    }
    /// Status of discovery for an asset.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DiscoveryStatus {
        /// The current status of the discovery feature.
        #[prost(enumeration="discovery_status::State", tag="1")]
        pub state: i32,
        /// Additional information about the current state.
        #[prost(string, tag="2")]
        pub message: ::prost::alloc::string::String,
        /// Last update time of the status.
        #[prost(message, optional, tag="3")]
        pub update_time: ::core::option::Option<::prost_types::Timestamp>,
        /// The start time of the last discovery run.
        #[prost(message, optional, tag="4")]
        pub last_run_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Data Stats of the asset reported by discovery.
        #[prost(message, optional, tag="6")]
        pub stats: ::core::option::Option<discovery_status::Stats>,
        /// The duration of the last discovery run.
        #[prost(message, optional, tag="7")]
        pub last_run_duration: ::core::option::Option<::prost_types::Duration>,
    }
    /// Nested message and enum types in `DiscoveryStatus`.
    pub mod discovery_status {
        /// The aggregated data statistics for the asset reported by discovery.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Stats {
            /// The count of data items within the referenced resource.
            #[prost(int64, tag="1")]
            pub data_items: i64,
            /// The number of stored data bytes within the referenced resource.
            #[prost(int64, tag="2")]
            pub data_size: i64,
            /// The count of table entities within the referenced resource.
            #[prost(int64, tag="3")]
            pub tables: i64,
            /// The count of fileset entities within the referenced resource.
            #[prost(int64, tag="4")]
            pub filesets: i64,
        }
        /// Current state of discovery.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum State {
            /// State is unspecified.
            Unspecified = 0,
            /// Discovery for the asset is scheduled.
            Scheduled = 1,
            /// Discovery for the asset is running.
            InProgress = 2,
            /// Discovery for the asset is currently paused (e.g. due to a lack
            /// of available resources). It will be automatically resumed.
            Paused = 3,
            /// Discovery for the asset is disabled.
            Disabled = 5,
        }
    }
}
/// State of a resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum State {
    /// State is not specified.
    Unspecified = 0,
    /// Resource is active, i.e., ready to use.
    Active = 1,
    /// Resource is under creation.
    Creating = 2,
    /// Resource is under deletion.
    Deleting = 3,
    /// Resource is active but has unresolved actions.
    ActionRequired = 4,
}
/// Create a metadata entity request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEntityRequest {
    /// Required. The resource name of the parent zone:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Entity resource.
    #[prost(message, optional, tag="3")]
    pub entity: ::core::option::Option<Entity>,
    /// Optional. Only validate the request, but do not perform mutations.
    /// The default is false.
    #[prost(bool, tag="4")]
    pub validate_only: bool,
}
/// Update a metadata entity request.
/// The exiting entity will be fully replaced by the entity in the request.
/// The entity ID is mutable. To modify the ID, use the current entity ID in the
/// request URL and specify the new ID in the request body.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEntityRequest {
    /// Required. Update description.
    #[prost(message, optional, tag="2")]
    pub entity: ::core::option::Option<Entity>,
    /// Optional. Only validate the request, but do not perform mutations.
    /// The default is false.
    #[prost(bool, tag="3")]
    pub validate_only: bool,
}
/// Delete a metadata entity request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEntityRequest {
    /// Required. The resource name of the entity:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/entities/{entity_id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The etag associated with the entity, which can be retrieved with a
    /// \[GetEntity][\] request.
    #[prost(string, tag="2")]
    pub etag: ::prost::alloc::string::String,
}
/// List metadata entities request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntitiesRequest {
    /// Required. The resource name of the parent zone:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Specify the entity view to make a partial list request.
    #[prost(enumeration="list_entities_request::EntityView", tag="2")]
    pub view: i32,
    /// Optional. Maximum number of entities to return. The service may return fewer than
    /// this value. If unspecified, 100 entities will be returned by default. The
    /// maximum value is 500; larger values will will be truncated to 500.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// Optional. Page token received from a previous `ListEntities` call. Provide
    /// this to retrieve the subsequent page. When paginating, all other parameters
    /// provided to `ListEntities` must match the call that provided the
    /// page token.
    #[prost(string, tag="4")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The following filter parameters can be added to the URL to limit the
    /// entities returned by the API:
    ///
    /// - Entity ID: ?filter="id=entityID"
    /// - Asset ID: ?filter="asset=assetID"
    /// - Data path ?filter="data_path=gs://my-bucket"
    /// - Is HIVE compatible: ?filter="hive_compatible=true"
    /// - Is BigQuery compatible: ?filter="bigquery_compatible=true"
    #[prost(string, tag="5")]
    pub filter: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ListEntitiesRequest`.
pub mod list_entities_request {
    /// Entity views.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EntityView {
        /// The default unset value. Return both table and fileset entities
        /// if unspecified.
        Unspecified = 0,
        /// Only list table entities.
        Tables = 1,
        /// Only list fileset entities.
        Filesets = 2,
    }
}
/// List metadata entities response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntitiesResponse {
    /// Entities in the specified parent zone.
    #[prost(message, repeated, tag="1")]
    pub entities: ::prost::alloc::vec::Vec<Entity>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// remaining results in the list.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Get metadata entity request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntityRequest {
    /// Required. The resource name of the entity:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/entities/{entity_id}.`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Used to select the subset of entity information to return.
    /// Defaults to `BASIC`.
    #[prost(enumeration="get_entity_request::EntityView", tag="2")]
    pub view: i32,
}
/// Nested message and enum types in `GetEntityRequest`.
pub mod get_entity_request {
    /// Entity views for get entity partial result.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EntityView {
        /// The API will default to the `BASIC` view.
        Unspecified = 0,
        /// Minimal view that does not include the schema.
        Basic = 1,
        /// Include basic information and schema.
        Schema = 2,
        /// Include everything. Currently, this is the same as the SCHEMA view.
        Full = 4,
    }
}
/// List metadata partitions request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPartitionsRequest {
    /// Required. The resource name of the parent entity:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/entities/{entity_id}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of partitions to return. The service may return fewer than
    /// this value. If unspecified, 100 partitions will be returned by default. The
    /// maximum page size is 500; larger values will will be truncated to 500.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. Page token received from a previous `ListPartitions` call. Provide
    /// this to retrieve the subsequent page. When paginating, all other parameters
    /// provided to `ListPartitions` must match the call that provided the
    /// page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filter the partitions returned to the caller using a key value pair
    /// expression. Supported operators and syntax:
    ///
    /// - logic operators: AND, OR
    /// - comparison operators: <, >, >=, <= ,=, !=
    /// - LIKE operators:
    ///   - The right hand of a LIKE operator supports "." and
    ///     "*" for wildcard searches, for example "value1 LIKE ".*oo.*"
    /// - parenthetical grouping: ( )
    ///
    /// Sample filter expression: `?filter="key1 < value1 OR key2 > value2"
    ///
    /// **Notes:**
    ///
    /// - Keys to the left of operators are case insensitive.
    /// - Partition results are sorted first by creation time, then by
    ///   lexicographic order.
    /// - Up to 20 key value filter pairs are allowed, but due to performance
    ///   considerations, only the first 10 will be used as a filter.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
}
/// Create metadata partition request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePartitionRequest {
    /// Required. The resource name of the parent zone:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/entities/{entity_id}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Partition resource.
    #[prost(message, optional, tag="3")]
    pub partition: ::core::option::Option<Partition>,
    /// Optional. Only validate the request, but do not perform mutations.
    /// The default is false.
    #[prost(bool, tag="4")]
    pub validate_only: bool,
}
/// Delete metadata partition request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePartitionRequest {
    /// Required. The resource name of the partition.
    /// format:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/entities/{entity_id}/partitions/{partition_value_path}`.
    /// The {partition_value_path} segment consists of an ordered sequence of
    /// partition values separated by "/". All values must be provided.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The etag associated with the partition.
    #[deprecated]
    #[prost(string, tag="2")]
    pub etag: ::prost::alloc::string::String,
}
/// List metadata partitions response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPartitionsResponse {
    /// Partitions under the specified parent entity.
    #[prost(message, repeated, tag="1")]
    pub partitions: ::prost::alloc::vec::Vec<Partition>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// remaining results in the list.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Get metadata partition request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPartitionRequest {
    /// Required. The resource name of the partition:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/entities/{entity_id}/partitions/{partition_value_path}`.
    /// The {partition_value_path} segment consists of an ordered sequence of
    /// partition values separated by "/". All values must be provided.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Represents tables and fileset metadata contained within a zone.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entity {
    /// Output only. The resource name of the entity, of the form:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/entities/{id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Display name must be shorter than or equal to 256 characters.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. User friendly longer description text. Must be shorter than or equal to
    /// 1024 characters.
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The time when the entity was created.
    #[prost(message, optional, tag="5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the entity was last updated.
    #[prost(message, optional, tag="6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. A user-provided entity ID. It is mutable, and will be used as the
    /// published table name. Specifying a new ID in an update entity
    /// request will override the existing value.
    /// The ID must contain only letters (a-z, A-Z), numbers (0-9), and
    /// underscores. Must begin with a letter and consist of 256 or fewer
    /// characters.
    #[prost(string, tag="7")]
    pub id: ::prost::alloc::string::String,
    /// Optional. The etag associated with the entity, which can be retrieved with a
    /// \[GetEntity][\] request. Required for update and delete requests.
    #[prost(string, tag="8")]
    pub etag: ::prost::alloc::string::String,
    /// Required. Immutable. The type of entity.
    #[prost(enumeration="entity::Type", tag="10")]
    pub r#type: i32,
    /// Required. Immutable. The ID of the asset associated with the storage location containing the
    /// entity data. The entity must be with in the same zone with the asset.
    #[prost(string, tag="11")]
    pub asset: ::prost::alloc::string::String,
    /// Required. Immutable. The storage path of the entity data.
    /// For Cloud Storage data, this is the fully-qualified path to the entity,
    /// such as `gs://bucket/path/to/data`. For BigQuery data, this is the name of
    /// the table resource, such as
    /// `projects/project_id/datasets/dataset_id/tables/table_id`.
    #[prost(string, tag="12")]
    pub data_path: ::prost::alloc::string::String,
    /// Optional. The set of items within the data path constituting the data in the entity,
    /// represented as a glob path.
    /// Example: `gs://bucket/path/to/data/**/*.csv`.
    #[prost(string, tag="13")]
    pub data_path_pattern: ::prost::alloc::string::String,
    /// Output only. The name of the associated Data Catalog entry.
    #[prost(string, tag="14")]
    pub catalog_entry: ::prost::alloc::string::String,
    /// Required. Immutable. Identifies the storage system of the entity data.
    #[prost(enumeration="StorageSystem", tag="15")]
    pub system: i32,
    /// Required. Identifies the storage format of the entity data.
    /// It does not apply to entities with data stored in BigQuery.
    #[prost(message, optional, tag="16")]
    pub format: ::core::option::Option<StorageFormat>,
    /// Output only. Metadata stores that the entity is compatible with.
    #[prost(message, optional, tag="19")]
    pub compatibility: ::core::option::Option<entity::CompatibilityStatus>,
    /// Required. The description of the data structure and layout.
    /// The schema is not included in list responses. It is only included in
    /// `SCHEMA` and `FULL` entity views of a `GetEntity` response.
    #[prost(message, optional, tag="50")]
    pub schema: ::core::option::Option<Schema>,
}
/// Nested message and enum types in `Entity`.
pub mod entity {
    /// Provides compatibility information for various metadata stores.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CompatibilityStatus {
        /// Output only. Whether this entity is compatible with Hive Metastore.
        #[prost(message, optional, tag="1")]
        pub hive_metastore: ::core::option::Option<compatibility_status::Compatibility>,
        /// Output only. Whether this entity is compatible with BigQuery.
        #[prost(message, optional, tag="2")]
        pub bigquery: ::core::option::Option<compatibility_status::Compatibility>,
    }
    /// Nested message and enum types in `CompatibilityStatus`.
    pub mod compatibility_status {
        /// Provides compatibility information for a specific metadata store.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Compatibility {
            /// Output only. Whether the entity is compatible and can be represented in the metadata
            /// store.
            #[prost(bool, tag="1")]
            pub compatible: bool,
            /// Output only. Provides additional detail if the entity is incompatible with the
            /// metadata store.
            #[prost(string, tag="2")]
            pub reason: ::prost::alloc::string::String,
        }
    }
    /// The type of entity.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Type unspecified.
        Unspecified = 0,
        /// Structured and semi-structured data.
        Table = 1,
        /// Unstructured data.
        Fileset = 2,
    }
}
/// Represents partition metadata contained within entity instances.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Partition {
    /// Output only. Partition values used in the HTTP URL must be
    /// double encoded. For example, `url_encode(url_encode(value))` can be used
    /// to encode "US:CA/CA#Sunnyvale so that the request URL ends
    /// with "/partitions/US%253ACA/CA%2523Sunnyvale".
    /// The name field in the response retains the encoded format.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Immutable. The set of values representing the partition, which correspond to the
    /// partition schema defined in the parent entity.
    #[prost(string, repeated, tag="2")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. Immutable. The location of the entity data within the partition, for example,
    /// `gs://bucket/path/to/entity/key1=value1/key2=value2`.
    /// Or `projects/<project_id>/datasets/<dataset_id>/tables/<table_id>`
    #[prost(string, tag="3")]
    pub location: ::prost::alloc::string::String,
    /// Optional. The etag for this partition.
    #[deprecated]
    #[prost(string, tag="4")]
    pub etag: ::prost::alloc::string::String,
}
/// Schema information describing the structure and layout of the data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schema {
    /// Required. Set to `true` if user-managed or `false` if managed by Dataplex. The
    /// default is `false` (managed by Dataplex).
    ///
    /// - Set to `false`to enable Dataplex discovery to update the schema.
    ///   including new data discovery, schema inference, and schema evolution.
    ///   Users retain the ability to input and edit the schema. Dataplex
    ///   treats schema input by the user as though produced
    ///   by a previous Dataplex discovery operation, and it will
    ///   evolve the schema and take action based on that treatment.
    ///
    /// - Set to `true` to fully manage the entity
    ///   schema. This setting guarantees that Dataplex will not
    ///   change schema fields.
    #[prost(bool, tag="1")]
    pub user_managed: bool,
    /// Optional. The sequence of fields describing data in table entities.
    /// **Note:** BigQuery SchemaFields are immutable.
    #[prost(message, repeated, tag="2")]
    pub fields: ::prost::alloc::vec::Vec<schema::SchemaField>,
    /// Optional. The sequence of fields describing the partition structure in entities.
    /// If this field is empty, there are no partitions within the data.
    #[prost(message, repeated, tag="3")]
    pub partition_fields: ::prost::alloc::vec::Vec<schema::PartitionField>,
    /// Optional. The structure of paths containing partition data within the entity.
    #[prost(enumeration="schema::PartitionStyle", tag="4")]
    pub partition_style: i32,
}
/// Nested message and enum types in `Schema`.
pub mod schema {
    /// Represents a column field within a table schema.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SchemaField {
        /// Required. The name of the field. Must contain only letters, numbers and
        /// underscores, with a maximum length of 767 characters,
        /// and must begin with a letter or underscore.
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        /// Optional. User friendly field description. Must be less than or equal to 1024
        /// characters.
        #[prost(string, tag="2")]
        pub description: ::prost::alloc::string::String,
        /// Required. The type of field.
        #[prost(enumeration="Type", tag="3")]
        pub r#type: i32,
        /// Required. Additional field semantics.
        #[prost(enumeration="Mode", tag="4")]
        pub mode: i32,
        /// Optional. Any nested field for complex types.
        #[prost(message, repeated, tag="10")]
        pub fields: ::prost::alloc::vec::Vec<SchemaField>,
    }
    /// Represents a key field within the entity's partition structure. You could
    /// have up to 20 partition fields, but only the first 10 partitions have the
    /// filtering ability due to performance consideration. **Note:**
    /// Partition fields are immutable.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PartitionField {
        /// Required. Partition field name must consist of letters, numbers, and underscores
        /// only, with a maximum of length of 256 characters,
        /// and must begin with a letter or underscore..
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        /// Required. Immutable. The type of field.
        #[prost(enumeration="Type", tag="2")]
        pub r#type: i32,
    }
    /// Type information for fields in schemas and partition schemas.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// SchemaType unspecified.
        Unspecified = 0,
        /// Boolean field.
        Boolean = 1,
        /// Single byte numeric field.
        Byte = 2,
        /// 16-bit numeric field.
        Int16 = 3,
        /// 32-bit numeric field.
        Int32 = 4,
        /// 64-bit numeric field.
        Int64 = 5,
        /// Floating point numeric field.
        Float = 6,
        /// Double precision numeric field.
        Double = 7,
        /// Real value numeric field.
        Decimal = 8,
        /// Sequence of characters field.
        String = 9,
        /// Sequence of bytes field.
        Binary = 10,
        /// Date and time field.
        Timestamp = 11,
        /// Date field.
        Date = 12,
        /// Time field.
        Time = 13,
        /// Structured field. Nested fields that define the structure of the map.
        /// If all nested fields are nullable, this field represents a union.
        Record = 14,
        /// Null field that does not have values.
        Null = 100,
    }
    /// Additional qualifiers to define field semantics.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Mode {
        /// Mode unspecified.
        Unspecified = 0,
        /// The field has required semantics.
        Required = 1,
        /// The field has optional semantics, and may be null.
        Nullable = 2,
        /// The field has repeated (0 or more) semantics, and is a list of values.
        Repeated = 3,
    }
    /// The structure of paths within the entity, which represent partitions.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PartitionStyle {
        /// PartitionStyle unspecified
        Unspecified = 0,
        /// Partitions are hive-compatible.
        /// Examples: `gs://bucket/path/to/table/dt=2019-10-31/lang=en`,
        /// `gs://bucket/path/to/table/dt=2019-10-31/lang=en/late`.
        HiveCompatible = 1,
    }
}
/// Describes the format of the data within its storage location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageFormat {
    /// Output only. The data format associated with the stored data, which represents
    /// content type values. The value is inferred from mime type.
    #[prost(enumeration="storage_format::Format", tag="1")]
    pub format: i32,
    /// Optional. The compression type associated with the stored data.
    /// If unspecified, the data is uncompressed.
    #[prost(enumeration="storage_format::CompressionFormat", tag="2")]
    pub compression_format: i32,
    /// Required. The mime type descriptor for the data. Must match the pattern
    /// {type}/{subtype}. Supported values:
    ///
    /// - application/x-parquet
    /// - application/x-avro
    /// - application/x-orc
    /// - application/x-tfrecord
    /// - application/json
    /// - application/{subtypes}
    /// - text/csv
    /// - text/<subtypes>
    /// - image/{image subtype}
    /// - video/{video subtype}
    /// - audio/{audio subtype}
    #[prost(string, tag="3")]
    pub mime_type: ::prost::alloc::string::String,
    /// Additional format-specific options.
    #[prost(oneof="storage_format::Options", tags="10, 11")]
    pub options: ::core::option::Option<storage_format::Options>,
}
/// Nested message and enum types in `StorageFormat`.
pub mod storage_format {
    /// Describes CSV and similar semi-structured data formats.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CsvOptions {
        /// Optional. The character encoding of the data. Accepts "US-ASCII", "UTF-8", and
        /// "ISO-8859-1". Defaults to UTF-8 if unspecified.
        #[prost(string, tag="1")]
        pub encoding: ::prost::alloc::string::String,
        /// Optional. The number of rows to interpret as header rows that should be skipped
        /// when reading data rows. Defaults to 0.
        #[prost(int32, tag="2")]
        pub header_rows: i32,
        /// Optional. The delimiter used to separate values. Defaults to ','.
        #[prost(string, tag="3")]
        pub delimiter: ::prost::alloc::string::String,
        /// Optional. The character used to quote column values. Accepts '"'
        /// (double quotation mark) or ''' (single quotation mark). Defaults to
        /// '"' (double quotation mark) if unspecified.
        #[prost(string, tag="4")]
        pub quote: ::prost::alloc::string::String,
    }
    /// Describes JSON data format.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct JsonOptions {
        /// Optional. The character encoding of the data. Accepts "US-ASCII", "UTF-8" and
        /// "ISO-8859-1". Defaults to UTF-8 if not specified.
        #[prost(string, tag="1")]
        pub encoding: ::prost::alloc::string::String,
    }
    /// The specific file format of the data.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Format {
        /// Format unspecified.
        Unspecified = 0,
        /// Parquet-formatted structured data.
        Parquet = 1,
        /// Avro-formatted structured data.
        Avro = 2,
        /// Orc-formatted structured data.
        Orc = 3,
        /// Csv-formatted semi-structured data.
        Csv = 100,
        /// Json-formatted semi-structured data.
        Json = 101,
        /// Image data formats (such as jpg and png).
        Image = 200,
        /// Audio data formats (such as mp3, and wav).
        Audio = 201,
        /// Video data formats (such as mp4 and mpg).
        Video = 202,
        /// Textual data formats (such as txt and xml).
        Text = 203,
        /// TensorFlow record format.
        Tfrecord = 204,
        /// Data that doesn't match a specific format.
        Other = 1000,
        /// Data of an unknown format.
        Unknown = 1001,
    }
    /// The specific compressed file format of the data.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CompressionFormat {
        /// CompressionFormat unspecified. Implies uncompressed data.
        Unspecified = 0,
        /// GZip compressed set of files.
        Gzip = 2,
        /// BZip2 compressed set of files.
        Bzip2 = 3,
    }
    /// Additional format-specific options.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Options {
        /// Optional. Additional information about CSV formatted data.
        #[prost(message, tag="10")]
        Csv(CsvOptions),
        /// Optional. Additional information about CSV formatted data.
        #[prost(message, tag="11")]
        Json(JsonOptions),
    }
}
/// Identifies the cloud system that manages the data storage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StorageSystem {
    /// Storage system unspecified.
    Unspecified = 0,
    /// The entity data is contained within a Cloud Storage bucket.
    CloudStorage = 1,
    /// The entity data is contained within a BigQuery dataset.
    Bigquery = 2,
}
/// Generated client implementations.
pub mod metadata_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Metadata service manages metadata resources such as tables, filesets and
    /// partitions.
    #[derive(Debug, Clone)]
    pub struct MetadataServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> MetadataServiceClient<T>
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
        ) -> MetadataServiceClient<InterceptedService<T, F>>
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
            MetadataServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Create a metadata entity.
        pub async fn create_entity(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEntityRequest>,
        ) -> Result<tonic::Response<super::Entity>, tonic::Status> {
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
                "/google.cloud.dataplex.v1.MetadataService/CreateEntity",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Update a metadata entity. Only supports full resource update.
        pub async fn update_entity(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEntityRequest>,
        ) -> Result<tonic::Response<super::Entity>, tonic::Status> {
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
                "/google.cloud.dataplex.v1.MetadataService/UpdateEntity",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Delete a metadata entity.
        pub async fn delete_entity(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEntityRequest>,
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
                "/google.cloud.dataplex.v1.MetadataService/DeleteEntity",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get a metadata entity.
        pub async fn get_entity(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEntityRequest>,
        ) -> Result<tonic::Response<super::Entity>, tonic::Status> {
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
                "/google.cloud.dataplex.v1.MetadataService/GetEntity",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List metadata entities in a zone.
        pub async fn list_entities(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEntitiesRequest>,
        ) -> Result<tonic::Response<super::ListEntitiesResponse>, tonic::Status> {
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
                "/google.cloud.dataplex.v1.MetadataService/ListEntities",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Create a metadata partition.
        pub async fn create_partition(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePartitionRequest>,
        ) -> Result<tonic::Response<super::Partition>, tonic::Status> {
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
                "/google.cloud.dataplex.v1.MetadataService/CreatePartition",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Delete a metadata partition.
        pub async fn delete_partition(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePartitionRequest>,
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
                "/google.cloud.dataplex.v1.MetadataService/DeletePartition",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get a metadata partition of an entity.
        pub async fn get_partition(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPartitionRequest>,
        ) -> Result<tonic::Response<super::Partition>, tonic::Status> {
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
                "/google.cloud.dataplex.v1.MetadataService/GetPartition",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List metadata partitions of an entity.
        pub async fn list_partitions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPartitionsRequest>,
        ) -> Result<tonic::Response<super::ListPartitionsResponse>, tonic::Status> {
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
                "/google.cloud.dataplex.v1.MetadataService/ListPartitions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// A task represents a user-visible job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Task {
    /// Output only. The relative resource name of the task, of the form:
    /// projects/{project_number}/locations/{location_id}/lakes/{lake_id}/
    /// tasks/{task_id}.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. System generated globally unique ID for the task. This ID will be
    /// different if the task is deleted and re-created with the same name.
    #[prost(string, tag="2")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. The time when the task was created.
    #[prost(message, optional, tag="3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the task was last updated.
    #[prost(message, optional, tag="4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Description of the task.
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
    /// Optional. User friendly display name.
    #[prost(string, tag="6")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. Current state of the task.
    #[prost(enumeration="State", tag="7")]
    pub state: i32,
    /// Optional. User-defined labels for the task.
    #[prost(btree_map="string, string", tag="8")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Required. Spec related to how often and when a task should be triggered.
    #[prost(message, optional, tag="100")]
    pub trigger_spec: ::core::option::Option<task::TriggerSpec>,
    /// Required. Spec related to how a task is executed.
    #[prost(message, optional, tag="101")]
    pub execution_spec: ::core::option::Option<task::ExecutionSpec>,
    /// Output only. Status of the latest task executions.
    #[prost(message, optional, tag="201")]
    pub execution_status: ::core::option::Option<task::ExecutionStatus>,
    /// Task template specific user-specified config.
    #[prost(oneof="task::Config", tags="300")]
    pub config: ::core::option::Option<task::Config>,
}
/// Nested message and enum types in `Task`.
pub mod task {
    /// Configuration for the underlying infrastructure used to run workloads.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InfrastructureSpec {
        /// Hardware config.
        #[prost(oneof="infrastructure_spec::Resources", tags="52")]
        pub resources: ::core::option::Option<infrastructure_spec::Resources>,
        /// Software config.
        #[prost(oneof="infrastructure_spec::Runtime", tags="101")]
        pub runtime: ::core::option::Option<infrastructure_spec::Runtime>,
        /// Networking config.
        #[prost(oneof="infrastructure_spec::Network", tags="150")]
        pub network: ::core::option::Option<infrastructure_spec::Network>,
    }
    /// Nested message and enum types in `InfrastructureSpec`.
    pub mod infrastructure_spec {
        /// Batch compute resources associated with the task.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct BatchComputeResources {
            /// Optional. Total number of job executors.
            /// Executor Count should be between 2 and 100. \[Default=2\]
            #[prost(int32, tag="1")]
            pub executors_count: i32,
            /// Optional. Max configurable executors.
            /// If max_executors_count > executors_count, then auto-scaling is enabled.
            /// Max Executor Count should be between 2 and 1000. \[Default=1000\]
            #[prost(int32, tag="2")]
            pub max_executors_count: i32,
        }
        /// Container Image Runtime Configuration used with Batch execution.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ContainerImageRuntime {
            /// Optional. Container image to use.
            #[prost(string, tag="1")]
            pub image: ::prost::alloc::string::String,
            /// Optional. A list of Java JARS to add to the classpath.
            /// Valid input includes Cloud Storage URIs to Jar binaries.
            /// For example, gs://bucket-name/my/path/to/file.jar
            #[prost(string, repeated, tag="2")]
            pub java_jars: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// Optional. A list of python packages to be installed.
            /// Valid formats include Cloud Storage URI to a PIP installable library.
            /// For example, gs://bucket-name/my/path/to/lib.tar.gz
            #[prost(string, repeated, tag="3")]
            pub python_packages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// Optional. Override to common configuration of open source components installed on
            /// the Dataproc cluster.
            /// The properties to set on daemon config files.
            /// Property keys are specified in `prefix:property` format, for example
            /// `core:hadoop.tmp.dir`.
            /// For more information, see [Cluster
            /// properties](<https://cloud.google.com/dataproc/docs/concepts/cluster-properties>).
            #[prost(btree_map="string, string", tag="4")]
            pub properties: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
        }
        /// Cloud VPC Network used to run the infrastructure.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct VpcNetwork {
            /// Optional. List of network tags to apply to the job.
            #[prost(string, repeated, tag="3")]
            pub network_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// The Cloud VPC network identifier.
            #[prost(oneof="vpc_network::NetworkName", tags="1, 2")]
            pub network_name: ::core::option::Option<vpc_network::NetworkName>,
        }
        /// Nested message and enum types in `VpcNetwork`.
        pub mod vpc_network {
            /// The Cloud VPC network identifier.
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum NetworkName {
                /// Optional. The Cloud VPC network in which the job is run. By default, the Cloud
                /// VPC network named Default within the project is used.
                #[prost(string, tag="1")]
                Network(::prost::alloc::string::String),
                /// Optional. The Cloud VPC sub-network in which the job is run.
                #[prost(string, tag="2")]
                SubNetwork(::prost::alloc::string::String),
            }
        }
        /// Hardware config.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Resources {
            /// Compute resources needed for a Task when using Dataproc Serverless.
            #[prost(message, tag="52")]
            Batch(BatchComputeResources),
        }
        /// Software config.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Runtime {
            /// Container Image Runtime Configuration.
            #[prost(message, tag="101")]
            ContainerImage(ContainerImageRuntime),
        }
        /// Networking config.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Network {
            /// Vpc network.
            #[prost(message, tag="150")]
            VpcNetwork(VpcNetwork),
        }
    }
    /// Task scheduling and trigger settings.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TriggerSpec {
        /// Required. Immutable. Trigger type of the user-specified Task.
        #[prost(enumeration="trigger_spec::Type", tag="5")]
        pub r#type: i32,
        /// Optional. The first run of the task will be after this time.
        /// If not specified, the task will run shortly after being submitted if
        /// ON_DEMAND and based on the schedule if RECURRING.
        #[prost(message, optional, tag="6")]
        pub start_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Optional. Prevent the task from executing.
        /// This does not cancel already running tasks. It is intended to temporarily
        /// disable RECURRING tasks.
        #[prost(bool, tag="4")]
        pub disabled: bool,
        /// Optional. Number of retry attempts before aborting.
        /// Set to zero to never attempt to retry a failed task.
        #[prost(int32, tag="7")]
        pub max_retries: i32,
        /// Trigger only applies for RECURRING tasks.
        #[prost(oneof="trigger_spec::Trigger", tags="100")]
        pub trigger: ::core::option::Option<trigger_spec::Trigger>,
    }
    /// Nested message and enum types in `TriggerSpec`.
    pub mod trigger_spec {
        /// Determines how often and when the job will run.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Type {
            /// Unspecified trigger type.
            Unspecified = 0,
            /// The task runs one-time shortly after Task Creation.
            OnDemand = 1,
            /// The task is scheduled to run periodically.
            Recurring = 2,
        }
        /// Trigger only applies for RECURRING tasks.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Trigger {
            /// Optional. Cron schedule (<https://en.wikipedia.org/wiki/Cron>) for running
            /// tasks periodically.
            /// To explicitly set a timezone to the cron tab, apply a prefix in the
            /// cron tab: "CRON_TZ=${IANA_TIME_ZONE}" or "TZ=${IANA_TIME_ZONE}".
            /// The ${IANA_TIME_ZONE} may only be a valid string from IANA time zone
            /// database. For example, "CRON_TZ=America/New_York 1 * * * *", or
            /// "TZ=America/New_York 1 * * * *".
            /// This field is required for RECURRING tasks.
            #[prost(string, tag="100")]
            Schedule(::prost::alloc::string::String),
        }
    }
    /// Execution related settings, like retry and service_account.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExecutionSpec {
        /// Optional. The arguments to pass to the task.
        /// The args can use placeholders of the format ${placeholder} as
        /// part of key/value string. These will be interpolated before passing the
        /// args to the driver. Currently supported placeholders:
        /// - ${task_id}
        /// - ${job_time}
        /// To pass positional args, set the key as TASK_ARGS. The value should be a
        /// comma-separated string of all the positional arguments. To use a
        /// delimiter other than comma, refer to
        /// <https://cloud.google.com/sdk/gcloud/reference/topic/escaping.> In case of
        /// other keys being present in the args, then TASK_ARGS will be passed as
        /// the last argument.
        #[prost(btree_map="string, string", tag="4")]
        pub args: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
        /// Required. Service account to use to execute a task.
        /// If not provided, the default Compute service account for the project is
        /// used.
        #[prost(string, tag="5")]
        pub service_account: ::prost::alloc::string::String,
        /// Optional. The project in which jobs are run. By default, the project containing the
        /// Lake is used. If a project is provided, the
        /// \[ExecutionSpec.service_account][google.cloud.dataplex.v1.Task.ExecutionSpec.service_account\] must belong to this project.
        #[prost(string, tag="7")]
        pub project: ::prost::alloc::string::String,
        /// Optional. The maximum duration after which the job execution is expired.
        #[prost(message, optional, tag="8")]
        pub max_job_execution_lifetime: ::core::option::Option<::prost_types::Duration>,
        /// Optional. The Cloud KMS key to use for encryption, of the form:
        /// `projects/{project_number}/locations/{location_id}/keyRings/{key-ring-name}/cryptoKeys/{key-name}`.
        #[prost(string, tag="9")]
        pub kms_key: ::prost::alloc::string::String,
    }
    /// User-specified config for running a Spark task.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SparkTaskConfig {
        /// Optional. Cloud Storage URIs of files to be placed in the working directory of each
        /// executor.
        #[prost(string, repeated, tag="3")]
        pub file_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Optional. Cloud Storage URIs of archives to be extracted into the working directory
        /// of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and
        /// .zip.
        #[prost(string, repeated, tag="4")]
        pub archive_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Optional. Infrastructure specification for the execution.
        #[prost(message, optional, tag="6")]
        pub infrastructure_spec: ::core::option::Option<InfrastructureSpec>,
        /// Required. The specification of the main method to call to drive the
        /// job. Specify either the jar file that contains the main class or the
        /// main class name.
        #[prost(oneof="spark_task_config::Driver", tags="100, 101, 102, 104, 105")]
        pub driver: ::core::option::Option<spark_task_config::Driver>,
    }
    /// Nested message and enum types in `SparkTaskConfig`.
    pub mod spark_task_config {
        /// Required. The specification of the main method to call to drive the
        /// job. Specify either the jar file that contains the main class or the
        /// main class name.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Driver {
            /// The Cloud Storage URI of the jar file that contains the main class.
            /// The execution args are passed in as a sequence of named process
            /// arguments (`--key=value`).
            #[prost(string, tag="100")]
            MainJarFileUri(::prost::alloc::string::String),
            /// The name of the driver's main class. The jar file that contains the
            /// class must be in the default CLASSPATH or specified in
            /// `jar_file_uris`.
            /// The execution args are passed in as a sequence of named process
            /// arguments (`--key=value`).
            #[prost(string, tag="101")]
            MainClass(::prost::alloc::string::String),
            /// The Gcloud Storage URI of the main Python file to use as the driver.
            /// Must be a .py file. The execution args are passed in as a sequence of
            /// named process arguments (`--key=value`).
            #[prost(string, tag="102")]
            PythonScriptFile(::prost::alloc::string::String),
            /// A reference to a query file. This can be the Cloud Storage URI of the
            /// query file or it can the path to a SqlScript Content. The execution
            /// args are used to declare a set of script variables
            /// (`set key="value";`).
            #[prost(string, tag="104")]
            SqlScriptFile(::prost::alloc::string::String),
            /// The query text.
            /// The execution args are used to declare a set of script variables
            /// (`set key="value";`).
            #[prost(string, tag="105")]
            SqlScript(::prost::alloc::string::String),
        }
    }
    /// Status of the task execution (e.g. Jobs).
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExecutionStatus {
        /// Output only. Last update time of the status.
        #[prost(message, optional, tag="3")]
        pub update_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Output only. latest job execution
        #[prost(message, optional, tag="9")]
        pub latest_job: ::core::option::Option<super::Job>,
    }
    /// Task template specific user-specified config.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Config {
        /// Config related to running custom Spark tasks.
        #[prost(message, tag="300")]
        Spark(SparkTaskConfig),
    }
}
/// A job represents an instance of a task.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Job {
    /// Output only. The relative resource name of the job, of the form:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}/tasks/{task_id}/jobs/{job_id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. System generated globally unique ID for the job.
    #[prost(string, tag="2")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. The time when the job was started.
    #[prost(message, optional, tag="3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the job ended.
    #[prost(message, optional, tag="4")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Execution state for the job.
    #[prost(enumeration="job::State", tag="5")]
    pub state: i32,
    /// Output only. The number of times the job has been retried (excluding the
    /// initial attempt).
    #[prost(uint32, tag="6")]
    pub retry_count: u32,
    /// Output only. The underlying service running a job.
    #[prost(enumeration="job::Service", tag="7")]
    pub service: i32,
    /// Output only. The full resource name for the job run under a particular service.
    #[prost(string, tag="8")]
    pub service_job: ::prost::alloc::string::String,
    /// Output only. Additional information about the current state.
    #[prost(string, tag="9")]
    pub message: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Job`.
pub mod job {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Service {
        /// Service used to run the job is unspecified.
        Unspecified = 0,
        /// Dataproc service is used to run this job.
        Dataproc = 1,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The job state is unknown.
        Unspecified = 0,
        /// The job is running.
        Running = 1,
        /// The job is cancelling.
        Cancelling = 2,
        /// The job cancellation was successful.
        Cancelled = 3,
        /// The job completed successfully.
        Succeeded = 4,
        /// The job is no longer running due to an error.
        Failed = 5,
        /// The job was cancelled outside of Dataplex.
        Aborted = 6,
    }
}
/// The payload associated with Discovery data processing.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscoveryEvent {
    /// The log message.
    #[prost(string, tag="1")]
    pub message: ::prost::alloc::string::String,
    /// The id of the associated lake.
    #[prost(string, tag="2")]
    pub lake_id: ::prost::alloc::string::String,
    /// The id of the associated zone.
    #[prost(string, tag="3")]
    pub zone_id: ::prost::alloc::string::String,
    /// The id of the associated asset.
    #[prost(string, tag="4")]
    pub asset_id: ::prost::alloc::string::String,
    /// The data location associated with the event.
    #[prost(string, tag="5")]
    pub data_location: ::prost::alloc::string::String,
    /// The type of the event being logged.
    #[prost(enumeration="discovery_event::EventType", tag="10")]
    pub r#type: i32,
    /// Additional details about the event.
    #[prost(oneof="discovery_event::Details", tags="20, 21, 22, 23")]
    pub details: ::core::option::Option<discovery_event::Details>,
}
/// Nested message and enum types in `DiscoveryEvent`.
pub mod discovery_event {
    /// Details about configuration events.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConfigDetails {
        /// A list of discovery configuration parameters in effect.
        /// The keys are the field paths within DiscoverySpec.
        /// Eg. includePatterns, excludePatterns, csvOptions.disableTypeInference,
        /// etc.
        #[prost(btree_map="string, string", tag="1")]
        pub parameters: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    }
    /// Details about the entity.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EntityDetails {
        /// The name of the entity resource.
        /// The name is the fully-qualified resource name.
        #[prost(string, tag="1")]
        pub entity: ::prost::alloc::string::String,
        /// The type of the entity resource.
        #[prost(enumeration="EntityType", tag="2")]
        pub r#type: i32,
    }
    /// Details about the partition.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PartitionDetails {
        /// The name to the partition resource.
        /// The name is the fully-qualified resource name.
        #[prost(string, tag="1")]
        pub partition: ::prost::alloc::string::String,
        /// The name to the containing entity resource.
        /// The name is the fully-qualified resource name.
        #[prost(string, tag="2")]
        pub entity: ::prost::alloc::string::String,
        /// The type of the containing entity resource.
        #[prost(enumeration="EntityType", tag="3")]
        pub r#type: i32,
        /// The locations of the data items (e.g., a Cloud Storage objects) sampled
        /// for metadata inference.
        #[prost(string, repeated, tag="4")]
        pub sampled_data_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// Details about the action.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ActionDetails {
        /// The type of action.
        /// Eg. IncompatibleDataSchema, InvalidDataFormat
        #[prost(string, tag="1")]
        pub r#type: ::prost::alloc::string::String,
    }
    /// The type of the event.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EventType {
        /// An unspecified event type.
        Unspecified = 0,
        /// An event representing discovery configuration in effect.
        Config = 1,
        /// An event representing a metadata entity being created.
        EntityCreated = 2,
        /// An event representing a metadata entity being updated.
        EntityUpdated = 3,
        /// An event representing a metadata entity being deleted.
        EntityDeleted = 4,
        /// An event representing a partition being created.
        PartitionCreated = 5,
        /// An event representing a partition being updated.
        PartitionUpdated = 6,
        /// An event representing a partition being deleted.
        PartitionDeleted = 7,
    }
    /// The type of the entity.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EntityType {
        /// An unspecified event type.
        Unspecified = 0,
        /// Entities representing structured data.
        Table = 1,
        /// Entities representing unstructured data.
        Fileset = 2,
    }
    /// Additional details about the event.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Details {
        /// Details about discovery configuration in effect.
        #[prost(message, tag="20")]
        Config(ConfigDetails),
        /// Details about the entity associated with the event.
        #[prost(message, tag="21")]
        Entity(EntityDetails),
        /// Details about the partition associated with the event.
        #[prost(message, tag="22")]
        Partition(PartitionDetails),
        /// Details about the action associated with the event.
        #[prost(message, tag="23")]
        Action(ActionDetails),
    }
}
/// The payload associated with Job logs that contains events describing jobs
/// that have run within a Lake.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobEvent {
    /// The log message.
    #[prost(string, tag="1")]
    pub message: ::prost::alloc::string::String,
    /// The unique id identifying the job.
    #[prost(string, tag="2")]
    pub job_id: ::prost::alloc::string::String,
    /// The time when the job started running.
    #[prost(message, optional, tag="3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time when the job ended running.
    #[prost(message, optional, tag="4")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The job state on completion.
    #[prost(enumeration="job_event::State", tag="5")]
    pub state: i32,
    /// The number of retries.
    #[prost(int32, tag="6")]
    pub retries: i32,
    /// The type of the job.
    #[prost(enumeration="job_event::Type", tag="7")]
    pub r#type: i32,
    /// The service used to execute the job.
    #[prost(enumeration="job_event::Service", tag="8")]
    pub service: i32,
    /// The reference to the job within the service.
    #[prost(string, tag="9")]
    pub service_job: ::prost::alloc::string::String,
}
/// Nested message and enum types in `JobEvent`.
pub mod job_event {
    /// The type of the job.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Unspecified job type.
        Unspecified = 0,
        /// Spark jobs.
        Spark = 1,
        /// Notebook jobs.
        Notebook = 2,
    }
    /// The completion status of the job.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified job state.
        Unspecified = 0,
        /// Job successfully completed.
        Succeeded = 1,
        /// Job was unsuccessful.
        Failed = 2,
        /// Job was cancelled by the user.
        Cancelled = 3,
        /// Job was cancelled or aborted via the service executing the job.
        Aborted = 4,
    }
    /// The service used to execute the job.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Service {
        /// Unspecified service.
        Unspecified = 0,
        /// Cloud Dataproc.
        Dataproc = 1,
    }
}
/// These messages contain information about sessions within an environment.
/// The monitored resource is 'Environment'.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionEvent {
    /// The log message.
    #[prost(string, tag="1")]
    pub message: ::prost::alloc::string::String,
    /// The information about the user that created the session.
    #[prost(string, tag="2")]
    pub user_id: ::prost::alloc::string::String,
    /// Unique identifier for the session.
    #[prost(string, tag="3")]
    pub session_id: ::prost::alloc::string::String,
    /// The type of the event.
    #[prost(enumeration="session_event::EventType", tag="4")]
    pub r#type: i32,
    /// Additional information about the Query metadata.
    #[prost(oneof="session_event::Detail", tags="5")]
    pub detail: ::core::option::Option<session_event::Detail>,
}
/// Nested message and enum types in `SessionEvent`.
pub mod session_event {
    /// Execution details of the query.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QueryDetail {
        /// The unique Query id identifying the query.
        #[prost(string, tag="1")]
        pub query_id: ::prost::alloc::string::String,
        /// The query text executed.
        #[prost(string, tag="2")]
        pub query_text: ::prost::alloc::string::String,
        /// Query Execution engine.
        #[prost(enumeration="query_detail::Engine", tag="3")]
        pub engine: i32,
        /// Time taken for execution of the query.
        #[prost(message, optional, tag="4")]
        pub duration: ::core::option::Option<::prost_types::Duration>,
        /// The size of results the query produced.
        #[prost(int64, tag="5")]
        pub result_size_bytes: i64,
        /// The data processed by the query.
        #[prost(int64, tag="6")]
        pub data_processed_bytes: i64,
    }
    /// Nested message and enum types in `QueryDetail`.
    pub mod query_detail {
        /// Query Execution engine.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Engine {
            /// An unspecified Engine type.
            Unspecified = 0,
            /// Spark-sql engine is specified in Query.
            SparkSql = 1,
            /// BigQuery engine is specified in Query.
            Bigquery = 2,
        }
    }
    /// The type of the event.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EventType {
        /// An unspecified event type.
        Unspecified = 0,
        /// Event for start of a session.
        Start = 1,
        /// Event for stop of a session.
        Stop = 2,
        /// Query events in the session.
        Query = 3,
    }
    /// Additional information about the Query metadata.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Detail {
        /// The execution details of the query.
        #[prost(message, tag="5")]
        Query(QueryDetail),
    }
}
/// Environment represents a user-visible compute infrastructure for analytics
/// within a lake.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Environment {
    /// Output only. The relative resource name of the environment, of the form:
    /// projects/{project_id}/locations/{location_id}/lakes/{lake_id}/environment/{environment_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. User friendly display name.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. System generated globally unique ID for the environment. This ID will be
    /// different if the environment is deleted and re-created with the same name.
    #[prost(string, tag="3")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. Environment creation time.
    #[prost(message, optional, tag="4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the environment was last updated.
    #[prost(message, optional, tag="5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. User defined labels for the environment.
    #[prost(btree_map="string, string", tag="6")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. Description of the environment.
    #[prost(string, tag="7")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Current state of the environment.
    #[prost(enumeration="State", tag="8")]
    pub state: i32,
    /// Required. Infrastructure specification for the Environment.
    #[prost(message, optional, tag="100")]
    pub infrastructure_spec: ::core::option::Option<environment::InfrastructureSpec>,
    /// Optional. Configuration for sessions created for this environment.
    #[prost(message, optional, tag="101")]
    pub session_spec: ::core::option::Option<environment::SessionSpec>,
    /// Output only. Status of sessions created for this environment.
    #[prost(message, optional, tag="102")]
    pub session_status: ::core::option::Option<environment::SessionStatus>,
    /// Output only. URI Endpoints to access sessions associated with the Environment.
    #[prost(message, optional, tag="200")]
    pub endpoints: ::core::option::Option<environment::Endpoints>,
}
/// Nested message and enum types in `Environment`.
pub mod environment {
    /// Configuration for the underlying infrastructure used to run workloads.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InfrastructureSpec {
        /// Hardware config
        #[prost(oneof="infrastructure_spec::Resources", tags="50")]
        pub resources: ::core::option::Option<infrastructure_spec::Resources>,
        /// Software config
        #[prost(oneof="infrastructure_spec::Runtime", tags="100")]
        pub runtime: ::core::option::Option<infrastructure_spec::Runtime>,
    }
    /// Nested message and enum types in `InfrastructureSpec`.
    pub mod infrastructure_spec {
        /// Compute resources associated with the analyze interactive workloads.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ComputeResources {
            /// Optional. Size in GB of the disk. Default is 100 GB.
            #[prost(int32, tag="1")]
            pub disk_size_gb: i32,
            /// Optional. Total number of nodes in the sessions created for this environment.
            #[prost(int32, tag="2")]
            pub node_count: i32,
            /// Optional. Max configurable nodes.
            /// If max_node_count > node_count, then auto-scaling is enabled.
            #[prost(int32, tag="3")]
            pub max_node_count: i32,
        }
        /// Software Runtime Configuration to run Analyze.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct OsImageRuntime {
            /// Required. Dataplex Image version.
            #[prost(string, tag="1")]
            pub image_version: ::prost::alloc::string::String,
            /// Optional. List of Java jars to be included in the runtime environment.
            /// Valid input includes Cloud Storage URIs to Jar binaries.
            /// For example, gs://bucket-name/my/path/to/file.jar
            #[prost(string, repeated, tag="2")]
            pub java_libraries: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// Optional. A list of python packages to be installed.
            /// Valid formats include Cloud Storage URI to a PIP installable library.
            /// For example, gs://bucket-name/my/path/to/lib.tar.gz
            #[prost(string, repeated, tag="3")]
            pub python_packages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// Optional. Spark properties to provide configuration for use in sessions created
            /// for this environment. The properties to set on daemon config files.
            /// Property keys are specified in `prefix:property` format.
            /// The prefix must be "spark".
            #[prost(btree_map="string, string", tag="4")]
            pub properties: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
        }
        /// Hardware config
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Resources {
            /// Optional. Compute resources needed for analyze interactive workloads.
            #[prost(message, tag="50")]
            Compute(ComputeResources),
        }
        /// Software config
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Runtime {
            /// Required. Software Runtime Configuration for analyze interactive workloads.
            #[prost(message, tag="100")]
            OsImage(OsImageRuntime),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SessionSpec {
        /// Optional. The idle time configuration of the session. The session will be
        /// auto-terminated at the end of this period.
        #[prost(message, optional, tag="1")]
        pub max_idle_duration: ::core::option::Option<::prost_types::Duration>,
        /// Optional. If True, this causes sessions to be pre-created and available for faster
        /// startup to enable interactive exploration use-cases. This defaults to
        /// False to avoid additional billed charges.
        /// These can only be set to True for the environment with name set to
        /// "default", and with default configuration.
        #[prost(bool, tag="2")]
        pub enable_fast_startup: bool,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SessionStatus {
        /// Output only. Queries over sessions to mark whether the environment is currently
        /// active or not
        #[prost(bool, tag="1")]
        pub active: bool,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Endpoints {
        /// Output only. URI to serve notebook APIs
        #[prost(string, tag="1")]
        pub notebooks: ::prost::alloc::string::String,
        /// Output only. URI to serve SQL APIs
        #[prost(string, tag="2")]
        pub sql: ::prost::alloc::string::String,
    }
}
/// Content represents a user-visible notebook or a sql script
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Content {
    /// Output only. The relative resource name of the content, of the form:
    /// projects/{project_id}/locations/{location_id}/lakes/{lake_id}/content/{content_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. System generated globally unique ID for the content. This ID will be
    /// different if the content is deleted and re-created with the same name.
    #[prost(string, tag="2")]
    pub uid: ::prost::alloc::string::String,
    /// Required. The path for the Content file, represented as directory structure.
    /// Unique within a lake.
    /// Limited to alphanumerics, hyphens, underscores, dots and slashes.
    #[prost(string, tag="3")]
    pub path: ::prost::alloc::string::String,
    /// Output only. Content creation time.
    #[prost(message, optional, tag="4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the content was last updated.
    #[prost(message, optional, tag="5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. User defined labels for the content.
    #[prost(btree_map="string, string", tag="6")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. Description of the content.
    #[prost(string, tag="7")]
    pub description: ::prost::alloc::string::String,
    /// Only returned in `GetContent` requests and not in `ListContent` request.
    #[prost(oneof="content::Data", tags="9")]
    pub data: ::core::option::Option<content::Data>,
    #[prost(oneof="content::Content", tags="100, 101")]
    pub content: ::core::option::Option<content::Content>,
}
/// Nested message and enum types in `Content`.
pub mod content {
    /// Configuration for the Sql Script content.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SqlScript {
        /// Required. Query Engine to be used for the Sql Query.
        #[prost(enumeration="sql_script::QueryEngine", tag="1")]
        pub engine: i32,
    }
    /// Nested message and enum types in `SqlScript`.
    pub mod sql_script {
        /// Query Engine Type of the SQL Script.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum QueryEngine {
            /// Value was unspecified.
            Unspecified = 0,
            /// Spark SQL Query.
            Spark = 2,
        }
    }
    /// Configuration for Notebook content.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Notebook {
        /// Required. Kernel Type of the notebook.
        #[prost(enumeration="notebook::KernelType", tag="1")]
        pub kernel_type: i32,
    }
    /// Nested message and enum types in `Notebook`.
    pub mod notebook {
        /// Kernel Type of the Jupyter notebook.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum KernelType {
            /// Kernel Type unspecified.
            Unspecified = 0,
            /// Python 3 Kernel.
            Python3 = 1,
        }
    }
    /// Only returned in `GetContent` requests and not in `ListContent` request.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        /// Required. Content data in string format.
        #[prost(string, tag="9")]
        DataText(::prost::alloc::string::String),
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        /// Sql Script related configurations.
        #[prost(message, tag="100")]
        SqlScript(SqlScript),
        /// Notebook related configurations.
        #[prost(message, tag="101")]
        Notebook(Notebook),
    }
}
/// Represents an active analyze session running for a user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Session {
    /// Output only. The relative resource name of the content, of the form:
    /// projects/{project_id}/locations/{location_id}/lakes/{lake_id}/environment/{environment_id}/sessions/{session_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Email of user running the session.
    #[prost(string, tag="2")]
    pub user_id: ::prost::alloc::string::String,
    /// Output only. Session start time.
    #[prost(message, optional, tag="3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(enumeration="State", tag="4")]
    pub state: i32,
}
/// Create lake request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateLakeRequest {
    /// Required. The resource name of the lake location, of the form:
    /// projects/{project_number}/locations/{location_id}
    /// where `location_id` refers to a GCP region.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Lake identifier.
    /// This ID will be used to generate names such as database and dataset names
    /// when publishing metadata to Hive Metastore and BigQuery.
    /// * Must contain only lowercase letters, numbers and hyphens.
    /// * Must start with a letter.
    /// * Must end with a number or a letter.
    /// * Must be between 1-63 characters.
    /// * Must be unique within the customer project / location.
    #[prost(string, tag="2")]
    pub lake_id: ::prost::alloc::string::String,
    /// Required. Lake resource
    #[prost(message, optional, tag="3")]
    pub lake: ::core::option::Option<Lake>,
    /// Optional. Only validate the request, but do not perform mutations.
    /// The default is false.
    #[prost(bool, tag="4")]
    pub validate_only: bool,
}
/// Update lake request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLakeRequest {
    /// Required. Mask of fields to update.
    #[prost(message, optional, tag="1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Update description.
    /// Only fields specified in `update_mask` are updated.
    #[prost(message, optional, tag="2")]
    pub lake: ::core::option::Option<Lake>,
    /// Optional. Only validate the request, but do not perform mutations.
    /// The default is false.
    #[prost(bool, tag="3")]
    pub validate_only: bool,
}
/// Delete lake request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteLakeRequest {
    /// Required. The resource name of the lake:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// List lakes request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLakesRequest {
    /// Required. The resource name of the lake location, of the form:
    /// `projects/{project_number}/locations/{location_id}`
    /// where `location_id` refers to a GCP region.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of Lakes to return. The service may return fewer than this
    /// value. If unspecified, at most 10 lakes will be returned. The maximum
    /// value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. Page token received from a previous `ListLakes` call. Provide this to
    /// retrieve the subsequent page. When paginating, all other parameters
    /// provided to `ListLakes` must match the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filter request.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Order by fields for the result.
    #[prost(string, tag="5")]
    pub order_by: ::prost::alloc::string::String,
}
/// List lakes response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLakesResponse {
    /// Lakes under the given parent location.
    #[prost(message, repeated, tag="1")]
    pub lakes: ::prost::alloc::vec::Vec<Lake>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag="3")]
    pub unreachable_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// List lake actions request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLakeActionsRequest {
    /// Required. The resource name of the parent lake:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of actions to return. The service may return fewer than this
    /// value. If unspecified, at most 10 actions will be returned. The maximum
    /// value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. Page token received from a previous `ListLakeActions` call. Provide this to
    /// retrieve the subsequent page. When paginating, all other parameters
    /// provided to `ListLakeActions` must match the call that provided the page
    /// token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// List actions response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListActionsResponse {
    /// Actions under the given parent lake/zone/asset.
    #[prost(message, repeated, tag="1")]
    pub actions: ::prost::alloc::vec::Vec<Action>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Get lake request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLakeRequest {
    /// Required. The resource name of the lake:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Create zone request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateZoneRequest {
    /// Required. The resource name of the parent lake:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Zone identifier.
    /// This ID will be used to generate names such as database and dataset names
    /// when publishing metadata to Hive Metastore and BigQuery.
    /// * Must contain only lowercase letters, numbers and hyphens.
    /// * Must start with a letter.
    /// * Must end with a number or a letter.
    /// * Must be between 1-63 characters.
    /// * Must be unique across all lakes from all locations in a project.
    /// * Must not be one of the reserved IDs (i.e. "default", "global-temp")
    #[prost(string, tag="2")]
    pub zone_id: ::prost::alloc::string::String,
    /// Required. Zone resource.
    #[prost(message, optional, tag="3")]
    pub zone: ::core::option::Option<Zone>,
    /// Optional. Only validate the request, but do not perform mutations.
    /// The default is false.
    #[prost(bool, tag="4")]
    pub validate_only: bool,
}
/// Update zone request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateZoneRequest {
    /// Required. Mask of fields to update.
    #[prost(message, optional, tag="1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Update description.
    /// Only fields specified in `update_mask` are updated.
    #[prost(message, optional, tag="2")]
    pub zone: ::core::option::Option<Zone>,
    /// Optional. Only validate the request, but do not perform mutations.
    /// The default is false.
    #[prost(bool, tag="3")]
    pub validate_only: bool,
}
/// Delete zone request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteZoneRequest {
    /// Required. The resource name of the zone:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// List zones request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListZonesRequest {
    /// Required. The resource name of the parent lake:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of zones to return. The service may return fewer than this
    /// value. If unspecified, at most 10 zones will be returned. The maximum
    /// value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. Page token received from a previous `ListZones` call. Provide this to
    /// retrieve the subsequent page. When paginating, all other parameters
    /// provided to `ListZones` must match the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filter request.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Order by fields for the result.
    #[prost(string, tag="5")]
    pub order_by: ::prost::alloc::string::String,
}
/// List zones response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListZonesResponse {
    /// Zones under the given parent lake.
    #[prost(message, repeated, tag="1")]
    pub zones: ::prost::alloc::vec::Vec<Zone>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// List zone actions request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListZoneActionsRequest {
    /// Required. The resource name of the parent zone:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of actions to return. The service may return fewer than this
    /// value. If unspecified, at most 10 actions will be returned. The maximum
    /// value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. Page token received from a previous `ListZoneActions` call. Provide this to
    /// retrieve the subsequent page. When paginating, all other parameters
    /// provided to `ListZoneActions` must match the call that provided the page
    /// token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Get zone request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetZoneRequest {
    /// Required. The resource name of the zone:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Create asset request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAssetRequest {
    /// Required. The resource name of the parent zone:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Asset identifier.
    /// This ID will be used to generate names such as table names when publishing
    /// metadata to Hive Metastore and BigQuery.
    /// * Must contain only lowercase letters, numbers and hyphens.
    /// * Must start with a letter.
    /// * Must end with a number or a letter.
    /// * Must be between 1-63 characters.
    /// * Must be unique within the zone.
    #[prost(string, tag="2")]
    pub asset_id: ::prost::alloc::string::String,
    /// Required. Asset resource.
    #[prost(message, optional, tag="3")]
    pub asset: ::core::option::Option<Asset>,
    /// Optional. Only validate the request, but do not perform mutations.
    /// The default is false.
    #[prost(bool, tag="4")]
    pub validate_only: bool,
}
/// Update asset request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAssetRequest {
    /// Required. Mask of fields to update.
    #[prost(message, optional, tag="1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Update description.
    /// Only fields specified in `update_mask` are updated.
    #[prost(message, optional, tag="2")]
    pub asset: ::core::option::Option<Asset>,
    /// Optional. Only validate the request, but do not perform mutations.
    /// The default is false.
    #[prost(bool, tag="3")]
    pub validate_only: bool,
}
/// Delete asset request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAssetRequest {
    /// Required. The resource name of the asset:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/assets/{asset_id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// List assets request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAssetsRequest {
    /// Required. The resource name of the parent zone:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of asset to return. The service may return fewer than
    /// this value. If unspecified, at most 10 assets will be returned. The
    /// maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. Page token received from a previous `ListAssets` call. Provide this to
    /// retrieve the subsequent page. When paginating, all other parameters
    /// provided to `ListAssets` must match the call that provided the page
    /// token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filter request.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Order by fields for the result.
    #[prost(string, tag="5")]
    pub order_by: ::prost::alloc::string::String,
}
/// List assets response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAssetsResponse {
    /// Asset under the given parent zone.
    #[prost(message, repeated, tag="1")]
    pub assets: ::prost::alloc::vec::Vec<Asset>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// List asset actions request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAssetActionsRequest {
    /// Required. The resource name of the parent asset:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/assets/{asset_id}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of actions to return. The service may return fewer than this
    /// value. If unspecified, at most 10 actions will be returned. The maximum
    /// value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. Page token received from a previous `ListAssetActions` call. Provide this
    /// to retrieve the subsequent page. When paginating, all other parameters
    /// provided to `ListAssetActions` must match the call that provided the page
    /// token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Get asset request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAssetRequest {
    /// Required. The resource name of the asset:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/assets/{asset_id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Represents the metadata of a long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag="1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag="3")]
    pub target: ::prost::alloc::string::String,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag="4")]
    pub verb: ::prost::alloc::string::String,
    /// Output only. Human-readable status of the operation, if any.
    #[prost(string, tag="5")]
    pub status_message: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have \[Operation.error][\] value with a \[google.rpc.Status.code][google.rpc.Status.code\] of 1,
    /// corresponding to `Code.CANCELLED`.
    #[prost(bool, tag="6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag="7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Create task request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTaskRequest {
    /// Required. The resource name of the parent lake:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Task identifier.
    #[prost(string, tag="2")]
    pub task_id: ::prost::alloc::string::String,
    /// Required. Task resource.
    #[prost(message, optional, tag="3")]
    pub task: ::core::option::Option<Task>,
    /// Optional. Only validate the request, but do not perform mutations.
    /// The default is false.
    #[prost(bool, tag="4")]
    pub validate_only: bool,
}
/// Update task request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTaskRequest {
    /// Required. Mask of fields to update.
    #[prost(message, optional, tag="1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Update description.
    /// Only fields specified in `update_mask` are updated.
    #[prost(message, optional, tag="2")]
    pub task: ::core::option::Option<Task>,
    /// Optional. Only validate the request, but do not perform mutations.
    /// The default is false.
    #[prost(bool, tag="3")]
    pub validate_only: bool,
}
/// Delete task request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTaskRequest {
    /// Required. The resource name of the task:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}/task/{task_id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// List tasks request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTasksRequest {
    /// Required. The resource name of the parent lake:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of tasks to return. The service may return fewer than this
    /// value. If unspecified, at most 10 tasks will be returned. The maximum
    /// value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. Page token received from a previous `ListZones` call. Provide this to
    /// retrieve the subsequent page. When paginating, all other parameters
    /// provided to `ListZones` must match the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filter request.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Order by fields for the result.
    #[prost(string, tag="5")]
    pub order_by: ::prost::alloc::string::String,
}
/// List tasks response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTasksResponse {
    /// Tasks under the given parent lake.
    #[prost(message, repeated, tag="1")]
    pub tasks: ::prost::alloc::vec::Vec<Task>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag="3")]
    pub unreachable_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Get task request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTaskRequest {
    /// Required. The resource name of the task:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}/tasks/{tasks_id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Get job request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJobRequest {
    /// Required. The resource name of the job:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}/tasks/{task_id}/jobs/{job_id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// List jobs request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobsRequest {
    /// Required. The resource name of the parent environment:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}/tasks/{task_id}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of jobs to return. The service may return fewer than
    /// this value. If unspecified, at most 10 jobs will be returned. The
    /// maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. Page token received from a previous `ListJobs` call. Provide this to
    /// retrieve the subsequent page. When paginating, all other parameters
    /// provided to `ListJobs` must match the call that provided the page
    /// token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// List jobs response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobsResponse {
    /// Jobs under a given task.
    #[prost(message, repeated, tag="1")]
    pub jobs: ::prost::alloc::vec::Vec<Job>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Cancel task jobs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelJobRequest {
    /// Required. The resource name of the job:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}/task/{task_id}/job/{job_id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Create environment request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEnvironmentRequest {
    /// Required. The resource name of the parent lake:
    /// `projects/{project_id}/locations/{location_id}/lakes/{lake_id}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Environment identifier.
    /// * Must contain only lowercase letters, numbers and hyphens.
    /// * Must start with a letter.
    /// * Must be between 1-63 characters.
    /// * Must end with a number or a letter.
    /// * Must be unique within the lake.
    #[prost(string, tag="2")]
    pub environment_id: ::prost::alloc::string::String,
    /// Required. Environment resource.
    #[prost(message, optional, tag="3")]
    pub environment: ::core::option::Option<Environment>,
    /// Optional. Only validate the request, but do not perform mutations.
    /// The default is false.
    #[prost(bool, tag="4")]
    pub validate_only: bool,
}
/// Update environment request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEnvironmentRequest {
    /// Required. Mask of fields to update.
    #[prost(message, optional, tag="1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Update description.
    /// Only fields specified in `update_mask` are updated.
    #[prost(message, optional, tag="2")]
    pub environment: ::core::option::Option<Environment>,
    /// Optional. Only validate the request, but do not perform mutations.
    /// The default is false.
    #[prost(bool, tag="3")]
    pub validate_only: bool,
}
/// Delete environment request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEnvironmentRequest {
    /// Required. The resource name of the environment:
    /// `projects/{project_id}/locations/{location_id}/lakes/{lake_id}/environments/{environment_id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// List environments request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEnvironmentsRequest {
    /// Required. The resource name of the parent lake:
    /// `projects/{project_id}/locations/{location_id}/lakes/{lake_id}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of environments to return. The service may return fewer than
    /// this value. If unspecified, at most 10 environments will be returned. The
    /// maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. Page token received from a previous `ListEnvironments` call. Provide this
    /// to retrieve the subsequent page. When paginating, all other parameters
    /// provided to `ListEnvironments` must match the call that provided the page
    /// token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filter request.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Order by fields for the result.
    #[prost(string, tag="5")]
    pub order_by: ::prost::alloc::string::String,
}
/// List environments response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEnvironmentsResponse {
    /// Environments under the given parent lake.
    #[prost(message, repeated, tag="1")]
    pub environments: ::prost::alloc::vec::Vec<Environment>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Get environment request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEnvironmentRequest {
    /// Required. The resource name of the environment:
    /// `projects/{project_id}/locations/{location_id}/lakes/{lake_id}/environments/{environment_id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// List sessions request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSessionsRequest {
    /// Required. The resource name of the parent environment:
    /// `projects/{project_number}/locations/{location_id}/lakes/{lake_id}/environment/{environment_id}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of sessions to return. The service may return fewer than
    /// this value. If unspecified, at most 10 sessions will be returned. The
    /// maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. Page token received from a previous `ListSessions` call. Provide this to
    /// retrieve the subsequent page. When paginating, all other parameters
    /// provided to `ListSessions` must match the call that provided the page
    /// token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filter request. The following `mode` filter is supported to return only the
    /// sessions belonging to the requester when the mode is USER and return
    /// sessions of all the users when the mode is ADMIN. When no filter is sent
    /// default to USER mode.
    /// NOTE: When the mode is ADMIN, the requester should have
    /// `dataplex.environments.listAllSessions` permission to list all sessions,
    /// in absence of the permission, the request fails.
    ///
    /// mode = ADMIN | USER
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
}
/// List sessions response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSessionsResponse {
    /// Sessions under a given environment.
    #[prost(message, repeated, tag="1")]
    pub sessions: ::prost::alloc::vec::Vec<Session>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod dataplex_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Dataplex service provides data lakes as a service. The primary resources
    /// offered by this service are Lakes, Zones and Assets which collectively allow
    /// a data administrator to organize, manage, secure and catalog data across
    /// their organization located across cloud projects in a variety of storage
    /// systems including Cloud Storage and BigQuery.
    #[derive(Debug, Clone)]
    pub struct DataplexServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DataplexServiceClient<T>
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
        ) -> DataplexServiceClient<InterceptedService<T, F>>
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
            DataplexServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a lake resource.
        pub async fn create_lake(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateLakeRequest>,
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
                "/google.cloud.dataplex.v1.DataplexService/CreateLake",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a lake resource.
        pub async fn update_lake(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateLakeRequest>,
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
                "/google.cloud.dataplex.v1.DataplexService/UpdateLake",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a lake resource. All zones within the lake must be deleted before
        /// the lake can be deleted.
        pub async fn delete_lake(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteLakeRequest>,
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
                "/google.cloud.dataplex.v1.DataplexService/DeleteLake",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists lake resources in a project and location.
        pub async fn list_lakes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLakesRequest>,
        ) -> Result<tonic::Response<super::ListLakesResponse>, tonic::Status> {
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
                "/google.cloud.dataplex.v1.DataplexService/ListLakes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves a lake resource.
        pub async fn get_lake(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLakeRequest>,
        ) -> Result<tonic::Response<super::Lake>, tonic::Status> {
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
                "/google.cloud.dataplex.v1.DataplexService/GetLake",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists action resources in a lake.
        pub async fn list_lake_actions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLakeActionsRequest>,
        ) -> Result<tonic::Response<super::ListActionsResponse>, tonic::Status> {
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
                "/google.cloud.dataplex.v1.DataplexService/ListLakeActions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a zone resource within a lake.
        pub async fn create_zone(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateZoneRequest>,
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
                "/google.cloud.dataplex.v1.DataplexService/CreateZone",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a zone resource.
        pub async fn update_zone(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateZoneRequest>,
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
                "/google.cloud.dataplex.v1.DataplexService/UpdateZone",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a zone resource. All assets within a zone must be deleted before
        /// the zone can be deleted.
        pub async fn delete_zone(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteZoneRequest>,
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
                "/google.cloud.dataplex.v1.DataplexService/DeleteZone",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists zone resources in a lake.
        pub async fn list_zones(
            &mut self,
            request: impl tonic::IntoRequest<super::ListZonesRequest>,
        ) -> Result<tonic::Response<super::ListZonesResponse>, tonic::Status> {
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
                "/google.cloud.dataplex.v1.DataplexService/ListZones",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves a zone resource.
        pub async fn get_zone(
            &mut self,
            request: impl tonic::IntoRequest<super::GetZoneRequest>,
        ) -> Result<tonic::Response<super::Zone>, tonic::Status> {
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
                "/google.cloud.dataplex.v1.DataplexService/GetZone",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists action resources in a zone.
        pub async fn list_zone_actions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListZoneActionsRequest>,
        ) -> Result<tonic::Response<super::ListActionsResponse>, tonic::Status> {
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
                "/google.cloud.dataplex.v1.DataplexService/ListZoneActions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates an asset resource.
        pub async fn create_asset(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAssetRequest>,
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
                "/google.cloud.dataplex.v1.DataplexService/CreateAsset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an asset resource.
        pub async fn update_asset(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAssetRequest>,
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
                "/google.cloud.dataplex.v1.DataplexService/UpdateAsset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes an asset resource. The referenced storage resource is detached
        /// (default) or deleted based on the associated Lifecycle policy.
        pub async fn delete_asset(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAssetRequest>,
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
                "/google.cloud.dataplex.v1.DataplexService/DeleteAsset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists asset resources in a zone.
        pub async fn list_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAssetsRequest>,
        ) -> Result<tonic::Response<super::ListAssetsResponse>, tonic::Status> {
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
                "/google.cloud.dataplex.v1.DataplexService/ListAssets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves an asset resource.
        pub async fn get_asset(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAssetRequest>,
        ) -> Result<tonic::Response<super::Asset>, tonic::Status> {
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
                "/google.cloud.dataplex.v1.DataplexService/GetAsset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists action resources in an asset.
        pub async fn list_asset_actions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAssetActionsRequest>,
        ) -> Result<tonic::Response<super::ListActionsResponse>, tonic::Status> {
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
                "/google.cloud.dataplex.v1.DataplexService/ListAssetActions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a task resource within a lake.
        pub async fn create_task(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTaskRequest>,
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
                "/google.cloud.dataplex.v1.DataplexService/CreateTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Update the task resource.
        pub async fn update_task(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTaskRequest>,
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
                "/google.cloud.dataplex.v1.DataplexService/UpdateTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Delete the task resource.
        pub async fn delete_task(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTaskRequest>,
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
                "/google.cloud.dataplex.v1.DataplexService/DeleteTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists tasks under the given lake.
        pub async fn list_tasks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTasksRequest>,
        ) -> Result<tonic::Response<super::ListTasksResponse>, tonic::Status> {
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
                "/google.cloud.dataplex.v1.DataplexService/ListTasks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get task resource.
        pub async fn get_task(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTaskRequest>,
        ) -> Result<tonic::Response<super::Task>, tonic::Status> {
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
                "/google.cloud.dataplex.v1.DataplexService/GetTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists Jobs under the given task.
        pub async fn list_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListJobsRequest>,
        ) -> Result<tonic::Response<super::ListJobsResponse>, tonic::Status> {
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
                "/google.cloud.dataplex.v1.DataplexService/ListJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get job resource.
        pub async fn get_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status> {
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
                "/google.cloud.dataplex.v1.DataplexService/GetJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Cancel jobs running for the task resource.
        pub async fn cancel_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelJobRequest>,
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
                "/google.cloud.dataplex.v1.DataplexService/CancelJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Create an environment resource.
        pub async fn create_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEnvironmentRequest>,
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
                "/google.cloud.dataplex.v1.DataplexService/CreateEnvironment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Update the environment resource.
        pub async fn update_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEnvironmentRequest>,
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
                "/google.cloud.dataplex.v1.DataplexService/UpdateEnvironment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Delete the environment resource. All the child resources must have been
        /// deleted before environment deletion can be initiated.
        pub async fn delete_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEnvironmentRequest>,
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
                "/google.cloud.dataplex.v1.DataplexService/DeleteEnvironment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists environments under the given lake.
        pub async fn list_environments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEnvironmentsRequest>,
        ) -> Result<tonic::Response<super::ListEnvironmentsResponse>, tonic::Status> {
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
                "/google.cloud.dataplex.v1.DataplexService/ListEnvironments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get environment resource.
        pub async fn get_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEnvironmentRequest>,
        ) -> Result<tonic::Response<super::Environment>, tonic::Status> {
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
                "/google.cloud.dataplex.v1.DataplexService/GetEnvironment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists session resources in an environment.
        pub async fn list_sessions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSessionsRequest>,
        ) -> Result<tonic::Response<super::ListSessionsResponse>, tonic::Status> {
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
                "/google.cloud.dataplex.v1.DataplexService/ListSessions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Create content request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateContentRequest {
    /// Required. The resource name of the parent lake:
    /// projects/{project_id}/locations/{location_id}/lakes/{lake_id}
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Content resource.
    #[prost(message, optional, tag="2")]
    pub content: ::core::option::Option<Content>,
    /// Optional. Only validate the request, but do not perform mutations.
    /// The default is false.
    #[prost(bool, tag="3")]
    pub validate_only: bool,
}
/// Update content request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateContentRequest {
    /// Required. Mask of fields to update.
    #[prost(message, optional, tag="1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Update description.
    /// Only fields specified in `update_mask` are updated.
    #[prost(message, optional, tag="2")]
    pub content: ::core::option::Option<Content>,
    /// Optional. Only validate the request, but do not perform mutations.
    /// The default is false.
    #[prost(bool, tag="3")]
    pub validate_only: bool,
}
/// Delete content request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteContentRequest {
    /// Required. The resource name of the content:
    /// projects/{project_id}/locations/{location_id}/lakes/{lake_id}/content/{content_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// List content request. Returns the BASIC Content view.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListContentRequest {
    /// Required. The resource name of the parent lake:
    /// projects/{project_id}/locations/{location_id}/lakes/{lake_id}
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of content to return. The service may return fewer than
    /// this value. If unspecified, at most 10 content will be returned. The
    /// maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. Page token received from a previous `ListContent` call. Provide this
    /// to retrieve the subsequent page. When paginating, all other parameters
    /// provided to `ListContent` must match the call that provided the page
    /// token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filter request. Filters are case-sensitive.
    /// The following formats are supported:
    ///
    /// labels.key1 = "value1"
    /// labels:key1
    /// type = "NOTEBOOK"
    /// type = "SQL_SCRIPT"
    ///
    /// These restrictions can be coinjoined with AND, OR and NOT conjunctions.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
}
/// List content response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListContentResponse {
    /// Content under the given parent lake.
    #[prost(message, repeated, tag="1")]
    pub content: ::prost::alloc::vec::Vec<Content>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Get content request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetContentRequest {
    /// Required. The resource name of the content:
    /// projects/{project_id}/locations/{location_id}/lakes/{lake_id}/content/{content_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Specify content view to make a partial request.
    #[prost(enumeration="get_content_request::ContentView", tag="2")]
    pub view: i32,
}
/// Nested message and enum types in `GetContentRequest`.
pub mod get_content_request {
    /// Specifies whether the request should return the full or the partial
    /// representation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ContentView {
        /// Content view not specified. Defaults to BASIC.
        /// The API will default to the BASIC view.
        Unspecified = 0,
        /// Will not return the `data_text` field.
        Basic = 1,
        /// Returns the complete proto.
        Full = 2,
    }
}
/// Generated client implementations.
pub mod content_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// ContentService manages Notebook and SQL Scripts for Dataplex.
    #[derive(Debug, Clone)]
    pub struct ContentServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ContentServiceClient<T>
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
        ) -> ContentServiceClient<InterceptedService<T, F>>
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
            ContentServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Create a content.
        pub async fn create_content(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateContentRequest>,
        ) -> Result<tonic::Response<super::Content>, tonic::Status> {
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
                "/google.cloud.dataplex.v1.ContentService/CreateContent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Update a content. Only supports full resource update.
        pub async fn update_content(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateContentRequest>,
        ) -> Result<tonic::Response<super::Content>, tonic::Status> {
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
                "/google.cloud.dataplex.v1.ContentService/UpdateContent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Delete a content.
        pub async fn delete_content(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteContentRequest>,
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
                "/google.cloud.dataplex.v1.ContentService/DeleteContent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get a content resource.
        pub async fn get_content(
            &mut self,
            request: impl tonic::IntoRequest<super::GetContentRequest>,
        ) -> Result<tonic::Response<super::Content>, tonic::Status> {
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
                "/google.cloud.dataplex.v1.ContentService/GetContent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the access control policy for a contentitem resource. A `NOT_FOUND`
        /// error is returned if the resource does not exist. An empty policy is
        /// returned if the resource exists but does not have a policy set on it.
        ///
        /// Caller must have Google IAM `dataplex.content.getIamPolicy` permission
        /// on the resource.
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.dataplex.v1.ContentService/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the access control policy on the specified contentitem resource.
        /// Replaces any existing policy.
        ///
        /// Caller must have Google IAM `dataplex.content.setIamPolicy` permission
        /// on the resource.
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.dataplex.v1.ContentService/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the caller's permissions on a resource.
        /// If the resource does not exist, an empty set of
        /// permissions is returned (a `NOT_FOUND` error is not returned).
        ///
        /// A caller is not required to have Google IAM permission to make this
        /// request.
        ///
        /// Note: This operation is designed to be used for building permission-aware
        /// UIs and command-line tools, not for authorization checking. This operation
        /// may "fail open" without warning.
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::iam::v1::TestIamPermissionsResponse,
            >,
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
                "/google.cloud.dataplex.v1.ContentService/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List content.
        pub async fn list_content(
            &mut self,
            request: impl tonic::IntoRequest<super::ListContentRequest>,
        ) -> Result<tonic::Response<super::ListContentResponse>, tonic::Status> {
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
                "/google.cloud.dataplex.v1.ContentService/ListContent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
