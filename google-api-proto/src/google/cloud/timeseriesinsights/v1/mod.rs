/// Mapping of BigQuery columns to timestamp, group_id and dimensions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigqueryMapping {
    /// The column which should be used as the event timestamps. If not specified
    /// 'Timestamp' is used by default. The column may have TIMESTAMP or INT64
    /// type (the latter is interpreted as microseconds since the Unix epoch).
    #[prost(string, tag="1")]
    pub timestamp_column: ::prost::alloc::string::String,
    /// The column which should be used as the group ID (grouping events into
    /// sessions). If not specified 'GroupId' is used by default, if the input
    /// table does not have such a column, random unique group IDs are
    /// generated automatically (different group ID per input row).
    #[prost(string, tag="2")]
    pub group_id_column: ::prost::alloc::string::String,
    /// The list of columns that should be translated to dimensions. If empty,
    /// all columns are translated to dimensions. The timestamp and group_id
    /// columns should not be listed here again. Columns are expected to have
    /// primitive types (STRING, INT64, FLOAT64 or NUMERIC).
    #[prost(string, repeated, tag="3")]
    pub dimension_column: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A data source consists of multiple \[Event\] stored on
/// Cloud Storage.  Each Event should be in JSON format, with one Event
/// per line, also known as JSON Lines format.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataSource {
    /// Data source URI.
    ///
    /// 1) Google Cloud Storage files (JSON) are defined in the following form.
    /// `gs://bucket_name/object_name`. For more information on Cloud Storage URIs,
    /// please see <https://cloud.google.com/storage/docs/reference-uris.>
    #[prost(string, tag="1")]
    pub uri: ::prost::alloc::string::String,
    /// For BigQuery inputs defines the columns that should be used for dimensions
    /// (including time and group ID).
    #[prost(message, optional, tag="2")]
    pub bq_mapping: ::core::option::Option<BigqueryMapping>,
}
/// A collection of data sources sent for processing.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataSet {
    /// The dataset name, which will be used for querying, status and unload
    /// requests. This must be unique within a project.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// [Data dimension
    /// names]\[google.cloud.timeseriesinsights.v1.EventDimension.name\] allowed for
    /// this DataSet.
    ///
    /// If left empty, all dimension names are included. This field works as a
    /// filter to avoid regenerating the data.
    #[prost(string, repeated, tag="2")]
    pub data_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Input data.
    #[prost(message, repeated, tag="3")]
    pub data_sources: ::prost::alloc::vec::Vec<DataSource>,
    /// Dataset state in the system.
    #[prost(enumeration="data_set::State", tag="4")]
    pub state: i32,
    /// Dataset processing status.
    #[prost(message, optional, tag="5")]
    pub status: ::core::option::Option<super::super::super::rpc::Status>,
    /// Periodically we discard dataset \[Event\] that have timestamps older than
    /// 'ttl'.  Omitting this field or a zero value means no events are discarded.
    #[prost(message, optional, tag="6")]
    pub ttl: ::core::option::Option<::prost_types::Duration>,
}
/// Nested message and enum types in `DataSet`.
pub mod data_set {
    /// DataSet state.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified / undefined state.
        Unspecified = 0,
        /// Dataset is unknown to the system; we have never seen this dataset before
        /// or we have seen this dataset but have fully GC-ed it.
        Unknown = 1,
        /// Dataset processing is pending.
        Pending = 2,
        /// Dataset is loading.
        Loading = 3,
        /// Dataset is loaded and can be queried.
        Loaded = 4,
        /// Dataset is unloading.
        Unloading = 5,
        /// Dataset is unloaded and is removed from the system.
        Unloaded = 6,
        /// Dataset processing failed.
        Failed = 7,
    }
}
/// Represents an event dimension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventDimension {
    /// Dimension name.
    ///
    /// **NOTE**: EventDimension names must be composed of alphanumeric characters
    /// only, and are case insensitive. Unicode characters are *not* supported. The
    /// underscore '_' is also allowed.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Dimension value.
    ///
    /// **NOTE**: All entries of the dimension `name` must have the same `value`
    /// type.
    #[prost(oneof="event_dimension::Value", tags="2, 3, 4, 5")]
    pub value: ::core::option::Option<event_dimension::Value>,
}
/// Nested message and enum types in `EventDimension`.
pub mod event_dimension {
    /// Dimension value.
    ///
    /// **NOTE**: All entries of the dimension `name` must have the same `value`
    /// type.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// String representation.
        ///
        /// **NOTE**: String values are case insensitive. Unicode characters are
        /// supported.
        #[prost(string, tag="2")]
        StringVal(::prost::alloc::string::String),
        /// Long representation.
        #[prost(int64, tag="3")]
        LongVal(i64),
        /// Bool representation.
        #[prost(bool, tag="4")]
        BoolVal(bool),
        /// Double representation.
        #[prost(double, tag="5")]
        DoubleVal(f64),
    }
}
/// Represents an entry in a data source.
///
/// Each Event has:
///
/// * A timestamp at which the event occurs.
/// * One or multiple dimensions.
/// * Optionally, a group ID that allows clients to group logically related
///   events (for example, all events representing payments transactions done by
///   a user in a day have the same group ID).  If a group ID is not provided, an
///   internal one will be generated based on the content and `eventTime`.
///
/// **NOTE**:
///
/// * Internally, we discretize time in equal-sized chunks and we assume an
///   event has a 0
///   \[TimeseriesPoint.value][google.cloud.timeseriesinsights.v1.TimeseriesPoint.value\]
///   in a chunk that does not contain any occurrences of an event in the input.
/// * The number of Events with the same group ID should be limited.
/// * Group ID *cannot* be queried.
/// * Group ID does *not* correspond to a user ID or the like. If a user ID is of
///   interest to be queried, use a user ID `dimension` instead.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// Event dimensions.
    #[prost(message, repeated, tag="1")]
    pub dimensions: ::prost::alloc::vec::Vec<EventDimension>,
    /// Event group ID.
    ///
    /// **NOTE**: JSON encoding should use a string to hold a 64-bit integer value,
    /// because a native JSON number holds only 53 binary bits for an integer.
    #[prost(int64, tag="2")]
    pub group_id: i64,
    /// Event timestamp.
    #[prost(message, optional, tag="3")]
    pub event_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Appends events to an existing DataSet.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppendEventsRequest {
    /// Events to be appended.
    ///
    /// **NOTE**:
    ///
    /// 0. The \[DataSet\] must be shown in a `LOADED` state
    ///    in the results of [list method]\[ListDataSet\]; otherwise, all events from
    ///    the append request will be dropped, and a `NOT_FOUND` status will be
    ///    returned.
    /// 0. All events in a single request must have the same
    ///    \[groupId][google.cloud.timeseriesinsights.v1.Event.group_id\] if set;
    ///    otherwise, an `INVALID_ARGUMENT` status will be returned.
    /// 0. If \[groupId][google.cloud.timeseriesinsights.v1.Event.group_id\] is not
    /// set (or 0), there
    ///    should be only 1 event; otherwise, an `INVALID_ARGUMENT` status will be
    ///    returned.
    /// 0. The events must be newer than the current time minus
    ///    [DataSet TTL]\[google.cloud.timeseriesinsights.v1.DataSet.ttl\] or they
    ///    will be dropped.
    #[prost(message, repeated, tag="1")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    /// Required. The DataSet to which we want to append to in the format of
    /// "projects/{project}/datasets/{dataset}"
    #[prost(string, tag="2")]
    pub dataset: ::prost::alloc::string::String,
}
/// Response for an AppendEvents RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppendEventsResponse {
    /// Dropped events; empty if all events are successfully added.
    #[prost(message, repeated, tag="1")]
    pub dropped_events: ::prost::alloc::vec::Vec<Event>,
}
/// Create a DataSet request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDataSetRequest {
    /// Required. Client project name which will own this DataSet in the format of
    /// 'projects/{project}'.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Data set to be loaded.
    #[prost(message, optional, tag="2")]
    pub dataset: ::core::option::Option<DataSet>,
}
/// Unload DataSet request from the serving system.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDataSetRequest {
    /// Required. Data set name in the format of
    /// "projects/{project}/datasets/{dataset}"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// List the DataSets created by the current project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataSetsRequest {
    /// Required. Project owning the DataSet in the format of "projects/{project}".
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Number of results to return in the list.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Token to provide to skip to a particular spot in the list.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Created DataSets list response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataSetsResponse {
    /// The list of created DataSets.
    #[prost(message, repeated, tag="1")]
    pub datasets: ::prost::alloc::vec::Vec<DataSet>,
    /// Token to receive the next page of results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A categorical dimension fixed to a certain value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PinnedDimension {
    /// The name of the dimension for which we are fixing its value.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Dimension value.
    ///
    /// **NOTE**: The `value` type must match that in the data with the same
    /// `dimension` as name.
    #[prost(oneof="pinned_dimension::Value", tags="2, 3")]
    pub value: ::core::option::Option<pinned_dimension::Value>,
}
/// Nested message and enum types in `PinnedDimension`.
pub mod pinned_dimension {
    /// Dimension value.
    ///
    /// **NOTE**: The `value` type must match that in the data with the same
    /// `dimension` as name.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// A string value. This can be used for
        /// \[dimensions][google.cloud.timeseriesinsights.v1.EventDimension\] which
        /// have their value field set to
        /// \[string_val][google.cloud.timeseriesinsights.v1.EventDimension.string_val\].
        #[prost(string, tag="2")]
        StringVal(::prost::alloc::string::String),
        /// A bool value. This can be used for
        /// \[dimensions][google.cloud.timeseriesinsights.v1.EventDimension\] which
        /// have their value field set to
        /// \[bool_val][google.cloud.timeseriesinsights.v1.EventDimension.bool_val\].
        #[prost(bool, tag="3")]
        BoolVal(bool),
    }
}
/// Parameters that control the sensitivity and other options for the time series
/// forecast.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForecastParams {
    /// Optional. Penalize variations between the actual and forecasted values
    /// smaller than this. See the \[anomalyScore\](EvaluatedSlice.anomaly_score)
    /// formula for details on how this parameter impacts the score.
    ///
    /// Intuitively, anomaly scores summarize how statistically significant the
    /// change between the actual and forecasted value is compared with what we
    /// expect the change to be (see
    /// \[expectedDeviation\](EvaluatedSlice.expected_deviation)). However, in
    /// practice, depending on the application, changes smaller than certain
    /// absolute values, while statistically significant, may not be important.
    ///
    /// This parameter allows us to penalize such low absolute value changes.
    ///
    /// Must be in the (0.0, inf) range.
    ///
    /// If unspecified, it defaults to 0.000001.
    #[prost(double, optional, tag="12")]
    pub noise_threshold: ::core::option::Option<f64>,
    /// Optional. Specifying any known seasonality/periodicity in the time series
    /// for the slices we will analyze can improve the quality of the results.
    ///
    /// If unsure, simply leave it unspecified by not setting a value for this
    /// field.
    ///
    /// If your time series has multiple seasonal patterns, then set it to the most
    /// granular one (e.g. if it has daily and weekly patterns, set this to DAILY).
    #[prost(enumeration="forecast_params::Period", tag="10")]
    pub seasonality_hint: i32,
    /// Optional. The length of the returned [forecasted
    /// timeseries]\[EvaluatedSlice.forecast\].
    ///
    /// This duration is currently capped at 100 x
    /// \[granularity][google.cloud.timeseriesinsights.v1.TimeseriesParams.granularity\].
    ///
    /// Example: If the detection point is set to "2020-12-27T00:00:00Z", the
    /// \[granularity][google.cloud.timeseriesinsights.v1.TimeseriesParams.granularity\]
    /// to "3600s" and the horizon_duration to "10800s", then we will generate 3
    /// time series points (from "2020-12-27T01:00:00Z" to "2020-12-27T04:00:00Z"),
    /// for which we will return their forecasted values.
    ///
    /// **NOTE**: The horizon time is only used for forecasting not for anormaly
    /// detection. To detect anomalies for multiple points of time,
    /// simply send multiple queries with those as
    /// \[detectionTime][google.cloud.timeseriesinsights.v1.QueryDataSetRequest.detection_time\].
    #[prost(message, optional, tag="13")]
    pub horizon_duration: ::core::option::Option<::prost_types::Duration>,
}
/// Nested message and enum types in `ForecastParams`.
pub mod forecast_params {
    /// A time period of a fixed interval.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Period {
        /// Unknown or simply not given.
        Unspecified = 0,
        /// 1 hour
        Hourly = 5,
        /// 24 hours
        Daily = 1,
        /// 7 days
        Weekly = 2,
        /// 30 days
        Monthly = 3,
        /// 365 days
        Yearly = 4,
    }
}
/// A point in a time series.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeseriesPoint {
    /// The timestamp of this point.
    #[prost(message, optional, tag="1")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// The value for this point.
    ///
    /// It is computed by aggregating all events in the associated slice that are
    /// in the [time, time + granularity] range (see
    /// \[granularity][google.cloud.timeseriesinsights.v1.TimeseriesParams.granularity\])
    /// using the specified
    /// \[metric][google.cloud.timeseriesinsights.v1.TimeseriesParams.metric\].
    #[prost(double, optional, tag="2")]
    pub value: ::core::option::Option<f64>,
}
/// A time series.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Timeseries {
    /// The points in this time series, ordered by their timestamp.
    #[prost(message, repeated, tag="1")]
    pub point: ::prost::alloc::vec::Vec<TimeseriesPoint>,
}
/// Forecast result for a given slice.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluatedSlice {
    /// Values for all categorical dimensions that uniquely identify this slice.
    #[prost(message, repeated, tag="1")]
    pub dimensions: ::prost::alloc::vec::Vec<PinnedDimension>,
    /// The actual value at the detection time (see
    /// \[detectionTime][google.cloud.timeseriesinsights.v1.QueryDataSetRequest.detection_time\]).
    ///
    /// **NOTE**: This value can be an estimate, so it should not be used as a
    /// source of truth.
    #[prost(double, optional, tag="11")]
    pub detection_point_actual: ::core::option::Option<f64>,
    /// The expected value at the detection time, which is obtained by forecasting
    /// on the historical time series.
    #[prost(double, optional, tag="12")]
    pub detection_point_forecast: ::core::option::Option<f64>,
    /// How much our forecast model expects the detection point actual will
    /// deviate from its forecasted value based on how well it fit the input time
    /// series.
    ///
    /// In general, we expect the `detectionPointActual` to
    /// be in the `[detectionPointForecast - expectedDeviation,
    /// detectionPointForecast + expectedDeviation]` range. The more the actual
    /// value is outside this range, the more statistically significant the
    /// anomaly is.
    ///
    /// The expected deviation is always positive.
    #[prost(double, optional, tag="16")]
    pub expected_deviation: ::core::option::Option<f64>,
    /// Summarizes how significant the change between the actual and forecasted
    /// detection points are compared with the historical patterns observed on the
    /// \[history][google.cloud.timeseriesinsights.v1.EvaluatedSlice.history\] time
    /// series.
    ///
    /// Defined as `|a - f| / (e + nt)`, where:
    ///
    /// - `a` is the
    /// \[detectionPointActual][google.cloud.timeseriesinsights.v1.EvaluatedSlice.detection_point_actual\].
    /// - `f` is the
    /// \[detectionPointForecast][google.cloud.timeseriesinsights.v1.EvaluatedSlice.detection_point_forecast\].
    /// - `e` is the
    /// \[expectedDeviation][google.cloud.timeseriesinsights.v1.EvaluatedSlice.expected_deviation\].
    /// - `nt` is the
    /// \[noiseThreshold][google.cloud.timeseriesinsights.v1.ForecastParams.noise_threshold\].
    ///
    /// Anomaly scores between different requests and data sets are comparable. As
    /// a guideline, the risk of a slice being an anomaly based on the anomaly
    /// score is:
    ///
    /// - **Very High** if anomaly_score > 5.
    /// - **High** if the anomaly_score is in the [2, 5] range.
    /// - **Medium** if the anomaly_score is in the [1, 2) range.
    /// - **Low** if the anomaly_score is < 1.
    ///
    /// If there were issues evaluating this slice, then the anomaly score will be
    /// set to -1.0 and the
    /// \[status][google.cloud.timeseriesinsights.v1.EvaluatedSlice.status\] field
    /// will contain details on what went wrong.
    #[prost(double, optional, tag="17")]
    pub anomaly_score: ::core::option::Option<f64>,
    /// The actual values in the `[`
    /// \[detectionTime][google.cloud.timeseriesinsights.v1.QueryDataSetRequest.detection_time\]
    /// `-`
    /// \[forecastHistory][google.cloud.timeseriesinsights.v1.TimeseriesParams.forecast_history\]`,`
    /// \[detectionTime][google.cloud.timeseriesinsights.v1.QueryDataSetRequest.detection_time\]
    /// `]` time range.
    ///
    /// **NOTE**: This field is only populated
    /// <https://old.reddit.com/r/nba?utm_source=reddit&utm_medium=usertext&utm_name=nba&utm_content=t5_2qo4sif>
    /// \[returnTimeseries][google.cloud.timeseriesinsights.v1.QueryDataSetRequest.return_timeseries\]
    /// is true.
    #[prost(message, optional, tag="5")]
    pub history: ::core::option::Option<Timeseries>,
    /// The forecasted values in the `[`
    /// \[detectionTime][google.cloud.timeseriesinsights.v1.QueryDataSetRequest.detection_time\]
    /// `+`
    /// \[granularity][google.cloud.timeseriesinsights.v1.TimeseriesParams.granularity\]`,`
    /// \[forecastParams.horizonTime][google.cloud.timeseriesinsights.v1.QueryDataSetRequest.forecast_params\]
    /// `]` time range.
    ///
    /// **NOTE**: This field is only populated if
    /// \[returnTimeseries][google.cloud.timeseriesinsights.v1.QueryDataSetRequest.return_timeseries\]
    /// is true.
    #[prost(message, optional, tag="10")]
    pub forecast: ::core::option::Option<Timeseries>,
    /// Evaluation status. Contains an error message if the anomaly_score is < 0.
    ///
    /// Possible error messages:
    /// - "Time series too sparse": The returned time series for this slice did not
    /// contain enough data points (we require a minimum of 10).
    /// - "Not enough recent time series points": The time series contains the
    /// minimum of 10 points, but there are not enough close in time to the
    /// detection point.
    /// - "Missing detection point data": There were not events to be aggregated
    /// within the [detectionTime + granularity] time interval, so we don't have an
    /// actual value with which we can compare our prediction.
    /// - "Data retrieval error": We failed to retrieve the time series data for
    /// this slice and could not evaluate it successfully. Should be a transient
    /// error.
    /// - "Internal server error": Internal unexpected error.
    #[prost(message, optional, tag="18")]
    pub status: ::core::option::Option<super::super::super::rpc::Status>,
}
/// Parameters that control how we slice the data set and, optionally, filter
/// slices that have some specific values on some dimensions (pinned dimensions).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SlicingParams {
    /// Required. Dimensions over which we will group the events in slices. The
    /// names specified here come from the
    /// \[EventDimension.name][google.cloud.timeseriesinsights.v1.EventDimension.name\]
    /// field. At least one dimension name must be specified. All dimension names
    /// that do not exist in the queried DataSet will be ignored.
    ///
    /// Currently only dimensions that hold string values can be specified here.
    #[prost(string, repeated, tag="1")]
    pub dimension_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. We will only analyze slices for which
    /// \[ForecastSlice.dimensions][ForecastSlice.dimensions\] contain all of the
    /// following pinned dimensions. A query with a pinned dimension `{ name: "d3"
    /// stringVal: "v3" }` will only analyze events which contain the dimension `{
    /// name: "d3" stringVal: "v3" }`.
    /// The
    /// \[pinnedDimensions][google.cloud.timeseriesinsights.v1.SlicingParams.pinned_dimensions\]
    /// and
    /// \[dimensionNames][google.cloud.timeseriesinsights.v1.SlicingParams.dimension_names\]
    /// fields can **not** share the same dimension names.
    ///
    /// Example a valid specification:
    ///
    /// ```json
    /// {
    ///   dimensionNames: ["d1", "d2"],
    ///   pinnedDimensions: [
    ///     { name: "d3" stringVal: "v3" },
    ///     { name: "d4" stringVal: "v4" }
    ///   ]
    /// }
    /// ```
    ///
    /// In the previous example we will slice the data set by dimensions "d1",
    /// "d2", "d3" and "d4", but we will only analyze slices for which "d3=v3" and
    /// "d4=v4".
    ///
    /// The following example is **invalid** as "d2" is present in both
    /// dimensionNames and pinnedDimensions:
    ///
    /// ```json
    /// {
    ///   dimensionNames: ["d1", "d2"],
    ///   pinnedDimensions: [
    ///     { name: "d2" stringVal: "v2" },
    ///     { name: "d4" stringVal: "v4" }
    ///   ]
    /// }
    /// ```
    #[prost(message, repeated, tag="2")]
    pub pinned_dimensions: ::prost::alloc::vec::Vec<PinnedDimension>,
}
/// Parameters that control how we construct the time series for each slice.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeseriesParams {
    /// Required. How long should we go in the past when fetching the timeline used
    /// for forecasting each slice.
    ///
    /// This is used in combination with the
    /// \[detectionTime][google.cloud.timeseriesinsights.v1.QueryDataSetRequest.detection_time\]
    /// parameter. The time series we construct will have the following time range:
    /// [detectionTime - forecastHistory, detectionTime + granularity].
    ///
    /// The forecast history might be rounded up, so that a multiple of
    /// `granularity` is used to process the query.
    ///
    /// **NOTE**: Make sure there are enough events in the data set to cover the
    /// time interval: if the `detectionTime` is set to T, the data set should
    /// have enough events from "T - forecastHistory" to allow forecasting and
    /// anomaly detection.  If too few events are found in this range, no anomaly
    /// will be reported.  Users can use \[EvaluateSlice][EvaluateSlice\] to
    /// check the (approximate) count of events for each point of time in the
    /// range, by not setting `metric`.
    #[prost(message, optional, tag="1")]
    pub forecast_history: ::core::option::Option<::prost_types::Duration>,
    /// Required. The time granularity of the time series (on the x-axis). Each
    /// time series point starting at time T will aggregate all events for a
    /// particular slice in [T, T + granularity) time windows.
    ///
    /// **NOTE**: The aggregation is decided based on the
    /// \[metric][google.cloud.timeseriesinsights.v1.TimeseriesParams.metric\]
    /// parameter.
    ///
    /// This granularity defines the query-time aggregation windows and is not
    /// necessarily related to any event time granularity in the raw data (though
    /// we do recommend that the query-time granularity is not finer than the
    /// ingestion-time one).
    ///
    /// Currently, the minimal supported granularity is 10 seconds.
    #[prost(message, optional, tag="2")]
    pub granularity: ::core::option::Option<::prost_types::Duration>,
    /// Optional. Denotes the
    /// \[name][google.cloud.timeseriesinsights.v1.EventDimension.name\] of a
    /// numerical dimension that will have its values aggregated to compute the
    /// y-axis of the time series.
    ///
    /// The aggregation method must also be specified by setting the
    /// \[metricAggregationMethod][TimeseriesParams.metricAggregationMethod\] field.
    ///
    /// **Note**: Currently, if the aggregation method is unspecified, we will
    /// default to SUM for backwards compatibility reasons, but new implementations
    /// should set the
    /// \[metricAggregationMethod][TimeseriesParams.metricAggregationMethod\]
    /// explicitly.
    ///
    /// If the metric is unspecified, we will use the number of events that each
    /// time series point contains as the point value.
    ///
    /// Example: Let's assume we have the following three events in our data set:
    /// ```json
    /// {
    ///   eventTime: "2020-12-27T00:00:00Z",
    ///   dimensions: [
    ///     { name: "d1" stringVal: "v1" },
    ///     { name: "d2" stringVal: "v2" }
    ///     { name: "m1" longVal: 100 }
    ///     { name: "m2" longVal: 11 }
    ///   ]
    /// },
    /// {
    ///   eventTime: "2020-12-27T00:10:00Z",
    ///   dimensions: [
    ///     { name: "d1" stringVal: "v1" },
    ///     { name: "d2" stringVal: "v2" }
    ///     { name: "m1" longVal: 200 }
    ///     { name: "m2" longVal: 22 }
    ///   ]
    /// },
    /// {
    ///   eventTime: "2020-12-27T00:20:00Z",
    ///   dimensions: [
    ///     { name: "d1" stringVal: "v1" },
    ///     { name: "d2" stringVal: "v2" }
    ///     { name: "m1" longVal: 300 }
    ///     { name: "m2" longVal: 33 }
    ///   ]
    /// }
    /// ```
    ///
    /// These events are all within the same hour, spaced 10 minutes between each
    /// of them. Assuming our
    /// \[QueryDataSetRequest][google.cloud.timeseriesinsights.v1.QueryDataSetRequest\]
    /// had set
    /// \[slicingParams.dimensionNames][google.cloud.timeseriesinsights.v1.SlicingParams.dimension_names\]
    /// to ["d1", "d2"] and
    /// \[timeseries_params.granularity][google.cloud.timeseriesinsights.v1.TimeseriesParams.granularity\]
    /// to "3600s", then all the previous events will be aggregated into the same
    /// [timeseries point]\[google.cloud.timeseriesinsights.v1.TimeseriesPoint\].
    ///
    /// The time series point which they are all part of will have the
    /// \[time][google.cloud.timeseriesinsights.v1.TimeseriesPoint.time\] set to
    /// "2020-12-27T00:00:00Z" and the
    /// \[value][google.cloud.timeseriesinsights.v1.TimeseriesPoint.value\] populated
    /// based on this metric field:
    ///
    /// - If the metric is set to "m1" and metric_aggregation_method to SUM, then
    /// the value of the point will be 600.
    /// - If the metric is set to "m2" and metric_aggregation_method to SUM, then
    /// the value of the point will be 66.
    /// - If the metric is set to "m1" and metric_aggregation_method to AVERAGE,
    /// then the value of the point will be 200.
    /// - If the metric is set to "m2" and metric_aggregation_method to AVERAGE,
    /// then the value of the point will be 22.
    /// - If the metric field is "" or unspecified, then the value of the point
    /// will be 3, as we will simply count the events.
    #[prost(string, optional, tag="4")]
    pub metric: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional. Together with the
    /// \[metric][google.cloud.timeseriesinsights.v1.TimeseriesParams.metric\] field,
    /// specifies how we will aggregate multiple events to obtain the value of a
    /// time series point. See the
    /// \[metric][google.cloud.timeseriesinsights.v1.TimeseriesParams.metric\]
    /// documentation for more details.
    ///
    /// If the metric is not specified or "", then this field will be ignored.
    #[prost(enumeration="timeseries_params::AggregationMethod", tag="5")]
    pub metric_aggregation_method: i32,
}
/// Nested message and enum types in `TimeseriesParams`.
pub mod timeseries_params {
    /// Methods by which we can aggregate multiple events by a given
    /// \[metric][google.cloud.timeseriesinsights.v1.TimeseriesParams.metric\].
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AggregationMethod {
        /// Unspecified.
        Unspecified = 0,
        /// Aggregate multiple events by summing up the values found in the
        /// \[metric][google.cloud.timeseriesinsights.v1.TimeseriesParams.metric\]
        /// dimension.
        Sum = 1,
        /// Aggregate multiple events by averaging out the values found in the
        /// \[metric][google.cloud.timeseriesinsights.v1.TimeseriesParams.metric\]
        /// dimension.
        Average = 2,
    }
}
/// Request for performing a query against a loaded DataSet.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDataSetRequest {
    /// Required. Loaded DataSet to be queried in the format of
    /// "projects/{project}/datasets/{dataset}"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Required. This is the point in time which we want to probe for anomalies.
    ///
    /// The corresponding
    /// \[TimeseriesPoint][google.cloud.timeseriesinsights.v1.TimeseriesPoint\] is
    /// referred to as the detection point.
    ///
    /// **NOTE**: As with any other time series point, the value is given by
    /// aggregating all events in the slice that are in the
    /// [detectionTime, detectionTime + granularity) time interval, where
    /// the granularity is specified in the
    /// \[timeseriesParams.granularity][google.cloud.timeseriesinsights.v1.TimeseriesParams.granularity\]
    /// field.
    #[prost(message, optional, tag="11")]
    pub detection_time: ::core::option::Option<::prost_types::Timestamp>,
    /// How many slices are returned in
    /// \[QueryDataSetResponse.slices][google.cloud.timeseriesinsights.v1.QueryDataSetResponse.slices\].
    ///
    /// The returned slices are tentatively the ones with the highest [anomaly
    /// scores]\[EvaluatedSlice.anomaly_score\] in the data set that match the query,
    /// but it is not guaranteed.
    ///
    /// Reducing this number will improve query performance, both in terms of
    /// latency and resource usage.
    ///
    /// Defaults to 50.
    #[prost(int32, optional, tag="13")]
    pub num_returned_slices: ::core::option::Option<i32>,
    /// Parameters controlling how we will split the data set into the slices that
    /// we will analyze.
    #[prost(message, optional, tag="9")]
    pub slicing_params: ::core::option::Option<SlicingParams>,
    /// Parameters controlling how we will build the time series used to predict
    /// the
    /// \[detectionTime][google.cloud.timeseriesinsights.v1.QueryDataSetRequest.detection_time\]
    /// value for each slice.
    #[prost(message, optional, tag="10")]
    pub timeseries_params: ::core::option::Option<TimeseriesParams>,
    /// Parameters that control the time series forecasting models, such as the
    /// sensitivity of the anomaly detection.
    #[prost(message, optional, tag="5")]
    pub forecast_params: ::core::option::Option<ForecastParams>,
    /// If specified, we will return the actual and forecasted time for all
    /// returned slices.
    ///
    /// The time series are returned in the
    /// \[EvaluatedSlice.history][google.cloud.timeseriesinsights.v1.EvaluatedSlice.history\]
    /// and
    /// \[EvaluatedSlice.forecast][google.cloud.timeseriesinsights.v1.EvaluatedSlice.forecast\]
    /// fields.
    #[prost(bool, tag="8")]
    pub return_timeseries: bool,
}
/// Response for a query executed by the system.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDataSetResponse {
    /// Loaded DataSet that was queried.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Slices sorted in descending order by their
    /// \[anomalyScore][google.cloud.timeseriesinsights.v1.EvaluatedSlice.anomaly_score\].
    ///
    /// At most
    /// \[numReturnedSlices][google.cloud.timeseriesinsights.v1.QueryDataSetRequest.num_returned_slices\]
    /// slices are present in this field.
    #[prost(message, repeated, tag="3")]
    pub slices: ::prost::alloc::vec::Vec<EvaluatedSlice>,
}
/// Request for evaluateSlice.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluateSliceRequest {
    /// Required. Loaded DataSet to be queried in the format of
    /// "projects/{project}/datasets/{dataset}"
    #[prost(string, tag="1")]
    pub dataset: ::prost::alloc::string::String,
    /// Required. Dimensions with pinned values that specify the slice for which we
    /// will fetch the time series.
    #[prost(message, repeated, tag="2")]
    pub pinned_dimensions: ::prost::alloc::vec::Vec<PinnedDimension>,
    /// Required. This is the point in time which we want to probe for anomalies.
    ///
    /// See documentation for
    /// \[QueryDataSetRequest.detectionPoint][QueryDataSetRequest.detection_point\].
    #[prost(message, optional, tag="3")]
    pub detection_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Parameters controlling how we will build the time series used to predict
    /// the
    /// \[detectionTime][google.cloud.timeseriesinsights.v1.EvaluateSliceRequest.detection_time\]
    /// value for this slice.
    #[prost(message, optional, tag="4")]
    pub timeseries_params: ::core::option::Option<TimeseriesParams>,
    /// Parameters that control the time series forecasting models, such as the
    /// sensitivity of the anomaly detection.
    #[prost(message, optional, tag="5")]
    pub forecast_params: ::core::option::Option<ForecastParams>,
}
/// Generated client implementations.
pub mod timeseries_insights_controller_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct TimeseriesInsightsControllerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> TimeseriesInsightsControllerClient<T>
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
        ) -> TimeseriesInsightsControllerClient<InterceptedService<T, F>>
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
            TimeseriesInsightsControllerClient::new(
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
        /// Lists [DataSets][google.cloud.timeseriesinsights.v1.DataSet] under the
        /// project.
        ///
        /// The order of the results is unspecified but deterministic. Newly created
        /// [DataSets][google.cloud.timeseriesinsights.v1.DataSet] will not necessarily
        /// be added to the end of this list.
        pub async fn list_data_sets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDataSetsRequest>,
        ) -> Result<tonic::Response<super::ListDataSetsResponse>, tonic::Status> {
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
                "/google.cloud.timeseriesinsights.v1.TimeseriesInsightsController/ListDataSets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Create a [DataSet][google.cloud.timeseriesinsights.v1.DataSet] from data
        /// stored on Cloud Storage.
        ///
        /// The data must stay immutable while we process the
        /// [DataSet][google.cloud.timeseriesinsights.v1.DataSet] creation; otherwise,
        /// undefined outcomes might result.  For more information, see
        /// [DataSet][google.cloud.timeseriesinsights.v1.DataSet].
        pub async fn create_data_set(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDataSetRequest>,
        ) -> Result<tonic::Response<super::DataSet>, tonic::Status> {
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
                "/google.cloud.timeseriesinsights.v1.TimeseriesInsightsController/CreateDataSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Delete a [DataSet][google.cloud.timeseriesinsights.v1.DataSet] from the
        /// system.
        ///
        /// **NOTE**: If the [DataSet][google.cloud.timeseriesinsights.v1.DataSet] is
        /// still being processed, it will be aborted and deleted.
        pub async fn delete_data_set(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDataSetRequest>,
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
                "/google.cloud.timeseriesinsights.v1.TimeseriesInsightsController/DeleteDataSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Append events to a `LOADED`
        /// [DataSet][google.cloud.timeseriesinsights.v1.DataSet].
        pub async fn append_events(
            &mut self,
            request: impl tonic::IntoRequest<super::AppendEventsRequest>,
        ) -> Result<tonic::Response<super::AppendEventsResponse>, tonic::Status> {
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
                "/google.cloud.timeseriesinsights.v1.TimeseriesInsightsController/AppendEvents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Execute a Timeseries Insights query over a loaded
        /// [DataSet][google.cloud.timeseriesinsights.v1.DataSet].
        pub async fn query_data_set(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDataSetRequest>,
        ) -> Result<tonic::Response<super::QueryDataSetResponse>, tonic::Status> {
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
                "/google.cloud.timeseriesinsights.v1.TimeseriesInsightsController/QueryDataSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Evaluate an explicit slice from a loaded
        /// [DataSet][google.cloud.timeseriesinsights.v1.DataSet].
        pub async fn evaluate_slice(
            &mut self,
            request: impl tonic::IntoRequest<super::EvaluateSliceRequest>,
        ) -> Result<tonic::Response<super::EvaluatedSlice>, tonic::Status> {
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
                "/google.cloud.timeseriesinsights.v1.TimeseriesInsightsController/EvaluateSlice",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
