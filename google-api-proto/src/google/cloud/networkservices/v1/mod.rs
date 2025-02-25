/// ServiceBinding is the resource that defines a Service Directory Service to
/// be used in a BackendService resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceBinding {
    /// Required. Name of the ServiceBinding resource. It matches pattern
    /// `projects/*/locations/global/serviceBindings/service_binding_name>`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A free-text description of the resource. Max length 1024 characters.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The timestamp when the resource was created.
    #[prost(message, optional, tag="3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the resource was updated.
    #[prost(message, optional, tag="4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. The full service directory service name of the format
    /// /projects/*/locations/*/namespaces/*/services/*
    #[prost(string, tag="5")]
    pub service: ::prost::alloc::string::String,
    /// Optional. Set of label tags associated with the ServiceBinding resource.
    #[prost(btree_map="string, string", tag="7")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Request used with the ListServiceBindings method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServiceBindingsRequest {
    /// Required. The project and location from which the ServiceBindings should be
    /// listed, specified in the format `projects/*/locations/global`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of ServiceBindings to return per call.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// The value returned by the last `ListServiceBindingsResponse`
    /// Indicates that this is a continuation of a prior `ListRouters` call,
    /// and that the system should return the next page of data.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response returned by the ListServiceBindings method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServiceBindingsResponse {
    /// List of ServiceBinding resources.
    #[prost(message, repeated, tag="1")]
    pub service_bindings: ::prost::alloc::vec::Vec<ServiceBinding>,
    /// If there might be more results than those appearing in this response, then
    /// `next_page_token` is included. To get the next set of results, call this
    /// method again using the value of `next_page_token` as `page_token`.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request used by the GetServiceBinding method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceBindingRequest {
    /// Required. A name of the ServiceBinding to get. Must be in the format
    /// `projects/*/locations/global/serviceBindings/*`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request used by the ServiceBinding method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateServiceBindingRequest {
    /// Required. The parent resource of the ServiceBinding. Must be in the
    /// format `projects/*/locations/global`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Short name of the ServiceBinding resource to be created.
    #[prost(string, tag="2")]
    pub service_binding_id: ::prost::alloc::string::String,
    /// Required. ServiceBinding resource to be created.
    #[prost(message, optional, tag="3")]
    pub service_binding: ::core::option::Option<ServiceBinding>,
}
/// Request used by the DeleteServiceBinding method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServiceBindingRequest {
    /// Required. A name of the ServiceBinding to delete. Must be in the format
    /// `projects/*/locations/global/serviceBindings/*`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
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
/// Specification of a port-based selector.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrafficPortSelector {
    /// Optional. A list of ports. Can be port numbers or port range
    /// (example, \[80-90\] specifies all ports from 80 to 90, including
    /// 80 and 90) or named ports or * to specify all ports. If the
    /// list is empty, all ports are selected.
    #[prost(string, repeated, tag="1")]
    pub ports: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A definition of a matcher that selects endpoints to which the policies
/// should be applied.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndpointMatcher {
    /// Specifies type of the matcher used for this endpoint matcher.
    #[prost(oneof="endpoint_matcher::MatcherType", tags="1")]
    pub matcher_type: ::core::option::Option<endpoint_matcher::MatcherType>,
}
/// Nested message and enum types in `EndpointMatcher`.
pub mod endpoint_matcher {
    /// The matcher that is based on node metadata presented by xDS clients.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MetadataLabelMatcher {
        /// Specifies how matching should be done.
        ///
        /// Supported values are:
        /// MATCH_ANY: At least one of the Labels specified in the
        ///   matcher should match the metadata presented by xDS client.
        /// MATCH_ALL: The metadata presented by the xDS client should
        ///   contain all of the labels specified here.
        ///
        /// The selection is determined based on the best match. For
        /// example, suppose there are three EndpointPolicy
        /// resources P1, P2 and P3 and if P1 has a the matcher as
        /// MATCH_ANY <A:1, B:1>, P2 has MATCH_ALL <A:1,B:1>, and P3 has
        /// MATCH_ALL <A:1,B:1,C:1>.
        ///
        /// If a client with label <A:1> connects, the config from P1
        /// will be selected.
        ///
        /// If a client with label <A:1,B:1> connects, the config from P2
        /// will be selected.
        ///
        /// If a client with label <A:1,B:1,C:1> connects, the config
        /// from P3 will be selected.
        ///
        /// If there is more than one best match, (for example, if a
        /// config P4 with selector <A:1,D:1> exists and if a client with
        /// label <A:1,B:1,D:1> connects), an error will be thrown.
        #[prost(enumeration="metadata_label_matcher::MetadataLabelMatchCriteria", tag="1")]
        pub metadata_label_match_criteria: i32,
        /// The list of label value pairs that must match labels in the
        /// provided metadata based on filterMatchCriteria This list can
        /// have at most 64 entries. The list can be empty if the match
        /// criteria is MATCH_ANY, to specify a wildcard match (i.e this
        /// matches any client).
        #[prost(message, repeated, tag="2")]
        pub metadata_labels: ::prost::alloc::vec::Vec<metadata_label_matcher::MetadataLabels>,
    }
    /// Nested message and enum types in `MetadataLabelMatcher`.
    pub mod metadata_label_matcher {
        /// Defines a name-pair value for a single label.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MetadataLabels {
            /// Required. Label name presented as key in xDS Node Metadata.
            #[prost(string, tag="1")]
            pub label_name: ::prost::alloc::string::String,
            /// Required. Label value presented as value corresponding to the above
            /// key, in xDS Node Metadata.
            #[prost(string, tag="2")]
            pub label_value: ::prost::alloc::string::String,
        }
        /// Possible criteria values that define logic of how matching is made.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum MetadataLabelMatchCriteria {
            /// Default value. Should not be used.
            Unspecified = 0,
            /// At least one of the Labels specified in the matcher should match the
            /// metadata presented by xDS client.
            MatchAny = 1,
            /// The metadata presented by the xDS client should contain all of the
            /// labels specified here.
            MatchAll = 2,
        }
    }
    /// Specifies type of the matcher used for this endpoint matcher.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MatcherType {
        /// The matcher is based on node metadata presented by xDS clients.
        #[prost(message, tag="1")]
        MetadataLabelMatcher(MetadataLabelMatcher),
    }
}
/// EndpointPolicy is a resource that helps apply desired configuration
/// on the endpoints that match specific criteria.
/// For example, this resource can be used to apply "authentication config"
/// an all endpoints that serve on port 8080.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndpointPolicy {
    /// Required. Name of the EndpointPolicy resource. It matches pattern
    /// `projects/{project}/locations/global/endpointPolicies/{endpoint_policy}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The timestamp when the resource was created.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the resource was updated.
    #[prost(message, optional, tag="3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Set of label tags associated with the EndpointPolicy resource.
    #[prost(btree_map="string, string", tag="4")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Required. The type of endpoint policy. This is primarily used to validate
    /// the configuration.
    #[prost(enumeration="endpoint_policy::EndpointPolicyType", tag="5")]
    pub r#type: i32,
    /// Optional. This field specifies the URL of AuthorizationPolicy resource that
    /// applies authorization policies to the inbound traffic at the
    /// matched endpoints. Refer to Authorization. If this field is not
    /// specified, authorization is disabled(no authz checks) for this
    /// endpoint.
    #[prost(string, tag="7")]
    pub authorization_policy: ::prost::alloc::string::String,
    /// Required. A matcher that selects endpoints to which the policies should be applied.
    #[prost(message, optional, tag="9")]
    pub endpoint_matcher: ::core::option::Option<EndpointMatcher>,
    /// Optional. Port selector for the (matched) endpoints. If no port selector is
    /// provided, the matched config is applied to all ports.
    #[prost(message, optional, tag="10")]
    pub traffic_port_selector: ::core::option::Option<TrafficPortSelector>,
    /// Optional. A free-text description of the resource. Max length 1024 characters.
    #[prost(string, tag="11")]
    pub description: ::prost::alloc::string::String,
    /// Optional. A URL referring to ServerTlsPolicy resource. ServerTlsPolicy is used to
    /// determine the authentication policy to be applied to terminate the inbound
    /// traffic at the identified backends. If this field is not set,
    /// authentication is disabled(open) for this endpoint.
    #[prost(string, tag="12")]
    pub server_tls_policy: ::prost::alloc::string::String,
    /// Optional. A URL referring to a ClientTlsPolicy resource. ClientTlsPolicy can be set
    /// to specify the authentication for traffic from the proxy to the actual
    /// endpoints. More specifically, it is applied to the outgoing traffic from
    /// the proxy to the endpoint. This is typically used for sidecar model where
    /// the proxy identifies itself as endpoint to the control plane, with the
    /// connection between sidecar and endpoint requiring authentication. If this
    /// field is not set, authentication is disabled(open). Applicable only when
    /// EndpointPolicyType is SIDECAR_PROXY.
    #[prost(string, tag="13")]
    pub client_tls_policy: ::prost::alloc::string::String,
}
/// Nested message and enum types in `EndpointPolicy`.
pub mod endpoint_policy {
    /// The type of endpoint policy.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EndpointPolicyType {
        /// Default value. Must not be used.
        Unspecified = 0,
        /// Represents a proxy deployed as a sidecar.
        SidecarProxy = 1,
        /// Represents a proxyless gRPC backend.
        GrpcServer = 2,
    }
}
/// Request used with the ListEndpointPolicies method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEndpointPoliciesRequest {
    /// Required. The project and location from which the EndpointPolicies should be
    /// listed, specified in the format `projects/*/locations/global`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of EndpointPolicies to return per call.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// The value returned by the last `ListEndpointPoliciesResponse`
    /// Indicates that this is a continuation of a prior
    /// `ListEndpointPolicies` call, and that the system should return the
    /// next page of data.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response returned by the ListEndpointPolicies method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEndpointPoliciesResponse {
    /// List of EndpointPolicy resources.
    #[prost(message, repeated, tag="1")]
    pub endpoint_policies: ::prost::alloc::vec::Vec<EndpointPolicy>,
    /// If there might be more results than those appearing in this response, then
    /// `next_page_token` is included. To get the next set of results, call this
    /// method again using the value of `next_page_token` as `page_token`.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request used with the GetEndpointPolicy method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEndpointPolicyRequest {
    /// Required. A name of the EndpointPolicy to get. Must be in the format
    /// `projects/*/locations/global/endpointPolicies/*`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request used with the CreateEndpointPolicy method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEndpointPolicyRequest {
    /// Required. The parent resource of the EndpointPolicy. Must be in the
    /// format `projects/*/locations/global`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Short name of the EndpointPolicy resource to be created.
    /// E.g. "CustomECS".
    #[prost(string, tag="2")]
    pub endpoint_policy_id: ::prost::alloc::string::String,
    /// Required. EndpointPolicy resource to be created.
    #[prost(message, optional, tag="3")]
    pub endpoint_policy: ::core::option::Option<EndpointPolicy>,
}
/// Request used with the UpdateEndpointPolicy method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEndpointPolicyRequest {
    /// Optional. Field mask is used to specify the fields to be overwritten in the
    /// EndpointPolicy resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag="1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Updated EndpointPolicy resource.
    #[prost(message, optional, tag="2")]
    pub endpoint_policy: ::core::option::Option<EndpointPolicy>,
}
/// Request used with the DeleteEndpointPolicy method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEndpointPolicyRequest {
    /// Required. A name of the EndpointPolicy to delete. Must be in the format
    /// `projects/*/locations/global/endpointPolicies/*`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Mesh represents a logical configuration grouping for workload to workload
/// communication within a service mesh. Routes that point to mesh dictate how
/// requests are routed within this logical mesh boundary.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mesh {
    /// Required. Name of the Mesh resource. It matches pattern
    /// `projects/*/locations/global/meshes/<mesh_name>`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Server-defined URL of this resource
    #[prost(string, tag="9")]
    pub self_link: ::prost::alloc::string::String,
    /// Output only. The timestamp when the resource was created.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the resource was updated.
    #[prost(message, optional, tag="3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Set of label tags associated with the Mesh resource.
    #[prost(btree_map="string, string", tag="4")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. A free-text description of the resource. Max length 1024 characters.
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
    /// Optional. If set to a valid TCP port (1-65535), instructs the SIDECAR proxy to listen
    /// on the specified port of localhost (127.0.0.1) address. The SIDECAR proxy
    /// will expect all traffic to be redirected to this port regardless of its
    /// actual ip:port destination. If unset, a port '15001' is used as the
    /// interception port. This will is applicable only for sidecar proxy
    /// deployments.
    #[prost(int32, tag="8")]
    pub interception_port: i32,
}
/// Request used with the ListMeshes method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMeshesRequest {
    /// Required. The project and location from which the Meshes should be
    /// listed, specified in the format `projects/*/locations/global`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of Meshes to return per call.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// The value returned by the last `ListMeshesResponse`
    /// Indicates that this is a continuation of a prior `ListMeshes` call,
    /// and that the system should return the next page of data.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response returned by the ListMeshes method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMeshesResponse {
    /// List of Mesh resources.
    #[prost(message, repeated, tag="1")]
    pub meshes: ::prost::alloc::vec::Vec<Mesh>,
    /// If there might be more results than those appearing in this response, then
    /// `next_page_token` is included. To get the next set of results, call this
    /// method again using the value of `next_page_token` as `page_token`.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request used by the GetMesh method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMeshRequest {
    /// Required. A name of the Mesh to get. Must be in the format
    /// `projects/*/locations/global/meshes/*`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request used by the CreateMesh method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMeshRequest {
    /// Required. The parent resource of the Mesh. Must be in the
    /// format `projects/*/locations/global`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Short name of the Mesh resource to be created.
    #[prost(string, tag="2")]
    pub mesh_id: ::prost::alloc::string::String,
    /// Required. Mesh resource to be created.
    #[prost(message, optional, tag="3")]
    pub mesh: ::core::option::Option<Mesh>,
}
/// Request used by the UpdateMesh method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMeshRequest {
    /// Optional. Field mask is used to specify the fields to be overwritten in the
    /// Mesh resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag="1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Updated Mesh resource.
    #[prost(message, optional, tag="2")]
    pub mesh: ::core::option::Option<Mesh>,
}
/// Request used by the DeleteMesh method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMeshRequest {
    /// Required. A name of the Mesh to delete. Must be in the format
    /// `projects/*/locations/global/meshes/*`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// TlsRoute defines how traffic should be routed based on SNI and other matching
/// L3 attributes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TlsRoute {
    /// Required. Name of the TlsRoute resource. It matches pattern
    /// `projects/*/locations/global/tlsRoutes/tls_route_name>`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Server-defined URL of this resource
    #[prost(string, tag="8")]
    pub self_link: ::prost::alloc::string::String,
    /// Output only. The timestamp when the resource was created.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the resource was updated.
    #[prost(message, optional, tag="3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. A free-text description of the resource. Max length 1024 characters.
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
    /// Required. Rules that define how traffic is routed and handled. At least one RouteRule
    /// must be supplied. If there are multiple rules then the action taken will be
    /// the first rule to match.
    #[prost(message, repeated, tag="5")]
    pub rules: ::prost::alloc::vec::Vec<tls_route::RouteRule>,
    /// Optional. Meshes defines a list of meshes this TlsRoute is attached to, as one of the
    /// routing rules to route the requests served by the mesh.
    ///
    /// Each mesh reference should match the pattern:
    /// `projects/*/locations/global/meshes/<mesh_name>`
    ///
    /// The attached Mesh should be of a type SIDECAR
    #[prost(string, repeated, tag="6")]
    pub meshes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Gateways defines a list of gateways this TlsRoute is attached to, as one of
    /// the routing rules to route the requests served by the gateway.
    ///
    /// Each gateway reference should match the pattern:
    /// `projects/*/locations/global/gateways/<gateway_name>`
    #[prost(string, repeated, tag="7")]
    pub gateways: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `TlsRoute`.
pub mod tls_route {
    /// Specifies how to match traffic and how to route traffic when traffic is
    /// matched.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RouteRule {
        /// Required. RouteMatch defines the predicate used to match requests to a given
        /// action. Multiple match types are "OR"ed for evaluation.
        #[prost(message, repeated, tag="1")]
        pub matches: ::prost::alloc::vec::Vec<RouteMatch>,
        /// Required. The detailed rule defining how to route matched traffic.
        #[prost(message, optional, tag="2")]
        pub action: ::core::option::Option<RouteAction>,
    }
    /// RouteMatch defines the predicate used to match requests to a given action.
    /// Multiple match types are "AND"ed for evaluation.
    /// If no routeMatch field is specified, this rule will unconditionally match
    /// traffic.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RouteMatch {
        /// Optional. SNI (server name indicator) to match against.
        /// SNI will be matched against all wildcard domains, i.e. www.example.com
        /// will be first matched against www.example.com, then *.example.com, then
        /// *.com.
        /// Partial wildcards are not supported, and values like *w.example.com are
        /// invalid.
        /// At least one of sni_host and alpn is required.
        /// Up to 5 sni hosts across all matches can be set.
        #[prost(string, repeated, tag="1")]
        pub sni_host: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Optional. ALPN (Application-Layer Protocol Negotiation) to match against.
        /// Examples: "http/1.1", "h2".
        /// At least one of sni_host and alpn is required.
        /// Up to 5 alpns across all matches can be set.
        #[prost(string, repeated, tag="2")]
        pub alpn: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// The specifications for routing traffic and applying associated policies.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RouteAction {
        /// Required. The destination services to which traffic should be forwarded. At least
        /// one destination service is required.
        #[prost(message, repeated, tag="1")]
        pub destinations: ::prost::alloc::vec::Vec<RouteDestination>,
    }
    /// Describe the destination for traffic to be routed to.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RouteDestination {
        /// Required. The URL of a BackendService to route traffic to.
        #[prost(string, tag="1")]
        pub service_name: ::prost::alloc::string::String,
        /// Optional. Specifies the proportion of requests forwareded to the backend referenced
        /// by the service_name field. This is computed as:
        ///         weight/Sum(weights in destinations)
        /// Weights in all destinations does not need to sum up to 100.
        #[prost(int32, tag="2")]
        pub weight: i32,
    }
}
/// Request used with the ListTlsRoutes method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTlsRoutesRequest {
    /// Required. The project and location from which the TlsRoutes should be
    /// listed, specified in the format `projects/*/locations/global`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of TlsRoutes to return per call.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// The value returned by the last `ListTlsRoutesResponse`
    /// Indicates that this is a continuation of a prior `ListTlsRoutes` call,
    /// and that the system should return the next page of data.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response returned by the ListTlsRoutes method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTlsRoutesResponse {
    /// List of TlsRoute resources.
    #[prost(message, repeated, tag="1")]
    pub tls_routes: ::prost::alloc::vec::Vec<TlsRoute>,
    /// If there might be more results than those appearing in this response, then
    /// `next_page_token` is included. To get the next set of results, call this
    /// method again using the value of `next_page_token` as `page_token`.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request used by the GetTlsRoute method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTlsRouteRequest {
    /// Required. A name of the TlsRoute to get. Must be in the format
    /// `projects/*/locations/global/tlsRoutes/*`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request used by the TlsRoute method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTlsRouteRequest {
    /// Required. The parent resource of the TlsRoute. Must be in the
    /// format `projects/*/locations/global`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Short name of the TlsRoute resource to be created. E.g. TODO(Add an
    /// example).
    #[prost(string, tag="2")]
    pub tls_route_id: ::prost::alloc::string::String,
    /// Required. TlsRoute resource to be created.
    #[prost(message, optional, tag="3")]
    pub tls_route: ::core::option::Option<TlsRoute>,
}
/// Request used by the UpdateTlsRoute method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTlsRouteRequest {
    /// Optional. Field mask is used to specify the fields to be overwritten in the
    /// TlsRoute resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag="1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Updated TlsRoute resource.
    #[prost(message, optional, tag="2")]
    pub tls_route: ::core::option::Option<TlsRoute>,
}
/// Request used by the DeleteTlsRoute method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTlsRouteRequest {
    /// Required. A name of the TlsRoute to delete. Must be in the format
    /// `projects/*/locations/global/tlsRoutes/*`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Gateway represents the configuration for a proxy, typically a load balancer.
/// It captures the ip:port over which the services are exposed by the proxy,
/// along with any policy configurations. Routes have reference to to Gateways to
/// dictate how requests should be routed by this Gateway.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Gateway {
    /// Required. Name of the Gateway resource. It matches pattern
    /// `projects/*/locations/*/gateways/<gateway_name>`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Server-defined URL of this resource
    #[prost(string, tag="13")]
    pub self_link: ::prost::alloc::string::String,
    /// Output only. The timestamp when the resource was created.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the resource was updated.
    #[prost(message, optional, tag="3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Set of label tags associated with the Gateway resource.
    #[prost(btree_map="string, string", tag="4")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. A free-text description of the resource. Max length 1024 characters.
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
    /// Immutable. The type of the customer managed gateway.
    /// This field is required. If unspecified, an error is returned.
    #[prost(enumeration="gateway::Type", tag="6")]
    pub r#type: i32,
    /// Required. One or more ports that the Gateway must receive traffic on. The proxy binds
    /// to the ports specified. Gateway listen on 0.0.0.0 on the ports specified
    /// below.
    #[prost(int32, repeated, packed="false", tag="11")]
    pub ports: ::prost::alloc::vec::Vec<i32>,
    /// Required. Immutable. Scope determines how configuration across multiple Gateway instances are
    /// merged. The configuration for multiple Gateway instances with the same
    /// scope will be merged as presented as a single coniguration to the
    /// proxy/load balancer.
    ///
    /// Max length 64 characters.
    /// Scope should start with a letter and can only have letters, numbers,
    /// hyphens.
    #[prost(string, tag="8")]
    pub scope: ::prost::alloc::string::String,
    /// Optional. A fully-qualified ServerTLSPolicy URL reference. Specifies how TLS
    /// traffic is terminated. If empty, TLS termination is disabled.
    #[prost(string, tag="9")]
    pub server_tls_policy: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Gateway`.
pub mod gateway {
    /// The type of the customer-managed gateway.
    /// Possible values are:
    /// * OPEN_MESH
    /// * SECURE_WEB_GATEWAY
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// The type of the customer managed gateway is unspecified.
        Unspecified = 0,
        /// The type of the customer managed gateway is TrafficDirector Open
        /// Mesh.
        OpenMesh = 1,
        /// The type of the customer managed gateway is SecureWebGateway (SWG).
        SecureWebGateway = 2,
    }
}
/// Request used with the ListGateways method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGatewaysRequest {
    /// Required. The project and location from which the Gateways should be
    /// listed, specified in the format `projects/*/locations/*`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of Gateways to return per call.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// The value returned by the last `ListGatewaysResponse`
    /// Indicates that this is a continuation of a prior `ListGateways` call,
    /// and that the system should return the next page of data.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response returned by the ListGateways method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGatewaysResponse {
    /// List of Gateway resources.
    #[prost(message, repeated, tag="1")]
    pub gateways: ::prost::alloc::vec::Vec<Gateway>,
    /// If there might be more results than those appearing in this response, then
    /// `next_page_token` is included. To get the next set of results, call this
    /// method again using the value of `next_page_token` as `page_token`.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request used by the GetGateway method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGatewayRequest {
    /// Required. A name of the Gateway to get. Must be in the format
    /// `projects/*/locations/*/gateways/*`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request used by the CreateGateway method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGatewayRequest {
    /// Required. The parent resource of the Gateway. Must be in the
    /// format `projects/*/locations/*`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Short name of the Gateway resource to be created.
    #[prost(string, tag="2")]
    pub gateway_id: ::prost::alloc::string::String,
    /// Required. Gateway resource to be created.
    #[prost(message, optional, tag="3")]
    pub gateway: ::core::option::Option<Gateway>,
}
/// Request used by the UpdateGateway method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGatewayRequest {
    /// Optional. Field mask is used to specify the fields to be overwritten in the
    /// Gateway resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag="1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Updated Gateway resource.
    #[prost(message, optional, tag="2")]
    pub gateway: ::core::option::Option<Gateway>,
}
/// Request used by the DeleteGateway method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteGatewayRequest {
    /// Required. A name of the Gateway to delete. Must be in the format
    /// `projects/*/locations/*/gateways/*`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// GrpcRoute is the resource defining how gRPC traffic routed by a Mesh
/// or Gateway resource is routed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrpcRoute {
    /// Required. Name of the GrpcRoute resource. It matches pattern
    /// `projects/*/locations/global/grpcRoutes/<grpc_route_name>`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Server-defined URL of this resource
    #[prost(string, tag="12")]
    pub self_link: ::prost::alloc::string::String,
    /// Output only. The timestamp when the resource was created.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the resource was updated.
    #[prost(message, optional, tag="3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Set of label tags associated with the GrpcRoute resource.
    #[prost(btree_map="string, string", tag="4")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. A free-text description of the resource. Max length 1024 characters.
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
    /// Required. Service hostnames with an optional port for which this route describes
    /// traffic.
    ///
    /// Format: <hostname>\[:<port>\]
    ///
    /// Hostname is the fully qualified domain name of a network host. This matches
    /// the RFC 1123 definition of a hostname with 2 notable exceptions:
    ///  - IPs are not allowed.
    ///  - A hostname may be prefixed with a wildcard label (*.). The wildcard
    ///    label must appear by itself as the first label.
    ///
    /// Hostname can be "precise" which is a domain name without the terminating
    /// dot of a network host (e.g. "foo.example.com") or "wildcard", which is a
    /// domain name prefixed with a single wildcard label (e.g. *.example.com).
    ///
    /// Note that as per RFC1035 and RFC1123, a label must consist of lower case
    /// alphanumeric characters or '-', and must start and end with an alphanumeric
    /// character. No other punctuation is allowed.
    ///
    /// The routes associated with a Mesh or Gateway must have unique hostnames. If
    /// you attempt to attach multiple routes with conflicting hostnames, the
    /// configuration will be rejected.
    ///
    /// For example, while it is acceptable for routes for the hostnames
    /// "*.foo.bar.com" and "*.bar.com" to be associated with the same route, it is
    /// not possible to associate two routes both with "*.bar.com" or both with
    /// "bar.com".
    ///
    /// If a port is specified, then gRPC clients must use the channel URI with the
    /// port to match this rule (i.e. "xds:///service:123"), otherwise they must
    /// supply the URI without a port (i.e. "xds:///service").
    #[prost(string, repeated, tag="6")]
    pub hostnames: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Meshes defines a list of meshes this GrpcRoute is attached to, as one of
    /// the routing rules to route the requests served by the mesh.
    ///
    /// Each mesh reference should match the pattern:
    /// `projects/*/locations/global/meshes/<mesh_name>`
    #[prost(string, repeated, tag="9")]
    pub meshes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Gateways defines a list of gateways this GrpcRoute is attached to, as one
    /// of the routing rules to route the requests served by the gateway.
    ///
    /// Each gateway reference should match the pattern:
    /// `projects/*/locations/global/gateways/<gateway_name>`
    #[prost(string, repeated, tag="10")]
    pub gateways: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. A list of detailed rules defining how to route traffic.
    ///
    /// Within a single GrpcRoute, the GrpcRoute.RouteAction associated with the
    /// first matching GrpcRoute.RouteRule will be executed. At least one rule
    /// must be supplied.
    #[prost(message, repeated, tag="7")]
    pub rules: ::prost::alloc::vec::Vec<grpc_route::RouteRule>,
}
/// Nested message and enum types in `GrpcRoute`.
pub mod grpc_route {
    /// Specifies a match against a method.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MethodMatch {
        /// Optional. Specifies how to match against the name. If not specified, a default
        /// value of "EXACT" is used.
        #[prost(enumeration="method_match::Type", tag="1")]
        pub r#type: i32,
        /// Required. Name of the service to match against. If unspecified, will match all
        /// services.
        #[prost(string, tag="2")]
        pub grpc_service: ::prost::alloc::string::String,
        /// Required. Name of the method to match against. If unspecified, will match all
        /// methods.
        #[prost(string, tag="3")]
        pub grpc_method: ::prost::alloc::string::String,
        /// Optional. Specifies that matches are case sensitive.  The default value is true.
        /// case_sensitive must not be used with a type of REGULAR_EXPRESSION.
        #[prost(bool, optional, tag="4")]
        pub case_sensitive: ::core::option::Option<bool>,
    }
    /// Nested message and enum types in `MethodMatch`.
    pub mod method_match {
        /// The type of the match.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Type {
            /// Unspecified.
            Unspecified = 0,
            /// Will only match the exact name provided.
            Exact = 1,
            /// Will interpret grpc_method and grpc_service as regexes. RE2 syntax is
            /// supported.
            RegularExpression = 2,
        }
    }
    /// A match against a collection of headers.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HeaderMatch {
        /// Optional. Specifies how to match against the value of the header. If not
        /// specified, a default value of EXACT is used.
        #[prost(enumeration="header_match::Type", tag="1")]
        pub r#type: i32,
        /// Required. The key of the header.
        #[prost(string, tag="2")]
        pub key: ::prost::alloc::string::String,
        /// Required. The value of the header.
        #[prost(string, tag="3")]
        pub value: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `HeaderMatch`.
    pub mod header_match {
        /// The type of match.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Type {
            /// Unspecified.
            Unspecified = 0,
            /// Will only match the exact value provided.
            Exact = 1,
            /// Will match paths conforming to the prefix specified by value. RE2
            /// syntax is supported.
            RegularExpression = 2,
        }
    }
    /// Criteria for matching traffic. A RouteMatch will be considered to match
    /// when all supplied fields match.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RouteMatch {
        /// Optional. A gRPC method to match against. If this field is empty or omitted, will
        /// match all methods.
        #[prost(message, optional, tag="1")]
        pub method: ::core::option::Option<MethodMatch>,
        /// Optional. Specifies a collection of headers to match.
        #[prost(message, repeated, tag="2")]
        pub headers: ::prost::alloc::vec::Vec<HeaderMatch>,
    }
    /// The destination to which traffic will be routed.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Destination {
        /// Optional. Specifies the proportion of requests forwarded to the backend referenced
        /// by the serviceName field. This is computed as:
        ///         weight/Sum(weights in this destination list).
        /// For non-zero values, there may be some epsilon from the exact proportion
        /// defined here depending on the precision an implementation supports.
        ///
        /// If only one serviceName is specified and it has a weight greater than 0,
        /// 100% of the traffic is forwarded to that backend.
        ///
        /// If weights are specified for any one service name, they need to be
        /// specified for all of them.
        ///
        /// If weights are unspecified for all services, then, traffic is distributed
        /// in equal proportions to all of them.
        #[prost(int32, optional, tag="2")]
        pub weight: ::core::option::Option<i32>,
        /// Specifies the kind of destination to which traffic will be routed.
        #[prost(oneof="destination::DestinationType", tags="1")]
        pub destination_type: ::core::option::Option<destination::DestinationType>,
    }
    /// Nested message and enum types in `Destination`.
    pub mod destination {
        /// Specifies the kind of destination to which traffic will be routed.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum DestinationType {
            /// Required. The URL of a destination service to which to route traffic. Must refer
            /// to either a BackendService or ServiceDirectoryService.
            #[prost(string, tag="1")]
            ServiceName(::prost::alloc::string::String),
        }
    }
    /// The specification for fault injection introduced into traffic to test the
    /// resiliency of clients to destination service failure. As part of fault
    /// injection, when clients send requests to a destination, delays can be
    /// introduced on a percentage of requests before sending those requests to the
    /// destination service. Similarly requests from clients can be aborted by for
    /// a percentage of requests.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FaultInjectionPolicy {
        /// The specification for injecting delay to client requests.
        #[prost(message, optional, tag="1")]
        pub delay: ::core::option::Option<fault_injection_policy::Delay>,
        /// The specification for aborting to client requests.
        #[prost(message, optional, tag="2")]
        pub abort: ::core::option::Option<fault_injection_policy::Abort>,
    }
    /// Nested message and enum types in `FaultInjectionPolicy`.
    pub mod fault_injection_policy {
        /// Specification of how client requests are delayed as part of fault
        /// injection before being sent to a destination.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Delay {
            /// Specify a fixed delay before forwarding the request.
            #[prost(message, optional, tag="1")]
            pub fixed_delay: ::core::option::Option<::prost_types::Duration>,
            /// The percentage of traffic on which delay will be injected.
            ///
            /// The value must be between [0, 100]
            #[prost(int32, optional, tag="2")]
            pub percentage: ::core::option::Option<i32>,
        }
        /// Specification of how client requests are aborted as part of fault
        /// injection before being sent to a destination.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Abort {
            /// The HTTP status code used to abort the request.
            ///
            /// The value must be between 200 and 599 inclusive.
            #[prost(int32, optional, tag="1")]
            pub http_status: ::core::option::Option<i32>,
            /// The percentage of traffic which will be aborted.
            ///
            /// The value must be between [0, 100]
            #[prost(int32, optional, tag="2")]
            pub percentage: ::core::option::Option<i32>,
        }
    }
    /// The specifications for retries.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RetryPolicy {
        /// - connect-failure: Router will retry on failures connecting to Backend
        ///    Services, for example due to connection timeouts.
        /// - refused-stream: Router will retry if the backend service resets the
        /// stream
        ///    with a REFUSED_STREAM error code. This reset type indicates that it is
        ///    safe to retry.
        /// - cancelled: Router will retry if the gRPC status code in the response
        /// header
        ///    is set to cancelled
        /// - deadline-exceeded: Router will retry if the gRPC status code in the
        /// response
        ///    header is set to deadline-exceeded
        /// - resource-exhausted: Router will retry if the gRPC status code in the
        ///    response header is set to resource-exhausted
        /// - unavailable: Router will retry if the gRPC status code in the response
        ///    header is set to unavailable
        #[prost(string, repeated, tag="1")]
        pub retry_conditions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Specifies the allowed number of retries. This number must be > 0. If not
        /// specified, default to 1.
        #[prost(uint32, tag="2")]
        pub num_retries: u32,
    }
    /// Specifies how to route matched traffic.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RouteAction {
        /// Optional. The destination services to which traffic should be forwarded. If
        /// multiple destinations are specified, traffic will be split between
        /// Backend Service(s) according to the weight field of these destinations.
        #[prost(message, repeated, tag="1")]
        pub destinations: ::prost::alloc::vec::Vec<Destination>,
        /// Optional. The specification for fault injection introduced into traffic to test the
        /// resiliency of clients to destination service failure. As part of fault
        /// injection, when clients send requests to a destination, delays can be
        /// introduced on a percentage of requests before sending those requests to
        /// the destination service. Similarly requests from clients can be aborted
        /// by for a percentage of requests.
        ///
        /// timeout and retry_policy will be ignored by clients that are configured
        /// with a fault_injection_policy
        #[prost(message, optional, tag="3")]
        pub fault_injection_policy: ::core::option::Option<FaultInjectionPolicy>,
        /// Optional. Specifies the timeout for selected route. Timeout is computed from the
        /// time the request has been fully processed (i.e. end of stream) up until
        /// the response has been completely processed. Timeout includes all retries.
        #[prost(message, optional, tag="7")]
        pub timeout: ::core::option::Option<::prost_types::Duration>,
        /// Optional. Specifies the retry policy associated with this route.
        #[prost(message, optional, tag="8")]
        pub retry_policy: ::core::option::Option<RetryPolicy>,
    }
    /// Describes how to route traffic.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RouteRule {
        /// Optional. Matches define conditions used for matching the rule against incoming
        /// gRPC requests. Each match is independent, i.e. this rule will be matched
        /// if ANY one of the matches is satisfied.  If no matches field is
        /// specified, this rule will unconditionally match traffic.
        #[prost(message, repeated, tag="1")]
        pub matches: ::prost::alloc::vec::Vec<RouteMatch>,
        /// Required. A detailed rule defining how to route traffic. This field is required.
        #[prost(message, optional, tag="2")]
        pub action: ::core::option::Option<RouteAction>,
    }
}
/// Request used with the ListGrpcRoutes method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGrpcRoutesRequest {
    /// Required. The project and location from which the GrpcRoutes should be
    /// listed, specified in the format `projects/*/locations/global`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of GrpcRoutes to return per call.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// The value returned by the last `ListGrpcRoutesResponse`
    /// Indicates that this is a continuation of a prior `ListGrpcRoutes` call,
    /// and that the system should return the next page of data.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response returned by the ListGrpcRoutes method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGrpcRoutesResponse {
    /// List of GrpcRoute resources.
    #[prost(message, repeated, tag="1")]
    pub grpc_routes: ::prost::alloc::vec::Vec<GrpcRoute>,
    /// If there might be more results than those appearing in this response, then
    /// `next_page_token` is included. To get the next set of results, call this
    /// method again using the value of `next_page_token` as `page_token`.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request used by the GetGrpcRoute method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGrpcRouteRequest {
    /// Required. A name of the GrpcRoute to get. Must be in the format
    /// `projects/*/locations/global/grpcRoutes/*`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request used by the CreateGrpcRoute method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGrpcRouteRequest {
    /// Required. The parent resource of the GrpcRoute. Must be in the
    /// format `projects/*/locations/global`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Short name of the GrpcRoute resource to be created.
    #[prost(string, tag="2")]
    pub grpc_route_id: ::prost::alloc::string::String,
    /// Required. GrpcRoute resource to be created.
    #[prost(message, optional, tag="3")]
    pub grpc_route: ::core::option::Option<GrpcRoute>,
}
/// Request used by the UpdateGrpcRoute method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGrpcRouteRequest {
    /// Optional. Field mask is used to specify the fields to be overwritten in the
    /// GrpcRoute resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag="1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Updated GrpcRoute resource.
    #[prost(message, optional, tag="2")]
    pub grpc_route: ::core::option::Option<GrpcRoute>,
}
/// Request used by the DeleteGrpcRoute method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteGrpcRouteRequest {
    /// Required. A name of the GrpcRoute to delete. Must be in the format
    /// `projects/*/locations/global/grpcRoutes/*`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// HttpRoute is the resource defining how HTTP traffic should be routed by a
/// Mesh or Gateway resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpRoute {
    /// Required. Name of the HttpRoute resource. It matches pattern
    /// `projects/*/locations/global/httpRoutes/http_route_name>`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Server-defined URL of this resource
    #[prost(string, tag="11")]
    pub self_link: ::prost::alloc::string::String,
    /// Optional. A free-text description of the resource. Max length 1024 characters.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The timestamp when the resource was created.
    #[prost(message, optional, tag="3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the resource was updated.
    #[prost(message, optional, tag="4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. Hostnames define a set of hosts that should match against the HTTP host
    /// header to select a HttpRoute to process the request. Hostname is the fully
    /// qualified domain name of a network host, as defined by RFC 1123 with the
    /// exception that:
    ///  - IPs are not allowed.
    ///  - A hostname may be prefixed with a wildcard label (*.). The wildcard
    ///    label must appear by itself as the first label.
    ///
    /// Hostname can be "precise" which is a domain name without the terminating
    /// dot of a network host (e.g. "foo.example.com") or "wildcard", which is a
    /// domain name prefixed with a single wildcard label (e.g. *.example.com).
    ///
    /// Note that as per RFC1035 and RFC1123, a label must consist of lower case
    /// alphanumeric characters or '-', and must start and end with an alphanumeric
    /// character. No other punctuation is allowed.
    ///
    /// The routes associated with a Mesh or Gateways  must have unique hostnames.
    /// If you attempt to attach multiple routes with conflicting hostnames,
    /// the configuration will be rejected.
    ///
    /// For example, while it is acceptable for routes for the hostnames
    /// "*.foo.bar.com" and "*.bar.com" to be associated with the same Mesh (or
    /// Gateways under the same scope), it is not possible to associate two routes
    /// both with "*.bar.com" or both with "bar.com".
    #[prost(string, repeated, tag="5")]
    pub hostnames: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Meshes defines a list of meshes this HttpRoute is attached to, as one of
    /// the routing rules to route the requests served by the mesh.
    ///
    /// Each mesh reference should match the pattern:
    /// `projects/*/locations/global/meshes/<mesh_name>`
    ///
    /// The attached Mesh should be of a type SIDECAR
    #[prost(string, repeated, tag="8")]
    pub meshes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Gateways defines a list of gateways this HttpRoute is attached to, as one
    /// of the routing rules to route the requests served by the gateway.
    ///
    /// Each gateway reference should match the pattern:
    /// `projects/*/locations/global/gateways/<gateway_name>`
    #[prost(string, repeated, tag="9")]
    pub gateways: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Set of label tags associated with the HttpRoute resource.
    #[prost(btree_map="string, string", tag="10")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Required. Rules that define how traffic is routed and handled.
    /// Rules will be matched sequentially based on the RouteMatch specified for
    /// the rule.
    #[prost(message, repeated, tag="6")]
    pub rules: ::prost::alloc::vec::Vec<http_route::RouteRule>,
}
/// Nested message and enum types in `HttpRoute`.
pub mod http_route {
    /// Specifies how to select a route rule based on HTTP request headers.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HeaderMatch {
        /// The name of the HTTP header to match against.
        #[prost(string, tag="1")]
        pub header: ::prost::alloc::string::String,
        /// If specified, the match result will be inverted before checking. Default
        /// value is set to false.
        #[prost(bool, tag="8")]
        pub invert_match: bool,
        #[prost(oneof="header_match::MatchType", tags="2, 3, 4, 5, 6, 7")]
        pub match_type: ::core::option::Option<header_match::MatchType>,
    }
    /// Nested message and enum types in `HeaderMatch`.
    pub mod header_match {
        /// Represents an integer value range.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct IntegerRange {
            /// Start of the range (inclusive)
            #[prost(int32, tag="1")]
            pub start: i32,
            /// End of the range (exclusive)
            #[prost(int32, tag="2")]
            pub end: i32,
        }
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum MatchType {
            /// The value of the header should match exactly the content of
            /// exact_match.
            #[prost(string, tag="2")]
            ExactMatch(::prost::alloc::string::String),
            /// The value of the header must match the regular expression specified in
            /// regex_match. For regular expression grammar, please see:
            /// <https://github.com/google/re2/wiki/Syntax>
            #[prost(string, tag="3")]
            RegexMatch(::prost::alloc::string::String),
            /// The value of the header must start with the contents of prefix_match.
            #[prost(string, tag="4")]
            PrefixMatch(::prost::alloc::string::String),
            /// A header with header_name must exist. The match takes place whether or
            /// not the header has a value.
            #[prost(bool, tag="5")]
            PresentMatch(bool),
            /// The value of the header must end with the contents of suffix_match.
            #[prost(string, tag="6")]
            SuffixMatch(::prost::alloc::string::String),
            /// If specified, the rule will match if the request header value is within
            /// the range.
            #[prost(message, tag="7")]
            RangeMatch(IntegerRange),
        }
    }
    /// Specifications to match a query parameter in the request.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QueryParameterMatch {
        /// The name of the query parameter to match.
        #[prost(string, tag="1")]
        pub query_parameter: ::prost::alloc::string::String,
        #[prost(oneof="query_parameter_match::MatchType", tags="2, 3, 4")]
        pub match_type: ::core::option::Option<query_parameter_match::MatchType>,
    }
    /// Nested message and enum types in `QueryParameterMatch`.
    pub mod query_parameter_match {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum MatchType {
            /// The value of the query parameter must exactly match the contents of
            /// exact_match.
            ///
            /// Only one of exact_match, regex_match, or present_match must be set.
            #[prost(string, tag="2")]
            ExactMatch(::prost::alloc::string::String),
            /// The value of the query parameter must match the regular expression
            /// specified by regex_match. For regular expression grammar, please see
            /// <https://github.com/google/re2/wiki/Syntax>
            ///
            /// Only one of exact_match, regex_match, or present_match must be set.
            #[prost(string, tag="3")]
            RegexMatch(::prost::alloc::string::String),
            /// Specifies that the QueryParameterMatcher matches if request contains
            /// query parameter, irrespective of whether the parameter has a value or
            /// not.
            ///
            /// Only one of exact_match, regex_match, or present_match must be set.
            #[prost(bool, tag="4")]
            PresentMatch(bool),
        }
    }
    /// RouteMatch defines specifications used to match requests. If multiple match
    /// types are set, this RouteMatch will match if ALL type of matches are
    /// matched.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RouteMatch {
        /// Specifies if prefix_match and full_path_match matches are case sensitive.
        /// The default value is false.
        #[prost(bool, tag="4")]
        pub ignore_case: bool,
        /// Specifies a list of HTTP request headers to match against. ALL of the
        /// supplied headers must be matched.
        #[prost(message, repeated, tag="5")]
        pub headers: ::prost::alloc::vec::Vec<HeaderMatch>,
        /// Specifies a list of query parameters to match against. ALL of the query
        /// parameters must be matched.
        #[prost(message, repeated, tag="6")]
        pub query_parameters: ::prost::alloc::vec::Vec<QueryParameterMatch>,
        #[prost(oneof="route_match::PathMatch", tags="1, 2, 3")]
        pub path_match: ::core::option::Option<route_match::PathMatch>,
    }
    /// Nested message and enum types in `RouteMatch`.
    pub mod route_match {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum PathMatch {
            /// The HTTP request path value should exactly match this value.
            ///
            /// Only one of full_path_match, prefix_match, or regex_match should be
            /// used.
            #[prost(string, tag="1")]
            FullPathMatch(::prost::alloc::string::String),
            /// The HTTP request path value must begin with specified prefix_match.
            /// prefix_match must begin with a /.
            ///
            /// Only one of full_path_match, prefix_match, or regex_match should be
            /// used.
            #[prost(string, tag="2")]
            PrefixMatch(::prost::alloc::string::String),
            /// The HTTP request path value must satisfy the regular expression
            /// specified by regex_match after removing any query parameters and anchor
            /// supplied with the original URL. For regular expression grammar, please
            /// see <https://github.com/google/re2/wiki/Syntax>
            ///
            /// Only one of full_path_match, prefix_match, or regex_match should be
            /// used.
            #[prost(string, tag="3")]
            RegexMatch(::prost::alloc::string::String),
        }
    }
    /// Specifications of a destination to which the request should be routed to.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Destination {
        /// The URL of a BackendService to route traffic to.
        #[prost(string, tag="1")]
        pub service_name: ::prost::alloc::string::String,
        /// Specifies the proportion of requests forwarded to the backend referenced
        /// by the serviceName field. This is computed as:
        ///         weight/Sum(weights in this destination list).
        /// For non-zero values, there may be some epsilon from the exact proportion
        /// defined here depending on the precision an implementation supports.
        ///
        /// If only one serviceName is specified and it has a weight greater than 0,
        /// 100% of the traffic is forwarded to that backend.
        ///
        /// If weights are specified for any one service name, they need to be
        /// specified for all of them.
        ///
        /// If weights are unspecified for all services, then, traffic is distributed
        /// in equal proportions to all of them.
        #[prost(int32, tag="2")]
        pub weight: i32,
    }
    /// The specification for redirecting traffic.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Redirect {
        /// The host that will be used in the redirect response instead of the one
        /// that was supplied in the request.
        #[prost(string, tag="1")]
        pub host_redirect: ::prost::alloc::string::String,
        /// The path that will be used in the redirect response instead of the one
        /// that was supplied in the request.
        /// path_redirect can not be supplied together with prefix_redirect. Supply
        /// one alone or neither. If neither is supplied, the path of the original
        /// request will be used for the redirect.
        #[prost(string, tag="2")]
        pub path_redirect: ::prost::alloc::string::String,
        /// Indicates that during redirection, the matched prefix (or path) should be
        /// swapped with this value. This option allows URLs be dynamically created
        /// based on the request.
        #[prost(string, tag="3")]
        pub prefix_rewrite: ::prost::alloc::string::String,
        /// The HTTP Status code to use for the redirect.
        #[prost(enumeration="redirect::ResponseCode", tag="4")]
        pub response_code: i32,
        /// If set to true, the URL scheme in the redirected request is set to https.
        /// If set to false, the URL scheme of the redirected request will remain the
        /// same as that of the request.
        ///
        /// The default is set to false.
        #[prost(bool, tag="5")]
        pub https_redirect: bool,
        /// if set to true, any accompanying query portion of the original URL is
        /// removed prior to redirecting the request. If set to false, the query
        /// portion of the original URL is retained.
        ///
        /// The default is set to false.
        #[prost(bool, tag="6")]
        pub strip_query: bool,
        /// The port that will be used in the redirected request instead of the one
        /// that was supplied in the request.
        #[prost(int32, tag="7")]
        pub port_redirect: i32,
    }
    /// Nested message and enum types in `Redirect`.
    pub mod redirect {
        /// Supported HTTP response code.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum ResponseCode {
            /// Default value
            Unspecified = 0,
            /// Corresponds to 301.
            MovedPermanentlyDefault = 1,
            /// Corresponds to 302.
            Found = 2,
            /// Corresponds to 303.
            SeeOther = 3,
            /// Corresponds to 307. In this case, the request method will be retained.
            TemporaryRedirect = 4,
            /// Corresponds to 308. In this case, the request method will be retained.
            PermanentRedirect = 5,
        }
    }
    /// The specification for fault injection introduced into traffic to test the
    /// resiliency of clients to destination service failure. As part of fault
    /// injection, when clients send requests to a destination, delays can be
    /// introduced by client proxy on a percentage of requests before sending those
    /// requests to the destination service. Similarly requests can be aborted by
    /// client proxy for a percentage of requests.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FaultInjectionPolicy {
        /// The specification for injecting delay to client requests.
        #[prost(message, optional, tag="1")]
        pub delay: ::core::option::Option<fault_injection_policy::Delay>,
        /// The specification for aborting to client requests.
        #[prost(message, optional, tag="2")]
        pub abort: ::core::option::Option<fault_injection_policy::Abort>,
    }
    /// Nested message and enum types in `FaultInjectionPolicy`.
    pub mod fault_injection_policy {
        /// Specification of how client requests are delayed as part of fault
        /// injection before being sent to a destination.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Delay {
            /// Specify a fixed delay before forwarding the request.
            #[prost(message, optional, tag="1")]
            pub fixed_delay: ::core::option::Option<::prost_types::Duration>,
            /// The percentage of traffic on which delay will be injected.
            ///
            /// The value must be between [0, 100]
            #[prost(int32, tag="2")]
            pub percentage: i32,
        }
        /// Specification of how client requests are aborted as part of fault
        /// injection before being sent to a destination.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Abort {
            /// The HTTP status code used to abort the request.
            ///
            /// The value must be between 200 and 599 inclusive.
            #[prost(int32, tag="1")]
            pub http_status: i32,
            /// The percentage of traffic which will be aborted.
            ///
            /// The value must be between [0, 100]
            #[prost(int32, tag="2")]
            pub percentage: i32,
        }
    }
    /// The specification for modifying HTTP header in HTTP request and HTTP
    /// response.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HeaderModifier {
        /// Completely overwrite/replace the headers with given map where key is the
        /// name of the header, value is the value of the header.
        #[prost(btree_map="string, string", tag="1")]
        pub set: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
        /// Add the headers with given map where key is the name of the header, value
        /// is the value of the header.
        #[prost(btree_map="string, string", tag="2")]
        pub add: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
        /// Remove headers (matching by header names) specified in the list.
        #[prost(string, repeated, tag="3")]
        pub remove: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// The specification for modifying the URL of the request, prior to forwarding
    /// the request to the destination.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UrlRewrite {
        /// Prior to forwarding the request to the selected destination, the matching
        /// portion of the requests path is replaced by this value.
        #[prost(string, tag="1")]
        pub path_prefix_rewrite: ::prost::alloc::string::String,
        /// Prior to forwarding the request to the selected destination, the requests
        /// host header is replaced by this value.
        #[prost(string, tag="2")]
        pub host_rewrite: ::prost::alloc::string::String,
    }
    /// The specifications for retries.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RetryPolicy {
        /// Specifies one or more conditions when this retry policy applies. Valid
        /// values are:
        ///   5xx: Proxy will attempt a retry if the destination service responds
        ///     with any 5xx response code, of if the destination service does not
        ///     respond at all, example: disconnect, reset, read timeout, connection
        ///     failure and refused streams.
        ///
        ///   gateway-error: Similar to 5xx, but only applies to response codes 502,
        ///     503, 504.
        ///
        ///   reset: Proxy will attempt a retry if the destination service does not
        ///     respond at all (disconnect/reset/read timeout)
        ///
        ///   connect-failure: Proxy will retry on failures connecting to destination
        ///     for example due to connection timeouts.
        ///
        ///   retriable-4xx: Proxy will retry fro retriable 4xx response codes.
        ///     Currently the only retriable error supported is 409.
        ///
        ///   refused-stream: Proxy will retry if the destination resets the stream
        ///     with a REFUSED_STREAM error code. This reset type indicates that it
        ///     is safe to retry.
        #[prost(string, repeated, tag="1")]
        pub retry_conditions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Specifies the allowed number of retries. This number must be > 0. If not
        /// specified, default to 1.
        #[prost(int32, tag="2")]
        pub num_retries: i32,
        /// Specifies a non-zero timeout per retry attempt.
        #[prost(message, optional, tag="3")]
        pub per_try_timeout: ::core::option::Option<::prost_types::Duration>,
    }
    /// Specifies the policy on how requests are shadowed to a separate mirrored
    /// destination service. The proxy does not wait for responses from the
    /// shadow service. Prior to sending traffic to the shadow service, the
    /// host/authority header is suffixed with -shadow.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RequestMirrorPolicy {
        /// The destination the requests will be mirrored to. The weight of the
        /// destination will be ignored.
        #[prost(message, optional, tag="1")]
        pub destination: ::core::option::Option<Destination>,
    }
    /// The Specification for allowing client side cross-origin requests.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CorsPolicy {
        /// Specifies the list of origins that will be allowed to do CORS requests.
        /// An origin is allowed if it matches either an item in allow_origins or
        /// an item in allow_origin_regexes.
        #[prost(string, repeated, tag="1")]
        pub allow_origins: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Specifies the regular expression patterns that match allowed origins. For
        /// regular expression grammar, please see
        /// <https://github.com/google/re2/wiki/Syntax.>
        #[prost(string, repeated, tag="2")]
        pub allow_origin_regexes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Specifies the content for Access-Control-Allow-Methods header.
        #[prost(string, repeated, tag="3")]
        pub allow_methods: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Specifies the content for Access-Control-Allow-Headers header.
        #[prost(string, repeated, tag="4")]
        pub allow_headers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Specifies the content for Access-Control-Expose-Headers header.
        #[prost(string, repeated, tag="5")]
        pub expose_headers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Specifies how long result of a preflight request can be cached in
        /// seconds. This translates to the Access-Control-Max-Age header.
        #[prost(string, tag="6")]
        pub max_age: ::prost::alloc::string::String,
        /// In response to a preflight request, setting this to true indicates that
        /// the actual request can include user credentials. This translates to the
        /// Access-Control-Allow-Credentials header.
        ///
        /// Default value is false.
        #[prost(bool, tag="7")]
        pub allow_credentials: bool,
        /// If true, the CORS policy is disabled. The default value is false, which
        /// indicates that the CORS policy is in effect.
        #[prost(bool, tag="8")]
        pub disabled: bool,
    }
    /// The specifications for routing traffic and applying associated policies.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RouteAction {
        /// The destination to which traffic should be forwarded.
        #[prost(message, repeated, tag="1")]
        pub destinations: ::prost::alloc::vec::Vec<Destination>,
        /// If set, the request is directed as configured by this field.
        #[prost(message, optional, tag="2")]
        pub redirect: ::core::option::Option<Redirect>,
        /// The specification for fault injection introduced into traffic to test the
        /// resiliency of clients to backend service failure. As part of fault
        /// injection, when clients send requests to a backend service, delays can be
        /// introduced  on a percentage of requests before sending those requests to
        /// the backend service. Similarly requests from clients can be aborted for a
        /// percentage of requests.
        ///
        /// timeout and retry_policy will be ignored by clients that are configured
        /// with a fault_injection_policy
        #[prost(message, optional, tag="4")]
        pub fault_injection_policy: ::core::option::Option<FaultInjectionPolicy>,
        /// The specification for modifying the headers of a matching request prior
        /// to delivery of the request to the destination.
        #[prost(message, optional, tag="5")]
        pub request_header_modifier: ::core::option::Option<HeaderModifier>,
        /// The specification for modifying the headers of a response prior to
        /// sending the response back to the client.
        #[prost(message, optional, tag="6")]
        pub response_header_modifier: ::core::option::Option<HeaderModifier>,
        /// The specification for rewrite URL before forwarding requests to the
        /// destination.
        #[prost(message, optional, tag="7")]
        pub url_rewrite: ::core::option::Option<UrlRewrite>,
        /// Specifies the timeout for selected route. Timeout is computed from the
        /// time the request has been fully processed (i.e. end of stream) up until
        /// the response has been completely processed. Timeout includes all retries.
        #[prost(message, optional, tag="8")]
        pub timeout: ::core::option::Option<::prost_types::Duration>,
        /// Specifies the retry policy associated with this route.
        #[prost(message, optional, tag="9")]
        pub retry_policy: ::core::option::Option<RetryPolicy>,
        /// Specifies the policy on how requests intended for the routes destination
        /// are shadowed to a separate mirrored destination. Proxy will not wait for
        /// the shadow destination to respond before returning the response. Prior to
        /// sending traffic to the shadow service, the host/authority header is
        /// suffixed with -shadow.
        #[prost(message, optional, tag="10")]
        pub request_mirror_policy: ::core::option::Option<RequestMirrorPolicy>,
        /// The specification for allowing client side cross-origin requests.
        #[prost(message, optional, tag="11")]
        pub cors_policy: ::core::option::Option<CorsPolicy>,
    }
    /// Specifies how to match traffic and how to route traffic when traffic is
    /// matched.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RouteRule {
        /// A list of matches define conditions used for matching the rule against
        /// incoming HTTP requests. Each match is independent, i.e. this rule will be
        /// matched if ANY one of the matches is satisfied.
        ///
        /// If no matches field is specified, this rule will unconditionally match
        /// traffic.
        ///
        /// If a default rule is desired to be configured, add a rule with no matches
        /// specified to the end of the rules list.
        #[prost(message, repeated, tag="1")]
        pub matches: ::prost::alloc::vec::Vec<RouteMatch>,
        /// The detailed rule defining how to route matched traffic.
        #[prost(message, optional, tag="2")]
        pub action: ::core::option::Option<RouteAction>,
    }
}
/// Request used with the ListHttpRoutes method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHttpRoutesRequest {
    /// Required. The project and location from which the HttpRoutes should be
    /// listed, specified in the format `projects/*/locations/global`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of HttpRoutes to return per call.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// The value returned by the last `ListHttpRoutesResponse`
    /// Indicates that this is a continuation of a prior `ListHttpRoutes` call,
    /// and that the system should return the next page of data.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response returned by the ListHttpRoutes method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHttpRoutesResponse {
    /// List of HttpRoute resources.
    #[prost(message, repeated, tag="1")]
    pub http_routes: ::prost::alloc::vec::Vec<HttpRoute>,
    /// If there might be more results than those appearing in this response, then
    /// `next_page_token` is included. To get the next set of results, call this
    /// method again using the value of `next_page_token` as `page_token`.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request used by the GetHttpRoute method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHttpRouteRequest {
    /// Required. A name of the HttpRoute to get. Must be in the format
    /// `projects/*/locations/global/httpRoutes/*`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request used by the HttpRoute method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateHttpRouteRequest {
    /// Required. The parent resource of the HttpRoute. Must be in the
    /// format `projects/*/locations/global`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Short name of the HttpRoute resource to be created.
    #[prost(string, tag="2")]
    pub http_route_id: ::prost::alloc::string::String,
    /// Required. HttpRoute resource to be created.
    #[prost(message, optional, tag="3")]
    pub http_route: ::core::option::Option<HttpRoute>,
}
/// Request used by the UpdateHttpRoute method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateHttpRouteRequest {
    /// Optional. Field mask is used to specify the fields to be overwritten in the
    /// HttpRoute resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag="1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Updated HttpRoute resource.
    #[prost(message, optional, tag="2")]
    pub http_route: ::core::option::Option<HttpRoute>,
}
/// Request used by the DeleteHttpRoute method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteHttpRouteRequest {
    /// Required. A name of the HttpRoute to delete. Must be in the format
    /// `projects/*/locations/global/httpRoutes/*`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// TcpRoute is the resource defining how TCP traffic should be routed by a
/// Mesh/Gateway resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TcpRoute {
    /// Required. Name of the TcpRoute resource. It matches pattern
    /// `projects/*/locations/global/tcpRoutes/tcp_route_name>`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Server-defined URL of this resource
    #[prost(string, tag="11")]
    pub self_link: ::prost::alloc::string::String,
    /// Output only. The timestamp when the resource was created.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the resource was updated.
    #[prost(message, optional, tag="3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. A free-text description of the resource. Max length 1024 characters.
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
    /// Required. Rules that define how traffic is routed and handled. At least one RouteRule
    /// must be supplied. If there are multiple rules then the action taken will be
    /// the first rule to match.
    #[prost(message, repeated, tag="5")]
    pub rules: ::prost::alloc::vec::Vec<tcp_route::RouteRule>,
    /// Optional. Meshes defines a list of meshes this TcpRoute is attached to, as one of the
    /// routing rules to route the requests served by the mesh.
    ///
    /// Each mesh reference should match the pattern:
    /// `projects/*/locations/global/meshes/<mesh_name>`
    ///
    /// The attached Mesh should be of a type SIDECAR
    #[prost(string, repeated, tag="8")]
    pub meshes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Gateways defines a list of gateways this TcpRoute is attached to, as one of
    /// the routing rules to route the requests served by the gateway.
    ///
    /// Each gateway reference should match the pattern:
    /// `projects/*/locations/global/gateways/<gateway_name>`
    #[prost(string, repeated, tag="9")]
    pub gateways: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Set of label tags associated with the TcpRoute resource.
    #[prost(btree_map="string, string", tag="10")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Nested message and enum types in `TcpRoute`.
pub mod tcp_route {
    /// Specifies how to match traffic and how to route traffic when traffic is
    /// matched.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RouteRule {
        /// Optional. RouteMatch defines the predicate used to match requests to a given
        /// action. Multiple match types are "OR"ed for evaluation.
        /// If no routeMatch field is specified, this rule will unconditionally match
        /// traffic.
        #[prost(message, repeated, tag="1")]
        pub matches: ::prost::alloc::vec::Vec<RouteMatch>,
        /// Required. The detailed rule defining how to route matched traffic.
        #[prost(message, optional, tag="2")]
        pub action: ::core::option::Option<RouteAction>,
    }
    /// RouteMatch defines the predicate used to match requests to a given action.
    /// Multiple match types are "OR"ed for evaluation.
    /// If no routeMatch field is specified, this rule will unconditionally match
    /// traffic.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RouteMatch {
        /// Required. Must be specified in the CIDR range format. A CIDR range consists of an
        /// IP Address and a prefix length to construct the subnet mask. By default,
        /// the prefix length is 32 (i.e. matches a single IP address). Only IPV4
        /// addresses are supported.
        /// Examples:
        /// "10.0.0.1" - matches against this exact IP address.
        /// "10.0.0.0/8" - matches against any IP address within the 10.0.0.0 subnet
        /// and 255.255.255.0 mask.
        /// "0.0.0.0/0" - matches against any IP address'.
        #[prost(string, tag="1")]
        pub address: ::prost::alloc::string::String,
        /// Required. Specifies the destination port to match against.
        #[prost(string, tag="2")]
        pub port: ::prost::alloc::string::String,
    }
    /// The specifications for routing traffic and applying associated policies.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RouteAction {
        /// Optional. The destination services to which traffic should be forwarded. At least
        /// one destination service is required.
        #[prost(message, repeated, tag="1")]
        pub destinations: ::prost::alloc::vec::Vec<RouteDestination>,
        /// Optional. If true, Router will use the destination IP and port of the original
        /// connection as the destination of the request. Default is false.
        #[prost(bool, tag="3")]
        pub original_destination: bool,
    }
    /// Describe the destination for traffic to be routed to.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RouteDestination {
        /// Required. The URL of a BackendService to route traffic to.
        #[prost(string, tag="1")]
        pub service_name: ::prost::alloc::string::String,
        /// Optional. Specifies the proportion of requests forwarded to the backend referenced
        /// by the serviceName field. This is computed as:
        ///         weight/Sum(weights in this destination list).
        /// For non-zero values, there may be some epsilon from the exact proportion
        /// defined here depending on the precision an implementation supports.
        ///
        /// If only one serviceName is specified and it has a weight greater than 0,
        /// 100% of the traffic is forwarded to that backend.
        ///
        /// If weights are specified for any one service name, they need to be
        /// specified for all of them.
        ///
        /// If weights are unspecified for all services, then, traffic is distributed
        /// in equal proportions to all of them.
        #[prost(int32, tag="2")]
        pub weight: i32,
    }
}
/// Request used with the ListTcpRoutes method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTcpRoutesRequest {
    /// Required. The project and location from which the TcpRoutes should be
    /// listed, specified in the format `projects/*/locations/global`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of TcpRoutes to return per call.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// The value returned by the last `ListTcpRoutesResponse`
    /// Indicates that this is a continuation of a prior `ListTcpRoutes` call,
    /// and that the system should return the next page of data.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response returned by the ListTcpRoutes method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTcpRoutesResponse {
    /// List of TcpRoute resources.
    #[prost(message, repeated, tag="1")]
    pub tcp_routes: ::prost::alloc::vec::Vec<TcpRoute>,
    /// If there might be more results than those appearing in this response, then
    /// `next_page_token` is included. To get the next set of results, call this
    /// method again using the value of `next_page_token` as `page_token`.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request used by the GetTcpRoute method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTcpRouteRequest {
    /// Required. A name of the TcpRoute to get. Must be in the format
    /// `projects/*/locations/global/tcpRoutes/*`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request used by the TcpRoute method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTcpRouteRequest {
    /// Required. The parent resource of the TcpRoute. Must be in the
    /// format `projects/*/locations/global`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Short name of the TcpRoute resource to be created. E.g. TODO(Add an
    /// example).
    #[prost(string, tag="2")]
    pub tcp_route_id: ::prost::alloc::string::String,
    /// Required. TcpRoute resource to be created.
    #[prost(message, optional, tag="3")]
    pub tcp_route: ::core::option::Option<TcpRoute>,
}
/// Request used by the UpdateTcpRoute method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTcpRouteRequest {
    /// Optional. Field mask is used to specify the fields to be overwritten in the
    /// TcpRoute resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag="1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Updated TcpRoute resource.
    #[prost(message, optional, tag="2")]
    pub tcp_route: ::core::option::Option<TcpRoute>,
}
/// Request used by the DeleteTcpRoute method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTcpRouteRequest {
    /// Required. A name of the TcpRoute to delete. Must be in the format
    /// `projects/*/locations/global/tcpRoutes/*`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod network_services_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct NetworkServicesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> NetworkServicesClient<T>
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
        ) -> NetworkServicesClient<InterceptedService<T, F>>
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
            NetworkServicesClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists EndpointPolicies in a given project and location.
        pub async fn list_endpoint_policies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEndpointPoliciesRequest>,
        ) -> Result<
            tonic::Response<super::ListEndpointPoliciesResponse>,
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
                "/google.cloud.networkservices.v1.NetworkServices/ListEndpointPolicies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single EndpointPolicy.
        pub async fn get_endpoint_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEndpointPolicyRequest>,
        ) -> Result<tonic::Response<super::EndpointPolicy>, tonic::Status> {
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
                "/google.cloud.networkservices.v1.NetworkServices/GetEndpointPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new EndpointPolicy in a given project and location.
        pub async fn create_endpoint_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEndpointPolicyRequest>,
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
                "/google.cloud.networkservices.v1.NetworkServices/CreateEndpointPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of a single EndpointPolicy.
        pub async fn update_endpoint_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEndpointPolicyRequest>,
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
                "/google.cloud.networkservices.v1.NetworkServices/UpdateEndpointPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single EndpointPolicy.
        pub async fn delete_endpoint_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEndpointPolicyRequest>,
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
                "/google.cloud.networkservices.v1.NetworkServices/DeleteEndpointPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists Gateways in a given project and location.
        pub async fn list_gateways(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGatewaysRequest>,
        ) -> Result<tonic::Response<super::ListGatewaysResponse>, tonic::Status> {
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
                "/google.cloud.networkservices.v1.NetworkServices/ListGateways",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single Gateway.
        pub async fn get_gateway(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGatewayRequest>,
        ) -> Result<tonic::Response<super::Gateway>, tonic::Status> {
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
                "/google.cloud.networkservices.v1.NetworkServices/GetGateway",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new Gateway in a given project and location.
        pub async fn create_gateway(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateGatewayRequest>,
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
                "/google.cloud.networkservices.v1.NetworkServices/CreateGateway",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of a single Gateway.
        pub async fn update_gateway(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGatewayRequest>,
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
                "/google.cloud.networkservices.v1.NetworkServices/UpdateGateway",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single Gateway.
        pub async fn delete_gateway(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteGatewayRequest>,
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
                "/google.cloud.networkservices.v1.NetworkServices/DeleteGateway",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists GrpcRoutes in a given project and location.
        pub async fn list_grpc_routes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGrpcRoutesRequest>,
        ) -> Result<tonic::Response<super::ListGrpcRoutesResponse>, tonic::Status> {
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
                "/google.cloud.networkservices.v1.NetworkServices/ListGrpcRoutes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single GrpcRoute.
        pub async fn get_grpc_route(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGrpcRouteRequest>,
        ) -> Result<tonic::Response<super::GrpcRoute>, tonic::Status> {
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
                "/google.cloud.networkservices.v1.NetworkServices/GetGrpcRoute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new GrpcRoute in a given project and location.
        pub async fn create_grpc_route(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateGrpcRouteRequest>,
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
                "/google.cloud.networkservices.v1.NetworkServices/CreateGrpcRoute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of a single GrpcRoute.
        pub async fn update_grpc_route(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGrpcRouteRequest>,
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
                "/google.cloud.networkservices.v1.NetworkServices/UpdateGrpcRoute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single GrpcRoute.
        pub async fn delete_grpc_route(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteGrpcRouteRequest>,
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
                "/google.cloud.networkservices.v1.NetworkServices/DeleteGrpcRoute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists HttpRoute in a given project and location.
        pub async fn list_http_routes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListHttpRoutesRequest>,
        ) -> Result<tonic::Response<super::ListHttpRoutesResponse>, tonic::Status> {
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
                "/google.cloud.networkservices.v1.NetworkServices/ListHttpRoutes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single HttpRoute.
        pub async fn get_http_route(
            &mut self,
            request: impl tonic::IntoRequest<super::GetHttpRouteRequest>,
        ) -> Result<tonic::Response<super::HttpRoute>, tonic::Status> {
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
                "/google.cloud.networkservices.v1.NetworkServices/GetHttpRoute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new HttpRoute in a given project and location.
        pub async fn create_http_route(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateHttpRouteRequest>,
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
                "/google.cloud.networkservices.v1.NetworkServices/CreateHttpRoute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of a single HttpRoute.
        pub async fn update_http_route(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateHttpRouteRequest>,
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
                "/google.cloud.networkservices.v1.NetworkServices/UpdateHttpRoute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single HttpRoute.
        pub async fn delete_http_route(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteHttpRouteRequest>,
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
                "/google.cloud.networkservices.v1.NetworkServices/DeleteHttpRoute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists TcpRoute in a given project and location.
        pub async fn list_tcp_routes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTcpRoutesRequest>,
        ) -> Result<tonic::Response<super::ListTcpRoutesResponse>, tonic::Status> {
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
                "/google.cloud.networkservices.v1.NetworkServices/ListTcpRoutes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single TcpRoute.
        pub async fn get_tcp_route(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTcpRouteRequest>,
        ) -> Result<tonic::Response<super::TcpRoute>, tonic::Status> {
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
                "/google.cloud.networkservices.v1.NetworkServices/GetTcpRoute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new TcpRoute in a given project and location.
        pub async fn create_tcp_route(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTcpRouteRequest>,
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
                "/google.cloud.networkservices.v1.NetworkServices/CreateTcpRoute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of a single TcpRoute.
        pub async fn update_tcp_route(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTcpRouteRequest>,
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
                "/google.cloud.networkservices.v1.NetworkServices/UpdateTcpRoute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single TcpRoute.
        pub async fn delete_tcp_route(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTcpRouteRequest>,
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
                "/google.cloud.networkservices.v1.NetworkServices/DeleteTcpRoute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists TlsRoute in a given project and location.
        pub async fn list_tls_routes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTlsRoutesRequest>,
        ) -> Result<tonic::Response<super::ListTlsRoutesResponse>, tonic::Status> {
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
                "/google.cloud.networkservices.v1.NetworkServices/ListTlsRoutes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single TlsRoute.
        pub async fn get_tls_route(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTlsRouteRequest>,
        ) -> Result<tonic::Response<super::TlsRoute>, tonic::Status> {
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
                "/google.cloud.networkservices.v1.NetworkServices/GetTlsRoute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new TlsRoute in a given project and location.
        pub async fn create_tls_route(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTlsRouteRequest>,
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
                "/google.cloud.networkservices.v1.NetworkServices/CreateTlsRoute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of a single TlsRoute.
        pub async fn update_tls_route(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTlsRouteRequest>,
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
                "/google.cloud.networkservices.v1.NetworkServices/UpdateTlsRoute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single TlsRoute.
        pub async fn delete_tls_route(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTlsRouteRequest>,
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
                "/google.cloud.networkservices.v1.NetworkServices/DeleteTlsRoute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists ServiceBinding in a given project and location.
        pub async fn list_service_bindings(
            &mut self,
            request: impl tonic::IntoRequest<super::ListServiceBindingsRequest>,
        ) -> Result<tonic::Response<super::ListServiceBindingsResponse>, tonic::Status> {
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
                "/google.cloud.networkservices.v1.NetworkServices/ListServiceBindings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single ServiceBinding.
        pub async fn get_service_binding(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServiceBindingRequest>,
        ) -> Result<tonic::Response<super::ServiceBinding>, tonic::Status> {
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
                "/google.cloud.networkservices.v1.NetworkServices/GetServiceBinding",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new ServiceBinding in a given project and location.
        pub async fn create_service_binding(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateServiceBindingRequest>,
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
                "/google.cloud.networkservices.v1.NetworkServices/CreateServiceBinding",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single ServiceBinding.
        pub async fn delete_service_binding(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteServiceBindingRequest>,
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
                "/google.cloud.networkservices.v1.NetworkServices/DeleteServiceBinding",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists Meshes in a given project and location.
        pub async fn list_meshes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMeshesRequest>,
        ) -> Result<tonic::Response<super::ListMeshesResponse>, tonic::Status> {
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
                "/google.cloud.networkservices.v1.NetworkServices/ListMeshes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single Mesh.
        pub async fn get_mesh(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMeshRequest>,
        ) -> Result<tonic::Response<super::Mesh>, tonic::Status> {
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
                "/google.cloud.networkservices.v1.NetworkServices/GetMesh",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new Mesh in a given project and location.
        pub async fn create_mesh(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateMeshRequest>,
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
                "/google.cloud.networkservices.v1.NetworkServices/CreateMesh",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of a single Mesh.
        pub async fn update_mesh(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateMeshRequest>,
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
                "/google.cloud.networkservices.v1.NetworkServices/UpdateMesh",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single Mesh.
        pub async fn delete_mesh(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteMeshRequest>,
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
                "/google.cloud.networkservices.v1.NetworkServices/DeleteMesh",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
