/// Log message which notifies about expired or close to
/// expiry certificates.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificatesExpiry {
    /// Number of reported certificates.
    #[prost(int64, tag="1")]
    pub count: i64,
    /// Names of reported certificates. If there are too many, the list is sampled.
    #[prost(string, repeated, tag="2")]
    pub certificates: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// State of reported certificates.
    #[prost(enumeration="certificates_expiry::State", tag="3")]
    pub state: i32,
    /// Approximated expire time of reported certificates.
    /// Multiple certificates with close expire time are batched together in a
    /// single log, so the timestamp is not precise.
    #[prost(message, optional, tag="4")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `CertificatesExpiry`.
pub mod certificates_expiry {
    /// Expiration state of the certificate.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified state, should never be reported.
        Unspecified = 0,
        /// Certificate will expire soon.
        CloseToExpiry = 1,
        /// Certificate is expired.
        Expired = 2,
    }
}
