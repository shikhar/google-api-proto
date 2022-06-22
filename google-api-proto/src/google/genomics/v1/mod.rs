/// A single CIGAR operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CigarUnit {
    #[prost(enumeration="cigar_unit::Operation", tag="1")]
    pub operation: i32,
    /// The number of genomic bases that the operation runs for. Required.
    #[prost(int64, tag="2")]
    pub operation_length: i64,
    /// `referenceSequence` is only used at mismatches
    /// (`SEQUENCE_MISMATCH`) and deletions (`DELETE`).
    /// Filling this field replaces SAM's MD tag. If the relevant information is
    /// not available, this field is unset.
    #[prost(string, tag="3")]
    pub reference_sequence: ::prost::alloc::string::String,
}
/// Nested message and enum types in `CigarUnit`.
pub mod cigar_unit {
    /// Describes the different types of CIGAR alignment operations that exist.
    /// Used wherever CIGAR alignments are used.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Operation {
        Unspecified = 0,
        /// An alignment match indicates that a sequence can be aligned to the
        /// reference without evidence of an INDEL. Unlike the
        /// `SEQUENCE_MATCH` and `SEQUENCE_MISMATCH` operators,
        /// the `ALIGNMENT_MATCH` operator does not indicate whether the
        /// reference and read sequences are an exact match. This operator is
        /// equivalent to SAM's `M`.
        AlignmentMatch = 1,
        /// The insert operator indicates that the read contains evidence of bases
        /// being inserted into the reference. This operator is equivalent to SAM's
        /// `I`.
        Insert = 2,
        /// The delete operator indicates that the read contains evidence of bases
        /// being deleted from the reference. This operator is equivalent to SAM's
        /// `D`.
        Delete = 3,
        /// The skip operator indicates that this read skips a long segment of the
        /// reference, but the bases have not been deleted. This operator is commonly
        /// used when working with RNA-seq data, where reads may skip long segments
        /// of the reference between exons. This operator is equivalent to SAM's
        /// `N`.
        Skip = 4,
        /// The soft clip operator indicates that bases at the start/end of a read
        /// have not been considered during alignment. This may occur if the majority
        /// of a read maps, except for low quality bases at the start/end of a read.
        /// This operator is equivalent to SAM's `S`. Bases that are soft
        /// clipped will still be stored in the read.
        ClipSoft = 5,
        /// The hard clip operator indicates that bases at the start/end of a read
        /// have been omitted from this alignment. This may occur if this linear
        /// alignment is part of a chimeric alignment, or if the read has been
        /// trimmed (for example, during error correction or to trim poly-A tails for
        /// RNA-seq). This operator is equivalent to SAM's `H`.
        ClipHard = 6,
        /// The pad operator indicates that there is padding in an alignment. This
        /// operator is equivalent to SAM's `P`.
        Pad = 7,
        /// This operator indicates that this portion of the aligned sequence exactly
        /// matches the reference. This operator is equivalent to SAM's `=`.
        SequenceMatch = 8,
        /// This operator indicates that this portion of the aligned sequence is an
        /// alignment match to the reference, but a sequence mismatch. This can
        /// indicate a SNP or a read error. This operator is equivalent to SAM's
        /// `X`.
        SequenceMismatch = 9,
    }
}
/// Metadata describes a single piece of variant call metadata.
/// These data include a top level key and either a single value string (value)
/// or a list of key-value pairs (info.)
/// Value and info are mutually exclusive.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VariantSetMetadata {
    /// The top-level key.
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    /// The value field for simple metadata
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
    /// User-provided ID field, not enforced by this API.
    /// Two or more pieces of structured metadata with identical
    /// id and key fields are considered equivalent.
    #[prost(string, tag="4")]
    pub id: ::prost::alloc::string::String,
    /// The type of data. Possible types include: Integer, Float,
    /// Flag, Character, and String.
    #[prost(enumeration="variant_set_metadata::Type", tag="5")]
    pub r#type: i32,
    /// The number of values that can be included in a field described by this
    /// metadata.
    #[prost(string, tag="8")]
    pub number: ::prost::alloc::string::String,
    /// A textual description of this metadata.
    #[prost(string, tag="7")]
    pub description: ::prost::alloc::string::String,
    /// Remaining structured metadata key-value pairs. This must be of the form
    /// map<string, string[]> (string key mapping to a list of string values).
    #[prost(btree_map="string, message", tag="3")]
    pub info: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost_types::ListValue>,
}
/// Nested message and enum types in `VariantSetMetadata`.
pub mod variant_set_metadata {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        Unspecified = 0,
        Integer = 1,
        Float = 2,
        Flag = 3,
        Character = 4,
        String = 5,
    }
}
/// A variant set is a collection of call sets and variants. It contains summary
/// statistics of those contents. A variant set belongs to a dataset.
///
/// For more genomics resource definitions, see [Fundamentals of Google
/// Genomics](<https://cloud.google.com/genomics/fundamentals-of-google-genomics>)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VariantSet {
    /// The dataset to which this variant set belongs.
    #[prost(string, tag="1")]
    pub dataset_id: ::prost::alloc::string::String,
    /// The server-generated variant set ID, unique across all variant sets.
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    /// The reference set to which the variant set is mapped. The reference set
    /// describes the alignment provenance of the variant set, while the
    /// `referenceBounds` describe the shape of the actual variant data. The
    /// reference set's reference names are a superset of those found in the
    /// `referenceBounds`.
    ///
    /// For example, given a variant set that is mapped to the GRCh38 reference set
    /// and contains a single variant on reference 'X', `referenceBounds` would
    /// contain only an entry for 'X', while the associated reference set
    /// enumerates all possible references: '1', '2', 'X', 'Y', 'MT', etc.
    #[prost(string, tag="6")]
    pub reference_set_id: ::prost::alloc::string::String,
    /// A list of all references used by the variants in a variant set
    /// with associated coordinate upper bounds for each one.
    #[prost(message, repeated, tag="5")]
    pub reference_bounds: ::prost::alloc::vec::Vec<ReferenceBound>,
    /// The metadata associated with this variant set.
    #[prost(message, repeated, tag="4")]
    pub metadata: ::prost::alloc::vec::Vec<VariantSetMetadata>,
    /// User-specified, mutable name.
    #[prost(string, tag="7")]
    pub name: ::prost::alloc::string::String,
    /// A textual description of this variant set.
    #[prost(string, tag="8")]
    pub description: ::prost::alloc::string::String,
}
/// A variant represents a change in DNA sequence relative to a reference
/// sequence. For example, a variant could represent a SNP or an insertion.
/// Variants belong to a variant set.
///
/// For more genomics resource definitions, see [Fundamentals of Google
/// Genomics](<https://cloud.google.com/genomics/fundamentals-of-google-genomics>)
///
/// Each of the calls on a variant represent a determination of genotype with
/// respect to that variant. For example, a call might assign probability of 0.32
/// to the occurrence of a SNP named rs1234 in a sample named NA12345. A call
/// belongs to a call set, which contains related calls typically from one
/// sample.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Variant {
    /// The ID of the variant set this variant belongs to.
    #[prost(string, tag="15")]
    pub variant_set_id: ::prost::alloc::string::String,
    /// The server-generated variant ID, unique across all variants.
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    /// Names for the variant, for example a RefSNP ID.
    #[prost(string, repeated, tag="3")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The date this variant was created, in milliseconds from the epoch.
    #[prost(int64, tag="12")]
    pub created: i64,
    /// The reference on which this variant occurs.
    /// (such as `chr20` or `X`)
    #[prost(string, tag="14")]
    pub reference_name: ::prost::alloc::string::String,
    /// The position at which this variant occurs (0-based).
    /// This corresponds to the first base of the string of reference bases.
    #[prost(int64, tag="16")]
    pub start: i64,
    /// The end position (0-based) of this variant. This corresponds to the first
    /// base after the last base in the reference allele. So, the length of
    /// the reference allele is (end - start). This is useful for variants
    /// that don't explicitly give alternate bases, for example large deletions.
    #[prost(int64, tag="13")]
    pub end: i64,
    /// The reference bases for this variant. They start at the given
    /// position.
    #[prost(string, tag="6")]
    pub reference_bases: ::prost::alloc::string::String,
    /// The bases that appear instead of the reference bases.
    #[prost(string, repeated, tag="7")]
    pub alternate_bases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A measure of how likely this variant is to be real.
    /// A higher value is better.
    #[prost(double, tag="8")]
    pub quality: f64,
    /// A list of filters (normally quality filters) this variant has failed.
    /// `PASS` indicates this variant has passed all filters.
    #[prost(string, repeated, tag="9")]
    pub filter: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A map of additional variant information. This must be of the form
    /// map<string, string[]> (string key mapping to a list of string values).
    #[prost(btree_map="string, message", tag="10")]
    pub info: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost_types::ListValue>,
    /// The variant calls for this particular variant. Each one represents the
    /// determination of genotype with respect to this variant.
    #[prost(message, repeated, tag="11")]
    pub calls: ::prost::alloc::vec::Vec<VariantCall>,
}
/// A call represents the determination of genotype with respect to a particular
/// variant. It may include associated information such as quality and phasing.
/// For example, a call might assign a probability of 0.32 to the occurrence of
/// a SNP named rs1234 in a call set with the name NA12345.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VariantCall {
    /// The ID of the call set this variant call belongs to.
    #[prost(string, tag="8")]
    pub call_set_id: ::prost::alloc::string::String,
    /// The name of the call set this variant call belongs to.
    #[prost(string, tag="9")]
    pub call_set_name: ::prost::alloc::string::String,
    /// The genotype of this variant call. Each value represents either the value
    /// of the `referenceBases` field or a 1-based index into
    /// `alternateBases`. If a variant had a `referenceBases`
    /// value of `T` and an `alternateBases`
    /// value of `["A", "C"]`, and the `genotype` was
    /// `[2, 1]`, that would mean the call
    /// represented the heterozygous value `CA` for this variant.
    /// If the `genotype` was instead `[0, 1]`, the
    /// represented value would be `TA`. Ordering of the
    /// genotype values is important if the `phaseset` is present.
    /// If a genotype is not called (that is, a `.` is present in the
    /// GT string) -1 is returned.
    #[prost(int32, repeated, tag="7")]
    pub genotype: ::prost::alloc::vec::Vec<i32>,
    /// If this field is present, this variant call's genotype ordering implies
    /// the phase of the bases and is consistent with any other variant calls in
    /// the same reference sequence which have the same phaseset value.
    /// When importing data from VCF, if the genotype data was phased but no
    /// phase set was specified this field will be set to `*`.
    #[prost(string, tag="5")]
    pub phaseset: ::prost::alloc::string::String,
    /// The genotype likelihoods for this variant call. Each array entry
    /// represents how likely a specific genotype is for this call. The value
    /// ordering is defined by the GL tag in the VCF spec.
    /// If Phred-scaled genotype likelihood scores (PL) are available and
    /// log10(P) genotype likelihood scores (GL) are not, PL scores are converted
    /// to GL scores.  If both are available, PL scores are stored in `info`.
    #[prost(double, repeated, tag="6")]
    pub genotype_likelihood: ::prost::alloc::vec::Vec<f64>,
    /// A map of additional variant call information. This must be of the form
    /// map<string, string[]> (string key mapping to a list of string values).
    #[prost(btree_map="string, message", tag="2")]
    pub info: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost_types::ListValue>,
}
/// A call set is a collection of variant calls, typically for one sample. It
/// belongs to a variant set.
///
/// For more genomics resource definitions, see [Fundamentals of Google
/// Genomics](<https://cloud.google.com/genomics/fundamentals-of-google-genomics>)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallSet {
    /// The server-generated call set ID, unique across all call sets.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The call set name.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// The sample ID this call set corresponds to.
    #[prost(string, tag="7")]
    pub sample_id: ::prost::alloc::string::String,
    /// The IDs of the variant sets this call set belongs to. This field must
    /// have exactly length one, as a call set belongs to a single variant set.
    /// This field is repeated for compatibility with the
    /// [GA4GH 0.5.1
    /// API](<https://github.com/ga4gh/schemas/blob/v0.5.1/src/main/resources/avro/variants.avdl#L76>).
    #[prost(string, repeated, tag="6")]
    pub variant_set_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The date this call set was created in milliseconds from the epoch.
    #[prost(int64, tag="5")]
    pub created: i64,
    /// A map of additional call set information. This must be of the form
    /// map<string, string[]> (string key mapping to a list of string values).
    #[prost(btree_map="string, message", tag="4")]
    pub info: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost_types::ListValue>,
}
/// ReferenceBound records an upper bound for the starting coordinate of
/// variants in a particular reference.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReferenceBound {
    /// The name of the reference associated with this reference bound.
    #[prost(string, tag="1")]
    pub reference_name: ::prost::alloc::string::String,
    /// An upper bound (inclusive) on the starting coordinate of any
    /// variant in the reference sequence.
    #[prost(int64, tag="2")]
    pub upper_bound: i64,
}
/// The variant data import request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportVariantsRequest {
    /// Required. The variant set to which variant data should be imported.
    #[prost(string, tag="1")]
    pub variant_set_id: ::prost::alloc::string::String,
    /// A list of URIs referencing variant files in Google Cloud Storage. URIs can
    /// include wildcards [as described
    /// here](<https://cloud.google.com/storage/docs/gsutil/addlhelp/WildcardNames>).
    /// Note that recursive wildcards ('**') are not supported.
    #[prost(string, repeated, tag="2")]
    pub source_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The format of the variant data being imported. If unspecified, defaults to
    /// to `VCF`.
    #[prost(enumeration="import_variants_request::Format", tag="3")]
    pub format: i32,
    /// Convert reference names to the canonical representation.
    /// hg19 haploytypes (those reference names containing "_hap")
    /// are not modified in any way.
    /// All other reference names are modified according to the following rules:
    /// The reference name is capitalized.
    /// The "chr" prefix is dropped for all autosomes and sex chromsomes.
    /// For example "chr17" becomes "17" and "chrX" becomes "X".
    /// All mitochondrial chromosomes ("chrM", "chrMT", etc) become "MT".
    #[prost(bool, tag="5")]
    pub normalize_reference_names: bool,
    /// A mapping between info field keys and the InfoMergeOperations to
    /// be performed on them. This is plumbed down to the MergeVariantRequests
    /// generated by the resulting import job.
    #[prost(btree_map="string, enumeration(InfoMergeOperation)", tag="6")]
    pub info_merge_config: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, i32>,
}
/// Nested message and enum types in `ImportVariantsRequest`.
pub mod import_variants_request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Format {
        Unspecified = 0,
        /// VCF (Variant Call Format). The VCF files may be gzip compressed. gVCF is
        /// also supported.
        Vcf = 1,
        /// Complete Genomics masterVarBeta format. The masterVarBeta files may
        /// be bzip2 compressed.
        CompleteGenomics = 2,
    }
}
/// The variant data import response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportVariantsResponse {
    /// IDs of the call sets created during the import.
    #[prost(string, repeated, tag="1")]
    pub call_set_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The CreateVariantSet request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateVariantSetRequest {
    /// Required. The variant set to be created. Must have a valid `datasetId`.
    #[prost(message, optional, tag="1")]
    pub variant_set: ::core::option::Option<VariantSet>,
}
/// The variant data export request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportVariantSetRequest {
    /// Required. The ID of the variant set that contains variant data which
    /// should be exported. The caller must have READ access to this variant set.
    #[prost(string, tag="1")]
    pub variant_set_id: ::prost::alloc::string::String,
    /// If provided, only variant call information from the specified call sets
    /// will be exported. By default all variant calls are exported.
    #[prost(string, repeated, tag="2")]
    pub call_set_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The Google Cloud project ID that owns the destination
    /// BigQuery dataset. The caller must have WRITE access to this project.  This
    /// project will also own the resulting export job.
    #[prost(string, tag="3")]
    pub project_id: ::prost::alloc::string::String,
    /// The format for the exported data.
    #[prost(enumeration="export_variant_set_request::Format", tag="4")]
    pub format: i32,
    /// Required. The BigQuery dataset to export data to. This dataset must already
    /// exist. Note that this is distinct from the Genomics concept of "dataset".
    #[prost(string, tag="5")]
    pub bigquery_dataset: ::prost::alloc::string::String,
    /// Required. The BigQuery table to export data to.
    /// If the table doesn't exist, it will be created. If it already exists, it
    /// will be overwritten.
    #[prost(string, tag="6")]
    pub bigquery_table: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ExportVariantSetRequest`.
pub mod export_variant_set_request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Format {
        Unspecified = 0,
        /// Export the data to Google BigQuery.
        Bigquery = 1,
    }
}
/// The variant set request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVariantSetRequest {
    /// Required. The ID of the variant set.
    #[prost(string, tag="1")]
    pub variant_set_id: ::prost::alloc::string::String,
}
/// The search variant sets request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchVariantSetsRequest {
    /// Exactly one dataset ID must be provided here. Only variant sets which
    /// belong to this dataset will be returned.
    #[prost(string, repeated, tag="1")]
    pub dataset_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The continuation token, which is used to page through large result sets.
    /// To get the next page of results, set this parameter to the value of
    /// `nextPageToken` from the previous response.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single page. If unspecified,
    /// defaults to 1024.
    #[prost(int32, tag="3")]
    pub page_size: i32,
}
/// The search variant sets response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchVariantSetsResponse {
    /// The variant sets belonging to the requested dataset.
    #[prost(message, repeated, tag="1")]
    pub variant_sets: ::prost::alloc::vec::Vec<VariantSet>,
    /// The continuation token, which is used to page through large result sets.
    /// Provide this value in a subsequent request to return the next page of
    /// results. This field will be empty if there aren't any additional results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The delete variant set request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteVariantSetRequest {
    /// The ID of the variant set to be deleted.
    #[prost(string, tag="1")]
    pub variant_set_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVariantSetRequest {
    /// The ID of the variant to be updated (must already exist).
    #[prost(string, tag="1")]
    pub variant_set_id: ::prost::alloc::string::String,
    /// The new variant data. Only the variant_set.metadata will be considered
    /// for update.
    #[prost(message, optional, tag="2")]
    pub variant_set: ::core::option::Option<VariantSet>,
    /// An optional mask specifying which fields to update. Supported fields:
    ///
    /// * \[metadata][google.genomics.v1.VariantSet.metadata\].
    /// * \[name][google.genomics.v1.VariantSet.name\].
    /// * \[description][google.genomics.v1.VariantSet.description\].
    ///
    /// Leaving `updateMask` unset is equivalent to specifying all mutable
    /// fields.
    #[prost(message, optional, tag="5")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The variant search request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchVariantsRequest {
    /// At most one variant set ID must be provided. Only variants from this
    /// variant set will be returned. If omitted, a call set id must be included in
    /// the request.
    #[prost(string, repeated, tag="1")]
    pub variant_set_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Only return variants which have exactly this name.
    #[prost(string, tag="2")]
    pub variant_name: ::prost::alloc::string::String,
    /// Only return variant calls which belong to call sets with these ids.
    /// Leaving this blank returns all variant calls. If a variant has no
    /// calls belonging to any of these call sets, it won't be returned at all.
    #[prost(string, repeated, tag="3")]
    pub call_set_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. Only return variants in this reference sequence.
    #[prost(string, tag="4")]
    pub reference_name: ::prost::alloc::string::String,
    /// The beginning of the window (0-based, inclusive) for which
    /// overlapping variants should be returned. If unspecified, defaults to 0.
    #[prost(int64, tag="5")]
    pub start: i64,
    /// The end of the window, 0-based exclusive. If unspecified or 0, defaults to
    /// the length of the reference.
    #[prost(int64, tag="6")]
    pub end: i64,
    /// The continuation token, which is used to page through large result sets.
    /// To get the next page of results, set this parameter to the value of
    /// `nextPageToken` from the previous response.
    #[prost(string, tag="7")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of variants to return in a single page. If unspecified,
    /// defaults to 5000. The maximum value is 10000.
    #[prost(int32, tag="8")]
    pub page_size: i32,
    /// The maximum number of calls to return in a single page. Note that this
    /// limit may be exceeded in the event that a matching variant contains more
    /// calls than the requested maximum. If unspecified, defaults to 5000. The
    /// maximum value is 10000.
    #[prost(int32, tag="9")]
    pub max_calls: i32,
}
/// The variant search response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchVariantsResponse {
    /// The list of matching Variants.
    #[prost(message, repeated, tag="1")]
    pub variants: ::prost::alloc::vec::Vec<Variant>,
    /// The continuation token, which is used to page through large result sets.
    /// Provide this value in a subsequent request to return the next page of
    /// results. This field will be empty if there aren't any additional results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateVariantRequest {
    /// The variant to be created.
    #[prost(message, optional, tag="1")]
    pub variant: ::core::option::Option<Variant>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVariantRequest {
    /// The ID of the variant to be updated.
    #[prost(string, tag="1")]
    pub variant_id: ::prost::alloc::string::String,
    /// The new variant data.
    #[prost(message, optional, tag="2")]
    pub variant: ::core::option::Option<Variant>,
    /// An optional mask specifying which fields to update. At this time, mutable
    /// fields are \[names][google.genomics.v1.Variant.names\] and
    /// \[info][google.genomics.v1.Variant.info\]. Acceptable values are "names" and
    /// "info". If unspecified, all mutable fields will be updated.
    #[prost(message, optional, tag="3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteVariantRequest {
    /// The ID of the variant to be deleted.
    #[prost(string, tag="1")]
    pub variant_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVariantRequest {
    /// The ID of the variant.
    #[prost(string, tag="1")]
    pub variant_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MergeVariantsRequest {
    /// The destination variant set.
    #[prost(string, tag="1")]
    pub variant_set_id: ::prost::alloc::string::String,
    /// The variants to be merged with existing variants.
    #[prost(message, repeated, tag="2")]
    pub variants: ::prost::alloc::vec::Vec<Variant>,
    /// A mapping between info field keys and the InfoMergeOperations to
    /// be performed on them.
    #[prost(btree_map="string, enumeration(InfoMergeOperation)", tag="3")]
    pub info_merge_config: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, i32>,
}
/// The call set search request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchCallSetsRequest {
    /// Restrict the query to call sets within the given variant sets. At least one
    /// ID must be provided.
    #[prost(string, repeated, tag="1")]
    pub variant_set_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Only return call sets for which a substring of the name matches this
    /// string.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// The continuation token, which is used to page through large result sets.
    /// To get the next page of results, set this parameter to the value of
    /// `nextPageToken` from the previous response.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single page. If unspecified,
    /// defaults to 1024.
    #[prost(int32, tag="4")]
    pub page_size: i32,
}
/// The call set search response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchCallSetsResponse {
    /// The list of matching call sets.
    #[prost(message, repeated, tag="1")]
    pub call_sets: ::prost::alloc::vec::Vec<CallSet>,
    /// The continuation token, which is used to page through large result sets.
    /// Provide this value in a subsequent request to return the next page of
    /// results. This field will be empty if there aren't any additional results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCallSetRequest {
    /// The call set to be created.
    #[prost(message, optional, tag="1")]
    pub call_set: ::core::option::Option<CallSet>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCallSetRequest {
    /// The ID of the call set to be updated.
    #[prost(string, tag="1")]
    pub call_set_id: ::prost::alloc::string::String,
    /// The new call set data.
    #[prost(message, optional, tag="2")]
    pub call_set: ::core::option::Option<CallSet>,
    /// An optional mask specifying which fields to update. At this time, the only
    /// mutable field is \[name][google.genomics.v1.CallSet.name\]. The only
    /// acceptable value is "name". If unspecified, all mutable fields will be
    /// updated.
    #[prost(message, optional, tag="3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCallSetRequest {
    /// The ID of the call set to be deleted.
    #[prost(string, tag="1")]
    pub call_set_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCallSetRequest {
    /// The ID of the call set.
    #[prost(string, tag="1")]
    pub call_set_id: ::prost::alloc::string::String,
}
/// The stream variants request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamVariantsRequest {
    /// The Google Cloud project ID which will be billed
    /// for this access. The caller must have WRITE access to this project.
    /// Required.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The variant set ID from which to stream variants.
    #[prost(string, tag="2")]
    pub variant_set_id: ::prost::alloc::string::String,
    /// Only return variant calls which belong to call sets with these IDs.
    /// Leaving this blank returns all variant calls.
    #[prost(string, repeated, tag="3")]
    pub call_set_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. Only return variants in this reference sequence.
    #[prost(string, tag="4")]
    pub reference_name: ::prost::alloc::string::String,
    /// The beginning of the window (0-based, inclusive) for which
    /// overlapping variants should be returned.
    #[prost(int64, tag="5")]
    pub start: i64,
    /// The end of the window (0-based, exclusive) for which overlapping
    /// variants should be returned.
    #[prost(int64, tag="6")]
    pub end: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamVariantsResponse {
    #[prost(message, repeated, tag="1")]
    pub variants: ::prost::alloc::vec::Vec<Variant>,
}
/// Operations to be performed during import on Variant info fields.
/// These operations are set for each info field in the info_merge_config
/// map of ImportVariantsRequest, which is plumbed down to the
/// MergeVariantRequests generated by the import job.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InfoMergeOperation {
    Unspecified = 0,
    /// By default, Variant info fields are persisted if the Variant doesn't
    /// already exist in the variantset.  If the Variant is equivalent to a
    /// Variant already in the variantset, the incoming Variant's info field
    /// is ignored in favor of that of the already persisted Variant.
    IgnoreNew = 1,
    /// This operation removes an info field from the incoming Variant
    /// and persists this info field in each of the incoming Variant's Calls.
    MoveToCalls = 2,
}
/// Generated client implementations.
pub mod streaming_variant_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct StreamingVariantServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> StreamingVariantServiceClient<T>
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
        ) -> StreamingVariantServiceClient<InterceptedService<T, F>>
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
            StreamingVariantServiceClient::new(
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
        /// Returns a stream of all the variants matching the search request, ordered
        /// by reference name, position, and ID.
        pub async fn stream_variants(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamVariantsRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::StreamVariantsResponse>>,
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
                "/google.genomics.v1.StreamingVariantService/StreamVariants",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod variant_service_v1_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct VariantServiceV1Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> VariantServiceV1Client<T>
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
        ) -> VariantServiceV1Client<InterceptedService<T, F>>
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
            VariantServiceV1Client::new(InterceptedService::new(inner, interceptor))
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
        /// Creates variant data by asynchronously importing the provided information.
        ///
        /// For the definitions of variant sets and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        ///
        /// The variants for import will be merged with any existing variant that
        /// matches its reference sequence, start, end, reference bases, and
        /// alternative bases. If no such variant exists, a new one will be created.
        ///
        /// When variants are merged, the call information from the new variant
        /// is added to the existing variant, and Variant info fields are merged
        /// as specified in
        /// [infoMergeConfig][google.genomics.v1.ImportVariantsRequest.info_merge_config].
        /// As a special case, for single-sample VCF files, QUAL and FILTER fields will
        /// be moved to the call level; these are sometimes interpreted in a
        /// call-specific context.
        /// Imported VCF headers are appended to the metadata already in a variant set.
        pub async fn import_variants(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportVariantsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.genomics.v1.VariantServiceV1/ImportVariants",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new variant set.
        ///
        /// For the definitions of variant sets and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        ///
        /// The provided variant set must have a valid `datasetId` set - all other
        /// fields are optional. Note that the `id` field will be ignored, as this is
        /// assigned by the server.
        pub async fn create_variant_set(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateVariantSetRequest>,
        ) -> Result<tonic::Response<super::VariantSet>, tonic::Status> {
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
                "/google.genomics.v1.VariantServiceV1/CreateVariantSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Exports variant set data to an external destination.
        ///
        /// For the definitions of variant sets and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        pub async fn export_variant_set(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportVariantSetRequest>,
        ) -> Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.genomics.v1.VariantServiceV1/ExportVariantSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a variant set by ID.
        ///
        /// For the definitions of variant sets and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        pub async fn get_variant_set(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVariantSetRequest>,
        ) -> Result<tonic::Response<super::VariantSet>, tonic::Status> {
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
                "/google.genomics.v1.VariantServiceV1/GetVariantSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns a list of all variant sets matching search criteria.
        ///
        /// For the definitions of variant sets and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        ///
        /// Implements
        /// [GlobalAllianceApi.searchVariantSets](https://github.com/ga4gh/schemas/blob/v0.5.1/src/main/resources/avro/variantmethods.avdl#L49).
        pub async fn search_variant_sets(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchVariantSetsRequest>,
        ) -> Result<tonic::Response<super::SearchVariantSetsResponse>, tonic::Status> {
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
                "/google.genomics.v1.VariantServiceV1/SearchVariantSets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a variant set including all variants, call sets, and calls within.
        /// This is not reversible.
        ///
        /// For the definitions of variant sets and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        pub async fn delete_variant_set(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteVariantSetRequest>,
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
                "/google.genomics.v1.VariantServiceV1/DeleteVariantSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a variant set using patch semantics.
        ///
        /// For the definitions of variant sets and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        pub async fn update_variant_set(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateVariantSetRequest>,
        ) -> Result<tonic::Response<super::VariantSet>, tonic::Status> {
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
                "/google.genomics.v1.VariantServiceV1/UpdateVariantSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a list of variants matching the criteria.
        ///
        /// For the definitions of variants and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        ///
        /// Implements
        /// [GlobalAllianceApi.searchVariants](https://github.com/ga4gh/schemas/blob/v0.5.1/src/main/resources/avro/variantmethods.avdl#L126).
        pub async fn search_variants(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchVariantsRequest>,
        ) -> Result<tonic::Response<super::SearchVariantsResponse>, tonic::Status> {
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
                "/google.genomics.v1.VariantServiceV1/SearchVariants",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new variant.
        ///
        /// For the definitions of variants and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        pub async fn create_variant(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateVariantRequest>,
        ) -> Result<tonic::Response<super::Variant>, tonic::Status> {
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
                "/google.genomics.v1.VariantServiceV1/CreateVariant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a variant.
        ///
        /// For the definitions of variants and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        ///
        /// This method supports patch semantics. Returns the modified variant without
        /// its calls.
        pub async fn update_variant(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateVariantRequest>,
        ) -> Result<tonic::Response<super::Variant>, tonic::Status> {
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
                "/google.genomics.v1.VariantServiceV1/UpdateVariant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a variant.
        ///
        /// For the definitions of variants and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        pub async fn delete_variant(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteVariantRequest>,
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
                "/google.genomics.v1.VariantServiceV1/DeleteVariant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a variant by ID.
        ///
        /// For the definitions of variants and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        pub async fn get_variant(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVariantRequest>,
        ) -> Result<tonic::Response<super::Variant>, tonic::Status> {
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
                "/google.genomics.v1.VariantServiceV1/GetVariant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Merges the given variants with existing variants.
        ///
        /// For the definitions of variants and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        ///
        /// Each variant will be
        /// merged with an existing variant that matches its reference sequence,
        /// start, end, reference bases, and alternative bases. If no such variant
        /// exists, a new one will be created.
        ///
        /// When variants are merged, the call information from the new variant
        /// is added to the existing variant. Variant info fields are merged as
        /// specified in the
        /// [infoMergeConfig][google.genomics.v1.MergeVariantsRequest.info_merge_config]
        /// field of the MergeVariantsRequest.
        ///
        /// Please exercise caution when using this method!  It is easy to introduce
        /// mistakes in existing variants and difficult to back out of them.  For
        /// example,
        /// suppose you were trying to merge a new variant with an existing one and
        /// both
        /// variants contain calls that belong to callsets with the same callset ID.
        ///
        ///     // Existing variant - irrelevant fields trimmed for clarity
        ///     {
        ///         "variantSetId": "10473108253681171589",
        ///         "referenceName": "1",
        ///         "start": "10582",
        ///         "referenceBases": "G",
        ///         "alternateBases": [
        ///             "A"
        ///         ],
        ///         "calls": [
        ///             {
        ///                 "callSetId": "10473108253681171589-0",
        ///                 "callSetName": "CALLSET0",
        ///                 "genotype": [
        ///                     0,
        ///                     1
        ///                 ],
        ///             }
        ///         ]
        ///     }
        ///
        ///     // New variant with conflicting call information
        ///     {
        ///         "variantSetId": "10473108253681171589",
        ///         "referenceName": "1",
        ///         "start": "10582",
        ///         "referenceBases": "G",
        ///         "alternateBases": [
        ///             "A"
        ///         ],
        ///         "calls": [
        ///             {
        ///                 "callSetId": "10473108253681171589-0",
        ///                 "callSetName": "CALLSET0",
        ///                 "genotype": [
        ///                     1,
        ///                     1
        ///                 ],
        ///             }
        ///         ]
        ///     }
        ///
        /// The resulting merged variant would overwrite the existing calls with those
        /// from the new variant:
        ///
        ///     {
        ///         "variantSetId": "10473108253681171589",
        ///         "referenceName": "1",
        ///         "start": "10582",
        ///         "referenceBases": "G",
        ///         "alternateBases": [
        ///             "A"
        ///         ],
        ///         "calls": [
        ///             {
        ///                 "callSetId": "10473108253681171589-0",
        ///                 "callSetName": "CALLSET0",
        ///                 "genotype": [
        ///                     1,
        ///                     1
        ///                 ],
        ///             }
        ///         ]
        ///     }
        ///
        /// This may be the desired outcome, but it is up to the user to determine if
        /// if that is indeed the case.
        pub async fn merge_variants(
            &mut self,
            request: impl tonic::IntoRequest<super::MergeVariantsRequest>,
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
                "/google.genomics.v1.VariantServiceV1/MergeVariants",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a list of call sets matching the criteria.
        ///
        /// For the definitions of call sets and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        ///
        /// Implements
        /// [GlobalAllianceApi.searchCallSets](https://github.com/ga4gh/schemas/blob/v0.5.1/src/main/resources/avro/variantmethods.avdl#L178).
        pub async fn search_call_sets(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchCallSetsRequest>,
        ) -> Result<tonic::Response<super::SearchCallSetsResponse>, tonic::Status> {
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
                "/google.genomics.v1.VariantServiceV1/SearchCallSets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new call set.
        ///
        /// For the definitions of call sets and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        pub async fn create_call_set(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCallSetRequest>,
        ) -> Result<tonic::Response<super::CallSet>, tonic::Status> {
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
                "/google.genomics.v1.VariantServiceV1/CreateCallSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a call set.
        ///
        /// For the definitions of call sets and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        ///
        /// This method supports patch semantics.
        pub async fn update_call_set(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCallSetRequest>,
        ) -> Result<tonic::Response<super::CallSet>, tonic::Status> {
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
                "/google.genomics.v1.VariantServiceV1/UpdateCallSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a call set.
        ///
        /// For the definitions of call sets and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        pub async fn delete_call_set(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCallSetRequest>,
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
                "/google.genomics.v1.VariantServiceV1/DeleteCallSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a call set by ID.
        ///
        /// For the definitions of call sets and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        pub async fn get_call_set(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCallSetRequest>,
        ) -> Result<tonic::Response<super::CallSet>, tonic::Status> {
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
                "/google.genomics.v1.VariantServiceV1/GetCallSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Metadata describing an \[Operation][google.longrunning.Operation\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// The Google Cloud Project in which the job is scoped.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The time at which the job was submitted to the Genomics service.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time at which the job began to run.
    #[prost(message, optional, tag="3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time at which the job stopped running.
    #[prost(message, optional, tag="4")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The original request that started the operation. Note that this will be in
    /// current version of the API. If the operation was started with v1beta2 API
    /// and a GetOperation is performed on v1 API, a v1 request will be returned.
    #[prost(message, optional, tag="5")]
    pub request: ::core::option::Option<::prost_types::Any>,
    /// Optional event messages that were generated during the job's execution.
    /// This also contains any warnings that were generated during import
    /// or export.
    #[prost(message, repeated, tag="6")]
    pub events: ::prost::alloc::vec::Vec<OperationEvent>,
    /// This field is deprecated. Use `labels` instead. Optionally provided by the
    /// caller when submitting the request that creates the operation.
    #[prost(string, tag="7")]
    pub client_id: ::prost::alloc::string::String,
    /// Runtime metadata on this Operation.
    #[prost(message, optional, tag="8")]
    pub runtime_metadata: ::core::option::Option<::prost_types::Any>,
    /// Optionally provided by the caller when submitting the request that creates
    /// the operation.
    #[prost(btree_map="string, string", tag="9")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// An event that occurred during an \[Operation][google.longrunning.Operation\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationEvent {
    /// Optional time of when event started.
    #[prost(message, optional, tag="1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional time of when event finished. An event can have a start time and no
    /// finish time. If an event has a finish time, there must be a start time.
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required description of event.
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
}
/// A 0-based half-open genomic coordinate range for search requests.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Range {
    /// The reference sequence name, for example `chr1`,
    /// `1`, or `chrX`.
    #[prost(string, tag="1")]
    pub reference_name: ::prost::alloc::string::String,
    /// The start position of the range on the reference, 0-based inclusive.
    #[prost(int64, tag="2")]
    pub start: i64,
    /// The end position of the range on the reference, 0-based exclusive.
    #[prost(int64, tag="3")]
    pub end: i64,
}
/// An annotation set is a logical grouping of annotations that share consistent
/// type information and provenance. Examples of annotation sets include 'all
/// genes from refseq', and 'all variant annotations from ClinVar'.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotationSet {
    /// The server-generated annotation set ID, unique across all annotation sets.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The dataset to which this annotation set belongs.
    #[prost(string, tag="2")]
    pub dataset_id: ::prost::alloc::string::String,
    /// The ID of the reference set that defines the coordinate space for this
    /// set's annotations.
    #[prost(string, tag="3")]
    pub reference_set_id: ::prost::alloc::string::String,
    /// The display name for this annotation set.
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    /// The source URI describing the file from which this annotation set was
    /// generated, if any.
    #[prost(string, tag="5")]
    pub source_uri: ::prost::alloc::string::String,
    /// The type of annotations contained within this set.
    #[prost(enumeration="AnnotationType", tag="6")]
    pub r#type: i32,
    /// A map of additional read alignment information. This must be of the form
    /// map<string, string[]> (string key mapping to a list of string values).
    #[prost(btree_map="string, message", tag="17")]
    pub info: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost_types::ListValue>,
}
/// An annotation describes a region of reference genome. The value of an
/// annotation may be one of several canonical types, supplemented by arbitrary
/// info tags. An annotation is not inherently associated with a specific
/// sample or individual (though a client could choose to use annotations in
/// this way). Example canonical annotation types are `GENE` and
/// `VARIANT`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Annotation {
    /// The server-generated annotation ID, unique across all annotations.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The annotation set to which this annotation belongs.
    #[prost(string, tag="2")]
    pub annotation_set_id: ::prost::alloc::string::String,
    /// The display name of this annotation.
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    /// The ID of the Google Genomics reference associated with this range.
    #[prost(string, tag="4")]
    pub reference_id: ::prost::alloc::string::String,
    /// The display name corresponding to the reference specified by
    /// `referenceId`, for example `chr1`, `1`, or `chrX`.
    #[prost(string, tag="5")]
    pub reference_name: ::prost::alloc::string::String,
    /// The start position of the range on the reference, 0-based inclusive.
    #[prost(int64, tag="6")]
    pub start: i64,
    /// The end position of the range on the reference, 0-based exclusive.
    #[prost(int64, tag="7")]
    pub end: i64,
    /// Whether this range refers to the reverse strand, as opposed to the forward
    /// strand. Note that regardless of this field, the start/end position of the
    /// range always refer to the forward strand.
    #[prost(bool, tag="8")]
    pub reverse_strand: bool,
    /// The data type for this annotation. Must match the containing annotation
    /// set's type.
    #[prost(enumeration="AnnotationType", tag="9")]
    pub r#type: i32,
    /// A map of additional read alignment information. This must be of the form
    /// map<string, string[]> (string key mapping to a list of string values).
    #[prost(btree_map="string, message", tag="12")]
    pub info: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost_types::ListValue>,
    #[prost(oneof="annotation::Value", tags="10, 11")]
    pub value: ::core::option::Option<annotation::Value>,
}
/// Nested message and enum types in `Annotation`.
pub mod annotation {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// A variant annotation, which describes the effect of a variant on the
        /// genome, the coding sequence, and/or higher level consequences at the
        /// organism level e.g. pathogenicity. This field is only set for annotations
        /// of type `VARIANT`.
        #[prost(message, tag="10")]
        Variant(super::VariantAnnotation),
        /// A transcript value represents the assertion that a particular region of
        /// the reference genome may be transcribed as RNA. An alternative splicing
        /// pattern would be represented as a separate transcript object. This field
        /// is only set for annotations of type `TRANSCRIPT`.
        #[prost(message, tag="11")]
        Transcript(super::Transcript),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VariantAnnotation {
    /// Type has been adapted from ClinVar's list of variant types.
    #[prost(enumeration="variant_annotation::Type", tag="1")]
    pub r#type: i32,
    /// Effect of the variant on the coding sequence.
    #[prost(enumeration="variant_annotation::Effect", tag="2")]
    pub effect: i32,
    /// The alternate allele for this variant. If multiple alternate alleles
    /// exist at this location, create a separate variant for each one, as they
    /// may represent distinct conditions.
    #[prost(string, tag="3")]
    pub alternate_bases: ::prost::alloc::string::String,
    /// Google annotation ID of the gene affected by this variant. This should
    /// be provided when the variant is created.
    #[prost(string, tag="4")]
    pub gene_id: ::prost::alloc::string::String,
    /// Google annotation IDs of the transcripts affected by this variant. These
    /// should be provided when the variant is created.
    #[prost(string, repeated, tag="5")]
    pub transcript_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The set of conditions associated with this variant.
    /// A condition describes the way a variant influences human health.
    #[prost(message, repeated, tag="6")]
    pub conditions: ::prost::alloc::vec::Vec<variant_annotation::ClinicalCondition>,
    /// Describes the clinical significance of a variant.
    /// It is adapted from the ClinVar controlled vocabulary for clinical
    /// significance described at:
    /// <http://www.ncbi.nlm.nih.gov/clinvar/docs/clinsig/>
    #[prost(enumeration="variant_annotation::ClinicalSignificance", tag="7")]
    pub clinical_significance: i32,
}
/// Nested message and enum types in `VariantAnnotation`.
pub mod variant_annotation {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ClinicalCondition {
        /// A set of names for the condition.
        #[prost(string, repeated, tag="1")]
        pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The set of external IDs for this condition.
        #[prost(message, repeated, tag="2")]
        pub external_ids: ::prost::alloc::vec::Vec<super::ExternalId>,
        /// The MedGen concept id associated with this gene.
        /// Search for these IDs at <http://www.ncbi.nlm.nih.gov/medgen/>
        #[prost(string, tag="3")]
        pub concept_id: ::prost::alloc::string::String,
        /// The OMIM id for this condition.
        /// Search for these IDs at <http://omim.org/>
        #[prost(string, tag="4")]
        pub omim_id: ::prost::alloc::string::String,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        Unspecified = 0,
        /// `TYPE_OTHER` should be used when no other Type will suffice.
        /// Further explanation of the variant type may be included in the
        /// \[info][google.genomics.v1.Annotation.info\] field.
        Other = 1,
        /// `INSERTION` indicates an insertion.
        Insertion = 2,
        /// `DELETION` indicates a deletion.
        Deletion = 3,
        /// `SUBSTITUTION` indicates a block substitution of
        /// two or more nucleotides.
        Substitution = 4,
        /// `SNP` indicates a single nucleotide polymorphism.
        Snp = 5,
        /// `STRUCTURAL` indicates a large structural variant,
        /// including chromosomal fusions, inversions, etc.
        Structural = 6,
        /// `CNV` indicates a variation in copy number.
        Cnv = 7,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Effect {
        Unspecified = 0,
        /// `EFFECT_OTHER` should be used when no other Effect
        /// will suffice.
        Other = 1,
        /// `FRAMESHIFT` indicates a mutation in which the insertion or
        /// deletion of nucleotides resulted in a frameshift change.
        Frameshift = 2,
        /// `FRAME_PRESERVING_INDEL` indicates a mutation in which a
        /// multiple of three nucleotides has been inserted or deleted, resulting
        /// in no change to the reading frame of the coding sequence.
        FramePreservingIndel = 3,
        /// `SYNONYMOUS_SNP` indicates a single nucleotide polymorphism
        /// mutation that results in no amino acid change.
        SynonymousSnp = 4,
        /// `NONSYNONYMOUS_SNP` indicates a single nucleotide
        /// polymorphism mutation that results in an amino acid change.
        NonsynonymousSnp = 5,
        /// `STOP_GAIN` indicates a mutation that leads to the creation
        /// of a stop codon at the variant site. Frameshift mutations creating
        /// downstream stop codons do not count as `STOP_GAIN`.
        StopGain = 6,
        /// `STOP_LOSS` indicates a mutation that eliminates a
        /// stop codon at the variant site.
        StopLoss = 7,
        /// `SPLICE_SITE_DISRUPTION` indicates that this variant is
        /// found in a splice site for the associated transcript, and alters the
        /// normal splicing pattern.
        SpliceSiteDisruption = 8,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ClinicalSignificance {
        Unspecified = 0,
        /// `OTHER` should be used when no other clinical significance
        /// value will suffice.
        Other = 1,
        Uncertain = 2,
        Benign = 3,
        LikelyBenign = 4,
        LikelyPathogenic = 5,
        Pathogenic = 6,
        DrugResponse = 7,
        Histocompatibility = 8,
        ConfersSensitivity = 9,
        RiskFactor = 10,
        Association = 11,
        Protective = 12,
        /// `MULTIPLE_REPORTED` should be used when multiple clinical
        /// signficances are reported for a variant. The original clinical
        /// significance values may be provided in the `info` field.
        MultipleReported = 13,
    }
}
/// A transcript represents the assertion that a particular region of the
/// reference genome may be transcribed as RNA.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transcript {
    /// The annotation ID of the gene from which this transcript is transcribed.
    #[prost(string, tag="1")]
    pub gene_id: ::prost::alloc::string::String,
    /// The <a href="<http://en.wikipedia.org/wiki/Exon">exons</a>> that compose
    /// this transcript. This field should be unset for genomes where transcript
    /// splicing does not occur, for example prokaryotes.
    ///
    /// Introns are regions of the transcript that are not included in the
    /// spliced RNA product. Though not explicitly modeled here, intron ranges can
    /// be deduced; all regions of this transcript that are not exons are introns.
    ///
    /// Exonic sequences do not necessarily code for a translational product
    /// (amino acids). Only the regions of exons bounded by the
    /// \[codingSequence][google.genomics.v1.Transcript.coding_sequence\] correspond
    /// to coding DNA sequence.
    ///
    /// Exons are ordered by start position and may not overlap.
    #[prost(message, repeated, tag="2")]
    pub exons: ::prost::alloc::vec::Vec<transcript::Exon>,
    /// The range of the coding sequence for this transcript, if any. To determine
    /// the exact ranges of coding sequence, intersect this range with those of the
    /// \[exons][google.genomics.v1.Transcript.exons\], if any. If there are any
    /// \[exons][google.genomics.v1.Transcript.exons\], the
    /// \[codingSequence][google.genomics.v1.Transcript.coding_sequence\] must start
    /// and end within them.
    ///
    /// Note that in some cases, the reference genome will not exactly match the
    /// observed mRNA transcript e.g. due to variance in the source genome from
    /// reference. In these cases,
    /// \[exon.frame][google.genomics.v1.Transcript.Exon.frame\] will not necessarily
    /// match the expected reference reading frame and coding exon reference bases
    /// cannot necessarily be concatenated to produce the original transcript mRNA.
    #[prost(message, optional, tag="3")]
    pub coding_sequence: ::core::option::Option<transcript::CodingSequence>,
}
/// Nested message and enum types in `Transcript`.
pub mod transcript {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Exon {
        /// The start position of the exon on this annotation's reference sequence,
        /// 0-based inclusive. Note that this is relative to the reference start, and
        /// **not** the containing annotation start.
        #[prost(int64, tag="1")]
        pub start: i64,
        /// The end position of the exon on this annotation's reference sequence,
        /// 0-based exclusive. Note that this is relative to the reference start, and
        /// *not* the containing annotation start.
        #[prost(int64, tag="2")]
        pub end: i64,
        /// The frame of this exon. Contains a value of 0, 1, or 2, which indicates
        /// the offset of the first coding base of the exon within the reading frame
        /// of the coding DNA sequence, if any. This field is dependent on the
        /// strandedness of this annotation (see
        /// \[Annotation.reverse_strand][google.genomics.v1.Annotation.reverse_strand\]).
        /// For forward stranded annotations, this offset is relative to the
        /// \[exon.start][google.genomics.v1.Transcript.Exon.start\]. For reverse
        /// strand annotations, this offset is relative to the
        /// \[exon.end][google.genomics.v1.Transcript.Exon.end\] `- 1`.
        ///
        /// Unset if this exon does not intersect the coding sequence. Upon creation
        /// of a transcript, the frame must be populated for all or none of the
        /// coding exons.
        #[prost(message, optional, tag="3")]
        pub frame: ::core::option::Option<i32>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CodingSequence {
        /// The start of the coding sequence on this annotation's reference sequence,
        /// 0-based inclusive. Note that this position is relative to the reference
        /// start, and *not* the containing annotation start.
        #[prost(int64, tag="1")]
        pub start: i64,
        /// The end of the coding sequence on this annotation's reference sequence,
        /// 0-based exclusive. Note that this position is relative to the reference
        /// start, and *not* the containing annotation start.
        #[prost(int64, tag="2")]
        pub end: i64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalId {
    /// The name of the source of this data.
    #[prost(string, tag="1")]
    pub source_name: ::prost::alloc::string::String,
    /// The id used by the source of this data.
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAnnotationSetRequest {
    /// The annotation set to create.
    #[prost(message, optional, tag="1")]
    pub annotation_set: ::core::option::Option<AnnotationSet>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAnnotationSetRequest {
    /// The ID of the annotation set to be retrieved.
    #[prost(string, tag="1")]
    pub annotation_set_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAnnotationSetRequest {
    /// The ID of the annotation set to be updated.
    #[prost(string, tag="1")]
    pub annotation_set_id: ::prost::alloc::string::String,
    /// The new annotation set.
    #[prost(message, optional, tag="2")]
    pub annotation_set: ::core::option::Option<AnnotationSet>,
    /// An optional mask specifying which fields to update. Mutable fields are
    /// \[name][google.genomics.v1.AnnotationSet.name\],
    /// \[source_uri][google.genomics.v1.AnnotationSet.source_uri\], and
    /// \[info][google.genomics.v1.AnnotationSet.info\]. If unspecified, all
    /// mutable fields will be updated.
    #[prost(message, optional, tag="3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAnnotationSetRequest {
    /// The ID of the annotation set to be deleted.
    #[prost(string, tag="1")]
    pub annotation_set_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAnnotationSetsRequest {
    /// Required. The dataset IDs to search within. Caller must have `READ` access
    /// to these datasets.
    #[prost(string, repeated, tag="1")]
    pub dataset_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If specified, only annotation sets associated with the given reference set
    /// are returned.
    #[prost(string, tag="2")]
    pub reference_set_id: ::prost::alloc::string::String,
    /// Only return annotations sets for which a substring of the name matches this
    /// string (case insensitive).
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    /// If specified, only annotation sets that have any of these types are
    /// returned.
    #[prost(enumeration="AnnotationType", repeated, tag="4")]
    pub types: ::prost::alloc::vec::Vec<i32>,
    /// The continuation token, which is used to page through large result sets.
    /// To get the next page of results, set this parameter to the value of
    /// `nextPageToken` from the previous response.
    #[prost(string, tag="5")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single page. If unspecified,
    /// defaults to 128. The maximum value is 1024.
    #[prost(int32, tag="6")]
    pub page_size: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAnnotationSetsResponse {
    /// The matching annotation sets.
    #[prost(message, repeated, tag="1")]
    pub annotation_sets: ::prost::alloc::vec::Vec<AnnotationSet>,
    /// The continuation token, which is used to page through large result sets.
    /// Provide this value in a subsequent request to return the next page of
    /// results. This field will be empty if there aren't any additional results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAnnotationRequest {
    /// The annotation to be created.
    #[prost(message, optional, tag="1")]
    pub annotation: ::core::option::Option<Annotation>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateAnnotationsRequest {
    /// The annotations to be created. At most 4096 can be specified in a single
    /// request.
    #[prost(message, repeated, tag="1")]
    pub annotations: ::prost::alloc::vec::Vec<Annotation>,
    /// A unique request ID which enables the server to detect duplicated requests.
    /// If provided, duplicated requests will result in the same response; if not
    /// provided, duplicated requests may result in duplicated data. For a given
    /// annotation set, callers should not reuse `request_id`s when writing
    /// different batches of annotations - behavior in this case is undefined.
    /// A common approach is to use a UUID. For batch jobs where worker crashes are
    /// a possibility, consider using some unique variant of a worker or run ID.
    #[prost(string, tag="2")]
    pub request_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateAnnotationsResponse {
    /// The resulting per-annotation entries, ordered consistently with the
    /// original request.
    #[prost(message, repeated, tag="1")]
    pub entries: ::prost::alloc::vec::Vec<batch_create_annotations_response::Entry>,
}
/// Nested message and enum types in `BatchCreateAnnotationsResponse`.
pub mod batch_create_annotations_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entry {
        /// The creation status.
        #[prost(message, optional, tag="1")]
        pub status: ::core::option::Option<super::super::super::rpc::Status>,
        /// The created annotation, if creation was successful.
        #[prost(message, optional, tag="2")]
        pub annotation: ::core::option::Option<super::Annotation>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAnnotationRequest {
    /// The ID of the annotation to be retrieved.
    #[prost(string, tag="1")]
    pub annotation_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAnnotationRequest {
    /// The ID of the annotation to be updated.
    #[prost(string, tag="1")]
    pub annotation_id: ::prost::alloc::string::String,
    /// The new annotation.
    #[prost(message, optional, tag="2")]
    pub annotation: ::core::option::Option<Annotation>,
    /// An optional mask specifying which fields to update. Mutable fields are
    /// \[name][google.genomics.v1.Annotation.name\],
    /// \[variant][google.genomics.v1.Annotation.variant\],
    /// \[transcript][google.genomics.v1.Annotation.transcript\], and
    /// \[info][google.genomics.v1.Annotation.info\]. If unspecified, all mutable
    /// fields will be updated.
    #[prost(message, optional, tag="3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAnnotationRequest {
    /// The ID of the annotation to be deleted.
    #[prost(string, tag="1")]
    pub annotation_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAnnotationsRequest {
    /// Required. The annotation sets to search within. The caller must have
    /// `READ` access to these annotation sets.
    /// All queried annotation sets must have the same type.
    #[prost(string, repeated, tag="1")]
    pub annotation_set_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The start position of the range on the reference, 0-based inclusive. If
    /// specified,
    /// \[referenceId][google.genomics.v1.SearchAnnotationsRequest.reference_id\] or
    /// \[referenceName][google.genomics.v1.SearchAnnotationsRequest.reference_name\]
    /// must be specified. Defaults to 0.
    #[prost(int64, tag="4")]
    pub start: i64,
    /// The end position of the range on the reference, 0-based exclusive. If
    /// \[referenceId][google.genomics.v1.SearchAnnotationsRequest.reference_id\] or
    /// \[referenceName][google.genomics.v1.SearchAnnotationsRequest.reference_name\]
    /// must be specified, Defaults to the length of the reference.
    #[prost(int64, tag="5")]
    pub end: i64,
    /// The continuation token, which is used to page through large result sets.
    /// To get the next page of results, set this parameter to the value of
    /// `nextPageToken` from the previous response.
    #[prost(string, tag="6")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single page. If unspecified,
    /// defaults to 256. The maximum value is 2048.
    #[prost(int32, tag="7")]
    pub page_size: i32,
    /// Required. `reference_id` or `reference_name` must be set.
    #[prost(oneof="search_annotations_request::Reference", tags="2, 3")]
    pub reference: ::core::option::Option<search_annotations_request::Reference>,
}
/// Nested message and enum types in `SearchAnnotationsRequest`.
pub mod search_annotations_request {
    /// Required. `reference_id` or `reference_name` must be set.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Reference {
        /// The ID of the reference to query.
        #[prost(string, tag="2")]
        ReferenceId(::prost::alloc::string::String),
        /// The name of the reference to query, within the reference set associated
        /// with this query.
        #[prost(string, tag="3")]
        ReferenceName(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAnnotationsResponse {
    /// The matching annotations.
    #[prost(message, repeated, tag="1")]
    pub annotations: ::prost::alloc::vec::Vec<Annotation>,
    /// The continuation token, which is used to page through large result sets.
    /// Provide this value in a subsequent request to return the next page of
    /// results. This field will be empty if there aren't any additional results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// When an \[Annotation][google.genomics.v1.Annotation\] or
/// \[AnnotationSet][google.genomics.v1.AnnotationSet\] is created, if `type` is
/// not specified it will be set to `GENERIC`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AnnotationType {
    Unspecified = 0,
    /// A `GENERIC` annotation type should be used when no other annotation
    /// type will suffice. This represents an untyped annotation of the reference
    /// genome.
    Generic = 1,
    /// A `VARIANT` annotation type.
    Variant = 2,
    /// A `GENE` annotation type represents the existence of a gene at the
    /// associated reference coordinates. The start coordinate is typically the
    /// gene's transcription start site and the end is typically the end of the
    /// gene's last exon.
    Gene = 3,
    /// A `TRANSCRIPT` annotation type represents the assertion that a
    /// particular region of the reference genome may be transcribed as RNA.
    Transcript = 4,
}
/// Generated client implementations.
pub mod annotation_service_v1_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// This service provides storage and positional retrieval of genomic
    /// reference annotations, including variant annotations.
    #[derive(Debug, Clone)]
    pub struct AnnotationServiceV1Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AnnotationServiceV1Client<T>
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
        ) -> AnnotationServiceV1Client<InterceptedService<T, F>>
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
            AnnotationServiceV1Client::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a new annotation set. Caller must have WRITE permission for the
        /// associated dataset.
        ///
        /// The following fields are required:
        ///
        ///   * [datasetId][google.genomics.v1.AnnotationSet.dataset_id]
        ///   * [referenceSetId][google.genomics.v1.AnnotationSet.reference_set_id]
        ///
        /// All other fields may be optionally specified, unless documented as being
        /// server-generated (for example, the `id` field).
        pub async fn create_annotation_set(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAnnotationSetRequest>,
        ) -> Result<tonic::Response<super::AnnotationSet>, tonic::Status> {
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
                "/google.genomics.v1.AnnotationServiceV1/CreateAnnotationSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets an annotation set. Caller must have READ permission for
        /// the associated dataset.
        pub async fn get_annotation_set(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAnnotationSetRequest>,
        ) -> Result<tonic::Response<super::AnnotationSet>, tonic::Status> {
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
                "/google.genomics.v1.AnnotationServiceV1/GetAnnotationSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an annotation set. The update must respect all mutability
        /// restrictions and other invariants described on the annotation set resource.
        /// Caller must have WRITE permission for the associated dataset.
        pub async fn update_annotation_set(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAnnotationSetRequest>,
        ) -> Result<tonic::Response<super::AnnotationSet>, tonic::Status> {
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
                "/google.genomics.v1.AnnotationServiceV1/UpdateAnnotationSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes an annotation set. Caller must have WRITE permission
        /// for the associated annotation set.
        pub async fn delete_annotation_set(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAnnotationSetRequest>,
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
                "/google.genomics.v1.AnnotationServiceV1/DeleteAnnotationSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Searches for annotation sets that match the given criteria. Annotation sets
        /// are returned in an unspecified order. This order is consistent, such that
        /// two queries for the same content (regardless of page size) yield annotation
        /// sets in the same order across their respective streams of paginated
        /// responses. Caller must have READ permission for the queried datasets.
        pub async fn search_annotation_sets(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchAnnotationSetsRequest>,
        ) -> Result<
            tonic::Response<super::SearchAnnotationSetsResponse>,
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
                "/google.genomics.v1.AnnotationServiceV1/SearchAnnotationSets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new annotation. Caller must have WRITE permission
        /// for the associated annotation set.
        ///
        /// The following fields are required:
        ///
        /// * [annotationSetId][google.genomics.v1.Annotation.annotation_set_id]
        /// * [referenceName][google.genomics.v1.Annotation.reference_name] or
        ///   [referenceId][google.genomics.v1.Annotation.reference_id]
        ///
        /// ### Transcripts
        ///
        /// For annotations of type TRANSCRIPT, the following fields of
        /// [transcript][google.genomics.v1.Annotation.transcript] must be provided:
        ///
        /// * [exons.start][google.genomics.v1.Transcript.Exon.start]
        /// * [exons.end][google.genomics.v1.Transcript.Exon.end]
        ///
        /// All other fields may be optionally specified, unless documented as being
        /// server-generated (for example, the `id` field). The annotated
        /// range must be no longer than 100Mbp (mega base pairs). See the
        /// [Annotation resource][google.genomics.v1.Annotation]
        /// for additional restrictions on each field.
        pub async fn create_annotation(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAnnotationRequest>,
        ) -> Result<tonic::Response<super::Annotation>, tonic::Status> {
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
                "/google.genomics.v1.AnnotationServiceV1/CreateAnnotation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates one or more new annotations atomically. All annotations must
        /// belong to the same annotation set. Caller must have WRITE
        /// permission for this annotation set. For optimal performance, batch
        /// positionally adjacent annotations together.
        ///
        /// If the request has a systemic issue, such as an attempt to write to
        /// an inaccessible annotation set, the entire RPC will fail accordingly. For
        /// lesser data issues, when possible an error will be isolated to the
        /// corresponding batch entry in the response; the remaining well formed
        /// annotations will be created normally.
        ///
        /// For details on the requirements for each individual annotation resource,
        /// see
        /// [CreateAnnotation][google.genomics.v1.AnnotationServiceV1.CreateAnnotation].
        pub async fn batch_create_annotations(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCreateAnnotationsRequest>,
        ) -> Result<
            tonic::Response<super::BatchCreateAnnotationsResponse>,
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
                "/google.genomics.v1.AnnotationServiceV1/BatchCreateAnnotations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets an annotation. Caller must have READ permission
        /// for the associated annotation set.
        pub async fn get_annotation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAnnotationRequest>,
        ) -> Result<tonic::Response<super::Annotation>, tonic::Status> {
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
                "/google.genomics.v1.AnnotationServiceV1/GetAnnotation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an annotation. Caller must have
        /// WRITE permission for the associated dataset.
        pub async fn update_annotation(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAnnotationRequest>,
        ) -> Result<tonic::Response<super::Annotation>, tonic::Status> {
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
                "/google.genomics.v1.AnnotationServiceV1/UpdateAnnotation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes an annotation. Caller must have WRITE permission for
        /// the associated annotation set.
        pub async fn delete_annotation(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAnnotationRequest>,
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
                "/google.genomics.v1.AnnotationServiceV1/DeleteAnnotation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Searches for annotations that match the given criteria. Results are
        /// ordered by genomic coordinate (by reference sequence, then position).
        /// Annotations with equivalent genomic coordinates are returned in an
        /// unspecified order. This order is consistent, such that two queries for the
        /// same content (regardless of page size) yield annotations in the same order
        /// across their respective streams of paginated responses. Caller must have
        /// READ permission for the queried annotation sets.
        pub async fn search_annotations(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchAnnotationsRequest>,
        ) -> Result<tonic::Response<super::SearchAnnotationsResponse>, tonic::Status> {
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
                "/google.genomics.v1.AnnotationServiceV1/SearchAnnotations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// A read group is all the data that's processed the same way by the sequencer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadGroup {
    /// The server-generated read group ID, unique for all read groups.
    /// Note: This is different than the @RG ID field in the SAM spec. For that
    /// value, see \[name][google.genomics.v1.ReadGroup.name\].
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The dataset to which this read group belongs.
    #[prost(string, tag="2")]
    pub dataset_id: ::prost::alloc::string::String,
    /// The read group name. This corresponds to the @RG ID field in the SAM spec.
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    /// A free-form text description of this read group.
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
    /// A client-supplied sample identifier for the reads in this read group.
    #[prost(string, tag="5")]
    pub sample_id: ::prost::alloc::string::String,
    /// The experiment used to generate this read group.
    #[prost(message, optional, tag="6")]
    pub experiment: ::core::option::Option<read_group::Experiment>,
    /// The predicted insert size of this read group. The insert size is the length
    /// the sequenced DNA fragment from end-to-end, not including the adapters.
    #[prost(int32, tag="7")]
    pub predicted_insert_size: i32,
    /// The programs used to generate this read group. Programs are always
    /// identical for all read groups within a read group set. For this reason,
    /// only the first read group in a returned set will have this field
    /// populated.
    #[prost(message, repeated, tag="10")]
    pub programs: ::prost::alloc::vec::Vec<read_group::Program>,
    /// The reference set the reads in this read group are aligned to.
    #[prost(string, tag="11")]
    pub reference_set_id: ::prost::alloc::string::String,
    /// A map of additional read group information. This must be of the form
    /// map<string, string[]> (string key mapping to a list of string values).
    #[prost(btree_map="string, message", tag="12")]
    pub info: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost_types::ListValue>,
}
/// Nested message and enum types in `ReadGroup`.
pub mod read_group {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Experiment {
        /// A client-supplied library identifier; a library is a collection of DNA
        /// fragments which have been prepared for sequencing from a sample. This
        /// field is important for quality control as error or bias can be introduced
        /// during sample preparation.
        #[prost(string, tag="1")]
        pub library_id: ::prost::alloc::string::String,
        /// The platform unit used as part of this experiment, for example
        /// flowcell-barcode.lane for Illumina or slide for SOLiD. Corresponds to the
        /// @RG PU field in the SAM spec.
        #[prost(string, tag="2")]
        pub platform_unit: ::prost::alloc::string::String,
        /// The sequencing center used as part of this experiment.
        #[prost(string, tag="3")]
        pub sequencing_center: ::prost::alloc::string::String,
        /// The instrument model used as part of this experiment. This maps to
        /// sequencing technology in the SAM spec.
        #[prost(string, tag="4")]
        pub instrument_model: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Program {
        /// The command line used to run this program.
        #[prost(string, tag="1")]
        pub command_line: ::prost::alloc::string::String,
        /// The user specified locally unique ID of the program. Used along with
        /// `prevProgramId` to define an ordering between programs.
        #[prost(string, tag="2")]
        pub id: ::prost::alloc::string::String,
        /// The display name of the program. This is typically the colloquial name of
        /// the tool used, for example 'bwa' or 'picard'.
        #[prost(string, tag="3")]
        pub name: ::prost::alloc::string::String,
        /// The ID of the program run before this one.
        #[prost(string, tag="4")]
        pub prev_program_id: ::prost::alloc::string::String,
        /// The version of the program run.
        #[prost(string, tag="5")]
        pub version: ::prost::alloc::string::String,
    }
}
/// A reference is a canonical assembled DNA sequence, intended to act as a
/// reference coordinate space for other genomic annotations. A single reference
/// might represent the human chromosome 1 or mitochandrial DNA, for instance. A
/// reference belongs to one or more reference sets.
///
/// For more genomics resource definitions, see [Fundamentals of Google
/// Genomics](<https://cloud.google.com/genomics/fundamentals-of-google-genomics>)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reference {
    /// The server-generated reference ID, unique across all references.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The length of this reference's sequence.
    #[prost(int64, tag="2")]
    pub length: i64,
    /// MD5 of the upper-case sequence excluding all whitespace characters (this
    /// is equivalent to SQ:M5 in SAM). This value is represented in lower case
    /// hexadecimal format.
    #[prost(string, tag="3")]
    pub md5checksum: ::prost::alloc::string::String,
    /// The name of this reference, for example `22`.
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    /// The URI from which the sequence was obtained. Typically specifies a FASTA
    /// format file.
    #[prost(string, tag="5")]
    pub source_uri: ::prost::alloc::string::String,
    /// All known corresponding accession IDs in INSDC (GenBank/ENA/DDBJ) ideally
    /// with a version number, for example `GCF_000001405.26`.
    #[prost(string, repeated, tag="6")]
    pub source_accessions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// ID from <http://www.ncbi.nlm.nih.gov/taxonomy.> For example, 9606 for human.
    #[prost(int32, tag="7")]
    pub ncbi_taxon_id: i32,
}
/// A reference set is a set of references which typically comprise a reference
/// assembly for a species, such as `GRCh38` which is representative
/// of the human genome. A reference set defines a common coordinate space for
/// comparing reference-aligned experimental data. A reference set contains 1 or
/// more references.
///
/// For more genomics resource definitions, see [Fundamentals of Google
/// Genomics](<https://cloud.google.com/genomics/fundamentals-of-google-genomics>)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReferenceSet {
    /// The server-generated reference set ID, unique across all reference sets.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The IDs of the reference objects that are part of this set.
    /// `Reference.md5checksum` must be unique within this set.
    #[prost(string, repeated, tag="2")]
    pub reference_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Order-independent MD5 checksum which identifies this reference set. The
    /// checksum is computed by sorting all lower case hexidecimal string
    /// `reference.md5checksum` (for all reference in this set) in
    /// ascending lexicographic order, concatenating, and taking the MD5 of that
    /// value. The resulting value is represented in lower case hexadecimal format.
    #[prost(string, tag="3")]
    pub md5checksum: ::prost::alloc::string::String,
    /// ID from <http://www.ncbi.nlm.nih.gov/taxonomy> (for example, 9606 for human)
    /// indicating the species which this reference set is intended to model. Note
    /// that contained references may specify a different `ncbiTaxonId`, as
    /// assemblies may contain reference sequences which do not belong to the
    /// modeled species, for example EBV in a human reference genome.
    #[prost(int32, tag="4")]
    pub ncbi_taxon_id: i32,
    /// Free text description of this reference set.
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
    /// Public id of this reference set, such as `GRCh37`.
    #[prost(string, tag="6")]
    pub assembly_id: ::prost::alloc::string::String,
    /// The URI from which the references were obtained.
    #[prost(string, tag="7")]
    pub source_uri: ::prost::alloc::string::String,
    /// All known corresponding accession IDs in INSDC (GenBank/ENA/DDBJ) ideally
    /// with a version number, for example `NC_000001.11`.
    #[prost(string, repeated, tag="8")]
    pub source_accessions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchReferenceSetsRequest {
    /// If present, return reference sets for which the
    /// \[md5checksum][google.genomics.v1.ReferenceSet.md5checksum\] matches exactly.
    #[prost(string, repeated, tag="1")]
    pub md5checksums: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If present, return reference sets for which a prefix of any of
    /// \[sourceAccessions][google.genomics.v1.ReferenceSet.source_accessions\]
    /// match any of these strings. Accession numbers typically have a main number
    /// and a version, for example `NC_000001.11`.
    #[prost(string, repeated, tag="2")]
    pub accessions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If present, return reference sets for which a substring of their
    /// `assemblyId` matches this string (case insensitive).
    #[prost(string, tag="3")]
    pub assembly_id: ::prost::alloc::string::String,
    /// The continuation token, which is used to page through large result sets.
    /// To get the next page of results, set this parameter to the value of
    /// `nextPageToken` from the previous response.
    #[prost(string, tag="4")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single page. If unspecified,
    /// defaults to 1024. The maximum value is 4096.
    #[prost(int32, tag="5")]
    pub page_size: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchReferenceSetsResponse {
    /// The matching references sets.
    #[prost(message, repeated, tag="1")]
    pub reference_sets: ::prost::alloc::vec::Vec<ReferenceSet>,
    /// The continuation token, which is used to page through large result sets.
    /// Provide this value in a subsequent request to return the next page of
    /// results. This field will be empty if there aren't any additional results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReferenceSetRequest {
    /// The ID of the reference set.
    #[prost(string, tag="1")]
    pub reference_set_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchReferencesRequest {
    /// If present, return references for which the
    /// \[md5checksum][google.genomics.v1.Reference.md5checksum\] matches exactly.
    #[prost(string, repeated, tag="1")]
    pub md5checksums: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If present, return references for which a prefix of any of
    /// \[sourceAccessions][google.genomics.v1.Reference.source_accessions\] match
    /// any of these strings. Accession numbers typically have a main number and a
    /// version, for example `GCF_000001405.26`.
    #[prost(string, repeated, tag="2")]
    pub accessions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If present, return only references which belong to this reference set.
    #[prost(string, tag="3")]
    pub reference_set_id: ::prost::alloc::string::String,
    /// The continuation token, which is used to page through large result sets.
    /// To get the next page of results, set this parameter to the value of
    /// `nextPageToken` from the previous response.
    #[prost(string, tag="4")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single page. If unspecified,
    /// defaults to 1024. The maximum value is 4096.
    #[prost(int32, tag="5")]
    pub page_size: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchReferencesResponse {
    /// The matching references.
    #[prost(message, repeated, tag="1")]
    pub references: ::prost::alloc::vec::Vec<Reference>,
    /// The continuation token, which is used to page through large result sets.
    /// Provide this value in a subsequent request to return the next page of
    /// results. This field will be empty if there aren't any additional results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReferenceRequest {
    /// The ID of the reference.
    #[prost(string, tag="1")]
    pub reference_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBasesRequest {
    /// The ID of the reference.
    #[prost(string, tag="1")]
    pub reference_id: ::prost::alloc::string::String,
    /// The start position (0-based) of this query. Defaults to 0.
    #[prost(int64, tag="2")]
    pub start: i64,
    /// The end position (0-based, exclusive) of this query. Defaults to the length
    /// of this reference.
    #[prost(int64, tag="3")]
    pub end: i64,
    /// The continuation token, which is used to page through large result sets.
    /// To get the next page of results, set this parameter to the value of
    /// `nextPageToken` from the previous response.
    #[prost(string, tag="4")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of bases to return in a single page. If unspecified,
    /// defaults to 200Kbp (kilo base pairs). The maximum value is 10Mbp (mega base
    /// pairs).
    #[prost(int32, tag="5")]
    pub page_size: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBasesResponse {
    /// The offset position (0-based) of the given `sequence` from the
    /// start of this `Reference`. This value will differ for each page
    /// in a paginated request.
    #[prost(int64, tag="1")]
    pub offset: i64,
    /// A substring of the bases that make up this reference.
    #[prost(string, tag="2")]
    pub sequence: ::prost::alloc::string::String,
    /// The continuation token, which is used to page through large result sets.
    /// Provide this value in a subsequent request to return the next page of
    /// results. This field will be empty if there aren't any additional results.
    #[prost(string, tag="3")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod reference_service_v1_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct ReferenceServiceV1Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ReferenceServiceV1Client<T>
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
        ) -> ReferenceServiceV1Client<InterceptedService<T, F>>
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
            ReferenceServiceV1Client::new(InterceptedService::new(inner, interceptor))
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
        /// Searches for reference sets which match the given criteria.
        ///
        /// For the definitions of references and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        ///
        /// Implements
        /// [GlobalAllianceApi.searchReferenceSets](https://github.com/ga4gh/schemas/blob/v0.5.1/src/main/resources/avro/referencemethods.avdl#L71)
        pub async fn search_reference_sets(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchReferenceSetsRequest>,
        ) -> Result<tonic::Response<super::SearchReferenceSetsResponse>, tonic::Status> {
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
                "/google.genomics.v1.ReferenceServiceV1/SearchReferenceSets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a reference set.
        ///
        /// For the definitions of references and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        ///
        /// Implements
        /// [GlobalAllianceApi.getReferenceSet](https://github.com/ga4gh/schemas/blob/v0.5.1/src/main/resources/avro/referencemethods.avdl#L83).
        pub async fn get_reference_set(
            &mut self,
            request: impl tonic::IntoRequest<super::GetReferenceSetRequest>,
        ) -> Result<tonic::Response<super::ReferenceSet>, tonic::Status> {
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
                "/google.genomics.v1.ReferenceServiceV1/GetReferenceSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Searches for references which match the given criteria.
        ///
        /// For the definitions of references and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        ///
        /// Implements
        /// [GlobalAllianceApi.searchReferences](https://github.com/ga4gh/schemas/blob/v0.5.1/src/main/resources/avro/referencemethods.avdl#L146).
        pub async fn search_references(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchReferencesRequest>,
        ) -> Result<tonic::Response<super::SearchReferencesResponse>, tonic::Status> {
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
                "/google.genomics.v1.ReferenceServiceV1/SearchReferences",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a reference.
        ///
        /// For the definitions of references and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        ///
        /// Implements
        /// [GlobalAllianceApi.getReference](https://github.com/ga4gh/schemas/blob/v0.5.1/src/main/resources/avro/referencemethods.avdl#L158).
        pub async fn get_reference(
            &mut self,
            request: impl tonic::IntoRequest<super::GetReferenceRequest>,
        ) -> Result<tonic::Response<super::Reference>, tonic::Status> {
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
                "/google.genomics.v1.ReferenceServiceV1/GetReference",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists the bases in a reference, optionally restricted to a range.
        ///
        /// For the definitions of references and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        ///
        /// Implements
        /// [GlobalAllianceApi.getReferenceBases](https://github.com/ga4gh/schemas/blob/v0.5.1/src/main/resources/avro/referencemethods.avdl#L221).
        pub async fn list_bases(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBasesRequest>,
        ) -> Result<tonic::Response<super::ListBasesResponse>, tonic::Status> {
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
                "/google.genomics.v1.ReferenceServiceV1/ListBases",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// A read group set is a logical collection of read groups, which are
/// collections of reads produced by a sequencer. A read group set typically
/// models reads corresponding to one sample, sequenced one way, and aligned one
/// way.
///
/// * A read group set belongs to one dataset.
/// * A read group belongs to one read group set.
/// * A read belongs to one read group.
///
/// For more genomics resource definitions, see [Fundamentals of Google
/// Genomics](<https://cloud.google.com/genomics/fundamentals-of-google-genomics>)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadGroupSet {
    /// The server-generated read group set ID, unique for all read group sets.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The dataset to which this read group set belongs.
    #[prost(string, tag="2")]
    pub dataset_id: ::prost::alloc::string::String,
    /// The reference set to which the reads in this read group set are aligned.
    #[prost(string, tag="3")]
    pub reference_set_id: ::prost::alloc::string::String,
    /// The read group set name. By default this will be initialized to the sample
    /// name of the sequenced data contained in this set.
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    /// The filename of the original source file for this read group set, if any.
    #[prost(string, tag="5")]
    pub filename: ::prost::alloc::string::String,
    /// The read groups in this set. There are typically 1-10 read groups in a read
    /// group set.
    #[prost(message, repeated, tag="6")]
    pub read_groups: ::prost::alloc::vec::Vec<ReadGroup>,
    /// A map of additional read group set information.
    #[prost(btree_map="string, message", tag="7")]
    pub info: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost_types::ListValue>,
}
/// A Dataset is a collection of genomic data.
///
/// For more genomics resource definitions, see [Fundamentals of Google
/// Genomics](<https://cloud.google.com/genomics/fundamentals-of-google-genomics>)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dataset {
    /// The server-generated dataset ID, unique across all datasets.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The Google Cloud project ID that this dataset belongs to.
    #[prost(string, tag="2")]
    pub project_id: ::prost::alloc::string::String,
    /// The dataset name.
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    /// The time this dataset was created, in seconds from the epoch.
    #[prost(message, optional, tag="4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// The dataset list request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatasetsRequest {
    /// Required. The Google Cloud project ID to list datasets for.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single page. If unspecified,
    /// defaults to 50. The maximum value is 1024.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// The continuation token, which is used to page through large result sets.
    /// To get the next page of results, set this parameter to the value of
    /// `nextPageToken` from the previous response.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The dataset list response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatasetsResponse {
    /// The list of matching Datasets.
    #[prost(message, repeated, tag="1")]
    pub datasets: ::prost::alloc::vec::Vec<Dataset>,
    /// The continuation token, which is used to page through large result sets.
    /// Provide this value in a subsequent request to return the next page of
    /// results. This field will be empty if there aren't any additional results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatasetRequest {
    /// The dataset to be created. Must contain projectId and name.
    #[prost(message, optional, tag="1")]
    pub dataset: ::core::option::Option<Dataset>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDatasetRequest {
    /// The ID of the dataset to be updated.
    #[prost(string, tag="1")]
    pub dataset_id: ::prost::alloc::string::String,
    /// The new dataset data.
    #[prost(message, optional, tag="2")]
    pub dataset: ::core::option::Option<Dataset>,
    /// An optional mask specifying which fields to update. At this time, the only
    /// mutable field is \[name][google.genomics.v1.Dataset.name\]. The only
    /// acceptable value is "name". If unspecified, all mutable fields will be
    /// updated.
    #[prost(message, optional, tag="3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDatasetRequest {
    /// The ID of the dataset to be deleted.
    #[prost(string, tag="1")]
    pub dataset_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeleteDatasetRequest {
    /// The ID of the dataset to be undeleted.
    #[prost(string, tag="1")]
    pub dataset_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetRequest {
    /// The ID of the dataset.
    #[prost(string, tag="1")]
    pub dataset_id: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod dataset_service_v1_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// This service manages datasets, which are collections of genomic data.
    #[derive(Debug, Clone)]
    pub struct DatasetServiceV1Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DatasetServiceV1Client<T>
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
        ) -> DatasetServiceV1Client<InterceptedService<T, F>>
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
            DatasetServiceV1Client::new(InterceptedService::new(inner, interceptor))
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
        /// Lists datasets within a project.
        ///
        /// For the definitions of datasets and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        pub async fn list_datasets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDatasetsRequest>,
        ) -> Result<tonic::Response<super::ListDatasetsResponse>, tonic::Status> {
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
                "/google.genomics.v1.DatasetServiceV1/ListDatasets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new dataset.
        ///
        /// For the definitions of datasets and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        pub async fn create_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDatasetRequest>,
        ) -> Result<tonic::Response<super::Dataset>, tonic::Status> {
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
                "/google.genomics.v1.DatasetServiceV1/CreateDataset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a dataset by ID.
        ///
        /// For the definitions of datasets and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        pub async fn get_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatasetRequest>,
        ) -> Result<tonic::Response<super::Dataset>, tonic::Status> {
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
                "/google.genomics.v1.DatasetServiceV1/GetDataset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a dataset.
        ///
        /// For the definitions of datasets and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        ///
        /// This method supports patch semantics.
        pub async fn update_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDatasetRequest>,
        ) -> Result<tonic::Response<super::Dataset>, tonic::Status> {
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
                "/google.genomics.v1.DatasetServiceV1/UpdateDataset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a dataset and all of its contents (all read group sets,
        /// reference sets, variant sets, call sets, annotation sets, etc.)
        /// This is reversible (up to one week after the deletion) via
        /// the
        /// [datasets.undelete][google.genomics.v1.DatasetServiceV1.UndeleteDataset]
        /// operation.
        ///
        /// For the definitions of datasets and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        pub async fn delete_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDatasetRequest>,
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
                "/google.genomics.v1.DatasetServiceV1/DeleteDataset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Undeletes a dataset by restoring a dataset which was deleted via this API.
        ///
        /// For the definitions of datasets and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        ///
        /// This operation is only possible for a week after the deletion occurred.
        pub async fn undelete_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::UndeleteDatasetRequest>,
        ) -> Result<tonic::Response<super::Dataset>, tonic::Status> {
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
                "/google.genomics.v1.DatasetServiceV1/UndeleteDataset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the access control policy on the specified dataset. Replaces any
        /// existing policy.
        ///
        /// For the definitions of datasets and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        ///
        /// See <a href="/iam/docs/managing-policies#setting_a_policy">Setting a
        /// Policy</a> for more information.
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::iam::v1::Policy>,
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
                "/google.genomics.v1.DatasetServiceV1/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the access control policy for the dataset. This is empty if the
        /// policy or resource does not exist.
        ///
        /// See <a href="/iam/docs/managing-policies#getting_a_policy">Getting a
        /// Policy</a> for more information.
        ///
        /// For the definitions of datasets and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::iam::v1::Policy>,
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
                "/google.genomics.v1.DatasetServiceV1/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns permissions that a caller has on the specified resource.
        /// See <a href="/iam/docs/managing-policies#testing_permissions">Testing
        /// Permissions</a> for more information.
        ///
        /// For the definitions of datasets and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::iam::v1::TestIamPermissionsResponse>,
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
                "/google.genomics.v1.DatasetServiceV1/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// An abstraction for referring to a genomic position, in relation to some
/// already known reference. For now, represents a genomic position as a
/// reference name, a base number on that reference (0-based), and a
/// determination of forward or reverse strand.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Position {
    /// The name of the reference in whatever reference set is being used.
    #[prost(string, tag="1")]
    pub reference_name: ::prost::alloc::string::String,
    /// The 0-based offset from the start of the forward strand for that reference.
    #[prost(int64, tag="2")]
    pub position: i64,
    /// Whether this position is on the reverse strand, as opposed to the forward
    /// strand.
    #[prost(bool, tag="3")]
    pub reverse_strand: bool,
}
/// A linear alignment can be represented by one CIGAR string. Describes the
/// mapped position and local alignment of the read to the reference.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinearAlignment {
    /// The position of this alignment.
    #[prost(message, optional, tag="1")]
    pub position: ::core::option::Option<Position>,
    /// The mapping quality of this alignment. Represents how likely
    /// the read maps to this position as opposed to other locations.
    ///
    /// Specifically, this is -10 log10 Pr(mapping position is wrong), rounded to
    /// the nearest integer.
    #[prost(int32, tag="2")]
    pub mapping_quality: i32,
    /// Represents the local alignment of this sequence (alignment matches, indels,
    /// etc) against the reference.
    #[prost(message, repeated, tag="3")]
    pub cigar: ::prost::alloc::vec::Vec<CigarUnit>,
}
/// A read alignment describes a linear alignment of a string of DNA to a
/// [reference sequence]\[google.genomics.v1.Reference\], in addition to metadata
/// about the fragment (the molecule of DNA sequenced) and the read (the bases
/// which were read by the sequencer). A read is equivalent to a line in a SAM
/// file. A read belongs to exactly one read group and exactly one
/// [read group set]\[google.genomics.v1.ReadGroupSet\].
///
/// For more genomics resource definitions, see [Fundamentals of Google
/// Genomics](<https://cloud.google.com/genomics/fundamentals-of-google-genomics>)
///
/// ### Reverse-stranded reads
///
/// Mapped reads (reads having a non-null `alignment`) can be aligned to either
/// the forward or the reverse strand of their associated reference. Strandedness
/// of a mapped read is encoded by `alignment.position.reverseStrand`.
///
/// If we consider the reference to be a forward-stranded coordinate space of
/// `[0, reference.length)` with `0` as the left-most position and
/// `reference.length` as the right-most position, reads are always aligned left
/// to right. That is, `alignment.position.position` always refers to the
/// left-most reference coordinate and `alignment.cigar` describes the alignment
/// of this read to the reference from left to right. All per-base fields such as
/// `alignedSequence` and `alignedQuality` share this same left-to-right
/// orientation; this is true of reads which are aligned to either strand. For
/// reverse-stranded reads, this means that `alignedSequence` is the reverse
/// complement of the bases that were originally reported by the sequencing
/// machine.
///
/// ### Generating a reference-aligned sequence string
///
/// When interacting with mapped reads, it's often useful to produce a string
/// representing the local alignment of the read to reference. The following
/// pseudocode demonstrates one way of doing this:
///
///     out = ""
///     offset = 0
///     for c in read.alignment.cigar {
///       switch c.operation {
///       case "ALIGNMENT_MATCH", "SEQUENCE_MATCH", "SEQUENCE_MISMATCH":
///         out += read.alignedSequence\[offset:offset+c.operationLength\]
///         offset += c.operationLength
///         break
///       case "CLIP_SOFT", "INSERT":
///         offset += c.operationLength
///         break
///       case "PAD":
///         out += repeat("*", c.operationLength)
///         break
///       case "DELETE":
///         out += repeat("-", c.operationLength)
///         break
///       case "SKIP":
///         out += repeat(" ", c.operationLength)
///         break
///       case "CLIP_HARD":
///         break
///       }
///     }
///     return out
///
/// ### Converting to SAM's CIGAR string
///
/// The following pseudocode generates a SAM CIGAR string from the
/// `cigar` field. Note that this is a lossy conversion
/// (`cigar.referenceSequence` is lost).
///
///     cigarMap = {
///       "ALIGNMENT_MATCH": "M",
///       "INSERT": "I",
///       "DELETE": "D",
///       "SKIP": "N",
///       "CLIP_SOFT": "S",
///       "CLIP_HARD": "H",
///       "PAD": "P",
///       "SEQUENCE_MATCH": "=",
///       "SEQUENCE_MISMATCH": "X",
///     }
///     cigarStr = ""
///     for c in read.alignment.cigar {
///       cigarStr += c.operationLength + cigarMap\[c.operation\]
///     }
///     return cigarStr
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Read {
    /// The server-generated read ID, unique across all reads. This is different
    /// from the `fragmentName`.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The ID of the read group this read belongs to. A read belongs to exactly
    /// one read group. This is a server-generated ID which is distinct from SAM's
    /// RG tag (for that value, see
    /// \[ReadGroup.name][google.genomics.v1.ReadGroup.name\]).
    #[prost(string, tag="2")]
    pub read_group_id: ::prost::alloc::string::String,
    /// The ID of the read group set this read belongs to. A read belongs to
    /// exactly one read group set.
    #[prost(string, tag="3")]
    pub read_group_set_id: ::prost::alloc::string::String,
    /// The fragment name. Equivalent to QNAME (query template name) in SAM.
    #[prost(string, tag="4")]
    pub fragment_name: ::prost::alloc::string::String,
    /// The orientation and the distance between reads from the fragment are
    /// consistent with the sequencing protocol (SAM flag 0x2).
    #[prost(bool, tag="5")]
    pub proper_placement: bool,
    /// The fragment is a PCR or optical duplicate (SAM flag 0x400).
    #[prost(bool, tag="6")]
    pub duplicate_fragment: bool,
    /// The observed length of the fragment, equivalent to TLEN in SAM.
    #[prost(int32, tag="7")]
    pub fragment_length: i32,
    /// The read number in sequencing. 0-based and less than numberReads. This
    /// field replaces SAM flag 0x40 and 0x80.
    #[prost(int32, tag="8")]
    pub read_number: i32,
    /// The number of reads in the fragment (extension to SAM flag 0x1).
    #[prost(int32, tag="9")]
    pub number_reads: i32,
    /// Whether this read did not pass filters, such as platform or vendor quality
    /// controls (SAM flag 0x200).
    #[prost(bool, tag="10")]
    pub failed_vendor_quality_checks: bool,
    /// The linear alignment for this alignment record. This field is null for
    /// unmapped reads.
    #[prost(message, optional, tag="11")]
    pub alignment: ::core::option::Option<LinearAlignment>,
    /// Whether this alignment is secondary. Equivalent to SAM flag 0x100.
    /// A secondary alignment represents an alternative to the primary alignment
    /// for this read. Aligners may return secondary alignments if a read can map
    /// ambiguously to multiple coordinates in the genome. By convention, each read
    /// has one and only one alignment where both `secondaryAlignment`
    /// and `supplementaryAlignment` are false.
    #[prost(bool, tag="12")]
    pub secondary_alignment: bool,
    /// Whether this alignment is supplementary. Equivalent to SAM flag 0x800.
    /// Supplementary alignments are used in the representation of a chimeric
    /// alignment. In a chimeric alignment, a read is split into multiple
    /// linear alignments that map to different reference contigs. The first
    /// linear alignment in the read will be designated as the representative
    /// alignment; the remaining linear alignments will be designated as
    /// supplementary alignments. These alignments may have different mapping
    /// quality scores. In each linear alignment in a chimeric alignment, the read
    /// will be hard clipped. The `alignedSequence` and
    /// `alignedQuality` fields in the alignment record will only
    /// represent the bases for its respective linear alignment.
    #[prost(bool, tag="13")]
    pub supplementary_alignment: bool,
    /// The bases of the read sequence contained in this alignment record,
    /// **without CIGAR operations applied** (equivalent to SEQ in SAM).
    /// `alignedSequence` and `alignedQuality` may be
    /// shorter than the full read sequence and quality. This will occur if the
    /// alignment is part of a chimeric alignment, or if the read was trimmed. When
    /// this occurs, the CIGAR for this read will begin/end with a hard clip
    /// operator that will indicate the length of the excised sequence.
    #[prost(string, tag="14")]
    pub aligned_sequence: ::prost::alloc::string::String,
    /// The quality of the read sequence contained in this alignment record
    /// (equivalent to QUAL in SAM).
    /// `alignedSequence` and `alignedQuality` may be shorter than the full read
    /// sequence and quality. This will occur if the alignment is part of a
    /// chimeric alignment, or if the read was trimmed. When this occurs, the CIGAR
    /// for this read will begin/end with a hard clip operator that will indicate
    /// the length of the excised sequence.
    #[prost(int32, repeated, tag="15")]
    pub aligned_quality: ::prost::alloc::vec::Vec<i32>,
    /// The mapping of the primary alignment of the
    /// `(readNumber+1)%numberReads` read in the fragment. It replaces
    /// mate position and mate strand in SAM.
    #[prost(message, optional, tag="16")]
    pub next_mate_position: ::core::option::Option<Position>,
    /// A map of additional read alignment information. This must be of the form
    /// map<string, string[]> (string key mapping to a list of string values).
    #[prost(btree_map="string, message", tag="17")]
    pub info: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost_types::ListValue>,
}
/// The read group set search request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchReadGroupSetsRequest {
    /// Restricts this query to read group sets within the given datasets. At least
    /// one ID must be provided.
    #[prost(string, repeated, tag="1")]
    pub dataset_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Only return read group sets for which a substring of the name matches this
    /// string.
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    /// The continuation token, which is used to page through large result sets.
    /// To get the next page of results, set this parameter to the value of
    /// `nextPageToken` from the previous response.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single page. If unspecified,
    /// defaults to 256. The maximum value is 1024.
    #[prost(int32, tag="4")]
    pub page_size: i32,
}
/// The read group set search response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchReadGroupSetsResponse {
    /// The list of matching read group sets.
    #[prost(message, repeated, tag="1")]
    pub read_group_sets: ::prost::alloc::vec::Vec<ReadGroupSet>,
    /// The continuation token, which is used to page through large result sets.
    /// Provide this value in a subsequent request to return the next page of
    /// results. This field will be empty if there aren't any additional results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The read group set import request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportReadGroupSetsRequest {
    /// Required. The ID of the dataset these read group sets will belong to. The
    /// caller must have WRITE permissions to this dataset.
    #[prost(string, tag="1")]
    pub dataset_id: ::prost::alloc::string::String,
    /// The reference set to which the imported read group sets are aligned to, if
    /// any. The reference names of this reference set must be a superset of those
    /// found in the imported file headers. If no reference set id is provided, a
    /// best effort is made to associate with a matching reference set.
    #[prost(string, tag="4")]
    pub reference_set_id: ::prost::alloc::string::String,
    /// A list of URIs pointing at [BAM
    /// files](<https://samtools.github.io/hts-specs/SAMv1.pdf>)
    /// in Google Cloud Storage.
    /// Those URIs can include wildcards (*), but do not add or remove
    /// matching files before import has completed.
    ///
    /// Note that Google Cloud Storage object listing is only eventually
    /// consistent: files added may be not be immediately visible to
    /// everyone. Thus, if using a wildcard it is preferable not to start
    /// the import immediately after the files are created.
    #[prost(string, repeated, tag="2")]
    pub source_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The partition strategy describes how read groups are partitioned into read
    /// group sets.
    #[prost(enumeration="import_read_group_sets_request::PartitionStrategy", tag="5")]
    pub partition_strategy: i32,
}
/// Nested message and enum types in `ImportReadGroupSetsRequest`.
pub mod import_read_group_sets_request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PartitionStrategy {
        Unspecified = 0,
        /// In most cases, this strategy yields one read group set per file. This is
        /// the default behavior.
        ///
        /// Allocate one read group set per file per sample. For BAM files, read
        /// groups are considered to share a sample if they have identical sample
        /// names. Furthermore, all reads for each file which do not belong to a read
        /// group, if any, will be grouped into a single read group set per-file.
        PerFilePerSample = 1,
        /// Includes all read groups in all imported files into a single read group
        /// set. Requires that the headers for all imported files are equivalent. All
        /// reads which do not belong to a read group, if any, will be grouped into a
        /// separate read group set.
        MergeAll = 2,
    }
}
/// The read group set import response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportReadGroupSetsResponse {
    /// IDs of the read group sets that were created.
    #[prost(string, repeated, tag="1")]
    pub read_group_set_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The read group set export request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportReadGroupSetRequest {
    /// Required. The Google Cloud project ID that owns this
    /// export. The caller must have WRITE access to this project.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. A Google Cloud Storage URI for the exported BAM file.
    /// The currently authenticated user must have write access to the new file.
    /// An error will be returned if the URI already contains data.
    #[prost(string, tag="2")]
    pub export_uri: ::prost::alloc::string::String,
    /// Required. The ID of the read group set to export. The caller must have
    /// READ access to this read group set.
    #[prost(string, tag="3")]
    pub read_group_set_id: ::prost::alloc::string::String,
    /// The reference names to export. If this is not specified, all reference
    /// sequences, including unmapped reads, are exported.
    /// Use `*` to export only unmapped reads.
    #[prost(string, repeated, tag="4")]
    pub reference_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateReadGroupSetRequest {
    /// The ID of the read group set to be updated. The caller must have WRITE
    /// permissions to the dataset associated with this read group set.
    #[prost(string, tag="1")]
    pub read_group_set_id: ::prost::alloc::string::String,
    /// The new read group set data. See `updateMask` for details on mutability of
    /// fields.
    #[prost(message, optional, tag="2")]
    pub read_group_set: ::core::option::Option<ReadGroupSet>,
    /// An optional mask specifying which fields to update. Supported fields:
    ///
    /// * \[name][google.genomics.v1.ReadGroupSet.name\].
    /// * \[referenceSetId][google.genomics.v1.ReadGroupSet.reference_set_id\].
    ///
    /// Leaving `updateMask` unset is equivalent to specifying all mutable
    /// fields.
    #[prost(message, optional, tag="3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteReadGroupSetRequest {
    /// The ID of the read group set to be deleted. The caller must have WRITE
    /// permissions to the dataset associated with this read group set.
    #[prost(string, tag="1")]
    pub read_group_set_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReadGroupSetRequest {
    /// The ID of the read group set.
    #[prost(string, tag="1")]
    pub read_group_set_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCoverageBucketsRequest {
    /// Required. The ID of the read group set over which coverage is requested.
    #[prost(string, tag="1")]
    pub read_group_set_id: ::prost::alloc::string::String,
    /// The name of the reference to query, within the reference set associated
    /// with this query. Optional.
    #[prost(string, tag="3")]
    pub reference_name: ::prost::alloc::string::String,
    /// The start position of the range on the reference, 0-based inclusive. If
    /// specified, `referenceName` must also be specified. Defaults to 0.
    #[prost(int64, tag="4")]
    pub start: i64,
    /// The end position of the range on the reference, 0-based exclusive. If
    /// specified, `referenceName` must also be specified. If unset or 0, defaults
    /// to the length of the reference.
    #[prost(int64, tag="5")]
    pub end: i64,
    /// The desired width of each reported coverage bucket in base pairs. This
    /// will be rounded down to the nearest precomputed bucket width; the value
    /// of which is returned as `bucketWidth` in the response. Defaults
    /// to infinity (each bucket spans an entire reference sequence) or the length
    /// of the target range, if specified. The smallest precomputed
    /// `bucketWidth` is currently 2048 base pairs; this is subject to
    /// change.
    #[prost(int64, tag="6")]
    pub target_bucket_width: i64,
    /// The continuation token, which is used to page through large result sets.
    /// To get the next page of results, set this parameter to the value of
    /// `nextPageToken` from the previous response.
    #[prost(string, tag="7")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single page. If unspecified,
    /// defaults to 1024. The maximum value is 2048.
    #[prost(int32, tag="8")]
    pub page_size: i32,
}
/// A bucket over which read coverage has been precomputed. A bucket corresponds
/// to a specific range of the reference sequence.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CoverageBucket {
    /// The genomic coordinate range spanned by this bucket.
    #[prost(message, optional, tag="1")]
    pub range: ::core::option::Option<Range>,
    /// The average number of reads which are aligned to each individual
    /// reference base in this bucket.
    #[prost(float, tag="2")]
    pub mean_coverage: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCoverageBucketsResponse {
    /// The length of each coverage bucket in base pairs. Note that buckets at the
    /// end of a reference sequence may be shorter. This value is omitted if the
    /// bucket width is infinity (the default behaviour, with no range or
    /// `targetBucketWidth`).
    #[prost(int64, tag="1")]
    pub bucket_width: i64,
    /// The coverage buckets. The list of buckets is sparse; a bucket with 0
    /// overlapping reads is not returned. A bucket never crosses more than one
    /// reference sequence. Each bucket has width `bucketWidth`, unless
    /// its end is the end of the reference sequence.
    #[prost(message, repeated, tag="2")]
    pub coverage_buckets: ::prost::alloc::vec::Vec<CoverageBucket>,
    /// The continuation token, which is used to page through large result sets.
    /// Provide this value in a subsequent request to return the next page of
    /// results. This field will be empty if there aren't any additional results.
    #[prost(string, tag="3")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The read search request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchReadsRequest {
    /// The IDs of the read groups sets within which to search for reads. All
    /// specified read group sets must be aligned against a common set of reference
    /// sequences; this defines the genomic coordinates for the query. Must specify
    /// one of `readGroupSetIds` or `readGroupIds`.
    #[prost(string, repeated, tag="1")]
    pub read_group_set_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The IDs of the read groups within which to search for reads. All specified
    /// read groups must belong to the same read group sets. Must specify one of
    /// `readGroupSetIds` or `readGroupIds`.
    #[prost(string, repeated, tag="5")]
    pub read_group_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The reference sequence name, for example `chr1`, `1`, or `chrX`. If set to
    /// `*`, only unmapped reads are returned. If unspecified, all reads (mapped
    /// and unmapped) are returned.
    #[prost(string, tag="7")]
    pub reference_name: ::prost::alloc::string::String,
    /// The start position of the range on the reference, 0-based inclusive. If
    /// specified, `referenceName` must also be specified.
    #[prost(int64, tag="8")]
    pub start: i64,
    /// The end position of the range on the reference, 0-based exclusive. If
    /// specified, `referenceName` must also be specified.
    #[prost(int64, tag="9")]
    pub end: i64,
    /// The continuation token, which is used to page through large result sets.
    /// To get the next page of results, set this parameter to the value of
    /// `nextPageToken` from the previous response.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single page. If unspecified,
    /// defaults to 256. The maximum value is 2048.
    #[prost(int32, tag="4")]
    pub page_size: i32,
}
/// The read search response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchReadsResponse {
    /// The list of matching alignments sorted by mapped genomic coordinate,
    /// if any, ascending in position within the same reference. Unmapped reads,
    /// which have no position, are returned contiguously and are sorted in
    /// ascending lexicographic order by fragment name.
    #[prost(message, repeated, tag="1")]
    pub alignments: ::prost::alloc::vec::Vec<Read>,
    /// The continuation token, which is used to page through large result sets.
    /// Provide this value in a subsequent request to return the next page of
    /// results. This field will be empty if there aren't any additional results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The stream reads request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamReadsRequest {
    /// The Google Cloud project ID which will be billed
    /// for this access. The caller must have WRITE access to this project.
    /// Required.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The ID of the read group set from which to stream reads.
    #[prost(string, tag="2")]
    pub read_group_set_id: ::prost::alloc::string::String,
    /// The reference sequence name, for example `chr1`,
    /// `1`, or `chrX`. If set to *, only unmapped reads are
    /// returned.
    #[prost(string, tag="3")]
    pub reference_name: ::prost::alloc::string::String,
    /// The start position of the range on the reference, 0-based inclusive. If
    /// specified, `referenceName` must also be specified.
    #[prost(int64, tag="4")]
    pub start: i64,
    /// The end position of the range on the reference, 0-based exclusive. If
    /// specified, `referenceName` must also be specified.
    #[prost(int64, tag="5")]
    pub end: i64,
    /// Restricts results to a shard containing approximately `1/totalShards`
    /// of the normal response payload for this query. Results from a sharded
    /// request are disjoint from those returned by all queries which differ only
    /// in their shard parameter. A shard may yield 0 results; this is especially
    /// likely for large values of `totalShards`.
    ///
    /// Valid values are `[0, totalShards)`.
    #[prost(int32, tag="6")]
    pub shard: i32,
    /// Specifying `totalShards` causes a disjoint subset of the normal response
    /// payload to be returned for each query with a unique `shard` parameter
    /// specified. A best effort is made to yield equally sized shards. Sharding
    /// can be used to distribute processing amongst workers, where each worker is
    /// assigned a unique `shard` number and all workers specify the same
    /// `totalShards` number. The union of reads returned for all sharded queries
    /// `[0, totalShards)` is equal to those returned by a single unsharded query.
    ///
    /// Queries for different values of `totalShards` with common divisors will
    /// share shard boundaries. For example, streaming `shard` 2 of 5
    /// `totalShards` yields the same results as streaming `shard`s 4 and 5 of 10
    /// `totalShards`. This property can be leveraged for adaptive retries.
    #[prost(int32, tag="7")]
    pub total_shards: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamReadsResponse {
    #[prost(message, repeated, tag="1")]
    pub alignments: ::prost::alloc::vec::Vec<Read>,
}
/// Generated client implementations.
pub mod streaming_read_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct StreamingReadServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> StreamingReadServiceClient<T>
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
        ) -> StreamingReadServiceClient<InterceptedService<T, F>>
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
            StreamingReadServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Returns a stream of all the reads matching the search request, ordered
        /// by reference name, position, and ID.
        pub async fn stream_reads(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamReadsRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::StreamReadsResponse>>,
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
                "/google.genomics.v1.StreamingReadService/StreamReads",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod read_service_v1_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// The Readstore. A data store for DNA sequencing Reads.
    #[derive(Debug, Clone)]
    pub struct ReadServiceV1Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ReadServiceV1Client<T>
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
        ) -> ReadServiceV1Client<InterceptedService<T, F>>
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
            ReadServiceV1Client::new(InterceptedService::new(inner, interceptor))
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
        /// Creates read group sets by asynchronously importing the provided
        /// information.
        ///
        /// For the definitions of read group sets and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        ///
        /// The caller must have WRITE permissions to the dataset.
        ///
        /// ## Notes on [BAM](https://samtools.github.io/hts-specs/SAMv1.pdf) import
        ///
        /// - Tags will be converted to strings - tag types are not preserved
        /// - Comments (`@CO`) in the input file header will not be preserved
        /// - Original header order of references (`@SQ`) will not be preserved
        /// - Any reverse stranded unmapped reads will be reverse complemented, and
        /// their qualities (also the "BQ" and "OQ" tags, if any) will be reversed
        /// - Unmapped reads will be stripped of positional information (reference name
        /// and position)
        pub async fn import_read_group_sets(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportReadGroupSetsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.genomics.v1.ReadServiceV1/ImportReadGroupSets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Exports a read group set to a BAM file in Google Cloud Storage.
        ///
        /// For the definitions of read group sets and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        ///
        /// Note that currently there may be some differences between exported BAM
        /// files and the original BAM file at the time of import. See
        /// [ImportReadGroupSets][google.genomics.v1.ReadServiceV1.ImportReadGroupSets]
        /// for caveats.
        pub async fn export_read_group_set(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportReadGroupSetRequest>,
        ) -> Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.genomics.v1.ReadServiceV1/ExportReadGroupSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Searches for read group sets matching the criteria.
        ///
        /// For the definitions of read group sets and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        ///
        /// Implements
        /// [GlobalAllianceApi.searchReadGroupSets](https://github.com/ga4gh/schemas/blob/v0.5.1/src/main/resources/avro/readmethods.avdl#L135).
        pub async fn search_read_group_sets(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchReadGroupSetsRequest>,
        ) -> Result<tonic::Response<super::SearchReadGroupSetsResponse>, tonic::Status> {
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
                "/google.genomics.v1.ReadServiceV1/SearchReadGroupSets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a read group set.
        ///
        /// For the definitions of read group sets and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        ///
        /// This method supports patch semantics.
        pub async fn update_read_group_set(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateReadGroupSetRequest>,
        ) -> Result<tonic::Response<super::ReadGroupSet>, tonic::Status> {
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
                "/google.genomics.v1.ReadServiceV1/UpdateReadGroupSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a read group set.
        ///
        /// For the definitions of read group sets and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        pub async fn delete_read_group_set(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteReadGroupSetRequest>,
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
                "/google.genomics.v1.ReadServiceV1/DeleteReadGroupSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a read group set by ID.
        ///
        /// For the definitions of read group sets and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        pub async fn get_read_group_set(
            &mut self,
            request: impl tonic::IntoRequest<super::GetReadGroupSetRequest>,
        ) -> Result<tonic::Response<super::ReadGroupSet>, tonic::Status> {
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
                "/google.genomics.v1.ReadServiceV1/GetReadGroupSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists fixed width coverage buckets for a read group set, each of which
        /// correspond to a range of a reference sequence. Each bucket summarizes
        /// coverage information across its corresponding genomic range.
        ///
        /// For the definitions of read group sets and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        ///
        /// Coverage is defined as the number of reads which are aligned to a given
        /// base in the reference sequence. Coverage buckets are available at several
        /// precomputed bucket widths, enabling retrieval of various coverage 'zoom
        /// levels'. The caller must have READ permissions for the target read group
        /// set.
        pub async fn list_coverage_buckets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCoverageBucketsRequest>,
        ) -> Result<tonic::Response<super::ListCoverageBucketsResponse>, tonic::Status> {
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
                "/google.genomics.v1.ReadServiceV1/ListCoverageBuckets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a list of reads for one or more read group sets.
        ///
        /// For the definitions of read group sets and other genomics resources, see
        /// [Fundamentals of Google
        /// Genomics](https://cloud.google.com/genomics/fundamentals-of-google-genomics)
        ///
        /// Reads search operates over a genomic coordinate space of reference sequence
        /// & position defined over the reference sequences to which the requested
        /// read group sets are aligned.
        ///
        /// If a target positional range is specified, search returns all reads whose
        /// alignment to the reference genome overlap the range. A query which
        /// specifies only read group set IDs yields all reads in those read group
        /// sets, including unmapped reads.
        ///
        /// All reads returned (including reads on subsequent pages) are ordered by
        /// genomic coordinate (by reference sequence, then position). Reads with
        /// equivalent genomic coordinates are returned in an unspecified order. This
        /// order is consistent, such that two queries for the same content (regardless
        /// of page size) yield reads in the same order across their respective streams
        /// of paginated responses.
        ///
        /// Implements
        /// [GlobalAllianceApi.searchReads](https://github.com/ga4gh/schemas/blob/v0.5.1/src/main/resources/avro/readmethods.avdl#L85).
        pub async fn search_reads(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchReadsRequest>,
        ) -> Result<tonic::Response<super::SearchReadsResponse>, tonic::Status> {
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
                "/google.genomics.v1.ReadServiceV1/SearchReads",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
