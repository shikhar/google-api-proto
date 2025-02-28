/// JSON payload for the Cloud Logging event:
/// `organizations/\[organizationId\]/logs/cloudsetup.googleapis.com%2Fcomplete_deployment`.
/// This event gets emitted upon completion of a config deployment as part of the
/// Cloud Setup Checklist.
///
/// The deployment can fail even if it returns a
/// `config.googleapis.com/Deployment`. The state of that message will be
/// `FAILED`. Hence, if there is a `value` present, the `state` can still be,
/// `FAILED`. The message for the error or failure will be on the `error` or the
/// `value` if the Operation results in an error or if the `state` of the
/// Deployment is `FAILED`, respectively.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteDeploymentEvent {
    /// The `state` of this deployment completion event.
    #[prost(enumeration="complete_deployment_event::State", tag="3")]
    pub state: i32,
    /// Flag to indicate if deployment is preview only.
    #[prost(bool, tag="4")]
    pub preview_only: bool,
    #[prost(oneof="complete_deployment_event::Result", tags="1, 2")]
    pub result: ::core::option::Option<complete_deployment_event::Result>,
}
/// Nested message and enum types in `CompleteDeploymentEvent`.
pub mod complete_deployment_event {
    /// State of the completed deployment.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The zero value. It is applied when `state` is unset. Do not use.
        Unspecified = 0,
        /// Indicates that a Deployment value was present when the config deployment
        /// finished and the State was anything other than FAILED.
        Succeeded = 1,
        /// Indicates that the Operation result was an error or the Deployment
        /// `state` was FAILED.
        Failed = 2,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        /// Result of the Deployment recorded upon completion.
        #[prost(message, tag="1")]
        Value(super::CompleteDeploymentResult),
        /// Result of the Deployment if the `Operation` results in an error.
        #[prost(message, tag="2")]
        Error(super::super::super::super::super::rpc::Status),
    }
}
/// This message is used when the CompleteDeploymentEvent has a value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteDeploymentResult {
    /// This is the Deployment that completed.
    /// Format is projects/{project}/locations/{location}/deployments/{name}.
    #[prost(string, tag="1")]
    pub deployment: ::prost::alloc::string::String,
    /// This is the Preview that completed.
    /// Format is projects/{project}/locations/{location}/previews/{preview}
    #[prost(string, tag="3")]
    pub preview: ::prost::alloc::string::String,
    /// The message that is returned when a deployment completes. This **can** be
    /// an error message if the `Deployment` `state` is `FAILED`.
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
}
