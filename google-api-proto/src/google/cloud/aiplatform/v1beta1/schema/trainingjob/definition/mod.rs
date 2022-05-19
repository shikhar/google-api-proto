/// Configuration for exporting test set predictions to a BigQuery table.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportEvaluatedDataItemsConfig {
    /// URI of desired destination BigQuery table. Expected format:
    /// bq://<project_id>:<dataset_id>:<table>
    ///
    /// If not specified, then results are exported to the following auto-created
    /// BigQuery table:
    /// <project_id>:export_evaluated_examples_<model_name>_<yyyy_MM_dd'T'HH_mm_ss_SSS'Z'>.evaluated_examples
    #[prost(string, tag="1")]
    pub destination_bigquery_uri: ::prost::alloc::string::String,
    /// If true and an export destination is specified, then the contents of the
    /// destination are overwritten. Otherwise, if the export destination already
    /// exists, then the export operation fails.
    #[prost(bool, tag="2")]
    pub override_existing_table: bool,
}
/// A TrainingJob that trains and uploads an AutoML Forecasting Model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoMlForecasting {
    /// The input parameters of this TrainingJob.
    #[prost(message, optional, tag="1")]
    pub inputs: ::core::option::Option<AutoMlForecastingInputs>,
    /// The metadata information.
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<AutoMlForecastingMetadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoMlForecastingInputs {
    /// The name of the column that the model is to predict.
    #[prost(string, tag="1")]
    pub target_column: ::prost::alloc::string::String,
    /// The name of the column that identifies the time series.
    #[prost(string, tag="2")]
    pub time_series_identifier_column: ::prost::alloc::string::String,
    /// The name of the column that identifies time order in the time series.
    #[prost(string, tag="3")]
    pub time_column: ::prost::alloc::string::String,
    /// Each transformation will apply transform function to given input column.
    /// And the result will be used for training.
    /// When creating transformation for BigQuery Struct column, the column should
    /// be flattened using "." as the delimiter.
    #[prost(message, repeated, tag="4")]
    pub transformations: ::prost::alloc::vec::Vec<auto_ml_forecasting_inputs::Transformation>,
    /// Objective function the model is optimizing towards. The training process
    /// creates a model that optimizes the value of the objective
    /// function over the validation set.
    ///
    /// The supported optimization objectives:
    ///
    ///   * "minimize-rmse" (default) - Minimize root-mean-squared error (RMSE).
    ///
    ///   * "minimize-mae" - Minimize mean-absolute error (MAE).
    ///
    ///   * "minimize-rmsle" - Minimize root-mean-squared log error (RMSLE).
    ///
    ///   * "minimize-rmspe" - Minimize root-mean-squared percentage error (RMSPE).
    ///
    ///   * "minimize-wape-mae" - Minimize the combination of weighted absolute
    ///     percentage error (WAPE) and mean-absolute-error (MAE).
    ///
    ///   * "minimize-quantile-loss" - Minimize the quantile loss at the quantiles
    ///     defined in `quantiles`.
    #[prost(string, tag="5")]
    pub optimization_objective: ::prost::alloc::string::String,
    /// Required. The train budget of creating this model, expressed in milli node
    /// hours i.e. 1,000 value in this field means 1 node hour.
    ///
    /// The training cost of the model will not exceed this budget. The final cost
    /// will be attempted to be close to the budget, though may end up being (even)
    /// noticeably smaller - at the backend's discretion. This especially may
    /// happen when further model training ceases to provide any improvements.
    ///
    /// If the budget is set to a value known to be insufficient to train a
    /// model for the given dataset, the training won't be attempted and
    /// will error.
    ///
    /// The train budget must be between 1,000 and 72,000 milli node hours,
    /// inclusive.
    #[prost(int64, tag="6")]
    pub train_budget_milli_node_hours: i64,
    /// Column name that should be used as the weight column.
    /// Higher values in this column give more importance to the row
    /// during model training. The column must have numeric values between 0 and
    /// 10000 inclusively; 0 means the row is ignored for training. If weight
    /// column field is not set, then all rows are assumed to have equal weight
    /// of 1.
    #[prost(string, tag="7")]
    pub weight_column: ::prost::alloc::string::String,
    /// Column names that should be used as attribute columns.
    /// The value of these columns does not vary as a function of time.
    /// For example, store ID or item color.
    #[prost(string, repeated, tag="19")]
    pub time_series_attribute_columns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Names of columns that are unavailable when a forecast is requested.
    /// This column contains information for the given entity (identified
    /// by the time_series_identifier_column) that is unknown before the forecast
    /// For example, actual weather on a given day.
    #[prost(string, repeated, tag="20")]
    pub unavailable_at_forecast_columns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Names of columns that are available and provided when a forecast
    /// is requested. These columns
    /// contain information for the given entity (identified by the
    /// time_series_identifier_column column) that is known at forecast.
    /// For example, predicted weather for a specific day.
    #[prost(string, repeated, tag="21")]
    pub available_at_forecast_columns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Expected difference in time granularity between rows in the data.
    #[prost(message, optional, tag="22")]
    pub data_granularity: ::core::option::Option<auto_ml_forecasting_inputs::Granularity>,
    /// The amount of time into the future for which forecasted values for the
    /// target are returned. Expressed in number of units defined by the
    /// `data_granularity` field.
    #[prost(int64, tag="23")]
    pub forecast_horizon: i64,
    /// The amount of time into the past training and prediction data is used
    /// for model training and prediction respectively. Expressed in number of
    /// units defined by the `data_granularity` field.
    #[prost(int64, tag="24")]
    pub context_window: i64,
    /// Configuration for exporting test set predictions to a BigQuery table. If
    /// this configuration is absent, then the export is not performed.
    #[prost(message, optional, tag="15")]
    pub export_evaluated_data_items_config: ::core::option::Option<ExportEvaluatedDataItemsConfig>,
    /// Quantiles to use for minimize-quantile-loss `optimization_objective`. Up to
    /// 5 quantiles are allowed of values between 0 and 1, exclusive. Required if
    /// the value of optimization_objective is minimize-quantile-loss. Represents
    /// the percent quantiles to use for that objective. Quantiles must be unique.
    #[prost(double, repeated, tag="16")]
    pub quantiles: ::prost::alloc::vec::Vec<f64>,
    /// Validation options for the data validation component. The available options
    /// are:
    ///
    ///   * "fail-pipeline" - default, will validate against the validation and
    ///      fail the pipeline if it fails.
    ///
    ///   * "ignore-validation" - ignore the results of the validation and continue
    #[prost(string, tag="17")]
    pub validation_options: ::prost::alloc::string::String,
    /// Additional experiment flags for the time series forcasting training.
    #[prost(string, repeated, tag="25")]
    pub additional_experiments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `AutoMlForecastingInputs`.
pub mod auto_ml_forecasting_inputs {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Transformation {
        /// The transformation that the training pipeline will apply to the input
        /// columns.
        #[prost(oneof="transformation::TransformationDetail", tags="1, 2, 3, 4, 5")]
        pub transformation_detail: ::core::option::Option<transformation::TransformationDetail>,
    }
    /// Nested message and enum types in `Transformation`.
    pub mod transformation {
        /// Training pipeline will infer the proper transformation based on the
        /// statistic of dataset.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct AutoTransformation {
            #[prost(string, tag="1")]
            pub column_name: ::prost::alloc::string::String,
        }
        /// Training pipeline will perform following transformation functions.
        ///
        /// *  The value converted to float32.
        ///
        /// *  The z_score of the value.
        ///
        /// *  log(value+1) when the value is greater than or equal to 0. Otherwise,
        ///    this transformation is not applied and the value is considered a
        ///    missing value.
        ///
        /// *  z_score of log(value+1) when the value is greater than or equal to 0.
        ///    Otherwise, this transformation is not applied and the value is
        ///    considered a missing value.
        ///
        /// *  A boolean value that indicates whether the value is valid.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct NumericTransformation {
            #[prost(string, tag="1")]
            pub column_name: ::prost::alloc::string::String,
        }
        /// Training pipeline will perform following transformation functions.
        ///
        /// *  The categorical string as is--no change to case, punctuation,
        ///    spelling, tense, and so on.
        ///
        /// *  Convert the category name to a dictionary lookup index and generate an
        ///    embedding for each index.
        ///
        /// *  Categories that appear less than 5 times in the training dataset are
        ///    treated as the "unknown" category. The "unknown" category gets its own
        ///    special lookup index and resulting embedding.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CategoricalTransformation {
            #[prost(string, tag="1")]
            pub column_name: ::prost::alloc::string::String,
        }
        /// Training pipeline will perform following transformation functions.
        ///
        /// *  Apply the transformation functions for Numerical columns.
        ///
        /// *  Determine the year, month, day,and weekday. Treat each value from the
        ///    timestamp as a Categorical column.
        ///
        /// *  Invalid numerical values (for example, values that fall outside of a
        ///    typical timestamp range, or are extreme values) receive no special
        ///    treatment and are not removed.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TimestampTransformation {
            #[prost(string, tag="1")]
            pub column_name: ::prost::alloc::string::String,
            /// The format in which that time field is expressed. The time_format must
            /// either be one of:
            ///
            /// * `unix-seconds`
            ///
            /// * `unix-milliseconds`
            ///
            /// * `unix-microseconds`
            ///
            /// * `unix-nanoseconds`
            ///
            /// (for respectively number of seconds, milliseconds, microseconds and
            /// nanoseconds since start of the Unix epoch);
            ///
            /// or be written in `strftime` syntax.
            ///
            /// If time_format is not set, then the
            /// default format is RFC 3339 `date-time` format, where
            /// `time-offset` = `"Z"` (e.g. 1985-04-12T23:20:50.52Z)
            #[prost(string, tag="2")]
            pub time_format: ::prost::alloc::string::String,
        }
        /// Training pipeline will perform following transformation functions.
        ///
        /// *  The text as is--no change to case, punctuation, spelling, tense, and
        ///    so on.
        ///
        /// *  Convert the category name to a dictionary lookup index and generate an
        ///    embedding for each index.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TextTransformation {
            #[prost(string, tag="1")]
            pub column_name: ::prost::alloc::string::String,
        }
        /// The transformation that the training pipeline will apply to the input
        /// columns.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum TransformationDetail {
            #[prost(message, tag="1")]
            Auto(AutoTransformation),
            #[prost(message, tag="2")]
            Numeric(NumericTransformation),
            #[prost(message, tag="3")]
            Categorical(CategoricalTransformation),
            #[prost(message, tag="4")]
            Timestamp(TimestampTransformation),
            #[prost(message, tag="5")]
            Text(TextTransformation),
        }
    }
    /// A duration of time expressed in time granularity units.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Granularity {
        /// The time granularity unit of this time period.
        /// The supported units are:
        ///
        ///  * "minute"
        ///
        ///  * "hour"
        ///
        ///  * "day"
        ///
        ///  * "week"
        ///
        ///  * "month"
        ///
        ///  * "year"
        #[prost(string, tag="1")]
        pub unit: ::prost::alloc::string::String,
        /// The number of granularity_units between data points in the training
        /// data. If `granularity_unit` is `minute`,
        /// can be 1, 5, 10, 15, or 30. For all other values of `granularity_unit`,
        /// must be 1.
        #[prost(int64, tag="2")]
        pub quantity: i64,
    }
}
/// Model metadata specific to AutoML Forecasting.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoMlForecastingMetadata {
    /// Output only. The actual training cost of the model, expressed in milli
    /// node hours, i.e. 1,000 value in this field means 1 node hour. Guaranteed
    /// to not exceed the train budget.
    #[prost(int64, tag="1")]
    pub train_cost_milli_node_hours: i64,
}
/// A TrainingJob that trains and uploads an AutoML Video ObjectTracking Model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoMlVideoObjectTracking {
    /// The input parameters of this TrainingJob.
    #[prost(message, optional, tag="1")]
    pub inputs: ::core::option::Option<AutoMlVideoObjectTrackingInputs>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoMlVideoObjectTrackingInputs {
    #[prost(enumeration="auto_ml_video_object_tracking_inputs::ModelType", tag="1")]
    pub model_type: i32,
}
/// Nested message and enum types in `AutoMlVideoObjectTrackingInputs`.
pub mod auto_ml_video_object_tracking_inputs {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ModelType {
        /// Should not be set.
        Unspecified = 0,
        /// A model best tailored to be used within Google Cloud, and which c annot
        /// be exported. Default.
        Cloud = 1,
        /// A model that, in addition to being available within Google Cloud, can
        /// also be exported (see ModelService.ExportModel) as a TensorFlow or
        /// TensorFlow Lite model and used on a mobile or edge device afterwards.
        MobileVersatile1 = 2,
        /// A versatile model that is meant to be exported (see
        /// ModelService.ExportModel) and used on a Google Coral device.
        MobileCoralVersatile1 = 3,
        /// A model that trades off quality for low latency, to be exported (see
        /// ModelService.ExportModel) and used on a Google Coral device.
        MobileCoralLowLatency1 = 4,
        /// A versatile model that is meant to be exported (see
        /// ModelService.ExportModel) and used on an NVIDIA Jetson device.
        MobileJetsonVersatile1 = 5,
        /// A model that trades off quality for low latency, to be exported (see
        /// ModelService.ExportModel) and used on an NVIDIA Jetson device.
        MobileJetsonLowLatency1 = 6,
    }
}
/// A TrainingJob that trains and uploads an AutoML Image Object Detection Model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoMlImageObjectDetection {
    /// The input parameters of this TrainingJob.
    #[prost(message, optional, tag="1")]
    pub inputs: ::core::option::Option<AutoMlImageObjectDetectionInputs>,
    /// The metadata information
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<AutoMlImageObjectDetectionMetadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoMlImageObjectDetectionInputs {
    #[prost(enumeration="auto_ml_image_object_detection_inputs::ModelType", tag="1")]
    pub model_type: i32,
    /// The training budget of creating this model, expressed in milli node
    /// hours i.e. 1,000 value in this field means 1 node hour. The actual
    /// metadata.costMilliNodeHours will be equal or less than this value.
    /// If further model training ceases to provide any improvements, it will
    /// stop without using the full budget and the metadata.successfulStopReason
    /// will be `model-converged`.
    /// Note, node_hour  = actual_hour * number_of_nodes_involved.
    /// For modelType `cloud`(default), the budget must be between 20,000
    /// and 900,000 milli node hours, inclusive. The default value is 216,000
    /// which represents one day in wall time, considering 9 nodes are used.
    /// For model types `mobile-tf-low-latency-1`, `mobile-tf-versatile-1`,
    /// `mobile-tf-high-accuracy-1`
    /// the training budget must be between 1,000 and 100,000 milli node hours,
    /// inclusive. The default value is 24,000 which represents one day in
    /// wall time on a single node that is used.
    #[prost(int64, tag="2")]
    pub budget_milli_node_hours: i64,
    /// Use the entire training budget. This disables the early stopping feature.
    /// When false the early stopping feature is enabled, which means that AutoML
    /// Image Object Detection might stop training before the entire training
    /// budget has been used.
    #[prost(bool, tag="3")]
    pub disable_early_stopping: bool,
}
/// Nested message and enum types in `AutoMlImageObjectDetectionInputs`.
pub mod auto_ml_image_object_detection_inputs {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ModelType {
        /// Should not be set.
        Unspecified = 0,
        /// A model best tailored to be used within Google Cloud, and which cannot
        /// be exported. Expected to have a higher latency, but should also have a
        /// higher prediction quality than other cloud models.
        CloudHighAccuracy1 = 1,
        /// A model best tailored to be used within Google Cloud, and which cannot
        /// be exported. Expected to have a low latency, but may have lower
        /// prediction quality than other cloud models.
        CloudLowLatency1 = 2,
        /// A model that, in addition to being available within Google
        /// Cloud can also be exported (see ModelService.ExportModel) and
        /// used on a mobile or edge device with TensorFlow afterwards.
        /// Expected to have low latency, but may have lower prediction
        /// quality than other mobile models.
        MobileTfLowLatency1 = 3,
        /// A model that, in addition to being available within Google
        /// Cloud can also be exported (see ModelService.ExportModel) and
        /// used on a mobile or edge device with TensorFlow afterwards.
        MobileTfVersatile1 = 4,
        /// A model that, in addition to being available within Google
        /// Cloud, can also be exported (see ModelService.ExportModel) and
        /// used on a mobile or edge device with TensorFlow afterwards.
        /// Expected to have a higher latency, but should also have a higher
        /// prediction quality than other mobile models.
        MobileTfHighAccuracy1 = 5,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoMlImageObjectDetectionMetadata {
    /// The actual training cost of creating this model, expressed in
    /// milli node hours, i.e. 1,000 value in this field means 1 node hour.
    /// Guaranteed to not exceed inputs.budgetMilliNodeHours.
    #[prost(int64, tag="1")]
    pub cost_milli_node_hours: i64,
    /// For successful job completions, this is the reason why the job has
    /// finished.
    #[prost(enumeration="auto_ml_image_object_detection_metadata::SuccessfulStopReason", tag="2")]
    pub successful_stop_reason: i32,
}
/// Nested message and enum types in `AutoMlImageObjectDetectionMetadata`.
pub mod auto_ml_image_object_detection_metadata {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SuccessfulStopReason {
        /// Should not be set.
        Unspecified = 0,
        /// The inputs.budgetMilliNodeHours had been reached.
        BudgetReached = 1,
        /// Further training of the Model ceased to increase its quality, since it
        /// already has converged.
        ModelConverged = 2,
    }
}
/// A TrainingJob that trains and uploads an AutoML Tables Model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoMlTables {
    /// The input parameters of this TrainingJob.
    #[prost(message, optional, tag="1")]
    pub inputs: ::core::option::Option<AutoMlTablesInputs>,
    /// The metadata information.
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<AutoMlTablesMetadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoMlTablesInputs {
    /// The type of prediction the Model is to produce.
    ///   "classification" - Predict one out of multiple target values is
    ///                      picked for each row.
    ///   "regression" - Predict a value based on its relation to other values.
    ///                  This type is available only to columns that contain
    ///                  semantically numeric values, i.e. integers or floating
    ///                  point number, even if stored as e.g. strings.
    #[prost(string, tag="1")]
    pub prediction_type: ::prost::alloc::string::String,
    /// The column name of the target column that the model is to predict.
    #[prost(string, tag="2")]
    pub target_column: ::prost::alloc::string::String,
    /// Each transformation will apply transform function to given input column.
    /// And the result will be used for training.
    /// When creating transformation for BigQuery Struct column, the column should
    /// be flattened using "." as the delimiter.
    #[prost(message, repeated, tag="3")]
    pub transformations: ::prost::alloc::vec::Vec<auto_ml_tables_inputs::Transformation>,
    /// Objective function the model is optimizing towards. The training process
    /// creates a model that maximizes/minimizes the value of the objective
    /// function over the validation set.
    ///
    /// The supported optimization objectives depend on the prediction type.
    /// If the field is not set, a default objective function is used.
    ///
    /// classification (binary):
    ///   "maximize-au-roc" (default) - Maximize the area under the receiver
    ///                                 operating characteristic (ROC) curve.
    ///   "minimize-log-loss" - Minimize log loss.
    ///   "maximize-au-prc" - Maximize the area under the precision-recall curve.
    ///   "maximize-precision-at-recall" - Maximize precision for a specified
    ///                                   recall value.
    ///   "maximize-recall-at-precision" - Maximize recall for a specified
    ///                                    precision value.
    ///
    /// classification (multi-class):
    ///   "minimize-log-loss" (default) - Minimize log loss.
    ///
    /// regression:
    ///   "minimize-rmse" (default) - Minimize root-mean-squared error (RMSE).
    ///   "minimize-mae" - Minimize mean-absolute error (MAE).
    ///   "minimize-rmsle" - Minimize root-mean-squared log error (RMSLE).
    #[prost(string, tag="4")]
    pub optimization_objective: ::prost::alloc::string::String,
    /// Required. The train budget of creating this model, expressed in milli node
    /// hours i.e. 1,000 value in this field means 1 node hour.
    ///
    /// The training cost of the model will not exceed this budget. The final cost
    /// will be attempted to be close to the budget, though may end up being (even)
    /// noticeably smaller - at the backend's discretion. This especially may
    /// happen when further model training ceases to provide any improvements.
    ///
    /// If the budget is set to a value known to be insufficient to train a
    /// model for the given dataset, the training won't be attempted and
    /// will error.
    ///
    /// The train budget must be between 1,000 and 72,000 milli node hours,
    /// inclusive.
    #[prost(int64, tag="7")]
    pub train_budget_milli_node_hours: i64,
    /// Use the entire training budget. This disables the early stopping feature.
    /// By default, the early stopping feature is enabled, which means that AutoML
    /// Tables might stop training before the entire training budget has been used.
    #[prost(bool, tag="8")]
    pub disable_early_stopping: bool,
    /// Column name that should be used as the weight column.
    /// Higher values in this column give more importance to the row
    /// during model training. The column must have numeric values between 0 and
    /// 10000 inclusively; 0 means the row is ignored for training. If weight
    /// column field is not set, then all rows are assumed to have equal weight
    /// of 1.
    #[prost(string, tag="9")]
    pub weight_column_name: ::prost::alloc::string::String,
    /// Configuration for exporting test set predictions to a BigQuery table. If
    /// this configuration is absent, then the export is not performed.
    #[prost(message, optional, tag="10")]
    pub export_evaluated_data_items_config: ::core::option::Option<ExportEvaluatedDataItemsConfig>,
    /// Additional experiment flags for the Tables training pipeline.
    #[prost(string, repeated, tag="11")]
    pub additional_experiments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Additional optimization objective configuration. Required for
    /// `maximize-precision-at-recall` and `maximize-recall-at-precision`,
    /// otherwise unused.
    #[prost(oneof="auto_ml_tables_inputs::AdditionalOptimizationObjectiveConfig", tags="5, 6")]
    pub additional_optimization_objective_config: ::core::option::Option<auto_ml_tables_inputs::AdditionalOptimizationObjectiveConfig>,
}
/// Nested message and enum types in `AutoMlTablesInputs`.
pub mod auto_ml_tables_inputs {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Transformation {
        /// The transformation that the training pipeline will apply to the input
        /// columns.
        #[prost(oneof="transformation::TransformationDetail", tags="1, 2, 3, 4, 5, 6, 7, 8")]
        pub transformation_detail: ::core::option::Option<transformation::TransformationDetail>,
    }
    /// Nested message and enum types in `Transformation`.
    pub mod transformation {
        /// Training pipeline will infer the proper transformation based on the
        /// statistic of dataset.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct AutoTransformation {
            #[prost(string, tag="1")]
            pub column_name: ::prost::alloc::string::String,
        }
        /// Training pipeline will perform following transformation functions.
        /// *  The value converted to float32.
        /// *  The z_score of the value.
        /// *  log(value+1) when the value is greater than or equal to 0. Otherwise,
        ///    this transformation is not applied and the value is considered a
        ///    missing value.
        /// *  z_score of log(value+1) when the value is greater than or equal to 0.
        ///    Otherwise, this transformation is not applied and the value is
        ///    considered a missing value.
        /// *  A boolean value that indicates whether the value is valid.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct NumericTransformation {
            #[prost(string, tag="1")]
            pub column_name: ::prost::alloc::string::String,
            /// If invalid values is allowed, the training pipeline will create a
            /// boolean feature that indicated whether the value is valid.
            /// Otherwise, the training pipeline will discard the input row from
            /// trainining data.
            #[prost(bool, tag="2")]
            pub invalid_values_allowed: bool,
        }
        /// Training pipeline will perform following transformation functions.
        /// *  The categorical string as is--no change to case, punctuation,
        /// spelling,
        ///    tense, and so on.
        /// *  Convert the category name to a dictionary lookup index and generate an
        ///    embedding for each index.
        /// *  Categories that appear less than 5 times in the training dataset are
        ///    treated as the "unknown" category. The "unknown" category gets its own
        ///    special lookup index and resulting embedding.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CategoricalTransformation {
            #[prost(string, tag="1")]
            pub column_name: ::prost::alloc::string::String,
        }
        /// Training pipeline will perform following transformation functions.
        /// *  Apply the transformation functions for Numerical columns.
        /// *  Determine the year, month, day,and weekday. Treat each value from the
        /// *  timestamp as a Categorical column.
        /// *  Invalid numerical values (for example, values that fall outside of a
        ///    typical timestamp range, or are extreme values) receive no special
        ///    treatment and are not removed.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TimestampTransformation {
            #[prost(string, tag="1")]
            pub column_name: ::prost::alloc::string::String,
            /// The format in which that time field is expressed. The time_format must
            /// either be one of:
            /// * `unix-seconds`
            /// * `unix-milliseconds`
            /// * `unix-microseconds`
            /// * `unix-nanoseconds`
            /// (for respectively number of seconds, milliseconds, microseconds and
            /// nanoseconds since start of the Unix epoch);
            /// or be written in `strftime` syntax. If time_format is not set, then the
            /// default format is RFC 3339 `date-time` format, where
            /// `time-offset` = `"Z"` (e.g. 1985-04-12T23:20:50.52Z)
            #[prost(string, tag="2")]
            pub time_format: ::prost::alloc::string::String,
            /// If invalid values is allowed, the training pipeline will create a
            /// boolean feature that indicated whether the value is valid.
            /// Otherwise, the training pipeline will discard the input row from
            /// trainining data.
            #[prost(bool, tag="3")]
            pub invalid_values_allowed: bool,
        }
        /// Training pipeline will perform following transformation functions.
        /// *  The text as is--no change to case, punctuation, spelling, tense, and
        /// so
        ///    on.
        /// *  Tokenize text to words. Convert each words to a dictionary lookup
        /// index
        ///    and generate an embedding for each index. Combine the embedding of all
        ///    elements into a single embedding using the mean.
        /// *  Tokenization is based on unicode script boundaries.
        /// *  Missing values get their own lookup index and resulting embedding.
        /// *  Stop-words receive no special treatment and are not removed.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TextTransformation {
            #[prost(string, tag="1")]
            pub column_name: ::prost::alloc::string::String,
        }
        /// Treats the column as numerical array and performs following
        /// transformation functions.
        /// *  All transformations for Numerical types applied to the average of the
        ///    all elements.
        /// *  The average of empty arrays is treated as zero.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct NumericArrayTransformation {
            #[prost(string, tag="1")]
            pub column_name: ::prost::alloc::string::String,
            /// If invalid values is allowed, the training pipeline will create a
            /// boolean feature that indicated whether the value is valid.
            /// Otherwise, the training pipeline will discard the input row from
            /// trainining data.
            #[prost(bool, tag="2")]
            pub invalid_values_allowed: bool,
        }
        /// Treats the column as categorical array and performs following
        /// transformation functions.
        /// *  For each element in the array, convert the category name to a
        /// dictionary
        ///    lookup index and generate an embedding for each index.
        ///    Combine the embedding of all elements into a single embedding using
        ///    the mean.
        /// *  Empty arrays treated as an embedding of zeroes.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CategoricalArrayTransformation {
            #[prost(string, tag="1")]
            pub column_name: ::prost::alloc::string::String,
        }
        /// Treats the column as text array and performs following transformation
        /// functions.
        /// *  Concatenate all text values in the array into a single text value
        /// using
        ///    a space (" ") as a delimiter, and then treat the result as a single
        ///    text value. Apply the transformations for Text columns.
        /// *  Empty arrays treated as an empty text.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TextArrayTransformation {
            #[prost(string, tag="1")]
            pub column_name: ::prost::alloc::string::String,
        }
        /// The transformation that the training pipeline will apply to the input
        /// columns.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum TransformationDetail {
            #[prost(message, tag="1")]
            Auto(AutoTransformation),
            #[prost(message, tag="2")]
            Numeric(NumericTransformation),
            #[prost(message, tag="3")]
            Categorical(CategoricalTransformation),
            #[prost(message, tag="4")]
            Timestamp(TimestampTransformation),
            #[prost(message, tag="5")]
            Text(TextTransformation),
            #[prost(message, tag="6")]
            RepeatedNumeric(NumericArrayTransformation),
            #[prost(message, tag="7")]
            RepeatedCategorical(CategoricalArrayTransformation),
            #[prost(message, tag="8")]
            RepeatedText(TextArrayTransformation),
        }
    }
    /// Additional optimization objective configuration. Required for
    /// `maximize-precision-at-recall` and `maximize-recall-at-precision`,
    /// otherwise unused.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AdditionalOptimizationObjectiveConfig {
        /// Required when optimization_objective is "maximize-precision-at-recall".
        /// Must be between 0 and 1, inclusive.
        #[prost(float, tag="5")]
        OptimizationObjectiveRecallValue(f32),
        /// Required when optimization_objective is "maximize-recall-at-precision".
        /// Must be between 0 and 1, inclusive.
        #[prost(float, tag="6")]
        OptimizationObjectivePrecisionValue(f32),
    }
}
/// Model metadata specific to AutoML Tables.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoMlTablesMetadata {
    /// Output only. The actual training cost of the model, expressed in milli
    /// node hours, i.e. 1,000 value in this field means 1 node hour. Guaranteed
    /// to not exceed the train budget.
    #[prost(int64, tag="1")]
    pub train_cost_milli_node_hours: i64,
}
/// A TrainingJob that trains and uploads an AutoML Text Extraction Model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoMlTextExtraction {
    /// The input parameters of this TrainingJob.
    #[prost(message, optional, tag="1")]
    pub inputs: ::core::option::Option<AutoMlTextExtractionInputs>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoMlTextExtractionInputs {
}
/// A TrainingJob that trains and uploads an AutoML Video Classification Model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoMlVideoClassification {
    /// The input parameters of this TrainingJob.
    #[prost(message, optional, tag="1")]
    pub inputs: ::core::option::Option<AutoMlVideoClassificationInputs>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoMlVideoClassificationInputs {
    #[prost(enumeration="auto_ml_video_classification_inputs::ModelType", tag="1")]
    pub model_type: i32,
}
/// Nested message and enum types in `AutoMlVideoClassificationInputs`.
pub mod auto_ml_video_classification_inputs {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ModelType {
        /// Should not be set.
        Unspecified = 0,
        /// A model best tailored to be used within Google Cloud, and which cannot
        /// be exported. Default.
        Cloud = 1,
        /// A model that, in addition to being available within Google Cloud, can
        /// also be exported (see ModelService.ExportModel) as a TensorFlow or
        /// TensorFlow Lite model and used on a mobile or edge device afterwards.
        MobileVersatile1 = 2,
        /// A model that, in addition to being available within Google Cloud, can
        /// also be exported (see ModelService.ExportModel) to a Jetson device
        /// afterwards.
        MobileJetsonVersatile1 = 3,
    }
}
/// A TrainingJob that trains and uploads an AutoML Video Action Recognition
/// Model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoMlVideoActionRecognition {
    /// The input parameters of this TrainingJob.
    #[prost(message, optional, tag="1")]
    pub inputs: ::core::option::Option<AutoMlVideoActionRecognitionInputs>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoMlVideoActionRecognitionInputs {
    #[prost(enumeration="auto_ml_video_action_recognition_inputs::ModelType", tag="1")]
    pub model_type: i32,
}
/// Nested message and enum types in `AutoMlVideoActionRecognitionInputs`.
pub mod auto_ml_video_action_recognition_inputs {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ModelType {
        /// Should not be set.
        Unspecified = 0,
        /// A model best tailored to be used within Google Cloud, and which c annot
        /// be exported. Default.
        Cloud = 1,
        /// A model that, in addition to being available within Google Cloud, can
        /// also be exported (see ModelService.ExportModel) as a TensorFlow or
        /// TensorFlow Lite model and used on a mobile or edge device afterwards.
        MobileVersatile1 = 2,
        /// A model that, in addition to being available within Google Cloud, can
        /// also be exported (see ModelService.ExportModel) to a Jetson device
        /// afterwards.
        MobileJetsonVersatile1 = 3,
        /// A model that, in addition to being available within Google Cloud, can
        /// also be exported (see ModelService.ExportModel) as a TensorFlow or
        /// TensorFlow Lite model and used on a Coral device afterwards.
        MobileCoralVersatile1 = 4,
    }
}
/// A TrainingJob that trains and uploads an AutoML Text Classification Model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoMlTextClassification {
    /// The input parameters of this TrainingJob.
    #[prost(message, optional, tag="1")]
    pub inputs: ::core::option::Option<AutoMlTextClassificationInputs>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoMlTextClassificationInputs {
    #[prost(bool, tag="1")]
    pub multi_label: bool,
}
/// A TrainingJob that trains and uploads an AutoML Image Classification Model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoMlImageClassification {
    /// The input parameters of this TrainingJob.
    #[prost(message, optional, tag="1")]
    pub inputs: ::core::option::Option<AutoMlImageClassificationInputs>,
    /// The metadata information.
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<AutoMlImageClassificationMetadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoMlImageClassificationInputs {
    #[prost(enumeration="auto_ml_image_classification_inputs::ModelType", tag="1")]
    pub model_type: i32,
    /// The ID of the `base` model. If it is specified, the new model will be
    /// trained based on the `base` model. Otherwise, the new model will be
    /// trained from scratch. The `base` model must be in the same
    /// Project and Location as the new Model to train, and have the same
    /// modelType.
    #[prost(string, tag="2")]
    pub base_model_id: ::prost::alloc::string::String,
    /// The training budget of creating this model, expressed in milli node
    /// hours i.e. 1,000 value in this field means 1 node hour. The actual
    /// metadata.costMilliNodeHours will be equal or less than this value.
    /// If further model training ceases to provide any improvements, it will
    /// stop without using the full budget and the metadata.successfulStopReason
    /// will be `model-converged`.
    /// Note, node_hour  = actual_hour * number_of_nodes_involved.
    /// For modelType `cloud`(default), the budget must be between 8,000
    /// and 800,000 milli node hours, inclusive. The default value is 192,000
    /// which represents one day in wall time, considering 8 nodes are used.
    /// For model types `mobile-tf-low-latency-1`, `mobile-tf-versatile-1`,
    /// `mobile-tf-high-accuracy-1`, the training budget must be between
    /// 1,000 and 100,000 milli node hours, inclusive.
    /// The default value is 24,000 which represents one day in wall time on a
    /// single node that is used.
    #[prost(int64, tag="3")]
    pub budget_milli_node_hours: i64,
    /// Use the entire training budget. This disables the early stopping feature.
    /// When false the early stopping feature is enabled, which means that
    /// AutoML Image Classification might stop training before the entire
    /// training budget has been used.
    #[prost(bool, tag="4")]
    pub disable_early_stopping: bool,
    /// If false, a single-label (multi-class) Model will be trained (i.e.
    /// assuming that for each image just up to one annotation may be
    /// applicable). If true, a multi-label Model will be trained (i.e.
    /// assuming that for each image multiple annotations may be applicable).
    #[prost(bool, tag="5")]
    pub multi_label: bool,
}
/// Nested message and enum types in `AutoMlImageClassificationInputs`.
pub mod auto_ml_image_classification_inputs {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ModelType {
        /// Should not be set.
        Unspecified = 0,
        /// A Model best tailored to be used within Google Cloud, and which cannot
        /// be exported.
        /// Default.
        Cloud = 1,
        /// A model that, in addition to being available within Google
        /// Cloud, can also be exported (see ModelService.ExportModel) as TensorFlow
        /// or Core ML model and used on a mobile or edge device afterwards.
        /// Expected to have low latency, but may have lower prediction
        /// quality than other mobile models.
        MobileTfLowLatency1 = 2,
        /// A model that, in addition to being available within Google
        /// Cloud, can also be exported (see ModelService.ExportModel) as TensorFlow
        /// or Core ML model and used on a mobile or edge device with afterwards.
        MobileTfVersatile1 = 3,
        /// A model that, in addition to being available within Google
        /// Cloud, can also be exported (see ModelService.ExportModel) as TensorFlow
        /// or Core ML model and used on a mobile or edge device afterwards.
        /// Expected to have a higher latency, but should also have a higher
        /// prediction quality than other mobile models.
        MobileTfHighAccuracy1 = 4,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoMlImageClassificationMetadata {
    /// The actual training cost of creating this model, expressed in
    /// milli node hours, i.e. 1,000 value in this field means 1 node hour.
    /// Guaranteed to not exceed inputs.budgetMilliNodeHours.
    #[prost(int64, tag="1")]
    pub cost_milli_node_hours: i64,
    /// For successful job completions, this is the reason why the job has
    /// finished.
    #[prost(enumeration="auto_ml_image_classification_metadata::SuccessfulStopReason", tag="2")]
    pub successful_stop_reason: i32,
}
/// Nested message and enum types in `AutoMlImageClassificationMetadata`.
pub mod auto_ml_image_classification_metadata {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SuccessfulStopReason {
        /// Should not be set.
        Unspecified = 0,
        /// The inputs.budgetMilliNodeHours had been reached.
        BudgetReached = 1,
        /// Further training of the Model ceased to increase its quality, since it
        /// already has converged.
        ModelConverged = 2,
    }
}
/// A TrainingJob that trains and uploads an AutoML Image Segmentation Model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoMlImageSegmentation {
    /// The input parameters of this TrainingJob.
    #[prost(message, optional, tag="1")]
    pub inputs: ::core::option::Option<AutoMlImageSegmentationInputs>,
    /// The metadata information.
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<AutoMlImageSegmentationMetadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoMlImageSegmentationInputs {
    #[prost(enumeration="auto_ml_image_segmentation_inputs::ModelType", tag="1")]
    pub model_type: i32,
    /// The training budget of creating this model, expressed in milli node
    /// hours i.e. 1,000 value in this field means 1 node hour. The actual
    /// metadata.costMilliNodeHours will be equal or less than this value.
    /// If further model training ceases to provide any improvements, it will
    /// stop without using the full budget and the metadata.successfulStopReason
    /// will be `model-converged`.
    /// Note, node_hour  = actual_hour * number_of_nodes_involved. Or
    /// actaul_wall_clock_hours = train_budget_milli_node_hours /
    ///                           (number_of_nodes_involved * 1000)
    /// For modelType `cloud-high-accuracy-1`(default), the budget must be between
    /// 20,000 and 2,000,000 milli node hours, inclusive. The default value is
    /// 192,000 which represents one day in wall time
    /// (1000 milli * 24 hours * 8 nodes).
    #[prost(int64, tag="2")]
    pub budget_milli_node_hours: i64,
    /// The ID of the `base` model. If it is specified, the new model will be
    /// trained based on the `base` model. Otherwise, the new model will be
    /// trained from scratch. The `base` model must be in the same
    /// Project and Location as the new Model to train, and have the same
    /// modelType.
    #[prost(string, tag="3")]
    pub base_model_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `AutoMlImageSegmentationInputs`.
pub mod auto_ml_image_segmentation_inputs {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ModelType {
        /// Should not be set.
        Unspecified = 0,
        /// A model to be used via prediction calls to uCAIP API. Expected
        /// to have a higher latency, but should also have a higher prediction
        /// quality than other models.
        CloudHighAccuracy1 = 1,
        /// A model to be used via prediction calls to uCAIP API. Expected
        /// to have a lower latency but relatively lower prediction quality.
        CloudLowAccuracy1 = 2,
        /// A model that, in addition to being available within Google
        /// Cloud, can also be exported (see ModelService.ExportModel) as TensorFlow
        /// model and used on a mobile or edge device afterwards.
        /// Expected to have low latency, but may have lower prediction
        /// quality than other mobile models.
        MobileTfLowLatency1 = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoMlImageSegmentationMetadata {
    /// The actual training cost of creating this model, expressed in
    /// milli node hours, i.e. 1,000 value in this field means 1 node hour.
    /// Guaranteed to not exceed inputs.budgetMilliNodeHours.
    #[prost(int64, tag="1")]
    pub cost_milli_node_hours: i64,
    /// For successful job completions, this is the reason why the job has
    /// finished.
    #[prost(enumeration="auto_ml_image_segmentation_metadata::SuccessfulStopReason", tag="2")]
    pub successful_stop_reason: i32,
}
/// Nested message and enum types in `AutoMlImageSegmentationMetadata`.
pub mod auto_ml_image_segmentation_metadata {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SuccessfulStopReason {
        /// Should not be set.
        Unspecified = 0,
        /// The inputs.budgetMilliNodeHours had been reached.
        BudgetReached = 1,
        /// Further training of the Model ceased to increase its quality, since it
        /// already has converged.
        ModelConverged = 2,
    }
}
/// A TrainingJob that trains and uploads an AutoML Text Sentiment Model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoMlTextSentiment {
    /// The input parameters of this TrainingJob.
    #[prost(message, optional, tag="1")]
    pub inputs: ::core::option::Option<AutoMlTextSentimentInputs>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoMlTextSentimentInputs {
    /// A sentiment is expressed as an integer ordinal, where higher value
    /// means a more positive sentiment. The range of sentiments that will be used
    /// is between 0 and sentimentMax (inclusive on both ends), and all the values
    /// in the range must be represented in the dataset before a model can be
    /// created.
    /// Only the Annotations with this sentimentMax will be used for training.
    /// sentimentMax value must be between 1 and 10 (inclusive).
    #[prost(int32, tag="1")]
    pub sentiment_max: i32,
}
