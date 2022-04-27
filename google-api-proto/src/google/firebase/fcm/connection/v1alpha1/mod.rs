/// Request sent to FCM from the connected client.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpstreamRequest {
    /// The type of request the client is making to FCM.
    #[prost(oneof="upstream_request::RequestType", tags="1")]
    pub request_type: ::core::option::Option<upstream_request::RequestType>,
}
/// Nested message and enum types in `UpstreamRequest`.
pub mod upstream_request {
    /// The type of request the client is making to FCM.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequestType {
        /// Message acknowledgement.
        #[prost(message, tag="1")]
        Ack(super::Ack),
    }
}
/// Response sent to the connected client from FCM.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownstreamResponse {
    /// The type of response FCM is sending to the client.
    #[prost(oneof="downstream_response::ResponseType", tags="1")]
    pub response_type: ::core::option::Option<downstream_response::ResponseType>,
}
/// Nested message and enum types in `DownstreamResponse`.
pub mod downstream_response {
    /// The type of response FCM is sending to the client.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ResponseType {
        /// Message sent to FCM via the [Send
        /// API](<https://firebase.google.com/docs/cloud-messaging/send-message>)
        /// targeting this client.
        #[prost(message, tag="1")]
        Message(super::Message),
    }
}
/// Acknowledgement to indicate a client successfully received an FCM message.
///
/// If a message is not acked, FCM will continously resend the message until
/// it expires. Duplicate delivery in this case is working as intended.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ack {
    /// Id of message being acknowledged
    #[prost(string, tag="1")]
    pub message_id: ::prost::alloc::string::String,
}
/// Message created through the [Send
/// API](<https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#resource-message>).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    /// The identifier of the message. Used to ack the message.
    #[prost(string, tag="1")]
    pub message_id: ::prost::alloc::string::String,
    /// Time the message was received in FCM.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Expiry time of the message. Currently it is always 4 weeks.
    #[prost(message, optional, tag="3")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The arbitrary payload set in the [Send
    /// API](<https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#resource-message>).
    #[prost(btree_map="string, string", tag="4")]
    pub data: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Generated client implementations.
pub mod connection_api_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// FCM's service to create client connections to send/receive messages.
    #[derive(Debug, Clone)]
    pub struct ConnectionApiClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ConnectionApiClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Default + Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ConnectionApiClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
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
            ConnectionApiClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a streaming connection with FCM to send messages and their
        /// respective ACKs.
        ///
        /// The client credentials need to be passed in the [gRPC
        /// Metadata](https://grpc.io/docs/guides/concepts.html#metadata). The Format
        /// of the header is:
        ///   Key: "authorization"
        ///   Value: "Checkin [client_id:secret]"
        ///
        ///
        /// The project's API key also needs to be sent to authorize the project.
        /// That can be set in the X-Goog-Api-Key Metadata header.
        pub async fn connect(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::UpstreamRequest>,
        ) -> Result<
                tonic::Response<tonic::codec::Streaming<super::DownstreamResponse>>,
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
                "/google.firebase.fcm.connection.v1alpha1.ConnectionApi/Connect",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
    }
}
