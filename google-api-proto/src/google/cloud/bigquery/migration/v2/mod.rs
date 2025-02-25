/// The translation config to capture necessary settings for a translation task
/// and subtask.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TranslationConfigDetails {
    /// The dialect of the input files.
    #[prost(message, optional, tag="3")]
    pub source_dialect: ::core::option::Option<Dialect>,
    /// The target dialect for the engine to translate the input to.
    #[prost(message, optional, tag="4")]
    pub target_dialect: ::core::option::Option<Dialect>,
    /// The default source environment values for the translation.
    #[prost(message, optional, tag="6")]
    pub source_env: ::core::option::Option<SourceEnv>,
    /// The chosen path where the source for input files will be found.
    #[prost(oneof="translation_config_details::SourceLocation", tags="1")]
    pub source_location: ::core::option::Option<translation_config_details::SourceLocation>,
    /// The chosen path where the destination for output files will be found.
    #[prost(oneof="translation_config_details::TargetLocation", tags="2")]
    pub target_location: ::core::option::Option<translation_config_details::TargetLocation>,
    /// The mapping of full SQL object names from their current state to the
    /// desired output.
    #[prost(oneof="translation_config_details::OutputNameMapping", tags="5")]
    pub output_name_mapping: ::core::option::Option<translation_config_details::OutputNameMapping>,
}
/// Nested message and enum types in `TranslationConfigDetails`.
pub mod translation_config_details {
    /// The chosen path where the source for input files will be found.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SourceLocation {
        /// The Cloud Storage path for a directory of files to translate in a task.
        #[prost(string, tag="1")]
        GcsSourcePath(::prost::alloc::string::String),
    }
    /// The chosen path where the destination for output files will be found.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TargetLocation {
        /// The Cloud Storage path to write back the corresponding input files to.
        #[prost(string, tag="2")]
        GcsTargetPath(::prost::alloc::string::String),
    }
    /// The mapping of full SQL object names from their current state to the
    /// desired output.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OutputNameMapping {
        /// The mapping of objects to their desired output names in list form.
        #[prost(message, tag="5")]
        NameMappingList(super::ObjectNameMappingList),
    }
}
/// The possible dialect options for translation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dialect {
    /// The possible dialect options that this message represents.
    #[prost(oneof="dialect::DialectValue", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14")]
    pub dialect_value: ::core::option::Option<dialect::DialectValue>,
}
/// Nested message and enum types in `Dialect`.
pub mod dialect {
    /// The possible dialect options that this message represents.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DialectValue {
        /// The BigQuery dialect
        #[prost(message, tag="1")]
        BigqueryDialect(super::BigQueryDialect),
        /// The HiveQL dialect
        #[prost(message, tag="2")]
        HiveqlDialect(super::HiveQlDialect),
        /// The Redshift dialect
        #[prost(message, tag="3")]
        RedshiftDialect(super::RedshiftDialect),
        /// The Teradata dialect
        #[prost(message, tag="4")]
        TeradataDialect(super::TeradataDialect),
        /// The Oracle dialect
        #[prost(message, tag="5")]
        OracleDialect(super::OracleDialect),
        /// The SparkSQL dialect
        #[prost(message, tag="6")]
        SparksqlDialect(super::SparkSqlDialect),
        /// The Snowflake dialect
        #[prost(message, tag="7")]
        SnowflakeDialect(super::SnowflakeDialect),
        /// The Netezza dialect
        #[prost(message, tag="8")]
        NetezzaDialect(super::NetezzaDialect),
        /// The Azure Synapse dialect
        #[prost(message, tag="9")]
        AzureSynapseDialect(super::AzureSynapseDialect),
        /// The Vertica dialect
        #[prost(message, tag="10")]
        VerticaDialect(super::VerticaDialect),
        /// The SQL Server dialect
        #[prost(message, tag="11")]
        SqlServerDialect(super::SqlServerDialect),
        /// The Postgresql dialect
        #[prost(message, tag="12")]
        PostgresqlDialect(super::PostgresqlDialect),
        /// The Presto dialect
        #[prost(message, tag="13")]
        PrestoDialect(super::PrestoDialect),
        /// The MySQL dialect
        #[prost(message, tag="14")]
        MysqlDialect(super::MySqlDialect),
    }
}
/// The dialect definition for BigQuery.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryDialect {
}
/// The dialect definition for HiveQL.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HiveQlDialect {
}
/// The dialect definition for Redshift.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedshiftDialect {
}
/// The dialect definition for Teradata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TeradataDialect {
    /// Which Teradata sub-dialect mode the user specifies.
    #[prost(enumeration="teradata_dialect::Mode", tag="1")]
    pub mode: i32,
}
/// Nested message and enum types in `TeradataDialect`.
pub mod teradata_dialect {
    /// The sub-dialect options for Teradata.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Mode {
        /// Unspecified mode.
        Unspecified = 0,
        /// Teradata SQL mode.
        Sql = 1,
        /// BTEQ mode (which includes SQL).
        Bteq = 2,
    }
}
/// The dialect definition for Oracle.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OracleDialect {
}
/// The dialect definition for SparkSQL.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SparkSqlDialect {
}
/// The dialect definition for Snowflake.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnowflakeDialect {
}
/// The dialect definition for Netezza.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetezzaDialect {
}
/// The dialect definition for Azure Synapse.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureSynapseDialect {
}
/// The dialect definition for Vertica.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerticaDialect {
}
/// The dialect definition for SQL Server.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlServerDialect {
}
/// The dialect definition for Postgresql.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostgresqlDialect {
}
/// The dialect definition for Presto.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrestoDialect {
}
/// The dialect definition for MySQL.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MySqlDialect {
}
/// Represents a map of name mappings using a list of key:value proto messages of
/// existing name to desired output name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectNameMappingList {
    /// The elements of the object name map.
    #[prost(message, repeated, tag="1")]
    pub name_map: ::prost::alloc::vec::Vec<ObjectNameMapping>,
}
/// Represents a key-value pair of NameMappingKey to NameMappingValue to
/// represent the mapping of SQL names from the input value to desired output.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectNameMapping {
    /// The name of the object in source that is being mapped.
    #[prost(message, optional, tag="1")]
    pub source: ::core::option::Option<NameMappingKey>,
    /// The desired target name of the object that is being mapped.
    #[prost(message, optional, tag="2")]
    pub target: ::core::option::Option<NameMappingValue>,
}
/// The potential components of a full name mapping that will be mapped
/// during translation in the source data warehouse.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NameMappingKey {
    /// The type of object that is being mapped.
    #[prost(enumeration="name_mapping_key::Type", tag="1")]
    pub r#type: i32,
    /// The database name (BigQuery project ID equivalent in the source data
    /// warehouse).
    #[prost(string, tag="2")]
    pub database: ::prost::alloc::string::String,
    /// The schema name (BigQuery dataset equivalent in the source data warehouse).
    #[prost(string, tag="3")]
    pub schema: ::prost::alloc::string::String,
    /// The relation name (BigQuery table or view equivalent in the source data
    /// warehouse).
    #[prost(string, tag="4")]
    pub relation: ::prost::alloc::string::String,
    /// The attribute name (BigQuery column equivalent in the source data
    /// warehouse).
    #[prost(string, tag="5")]
    pub attribute: ::prost::alloc::string::String,
}
/// Nested message and enum types in `NameMappingKey`.
pub mod name_mapping_key {
    /// The type of the object that is being mapped.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Unspecified name mapping type.
        Unspecified = 0,
        /// The object being mapped is a database.
        Database = 1,
        /// The object being mapped is a schema.
        Schema = 2,
        /// The object being mapped is a relation.
        Relation = 3,
        /// The object being mapped is an attribute.
        Attribute = 4,
        /// The object being mapped is a relation alias.
        RelationAlias = 5,
        /// The object being mapped is a an attribute alias.
        AttributeAlias = 6,
        /// The object being mapped is a function.
        Function = 7,
    }
}
/// The potential components of a full name mapping that will be mapped
/// during translation in the target data warehouse.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NameMappingValue {
    /// The database name (BigQuery project ID equivalent in the target data
    /// warehouse).
    #[prost(string, tag="1")]
    pub database: ::prost::alloc::string::String,
    /// The schema name (BigQuery dataset equivalent in the target data warehouse).
    #[prost(string, tag="2")]
    pub schema: ::prost::alloc::string::String,
    /// The relation name (BigQuery table or view equivalent in the target data
    /// warehouse).
    #[prost(string, tag="3")]
    pub relation: ::prost::alloc::string::String,
    /// The attribute name (BigQuery column equivalent in the target data
    /// warehouse).
    #[prost(string, tag="4")]
    pub attribute: ::prost::alloc::string::String,
}
/// Represents the default source environment values for the translation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceEnv {
    /// The default database name to fully qualify SQL objects when their database
    /// name is missing.
    #[prost(string, tag="1")]
    pub default_database: ::prost::alloc::string::String,
    /// The schema search path. When SQL objects are missing schema name,
    /// translation engine will search through this list to find the value.
    #[prost(string, repeated, tag="2")]
    pub schema_search_path: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Provides details for errors and the corresponding resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceErrorDetail {
    /// Required. Information about the resource where the error is located.
    #[prost(message, optional, tag="1")]
    pub resource_info: ::core::option::Option<super::super::super::super::rpc::ResourceInfo>,
    /// Required. The error details for the resource.
    #[prost(message, repeated, tag="2")]
    pub error_details: ::prost::alloc::vec::Vec<ErrorDetail>,
    /// Required. How many errors there are in total for the resource. Truncation can be
    /// indicated by having an `error_count` that is higher than the size of
    /// `error_details`.
    #[prost(int32, tag="3")]
    pub error_count: i32,
}
/// Provides details for errors, e.g. issues that where encountered when
/// processing a subtask.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorDetail {
    /// Optional. The exact location within the resource (if applicable).
    #[prost(message, optional, tag="1")]
    pub location: ::core::option::Option<ErrorLocation>,
    /// Required. Describes the cause of the error with structured detail.
    #[prost(message, optional, tag="2")]
    pub error_info: ::core::option::Option<super::super::super::super::rpc::ErrorInfo>,
}
/// Holds information about where the error is located.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorLocation {
    /// Optional. If applicable, denotes the line where the error occurred. A zero value
    /// means that there is no line information.
    #[prost(int32, tag="1")]
    pub line: i32,
    /// Optional. If applicable, denotes the column where the error occurred. A zero value
    /// means that there is no columns information.
    #[prost(int32, tag="2")]
    pub column: i32,
}
/// The metrics object for a SubTask.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeSeries {
    /// Required. The name of the metric.
    ///
    /// If the metric is not known by the service yet, it will be auto-created.
    #[prost(string, tag="1")]
    pub metric: ::prost::alloc::string::String,
    /// Required. The value type of the time series.
    #[prost(enumeration="super::super::super::super::api::metric_descriptor::ValueType", tag="2")]
    pub value_type: i32,
    /// Optional. The metric kind of the time series.
    ///
    /// If present, it must be the same as the metric kind of the associated
    /// metric. If the associated metric's descriptor must be auto-created, then
    /// this field specifies the metric kind of the new descriptor and must be
    /// either `GAUGE` (the default) or `CUMULATIVE`.
    #[prost(enumeration="super::super::super::super::api::metric_descriptor::MetricKind", tag="3")]
    pub metric_kind: i32,
    /// Required. The data points of this time series. When listing time series, points are
    /// returned in reverse time order.
    ///
    /// When creating a time series, this field must contain exactly one point and
    /// the point's type must be the same as the value type of the associated
    /// metric. If the associated metric's descriptor must be auto-created, then
    /// the value type of the descriptor is determined by the point's type, which
    /// must be `BOOL`, `INT64`, `DOUBLE`, or `DISTRIBUTION`.
    #[prost(message, repeated, tag="4")]
    pub points: ::prost::alloc::vec::Vec<Point>,
}
/// A single data point in a time series.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Point {
    /// The time interval to which the data point applies.  For `GAUGE` metrics,
    /// the start time does not need to be supplied, but if it is supplied, it must
    /// equal the end time.  For `DELTA` metrics, the start and end time should
    /// specify a non-zero interval, with subsequent points specifying contiguous
    /// and non-overlapping intervals.  For `CUMULATIVE` metrics, the start and end
    /// time should specify a non-zero interval, with subsequent points specifying
    /// the same start time and increasing end times, until an event resets the
    /// cumulative value to zero and sets a new start time for the following
    /// points.
    #[prost(message, optional, tag="1")]
    pub interval: ::core::option::Option<TimeInterval>,
    /// The value of the data point.
    #[prost(message, optional, tag="2")]
    pub value: ::core::option::Option<TypedValue>,
}
/// A time interval extending just after a start time through an end time.
/// If the start time is the same as the end time, then the interval
/// represents a single point in time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeInterval {
    /// Optional. The beginning of the time interval.  The default value
    /// for the start time is the end time. The start time must not be
    /// later than the end time.
    #[prost(message, optional, tag="1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. The end of the time interval.
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// A single strongly-typed value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypedValue {
    /// The typed value field.
    #[prost(oneof="typed_value::Value", tags="1, 2, 3, 4, 5")]
    pub value: ::core::option::Option<typed_value::Value>,
}
/// Nested message and enum types in `TypedValue`.
pub mod typed_value {
    /// The typed value field.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// A Boolean value: `true` or `false`.
        #[prost(bool, tag="1")]
        BoolValue(bool),
        /// A 64-bit integer. Its range is approximately `+/-9.2x10^18`.
        #[prost(int64, tag="2")]
        Int64Value(i64),
        /// A 64-bit double-precision floating-point number. Its magnitude
        /// is approximately `+/-10^(+/-300)` and it has 16 significant digits of
        /// precision.
        #[prost(double, tag="3")]
        DoubleValue(f64),
        /// A variable-length string value.
        #[prost(string, tag="4")]
        StringValue(::prost::alloc::string::String),
        /// A distribution value.
        #[prost(message, tag="5")]
        DistributionValue(super::super::super::super::super::api::Distribution),
    }
}
/// A migration workflow which specifies what needs to be done for an EDW
/// migration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrationWorkflow {
    /// Output only. Immutable. The unique identifier for the migration workflow.
    /// The ID is server-generated.
    ///
    /// Example: `projects/123/locations/us/workflows/345`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The display name of the workflow. This can be set to give a workflow
    /// a descriptive name. There is no guarantee or enforcement of uniqueness.
    #[prost(string, tag="6")]
    pub display_name: ::prost::alloc::string::String,
    /// The tasks in a workflow in a named map. The name (i.e. key) has no
    /// meaning and is merely a convenient way to address a specific task
    /// in a workflow.
    #[prost(btree_map="string, message", tag="2")]
    pub tasks: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, MigrationTask>,
    /// Output only. That status of the workflow.
    #[prost(enumeration="migration_workflow::State", tag="3")]
    pub state: i32,
    /// Time when the workflow was created.
    #[prost(message, optional, tag="4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Time when the workflow was last updated.
    #[prost(message, optional, tag="5")]
    pub last_update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `MigrationWorkflow`.
pub mod migration_workflow {
    /// Possible migration workflow states.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Workflow state is unspecified.
        Unspecified = 0,
        /// Workflow is in draft status, i.e. tasks are not yet eligible for
        /// execution.
        Draft = 1,
        /// Workflow is running (i.e. tasks are eligible for execution).
        Running = 2,
        /// Workflow is paused. Tasks currently in progress may continue, but no
        /// further tasks will be scheduled.
        Paused = 3,
        /// Workflow is complete. There should not be any task in a non-terminal
        /// state, but if they are (e.g. forced termination), they will not be
        /// scheduled.
        Completed = 4,
    }
}
/// A single task for a migration which has details about the configuration of
/// the task.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrationTask {
    /// Output only. Immutable. The unique identifier for the migration task. The
    /// ID is server-generated.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The type of the task. This must be one of the supported task types:
    /// Translation_Teradata2BQ, Translation_Redshift2BQ, Translation_Bteq2BQ,
    /// Translation_Oracle2BQ, Translation_HiveQL2BQ, Translation_SparkSQL2BQ,
    /// Translation_Snowflake2BQ, Translation_Netezza2BQ,
    /// Translation_AzureSynapse2BQ, Translation_Vertica2BQ,
    /// Translation_SQLServer2BQ, Translation_Presto2BQ, Translation_MySQL2BQ.
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    /// Output only. The current state of the task.
    #[prost(enumeration="migration_task::State", tag="4")]
    pub state: i32,
    /// Output only. An explanation that may be populated when the task is in
    /// FAILED state.
    #[prost(message, optional, tag="5")]
    pub processing_error: ::core::option::Option<super::super::super::super::rpc::ErrorInfo>,
    /// Time when the task was created.
    #[prost(message, optional, tag="6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Time when the task was last updated.
    #[prost(message, optional, tag="7")]
    pub last_update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The details of the task.
    #[prost(oneof="migration_task::TaskDetails", tags="14")]
    pub task_details: ::core::option::Option<migration_task::TaskDetails>,
}
/// Nested message and enum types in `MigrationTask`.
pub mod migration_task {
    /// Possible states of a migration task.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The state is unspecified.
        Unspecified = 0,
        /// The task is waiting for orchestration.
        Pending = 1,
        /// The task is assigned to an orchestrator.
        Orchestrating = 2,
        /// The task is running, i.e. its subtasks are ready for execution.
        Running = 3,
        /// Tha task is paused. Assigned subtasks can continue, but no new subtasks
        /// will be scheduled.
        Paused = 4,
        /// The task finished successfully.
        Succeeded = 5,
        /// The task finished unsuccessfully.
        Failed = 6,
    }
    /// The details of the task.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TaskDetails {
        /// Task configuration for Batch SQL Translation.
        #[prost(message, tag="14")]
        TranslationConfigDetails(super::TranslationConfigDetails),
    }
}
/// A subtask for a migration which carries details about the configuration of
/// the subtask. The content of the details should not matter to the end user,
/// but is a contract between the subtask creator and subtask worker.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrationSubtask {
    /// Output only. Immutable. The resource name for the migration subtask. The ID
    /// is server-generated.
    ///
    /// Example: `projects/123/locations/us/workflows/345/subtasks/678`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The unique ID of the task to which this subtask belongs.
    #[prost(string, tag="2")]
    pub task_id: ::prost::alloc::string::String,
    /// The type of the Subtask. The migration service does not check whether this
    /// is a known type. It is up to the task creator (i.e. orchestrator or worker)
    /// to ensure it only creates subtasks for which there are compatible workers
    /// polling for Subtasks.
    #[prost(string, tag="3")]
    pub r#type: ::prost::alloc::string::String,
    /// Output only. The current state of the subtask.
    #[prost(enumeration="migration_subtask::State", tag="5")]
    pub state: i32,
    /// Output only. An explanation that may be populated when the task is in
    /// FAILED state.
    #[prost(message, optional, tag="6")]
    pub processing_error: ::core::option::Option<super::super::super::super::rpc::ErrorInfo>,
    /// Output only. Provides details to errors and issues encountered while
    /// processing the subtask. Presence of error details does not mean that the
    /// subtask failed.
    #[prost(message, repeated, tag="12")]
    pub resource_error_details: ::prost::alloc::vec::Vec<ResourceErrorDetail>,
    /// The number or resources with errors. Note: This is not the total
    /// number of errors as each resource can have more than one error.
    /// This is used to indicate truncation by having a `resource_error_count`
    /// that is higher than the size of `resource_error_details`.
    #[prost(int32, tag="13")]
    pub resource_error_count: i32,
    /// Time when the subtask was created.
    #[prost(message, optional, tag="7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Time when the subtask was last updated.
    #[prost(message, optional, tag="8")]
    pub last_update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The metrics for the subtask.
    #[prost(message, repeated, tag="11")]
    pub metrics: ::prost::alloc::vec::Vec<TimeSeries>,
}
/// Nested message and enum types in `MigrationSubtask`.
pub mod migration_subtask {
    /// Possible states of a migration subtask.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The state is unspecified.
        Unspecified = 0,
        /// The subtask is ready, i.e. it is ready for execution.
        Active = 1,
        /// The subtask is running, i.e. it is assigned to a worker for execution.
        Running = 2,
        /// The subtask finished successfully.
        Succeeded = 3,
        /// The subtask finished unsuccessfully.
        Failed = 4,
        /// The subtask is paused, i.e., it will not be scheduled. If it was already
        /// assigned,it might still finish but no new lease renewals will be granted.
        Paused = 5,
    }
}
/// Request to create a migration workflow resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMigrationWorkflowRequest {
    /// Required. The name of the project to which this migration workflow belongs.
    /// Example: `projects/foo/locations/bar`
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The migration workflow to create.
    #[prost(message, optional, tag="2")]
    pub migration_workflow: ::core::option::Option<MigrationWorkflow>,
}
/// A request to get a previously created migration workflow.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMigrationWorkflowRequest {
    /// Required. The unique identifier for the migration workflow.
    /// Example: `projects/123/locations/us/workflows/1234`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The list of fields to be retrieved.
    #[prost(message, optional, tag="2")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// A request to list previously created migration workflows.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMigrationWorkflowsRequest {
    /// Required. The project and location of the migration workflows to list.
    /// Example: `projects/123/locations/us`
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The list of fields to be retrieved.
    #[prost(message, optional, tag="2")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The maximum number of migration workflows to return. The service may return
    /// fewer than this number.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// A page token, received from previous `ListMigrationWorkflows` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListMigrationWorkflows`
    /// must match the call that provided the page token.
    #[prost(string, tag="4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response object for a `ListMigrationWorkflows` call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMigrationWorkflowsResponse {
    /// The migration workflows for the specified project / location.
    #[prost(message, repeated, tag="1")]
    pub migration_workflows: ::prost::alloc::vec::Vec<MigrationWorkflow>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A request to delete a previously created migration workflow.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMigrationWorkflowRequest {
    /// Required. The unique identifier for the migration workflow.
    /// Example: `projects/123/locations/us/workflows/1234`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to start a previously created migration workflow.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartMigrationWorkflowRequest {
    /// Required. The unique identifier for the migration workflow.
    /// Example: `projects/123/locations/us/workflows/1234`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to get a previously created migration subtasks.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMigrationSubtaskRequest {
    /// Required. The unique identifier for the migration subtask.
    /// Example: `projects/123/locations/us/workflows/1234/subtasks/543`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The list of fields to be retrieved.
    #[prost(message, optional, tag="2")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// A request to list previously created migration subtasks.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMigrationSubtasksRequest {
    /// Required. The migration task of the subtasks to list.
    /// Example: `projects/123/locations/us/workflows/1234`
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The list of fields to be retrieved.
    #[prost(message, optional, tag="2")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. The maximum number of migration tasks to return. The service may return
    /// fewer than this number.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// Optional. A page token, received from previous `ListMigrationSubtasks` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListMigrationSubtasks`
    /// must match the call that provided the page token.
    #[prost(string, tag="4")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The filter to apply. This can be used to get the subtasks of a specific
    /// tasks in a workflow, e.g. `migration_task = "ab012"` where `"ab012"` is the
    /// task ID (not the name in the named map).
    #[prost(string, tag="5")]
    pub filter: ::prost::alloc::string::String,
}
/// Response object for a `ListMigrationSubtasks` call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMigrationSubtasksResponse {
    /// The migration subtasks for the specified task.
    #[prost(message, repeated, tag="1")]
    pub migration_subtasks: ::prost::alloc::vec::Vec<MigrationSubtask>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod migration_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Service to handle EDW migrations.
    #[derive(Debug, Clone)]
    pub struct MigrationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> MigrationServiceClient<T>
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
        ) -> MigrationServiceClient<InterceptedService<T, F>>
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
            MigrationServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a migration workflow.
        pub async fn create_migration_workflow(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateMigrationWorkflowRequest>,
        ) -> Result<tonic::Response<super::MigrationWorkflow>, tonic::Status> {
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
                "/google.cloud.bigquery.migration.v2.MigrationService/CreateMigrationWorkflow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a previously created migration workflow.
        pub async fn get_migration_workflow(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMigrationWorkflowRequest>,
        ) -> Result<tonic::Response<super::MigrationWorkflow>, tonic::Status> {
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
                "/google.cloud.bigquery.migration.v2.MigrationService/GetMigrationWorkflow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists previously created migration workflow.
        pub async fn list_migration_workflows(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMigrationWorkflowsRequest>,
        ) -> Result<
            tonic::Response<super::ListMigrationWorkflowsResponse>,
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
                "/google.cloud.bigquery.migration.v2.MigrationService/ListMigrationWorkflows",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a migration workflow by name.
        pub async fn delete_migration_workflow(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteMigrationWorkflowRequest>,
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
                "/google.cloud.bigquery.migration.v2.MigrationService/DeleteMigrationWorkflow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Starts a previously created migration workflow. I.e., the state transitions
        /// from DRAFT to RUNNING. This is a no-op if the state is already RUNNING.
        /// An error will be signaled if the state is anything other than DRAFT or
        /// RUNNING.
        pub async fn start_migration_workflow(
            &mut self,
            request: impl tonic::IntoRequest<super::StartMigrationWorkflowRequest>,
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
                "/google.cloud.bigquery.migration.v2.MigrationService/StartMigrationWorkflow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a previously created migration subtask.
        pub async fn get_migration_subtask(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMigrationSubtaskRequest>,
        ) -> Result<tonic::Response<super::MigrationSubtask>, tonic::Status> {
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
                "/google.cloud.bigquery.migration.v2.MigrationService/GetMigrationSubtask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists previously created migration subtasks.
        pub async fn list_migration_subtasks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMigrationSubtasksRequest>,
        ) -> Result<
            tonic::Response<super::ListMigrationSubtasksResponse>,
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
                "/google.cloud.bigquery.migration.v2.MigrationService/ListMigrationSubtasks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
