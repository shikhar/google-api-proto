/// An attestation wrapper with a PGP-compatible signature. This message only
/// supports `ATTACHED` signatures, where the payload that is signed is included
/// alongside the signature itself in the same file.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PgpSignedAttestation {
    /// Required. The raw content of the signature, as output by GNU Privacy Guard
    /// (GPG) or equivalent. Since this message only supports attached signatures,
    /// the payload that was signed must be attached. While the signature format
    /// supported is dependent on the verification implementation, currently only
    /// ASCII-armored (`--armor` to gpg), non-clearsigned (`--sign` rather than
    /// `--clearsign` to gpg) are supported. Concretely, `gpg --sign --armor
    /// --output=signature.gpg payload.json` will create the signature content
    /// expected in this field in `signature.gpg` for the `payload.json`
    /// attestation payload.
    #[prost(string, tag="1")]
    pub signature: ::prost::alloc::string::String,
    /// Type (for example schema) of the attestation payload that was signed.
    /// The verifier must ensure that the provided type is one that the verifier
    /// supports, and that the attestation payload is a valid instantiation of that
    /// type (for example by validating a JSON schema).
    #[prost(enumeration="pgp_signed_attestation::ContentType", tag="3")]
    pub content_type: i32,
    /// This field is used by verifiers to select the public key used to validate
    /// the signature. Note that the policy of the verifier ultimately determines
    /// which public keys verify a signature based on the context of the
    /// verification. There is no guarantee validation will succeed if the
    /// verifier has no key matching this ID, even if it has a key under a
    /// different ID that would verify the signature. Note that this ID should also
    /// be present in the signature content above, but that is not expected to be
    /// used by the verifier.
    #[prost(oneof="pgp_signed_attestation::KeyId", tags="2")]
    pub key_id: ::core::option::Option<pgp_signed_attestation::KeyId>,
}
/// Nested message and enum types in `PgpSignedAttestation`.
pub mod pgp_signed_attestation {
    /// Type (for example schema) of the attestation payload that was signed.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ContentType {
        /// `ContentType` is not set.
        Unspecified = 0,
        /// Atomic format attestation signature. See
        /// <https://github.com/containers/image/blob/8a5d2f82a6e3263290c8e0276c3e0f64e77723e7/docs/atomic-signature.md>
        /// The payload extracted from `signature` is a JSON blob conforming to the
        /// linked schema.
        SimpleSigningJson = 1,
    }
    /// This field is used by verifiers to select the public key used to validate
    /// the signature. Note that the policy of the verifier ultimately determines
    /// which public keys verify a signature based on the context of the
    /// verification. There is no guarantee validation will succeed if the
    /// verifier has no key matching this ID, even if it has a key under a
    /// different ID that would verify the signature. Note that this ID should also
    /// be present in the signature content above, but that is not expected to be
    /// used by the verifier.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum KeyId {
        /// The cryptographic fingerprint of the key used to generate the signature,
        /// as output by, e.g. `gpg --list-keys`. This should be the version 4, full
        /// 160-bit fingerprint, expressed as a 40 character hexidecimal string. See
        /// <https://tools.ietf.org/html/rfc4880#section-12.2> for details.
        /// Implementations may choose to acknowledge "LONG", "SHORT", or other
        /// abbreviated key IDs, but only the full fingerprint is guaranteed to work.
        /// In gpg, the full fingerprint can be retrieved from the `fpr` field
        /// returned when calling --list-keys with --with-colons.  For example:
        /// ```
        /// gpg --with-colons --with-fingerprint --force-v4-certs \
        ///     --list-keys attester@example.com
        /// tru::1:1513631572:0:3:1:5
        /// pub:...<SNIP>...
        /// fpr:::::::::24FF6481B76AC91E66A00AC657A93A81EF3AE6FB:
        /// ```
        /// Above, the fingerprint is `24FF6481B76AC91E66A00AC657A93A81EF3AE6FB`.
        #[prost(string, tag="2")]
        PgpKeyId(::prost::alloc::string::String),
    }
}
/// An attestation wrapper that uses the Grafeas `Signature` message.
/// This attestation must define the `serialized_payload` that the `signatures`
/// verify and any metadata necessary to interpret that plaintext.  The
/// signatures should always be over the `serialized_payload` bytestring.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericSignedAttestation {
    /// Type (for example schema) of the attestation payload that was signed.
    /// The verifier must ensure that the provided type is one that the verifier
    /// supports, and that the attestation payload is a valid instantiation of that
    /// type (for example by validating a JSON schema).
    #[prost(enumeration="generic_signed_attestation::ContentType", tag="1")]
    pub content_type: i32,
    /// The serialized payload that is verified by one or more `signatures`.
    /// The encoding and semantic meaning of this payload must match what is set in
    /// `content_type`.
    #[prost(bytes="bytes", tag="2")]
    pub serialized_payload: ::prost::bytes::Bytes,
    /// One or more signatures over `serialized_payload`.  Verifier implementations
    /// should consider this attestation message verified if at least one
    /// `signature` verifies `serialized_payload`.  See `Signature` in common.proto
    /// for more details on signature structure and verification.
    #[prost(message, repeated, tag="3")]
    pub signatures: ::prost::alloc::vec::Vec<super::Signature>,
}
/// Nested message and enum types in `GenericSignedAttestation`.
pub mod generic_signed_attestation {
    /// Type of the attestation plaintext that was signed.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ContentType {
        /// `ContentType` is not set.
        Unspecified = 0,
        /// Atomic format attestation signature. See
        /// <https://github.com/containers/image/blob/8a5d2f82a6e3263290c8e0276c3e0f64e77723e7/docs/atomic-signature.md>
        /// The payload extracted in `plaintext` is a JSON blob conforming to the
        /// linked schema.
        SimpleSigningJson = 1,
    }
}
/// Note kind that represents a logical attestation "role" or "authority". For
/// example, an organization might have one `Authority` for "QA" and one for
/// "build". This note is intended to act strictly as a grouping mechanism for
/// the attached occurrences (Attestations). This grouping mechanism also
/// provides a security boundary, since IAM ACLs gate the ability for a principle
/// to attach an occurrence to a given note. It also provides a single point of
/// lookup to find all attached attestation occurrences, even if they don't all
/// live in the same project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Authority {
    /// Hint hints at the purpose of the attestation authority.
    #[prost(message, optional, tag="1")]
    pub hint: ::core::option::Option<authority::Hint>,
}
/// Nested message and enum types in `Authority`.
pub mod authority {
    /// This submessage provides human-readable hints about the purpose of the
    /// authority. Because the name of a note acts as its resource reference, it is
    /// important to disambiguate the canonical name of the Note (which might be a
    /// UUID for security purposes) from "readable" names more suitable for debug
    /// output. Note that these hints should not be used to look up authorities in
    /// security sensitive contexts, such as when looking up attestations to
    /// verify.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Hint {
        /// Required. The human readable name of this attestation authority, for
        /// example "qa".
        #[prost(string, tag="1")]
        pub human_readable_name: ::prost::alloc::string::String,
    }
}
/// Details of an attestation occurrence.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Details {
    /// Required. Attestation for the resource.
    #[prost(message, optional, tag="1")]
    pub attestation: ::core::option::Option<Attestation>,
}
/// Occurrence that represents a single "attestation". The authenticity of an
/// attestation can be verified using the attached signature. If the verifier
/// trusts the public key of the signer, then verifying the signature is
/// sufficient to establish trust. In this circumstance, the authority to which
/// this attestation is attached is primarily useful for look-up (how to find
/// this attestation if you already know the authority and artifact to be
/// verified) and intent (which authority was this attestation intended to sign
/// for).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attestation {
    /// Required. The signature, generally over the `resource_url`, that verifies
    /// this attestation. The semantics of the signature veracity are ultimately
    /// determined by the verification engine.
    #[prost(oneof="attestation::Signature", tags="1, 2")]
    pub signature: ::core::option::Option<attestation::Signature>,
}
/// Nested message and enum types in `Attestation`.
pub mod attestation {
    /// Required. The signature, generally over the `resource_url`, that verifies
    /// this attestation. The semantics of the signature veracity are ultimately
    /// determined by the verification engine.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Signature {
        /// A PGP signed attestation.
        #[prost(message, tag="1")]
        PgpSignedAttestation(super::PgpSignedAttestation),
        #[prost(message, tag="2")]
        GenericSignedAttestation(super::GenericSignedAttestation),
    }
}
