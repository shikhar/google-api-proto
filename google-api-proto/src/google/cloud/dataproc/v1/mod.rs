/// Describes an autoscaling policy for Dataproc cluster autoscaler.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoscalingPolicy {
    /// Required. The policy id.
    ///
    /// The id must contain only letters (a-z, A-Z), numbers (0-9),
    /// underscores (_), and hyphens (-). Cannot begin or end with underscore
    /// or hyphen. Must consist of between 3 and 50 characters.
    ///
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Output only. The "resource name" of the autoscaling policy, as described
    /// in <https://cloud.google.com/apis/design/resource_names.>
    ///
    /// * For `projects.regions.autoscalingPolicies`, the resource name of the
    ///   policy has the following format:
    ///   `projects/{project_id}/regions/{region}/autoscalingPolicies/{policy_id}`
    ///
    /// * For `projects.locations.autoscalingPolicies`, the resource name of the
    ///   policy has the following format:
    ///   `projects/{project_id}/locations/{location}/autoscalingPolicies/{policy_id}`
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// Required. Describes how the autoscaler will operate for primary workers.
    #[prost(message, optional, tag="4")]
    pub worker_config: ::core::option::Option<InstanceGroupAutoscalingPolicyConfig>,
    /// Optional. Describes how the autoscaler will operate for secondary workers.
    #[prost(message, optional, tag="5")]
    pub secondary_worker_config: ::core::option::Option<InstanceGroupAutoscalingPolicyConfig>,
    /// Optional. The labels to associate with this autoscaling policy.
    /// Label **keys** must contain 1 to 63 characters, and must conform to
    /// [RFC 1035](<https://www.ietf.org/rfc/rfc1035.txt>).
    /// Label **values** may be empty, but, if present, must contain 1 to 63
    /// characters, and must conform to [RFC
    /// 1035](<https://www.ietf.org/rfc/rfc1035.txt>). No more than 32 labels can be
    /// associated with an autoscaling policy.
    #[prost(btree_map="string, string", tag="6")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Autoscaling algorithm for policy.
    #[prost(oneof="autoscaling_policy::Algorithm", tags="3")]
    pub algorithm: ::core::option::Option<autoscaling_policy::Algorithm>,
}
/// Nested message and enum types in `AutoscalingPolicy`.
pub mod autoscaling_policy {
    /// Autoscaling algorithm for policy.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Algorithm {
        #[prost(message, tag="3")]
        BasicAlgorithm(super::BasicAutoscalingAlgorithm),
    }
}
/// Basic algorithm for autoscaling.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BasicAutoscalingAlgorithm {
    /// Optional. Duration between scaling events. A scaling period starts after
    /// the update operation from the previous event has completed.
    ///
    /// Bounds: [2m, 1d]. Default: 2m.
    #[prost(message, optional, tag="2")]
    pub cooldown_period: ::core::option::Option<::prost_types::Duration>,
    #[prost(oneof="basic_autoscaling_algorithm::Config", tags="1")]
    pub config: ::core::option::Option<basic_autoscaling_algorithm::Config>,
}
/// Nested message and enum types in `BasicAutoscalingAlgorithm`.
pub mod basic_autoscaling_algorithm {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Config {
        /// Required. YARN autoscaling configuration.
        #[prost(message, tag="1")]
        YarnConfig(super::BasicYarnAutoscalingConfig),
    }
}
/// Basic autoscaling configurations for YARN.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BasicYarnAutoscalingConfig {
    /// Required. Timeout for YARN graceful decommissioning of Node Managers.
    /// Specifies the duration to wait for jobs to complete before forcefully
    /// removing workers (and potentially interrupting jobs). Only applicable to
    /// downscaling operations.
    ///
    /// Bounds: [0s, 1d].
    #[prost(message, optional, tag="5")]
    pub graceful_decommission_timeout: ::core::option::Option<::prost_types::Duration>,
    /// Required. Fraction of average YARN pending memory in the last cooldown period
    /// for which to add workers. A scale-up factor of 1.0 will result in scaling
    /// up so that there is no pending memory remaining after the update (more
    /// aggressive scaling). A scale-up factor closer to 0 will result in a smaller
    /// magnitude of scaling up (less aggressive scaling).
    /// See [How autoscaling
    /// works](<https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/autoscaling#how_autoscaling_works>)
    /// for more information.
    ///
    /// Bounds: [0.0, 1.0].
    #[prost(double, tag="1")]
    pub scale_up_factor: f64,
    /// Required. Fraction of average YARN pending memory in the last cooldown period
    /// for which to remove workers. A scale-down factor of 1 will result in
    /// scaling down so that there is no available memory remaining after the
    /// update (more aggressive scaling). A scale-down factor of 0 disables
    /// removing workers, which can be beneficial for autoscaling a single job.
    /// See [How autoscaling
    /// works](<https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/autoscaling#how_autoscaling_works>)
    /// for more information.
    ///
    /// Bounds: [0.0, 1.0].
    #[prost(double, tag="2")]
    pub scale_down_factor: f64,
    /// Optional. Minimum scale-up threshold as a fraction of total cluster size
    /// before scaling occurs. For example, in a 20-worker cluster, a threshold of
    /// 0.1 means the autoscaler must recommend at least a 2-worker scale-up for
    /// the cluster to scale. A threshold of 0 means the autoscaler will scale up
    /// on any recommended change.
    ///
    /// Bounds: [0.0, 1.0]. Default: 0.0.
    #[prost(double, tag="3")]
    pub scale_up_min_worker_fraction: f64,
    /// Optional. Minimum scale-down threshold as a fraction of total cluster size
    /// before scaling occurs. For example, in a 20-worker cluster, a threshold of
    /// 0.1 means the autoscaler must recommend at least a 2 worker scale-down for
    /// the cluster to scale. A threshold of 0 means the autoscaler will scale down
    /// on any recommended change.
    ///
    /// Bounds: [0.0, 1.0]. Default: 0.0.
    #[prost(double, tag="4")]
    pub scale_down_min_worker_fraction: f64,
}
/// Configuration for the size bounds of an instance group, including its
/// proportional size to other groups.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceGroupAutoscalingPolicyConfig {
    /// Optional. Minimum number of instances for this group.
    ///
    /// Primary workers - Bounds: [2, max_instances]. Default: 2.
    /// Secondary workers - Bounds: [0, max_instances]. Default: 0.
    #[prost(int32, tag="1")]
    pub min_instances: i32,
    /// Required. Maximum number of instances for this group. Required for primary
    /// workers. Note that by default, clusters will not use secondary workers.
    /// Required for secondary workers if the minimum secondary instances is set.
    ///
    /// Primary workers - Bounds: [min_instances, ).
    /// Secondary workers - Bounds: [min_instances, ). Default: 0.
    #[prost(int32, tag="2")]
    pub max_instances: i32,
    /// Optional. Weight for the instance group, which is used to determine the
    /// fraction of total workers in the cluster from this instance group.
    /// For example, if primary workers have weight 2, and secondary workers have
    /// weight 1, the cluster will have approximately 2 primary workers for each
    /// secondary worker.
    ///
    /// The cluster may not reach the specified balance if constrained
    /// by min/max bounds or other autoscaling settings. For example, if
    /// `max_instances` for secondary workers is 0, then only primary workers will
    /// be added. The cluster can also be out of balance when created.
    ///
    /// If weight is not set on any instance group, the cluster will default to
    /// equal weight for all groups: the cluster will attempt to maintain an equal
    /// number of workers in each group within the configured size bounds for each
    /// group. If weight is set for one group only, the cluster will default to
    /// zero weight on the unset group. For example if weight is set only on
    /// primary workers, the cluster will use primary workers only and no
    /// secondary workers.
    #[prost(int32, tag="3")]
    pub weight: i32,
}
/// A request to create an autoscaling policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAutoscalingPolicyRequest {
    /// Required. The "resource name" of the region or location, as described
    /// in <https://cloud.google.com/apis/design/resource_names.>
    ///
    /// * For `projects.regions.autoscalingPolicies.create`, the resource name
    ///   of the region has the following format:
    ///   `projects/{project_id}/regions/{region}`
    ///
    /// * For `projects.locations.autoscalingPolicies.create`, the resource name
    ///   of the location has the following format:
    ///   `projects/{project_id}/locations/{location}`
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The autoscaling policy to create.
    #[prost(message, optional, tag="2")]
    pub policy: ::core::option::Option<AutoscalingPolicy>,
}
/// A request to fetch an autoscaling policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAutoscalingPolicyRequest {
    /// Required. The "resource name" of the autoscaling policy, as described
    /// in <https://cloud.google.com/apis/design/resource_names.>
    ///
    /// * For `projects.regions.autoscalingPolicies.get`, the resource name
    ///   of the policy has the following format:
    ///   `projects/{project_id}/regions/{region}/autoscalingPolicies/{policy_id}`
    ///
    /// * For `projects.locations.autoscalingPolicies.get`, the resource name
    ///   of the policy has the following format:
    ///   `projects/{project_id}/locations/{location}/autoscalingPolicies/{policy_id}`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to update an autoscaling policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAutoscalingPolicyRequest {
    /// Required. The updated autoscaling policy.
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<AutoscalingPolicy>,
}
/// A request to delete an autoscaling policy.
///
/// Autoscaling policies in use by one or more clusters will not be deleted.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAutoscalingPolicyRequest {
    /// Required. The "resource name" of the autoscaling policy, as described
    /// in <https://cloud.google.com/apis/design/resource_names.>
    ///
    /// * For `projects.regions.autoscalingPolicies.delete`, the resource name
    ///   of the policy has the following format:
    ///   `projects/{project_id}/regions/{region}/autoscalingPolicies/{policy_id}`
    ///
    /// * For `projects.locations.autoscalingPolicies.delete`, the resource name
    ///   of the policy has the following format:
    ///   `projects/{project_id}/locations/{location}/autoscalingPolicies/{policy_id}`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to list autoscaling policies in a project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAutoscalingPoliciesRequest {
    /// Required. The "resource name" of the region or location, as described
    /// in <https://cloud.google.com/apis/design/resource_names.>
    ///
    /// * For `projects.regions.autoscalingPolicies.list`, the resource name
    ///   of the region has the following format:
    ///   `projects/{project_id}/regions/{region}`
    ///
    /// * For `projects.locations.autoscalingPolicies.list`, the resource name
    ///   of the location has the following format:
    ///   `projects/{project_id}/locations/{location}`
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of results to return in each response.
    /// Must be less than or equal to 1000. Defaults to 100.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. The page token, returned by a previous call, to request the
    /// next page of results.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// A response to a request to list autoscaling policies in a project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAutoscalingPoliciesResponse {
    /// Output only. Autoscaling policies list.
    #[prost(message, repeated, tag="1")]
    pub policies: ::prost::alloc::vec::Vec<AutoscalingPolicy>,
    /// Output only. This token is included in the response if there are more
    /// results to fetch.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod autoscaling_policy_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// The API interface for managing autoscaling policies in the
    /// Dataproc API.
    #[derive(Debug, Clone)]
    pub struct AutoscalingPolicyServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AutoscalingPolicyServiceClient<T>
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
        ) -> AutoscalingPolicyServiceClient<InterceptedService<T, F>>
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
            AutoscalingPolicyServiceClient::new(
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
        /// Creates new autoscaling policy.
        pub async fn create_autoscaling_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAutoscalingPolicyRequest>,
        ) -> Result<tonic::Response<super::AutoscalingPolicy>, tonic::Status> {
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
                "/google.cloud.dataproc.v1.AutoscalingPolicyService/CreateAutoscalingPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates (replaces) autoscaling policy.
        ///
        /// Disabled check for update_mask, because all updates will be full
        /// replacements.
        pub async fn update_autoscaling_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAutoscalingPolicyRequest>,
        ) -> Result<tonic::Response<super::AutoscalingPolicy>, tonic::Status> {
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
                "/google.cloud.dataproc.v1.AutoscalingPolicyService/UpdateAutoscalingPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves autoscaling policy.
        pub async fn get_autoscaling_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAutoscalingPolicyRequest>,
        ) -> Result<tonic::Response<super::AutoscalingPolicy>, tonic::Status> {
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
                "/google.cloud.dataproc.v1.AutoscalingPolicyService/GetAutoscalingPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists autoscaling policies in the project.
        pub async fn list_autoscaling_policies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAutoscalingPoliciesRequest>,
        ) -> Result<
            tonic::Response<super::ListAutoscalingPoliciesResponse>,
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
                "/google.cloud.dataproc.v1.AutoscalingPolicyService/ListAutoscalingPolicies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes an autoscaling policy. It is an error to delete an autoscaling
        /// policy that is in use by one or more clusters.
        pub async fn delete_autoscaling_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAutoscalingPolicyRequest>,
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
                "/google.cloud.dataproc.v1.AutoscalingPolicyService/DeleteAutoscalingPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Runtime configuration for a workload.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeConfig {
    /// Optional. Version of the batch runtime.
    #[prost(string, tag="1")]
    pub version: ::prost::alloc::string::String,
    /// Optional. Optional custom container image for the job runtime environment. If
    /// not specified, a default container image will be used.
    #[prost(string, tag="2")]
    pub container_image: ::prost::alloc::string::String,
    /// Optional. A mapping of property names to values, which are used to configure workload
    /// execution.
    #[prost(btree_map="string, string", tag="3")]
    pub properties: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Environment configuration for a workload.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvironmentConfig {
    /// Optional. Execution configuration for a workload.
    #[prost(message, optional, tag="1")]
    pub execution_config: ::core::option::Option<ExecutionConfig>,
    /// Optional. Peripherals configuration that workload has access to.
    #[prost(message, optional, tag="2")]
    pub peripherals_config: ::core::option::Option<PeripheralsConfig>,
}
/// Execution configuration for a workload.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionConfig {
    /// Optional. Service account that used to execute workload.
    #[prost(string, tag="2")]
    pub service_account: ::prost::alloc::string::String,
    /// Optional. Tags used for network traffic control.
    #[prost(string, repeated, tag="6")]
    pub network_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The Cloud KMS key to use for encryption.
    #[prost(string, tag="7")]
    pub kms_key: ::prost::alloc::string::String,
    /// Network configuration for workload execution.
    #[prost(oneof="execution_config::Network", tags="4, 5")]
    pub network: ::core::option::Option<execution_config::Network>,
}
/// Nested message and enum types in `ExecutionConfig`.
pub mod execution_config {
    /// Network configuration for workload execution.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Network {
        /// Optional. Network URI to connect workload to.
        #[prost(string, tag="4")]
        NetworkUri(::prost::alloc::string::String),
        /// Optional. Subnetwork URI to connect workload to.
        #[prost(string, tag="5")]
        SubnetworkUri(::prost::alloc::string::String),
    }
}
/// Spark History Server configuration for the workload.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SparkHistoryServerConfig {
    /// Optional. Resource name of an existing Dataproc Cluster to act as a Spark History
    /// Server for the workload.
    ///
    /// Example:
    ///
    /// * `projects/\[project_id]/regions/[region]/clusters/[cluster_name\]`
    #[prost(string, tag="1")]
    pub dataproc_cluster: ::prost::alloc::string::String,
}
/// Auxiliary services configuration for a workload.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeripheralsConfig {
    /// Optional. Resource name of an existing Dataproc Metastore service.
    ///
    /// Example:
    ///
    /// * `projects/\[project_id]/locations/[region]/services/[service_id\]`
    #[prost(string, tag="1")]
    pub metastore_service: ::prost::alloc::string::String,
    /// Optional. The Spark History Server configuration for the workload.
    #[prost(message, optional, tag="2")]
    pub spark_history_server_config: ::core::option::Option<SparkHistoryServerConfig>,
}
/// Runtime information about workload execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeInfo {
    /// Output only. Map of remote access endpoints (such as web interfaces and APIs) to their
    /// URIs.
    #[prost(btree_map="string, string", tag="1")]
    pub endpoints: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. A URI pointing to the location of the stdout and stderr of the workload.
    #[prost(string, tag="2")]
    pub output_uri: ::prost::alloc::string::String,
    /// Output only. A URI pointing to the location of the diagnostics tarball.
    #[prost(string, tag="3")]
    pub diagnostic_output_uri: ::prost::alloc::string::String,
}
/// The cluster's GKE config.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GkeClusterConfig {
    /// Optional. A target GKE cluster to deploy to. It must be in the same project and
    /// region as the Dataproc cluster (the GKE cluster can be zonal or regional).
    /// Format: 'projects/{project}/locations/{location}/clusters/{cluster_id}'
    #[prost(string, tag="2")]
    pub gke_cluster_target: ::prost::alloc::string::String,
    /// Optional. GKE NodePools where workloads will be scheduled. At least one node pool
    /// must be assigned the 'default' role. Each role can be given to only a
    /// single NodePoolTarget. All NodePools must have the same location settings.
    /// If a nodePoolTarget is not specified, Dataproc constructs a default
    /// nodePoolTarget.
    #[prost(message, repeated, tag="3")]
    pub node_pool_target: ::prost::alloc::vec::Vec<GkeNodePoolTarget>,
}
/// The configuration for running the Dataproc cluster on Kubernetes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KubernetesClusterConfig {
    /// Optional. A namespace within the Kubernetes cluster to deploy into. If this namespace
    /// does not exist, it is created. If it exists, Dataproc
    /// verifies that another Dataproc VirtualCluster is not installed
    /// into it. If not specified, the name of the Dataproc Cluster is used.
    #[prost(string, tag="1")]
    pub kubernetes_namespace: ::prost::alloc::string::String,
    /// Optional. The software configuration for this Dataproc cluster running on Kubernetes.
    #[prost(message, optional, tag="3")]
    pub kubernetes_software_config: ::core::option::Option<KubernetesSoftwareConfig>,
    #[prost(oneof="kubernetes_cluster_config::Config", tags="2")]
    pub config: ::core::option::Option<kubernetes_cluster_config::Config>,
}
/// Nested message and enum types in `KubernetesClusterConfig`.
pub mod kubernetes_cluster_config {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Config {
        /// Required. The configuration for running the Dataproc cluster on GKE.
        #[prost(message, tag="2")]
        GkeClusterConfig(super::GkeClusterConfig),
    }
}
/// The software configuration for this Dataproc cluster running on Kubernetes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KubernetesSoftwareConfig {
    /// The components that should be installed in this Dataproc cluster. The key
    /// must be a string from the KubernetesComponent enumeration. The value is
    /// the version of the software to be installed.
    /// At least one entry must be specified.
    #[prost(btree_map="string, string", tag="1")]
    pub component_version: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// The properties to set on daemon config files.
    ///
    /// Property keys are specified in `prefix:property` format, for example
    /// `spark:spark.kubernetes.container.image`. The following are supported
    /// prefixes and their mappings:
    ///
    /// * spark:  `spark-defaults.conf`
    ///
    /// For more information, see [Cluster
    /// properties](<https://cloud.google.com/dataproc/docs/concepts/cluster-properties>).
    #[prost(btree_map="string, string", tag="2")]
    pub properties: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// GKE NodePools that Dataproc workloads run on.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GkeNodePoolTarget {
    /// Required. The target GKE NodePool.
    /// Format:
    /// 'projects/{project}/locations/{location}/clusters/{cluster}/nodePools/{node_pool}'
    #[prost(string, tag="1")]
    pub node_pool: ::prost::alloc::string::String,
    /// Required. The types of role for a GKE NodePool
    #[prost(enumeration="gke_node_pool_target::Role", repeated, packed="false", tag="2")]
    pub roles: ::prost::alloc::vec::Vec<i32>,
    /// Optional. The configuration for the GKE NodePool.
    ///
    /// If specified, Dataproc attempts to create a NodePool with the
    /// specified shape. If one with the same name already exists, it is
    /// verified against all specified fields. If a field differs, the
    /// virtual cluster creation will fail.
    ///
    /// If omitted, any NodePool with the specified name is used. If a
    /// NodePool with the specified name does not exist, Dataproc create a NodePool
    /// with default values.
    #[prost(message, optional, tag="3")]
    pub node_pool_config: ::core::option::Option<GkeNodePoolConfig>,
}
/// Nested message and enum types in `GkeNodePoolTarget`.
pub mod gke_node_pool_target {
    /// `Role` specifies whose tasks will run on the NodePool. The roles can be
    /// specific to workloads. Exactly one GkeNodePoolTarget within the
    /// VirtualCluster must have 'default' role, which is used to run all workloads
    /// that are not associated with a NodePool.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Role {
        /// Role is unspecified.
        Unspecified = 0,
        /// Any roles that are not directly assigned to a NodePool run on the
        /// `default` role's NodePool.
        Default = 1,
        /// Run controllers and webhooks.
        Controller = 2,
        /// Run spark driver.
        SparkDriver = 3,
        /// Run spark executors.
        SparkExecutor = 4,
    }
}
/// The configuration of a GKE NodePool used by a [Dataproc-on-GKE
/// cluster](<https://cloud.google.com/dataproc/docs/concepts/jobs/dataproc-gke#create-a-dataproc-on-gke-cluster>).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GkeNodePoolConfig {
    /// Optional. The node pool configuration.
    #[prost(message, optional, tag="2")]
    pub config: ::core::option::Option<gke_node_pool_config::GkeNodeConfig>,
    /// Optional. The list of Compute Engine
    /// \[zones\](<https://cloud.google.com/compute/docs/zones#available>) where
    /// NodePool's nodes will be located.
    ///
    /// **Note:** Currently, only one zone may be specified.
    ///
    /// If a location is not specified during NodePool creation, Dataproc will
    /// choose a location.
    #[prost(string, repeated, tag="13")]
    pub locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The autoscaler configuration for this NodePool. The autoscaler is enabled
    /// only when a valid configuration is present.
    #[prost(message, optional, tag="4")]
    pub autoscaling: ::core::option::Option<gke_node_pool_config::GkeNodePoolAutoscalingConfig>,
}
/// Nested message and enum types in `GkeNodePoolConfig`.
pub mod gke_node_pool_config {
    /// Parameters that describe cluster nodes.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GkeNodeConfig {
        /// Optional. The name of a Compute Engine [machine
        /// type](<https://cloud.google.com/compute/docs/machine-types>).
        #[prost(string, tag="1")]
        pub machine_type: ::prost::alloc::string::String,
        /// Optional. Whether the nodes are created as [preemptible VM
        /// instances](<https://cloud.google.com/compute/docs/instances/preemptible>).
        #[prost(bool, tag="10")]
        pub preemptible: bool,
        /// Optional. The number of local SSD disks to attach to the node, which is limited by
        /// the maximum number of disks allowable per zone (see [Adding Local
        /// SSDs](<https://cloud.google.com/compute/docs/disks/local-ssd>)).
        #[prost(int32, tag="7")]
        pub local_ssd_count: i32,
        /// Optional. A list of [hardware
        /// accelerators](<https://cloud.google.com/compute/docs/gpus>) to attach to
        /// each node.
        #[prost(message, repeated, tag="11")]
        pub accelerators: ::prost::alloc::vec::Vec<GkeNodePoolAcceleratorConfig>,
        /// Optional. [Minimum CPU
        /// platform](<https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform>)
        /// to be used by this instance. The instance may be scheduled on the
        /// specified or a newer CPU platform. Specify the friendly names of CPU
        /// platforms, such as "Intel Haswell"` or Intel Sandy Bridge".
        #[prost(string, tag="13")]
        pub min_cpu_platform: ::prost::alloc::string::String,
    }
    /// A GkeNodeConfigAcceleratorConfig represents a Hardware Accelerator request
    /// for a NodePool.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GkeNodePoolAcceleratorConfig {
        /// The number of accelerator cards exposed to an instance.
        #[prost(int64, tag="1")]
        pub accelerator_count: i64,
        /// The accelerator type resource namename (see GPUs on Compute Engine).
        #[prost(string, tag="2")]
        pub accelerator_type: ::prost::alloc::string::String,
    }
    /// GkeNodePoolAutoscaling contains information the cluster autoscaler needs to
    /// adjust the size of the node pool to the current cluster usage.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GkeNodePoolAutoscalingConfig {
        /// The minimum number of nodes in the NodePool. Must be >= 0 and <=
        /// max_node_count.
        #[prost(int32, tag="2")]
        pub min_node_count: i32,
        /// The maximum number of nodes in the NodePool. Must be >= min_node_count.
        /// **Note:** Quota must be sufficient to scale up the cluster.
        #[prost(int32, tag="3")]
        pub max_node_count: i32,
    }
}
/// Cluster components that can be activated.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Component {
    /// Unspecified component. Specifying this will cause Cluster creation to fail.
    Unspecified = 0,
    /// The Anaconda python distribution. The Anaconda component is not supported
    /// in the Dataproc
    /// <a
    /// href="/dataproc/docs/concepts/versioning/dataproc-release-2.0">2.0
    /// image</a>. The 2.0 image is pre-installed with Miniconda.
    Anaconda = 5,
    /// Docker
    Docker = 13,
    /// The Druid query engine. (alpha)
    Druid = 9,
    /// Flink
    Flink = 14,
    /// HBase. (beta)
    Hbase = 11,
    /// The Hive Web HCatalog (the REST service for accessing HCatalog).
    HiveWebhcat = 3,
    /// The Jupyter Notebook.
    Jupyter = 1,
    /// The Presto query engine.
    Presto = 6,
    /// The Ranger service.
    Ranger = 12,
    /// The Solr service.
    Solr = 10,
    /// The Zeppelin notebook.
    Zeppelin = 4,
    /// The Zookeeper service.
    Zookeeper = 8,
}
/// Actions in response to failure of a resource associated with a cluster.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FailureAction {
    /// When FailureAction is unspecified, failure action defaults to NO_ACTION.
    Unspecified = 0,
    /// Take no action on failure to create a cluster resource. NO_ACTION is the
    /// default.
    NoAction = 1,
    /// Delete the failed cluster resource.
    Delete = 2,
}
/// Describes the identifying information, config, and status of
/// a Dataproc cluster
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cluster {
    /// Required. The Google Cloud Platform project ID that the cluster belongs to.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. The cluster name. Cluster names within a project must be
    /// unique. Names of deleted clusters can be reused.
    #[prost(string, tag="2")]
    pub cluster_name: ::prost::alloc::string::String,
    /// Optional. The cluster config for a cluster of Compute Engine Instances.
    /// Note that Dataproc may set default values, and values may change
    /// when clusters are updated.
    #[prost(message, optional, tag="3")]
    pub config: ::core::option::Option<ClusterConfig>,
    /// Optional. The virtual cluster config, used when creating a Dataproc cluster that
    /// does not directly control the underlying compute resources, for example,
    /// when creating a [Dataproc-on-GKE
    /// cluster](<https://cloud.google.com/dataproc/docs/concepts/jobs/dataproc-gke#create-a-dataproc-on-gke-cluster>).
    /// Note that Dataproc may set default values, and values may change when
    /// clusters are updated. Exactly one of config or virtualClusterConfig must be
    /// specified.
    #[prost(message, optional, tag="10")]
    pub virtual_cluster_config: ::core::option::Option<VirtualClusterConfig>,
    /// Optional. The labels to associate with this cluster.
    /// Label **keys** must contain 1 to 63 characters, and must conform to
    /// [RFC 1035](<https://www.ietf.org/rfc/rfc1035.txt>).
    /// Label **values** may be empty, but, if present, must contain 1 to 63
    /// characters, and must conform to [RFC
    /// 1035](<https://www.ietf.org/rfc/rfc1035.txt>). No more than 32 labels can be
    /// associated with a cluster.
    #[prost(btree_map="string, string", tag="8")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. Cluster status.
    #[prost(message, optional, tag="4")]
    pub status: ::core::option::Option<ClusterStatus>,
    /// Output only. The previous cluster status.
    #[prost(message, repeated, tag="7")]
    pub status_history: ::prost::alloc::vec::Vec<ClusterStatus>,
    /// Output only. A cluster UUID (Unique Universal Identifier). Dataproc
    /// generates this value when it creates the cluster.
    #[prost(string, tag="6")]
    pub cluster_uuid: ::prost::alloc::string::String,
    /// Output only. Contains cluster daemon metrics such as HDFS and YARN stats.
    ///
    /// **Beta Feature**: This report is available for testing purposes only. It
    /// may be changed before final release.
    #[prost(message, optional, tag="9")]
    pub metrics: ::core::option::Option<ClusterMetrics>,
}
/// The cluster config.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterConfig {
    /// Optional. A Cloud Storage bucket used to stage job
    /// dependencies, config files, and job driver console output.
    /// If you do not specify a staging bucket, Cloud
    /// Dataproc will determine a Cloud Storage location (US,
    /// ASIA, or EU) for your cluster's staging bucket according to the
    /// Compute Engine zone where your cluster is deployed, and then create
    /// and manage this project-level, per-location bucket (see
    /// [Dataproc staging and temp
    /// buckets](<https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/staging-bucket>)).
    /// **This field requires a Cloud Storage bucket name, not a `gs://...` URI to
    /// a Cloud Storage bucket.**
    #[prost(string, tag="1")]
    pub config_bucket: ::prost::alloc::string::String,
    /// Optional. A Cloud Storage bucket used to store ephemeral cluster and jobs data,
    /// such as Spark and MapReduce history files.
    /// If you do not specify a temp bucket,
    /// Dataproc will determine a Cloud Storage location (US,
    /// ASIA, or EU) for your cluster's temp bucket according to the
    /// Compute Engine zone where your cluster is deployed, and then create
    /// and manage this project-level, per-location bucket. The default bucket has
    /// a TTL of 90 days, but you can use any TTL (or none) if you specify a
    /// bucket (see
    /// [Dataproc staging and temp
    /// buckets](<https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/staging-bucket>)).
    /// **This field requires a Cloud Storage bucket name, not a `gs://...` URI to
    /// a Cloud Storage bucket.**
    #[prost(string, tag="2")]
    pub temp_bucket: ::prost::alloc::string::String,
    /// Optional. The shared Compute Engine config settings for
    /// all instances in a cluster.
    #[prost(message, optional, tag="8")]
    pub gce_cluster_config: ::core::option::Option<GceClusterConfig>,
    /// Optional. The Compute Engine config settings for
    /// the cluster's master instance.
    #[prost(message, optional, tag="9")]
    pub master_config: ::core::option::Option<InstanceGroupConfig>,
    /// Optional. The Compute Engine config settings for
    /// the cluster's worker instances.
    #[prost(message, optional, tag="10")]
    pub worker_config: ::core::option::Option<InstanceGroupConfig>,
    /// Optional. The Compute Engine config settings for
    /// a cluster's secondary worker instances
    #[prost(message, optional, tag="12")]
    pub secondary_worker_config: ::core::option::Option<InstanceGroupConfig>,
    /// Optional. The config settings for cluster software.
    #[prost(message, optional, tag="13")]
    pub software_config: ::core::option::Option<SoftwareConfig>,
    /// Optional. Commands to execute on each node after config is
    /// completed. By default, executables are run on master and all worker nodes.
    /// You can test a node's `role` metadata to run an executable on
    /// a master or worker node, as shown below using `curl` (you can also use
    /// `wget`):
    ///
    ///     ROLE=$(curl -H Metadata-Flavor:Google
    ///     <http://metadata/computeMetadata/v1/instance/attributes/dataproc-role>)
    ///     if [[ "${ROLE}" == 'Master' ]]; then
    ///       ... master specific actions ...
    ///     else
    ///       ... worker specific actions ...
    ///     fi
    #[prost(message, repeated, tag="11")]
    pub initialization_actions: ::prost::alloc::vec::Vec<NodeInitializationAction>,
    /// Optional. Encryption settings for the cluster.
    #[prost(message, optional, tag="15")]
    pub encryption_config: ::core::option::Option<EncryptionConfig>,
    /// Optional. Autoscaling config for the policy associated with the cluster.
    /// Cluster does not autoscale if this field is unset.
    #[prost(message, optional, tag="18")]
    pub autoscaling_config: ::core::option::Option<AutoscalingConfig>,
    /// Optional. Security settings for the cluster.
    #[prost(message, optional, tag="16")]
    pub security_config: ::core::option::Option<SecurityConfig>,
    /// Optional. Lifecycle setting for the cluster.
    #[prost(message, optional, tag="17")]
    pub lifecycle_config: ::core::option::Option<LifecycleConfig>,
    /// Optional. Port/endpoint configuration for this cluster
    #[prost(message, optional, tag="19")]
    pub endpoint_config: ::core::option::Option<EndpointConfig>,
    /// Optional. Metastore configuration.
    #[prost(message, optional, tag="20")]
    pub metastore_config: ::core::option::Option<MetastoreConfig>,
}
/// Dataproc cluster config for a cluster that does not directly control the
/// underlying compute resources, such as a [Dataproc-on-GKE
/// cluster](<https://cloud.google.com/dataproc/docs/concepts/jobs/dataproc-gke#create-a-dataproc-on-gke-cluster>).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualClusterConfig {
    /// Optional. A Storage bucket used to stage job
    /// dependencies, config files, and job driver console output.
    /// If you do not specify a staging bucket, Cloud
    /// Dataproc will determine a Cloud Storage location (US,
    /// ASIA, or EU) for your cluster's staging bucket according to the
    /// Compute Engine zone where your cluster is deployed, and then create
    /// and manage this project-level, per-location bucket (see
    /// [Dataproc staging and temp
    /// buckets](<https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/staging-bucket>)).
    /// **This field requires a Cloud Storage bucket name, not a `gs://...` URI to
    /// a Cloud Storage bucket.**
    #[prost(string, tag="1")]
    pub staging_bucket: ::prost::alloc::string::String,
    /// Optional. Configuration of auxiliary services used by this cluster.
    #[prost(message, optional, tag="7")]
    pub auxiliary_services_config: ::core::option::Option<AuxiliaryServicesConfig>,
    #[prost(oneof="virtual_cluster_config::InfrastructureConfig", tags="6")]
    pub infrastructure_config: ::core::option::Option<virtual_cluster_config::InfrastructureConfig>,
}
/// Nested message and enum types in `VirtualClusterConfig`.
pub mod virtual_cluster_config {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum InfrastructureConfig {
        /// Required. The configuration for running the Dataproc cluster on Kubernetes.
        #[prost(message, tag="6")]
        KubernetesClusterConfig(super::KubernetesClusterConfig),
    }
}
/// Auxiliary services configuration for a Cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuxiliaryServicesConfig {
    /// Optional. The Hive Metastore configuration for this workload.
    #[prost(message, optional, tag="1")]
    pub metastore_config: ::core::option::Option<MetastoreConfig>,
    /// Optional. The Spark History Server configuration for the workload.
    #[prost(message, optional, tag="2")]
    pub spark_history_server_config: ::core::option::Option<SparkHistoryServerConfig>,
}
/// Endpoint config for this cluster
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndpointConfig {
    /// Output only. The map of port descriptions to URLs. Will only be populated
    /// if enable_http_port_access is true.
    #[prost(btree_map="string, string", tag="1")]
    pub http_ports: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. If true, enable http access to specific ports on the cluster
    /// from external sources. Defaults to false.
    #[prost(bool, tag="2")]
    pub enable_http_port_access: bool,
}
/// Autoscaling Policy config associated with the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoscalingConfig {
    /// Optional. The autoscaling policy used by the cluster.
    ///
    /// Only resource names including projectid and location (region) are valid.
    /// Examples:
    ///
    /// * `<https://www.googleapis.com/compute/v1/projects/\[project_id]/locations/[dataproc_region]/autoscalingPolicies/[policy_id\]`>
    /// * `projects/\[project_id]/locations/[dataproc_region]/autoscalingPolicies/[policy_id\]`
    ///
    /// Note that the policy must be in the same project and Dataproc region.
    #[prost(string, tag="1")]
    pub policy_uri: ::prost::alloc::string::String,
}
/// Encryption settings for the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptionConfig {
    /// Optional. The Cloud KMS key name to use for PD disk encryption for all
    /// instances in the cluster.
    #[prost(string, tag="1")]
    pub gce_pd_kms_key_name: ::prost::alloc::string::String,
}
/// Common config settings for resources of Compute Engine cluster
/// instances, applicable to all instances in the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GceClusterConfig {
    /// Optional. The zone where the Compute Engine cluster will be located.
    /// On a create request, it is required in the "global" region. If omitted
    /// in a non-global Dataproc region, the service will pick a zone in the
    /// corresponding Compute Engine region. On a get request, zone will
    /// always be present.
    ///
    /// A full URL, partial URI, or short name are valid. Examples:
    ///
    /// * `<https://www.googleapis.com/compute/v1/projects/\[project_id]/zones/[zone\]`>
    /// * `projects/\[project_id]/zones/[zone\]`
    /// * `us-central1-f`
    #[prost(string, tag="1")]
    pub zone_uri: ::prost::alloc::string::String,
    /// Optional. The Compute Engine network to be used for machine
    /// communications. Cannot be specified with subnetwork_uri. If neither
    /// `network_uri` nor `subnetwork_uri` is specified, the "default" network of
    /// the project is used, if it exists. Cannot be a "Custom Subnet Network" (see
    /// [Using Subnetworks](<https://cloud.google.com/compute/docs/subnetworks>) for
    /// more information).
    ///
    /// A full URL, partial URI, or short name are valid. Examples:
    ///
    /// * `<https://www.googleapis.com/compute/v1/projects/\[project_id\]/regions/global/default`>
    /// * `projects/\[project_id\]/regions/global/default`
    /// * `default`
    #[prost(string, tag="2")]
    pub network_uri: ::prost::alloc::string::String,
    /// Optional. The Compute Engine subnetwork to be used for machine
    /// communications. Cannot be specified with network_uri.
    ///
    /// A full URL, partial URI, or short name are valid. Examples:
    ///
    /// * `<https://www.googleapis.com/compute/v1/projects/\[project_id\]/regions/us-east1/subnetworks/sub0`>
    /// * `projects/\[project_id\]/regions/us-east1/subnetworks/sub0`
    /// * `sub0`
    #[prost(string, tag="6")]
    pub subnetwork_uri: ::prost::alloc::string::String,
    /// Optional. If true, all instances in the cluster will only have internal IP
    /// addresses. By default, clusters are not restricted to internal IP
    /// addresses, and will have ephemeral external IP addresses assigned to each
    /// instance. This `internal_ip_only` restriction can only be enabled for
    /// subnetwork enabled networks, and all off-cluster dependencies must be
    /// configured to be accessible without external IP addresses.
    #[prost(bool, tag="7")]
    pub internal_ip_only: bool,
    /// Optional. The type of IPv6 access for a cluster.
    #[prost(enumeration="gce_cluster_config::PrivateIpv6GoogleAccess", tag="12")]
    pub private_ipv6_google_access: i32,
    /// Optional. The [Dataproc service
    /// account](<https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/service-accounts#service_accounts_in_dataproc>)
    /// (also see [VM Data Plane
    /// identity](<https://cloud.google.com/dataproc/docs/concepts/iam/dataproc-principals#vm_service_account_data_plane_identity>))
    /// used by Dataproc cluster VM instances to access Google Cloud Platform
    /// services.
    ///
    /// If not specified, the
    /// [Compute Engine default service
    /// account](<https://cloud.google.com/compute/docs/access/service-accounts#default_service_account>)
    /// is used.
    #[prost(string, tag="8")]
    pub service_account: ::prost::alloc::string::String,
    /// Optional. The URIs of service account scopes to be included in
    /// Compute Engine instances. The following base set of scopes is always
    /// included:
    ///
    /// * <https://www.googleapis.com/auth/cloud.useraccounts.readonly>
    /// * <https://www.googleapis.com/auth/devstorage.read_write>
    /// * <https://www.googleapis.com/auth/logging.write>
    ///
    /// If no scopes are specified, the following defaults are also provided:
    ///
    /// * <https://www.googleapis.com/auth/bigquery>
    /// * <https://www.googleapis.com/auth/bigtable.admin.table>
    /// * <https://www.googleapis.com/auth/bigtable.data>
    /// * <https://www.googleapis.com/auth/devstorage.full_control>
    #[prost(string, repeated, tag="3")]
    pub service_account_scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The Compute Engine tags to add to all instances (see [Tagging
    /// instances](<https://cloud.google.com/compute/docs/label-or-tag-resources#tags>)).
    #[prost(string, repeated, tag="4")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The Compute Engine metadata entries to add to all instances (see
    /// [Project and instance
    /// metadata](<https://cloud.google.com/compute/docs/storing-retrieving-metadata#project_and_instance_metadata>)).
    #[prost(btree_map="string, string", tag="5")]
    pub metadata: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. Reservation Affinity for consuming Zonal reservation.
    #[prost(message, optional, tag="11")]
    pub reservation_affinity: ::core::option::Option<ReservationAffinity>,
    /// Optional. Node Group Affinity for sole-tenant clusters.
    #[prost(message, optional, tag="13")]
    pub node_group_affinity: ::core::option::Option<NodeGroupAffinity>,
    /// Optional. Shielded Instance Config for clusters using [Compute Engine Shielded
    /// VMs](<https://cloud.google.com/security/shielded-cloud/shielded-vm>).
    #[prost(message, optional, tag="14")]
    pub shielded_instance_config: ::core::option::Option<ShieldedInstanceConfig>,
    /// Optional. Confidential Instance Config for clusters using [Confidential
    /// VMs](<https://cloud.google.com/compute/confidential-vm/docs>).
    #[prost(message, optional, tag="15")]
    pub confidential_instance_config: ::core::option::Option<ConfidentialInstanceConfig>,
}
/// Nested message and enum types in `GceClusterConfig`.
pub mod gce_cluster_config {
    /// `PrivateIpv6GoogleAccess` controls whether and how Dataproc cluster nodes
    /// can communicate with Google Services through gRPC over IPv6.
    /// These values are directly mapped to corresponding values in the
    /// [Compute Engine Instance
    /// fields](<https://cloud.google.com/compute/docs/reference/rest/v1/instances>).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PrivateIpv6GoogleAccess {
        /// If unspecified, Compute Engine default behavior will apply, which
        /// is the same as \[INHERIT_FROM_SUBNETWORK][google.cloud.dataproc.v1.GceClusterConfig.PrivateIpv6GoogleAccess.INHERIT_FROM_SUBNETWORK\].
        Unspecified = 0,
        /// Private access to and from Google Services configuration
        /// inherited from the subnetwork configuration. This is the
        /// default Compute Engine behavior.
        InheritFromSubnetwork = 1,
        /// Enables outbound private IPv6 access to Google Services from the Dataproc
        /// cluster.
        Outbound = 2,
        /// Enables bidirectional private IPv6 access between Google Services and the
        /// Dataproc cluster.
        Bidirectional = 3,
    }
}
/// Node Group Affinity for clusters using sole-tenant node groups.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeGroupAffinity {
    /// Required. The URI of a
    /// sole-tenant [node group
    /// resource](<https://cloud.google.com/compute/docs/reference/rest/v1/nodeGroups>)
    /// that the cluster will be created on.
    ///
    /// A full URL, partial URI, or node group name are valid. Examples:
    ///
    /// * `<https://www.googleapis.com/compute/v1/projects/\[project_id\]/zones/us-central1-a/nodeGroups/node-group-1`>
    /// * `projects/\[project_id\]/zones/us-central1-a/nodeGroups/node-group-1`
    /// * `node-group-1`
    #[prost(string, tag="1")]
    pub node_group_uri: ::prost::alloc::string::String,
}
/// Shielded Instance Config for clusters using [Compute Engine Shielded
/// VMs](<https://cloud.google.com/security/shielded-cloud/shielded-vm>).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShieldedInstanceConfig {
    /// Optional. Defines whether instances have Secure Boot enabled.
    #[prost(bool, tag="1")]
    pub enable_secure_boot: bool,
    /// Optional. Defines whether instances have the vTPM enabled.
    #[prost(bool, tag="2")]
    pub enable_vtpm: bool,
    /// Optional. Defines whether instances have integrity monitoring enabled.
    #[prost(bool, tag="3")]
    pub enable_integrity_monitoring: bool,
}
/// Confidential Instance Config for clusters using [Confidential
/// VMs](<https://cloud.google.com/compute/confidential-vm/docs>)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfidentialInstanceConfig {
    /// Optional. Defines whether the instance should have confidential compute enabled.
    #[prost(bool, tag="1")]
    pub enable_confidential_compute: bool,
}
/// The config settings for Compute Engine resources in
/// an instance group, such as a master or worker group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceGroupConfig {
    /// Optional. The number of VM instances in the instance group.
    /// For [HA
    /// cluster](/dataproc/docs/concepts/configuring-clusters/high-availability)
    /// \[master_config\](#FIELDS.master_config) groups, **must be set to 3**.
    /// For standard cluster \[master_config\](#FIELDS.master_config) groups,
    /// **must be set to 1**.
    #[prost(int32, tag="1")]
    pub num_instances: i32,
    /// Output only. The list of instance names. Dataproc derives the names
    /// from `cluster_name`, `num_instances`, and the instance group.
    #[prost(string, repeated, tag="2")]
    pub instance_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The Compute Engine image resource used for cluster instances.
    ///
    /// The URI can represent an image or image family.
    ///
    /// Image examples:
    ///
    /// * `<https://www.googleapis.com/compute/beta/projects/\[project_id]/global/images/[image-id\]`>
    /// * `projects/\[project_id]/global/images/[image-id\]`
    /// * `image-id`
    ///
    /// Image family examples. Dataproc will use the most recent
    /// image from the family:
    ///
    /// * `<https://www.googleapis.com/compute/beta/projects/\[project_id]/global/images/family/[custom-image-family-name\]`>
    /// * `projects/\[project_id]/global/images/family/[custom-image-family-name\]`
    ///
    /// If the URI is unspecified, it will be inferred from
    /// `SoftwareConfig.image_version` or the system default.
    #[prost(string, tag="3")]
    pub image_uri: ::prost::alloc::string::String,
    /// Optional. The Compute Engine machine type used for cluster instances.
    ///
    /// A full URL, partial URI, or short name are valid. Examples:
    ///
    /// * `<https://www.googleapis.com/compute/v1/projects/\[project_id\]/zones/us-east1-a/machineTypes/n1-standard-2`>
    /// * `projects/\[project_id\]/zones/us-east1-a/machineTypes/n1-standard-2`
    /// * `n1-standard-2`
    ///
    /// **Auto Zone Exception**: If you are using the Dataproc
    /// [Auto Zone
    /// Placement](<https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/auto-zone#using_auto_zone_placement>)
    /// feature, you must use the short name of the machine type
    /// resource, for example, `n1-standard-2`.
    #[prost(string, tag="4")]
    pub machine_type_uri: ::prost::alloc::string::String,
    /// Optional. Disk option config settings.
    #[prost(message, optional, tag="5")]
    pub disk_config: ::core::option::Option<DiskConfig>,
    /// Output only. Specifies that this instance group contains preemptible
    /// instances.
    #[prost(bool, tag="6")]
    pub is_preemptible: bool,
    /// Optional. Specifies the preemptibility of the instance group.
    ///
    /// The default value for master and worker groups is
    /// `NON_PREEMPTIBLE`. This default cannot be changed.
    ///
    /// The default value for secondary instances is
    /// `PREEMPTIBLE`.
    #[prost(enumeration="instance_group_config::Preemptibility", tag="10")]
    pub preemptibility: i32,
    /// Output only. The config for Compute Engine Instance Group
    /// Manager that manages this group.
    /// This is only used for preemptible instance groups.
    #[prost(message, optional, tag="7")]
    pub managed_group_config: ::core::option::Option<ManagedGroupConfig>,
    /// Optional. The Compute Engine accelerator configuration for these
    /// instances.
    #[prost(message, repeated, tag="8")]
    pub accelerators: ::prost::alloc::vec::Vec<AcceleratorConfig>,
    /// Optional. Specifies the minimum cpu platform for the Instance Group.
    /// See [Dataproc -> Minimum CPU
    /// Platform](<https://cloud.google.com/dataproc/docs/concepts/compute/dataproc-min-cpu>).
    #[prost(string, tag="9")]
    pub min_cpu_platform: ::prost::alloc::string::String,
}
/// Nested message and enum types in `InstanceGroupConfig`.
pub mod instance_group_config {
    /// Controls the use of
    /// [preemptible instances]
    /// (<https://cloud.google.com/compute/docs/instances/preemptible>)
    /// within the group.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Preemptibility {
        /// Preemptibility is unspecified, the system will choose the
        /// appropriate setting for each instance group.
        Unspecified = 0,
        /// Instances are non-preemptible.
        ///
        /// This option is allowed for all instance groups and is the only valid
        /// value for Master and Worker instance groups.
        NonPreemptible = 1,
        /// Instances are preemptible.
        ///
        /// This option is allowed only for secondary worker groups.
        Preemptible = 2,
    }
}
/// Specifies the resources used to actively manage an instance group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManagedGroupConfig {
    /// Output only. The name of the Instance Template used for the Managed
    /// Instance Group.
    #[prost(string, tag="1")]
    pub instance_template_name: ::prost::alloc::string::String,
    /// Output only. The name of the Instance Group Manager for this group.
    #[prost(string, tag="2")]
    pub instance_group_manager_name: ::prost::alloc::string::String,
}
/// Specifies the type and number of accelerator cards attached to the instances
/// of an instance. See [GPUs on Compute
/// Engine](<https://cloud.google.com/compute/docs/gpus/>).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceleratorConfig {
    /// Full URL, partial URI, or short name of the accelerator type resource to
    /// expose to this instance. See
    /// [Compute Engine
    /// AcceleratorTypes](<https://cloud.google.com/compute/docs/reference/beta/acceleratorTypes>).
    ///
    /// Examples:
    ///
    /// * `<https://www.googleapis.com/compute/beta/projects/\[project_id\]/zones/us-east1-a/acceleratorTypes/nvidia-tesla-k80`>
    /// * `projects/\[project_id\]/zones/us-east1-a/acceleratorTypes/nvidia-tesla-k80`
    /// * `nvidia-tesla-k80`
    ///
    /// **Auto Zone Exception**: If you are using the Dataproc
    /// [Auto Zone
    /// Placement](<https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/auto-zone#using_auto_zone_placement>)
    /// feature, you must use the short name of the accelerator type
    /// resource, for example, `nvidia-tesla-k80`.
    #[prost(string, tag="1")]
    pub accelerator_type_uri: ::prost::alloc::string::String,
    /// The number of the accelerator cards of this type exposed to this instance.
    #[prost(int32, tag="2")]
    pub accelerator_count: i32,
}
/// Specifies the config of disk options for a group of VM instances.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiskConfig {
    /// Optional. Type of the boot disk (default is "pd-standard").
    /// Valid values: "pd-balanced" (Persistent Disk Balanced Solid State Drive),
    /// "pd-ssd" (Persistent Disk Solid State Drive),
    /// or "pd-standard" (Persistent Disk Hard Disk Drive).
    /// See [Disk types](<https://cloud.google.com/compute/docs/disks#disk-types>).
    #[prost(string, tag="3")]
    pub boot_disk_type: ::prost::alloc::string::String,
    /// Optional. Size in GB of the boot disk (default is 500GB).
    #[prost(int32, tag="1")]
    pub boot_disk_size_gb: i32,
    /// Optional. Number of attached SSDs, from 0 to 4 (default is 0).
    /// If SSDs are not attached, the boot disk is used to store runtime logs and
    /// \[HDFS\](<https://hadoop.apache.org/docs/r1.2.1/hdfs_user_guide.html>) data.
    /// If one or more SSDs are attached, this runtime bulk
    /// data is spread across them, and the boot disk contains only basic
    /// config and installed binaries.
    #[prost(int32, tag="2")]
    pub num_local_ssds: i32,
    /// Optional. Interface type of local SSDs (default is "scsi").
    /// Valid values: "scsi" (Small Computer System Interface),
    /// "nvme" (Non-Volatile Memory Express).
    /// See [local SSD
    /// performance](<https://cloud.google.com/compute/docs/disks/local-ssd#performance>).
    #[prost(string, tag="4")]
    pub local_ssd_interface: ::prost::alloc::string::String,
}
/// Specifies an executable to run on a fully configured node and a
/// timeout period for executable completion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeInitializationAction {
    /// Required. Cloud Storage URI of executable file.
    #[prost(string, tag="1")]
    pub executable_file: ::prost::alloc::string::String,
    /// Optional. Amount of time executable has to complete. Default is
    /// 10 minutes (see JSON representation of
    /// \[Duration\](<https://developers.google.com/protocol-buffers/docs/proto3#json>)).
    ///
    /// Cluster creation fails with an explanatory error message (the
    /// name of the executable that caused the error and the exceeded timeout
    /// period) if the executable is not completed at end of the timeout period.
    #[prost(message, optional, tag="2")]
    pub execution_timeout: ::core::option::Option<::prost_types::Duration>,
}
/// The status of a cluster and its instances.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterStatus {
    /// Output only. The cluster's state.
    #[prost(enumeration="cluster_status::State", tag="1")]
    pub state: i32,
    /// Optional. Output only. Details of cluster's state.
    #[prost(string, tag="2")]
    pub detail: ::prost::alloc::string::String,
    /// Output only. Time when this state was entered (see JSON representation of
    /// \[Timestamp\](<https://developers.google.com/protocol-buffers/docs/proto3#json>)).
    #[prost(message, optional, tag="3")]
    pub state_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Additional state information that includes
    /// status reported by the agent.
    #[prost(enumeration="cluster_status::Substate", tag="4")]
    pub substate: i32,
}
/// Nested message and enum types in `ClusterStatus`.
pub mod cluster_status {
    /// The cluster state.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The cluster state is unknown.
        Unknown = 0,
        /// The cluster is being created and set up. It is not ready for use.
        Creating = 1,
        /// The cluster is currently running and healthy. It is ready for use.
        ///
        /// **Note:** The cluster state changes from "creating" to "running" status
        /// after the master node(s), first two primary worker nodes (and the last
        /// primary worker node if primary workers > 2) are running.
        Running = 2,
        /// The cluster encountered an error. It is not ready for use.
        Error = 3,
        /// The cluster has encountered an error while being updated. Jobs can
        /// be submitted to the cluster, but the cluster cannot be updated.
        ErrorDueToUpdate = 9,
        /// The cluster is being deleted. It cannot be used.
        Deleting = 4,
        /// The cluster is being updated. It continues to accept and process jobs.
        Updating = 5,
        /// The cluster is being stopped. It cannot be used.
        Stopping = 6,
        /// The cluster is currently stopped. It is not ready for use.
        Stopped = 7,
        /// The cluster is being started. It is not ready for use.
        Starting = 8,
    }
    /// The cluster substate.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Substate {
        /// The cluster substate is unknown.
        Unspecified = 0,
        /// The cluster is known to be in an unhealthy state
        /// (for example, critical daemons are not running or HDFS capacity is
        /// exhausted).
        ///
        /// Applies to RUNNING state.
        Unhealthy = 1,
        /// The agent-reported status is out of date (may occur if
        /// Dataproc loses communication with Agent).
        ///
        /// Applies to RUNNING state.
        StaleStatus = 2,
    }
}
/// Security related configuration, including encryption, Kerberos, etc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityConfig {
    /// Optional. Kerberos related configuration.
    #[prost(message, optional, tag="1")]
    pub kerberos_config: ::core::option::Option<KerberosConfig>,
    /// Optional. Identity related configuration, including service account based
    /// secure multi-tenancy user mappings.
    #[prost(message, optional, tag="2")]
    pub identity_config: ::core::option::Option<IdentityConfig>,
}
/// Specifies Kerberos related configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KerberosConfig {
    /// Optional. Flag to indicate whether to Kerberize the cluster (default: false). Set
    /// this field to true to enable Kerberos on a cluster.
    #[prost(bool, tag="1")]
    pub enable_kerberos: bool,
    /// Optional. The Cloud Storage URI of a KMS encrypted file containing the root
    /// principal password.
    #[prost(string, tag="2")]
    pub root_principal_password_uri: ::prost::alloc::string::String,
    /// Optional. The uri of the KMS key used to encrypt various sensitive
    /// files.
    #[prost(string, tag="3")]
    pub kms_key_uri: ::prost::alloc::string::String,
    /// Optional. The Cloud Storage URI of the keystore file used for SSL
    /// encryption. If not provided, Dataproc will provide a self-signed
    /// certificate.
    #[prost(string, tag="4")]
    pub keystore_uri: ::prost::alloc::string::String,
    /// Optional. The Cloud Storage URI of the truststore file used for SSL
    /// encryption. If not provided, Dataproc will provide a self-signed
    /// certificate.
    #[prost(string, tag="5")]
    pub truststore_uri: ::prost::alloc::string::String,
    /// Optional. The Cloud Storage URI of a KMS encrypted file containing the
    /// password to the user provided keystore. For the self-signed certificate,
    /// this password is generated by Dataproc.
    #[prost(string, tag="6")]
    pub keystore_password_uri: ::prost::alloc::string::String,
    /// Optional. The Cloud Storage URI of a KMS encrypted file containing the
    /// password to the user provided key. For the self-signed certificate, this
    /// password is generated by Dataproc.
    #[prost(string, tag="7")]
    pub key_password_uri: ::prost::alloc::string::String,
    /// Optional. The Cloud Storage URI of a KMS encrypted file containing the
    /// password to the user provided truststore. For the self-signed certificate,
    /// this password is generated by Dataproc.
    #[prost(string, tag="8")]
    pub truststore_password_uri: ::prost::alloc::string::String,
    /// Optional. The remote realm the Dataproc on-cluster KDC will trust, should
    /// the user enable cross realm trust.
    #[prost(string, tag="9")]
    pub cross_realm_trust_realm: ::prost::alloc::string::String,
    /// Optional. The KDC (IP or hostname) for the remote trusted realm in a cross
    /// realm trust relationship.
    #[prost(string, tag="10")]
    pub cross_realm_trust_kdc: ::prost::alloc::string::String,
    /// Optional. The admin server (IP or hostname) for the remote trusted realm in
    /// a cross realm trust relationship.
    #[prost(string, tag="11")]
    pub cross_realm_trust_admin_server: ::prost::alloc::string::String,
    /// Optional. The Cloud Storage URI of a KMS encrypted file containing the
    /// shared password between the on-cluster Kerberos realm and the remote
    /// trusted realm, in a cross realm trust relationship.
    #[prost(string, tag="12")]
    pub cross_realm_trust_shared_password_uri: ::prost::alloc::string::String,
    /// Optional. The Cloud Storage URI of a KMS encrypted file containing the
    /// master key of the KDC database.
    #[prost(string, tag="13")]
    pub kdc_db_key_uri: ::prost::alloc::string::String,
    /// Optional. The lifetime of the ticket granting ticket, in hours.
    /// If not specified, or user specifies 0, then default value 10
    /// will be used.
    #[prost(int32, tag="14")]
    pub tgt_lifetime_hours: i32,
    /// Optional. The name of the on-cluster Kerberos realm.
    /// If not specified, the uppercased domain of hostnames will be the realm.
    #[prost(string, tag="15")]
    pub realm: ::prost::alloc::string::String,
}
/// Identity related configuration, including service account based
/// secure multi-tenancy user mappings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentityConfig {
    /// Required. Map of user to service account.
    #[prost(btree_map="string, string", tag="1")]
    pub user_service_account_mapping: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Specifies the selection and config of software inside the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SoftwareConfig {
    /// Optional. The version of software inside the cluster. It must be one of the
    /// supported [Dataproc
    /// Versions](<https://cloud.google.com/dataproc/docs/concepts/versioning/dataproc-versions#supported_dataproc_versions>),
    /// such as "1.2" (including a subminor version, such as "1.2.29"), or the
    /// ["preview"
    /// version](<https://cloud.google.com/dataproc/docs/concepts/versioning/dataproc-versions#other_versions>).
    /// If unspecified, it defaults to the latest Debian version.
    #[prost(string, tag="1")]
    pub image_version: ::prost::alloc::string::String,
    /// Optional. The properties to set on daemon config files.
    ///
    /// Property keys are specified in `prefix:property` format, for example
    /// `core:hadoop.tmp.dir`. The following are supported prefixes
    /// and their mappings:
    ///
    /// * capacity-scheduler: `capacity-scheduler.xml`
    /// * core:   `core-site.xml`
    /// * distcp: `distcp-default.xml`
    /// * hdfs:   `hdfs-site.xml`
    /// * hive:   `hive-site.xml`
    /// * mapred: `mapred-site.xml`
    /// * pig:    `pig.properties`
    /// * spark:  `spark-defaults.conf`
    /// * yarn:   `yarn-site.xml`
    ///
    /// For more information, see [Cluster
    /// properties](<https://cloud.google.com/dataproc/docs/concepts/cluster-properties>).
    #[prost(btree_map="string, string", tag="2")]
    pub properties: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. The set of components to activate on the cluster.
    #[prost(enumeration="Component", repeated, packed="false", tag="3")]
    pub optional_components: ::prost::alloc::vec::Vec<i32>,
}
/// Specifies the cluster auto-delete schedule configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LifecycleConfig {
    /// Optional. The duration to keep the cluster alive while idling (when no jobs
    /// are running). Passing this threshold will cause the cluster to be
    /// deleted. Minimum value is 5 minutes; maximum value is 14 days (see JSON
    /// representation of
    /// \[Duration\](<https://developers.google.com/protocol-buffers/docs/proto3#json>)).
    #[prost(message, optional, tag="1")]
    pub idle_delete_ttl: ::core::option::Option<::prost_types::Duration>,
    /// Output only. The time when cluster became idle (most recent job finished)
    /// and became eligible for deletion due to idleness (see JSON representation
    /// of
    /// \[Timestamp\](<https://developers.google.com/protocol-buffers/docs/proto3#json>)).
    #[prost(message, optional, tag="4")]
    pub idle_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Either the exact time the cluster should be deleted at or
    /// the cluster maximum age.
    #[prost(oneof="lifecycle_config::Ttl", tags="2, 3")]
    pub ttl: ::core::option::Option<lifecycle_config::Ttl>,
}
/// Nested message and enum types in `LifecycleConfig`.
pub mod lifecycle_config {
    /// Either the exact time the cluster should be deleted at or
    /// the cluster maximum age.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Ttl {
        /// Optional. The time when cluster will be auto-deleted (see JSON representation of
        /// \[Timestamp\](<https://developers.google.com/protocol-buffers/docs/proto3#json>)).
        #[prost(message, tag="2")]
        AutoDeleteTime(::prost_types::Timestamp),
        /// Optional. The lifetime duration of cluster. The cluster will be
        /// auto-deleted at the end of this period. Minimum value is 10 minutes;
        /// maximum value is 14 days (see JSON representation of
        /// \[Duration\](<https://developers.google.com/protocol-buffers/docs/proto3#json>)).
        #[prost(message, tag="3")]
        AutoDeleteTtl(::prost_types::Duration),
    }
}
/// Specifies a Metastore configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetastoreConfig {
    /// Required. Resource name of an existing Dataproc Metastore service.
    ///
    /// Example:
    ///
    /// * `projects/\[project_id]/locations/[dataproc_region]/services/[service-name\]`
    #[prost(string, tag="1")]
    pub dataproc_metastore_service: ::prost::alloc::string::String,
}
/// Contains cluster daemon metrics, such as HDFS and YARN stats.
///
/// **Beta Feature**: This report is available for testing purposes only. It may
/// be changed before final release.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterMetrics {
    /// The HDFS metrics.
    #[prost(btree_map="string, int64", tag="1")]
    pub hdfs_metrics: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, i64>,
    /// The YARN metrics.
    #[prost(btree_map="string, int64", tag="2")]
    pub yarn_metrics: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, i64>,
}
/// A request to create a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateClusterRequest {
    /// Required. The ID of the Google Cloud Platform project that the cluster
    /// belongs to.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. The Dataproc region in which to handle the request.
    #[prost(string, tag="3")]
    pub region: ::prost::alloc::string::String,
    /// Required. The cluster to create.
    #[prost(message, optional, tag="2")]
    pub cluster: ::core::option::Option<Cluster>,
    /// Optional. A unique ID used to identify the request. If the server receives two
    /// \[CreateClusterRequest\](<https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#google.cloud.dataproc.v1.CreateClusterRequest>)s
    /// with the same id, then the second request will be ignored and the
    /// first \[google.longrunning.Operation][google.longrunning.Operation\] created and stored in the backend
    /// is returned.
    ///
    /// It is recommended to always set this value to a
    /// \[UUID\](<https://en.wikipedia.org/wiki/Universally_unique_identifier>).
    ///
    /// The ID must contain only letters (a-z, A-Z), numbers (0-9),
    /// underscores (_), and hyphens (-). The maximum length is 40 characters.
    #[prost(string, tag="4")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. Failure action when primary worker creation fails.
    #[prost(enumeration="FailureAction", tag="5")]
    pub action_on_failed_primary_workers: i32,
}
/// A request to update a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateClusterRequest {
    /// Required. The ID of the Google Cloud Platform project the
    /// cluster belongs to.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. The Dataproc region in which to handle the request.
    #[prost(string, tag="5")]
    pub region: ::prost::alloc::string::String,
    /// Required. The cluster name.
    #[prost(string, tag="2")]
    pub cluster_name: ::prost::alloc::string::String,
    /// Required. The changes to the cluster.
    #[prost(message, optional, tag="3")]
    pub cluster: ::core::option::Option<Cluster>,
    /// Optional. Timeout for graceful YARN decomissioning. Graceful
    /// decommissioning allows removing nodes from the cluster without
    /// interrupting jobs in progress. Timeout specifies how long to wait for jobs
    /// in progress to finish before forcefully removing nodes (and potentially
    /// interrupting jobs). Default timeout is 0 (for forceful decommission), and
    /// the maximum allowed timeout is 1 day. (see JSON representation of
    /// \[Duration\](<https://developers.google.com/protocol-buffers/docs/proto3#json>)).
    ///
    /// Only supported on Dataproc image versions 1.2 and higher.
    #[prost(message, optional, tag="6")]
    pub graceful_decommission_timeout: ::core::option::Option<::prost_types::Duration>,
    /// Required. Specifies the path, relative to `Cluster`, of
    /// the field to update. For example, to change the number of workers
    /// in a cluster to 5, the `update_mask` parameter would be
    /// specified as `config.worker_config.num_instances`,
    /// and the `PATCH` request body would specify the new value, as follows:
    ///
    ///     {
    ///       "config":{
    ///         "workerConfig":{
    ///           "numInstances":"5"
    ///         }
    ///       }
    ///     }
    /// Similarly, to change the number of preemptible workers in a cluster to 5,
    /// the `update_mask` parameter would be
    /// `config.secondary_worker_config.num_instances`, and the `PATCH` request
    /// body would be set as follows:
    ///
    ///     {
    ///       "config":{
    ///         "secondaryWorkerConfig":{
    ///           "numInstances":"5"
    ///         }
    ///       }
    ///     }
    /// <strong>Note:</strong> Currently, only the following fields can be updated:
    ///
    ///  <table>
    ///  <tbody>
    ///  <tr>
    ///  <td><strong>Mask</strong></td>
    ///  <td><strong>Purpose</strong></td>
    ///  </tr>
    ///  <tr>
    ///  <td><strong><em>labels</em></strong></td>
    ///  <td>Update labels</td>
    ///  </tr>
    ///  <tr>
    ///  <td><strong><em>config.worker_config.num_instances</em></strong></td>
    ///  <td>Resize primary worker group</td>
    ///  </tr>
    ///  <tr>
    ///  <td><strong><em>config.secondary_worker_config.num_instances</em></strong></td>
    ///  <td>Resize secondary worker group</td>
    ///  </tr>
    ///  <tr>
    ///  <td>config.autoscaling_config.policy_uri</td><td>Use, stop using, or
    ///  change autoscaling policies</td>
    ///  </tr>
    ///  </tbody>
    ///  </table>
    #[prost(message, optional, tag="4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. A unique ID used to identify the request. If the server
    /// receives two
    /// \[UpdateClusterRequest\](<https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#google.cloud.dataproc.v1.UpdateClusterRequest>)s
    /// with the same id, then the second request will be ignored and the
    /// first \[google.longrunning.Operation][google.longrunning.Operation\] created and stored in the
    /// backend is returned.
    ///
    /// It is recommended to always set this value to a
    /// \[UUID\](<https://en.wikipedia.org/wiki/Universally_unique_identifier>).
    ///
    /// The ID must contain only letters (a-z, A-Z), numbers (0-9),
    /// underscores (_), and hyphens (-). The maximum length is 40 characters.
    #[prost(string, tag="7")]
    pub request_id: ::prost::alloc::string::String,
}
/// A request to stop a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopClusterRequest {
    /// Required. The ID of the Google Cloud Platform project the
    /// cluster belongs to.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. The Dataproc region in which to handle the request.
    #[prost(string, tag="2")]
    pub region: ::prost::alloc::string::String,
    /// Required. The cluster name.
    #[prost(string, tag="3")]
    pub cluster_name: ::prost::alloc::string::String,
    /// Optional. Specifying the `cluster_uuid` means the RPC will fail
    /// (with error NOT_FOUND) if a cluster with the specified UUID does not exist.
    #[prost(string, tag="4")]
    pub cluster_uuid: ::prost::alloc::string::String,
    /// Optional. A unique ID used to identify the request. If the server
    /// receives two
    /// \[StopClusterRequest\](<https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#google.cloud.dataproc.v1.StopClusterRequest>)s
    /// with the same id, then the second request will be ignored and the
    /// first \[google.longrunning.Operation][google.longrunning.Operation\] created and stored in the
    /// backend is returned.
    ///
    /// Recommendation: Set this value to a
    /// \[UUID\](<https://en.wikipedia.org/wiki/Universally_unique_identifier>).
    ///
    /// The ID must contain only letters (a-z, A-Z), numbers (0-9),
    /// underscores (_), and hyphens (-). The maximum length is 40 characters.
    #[prost(string, tag="5")]
    pub request_id: ::prost::alloc::string::String,
}
/// A request to start a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartClusterRequest {
    /// Required. The ID of the Google Cloud Platform project the
    /// cluster belongs to.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. The Dataproc region in which to handle the request.
    #[prost(string, tag="2")]
    pub region: ::prost::alloc::string::String,
    /// Required. The cluster name.
    #[prost(string, tag="3")]
    pub cluster_name: ::prost::alloc::string::String,
    /// Optional. Specifying the `cluster_uuid` means the RPC will fail
    /// (with error NOT_FOUND) if a cluster with the specified UUID does not exist.
    #[prost(string, tag="4")]
    pub cluster_uuid: ::prost::alloc::string::String,
    /// Optional. A unique ID used to identify the request. If the server
    /// receives two
    /// \[StartClusterRequest\](<https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#google.cloud.dataproc.v1.StartClusterRequest>)s
    /// with the same id, then the second request will be ignored and the
    /// first \[google.longrunning.Operation][google.longrunning.Operation\] created and stored in the
    /// backend is returned.
    ///
    /// Recommendation: Set this value to a
    /// \[UUID\](<https://en.wikipedia.org/wiki/Universally_unique_identifier>).
    ///
    /// The ID must contain only letters (a-z, A-Z), numbers (0-9),
    /// underscores (_), and hyphens (-). The maximum length is 40 characters.
    #[prost(string, tag="5")]
    pub request_id: ::prost::alloc::string::String,
}
/// A request to delete a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteClusterRequest {
    /// Required. The ID of the Google Cloud Platform project that the cluster
    /// belongs to.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. The Dataproc region in which to handle the request.
    #[prost(string, tag="3")]
    pub region: ::prost::alloc::string::String,
    /// Required. The cluster name.
    #[prost(string, tag="2")]
    pub cluster_name: ::prost::alloc::string::String,
    /// Optional. Specifying the `cluster_uuid` means the RPC should fail
    /// (with error NOT_FOUND) if cluster with specified UUID does not exist.
    #[prost(string, tag="4")]
    pub cluster_uuid: ::prost::alloc::string::String,
    /// Optional. A unique ID used to identify the request. If the server
    /// receives two
    /// \[DeleteClusterRequest\](<https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#google.cloud.dataproc.v1.DeleteClusterRequest>)s
    /// with the same id, then the second request will be ignored and the
    /// first \[google.longrunning.Operation][google.longrunning.Operation\] created and stored in the
    /// backend is returned.
    ///
    /// It is recommended to always set this value to a
    /// \[UUID\](<https://en.wikipedia.org/wiki/Universally_unique_identifier>).
    ///
    /// The ID must contain only letters (a-z, A-Z), numbers (0-9),
    /// underscores (_), and hyphens (-). The maximum length is 40 characters.
    #[prost(string, tag="5")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request to get the resource representation for a cluster in a project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClusterRequest {
    /// Required. The ID of the Google Cloud Platform project that the cluster
    /// belongs to.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. The Dataproc region in which to handle the request.
    #[prost(string, tag="3")]
    pub region: ::prost::alloc::string::String,
    /// Required. The cluster name.
    #[prost(string, tag="2")]
    pub cluster_name: ::prost::alloc::string::String,
}
/// A request to list the clusters in a project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClustersRequest {
    /// Required. The ID of the Google Cloud Platform project that the cluster
    /// belongs to.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. The Dataproc region in which to handle the request.
    #[prost(string, tag="4")]
    pub region: ::prost::alloc::string::String,
    /// Optional. A filter constraining the clusters to list. Filters are
    /// case-sensitive and have the following syntax:
    ///
    /// field = value [AND [field = value]] ...
    ///
    /// where **field** is one of `status.state`, `clusterName`, or `labels.\[KEY\]`,
    /// and `\[KEY\]` is a label key. **value** can be `*` to match all values.
    /// `status.state` can be one of the following: `ACTIVE`, `INACTIVE`,
    /// `CREATING`, `RUNNING`, `ERROR`, `DELETING`, or `UPDATING`. `ACTIVE`
    /// contains the `CREATING`, `UPDATING`, and `RUNNING` states. `INACTIVE`
    /// contains the `DELETING` and `ERROR` states.
    /// `clusterName` is the name of the cluster provided at creation time.
    /// Only the logical `AND` operator is supported; space-separated items are
    /// treated as having an implicit `AND` operator.
    ///
    /// Example filter:
    ///
    /// status.state = ACTIVE AND clusterName = mycluster
    /// AND labels.env = staging AND labels.starred = *
    #[prost(string, tag="5")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. The standard List page size.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. The standard List page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The list of all clusters in a project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClustersResponse {
    /// Output only. The clusters in the project.
    #[prost(message, repeated, tag="1")]
    pub clusters: ::prost::alloc::vec::Vec<Cluster>,
    /// Output only. This token is included in the response if there are more
    /// results to fetch. To fetch additional results, provide this value as the
    /// `page_token` in a subsequent `ListClustersRequest`.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A request to collect cluster diagnostic information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiagnoseClusterRequest {
    /// Required. The ID of the Google Cloud Platform project that the cluster
    /// belongs to.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. The Dataproc region in which to handle the request.
    #[prost(string, tag="3")]
    pub region: ::prost::alloc::string::String,
    /// Required. The cluster name.
    #[prost(string, tag="2")]
    pub cluster_name: ::prost::alloc::string::String,
}
/// The location of diagnostic output.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiagnoseClusterResults {
    /// Output only. The Cloud Storage URI of the diagnostic output.
    /// The output report is a plain text file with a summary of collected
    /// diagnostics.
    #[prost(string, tag="1")]
    pub output_uri: ::prost::alloc::string::String,
}
/// Reservation Affinity for consuming Zonal reservation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReservationAffinity {
    /// Optional. Type of reservation to consume
    #[prost(enumeration="reservation_affinity::Type", tag="1")]
    pub consume_reservation_type: i32,
    /// Optional. Corresponds to the label key of reservation resource.
    #[prost(string, tag="2")]
    pub key: ::prost::alloc::string::String,
    /// Optional. Corresponds to the label values of reservation resource.
    #[prost(string, repeated, tag="3")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `ReservationAffinity`.
pub mod reservation_affinity {
    /// Indicates whether to consume capacity from an reservation or not.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        Unspecified = 0,
        /// Do not consume from any allocated capacity.
        NoReservation = 1,
        /// Consume any reservation available.
        AnyReservation = 2,
        /// Must consume from a specific reservation. Must specify key value fields
        /// for specifying the reservations.
        SpecificReservation = 3,
    }
}
/// Generated client implementations.
pub mod cluster_controller_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// The ClusterControllerService provides methods to manage clusters
    /// of Compute Engine instances.
    #[derive(Debug, Clone)]
    pub struct ClusterControllerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ClusterControllerClient<T>
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
        ) -> ClusterControllerClient<InterceptedService<T, F>>
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
            ClusterControllerClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a cluster in a project. The returned
        /// [Operation.metadata][google.longrunning.Operation.metadata] will be
        /// [ClusterOperationMetadata](https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#clusteroperationmetadata).
        pub async fn create_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateClusterRequest>,
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
                "/google.cloud.dataproc.v1.ClusterController/CreateCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a cluster in a project. The returned
        /// [Operation.metadata][google.longrunning.Operation.metadata] will be
        /// [ClusterOperationMetadata](https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#clusteroperationmetadata).
        /// The cluster must be in a [`RUNNING`][google.cloud.dataproc.v1.ClusterStatus.State] state or an error
        /// is returned.
        pub async fn update_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateClusterRequest>,
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
                "/google.cloud.dataproc.v1.ClusterController/UpdateCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Stops a cluster in a project.
        pub async fn stop_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::StopClusterRequest>,
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
                "/google.cloud.dataproc.v1.ClusterController/StopCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Starts a cluster in a project.
        pub async fn start_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::StartClusterRequest>,
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
                "/google.cloud.dataproc.v1.ClusterController/StartCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a cluster in a project. The returned
        /// [Operation.metadata][google.longrunning.Operation.metadata] will be
        /// [ClusterOperationMetadata](https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#clusteroperationmetadata).
        pub async fn delete_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteClusterRequest>,
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
                "/google.cloud.dataproc.v1.ClusterController/DeleteCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the resource representation for a cluster in a project.
        pub async fn get_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::GetClusterRequest>,
        ) -> Result<tonic::Response<super::Cluster>, tonic::Status> {
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
                "/google.cloud.dataproc.v1.ClusterController/GetCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all regions/{region}/clusters in a project alphabetically.
        pub async fn list_clusters(
            &mut self,
            request: impl tonic::IntoRequest<super::ListClustersRequest>,
        ) -> Result<tonic::Response<super::ListClustersResponse>, tonic::Status> {
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
                "/google.cloud.dataproc.v1.ClusterController/ListClusters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets cluster diagnostic information. The returned
        /// [Operation.metadata][google.longrunning.Operation.metadata] will be
        /// [ClusterOperationMetadata](https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#clusteroperationmetadata).
        /// After the operation completes,
        /// [Operation.response][google.longrunning.Operation.response]
        /// contains
        /// [DiagnoseClusterResults](https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#diagnoseclusterresults).
        pub async fn diagnose_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::DiagnoseClusterRequest>,
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
                "/google.cloud.dataproc.v1.ClusterController/DiagnoseCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// A request to create a batch workload.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBatchRequest {
    /// Required. The parent resource where this batch will be created.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The batch to create.
    #[prost(message, optional, tag="2")]
    pub batch: ::core::option::Option<Batch>,
    /// Optional. The ID to use for the batch, which will become the final component of
    /// the batch's resource name.
    ///
    /// This value must be 4-63 characters. Valid characters are `/\[a-z][0-9\]-/`.
    #[prost(string, tag="3")]
    pub batch_id: ::prost::alloc::string::String,
    /// Optional. A unique ID used to identify the request. If the service
    /// receives two
    /// \[CreateBatchRequest\](<https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#google.cloud.dataproc.v1.CreateBatchRequest>)s
    /// with the same request_id, the second request is ignored and the
    /// Operation that corresponds to the first Batch created and stored
    /// in the backend is returned.
    ///
    /// Recommendation: Set this value to a
    /// \[UUID\](<https://en.wikipedia.org/wiki/Universally_unique_identifier>).
    ///
    /// The value must contain only letters (a-z, A-Z), numbers (0-9),
    /// underscores (_), and hyphens (-). The maximum length is 40 characters.
    #[prost(string, tag="4")]
    pub request_id: ::prost::alloc::string::String,
}
/// A request to get the resource representation for a batch workload.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBatchRequest {
    /// Required. The name of the batch to retrieve.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to list batch workloads in a project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBatchesRequest {
    /// Required. The parent, which owns this collection of batches.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of batches to return in each response.
    /// The service may return fewer than this value.
    /// The default page size is 20; the maximum page size is 1000.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. A page token received from a previous `ListBatches` call.
    /// Provide this token to retrieve the subsequent page.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// A list of batch workloads.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBatchesResponse {
    /// The batches from the specified collection.
    #[prost(message, repeated, tag="1")]
    pub batches: ::prost::alloc::vec::Vec<Batch>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A request to delete a batch workload.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBatchRequest {
    /// Required. The name of the batch resource to delete.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// A representation of a batch workload in the service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Batch {
    /// Output only. The resource name of the batch.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. A batch UUID (Unique Universal Identifier). The service
    /// generates this value when it creates the batch.
    #[prost(string, tag="2")]
    pub uuid: ::prost::alloc::string::String,
    /// Output only. The time when the batch was created.
    #[prost(message, optional, tag="3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Runtime information about batch execution.
    #[prost(message, optional, tag="8")]
    pub runtime_info: ::core::option::Option<RuntimeInfo>,
    /// Output only. The state of the batch.
    #[prost(enumeration="batch::State", tag="9")]
    pub state: i32,
    /// Output only. Batch state details, such as a failure
    /// description if the state is `FAILED`.
    #[prost(string, tag="10")]
    pub state_message: ::prost::alloc::string::String,
    /// Output only. The time when the batch entered a current state.
    #[prost(message, optional, tag="11")]
    pub state_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The email address of the user who created the batch.
    #[prost(string, tag="12")]
    pub creator: ::prost::alloc::string::String,
    /// Optional. The labels to associate with this batch.
    /// Label **keys** must contain 1 to 63 characters, and must conform to
    /// [RFC 1035](<https://www.ietf.org/rfc/rfc1035.txt>).
    /// Label **values** may be empty, but, if present, must contain 1 to 63
    /// characters, and must conform to [RFC
    /// 1035](<https://www.ietf.org/rfc/rfc1035.txt>). No more than 32 labels can be
    /// associated with a batch.
    #[prost(btree_map="string, string", tag="13")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. Runtime configuration for the batch execution.
    #[prost(message, optional, tag="14")]
    pub runtime_config: ::core::option::Option<RuntimeConfig>,
    /// Optional. Environment configuration for the batch execution.
    #[prost(message, optional, tag="15")]
    pub environment_config: ::core::option::Option<EnvironmentConfig>,
    /// Output only. The resource name of the operation associated with this batch.
    #[prost(string, tag="16")]
    pub operation: ::prost::alloc::string::String,
    /// Output only. Historical state information for the batch.
    #[prost(message, repeated, tag="17")]
    pub state_history: ::prost::alloc::vec::Vec<batch::StateHistory>,
    /// The application/framework-specific portion of the batch configuration.
    #[prost(oneof="batch::BatchConfig", tags="4, 5, 6, 7")]
    pub batch_config: ::core::option::Option<batch::BatchConfig>,
}
/// Nested message and enum types in `Batch`.
pub mod batch {
    /// Historical state information.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StateHistory {
        /// Output only. The state of the batch at this point in history.
        #[prost(enumeration="State", tag="1")]
        pub state: i32,
        /// Output only. Details about the state at this point in history.
        #[prost(string, tag="2")]
        pub state_message: ::prost::alloc::string::String,
        /// Output only. The time when the batch entered the historical state.
        #[prost(message, optional, tag="3")]
        pub state_start_time: ::core::option::Option<::prost_types::Timestamp>,
    }
    /// The batch state.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The batch state is unknown.
        Unspecified = 0,
        /// The batch is created before running.
        Pending = 1,
        /// The batch is running.
        Running = 2,
        /// The batch is cancelling.
        Cancelling = 3,
        /// The batch cancellation was successful.
        Cancelled = 4,
        /// The batch completed successfully.
        Succeeded = 5,
        /// The batch is no longer running due to an error.
        Failed = 6,
    }
    /// The application/framework-specific portion of the batch configuration.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum BatchConfig {
        /// Optional. PySpark batch config.
        #[prost(message, tag="4")]
        PysparkBatch(super::PySparkBatch),
        /// Optional. Spark batch config.
        #[prost(message, tag="5")]
        SparkBatch(super::SparkBatch),
        /// Optional. SparkR batch config.
        #[prost(message, tag="6")]
        SparkRBatch(super::SparkRBatch),
        /// Optional. SparkSql batch config.
        #[prost(message, tag="7")]
        SparkSqlBatch(super::SparkSqlBatch),
    }
}
/// A configuration for running an
/// [Apache
/// PySpark](<https://spark.apache.org/docs/latest/api/python/getting_started/quickstart.html>)
/// batch workload.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PySparkBatch {
    /// Required. The HCFS URI of the main Python file to use as the Spark driver. Must
    /// be a .py file.
    #[prost(string, tag="1")]
    pub main_python_file_uri: ::prost::alloc::string::String,
    /// Optional. The arguments to pass to the driver. Do not include arguments
    /// that can be set as batch properties, such as `--conf`, since a collision
    /// can occur that causes an incorrect batch submission.
    #[prost(string, repeated, tag="2")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. HCFS file URIs of Python files to pass to the PySpark
    /// framework. Supported file types: `.py`, `.egg`, and `.zip`.
    #[prost(string, repeated, tag="3")]
    pub python_file_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. HCFS URIs of jar files to add to the classpath of the
    /// Spark driver and tasks.
    #[prost(string, repeated, tag="4")]
    pub jar_file_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. HCFS URIs of files to be placed in the working directory of
    /// each executor.
    #[prost(string, repeated, tag="5")]
    pub file_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. HCFS URIs of archives to be extracted into the working directory
    /// of each executor. Supported file types:
    /// `.jar`, `.tar`, `.tar.gz`, `.tgz`, and `.zip`.
    #[prost(string, repeated, tag="6")]
    pub archive_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A configuration for running an [Apache Spark](<http://spark.apache.org/>)
/// batch workload.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SparkBatch {
    /// Optional. The arguments to pass to the driver. Do not include arguments
    /// that can be set as batch properties, such as `--conf`, since a collision
    /// can occur that causes an incorrect batch submission.
    #[prost(string, repeated, tag="3")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. HCFS URIs of jar files to add to the classpath of the
    /// Spark driver and tasks.
    #[prost(string, repeated, tag="4")]
    pub jar_file_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. HCFS URIs of files to be placed in the working directory of
    /// each executor.
    #[prost(string, repeated, tag="5")]
    pub file_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. HCFS URIs of archives to be extracted into the working directory
    /// of each executor. Supported file types:
    /// `.jar`, `.tar`, `.tar.gz`, `.tgz`, and `.zip`.
    #[prost(string, repeated, tag="6")]
    pub archive_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The specification of the main method to call to drive the Spark
    /// workload. Specify either the jar file that contains the main class or the
    /// main class name. To pass both a main jar and a main class in that jar, add
    /// the jar to `jar_file_uris`, and then specify the main class
    /// name in `main_class`.
    #[prost(oneof="spark_batch::Driver", tags="1, 2")]
    pub driver: ::core::option::Option<spark_batch::Driver>,
}
/// Nested message and enum types in `SparkBatch`.
pub mod spark_batch {
    /// The specification of the main method to call to drive the Spark
    /// workload. Specify either the jar file that contains the main class or the
    /// main class name. To pass both a main jar and a main class in that jar, add
    /// the jar to `jar_file_uris`, and then specify the main class
    /// name in `main_class`.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Driver {
        /// Optional. The HCFS URI of the jar file that contains the main class.
        #[prost(string, tag="1")]
        MainJarFileUri(::prost::alloc::string::String),
        /// Optional. The name of the driver main class. The jar file that contains the class
        /// must be in the classpath or specified in `jar_file_uris`.
        #[prost(string, tag="2")]
        MainClass(::prost::alloc::string::String),
    }
}
/// A configuration for running an
/// [Apache SparkR](<https://spark.apache.org/docs/latest/sparkr.html>)
/// batch workload.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SparkRBatch {
    /// Required. The HCFS URI of the main R file to use as the driver.
    /// Must be a `.R` or `.r` file.
    #[prost(string, tag="1")]
    pub main_r_file_uri: ::prost::alloc::string::String,
    /// Optional. The arguments to pass to the Spark driver. Do not include arguments
    /// that can be set as batch properties, such as `--conf`, since a collision
    /// can occur that causes an incorrect batch submission.
    #[prost(string, repeated, tag="2")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. HCFS URIs of files to be placed in the working directory of
    /// each executor.
    #[prost(string, repeated, tag="3")]
    pub file_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. HCFS URIs of archives to be extracted into the working directory
    /// of each executor. Supported file types:
    /// `.jar`, `.tar`, `.tar.gz`, `.tgz`, and `.zip`.
    #[prost(string, repeated, tag="4")]
    pub archive_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A configuration for running
/// [Apache Spark SQL](<http://spark.apache.org/sql/>) queries as a batch workload.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SparkSqlBatch {
    /// Required. The HCFS URI of the script that contains Spark SQL queries to execute.
    #[prost(string, tag="1")]
    pub query_file_uri: ::prost::alloc::string::String,
    /// Optional. Mapping of query variable names to values (equivalent to the
    /// Spark SQL command: `SET name="value";`).
    #[prost(btree_map="string, string", tag="2")]
    pub query_variables: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. HCFS URIs of jar files to be added to the Spark CLASSPATH.
    #[prost(string, repeated, tag="3")]
    pub jar_file_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Generated client implementations.
pub mod batch_controller_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// The BatchController provides methods to manage batch workloads.
    #[derive(Debug, Clone)]
    pub struct BatchControllerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BatchControllerClient<T>
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
        ) -> BatchControllerClient<InterceptedService<T, F>>
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
            BatchControllerClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a batch workload that executes asynchronously.
        pub async fn create_batch(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBatchRequest>,
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
                "/google.cloud.dataproc.v1.BatchController/CreateBatch",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the batch workload resource representation.
        pub async fn get_batch(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBatchRequest>,
        ) -> Result<tonic::Response<super::Batch>, tonic::Status> {
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
                "/google.cloud.dataproc.v1.BatchController/GetBatch",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists batch workloads.
        pub async fn list_batches(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBatchesRequest>,
        ) -> Result<tonic::Response<super::ListBatchesResponse>, tonic::Status> {
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
                "/google.cloud.dataproc.v1.BatchController/ListBatches",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes the batch workload resource. If the batch is not in terminal state,
        /// the delete fails and the response returns `FAILED_PRECONDITION`.
        pub async fn delete_batch(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBatchRequest>,
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
                "/google.cloud.dataproc.v1.BatchController/DeleteBatch",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Metadata describing the Batch operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchOperationMetadata {
    /// Name of the batch for the operation.
    #[prost(string, tag="1")]
    pub batch: ::prost::alloc::string::String,
    /// Batch UUID for the operation.
    #[prost(string, tag="2")]
    pub batch_uuid: ::prost::alloc::string::String,
    /// The time when the operation was created.
    #[prost(message, optional, tag="3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time when the operation finished.
    #[prost(message, optional, tag="4")]
    pub done_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The operation type.
    #[prost(enumeration="batch_operation_metadata::BatchOperationType", tag="6")]
    pub operation_type: i32,
    /// Short description of the operation.
    #[prost(string, tag="7")]
    pub description: ::prost::alloc::string::String,
    /// Labels associated with the operation.
    #[prost(btree_map="string, string", tag="8")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Warnings encountered during operation execution.
    #[prost(string, repeated, tag="9")]
    pub warnings: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `BatchOperationMetadata`.
pub mod batch_operation_metadata {
    /// Operation type for Batch resources
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BatchOperationType {
        /// Batch operation type is unknown.
        Unspecified = 0,
        /// Batch operation type.
        Batch = 1,
    }
}
/// The status of the operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterOperationStatus {
    /// Output only. A message containing the operation state.
    #[prost(enumeration="cluster_operation_status::State", tag="1")]
    pub state: i32,
    /// Output only. A message containing the detailed operation state.
    #[prost(string, tag="2")]
    pub inner_state: ::prost::alloc::string::String,
    /// Output only. A message containing any operation metadata details.
    #[prost(string, tag="3")]
    pub details: ::prost::alloc::string::String,
    /// Output only. The time this state was entered.
    #[prost(message, optional, tag="4")]
    pub state_start_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `ClusterOperationStatus`.
pub mod cluster_operation_status {
    /// The operation state.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unused.
        Unknown = 0,
        /// The operation has been created.
        Pending = 1,
        /// The operation is running.
        Running = 2,
        /// The operation is done; either cancelled or completed.
        Done = 3,
    }
}
/// Metadata describing the operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterOperationMetadata {
    /// Output only. Name of the cluster for the operation.
    #[prost(string, tag="7")]
    pub cluster_name: ::prost::alloc::string::String,
    /// Output only. Cluster UUID for the operation.
    #[prost(string, tag="8")]
    pub cluster_uuid: ::prost::alloc::string::String,
    /// Output only. Current operation status.
    #[prost(message, optional, tag="9")]
    pub status: ::core::option::Option<ClusterOperationStatus>,
    /// Output only. The previous operation status.
    #[prost(message, repeated, tag="10")]
    pub status_history: ::prost::alloc::vec::Vec<ClusterOperationStatus>,
    /// Output only. The operation type.
    #[prost(string, tag="11")]
    pub operation_type: ::prost::alloc::string::String,
    /// Output only. Short description of operation.
    #[prost(string, tag="12")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Labels associated with the operation
    #[prost(btree_map="string, string", tag="13")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. Errors encountered during operation execution.
    #[prost(string, repeated, tag="14")]
    pub warnings: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The runtime logging config of the job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoggingConfig {
    /// The per-package log levels for the driver. This may include
    /// "root" package name to configure rootLogger.
    /// Examples:
    ///   'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'
    #[prost(btree_map="string, enumeration(logging_config::Level)", tag="2")]
    pub driver_log_levels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, i32>,
}
/// Nested message and enum types in `LoggingConfig`.
pub mod logging_config {
    /// The Log4j level for job execution. When running an
    /// [Apache Hive](<https://hive.apache.org/>) job, Cloud
    /// Dataproc configures the Hive client to an equivalent verbosity level.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Level {
        /// Level is unspecified. Use default level for log4j.
        Unspecified = 0,
        /// Use ALL level for log4j.
        All = 1,
        /// Use TRACE level for log4j.
        Trace = 2,
        /// Use DEBUG level for log4j.
        Debug = 3,
        /// Use INFO level for log4j.
        Info = 4,
        /// Use WARN level for log4j.
        Warn = 5,
        /// Use ERROR level for log4j.
        Error = 6,
        /// Use FATAL level for log4j.
        Fatal = 7,
        /// Turn off log4j.
        Off = 8,
    }
}
/// A Dataproc job for running
/// [Apache Hadoop
/// MapReduce](<https://hadoop.apache.org/docs/current/hadoop-mapreduce-client/hadoop-mapreduce-client-core/MapReduceTutorial.html>)
/// jobs on [Apache Hadoop
/// YARN](<https://hadoop.apache.org/docs/r2.7.1/hadoop-yarn/hadoop-yarn-site/YARN.html>).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HadoopJob {
    /// Optional. The arguments to pass to the driver. Do not
    /// include arguments, such as `-libjars` or `-Dfoo=bar`, that can be set as
    /// job properties, since a collision may occur that causes an incorrect job
    /// submission.
    #[prost(string, repeated, tag="3")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Jar file URIs to add to the CLASSPATHs of the
    /// Hadoop driver and tasks.
    #[prost(string, repeated, tag="4")]
    pub jar_file_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. HCFS (Hadoop Compatible Filesystem) URIs of files to be copied
    /// to the working directory of Hadoop drivers and distributed tasks. Useful
    /// for naively parallel tasks.
    #[prost(string, repeated, tag="5")]
    pub file_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. HCFS URIs of archives to be extracted in the working directory of
    /// Hadoop drivers and tasks. Supported file types:
    /// .jar, .tar, .tar.gz, .tgz, or .zip.
    #[prost(string, repeated, tag="6")]
    pub archive_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. A mapping of property names to values, used to configure Hadoop.
    /// Properties that conflict with values set by the Dataproc API may be
    /// overwritten. Can include properties set in /etc/hadoop/conf/*-site and
    /// classes in user code.
    #[prost(btree_map="string, string", tag="7")]
    pub properties: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. The runtime log config for job execution.
    #[prost(message, optional, tag="8")]
    pub logging_config: ::core::option::Option<LoggingConfig>,
    /// Required. Indicates the location of the driver's main class. Specify
    /// either the jar file that contains the main class or the main class name.
    /// To specify both, add the jar file to `jar_file_uris`, and then specify
    /// the main class name in this property.
    #[prost(oneof="hadoop_job::Driver", tags="1, 2")]
    pub driver: ::core::option::Option<hadoop_job::Driver>,
}
/// Nested message and enum types in `HadoopJob`.
pub mod hadoop_job {
    /// Required. Indicates the location of the driver's main class. Specify
    /// either the jar file that contains the main class or the main class name.
    /// To specify both, add the jar file to `jar_file_uris`, and then specify
    /// the main class name in this property.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Driver {
        /// The HCFS URI of the jar file containing the main class.
        /// Examples:
        ///     'gs://foo-bucket/analytics-binaries/extract-useful-metrics-mr.jar'
        ///     'hdfs:/tmp/test-samples/custom-wordcount.jar'
        ///     'file:///home/usr/lib/hadoop-mapreduce/hadoop-mapreduce-examples.jar'
        #[prost(string, tag="1")]
        MainJarFileUri(::prost::alloc::string::String),
        /// The name of the driver's main class. The jar file containing the class
        /// must be in the default CLASSPATH or specified in `jar_file_uris`.
        #[prost(string, tag="2")]
        MainClass(::prost::alloc::string::String),
    }
}
/// A Dataproc job for running [Apache Spark](<http://spark.apache.org/>)
/// applications on YARN.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SparkJob {
    /// Optional. The arguments to pass to the driver. Do not include arguments,
    /// such as `--conf`, that can be set as job properties, since a collision may
    /// occur that causes an incorrect job submission.
    #[prost(string, repeated, tag="3")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. HCFS URIs of jar files to add to the CLASSPATHs of the
    /// Spark driver and tasks.
    #[prost(string, repeated, tag="4")]
    pub jar_file_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. HCFS URIs of files to be placed in the working directory of
    /// each executor. Useful for naively parallel tasks.
    #[prost(string, repeated, tag="5")]
    pub file_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. HCFS URIs of archives to be extracted into the working directory
    /// of each executor. Supported file types:
    /// .jar, .tar, .tar.gz, .tgz, and .zip.
    #[prost(string, repeated, tag="6")]
    pub archive_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. A mapping of property names to values, used to configure Spark.
    /// Properties that conflict with values set by the Dataproc API may be
    /// overwritten. Can include properties set in
    /// /etc/spark/conf/spark-defaults.conf and classes in user code.
    #[prost(btree_map="string, string", tag="7")]
    pub properties: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. The runtime log config for job execution.
    #[prost(message, optional, tag="8")]
    pub logging_config: ::core::option::Option<LoggingConfig>,
    /// Required. The specification of the main method to call to drive the job.
    /// Specify either the jar file that contains the main class or the main class
    /// name. To pass both a main jar and a main class in that jar, add the jar to
    /// `CommonJob.jar_file_uris`, and then specify the main class name in
    /// `main_class`.
    #[prost(oneof="spark_job::Driver", tags="1, 2")]
    pub driver: ::core::option::Option<spark_job::Driver>,
}
/// Nested message and enum types in `SparkJob`.
pub mod spark_job {
    /// Required. The specification of the main method to call to drive the job.
    /// Specify either the jar file that contains the main class or the main class
    /// name. To pass both a main jar and a main class in that jar, add the jar to
    /// `CommonJob.jar_file_uris`, and then specify the main class name in
    /// `main_class`.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Driver {
        /// The HCFS URI of the jar file that contains the main class.
        #[prost(string, tag="1")]
        MainJarFileUri(::prost::alloc::string::String),
        /// The name of the driver's main class. The jar file that contains the class
        /// must be in the default CLASSPATH or specified in `jar_file_uris`.
        #[prost(string, tag="2")]
        MainClass(::prost::alloc::string::String),
    }
}
/// A Dataproc job for running
/// [Apache
/// PySpark](<https://spark.apache.org/docs/0.9.0/python-programming-guide.html>)
/// applications on YARN.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PySparkJob {
    /// Required. The HCFS URI of the main Python file to use as the driver. Must
    /// be a .py file.
    #[prost(string, tag="1")]
    pub main_python_file_uri: ::prost::alloc::string::String,
    /// Optional. The arguments to pass to the driver.  Do not include arguments,
    /// such as `--conf`, that can be set as job properties, since a collision may
    /// occur that causes an incorrect job submission.
    #[prost(string, repeated, tag="2")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. HCFS file URIs of Python files to pass to the PySpark
    /// framework. Supported file types: .py, .egg, and .zip.
    #[prost(string, repeated, tag="3")]
    pub python_file_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. HCFS URIs of jar files to add to the CLASSPATHs of the
    /// Python driver and tasks.
    #[prost(string, repeated, tag="4")]
    pub jar_file_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. HCFS URIs of files to be placed in the working directory of
    /// each executor. Useful for naively parallel tasks.
    #[prost(string, repeated, tag="5")]
    pub file_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. HCFS URIs of archives to be extracted into the working directory
    /// of each executor. Supported file types:
    /// .jar, .tar, .tar.gz, .tgz, and .zip.
    #[prost(string, repeated, tag="6")]
    pub archive_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. A mapping of property names to values, used to configure PySpark.
    /// Properties that conflict with values set by the Dataproc API may be
    /// overwritten. Can include properties set in
    /// /etc/spark/conf/spark-defaults.conf and classes in user code.
    #[prost(btree_map="string, string", tag="7")]
    pub properties: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. The runtime log config for job execution.
    #[prost(message, optional, tag="8")]
    pub logging_config: ::core::option::Option<LoggingConfig>,
}
/// A list of queries to run on a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryList {
    /// Required. The queries to execute. You do not need to end a query expression
    /// with a semicolon. Multiple queries can be specified in one
    /// string by separating each with a semicolon. Here is an example of a
    /// Dataproc API snippet that uses a QueryList to specify a HiveJob:
    ///
    ///     "hiveJob": {
    ///       "queryList": {
    ///         "queries": [
    ///           "query1",
    ///           "query2",
    ///           "query3;query4",
    ///         ]
    ///       }
    ///     }
    #[prost(string, repeated, tag="1")]
    pub queries: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A Dataproc job for running [Apache Hive](<https://hive.apache.org/>)
/// queries on YARN.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HiveJob {
    /// Optional. Whether to continue executing queries if a query fails.
    /// The default value is `false`. Setting to `true` can be useful when
    /// executing independent parallel queries.
    #[prost(bool, tag="3")]
    pub continue_on_failure: bool,
    /// Optional. Mapping of query variable names to values (equivalent to the
    /// Hive command: `SET name="value";`).
    #[prost(btree_map="string, string", tag="4")]
    pub script_variables: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. A mapping of property names and values, used to configure Hive.
    /// Properties that conflict with values set by the Dataproc API may be
    /// overwritten. Can include properties set in /etc/hadoop/conf/*-site.xml,
    /// /etc/hive/conf/hive-site.xml, and classes in user code.
    #[prost(btree_map="string, string", tag="5")]
    pub properties: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. HCFS URIs of jar files to add to the CLASSPATH of the
    /// Hive server and Hadoop MapReduce (MR) tasks. Can contain Hive SerDes
    /// and UDFs.
    #[prost(string, repeated, tag="6")]
    pub jar_file_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The sequence of Hive queries to execute, specified as either
    /// an HCFS file URI or a list of queries.
    #[prost(oneof="hive_job::Queries", tags="1, 2")]
    pub queries: ::core::option::Option<hive_job::Queries>,
}
/// Nested message and enum types in `HiveJob`.
pub mod hive_job {
    /// Required. The sequence of Hive queries to execute, specified as either
    /// an HCFS file URI or a list of queries.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Queries {
        /// The HCFS URI of the script that contains Hive queries.
        #[prost(string, tag="1")]
        QueryFileUri(::prost::alloc::string::String),
        /// A list of queries.
        #[prost(message, tag="2")]
        QueryList(super::QueryList),
    }
}
/// A Dataproc job for running [Apache Spark
/// SQL](<http://spark.apache.org/sql/>) queries.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SparkSqlJob {
    /// Optional. Mapping of query variable names to values (equivalent to the
    /// Spark SQL command: SET `name="value";`).
    #[prost(btree_map="string, string", tag="3")]
    pub script_variables: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. A mapping of property names to values, used to configure
    /// Spark SQL's SparkConf. Properties that conflict with values set by the
    /// Dataproc API may be overwritten.
    #[prost(btree_map="string, string", tag="4")]
    pub properties: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. HCFS URIs of jar files to be added to the Spark CLASSPATH.
    #[prost(string, repeated, tag="56")]
    pub jar_file_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The runtime log config for job execution.
    #[prost(message, optional, tag="6")]
    pub logging_config: ::core::option::Option<LoggingConfig>,
    /// Required. The sequence of Spark SQL queries to execute, specified as
    /// either an HCFS file URI or as a list of queries.
    #[prost(oneof="spark_sql_job::Queries", tags="1, 2")]
    pub queries: ::core::option::Option<spark_sql_job::Queries>,
}
/// Nested message and enum types in `SparkSqlJob`.
pub mod spark_sql_job {
    /// Required. The sequence of Spark SQL queries to execute, specified as
    /// either an HCFS file URI or as a list of queries.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Queries {
        /// The HCFS URI of the script that contains SQL queries.
        #[prost(string, tag="1")]
        QueryFileUri(::prost::alloc::string::String),
        /// A list of queries.
        #[prost(message, tag="2")]
        QueryList(super::QueryList),
    }
}
/// A Dataproc job for running [Apache Pig](<https://pig.apache.org/>)
/// queries on YARN.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PigJob {
    /// Optional. Whether to continue executing queries if a query fails.
    /// The default value is `false`. Setting to `true` can be useful when
    /// executing independent parallel queries.
    #[prost(bool, tag="3")]
    pub continue_on_failure: bool,
    /// Optional. Mapping of query variable names to values (equivalent to the Pig
    /// command: `name=\[value\]`).
    #[prost(btree_map="string, string", tag="4")]
    pub script_variables: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. A mapping of property names to values, used to configure Pig.
    /// Properties that conflict with values set by the Dataproc API may be
    /// overwritten. Can include properties set in /etc/hadoop/conf/*-site.xml,
    /// /etc/pig/conf/pig.properties, and classes in user code.
    #[prost(btree_map="string, string", tag="5")]
    pub properties: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. HCFS URIs of jar files to add to the CLASSPATH of
    /// the Pig Client and Hadoop MapReduce (MR) tasks. Can contain Pig UDFs.
    #[prost(string, repeated, tag="6")]
    pub jar_file_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The runtime log config for job execution.
    #[prost(message, optional, tag="7")]
    pub logging_config: ::core::option::Option<LoggingConfig>,
    /// Required. The sequence of Pig queries to execute, specified as an HCFS
    /// file URI or a list of queries.
    #[prost(oneof="pig_job::Queries", tags="1, 2")]
    pub queries: ::core::option::Option<pig_job::Queries>,
}
/// Nested message and enum types in `PigJob`.
pub mod pig_job {
    /// Required. The sequence of Pig queries to execute, specified as an HCFS
    /// file URI or a list of queries.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Queries {
        /// The HCFS URI of the script that contains the Pig queries.
        #[prost(string, tag="1")]
        QueryFileUri(::prost::alloc::string::String),
        /// A list of queries.
        #[prost(message, tag="2")]
        QueryList(super::QueryList),
    }
}
/// A Dataproc job for running
/// [Apache SparkR](<https://spark.apache.org/docs/latest/sparkr.html>)
/// applications on YARN.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SparkRJob {
    /// Required. The HCFS URI of the main R file to use as the driver.
    /// Must be a .R file.
    #[prost(string, tag="1")]
    pub main_r_file_uri: ::prost::alloc::string::String,
    /// Optional. The arguments to pass to the driver.  Do not include arguments,
    /// such as `--conf`, that can be set as job properties, since a collision may
    /// occur that causes an incorrect job submission.
    #[prost(string, repeated, tag="2")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. HCFS URIs of files to be placed in the working directory of
    /// each executor. Useful for naively parallel tasks.
    #[prost(string, repeated, tag="3")]
    pub file_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. HCFS URIs of archives to be extracted into the working directory
    /// of each executor. Supported file types:
    /// .jar, .tar, .tar.gz, .tgz, and .zip.
    #[prost(string, repeated, tag="4")]
    pub archive_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. A mapping of property names to values, used to configure SparkR.
    /// Properties that conflict with values set by the Dataproc API may be
    /// overwritten. Can include properties set in
    /// /etc/spark/conf/spark-defaults.conf and classes in user code.
    #[prost(btree_map="string, string", tag="5")]
    pub properties: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. The runtime log config for job execution.
    #[prost(message, optional, tag="6")]
    pub logging_config: ::core::option::Option<LoggingConfig>,
}
/// A Dataproc job for running \[Presto\](<https://prestosql.io/>) queries.
/// **IMPORTANT**: The [Dataproc Presto Optional
/// Component](<https://cloud.google.com/dataproc/docs/concepts/components/presto>)
/// must be enabled when the cluster is created to submit a Presto job to the
/// cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrestoJob {
    /// Optional. Whether to continue executing queries if a query fails.
    /// The default value is `false`. Setting to `true` can be useful when
    /// executing independent parallel queries.
    #[prost(bool, tag="3")]
    pub continue_on_failure: bool,
    /// Optional. The format in which query output will be displayed. See the
    /// Presto documentation for supported output formats
    #[prost(string, tag="4")]
    pub output_format: ::prost::alloc::string::String,
    /// Optional. Presto client tags to attach to this query
    #[prost(string, repeated, tag="5")]
    pub client_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. A mapping of property names to values. Used to set Presto
    /// [session properties](<https://prestodb.io/docs/current/sql/set-session.html>)
    /// Equivalent to using the --session flag in the Presto CLI
    #[prost(btree_map="string, string", tag="6")]
    pub properties: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. The runtime log config for job execution.
    #[prost(message, optional, tag="7")]
    pub logging_config: ::core::option::Option<LoggingConfig>,
    /// Required. The sequence of Presto queries to execute, specified as
    /// either an HCFS file URI or as a list of queries.
    #[prost(oneof="presto_job::Queries", tags="1, 2")]
    pub queries: ::core::option::Option<presto_job::Queries>,
}
/// Nested message and enum types in `PrestoJob`.
pub mod presto_job {
    /// Required. The sequence of Presto queries to execute, specified as
    /// either an HCFS file URI or as a list of queries.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Queries {
        /// The HCFS URI of the script that contains SQL queries.
        #[prost(string, tag="1")]
        QueryFileUri(::prost::alloc::string::String),
        /// A list of queries.
        #[prost(message, tag="2")]
        QueryList(super::QueryList),
    }
}
/// Dataproc job config.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobPlacement {
    /// Required. The name of the cluster where the job will be submitted.
    #[prost(string, tag="1")]
    pub cluster_name: ::prost::alloc::string::String,
    /// Output only. A cluster UUID generated by the Dataproc service when
    /// the job is submitted.
    #[prost(string, tag="2")]
    pub cluster_uuid: ::prost::alloc::string::String,
    /// Optional. Cluster labels to identify a cluster where the job will be submitted.
    #[prost(btree_map="string, string", tag="3")]
    pub cluster_labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Dataproc job status.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobStatus {
    /// Output only. A state message specifying the overall job state.
    #[prost(enumeration="job_status::State", tag="1")]
    pub state: i32,
    /// Optional. Output only. Job state details, such as an error
    /// description if the state is <code>ERROR</code>.
    #[prost(string, tag="2")]
    pub details: ::prost::alloc::string::String,
    /// Output only. The time when this state was entered.
    #[prost(message, optional, tag="6")]
    pub state_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Additional state information, which includes
    /// status reported by the agent.
    #[prost(enumeration="job_status::Substate", tag="7")]
    pub substate: i32,
}
/// Nested message and enum types in `JobStatus`.
pub mod job_status {
    /// The job state.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The job state is unknown.
        Unspecified = 0,
        /// The job is pending; it has been submitted, but is not yet running.
        Pending = 1,
        /// Job has been received by the service and completed initial setup;
        /// it will soon be submitted to the cluster.
        SetupDone = 8,
        /// The job is running on the cluster.
        Running = 2,
        /// A CancelJob request has been received, but is pending.
        CancelPending = 3,
        /// Transient in-flight resources have been canceled, and the request to
        /// cancel the running job has been issued to the cluster.
        CancelStarted = 7,
        /// The job cancellation was successful.
        Cancelled = 4,
        /// The job has completed successfully.
        Done = 5,
        /// The job has completed, but encountered an error.
        Error = 6,
        /// Job attempt has failed. The detail field contains failure details for
        /// this attempt.
        ///
        /// Applies to restartable jobs only.
        AttemptFailure = 9,
    }
    /// The job substate.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Substate {
        /// The job substate is unknown.
        Unspecified = 0,
        /// The Job is submitted to the agent.
        ///
        /// Applies to RUNNING state.
        Submitted = 1,
        /// The Job has been received and is awaiting execution (it may be waiting
        /// for a condition to be met). See the "details" field for the reason for
        /// the delay.
        ///
        /// Applies to RUNNING state.
        Queued = 2,
        /// The agent-reported status is out of date, which may be caused by a
        /// loss of communication between the agent and Dataproc. If the
        /// agent does not send a timely update, the job will fail.
        ///
        /// Applies to RUNNING state.
        StaleStatus = 3,
    }
}
/// Encapsulates the full scoping used to reference a job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobReference {
    /// Optional. The ID of the Google Cloud Platform project that the job belongs to. If
    /// specified, must match the request project ID.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// Optional. The job ID, which must be unique within the project.
    ///
    /// The ID must contain only letters (a-z, A-Z), numbers (0-9),
    /// underscores (_), or hyphens (-). The maximum length is 100 characters.
    ///
    /// If not specified by the caller, the job ID will be provided by the server.
    #[prost(string, tag="2")]
    pub job_id: ::prost::alloc::string::String,
}
/// A YARN application created by a job. Application information is a subset of
/// <code>org.apache.hadoop.yarn.proto.YarnProtos.ApplicationReportProto</code>.
///
/// **Beta Feature**: This report is available for testing purposes only. It may
/// be changed before final release.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct YarnApplication {
    /// Required. The application name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The application state.
    #[prost(enumeration="yarn_application::State", tag="2")]
    pub state: i32,
    /// Required. The numerical progress of the application, from 1 to 100.
    #[prost(float, tag="3")]
    pub progress: f32,
    /// Optional. The HTTP URL of the ApplicationMaster, HistoryServer, or
    /// TimelineServer that provides application-specific information. The URL uses
    /// the internal hostname, and requires a proxy server for resolution and,
    /// possibly, access.
    #[prost(string, tag="4")]
    pub tracking_url: ::prost::alloc::string::String,
}
/// Nested message and enum types in `YarnApplication`.
pub mod yarn_application {
    /// The application state, corresponding to
    /// <code>YarnProtos.YarnApplicationStateProto</code>.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Status is unspecified.
        Unspecified = 0,
        /// Status is NEW.
        New = 1,
        /// Status is NEW_SAVING.
        NewSaving = 2,
        /// Status is SUBMITTED.
        Submitted = 3,
        /// Status is ACCEPTED.
        Accepted = 4,
        /// Status is RUNNING.
        Running = 5,
        /// Status is FINISHED.
        Finished = 6,
        /// Status is FAILED.
        Failed = 7,
        /// Status is KILLED.
        Killed = 8,
    }
}
/// A Dataproc job resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Job {
    /// Optional. The fully qualified reference to the job, which can be used to
    /// obtain the equivalent REST path of the job resource. If this property
    /// is not specified when a job is created, the server generates a
    /// <code>job_id</code>.
    #[prost(message, optional, tag="1")]
    pub reference: ::core::option::Option<JobReference>,
    /// Required. Job information, including how, when, and where to
    /// run the job.
    #[prost(message, optional, tag="2")]
    pub placement: ::core::option::Option<JobPlacement>,
    /// Output only. The job status. Additional application-specific
    /// status information may be contained in the <code>type_job</code>
    /// and <code>yarn_applications</code> fields.
    #[prost(message, optional, tag="8")]
    pub status: ::core::option::Option<JobStatus>,
    /// Output only. The previous job status.
    #[prost(message, repeated, tag="13")]
    pub status_history: ::prost::alloc::vec::Vec<JobStatus>,
    /// Output only. The collection of YARN applications spun up by this job.
    ///
    /// **Beta** Feature: This report is available for testing purposes only. It
    /// may be changed before final release.
    #[prost(message, repeated, tag="9")]
    pub yarn_applications: ::prost::alloc::vec::Vec<YarnApplication>,
    /// Output only. A URI pointing to the location of the stdout of the job's
    /// driver program.
    #[prost(string, tag="17")]
    pub driver_output_resource_uri: ::prost::alloc::string::String,
    /// Output only. If present, the location of miscellaneous control files
    /// which may be used as part of job setup and handling. If not present,
    /// control files may be placed in the same location as `driver_output_uri`.
    #[prost(string, tag="15")]
    pub driver_control_files_uri: ::prost::alloc::string::String,
    /// Optional. The labels to associate with this job.
    /// Label **keys** must contain 1 to 63 characters, and must conform to
    /// [RFC 1035](<https://www.ietf.org/rfc/rfc1035.txt>).
    /// Label **values** may be empty, but, if present, must contain 1 to 63
    /// characters, and must conform to [RFC
    /// 1035](<https://www.ietf.org/rfc/rfc1035.txt>). No more than 32 labels can be
    /// associated with a job.
    #[prost(btree_map="string, string", tag="18")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. Job scheduling configuration.
    #[prost(message, optional, tag="20")]
    pub scheduling: ::core::option::Option<JobScheduling>,
    /// Output only. A UUID that uniquely identifies a job within the project
    /// over time. This is in contrast to a user-settable reference.job_id that
    /// may be reused over time.
    #[prost(string, tag="22")]
    pub job_uuid: ::prost::alloc::string::String,
    /// Output only. Indicates whether the job is completed. If the value is `false`,
    /// the job is still in progress. If `true`, the job is completed, and
    /// `status.state` field will indicate if it was successful, failed,
    /// or cancelled.
    #[prost(bool, tag="24")]
    pub done: bool,
    /// Required. The application/framework-specific portion of the job.
    #[prost(oneof="job::TypeJob", tags="3, 4, 5, 6, 7, 21, 12, 23")]
    pub type_job: ::core::option::Option<job::TypeJob>,
}
/// Nested message and enum types in `Job`.
pub mod job {
    /// Required. The application/framework-specific portion of the job.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TypeJob {
        /// Optional. Job is a Hadoop job.
        #[prost(message, tag="3")]
        HadoopJob(super::HadoopJob),
        /// Optional. Job is a Spark job.
        #[prost(message, tag="4")]
        SparkJob(super::SparkJob),
        /// Optional. Job is a PySpark job.
        #[prost(message, tag="5")]
        PysparkJob(super::PySparkJob),
        /// Optional. Job is a Hive job.
        #[prost(message, tag="6")]
        HiveJob(super::HiveJob),
        /// Optional. Job is a Pig job.
        #[prost(message, tag="7")]
        PigJob(super::PigJob),
        /// Optional. Job is a SparkR job.
        #[prost(message, tag="21")]
        SparkRJob(super::SparkRJob),
        /// Optional. Job is a SparkSql job.
        #[prost(message, tag="12")]
        SparkSqlJob(super::SparkSqlJob),
        /// Optional. Job is a Presto job.
        #[prost(message, tag="23")]
        PrestoJob(super::PrestoJob),
    }
}
/// Job scheduling options.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobScheduling {
    /// Optional. Maximum number of times per hour a driver may be restarted as
    /// a result of driver exiting with non-zero code before job is
    /// reported failed.
    ///
    /// A job may be reported as thrashing if driver exits with non-zero code
    /// 4 times within 10 minute window.
    ///
    /// Maximum value is 10.
    ///
    /// **Note:** Currently, this restartable job option is
    /// not supported in Dataproc
    /// [workflow
    /// template](<https://cloud.google.com/dataproc/docs/concepts/workflows/using-workflows#adding_jobs_to_a_template>)
    /// jobs.
    #[prost(int32, tag="1")]
    pub max_failures_per_hour: i32,
    /// Optional. Maximum number of times in total a driver may be restarted as a result of
    /// driver exiting with non-zero code before job is reported failed.
    /// Maximum value is 240.
    ///
    /// **Note:** Currently, this restartable job option is
    /// not supported in Dataproc
    /// [workflow
    /// template](<https://cloud.google.com/dataproc/docs/concepts/workflows/using-workflows#adding_jobs_to_a_template>)
    /// jobs.
    #[prost(int32, tag="2")]
    pub max_failures_total: i32,
}
/// A request to submit a job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitJobRequest {
    /// Required. The ID of the Google Cloud Platform project that the job
    /// belongs to.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. The Dataproc region in which to handle the request.
    #[prost(string, tag="3")]
    pub region: ::prost::alloc::string::String,
    /// Required. The job resource.
    #[prost(message, optional, tag="2")]
    pub job: ::core::option::Option<Job>,
    /// Optional. A unique id used to identify the request. If the server
    /// receives two
    /// \[SubmitJobRequest\](<https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#google.cloud.dataproc.v1.SubmitJobRequest>)s
    /// with the same id, then the second request will be ignored and the
    /// first \[Job][google.cloud.dataproc.v1.Job\] created and stored in the backend
    /// is returned.
    ///
    /// It is recommended to always set this value to a
    /// \[UUID\](<https://en.wikipedia.org/wiki/Universally_unique_identifier>).
    ///
    /// The id must contain only letters (a-z, A-Z), numbers (0-9),
    /// underscores (_), and hyphens (-). The maximum length is 40 characters.
    #[prost(string, tag="4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Job Operation metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobMetadata {
    /// Output only. The job id.
    #[prost(string, tag="1")]
    pub job_id: ::prost::alloc::string::String,
    /// Output only. Most recent job status.
    #[prost(message, optional, tag="2")]
    pub status: ::core::option::Option<JobStatus>,
    /// Output only. Operation type.
    #[prost(string, tag="3")]
    pub operation_type: ::prost::alloc::string::String,
    /// Output only. Job submission time.
    #[prost(message, optional, tag="4")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// A request to get the resource representation for a job in a project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJobRequest {
    /// Required. The ID of the Google Cloud Platform project that the job
    /// belongs to.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. The Dataproc region in which to handle the request.
    #[prost(string, tag="3")]
    pub region: ::prost::alloc::string::String,
    /// Required. The job ID.
    #[prost(string, tag="2")]
    pub job_id: ::prost::alloc::string::String,
}
/// A request to list jobs in a project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobsRequest {
    /// Required. The ID of the Google Cloud Platform project that the job
    /// belongs to.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. The Dataproc region in which to handle the request.
    #[prost(string, tag="6")]
    pub region: ::prost::alloc::string::String,
    /// Optional. The number of results to return in each response.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. The page token, returned by a previous call, to request the
    /// next page of results.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. If set, the returned jobs list includes only jobs that were
    /// submitted to the named cluster.
    #[prost(string, tag="4")]
    pub cluster_name: ::prost::alloc::string::String,
    /// Optional. Specifies enumerated categories of jobs to list.
    /// (default = match ALL jobs).
    ///
    /// If `filter` is provided, `jobStateMatcher` will be ignored.
    #[prost(enumeration="list_jobs_request::JobStateMatcher", tag="5")]
    pub job_state_matcher: i32,
    /// Optional. A filter constraining the jobs to list. Filters are
    /// case-sensitive and have the following syntax:
    ///
    /// [field = value] AND [field [= value]] ...
    ///
    /// where **field** is `status.state` or `labels.\[KEY\]`, and `\[KEY\]` is a label
    /// key. **value** can be `*` to match all values.
    /// `status.state` can be either `ACTIVE` or `NON_ACTIVE`.
    /// Only the logical `AND` operator is supported; space-separated items are
    /// treated as having an implicit `AND` operator.
    ///
    /// Example filter:
    ///
    /// status.state = ACTIVE AND labels.env = staging AND labels.starred = *
    #[prost(string, tag="7")]
    pub filter: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ListJobsRequest`.
pub mod list_jobs_request {
    /// A matcher that specifies categories of job states.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum JobStateMatcher {
        /// Match all jobs, regardless of state.
        All = 0,
        /// Only match jobs in non-terminal states: PENDING, RUNNING, or
        /// CANCEL_PENDING.
        Active = 1,
        /// Only match jobs in terminal states: CANCELLED, DONE, or ERROR.
        NonActive = 2,
    }
}
/// A request to update a job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateJobRequest {
    /// Required. The ID of the Google Cloud Platform project that the job
    /// belongs to.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. The Dataproc region in which to handle the request.
    #[prost(string, tag="2")]
    pub region: ::prost::alloc::string::String,
    /// Required. The job ID.
    #[prost(string, tag="3")]
    pub job_id: ::prost::alloc::string::String,
    /// Required. The changes to the job.
    #[prost(message, optional, tag="4")]
    pub job: ::core::option::Option<Job>,
    /// Required. Specifies the path, relative to <code>Job</code>, of
    /// the field to update. For example, to update the labels of a Job the
    /// <code>update_mask</code> parameter would be specified as
    /// <code>labels</code>, and the `PATCH` request body would specify the new
    /// value. <strong>Note:</strong> Currently, <code>labels</code> is the only
    /// field that can be updated.
    #[prost(message, optional, tag="5")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// A list of jobs in a project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobsResponse {
    /// Output only. Jobs list.
    #[prost(message, repeated, tag="1")]
    pub jobs: ::prost::alloc::vec::Vec<Job>,
    /// Optional. This token is included in the response if there are more results
    /// to fetch. To fetch additional results, provide this value as the
    /// `page_token` in a subsequent <code>ListJobsRequest</code>.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A request to cancel a job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelJobRequest {
    /// Required. The ID of the Google Cloud Platform project that the job
    /// belongs to.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. The Dataproc region in which to handle the request.
    #[prost(string, tag="3")]
    pub region: ::prost::alloc::string::String,
    /// Required. The job ID.
    #[prost(string, tag="2")]
    pub job_id: ::prost::alloc::string::String,
}
/// A request to delete a job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteJobRequest {
    /// Required. The ID of the Google Cloud Platform project that the job
    /// belongs to.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. The Dataproc region in which to handle the request.
    #[prost(string, tag="3")]
    pub region: ::prost::alloc::string::String,
    /// Required. The job ID.
    #[prost(string, tag="2")]
    pub job_id: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod job_controller_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// The JobController provides methods to manage jobs.
    #[derive(Debug, Clone)]
    pub struct JobControllerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> JobControllerClient<T>
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
        ) -> JobControllerClient<InterceptedService<T, F>>
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
            JobControllerClient::new(InterceptedService::new(inner, interceptor))
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
        /// Submits a job to a cluster.
        pub async fn submit_job(
            &mut self,
            request: impl tonic::IntoRequest<super::SubmitJobRequest>,
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
                "/google.cloud.dataproc.v1.JobController/SubmitJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Submits job to a cluster.
        pub async fn submit_job_as_operation(
            &mut self,
            request: impl tonic::IntoRequest<super::SubmitJobRequest>,
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
                "/google.cloud.dataproc.v1.JobController/SubmitJobAsOperation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the resource representation for a job in a project.
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
                "/google.cloud.dataproc.v1.JobController/GetJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists regions/{region}/jobs in a project.
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
                "/google.cloud.dataproc.v1.JobController/ListJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a job in a project.
        pub async fn update_job(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateJobRequest>,
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
                "/google.cloud.dataproc.v1.JobController/UpdateJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Starts a job cancellation request. To access the job resource
        /// after cancellation, call
        /// [regions/{region}/jobs.list](https://cloud.google.com/dataproc/docs/reference/rest/v1/projects.regions.jobs/list)
        /// or
        /// [regions/{region}/jobs.get](https://cloud.google.com/dataproc/docs/reference/rest/v1/projects.regions.jobs/get).
        pub async fn cancel_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelJobRequest>,
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
                "/google.cloud.dataproc.v1.JobController/CancelJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes the job from the project. If the job is active, the delete fails,
        /// and the response returns `FAILED_PRECONDITION`.
        pub async fn delete_job(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteJobRequest>,
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
                "/google.cloud.dataproc.v1.JobController/DeleteJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// A Dataproc workflow template resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkflowTemplate {
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    /// Output only. The resource name of the workflow template, as described
    /// in <https://cloud.google.com/apis/design/resource_names.>
    ///
    /// * For `projects.regions.workflowTemplates`, the resource name of the
    ///   template has the following format:
    ///   `projects/{project_id}/regions/{region}/workflowTemplates/{template_id}`
    ///
    /// * For `projects.locations.workflowTemplates`, the resource name of the
    ///   template has the following format:
    ///   `projects/{project_id}/locations/{location}/workflowTemplates/{template_id}`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Used to perform a consistent read-modify-write.
    ///
    /// This field should be left blank for a `CreateWorkflowTemplate` request. It
    /// is required for an `UpdateWorkflowTemplate` request, and must match the
    /// current server version. A typical update template flow would fetch the
    /// current template with a `GetWorkflowTemplate` request, which will return
    /// the current template with the `version` field filled in with the
    /// current server version. The user updates other fields in the template,
    /// then returns it as part of the `UpdateWorkflowTemplate` request.
    #[prost(int32, tag="3")]
    pub version: i32,
    /// Output only. The time template was created.
    #[prost(message, optional, tag="4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time template was last updated.
    #[prost(message, optional, tag="5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. The labels to associate with this template. These labels
    /// will be propagated to all jobs and clusters created by the workflow
    /// instance.
    ///
    /// Label **keys** must contain 1 to 63 characters, and must conform to
    /// [RFC 1035](<https://www.ietf.org/rfc/rfc1035.txt>).
    ///
    /// Label **values** may be empty, but, if present, must contain 1 to 63
    /// characters, and must conform to
    /// [RFC 1035](<https://www.ietf.org/rfc/rfc1035.txt>).
    ///
    /// No more than 32 labels can be associated with a template.
    #[prost(btree_map="string, string", tag="6")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Required. WorkflowTemplate scheduling information.
    #[prost(message, optional, tag="7")]
    pub placement: ::core::option::Option<WorkflowTemplatePlacement>,
    /// Required. The Directed Acyclic Graph of Jobs to submit.
    #[prost(message, repeated, tag="8")]
    pub jobs: ::prost::alloc::vec::Vec<OrderedJob>,
    /// Optional. Template parameters whose values are substituted into the
    /// template. Values for parameters must be provided when the template is
    /// instantiated.
    #[prost(message, repeated, tag="9")]
    pub parameters: ::prost::alloc::vec::Vec<TemplateParameter>,
    /// Optional. Timeout duration for the DAG of jobs, expressed in seconds (see
    /// [JSON representation of
    /// duration](<https://developers.google.com/protocol-buffers/docs/proto3#json>)).
    /// The timeout duration must be from 10 minutes ("600s") to 24 hours
    /// ("86400s"). The timer begins when the first job is submitted. If the
    /// workflow is running at the end of the timeout period, any remaining jobs
    /// are cancelled, the workflow is ended, and if the workflow was running on a
    /// [managed
    /// cluster](/dataproc/docs/concepts/workflows/using-workflows#configuring_or_selecting_a_cluster),
    /// the cluster is deleted.
    #[prost(message, optional, tag="10")]
    pub dag_timeout: ::core::option::Option<::prost_types::Duration>,
}
/// Specifies workflow execution target.
///
/// Either `managed_cluster` or `cluster_selector` is required.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkflowTemplatePlacement {
    /// Required. Specifies where workflow executes; either on a managed
    /// cluster or an existing cluster chosen by labels.
    #[prost(oneof="workflow_template_placement::Placement", tags="1, 2")]
    pub placement: ::core::option::Option<workflow_template_placement::Placement>,
}
/// Nested message and enum types in `WorkflowTemplatePlacement`.
pub mod workflow_template_placement {
    /// Required. Specifies where workflow executes; either on a managed
    /// cluster or an existing cluster chosen by labels.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Placement {
        /// A cluster that is managed by the workflow.
        #[prost(message, tag="1")]
        ManagedCluster(super::ManagedCluster),
        /// Optional. A selector that chooses target cluster for jobs based
        /// on metadata.
        ///
        /// The selector is evaluated at the time each job is submitted.
        #[prost(message, tag="2")]
        ClusterSelector(super::ClusterSelector),
    }
}
/// Cluster that is managed by the workflow.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManagedCluster {
    /// Required. The cluster name prefix. A unique cluster name will be formed by
    /// appending a random suffix.
    ///
    /// The name must contain only lower-case letters (a-z), numbers (0-9),
    /// and hyphens (-). Must begin with a letter. Cannot begin or end with
    /// hyphen. Must consist of between 2 and 35 characters.
    #[prost(string, tag="2")]
    pub cluster_name: ::prost::alloc::string::String,
    /// Required. The cluster configuration.
    #[prost(message, optional, tag="3")]
    pub config: ::core::option::Option<ClusterConfig>,
    /// Optional. The labels to associate with this cluster.
    ///
    /// Label keys must be between 1 and 63 characters long, and must conform to
    /// the following PCRE regular expression:
    /// \[\p{Ll}\p{Lo}][\p{Ll}\p{Lo}\p{N}_-\]{0,62}
    ///
    /// Label values must be between 1 and 63 characters long, and must conform to
    /// the following PCRE regular expression: \[\p{Ll}\p{Lo}\p{N}_-\]{0,63}
    ///
    /// No more than 32 labels can be associated with a given cluster.
    #[prost(btree_map="string, string", tag="4")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// A selector that chooses target cluster for jobs based on metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterSelector {
    /// Optional. The zone where workflow process executes. This parameter does not
    /// affect the selection of the cluster.
    ///
    /// If unspecified, the zone of the first cluster matching the selector
    /// is used.
    #[prost(string, tag="1")]
    pub zone: ::prost::alloc::string::String,
    /// Required. The cluster labels. Cluster must have all labels
    /// to match.
    #[prost(btree_map="string, string", tag="2")]
    pub cluster_labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// A job executed by the workflow.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderedJob {
    /// Required. The step id. The id must be unique among all jobs
    /// within the template.
    ///
    /// The step id is used as prefix for job id, as job
    /// `goog-dataproc-workflow-step-id` label, and in
    /// \[prerequisiteStepIds][google.cloud.dataproc.v1.OrderedJob.prerequisite_step_ids\] field from other
    /// steps.
    ///
    /// The id must contain only letters (a-z, A-Z), numbers (0-9),
    /// underscores (_), and hyphens (-). Cannot begin or end with underscore
    /// or hyphen. Must consist of between 3 and 50 characters.
    #[prost(string, tag="1")]
    pub step_id: ::prost::alloc::string::String,
    /// Optional. The labels to associate with this job.
    ///
    /// Label keys must be between 1 and 63 characters long, and must conform to
    /// the following regular expression:
    /// \[\p{Ll}\p{Lo}][\p{Ll}\p{Lo}\p{N}_-\]{0,62}
    ///
    /// Label values must be between 1 and 63 characters long, and must conform to
    /// the following regular expression: \[\p{Ll}\p{Lo}\p{N}_-\]{0,63}
    ///
    /// No more than 32 labels can be associated with a given job.
    #[prost(btree_map="string, string", tag="8")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. Job scheduling configuration.
    #[prost(message, optional, tag="9")]
    pub scheduling: ::core::option::Option<JobScheduling>,
    /// Optional. The optional list of prerequisite job step_ids.
    /// If not specified, the job will start at the beginning of workflow.
    #[prost(string, repeated, tag="10")]
    pub prerequisite_step_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The job definition.
    #[prost(oneof="ordered_job::JobType", tags="2, 3, 4, 5, 6, 11, 7, 12")]
    pub job_type: ::core::option::Option<ordered_job::JobType>,
}
/// Nested message and enum types in `OrderedJob`.
pub mod ordered_job {
    /// Required. The job definition.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum JobType {
        /// Optional. Job is a Hadoop job.
        #[prost(message, tag="2")]
        HadoopJob(super::HadoopJob),
        /// Optional. Job is a Spark job.
        #[prost(message, tag="3")]
        SparkJob(super::SparkJob),
        /// Optional. Job is a PySpark job.
        #[prost(message, tag="4")]
        PysparkJob(super::PySparkJob),
        /// Optional. Job is a Hive job.
        #[prost(message, tag="5")]
        HiveJob(super::HiveJob),
        /// Optional. Job is a Pig job.
        #[prost(message, tag="6")]
        PigJob(super::PigJob),
        /// Optional. Job is a SparkR job.
        #[prost(message, tag="11")]
        SparkRJob(super::SparkRJob),
        /// Optional. Job is a SparkSql job.
        #[prost(message, tag="7")]
        SparkSqlJob(super::SparkSqlJob),
        /// Optional. Job is a Presto job.
        #[prost(message, tag="12")]
        PrestoJob(super::PrestoJob),
    }
}
/// A configurable parameter that replaces one or more fields in the template.
/// Parameterizable fields:
/// - Labels
/// - File uris
/// - Job properties
/// - Job arguments
/// - Script variables
/// - Main class (in HadoopJob and SparkJob)
/// - Zone (in ClusterSelector)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TemplateParameter {
    /// Required. Parameter name.
    /// The parameter name is used as the key, and paired with the
    /// parameter value, which are passed to the template when the template
    /// is instantiated.
    /// The name must contain only capital letters (A-Z), numbers (0-9), and
    /// underscores (_), and must not start with a number. The maximum length is
    /// 40 characters.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Paths to all fields that the parameter replaces.
    /// A field is allowed to appear in at most one parameter's list of field
    /// paths.
    ///
    /// A field path is similar in syntax to a \[google.protobuf.FieldMask][google.protobuf.FieldMask\].
    /// For example, a field path that references the zone field of a workflow
    /// template's cluster selector would be specified as
    /// `placement.clusterSelector.zone`.
    ///
    /// Also, field paths can reference fields using the following syntax:
    ///
    /// * Values in maps can be referenced by key:
    ///     * labels\['key'\]
    ///     * placement.clusterSelector.clusterLabels\['key'\]
    ///     * placement.managedCluster.labels\['key'\]
    ///     * placement.clusterSelector.clusterLabels\['key'\]
    ///     * jobs\['step-id'].labels['key'\]
    ///
    /// * Jobs in the jobs list can be referenced by step-id:
    ///     * jobs\['step-id'\].hadoopJob.mainJarFileUri
    ///     * jobs\['step-id'\].hiveJob.queryFileUri
    ///     * jobs\['step-id'\].pySparkJob.mainPythonFileUri
    ///     * jobs\['step-id'].hadoopJob.jarFileUris[0\]
    ///     * jobs\['step-id'].hadoopJob.archiveUris[0\]
    ///     * jobs\['step-id'].hadoopJob.fileUris[0\]
    ///     * jobs\['step-id'].pySparkJob.pythonFileUris[0\]
    ///
    /// * Items in repeated fields can be referenced by a zero-based index:
    ///     * jobs\['step-id'].sparkJob.args[0\]
    ///
    /// * Other examples:
    ///     * jobs\['step-id'].hadoopJob.properties['key'\]
    ///     * jobs\['step-id'].hadoopJob.args[0\]
    ///     * jobs\['step-id'].hiveJob.scriptVariables['key'\]
    ///     * jobs\['step-id'\].hadoopJob.mainJarFileUri
    ///     * placement.clusterSelector.zone
    ///
    /// It may not be possible to parameterize maps and repeated fields in their
    /// entirety since only individual map values and individual items in repeated
    /// fields can be referenced. For example, the following field paths are
    /// invalid:
    ///
    /// - placement.clusterSelector.clusterLabels
    /// - jobs\['step-id'\].sparkJob.args
    #[prost(string, repeated, tag="2")]
    pub fields: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Brief description of the parameter.
    /// Must not exceed 1024 characters.
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    /// Optional. Validation rules to be applied to this parameter's value.
    #[prost(message, optional, tag="4")]
    pub validation: ::core::option::Option<ParameterValidation>,
}
/// Configuration for parameter validation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParameterValidation {
    /// Required. The type of validation to be performed.
    #[prost(oneof="parameter_validation::ValidationType", tags="1, 2")]
    pub validation_type: ::core::option::Option<parameter_validation::ValidationType>,
}
/// Nested message and enum types in `ParameterValidation`.
pub mod parameter_validation {
    /// Required. The type of validation to be performed.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ValidationType {
        /// Validation based on regular expressions.
        #[prost(message, tag="1")]
        Regex(super::RegexValidation),
        /// Validation based on a list of allowed values.
        #[prost(message, tag="2")]
        Values(super::ValueValidation),
    }
}
/// Validation based on regular expressions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegexValidation {
    /// Required. RE2 regular expressions used to validate the parameter's value.
    /// The value must match the regex in its entirety (substring
    /// matches are not sufficient).
    #[prost(string, repeated, tag="1")]
    pub regexes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Validation based on a list of allowed values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueValidation {
    /// Required. List of allowed values for the parameter.
    #[prost(string, repeated, tag="1")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A Dataproc workflow template resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkflowMetadata {
    /// Output only. The resource name of the workflow template as described
    /// in <https://cloud.google.com/apis/design/resource_names.>
    ///
    /// * For `projects.regions.workflowTemplates`, the resource name of the
    ///   template has the following format:
    ///   `projects/{project_id}/regions/{region}/workflowTemplates/{template_id}`
    ///
    /// * For `projects.locations.workflowTemplates`, the resource name of the
    ///   template has the following format:
    ///   `projects/{project_id}/locations/{location}/workflowTemplates/{template_id}`
    #[prost(string, tag="1")]
    pub template: ::prost::alloc::string::String,
    /// Output only. The version of template at the time of
    /// workflow instantiation.
    #[prost(int32, tag="2")]
    pub version: i32,
    /// Output only. The create cluster operation metadata.
    #[prost(message, optional, tag="3")]
    pub create_cluster: ::core::option::Option<ClusterOperation>,
    /// Output only. The workflow graph.
    #[prost(message, optional, tag="4")]
    pub graph: ::core::option::Option<WorkflowGraph>,
    /// Output only. The delete cluster operation metadata.
    #[prost(message, optional, tag="5")]
    pub delete_cluster: ::core::option::Option<ClusterOperation>,
    /// Output only. The workflow state.
    #[prost(enumeration="workflow_metadata::State", tag="6")]
    pub state: i32,
    /// Output only. The name of the target cluster.
    #[prost(string, tag="7")]
    pub cluster_name: ::prost::alloc::string::String,
    /// Map from parameter names to values that were used for those parameters.
    #[prost(btree_map="string, string", tag="8")]
    pub parameters: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. Workflow start time.
    #[prost(message, optional, tag="9")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Workflow end time.
    #[prost(message, optional, tag="10")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The UUID of target cluster.
    #[prost(string, tag="11")]
    pub cluster_uuid: ::prost::alloc::string::String,
    /// Output only. The timeout duration for the DAG of jobs, expressed in seconds (see
    /// [JSON representation of
    /// duration](<https://developers.google.com/protocol-buffers/docs/proto3#json>)).
    #[prost(message, optional, tag="12")]
    pub dag_timeout: ::core::option::Option<::prost_types::Duration>,
    /// Output only. DAG start time, only set for workflows with \[dag_timeout][google.cloud.dataproc.v1.WorkflowMetadata.dag_timeout\] when DAG
    /// begins.
    #[prost(message, optional, tag="13")]
    pub dag_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. DAG end time, only set for workflows with \[dag_timeout][google.cloud.dataproc.v1.WorkflowMetadata.dag_timeout\] when DAG ends.
    #[prost(message, optional, tag="14")]
    pub dag_end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `WorkflowMetadata`.
pub mod workflow_metadata {
    /// The operation state.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unused.
        Unknown = 0,
        /// The operation has been created.
        Pending = 1,
        /// The operation is running.
        Running = 2,
        /// The operation is done; either cancelled or completed.
        Done = 3,
    }
}
/// The cluster operation triggered by a workflow.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterOperation {
    /// Output only. The id of the cluster operation.
    #[prost(string, tag="1")]
    pub operation_id: ::prost::alloc::string::String,
    /// Output only. Error, if operation failed.
    #[prost(string, tag="2")]
    pub error: ::prost::alloc::string::String,
    /// Output only. Indicates the operation is done.
    #[prost(bool, tag="3")]
    pub done: bool,
}
/// The workflow graph.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkflowGraph {
    /// Output only. The workflow nodes.
    #[prost(message, repeated, tag="1")]
    pub nodes: ::prost::alloc::vec::Vec<WorkflowNode>,
}
/// The workflow node.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkflowNode {
    /// Output only. The name of the node.
    #[prost(string, tag="1")]
    pub step_id: ::prost::alloc::string::String,
    /// Output only. Node's prerequisite nodes.
    #[prost(string, repeated, tag="2")]
    pub prerequisite_step_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The job id; populated after the node enters RUNNING state.
    #[prost(string, tag="3")]
    pub job_id: ::prost::alloc::string::String,
    /// Output only. The node state.
    #[prost(enumeration="workflow_node::NodeState", tag="5")]
    pub state: i32,
    /// Output only. The error detail.
    #[prost(string, tag="6")]
    pub error: ::prost::alloc::string::String,
}
/// Nested message and enum types in `WorkflowNode`.
pub mod workflow_node {
    /// The workflow node state.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum NodeState {
        /// State is unspecified.
        Unspecified = 0,
        /// The node is awaiting prerequisite node to finish.
        Blocked = 1,
        /// The node is runnable but not running.
        Runnable = 2,
        /// The node is running.
        Running = 3,
        /// The node completed successfully.
        Completed = 4,
        /// The node failed. A node can be marked FAILED because
        /// its ancestor or peer failed.
        Failed = 5,
    }
}
/// A request to create a workflow template.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkflowTemplateRequest {
    /// Required. The resource name of the region or location, as described
    /// in <https://cloud.google.com/apis/design/resource_names.>
    ///
    /// * For `projects.regions.workflowTemplates.create`, the resource name of the
    ///   region has the following format:
    ///   `projects/{project_id}/regions/{region}`
    ///
    /// * For `projects.locations.workflowTemplates.create`, the resource name of
    ///   the location has the following format:
    ///   `projects/{project_id}/locations/{location}`
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Dataproc workflow template to create.
    #[prost(message, optional, tag="2")]
    pub template: ::core::option::Option<WorkflowTemplate>,
}
/// A request to fetch a workflow template.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkflowTemplateRequest {
    /// Required. The resource name of the workflow template, as described
    /// in <https://cloud.google.com/apis/design/resource_names.>
    ///
    /// * For `projects.regions.workflowTemplates.get`, the resource name of the
    ///   template has the following format:
    ///   `projects/{project_id}/regions/{region}/workflowTemplates/{template_id}`
    ///
    /// * For `projects.locations.workflowTemplates.get`, the resource name of the
    ///   template has the following format:
    ///   `projects/{project_id}/locations/{location}/workflowTemplates/{template_id}`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The version of workflow template to retrieve. Only previously
    /// instantiated versions can be retrieved.
    ///
    /// If unspecified, retrieves the current version.
    #[prost(int32, tag="2")]
    pub version: i32,
}
/// A request to instantiate a workflow template.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstantiateWorkflowTemplateRequest {
    /// Required. The resource name of the workflow template, as described
    /// in <https://cloud.google.com/apis/design/resource_names.>
    ///
    /// * For `projects.regions.workflowTemplates.instantiate`, the resource name
    /// of the template has the following format:
    ///   `projects/{project_id}/regions/{region}/workflowTemplates/{template_id}`
    ///
    /// * For `projects.locations.workflowTemplates.instantiate`, the resource name
    ///   of the template has the following format:
    ///   `projects/{project_id}/locations/{location}/workflowTemplates/{template_id}`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The version of workflow template to instantiate. If specified,
    /// the workflow will be instantiated only if the current version of
    /// the workflow template has the supplied version.
    ///
    /// This option cannot be used to instantiate a previous version of
    /// workflow template.
    #[prost(int32, tag="2")]
    pub version: i32,
    /// Optional. A tag that prevents multiple concurrent workflow
    /// instances with the same tag from running. This mitigates risk of
    /// concurrent instances started due to retries.
    ///
    /// It is recommended to always set this value to a
    /// \[UUID\](<https://en.wikipedia.org/wiki/Universally_unique_identifier>).
    ///
    /// The tag must contain only letters (a-z, A-Z), numbers (0-9),
    /// underscores (_), and hyphens (-). The maximum length is 40 characters.
    #[prost(string, tag="5")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. Map from parameter names to values that should be used for those
    /// parameters. Values may not exceed 1000 characters.
    #[prost(btree_map="string, string", tag="6")]
    pub parameters: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// A request to instantiate an inline workflow template.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstantiateInlineWorkflowTemplateRequest {
    /// Required. The resource name of the region or location, as described
    /// in <https://cloud.google.com/apis/design/resource_names.>
    ///
    /// * For `projects.regions.workflowTemplates,instantiateinline`, the resource
    ///   name of the region has the following format:
    ///   `projects/{project_id}/regions/{region}`
    ///
    /// * For `projects.locations.workflowTemplates.instantiateinline`, the
    ///   resource name of the location has the following format:
    ///   `projects/{project_id}/locations/{location}`
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The workflow template to instantiate.
    #[prost(message, optional, tag="2")]
    pub template: ::core::option::Option<WorkflowTemplate>,
    /// Optional. A tag that prevents multiple concurrent workflow
    /// instances with the same tag from running. This mitigates risk of
    /// concurrent instances started due to retries.
    ///
    /// It is recommended to always set this value to a
    /// \[UUID\](<https://en.wikipedia.org/wiki/Universally_unique_identifier>).
    ///
    /// The tag must contain only letters (a-z, A-Z), numbers (0-9),
    /// underscores (_), and hyphens (-). The maximum length is 40 characters.
    #[prost(string, tag="3")]
    pub request_id: ::prost::alloc::string::String,
}
/// A request to update a workflow template.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateWorkflowTemplateRequest {
    /// Required. The updated workflow template.
    ///
    /// The `template.version` field must match the current version.
    #[prost(message, optional, tag="1")]
    pub template: ::core::option::Option<WorkflowTemplate>,
}
/// A request to list workflow templates in a project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkflowTemplatesRequest {
    /// Required. The resource name of the region or location, as described
    /// in <https://cloud.google.com/apis/design/resource_names.>
    ///
    /// * For `projects.regions.workflowTemplates,list`, the resource
    ///   name of the region has the following format:
    ///   `projects/{project_id}/regions/{region}`
    ///
    /// * For `projects.locations.workflowTemplates.list`, the
    ///   resource name of the location has the following format:
    ///   `projects/{project_id}/locations/{location}`
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of results to return in each response.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. The page token, returned by a previous call, to request the
    /// next page of results.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// A response to a request to list workflow templates in a project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkflowTemplatesResponse {
    /// Output only. WorkflowTemplates list.
    #[prost(message, repeated, tag="1")]
    pub templates: ::prost::alloc::vec::Vec<WorkflowTemplate>,
    /// Output only. This token is included in the response if there are more
    /// results to fetch. To fetch additional results, provide this value as the
    /// page_token in a subsequent <code>ListWorkflowTemplatesRequest</code>.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A request to delete a workflow template.
///
/// Currently started workflows will remain running.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWorkflowTemplateRequest {
    /// Required. The resource name of the workflow template, as described
    /// in <https://cloud.google.com/apis/design/resource_names.>
    ///
    /// * For `projects.regions.workflowTemplates.delete`, the resource name
    /// of the template has the following format:
    ///   `projects/{project_id}/regions/{region}/workflowTemplates/{template_id}`
    ///
    /// * For `projects.locations.workflowTemplates.instantiate`, the resource name
    ///   of the template has the following format:
    ///   `projects/{project_id}/locations/{location}/workflowTemplates/{template_id}`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The version of workflow template to delete. If specified,
    /// will only delete the template if the current server version matches
    /// specified version.
    #[prost(int32, tag="2")]
    pub version: i32,
}
/// Generated client implementations.
pub mod workflow_template_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// The API interface for managing Workflow Templates in the
    /// Dataproc API.
    #[derive(Debug, Clone)]
    pub struct WorkflowTemplateServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> WorkflowTemplateServiceClient<T>
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
        ) -> WorkflowTemplateServiceClient<InterceptedService<T, F>>
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
            WorkflowTemplateServiceClient::new(
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
        /// Creates new workflow template.
        pub async fn create_workflow_template(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWorkflowTemplateRequest>,
        ) -> Result<tonic::Response<super::WorkflowTemplate>, tonic::Status> {
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
                "/google.cloud.dataproc.v1.WorkflowTemplateService/CreateWorkflowTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves the latest workflow template.
        ///
        /// Can retrieve previously instantiated template by specifying optional
        /// version parameter.
        pub async fn get_workflow_template(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWorkflowTemplateRequest>,
        ) -> Result<tonic::Response<super::WorkflowTemplate>, tonic::Status> {
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
                "/google.cloud.dataproc.v1.WorkflowTemplateService/GetWorkflowTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Instantiates a template and begins execution.
        ///
        /// The returned Operation can be used to track execution of
        /// workflow by polling
        /// [operations.get][google.longrunning.Operations.GetOperation].
        /// The Operation will complete when entire workflow is finished.
        ///
        /// The running workflow can be aborted via
        /// [operations.cancel][google.longrunning.Operations.CancelOperation].
        /// This will cause any inflight jobs to be cancelled and workflow-owned
        /// clusters to be deleted.
        ///
        /// The [Operation.metadata][google.longrunning.Operation.metadata] will be
        /// [WorkflowMetadata](https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#workflowmetadata).
        /// Also see [Using
        /// WorkflowMetadata](https://cloud.google.com/dataproc/docs/concepts/workflows/debugging#using_workflowmetadata).
        ///
        /// On successful completion,
        /// [Operation.response][google.longrunning.Operation.response] will be
        /// [Empty][google.protobuf.Empty].
        pub async fn instantiate_workflow_template(
            &mut self,
            request: impl tonic::IntoRequest<super::InstantiateWorkflowTemplateRequest>,
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
                "/google.cloud.dataproc.v1.WorkflowTemplateService/InstantiateWorkflowTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Instantiates a template and begins execution.
        ///
        /// This method is equivalent to executing the sequence
        /// [CreateWorkflowTemplate][google.cloud.dataproc.v1.WorkflowTemplateService.CreateWorkflowTemplate], [InstantiateWorkflowTemplate][google.cloud.dataproc.v1.WorkflowTemplateService.InstantiateWorkflowTemplate],
        /// [DeleteWorkflowTemplate][google.cloud.dataproc.v1.WorkflowTemplateService.DeleteWorkflowTemplate].
        ///
        /// The returned Operation can be used to track execution of
        /// workflow by polling
        /// [operations.get][google.longrunning.Operations.GetOperation].
        /// The Operation will complete when entire workflow is finished.
        ///
        /// The running workflow can be aborted via
        /// [operations.cancel][google.longrunning.Operations.CancelOperation].
        /// This will cause any inflight jobs to be cancelled and workflow-owned
        /// clusters to be deleted.
        ///
        /// The [Operation.metadata][google.longrunning.Operation.metadata] will be
        /// [WorkflowMetadata](https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#workflowmetadata).
        /// Also see [Using
        /// WorkflowMetadata](https://cloud.google.com/dataproc/docs/concepts/workflows/debugging#using_workflowmetadata).
        ///
        /// On successful completion,
        /// [Operation.response][google.longrunning.Operation.response] will be
        /// [Empty][google.protobuf.Empty].
        pub async fn instantiate_inline_workflow_template(
            &mut self,
            request: impl tonic::IntoRequest<
                super::InstantiateInlineWorkflowTemplateRequest,
            >,
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
                "/google.cloud.dataproc.v1.WorkflowTemplateService/InstantiateInlineWorkflowTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates (replaces) workflow template. The updated template
        /// must contain version that matches the current server version.
        pub async fn update_workflow_template(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateWorkflowTemplateRequest>,
        ) -> Result<tonic::Response<super::WorkflowTemplate>, tonic::Status> {
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
                "/google.cloud.dataproc.v1.WorkflowTemplateService/UpdateWorkflowTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists workflows that match the specified filter in the request.
        pub async fn list_workflow_templates(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWorkflowTemplatesRequest>,
        ) -> Result<
            tonic::Response<super::ListWorkflowTemplatesResponse>,
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
                "/google.cloud.dataproc.v1.WorkflowTemplateService/ListWorkflowTemplates",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a workflow template. It does not cancel in-progress workflows.
        pub async fn delete_workflow_template(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteWorkflowTemplateRequest>,
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
                "/google.cloud.dataproc.v1.WorkflowTemplateService/DeleteWorkflowTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
