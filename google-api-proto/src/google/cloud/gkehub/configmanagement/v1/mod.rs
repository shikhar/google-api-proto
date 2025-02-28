/// **Anthos Config Management**: State for a single cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MembershipState {
    /// The user-defined name for the cluster used by ClusterSelectors to group
    /// clusters together. This should match Membership's membership_name,
    /// unless the user installed ACM on the cluster manually prior to enabling
    /// the ACM hub feature.
    /// Unique within a Anthos Config Management installation.
    #[prost(string, tag="1")]
    pub cluster_name: ::prost::alloc::string::String,
    /// Membership configuration in the cluster. This represents the actual state
    /// in the cluster, while the MembershipSpec in the FeatureSpec represents
    /// the intended state
    #[prost(message, optional, tag="2")]
    pub membership_spec: ::core::option::Option<MembershipSpec>,
    /// Current install status of ACM's Operator
    #[prost(message, optional, tag="3")]
    pub operator_state: ::core::option::Option<OperatorState>,
    /// Current sync status
    #[prost(message, optional, tag="4")]
    pub config_sync_state: ::core::option::Option<ConfigSyncState>,
    /// PolicyController status
    #[prost(message, optional, tag="5")]
    pub policy_controller_state: ::core::option::Option<PolicyControllerState>,
    /// Hierarchy Controller status
    #[prost(message, optional, tag="7")]
    pub hierarchy_controller_state: ::core::option::Option<HierarchyControllerState>,
}
/// **Anthos Config Management**: Configuration for a single cluster.
/// Intended to parallel the ConfigManagement CR.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MembershipSpec {
    /// Config Sync configuration for the cluster.
    #[prost(message, optional, tag="1")]
    pub config_sync: ::core::option::Option<ConfigSync>,
    /// Policy Controller configuration for the cluster.
    #[prost(message, optional, tag="2")]
    pub policy_controller: ::core::option::Option<PolicyController>,
    /// Hierarchy Controller configuration for the cluster.
    #[prost(message, optional, tag="4")]
    pub hierarchy_controller: ::core::option::Option<HierarchyControllerConfig>,
    /// Version of ACM installed.
    #[prost(string, tag="10")]
    pub version: ::prost::alloc::string::String,
}
/// Configuration for Config Sync
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigSync {
    /// Git repo configuration for the cluster.
    #[prost(message, optional, tag="7")]
    pub git: ::core::option::Option<GitConfig>,
    /// Specifies whether the Config Sync Repo is
    /// in “hierarchical” or “unstructured” mode.
    #[prost(string, tag="8")]
    pub source_format: ::prost::alloc::string::String,
}
/// Git repo configuration for a single cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitConfig {
    /// The URL of the Git repository to use as the source of truth.
    #[prost(string, tag="1")]
    pub sync_repo: ::prost::alloc::string::String,
    /// The branch of the repository to sync from. Default: master.
    #[prost(string, tag="2")]
    pub sync_branch: ::prost::alloc::string::String,
    /// The path within the Git repository that represents the top level of the
    /// repo to sync. Default: the root directory of the repository.
    #[prost(string, tag="3")]
    pub policy_dir: ::prost::alloc::string::String,
    /// Period in seconds between consecutive syncs. Default: 15.
    #[prost(int64, tag="4")]
    pub sync_wait_secs: i64,
    /// Git revision (tag or hash) to check out. Default HEAD.
    #[prost(string, tag="5")]
    pub sync_rev: ::prost::alloc::string::String,
    /// Type of secret configured for access to the Git repo.
    #[prost(string, tag="6")]
    pub secret_type: ::prost::alloc::string::String,
    /// URL for the HTTPS proxy to be used when communicating with the Git repo.
    #[prost(string, tag="7")]
    pub https_proxy: ::prost::alloc::string::String,
    /// The GCP Service Account Email used for auth when secret_type is
    /// gcpServiceAccount.
    #[prost(string, tag="8")]
    pub gcp_service_account_email: ::prost::alloc::string::String,
}
/// Configuration for Policy Controller
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyController {
    /// Enables the installation of Policy Controller.
    /// If false, the rest of PolicyController fields take no
    /// effect.
    #[prost(bool, tag="1")]
    pub enabled: bool,
    /// Installs the default template library along with Policy Controller.
    #[prost(bool, optional, tag="2")]
    pub template_library_installed: ::core::option::Option<bool>,
    /// Sets the interval for Policy Controller Audit Scans (in seconds).
    /// When set to 0, this disables audit functionality altogether.
    #[prost(int64, optional, tag="3")]
    pub audit_interval_seconds: ::core::option::Option<i64>,
    /// The set of namespaces that are excluded from Policy Controller checks.
    /// Namespaces do not need to currently exist on the cluster.
    #[prost(string, repeated, tag="4")]
    pub exemptable_namespaces: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Enables the ability to use Constraint Templates that reference to objects
    /// other than the object currently being evaluated.
    #[prost(bool, tag="5")]
    pub referential_rules_enabled: bool,
    /// Logs all denies and dry run failures.
    #[prost(bool, tag="6")]
    pub log_denies_enabled: bool,
}
/// Configuration for Hierarchy Controller
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HierarchyControllerConfig {
    /// Whether Hierarchy Controller is enabled in this cluster.
    #[prost(bool, tag="1")]
    pub enabled: bool,
    /// Whether pod tree labels are enabled in this cluster.
    #[prost(bool, tag="2")]
    pub enable_pod_tree_labels: bool,
    /// Whether hierarchical resource quota is enabled in this cluster.
    #[prost(bool, tag="3")]
    pub enable_hierarchical_resource_quota: bool,
}
/// Deployment state for Hierarchy Controller
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HierarchyControllerDeploymentState {
    /// The deployment state for open source HNC (e.g. v0.7.0-hc.0)
    #[prost(enumeration="DeploymentState", tag="1")]
    pub hnc: i32,
    /// The deployment state for Hierarchy Controller extension (e.g. v0.7.0-hc.1)
    #[prost(enumeration="DeploymentState", tag="2")]
    pub extension: i32,
}
/// Version for Hierarchy Controller
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HierarchyControllerVersion {
    /// Version for open source HNC
    #[prost(string, tag="1")]
    pub hnc: ::prost::alloc::string::String,
    /// Version for Hierarchy Controller extension
    #[prost(string, tag="2")]
    pub extension: ::prost::alloc::string::String,
}
/// State for Hierarchy Controller
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HierarchyControllerState {
    /// The version for Hierarchy Controller
    #[prost(message, optional, tag="1")]
    pub version: ::core::option::Option<HierarchyControllerVersion>,
    /// The deployment state for Hierarchy Controller
    #[prost(message, optional, tag="2")]
    pub state: ::core::option::Option<HierarchyControllerDeploymentState>,
}
/// State information for an ACM's Operator
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperatorState {
    /// The semenatic version number of the operator
    #[prost(string, tag="1")]
    pub version: ::prost::alloc::string::String,
    /// The state of the Operator's deployment
    #[prost(enumeration="DeploymentState", tag="2")]
    pub deployment_state: i32,
    /// Install errors.
    #[prost(message, repeated, tag="3")]
    pub errors: ::prost::alloc::vec::Vec<InstallError>,
}
/// Errors pertaining to the installation of ACM
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstallError {
    /// A string representing the user facing error message
    #[prost(string, tag="1")]
    pub error_message: ::prost::alloc::string::String,
}
/// State information for ConfigSync
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigSyncState {
    /// The version of ConfigSync deployed
    #[prost(message, optional, tag="1")]
    pub version: ::core::option::Option<ConfigSyncVersion>,
    /// Information about the deployment of ConfigSync, including the version
    /// of the various Pods deployed
    #[prost(message, optional, tag="2")]
    pub deployment_state: ::core::option::Option<ConfigSyncDeploymentState>,
    /// The state of ConfigSync's process to sync configs to a cluster
    #[prost(message, optional, tag="3")]
    pub sync_state: ::core::option::Option<SyncState>,
}
/// Specific versioning information pertaining to ConfigSync's Pods
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigSyncVersion {
    /// Version of the deployed importer pod
    #[prost(string, tag="1")]
    pub importer: ::prost::alloc::string::String,
    /// Version of the deployed syncer pod
    #[prost(string, tag="2")]
    pub syncer: ::prost::alloc::string::String,
    /// Version of the deployed git-sync pod
    #[prost(string, tag="3")]
    pub git_sync: ::prost::alloc::string::String,
    /// Version of the deployed monitor pod
    #[prost(string, tag="4")]
    pub monitor: ::prost::alloc::string::String,
    /// Version of the deployed reconciler-manager pod
    #[prost(string, tag="5")]
    pub reconciler_manager: ::prost::alloc::string::String,
    /// Version of the deployed reconciler container in root-reconciler pod
    #[prost(string, tag="6")]
    pub root_reconciler: ::prost::alloc::string::String,
}
/// The state of ConfigSync's deployment on a cluster
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigSyncDeploymentState {
    /// Deployment state of the importer pod
    #[prost(enumeration="DeploymentState", tag="1")]
    pub importer: i32,
    /// Deployment state of the syncer pod
    #[prost(enumeration="DeploymentState", tag="2")]
    pub syncer: i32,
    /// Deployment state of the git-sync pod
    #[prost(enumeration="DeploymentState", tag="3")]
    pub git_sync: i32,
    /// Deployment state of the monitor pod
    #[prost(enumeration="DeploymentState", tag="4")]
    pub monitor: i32,
    /// Deployment state of reconciler-manager pod
    #[prost(enumeration="DeploymentState", tag="5")]
    pub reconciler_manager: i32,
    /// Deployment state of root-reconciler
    #[prost(enumeration="DeploymentState", tag="6")]
    pub root_reconciler: i32,
}
/// State indicating an ACM's progress syncing configurations to a cluster
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncState {
    /// Token indicating the state of the repo.
    #[prost(string, tag="1")]
    pub source_token: ::prost::alloc::string::String,
    /// Token indicating the state of the importer.
    #[prost(string, tag="2")]
    pub import_token: ::prost::alloc::string::String,
    /// Token indicating the state of the syncer.
    #[prost(string, tag="3")]
    pub sync_token: ::prost::alloc::string::String,
    /// Deprecated: use last_sync_time instead.
    /// Timestamp of when ACM last successfully synced the repo
    /// The time format is specified in <https://golang.org/pkg/time/#Time.String>
    #[deprecated]
    #[prost(string, tag="4")]
    pub last_sync: ::prost::alloc::string::String,
    /// Timestamp type of when ACM last successfully synced the repo
    #[prost(message, optional, tag="7")]
    pub last_sync_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Sync status code
    #[prost(enumeration="sync_state::SyncCode", tag="5")]
    pub code: i32,
    /// A list of errors resulting from problematic configs.
    /// This list will be truncated after 100 errors, although it is
    /// unlikely for that many errors to simultaneously exist.
    #[prost(message, repeated, tag="6")]
    pub errors: ::prost::alloc::vec::Vec<SyncError>,
}
/// Nested message and enum types in `SyncState`.
pub mod sync_state {
    /// An enum representing an ACM's status syncing configs to a cluster
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SyncCode {
        /// ACM cannot determine a sync code
        Unspecified = 0,
        /// ACM successfully synced the git Repo with the cluster
        Synced = 1,
        /// ACM is in the progress of syncing a new change
        Pending = 2,
        /// Indicates an error configuring ACM, and user action is required
        Error = 3,
        /// ACM has been installed (operator manifest deployed),
        /// but not configured.
        NotConfigured = 4,
        /// ACM has not been installed (no operator pod found)
        NotInstalled = 5,
        /// Error authorizing with the cluster
        Unauthorized = 6,
        /// Cluster could not be reached
        Unreachable = 7,
    }
}
/// An ACM created error representing a problem syncing configurations
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncError {
    /// An ACM defined error code
    #[prost(string, tag="1")]
    pub code: ::prost::alloc::string::String,
    /// A description of the error
    #[prost(string, tag="2")]
    pub error_message: ::prost::alloc::string::String,
    /// A list of config(s) associated with the error, if any
    #[prost(message, repeated, tag="3")]
    pub error_resources: ::prost::alloc::vec::Vec<ErrorResource>,
}
/// Model for a config file in the git repo with an associated Sync error
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorResource {
    /// Path in the git repo of the erroneous config
    #[prost(string, tag="1")]
    pub source_path: ::prost::alloc::string::String,
    /// Metadata name of the resource that is causing an error
    #[prost(string, tag="2")]
    pub resource_name: ::prost::alloc::string::String,
    /// Namespace of the resource that is causing an error
    #[prost(string, tag="3")]
    pub resource_namespace: ::prost::alloc::string::String,
    /// Group/version/kind of the resource that is causing an error
    #[prost(message, optional, tag="4")]
    pub resource_gvk: ::core::option::Option<GroupVersionKind>,
}
/// A Kubernetes object's GVK
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupVersionKind {
    /// Kubernetes Group
    #[prost(string, tag="1")]
    pub group: ::prost::alloc::string::String,
    /// Kubernetes Version
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
    /// Kubernetes Kind
    #[prost(string, tag="3")]
    pub kind: ::prost::alloc::string::String,
}
/// State for PolicyControllerState.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyControllerState {
    /// The version of Gatekeeper Policy Controller deployed.
    #[prost(message, optional, tag="1")]
    pub version: ::core::option::Option<PolicyControllerVersion>,
    /// The state about the policy controller installation.
    #[prost(message, optional, tag="2")]
    pub deployment_state: ::core::option::Option<GatekeeperDeploymentState>,
}
/// The build version of Gatekeeper Policy Controller is using.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyControllerVersion {
    /// The gatekeeper image tag that is composed of ACM version, git tag, build
    /// number.
    #[prost(string, tag="1")]
    pub version: ::prost::alloc::string::String,
}
/// State of Policy Controller installation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GatekeeperDeploymentState {
    /// Status of gatekeeper-controller-manager pod.
    #[prost(enumeration="DeploymentState", tag="1")]
    pub gatekeeper_controller_manager_state: i32,
    /// Status of gatekeeper-audit deployment.
    #[prost(enumeration="DeploymentState", tag="2")]
    pub gatekeeper_audit: i32,
}
/// Enum representing the state of an ACM's deployment on a cluster
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DeploymentState {
    /// Deployment's state cannot be determined
    Unspecified = 0,
    /// Deployment is not installed
    NotInstalled = 1,
    /// Deployment is installed
    Installed = 2,
    /// Deployment was attempted to be installed, but has errors
    Error = 3,
}
