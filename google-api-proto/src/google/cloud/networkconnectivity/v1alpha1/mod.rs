/// Network Connectivity Center is a hub-and-spoke abstraction for
/// network connectivity management in Google Cloud. It reduces
/// operational complexity through a simple, centralized connectivity management
/// model. Following is the resource message of a hub.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hub {
    /// Immutable. The name of a Hub resource.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Time when the Hub was created.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Time when the Hub was updated.
    #[prost(message, optional, tag="3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// User-defined labels.
    #[prost(btree_map="string, string", tag="4")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Short description of the hub resource.
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
    /// Output only. A list of the URIs of all attached spokes
    #[prost(string, repeated, tag="6")]
    pub spokes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. Google-generated UUID for this resource. This is unique across all Hub
    /// resources. If a Hub resource is deleted and another with the same name is
    /// created, it gets a different unique_id.
    #[prost(string, tag="8")]
    pub unique_id: ::prost::alloc::string::String,
    /// Output only. The current lifecycle state of this Hub.
    #[prost(enumeration="State", tag="9")]
    pub state: i32,
}
/// A Spoke is an  abstraction of a network attachment being attached
/// to a Hub. A Spoke can be underlying a VPN tunnel, a
/// VLAN (interconnect) attachment, a Router appliance, etc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Spoke {
    /// Immutable. The name of a Spoke resource.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The time when the Spoke was created.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time when the Spoke was updated.
    #[prost(message, optional, tag="3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// User-defined labels.
    #[prost(btree_map="string, string", tag="4")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Short description of the spoke resource
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
    /// The resource URL of the hub resource that the spoke is attached to
    #[prost(string, tag="6")]
    pub hub: ::prost::alloc::string::String,
    /// The URIs of linked VPN tunnel resources
    #[prost(string, repeated, tag="12")]
    pub linked_vpn_tunnels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The URIs of linked interconnect attachment resources
    #[prost(string, repeated, tag="13")]
    pub linked_interconnect_attachments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The URIs of linked Router appliance resources
    #[prost(message, repeated, tag="14")]
    pub linked_router_appliance_instances: ::prost::alloc::vec::Vec<RouterApplianceInstance>,
    /// Output only. Google-generated UUID for this resource. This is unique across all Spoke
    /// resources. If a Spoke resource is deleted and another with the same name is
    /// created, it gets a different unique_id.
    #[prost(string, tag="11")]
    pub unique_id: ::prost::alloc::string::String,
    /// Output only. The current lifecycle state of this Hub.
    #[prost(enumeration="State", tag="15")]
    pub state: i32,
}
/// Request for \[HubService.ListHubs][google.cloud.networkconnectivity.v1alpha1.HubService.ListHubs\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHubsRequest {
    /// Required. The parent resource's name.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of results per page that should be returned.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// The page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that filters the results listed in the response.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
    /// Sort the results by a certain order.
    #[prost(string, tag="5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for \[HubService.ListHubs][google.cloud.networkconnectivity.v1alpha1.HubService.ListHubs\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHubsResponse {
    /// Hubs to be returned.
    #[prost(message, repeated, tag="1")]
    pub hubs: ::prost::alloc::vec::Vec<Hub>,
    /// The next pagination token in the List response. It should be used as
    /// page_token for the following request. An empty value means no more result.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag="3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for \[HubService.GetHub][google.cloud.networkconnectivity.v1alpha1.HubService.GetHub\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHubRequest {
    /// Required. Name of the Hub resource to get.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for \[HubService.CreateHub][google.cloud.networkconnectivity.v1alpha1.HubService.CreateHub\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateHubRequest {
    /// Required. The parent resource's name of the Hub.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Unique id for the Hub to create.
    #[prost(string, tag="2")]
    pub hub_id: ::prost::alloc::string::String,
    /// Required. Initial values for a new Hub.
    #[prost(message, optional, tag="3")]
    pub hub: ::core::option::Option<Hub>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag="4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for \[HubService.UpdateHub][google.cloud.networkconnectivity.v1alpha1.HubService.UpdateHub\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateHubRequest {
    /// Optional. Field mask is used to specify the fields to be overwritten in the
    /// Hub resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag="1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The state that the Hub should be in after the update.
    #[prost(message, optional, tag="2")]
    pub hub: ::core::option::Option<Hub>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag="3")]
    pub request_id: ::prost::alloc::string::String,
}
/// The request for \[HubService.DeleteHub][google.cloud.networkconnectivity.v1alpha1.HubService.DeleteHub\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteHubRequest {
    /// Required. The name of the Hub to delete.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag="2")]
    pub request_id: ::prost::alloc::string::String,
}
/// The request for \[HubService.ListSpokes][google.cloud.networkconnectivity.v1alpha1.HubService.ListSpokes\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSpokesRequest {
    /// Required. The parent's resource name.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of results per page that should be returned.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// The page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that filters the results listed in the response.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
    /// Sort the results by a certain order.
    #[prost(string, tag="5")]
    pub order_by: ::prost::alloc::string::String,
}
/// The response for \[HubService.ListSpokes][google.cloud.networkconnectivity.v1alpha1.HubService.ListSpokes\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSpokesResponse {
    /// Spokes to be returned.
    #[prost(message, repeated, tag="1")]
    pub spokes: ::prost::alloc::vec::Vec<Spoke>,
    /// The next pagination token in the List response. It should be used as
    /// page_token for the following request. An empty value means no more result.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag="3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The request for \[HubService.GetSpoke][google.cloud.networkconnectivity.v1alpha1.HubService.GetSpoke\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSpokeRequest {
    /// Required. The name of Spoke resource.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// The request for \[HubService.CreateSpoke][google.cloud.networkconnectivity.v1alpha1.HubService.CreateSpoke\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSpokeRequest {
    /// Required. The parent's resource name of the Spoke.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Unique id for the Spoke to create.
    #[prost(string, tag="2")]
    pub spoke_id: ::prost::alloc::string::String,
    /// Required. Initial values for a new Hub.
    #[prost(message, optional, tag="3")]
    pub spoke: ::core::option::Option<Spoke>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag="4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for \[HubService.UpdateSpoke][google.cloud.networkconnectivity.v1alpha1.HubService.UpdateSpoke\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSpokeRequest {
    /// Optional. Field mask is used to specify the fields to be overwritten in the
    /// Spoke resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag="1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The state that the Spoke should be in after the update.
    #[prost(message, optional, tag="2")]
    pub spoke: ::core::option::Option<Spoke>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag="3")]
    pub request_id: ::prost::alloc::string::String,
}
/// The request for \[HubService.DeleteSpoke][google.cloud.networkconnectivity.v1alpha1.HubService.DeleteSpoke\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSpokeRequest {
    /// Required. The name of the Spoke to delete.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag="2")]
    pub request_id: ::prost::alloc::string::String,
}
/// RouterAppliance represents a Router appliance which is specified by a VM URI
/// and a NIC address.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouterApplianceInstance {
    /// The URI of the virtual machine resource
    #[prost(string, tag="1")]
    pub virtual_machine: ::prost::alloc::string::String,
    /// The IP address of the network interface to use for peering.
    #[prost(string, tag="3")]
    pub ip_address: ::prost::alloc::string::String,
    #[deprecated]
    #[prost(string, tag="2")]
    pub network_interface: ::prost::alloc::string::String,
}
/// The State enum represents the lifecycle of a Network Connectivity Center
/// resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum State {
    /// No state information available
    Unspecified = 0,
    /// The resource's create operation is in progress
    Creating = 1,
    /// The resource is active
    Active = 2,
    /// The resource's Delete operation is in progress
    Deleting = 3,
}
impl State {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            State::Unspecified => "STATE_UNSPECIFIED",
            State::Creating => "CREATING",
            State::Active => "ACTIVE",
            State::Deleting => "DELETING",
        }
    }
}
/// Generated client implementations.
pub mod hub_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Network Connectivity Center is a hub-and-spoke abstraction for
    /// network connectivity management in Google Cloud. It reduces
    /// operational complexity through a simple, centralized connectivity management
    /// model.
    #[derive(Debug, Clone)]
    pub struct HubServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> HubServiceClient<T>
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
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> HubServiceClient<InterceptedService<T, F>>
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
            HubServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Lists Hubs in a given project and location.
        pub async fn list_hubs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListHubsRequest>,
        ) -> Result<tonic::Response<super::ListHubsResponse>, tonic::Status> {
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
                "/google.cloud.networkconnectivity.v1alpha1.HubService/ListHubs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single Hub.
        pub async fn get_hub(
            &mut self,
            request: impl tonic::IntoRequest<super::GetHubRequest>,
        ) -> Result<tonic::Response<super::Hub>, tonic::Status> {
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
                "/google.cloud.networkconnectivity.v1alpha1.HubService/GetHub",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new Hub in a given project and location.
        pub async fn create_hub(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateHubRequest>,
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
                "/google.cloud.networkconnectivity.v1alpha1.HubService/CreateHub",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of a single Hub.
        pub async fn update_hub(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateHubRequest>,
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
                "/google.cloud.networkconnectivity.v1alpha1.HubService/UpdateHub",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single Hub.
        pub async fn delete_hub(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteHubRequest>,
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
                "/google.cloud.networkconnectivity.v1alpha1.HubService/DeleteHub",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists Spokes in a given project and location.
        pub async fn list_spokes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSpokesRequest>,
        ) -> Result<tonic::Response<super::ListSpokesResponse>, tonic::Status> {
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
                "/google.cloud.networkconnectivity.v1alpha1.HubService/ListSpokes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single Spoke.
        pub async fn get_spoke(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSpokeRequest>,
        ) -> Result<tonic::Response<super::Spoke>, tonic::Status> {
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
                "/google.cloud.networkconnectivity.v1alpha1.HubService/GetSpoke",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new Spoke in a given project and location.
        pub async fn create_spoke(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSpokeRequest>,
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
                "/google.cloud.networkconnectivity.v1alpha1.HubService/CreateSpoke",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of a single Spoke.
        pub async fn update_spoke(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSpokeRequest>,
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
                "/google.cloud.networkconnectivity.v1alpha1.HubService/UpdateSpoke",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single Spoke.
        pub async fn delete_spoke(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSpokeRequest>,
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
                "/google.cloud.networkconnectivity.v1alpha1.HubService/DeleteSpoke",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Represents the metadata of the long-running operation.
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
