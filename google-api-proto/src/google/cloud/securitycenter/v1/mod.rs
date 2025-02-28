/// Contains compliance information about a security standard indicating unmet
/// recommendations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Compliance {
    /// Refers to industry wide standards or benchmarks e.g. "cis", "pci", "owasp",
    /// etc.
    #[prost(string, tag="1")]
    pub standard: ::prost::alloc::string::String,
    /// Version of the standard/benchmark e.g. 1.1
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
    /// Policies within the standard/benchmark e.g. A.12.4.1
    #[prost(string, repeated, tag="3")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Represents what's commonly known as an Indicator of compromise (IoC) in
/// computer forensics. This is an artifact observed on a network or in an
/// operating system that, with high confidence, indicates a computer intrusion.
/// Reference: <https://en.wikipedia.org/wiki/Indicator_of_compromise>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Indicator {
    /// List of ip addresses associated to the Finding.
    #[prost(string, repeated, tag="1")]
    pub ip_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// List of domains associated to the Finding.
    #[prost(string, repeated, tag="2")]
    pub domains: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The list of matched signatures indicating that the given
    /// process is present in the environment.
    #[prost(message, repeated, tag="3")]
    pub signatures: ::prost::alloc::vec::Vec<indicator::ProcessSignature>,
    /// The list of URIs associated to the Findings.
    #[prost(string, repeated, tag="4")]
    pub uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Indicator`.
pub mod indicator {
    /// Indicates what signature matched this process.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProcessSignature {
        #[prost(oneof="process_signature::Signature", tags="6, 7")]
        pub signature: ::core::option::Option<process_signature::Signature>,
    }
    /// Nested message and enum types in `ProcessSignature`.
    pub mod process_signature {
        /// A signature corresponding to memory page hashes.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MemoryHashSignature {
            /// The binary family.
            #[prost(string, tag="1")]
            pub binary_family: ::prost::alloc::string::String,
            /// The list of memory hash detections contributing to the binary family
            /// match.
            #[prost(message, repeated, tag="4")]
            pub detections: ::prost::alloc::vec::Vec<memory_hash_signature::Detection>,
        }
        /// Nested message and enum types in `MemoryHashSignature`.
        pub mod memory_hash_signature {
            /// Memory hash detection contributing to the binary family match.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Detection {
                /// The name of the binary associated with the memory hash
                /// signature detection.
                #[prost(string, tag="2")]
                pub binary: ::prost::alloc::string::String,
                /// The percentage of memory page hashes in the signature
                /// that were matched.
                #[prost(double, tag="3")]
                pub percent_pages_matched: f64,
            }
        }
        /// A signature corresponding to a YARA rule.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct YaraRuleSignature {
            /// The name of the YARA rule.
            #[prost(string, tag="5")]
            pub yara_rule: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Signature {
            /// Signature indicating that a binary family was matched.
            #[prost(message, tag="6")]
            MemoryHashSignature(MemoryHashSignature),
            /// Signature indicating that a YARA rule was matched.
            #[prost(message, tag="7")]
            YaraRuleSignature(YaraRuleSignature),
        }
    }
}
/// Refers to common vulnerability fields e.g. cve, cvss, cwe etc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vulnerability {
    /// CVE stands for Common Vulnerabilities and Exposures
    /// (<https://cve.mitre.org/about/>)
    #[prost(message, optional, tag="1")]
    pub cve: ::core::option::Option<Cve>,
}
/// CVE stands for Common Vulnerabilities and Exposures.
/// More information: <https://cve.mitre.org>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cve {
    /// The unique identifier for the vulnerability. e.g. CVE-2021-34527
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Additional information about the CVE.
    /// e.g. <https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2021-34527>
    #[prost(message, repeated, tag="2")]
    pub references: ::prost::alloc::vec::Vec<Reference>,
    /// Describe Common Vulnerability Scoring System specified at
    /// <https://www.first.org/cvss/v3.1/specification-document>
    #[prost(message, optional, tag="3")]
    pub cvssv3: ::core::option::Option<Cvssv3>,
    /// Whether upstream fix is available for the CVE.
    #[prost(bool, tag="4")]
    pub upstream_fix_available: bool,
}
/// Additional Links
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reference {
    /// Source of the reference e.g. NVD
    #[prost(string, tag="1")]
    pub source: ::prost::alloc::string::String,
    /// Uri for the mentioned source e.g.
    /// <https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2021-34527.>
    #[prost(string, tag="2")]
    pub uri: ::prost::alloc::string::String,
}
/// Common Vulnerability Scoring System version 3.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cvssv3 {
    /// The base score is a function of the base metric scores.
    #[prost(double, tag="1")]
    pub base_score: f64,
    /// Base Metrics
    /// Represents the intrinsic characteristics of a vulnerability that are
    /// constant over time and across user environments.
    /// This metric reflects the context by which vulnerability exploitation is
    /// possible.
    #[prost(enumeration="cvssv3::AttackVector", tag="5")]
    pub attack_vector: i32,
    /// This metric describes the conditions beyond the attacker's control that
    /// must exist in order to exploit the vulnerability.
    #[prost(enumeration="cvssv3::AttackComplexity", tag="6")]
    pub attack_complexity: i32,
    /// This metric describes the level of privileges an attacker must possess
    /// before successfully exploiting the vulnerability.
    #[prost(enumeration="cvssv3::PrivilegesRequired", tag="7")]
    pub privileges_required: i32,
    /// This metric captures the requirement for a human user, other than the
    /// attacker, to participate in the successful compromise of the vulnerable
    /// component.
    #[prost(enumeration="cvssv3::UserInteraction", tag="8")]
    pub user_interaction: i32,
    /// The Scope metric captures whether a vulnerability in one vulnerable
    /// component impacts resources in components beyond its security scope.
    #[prost(enumeration="cvssv3::Scope", tag="9")]
    pub scope: i32,
    /// This metric measures the impact to the confidentiality of the information
    /// resources managed by a software component due to a successfully exploited
    /// vulnerability.
    #[prost(enumeration="cvssv3::Impact", tag="10")]
    pub confidentiality_impact: i32,
    /// This metric measures the impact to integrity of a successfully exploited
    /// vulnerability.
    #[prost(enumeration="cvssv3::Impact", tag="11")]
    pub integrity_impact: i32,
    /// This metric measures the impact to the availability of the impacted
    /// component resulting from a successfully exploited vulnerability.
    #[prost(enumeration="cvssv3::Impact", tag="12")]
    pub availability_impact: i32,
}
/// Nested message and enum types in `Cvssv3`.
pub mod cvssv3 {
    /// This metric reflects the context by which vulnerability exploitation is
    /// possible.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AttackVector {
        /// Invalid value.
        Unspecified = 0,
        /// The vulnerable component is bound to the network stack and the set of
        /// possible attackers extends beyond the other options listed below, up to
        /// and including the entire Internet.
        Network = 1,
        /// The vulnerable component is bound to the network stack, but the attack is
        /// limited at the protocol level to a logically adjacent topology.
        Adjacent = 2,
        /// The vulnerable component is not bound to the network stack and the
        /// attacker's path is via read/write/execute capabilities.
        Local = 3,
        /// The attack requires the attacker to physically touch or manipulate the
        /// vulnerable component.
        Physical = 4,
    }
    /// This metric describes the conditions beyond the attacker's control that
    /// must exist in order to exploit the vulnerability.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AttackComplexity {
        /// Invalid value.
        Unspecified = 0,
        /// Specialized access conditions or extenuating circumstances do not exist.
        /// An attacker can expect repeatable success when attacking the vulnerable
        /// component.
        Low = 1,
        /// A successful attack depends on conditions beyond the attacker's control.
        /// That is, a successful attack cannot be accomplished at will, but requires
        /// the attacker to invest in some measurable amount of effort in preparation
        /// or execution against the vulnerable component before a successful attack
        /// can be expected.
        High = 2,
    }
    /// This metric describes the level of privileges an attacker must possess
    /// before successfully exploiting the vulnerability.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PrivilegesRequired {
        /// Invalid value.
        Unspecified = 0,
        /// The attacker is unauthorized prior to attack, and therefore does not
        /// require any access to settings or files of the vulnerable system to
        /// carry out an attack.
        None = 1,
        /// The attacker requires privileges that provide basic user capabilities
        /// that could normally affect only settings and files owned by a user.
        /// Alternatively, an attacker with Low privileges has the ability to access
        /// only non-sensitive resources.
        Low = 2,
        /// The attacker requires privileges that provide significant (e.g.,
        /// administrative) control over the vulnerable component allowing access to
        /// component-wide settings and files.
        High = 3,
    }
    /// This metric captures the requirement for a human user, other than the
    /// attacker, to participate in the successful compromise of the vulnerable
    /// component.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum UserInteraction {
        /// Invalid value.
        Unspecified = 0,
        /// The vulnerable system can be exploited without interaction from any user.
        None = 1,
        /// Successful exploitation of this vulnerability requires a user to take
        /// some action before the vulnerability can be exploited.
        Required = 2,
    }
    /// The Scope metric captures whether a vulnerability in one vulnerable
    /// component impacts resources in components beyond its security scope.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Scope {
        /// Invalid value.
        Unspecified = 0,
        /// An exploited vulnerability can only affect resources managed by the same
        /// security authority.
        Unchanged = 1,
        /// An exploited vulnerability can affect resources beyond the security scope
        /// managed by the security authority of the vulnerable component.
        Changed = 2,
    }
    /// The Impact metrics capture the effects of a successfully exploited
    /// vulnerability on the component that suffers the worst outcome that is most
    /// directly and predictably associated with the attack.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Impact {
        /// Invalid value.
        Unspecified = 0,
        /// High impact.
        High = 1,
        /// Low impact.
        Low = 2,
        /// No impact.
        None = 3,
    }
}
/// MITRE ATT&CK tactics and techniques related to this finding.
/// See: <https://attack.mitre.org>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MitreAttack {
    /// The MITRE ATT&CK tactic most closely represented by this finding, if any.
    #[prost(enumeration="mitre_attack::Tactic", tag="1")]
    pub primary_tactic: i32,
    /// The MITRE ATT&CK technique most closely represented by this finding, if
    /// any. primary_techniques is a repeated field because there are multiple
    /// levels of MITRE ATT&CK techniques.  If the technique most closely
    /// represented by this finding is a sub-technique (e.g. `SCANNING_IP_BLOCKS`),
    /// both the sub-technique and its parent technique(s) will be listed (e.g.
    /// `SCANNING_IP_BLOCKS`, `ACTIVE_SCANNING`).
    #[prost(enumeration="mitre_attack::Technique", repeated, tag="2")]
    pub primary_techniques: ::prost::alloc::vec::Vec<i32>,
    /// Additional MITRE ATT&CK tactics related to this finding, if any.
    #[prost(enumeration="mitre_attack::Tactic", repeated, tag="3")]
    pub additional_tactics: ::prost::alloc::vec::Vec<i32>,
    /// Additional MITRE ATT&CK techniques related to this finding, if any, along
    /// with any of their respective parent techniques.
    #[prost(enumeration="mitre_attack::Technique", repeated, tag="4")]
    pub additional_techniques: ::prost::alloc::vec::Vec<i32>,
    /// The MITRE ATT&CK version referenced by the above fields. E.g. "8".
    #[prost(string, tag="5")]
    pub version: ::prost::alloc::string::String,
}
/// Nested message and enum types in `MitreAttack`.
pub mod mitre_attack {
    /// MITRE ATT&CK tactics that can be referenced by SCC findings.
    /// See: <https://attack.mitre.org/tactics/enterprise/>
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Tactic {
        /// Unspecified value.
        Unspecified = 0,
        /// TA0043
        Reconnaissance = 1,
        /// TA0042
        ResourceDevelopment = 2,
        /// TA0001
        InitialAccess = 5,
        /// TA0002
        Execution = 3,
        /// TA0003
        Persistence = 6,
        /// TA0004
        PrivilegeEscalation = 8,
        /// TA0005
        DefenseEvasion = 7,
        /// TA0006
        CredentialAccess = 9,
        /// TA0007
        Discovery = 10,
        /// TA0008
        LateralMovement = 11,
        /// TA0009
        Collection = 12,
        /// TA0011
        CommandAndControl = 4,
        /// TA0010
        Exfiltration = 13,
        /// TA0040
        Impact = 14,
    }
    /// MITRE ATT&CK techniques that can be referenced by SCC findings.
    /// See: <https://attack.mitre.org/techniques/enterprise/>
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Technique {
        /// Unspecified value.
        Unspecified = 0,
        /// T1595
        ActiveScanning = 1,
        /// T1595.001
        ScanningIpBlocks = 2,
        /// T1105
        IngressToolTransfer = 3,
        /// T1106
        NativeApi = 4,
        /// T1129
        SharedModules = 5,
        /// T1059
        CommandAndScriptingInterpreter = 6,
        /// T1059.004
        UnixShell = 7,
        /// T1496
        ResourceHijacking = 8,
        /// T1090
        Proxy = 9,
        /// T1090.002
        ExternalProxy = 10,
        /// T1090.003
        MultiHopProxy = 11,
        /// T1568
        DynamicResolution = 12,
        /// T1552
        UnsecuredCredentials = 13,
        /// T1078
        ValidAccounts = 14,
        /// T1078.003
        LocalAccounts = 15,
        /// T1078.004
        CloudAccounts = 16,
        /// T1498
        NetworkDenialOfService = 17,
        /// T1069
        PermissionGroupsDiscovery = 18,
        /// T1069.003
        CloudGroups = 19,
        /// T1567
        ExfiltrationOverWebService = 20,
        /// T1567.002
        ExfiltrationToCloudStorage = 21,
        /// T1098
        AccountManipulation = 22,
        /// T1098.004
        SshAuthorizedKeys = 23,
        /// T1543
        CreateOrModifySystemProcess = 24,
        /// T1539
        StealWebSessionCookie = 25,
        /// T1578
        ModifyCloudComputeInfrastructure = 26,
        /// T1190
        ExploitPublicFacingApplication = 27,
        /// T1556
        ModifyAuthenticationProcess = 28,
        /// T1485
        DataDestruction = 29,
        /// T1484
        DomainPolicyModification = 30,
        /// T1562
        ImpairDefenses = 31,
        /// T1046
        NetworkServiceDiscovery = 32,
        /// T1134
        AccessTokenManipulation = 33,
        /// T1548
        AbuseElevationControlMechanism = 34,
    }
}
/// Represents an access event.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Access {
    /// Associated email, such as "foo@google.com".
    ///
    /// The email address of the authenticated user (or service account on behalf
    /// of third party principal) making the request. For third party identity
    /// callers, the `principal_subject` field is populated instead of this field.
    /// For privacy reasons, the principal email address is sometimes redacted.
    /// For more information, see [Caller identities in audit
    /// logs](<https://cloud.google.com/logging/docs/audit#user-id>).
    #[prost(string, tag="1")]
    pub principal_email: ::prost::alloc::string::String,
    /// Caller's IP address, such as "1.1.1.1".
    #[prost(string, tag="2")]
    pub caller_ip: ::prost::alloc::string::String,
    /// The caller IP's geolocation, which identifies where the call came from.
    #[prost(message, optional, tag="3")]
    pub caller_ip_geo: ::core::option::Option<Geolocation>,
    /// What kind of user agent is associated, e.g. operating system shells,
    /// embedded or stand-alone applications, etc.
    #[prost(string, tag="4")]
    pub user_agent_family: ::prost::alloc::string::String,
    /// This is the API service that the service account made a call to, e.g.
    /// "iam.googleapis.com"
    #[prost(string, tag="5")]
    pub service_name: ::prost::alloc::string::String,
    /// The method that the service account called, e.g. "SetIamPolicy".
    #[prost(string, tag="6")]
    pub method_name: ::prost::alloc::string::String,
    /// A string representing the principal_subject associated with the identity.
    /// As compared to `principal_email`, supports principals that aren't
    /// associated with email addresses, such as third party principals. For most
    /// identities, the format will be `principal://iam.googleapis.com/{identity
    /// pool name}/subjects/{subject}` except for some GKE identities
    /// (GKE_WORKLOAD, FREEFORM, GKE_HUB_WORKLOAD) that are still in the legacy
    /// format `serviceAccount:{identity pool name}\[{subject}\]`
    #[prost(string, tag="7")]
    pub principal_subject: ::prost::alloc::string::String,
    /// The name of the service account key used to create or exchange
    /// credentials for authenticating the service account making the request.
    /// This is a scheme-less URI full resource name. For example:
    ///
    /// "//iam.googleapis.com/projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}/keys/{key}"
    ///
    #[prost(string, tag="8")]
    pub service_account_key_name: ::prost::alloc::string::String,
    /// Identity delegation history of an authenticated service account that makes
    /// the request. It contains information on the real authorities that try to
    /// access GCP resources by delegating on a service account. When multiple
    /// authorities are present, they are guaranteed to be sorted based on the
    /// original ordering of the identity delegation events.
    #[prost(message, repeated, tag="9")]
    pub service_account_delegation_info: ::prost::alloc::vec::Vec<ServiceAccountDelegationInfo>,
}
/// Identity delegation history of an authenticated service account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceAccountDelegationInfo {
    /// The email address of a Google account.
    #[prost(string, tag="1")]
    pub principal_email: ::prost::alloc::string::String,
    /// A string representing the principal_subject associated with the identity.
    /// As compared to `principal_email`, supports principals that aren't
    /// associated with email addresses, such as third party principals. For most
    /// identities, the format will be `principal://iam.googleapis.com/{identity
    /// pool name}/subjects/{subject}` except for some GKE identities
    /// (GKE_WORKLOAD, FREEFORM, GKE_HUB_WORKLOAD) that are still in the legacy
    /// format `serviceAccount:{identity pool name}\[{subject}\]`
    #[prost(string, tag="2")]
    pub principal_subject: ::prost::alloc::string::String,
}
/// Represents a geographical location for a given access.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Geolocation {
    /// A CLDR.
    #[prost(string, tag="1")]
    pub region_code: ::prost::alloc::string::String,
}
/// File information about the related binary/library used by an executable, or
/// the script used by a script interpreter
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct File {
    /// Absolute path of the file as a JSON encoded string.
    #[prost(string, tag="1")]
    pub path: ::prost::alloc::string::String,
    /// Size of the file in bytes.
    #[prost(int64, tag="2")]
    pub size: i64,
    /// SHA256 hash of the first hashed_size bytes of the file encoded as a
    /// hex string.  If hashed_size == size, sha256 represents the SHA256 hash
    /// of the entire file.
    #[prost(string, tag="3")]
    pub sha256: ::prost::alloc::string::String,
    /// The length in bytes of the file prefix that was hashed.  If
    /// hashed_size == size, any hashes reported represent the entire
    /// file.
    #[prost(int64, tag="4")]
    pub hashed_size: i64,
    /// True when the hash covers only a prefix of the file.
    #[prost(bool, tag="5")]
    pub partially_hashed: bool,
    /// Prefix of the file contents as a JSON encoded string.
    /// (Currently only populated for Malicious Script Executed findings.)
    #[prost(string, tag="6")]
    pub contents: ::prost::alloc::string::String,
}
/// Represents an operating system process.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Process {
    /// The process name visible in utilities like `top` and `ps`; it can
    /// be accessed via `/proc/\[pid\]/comm` and changed with `prctl(PR_SET_NAME)`.
    #[prost(string, tag="12")]
    pub name: ::prost::alloc::string::String,
    /// File information for the process executable.
    #[prost(message, optional, tag="3")]
    pub binary: ::core::option::Option<File>,
    /// File information for libraries loaded by the process.
    #[prost(message, repeated, tag="4")]
    pub libraries: ::prost::alloc::vec::Vec<File>,
    /// When the process represents the invocation of a script,
    /// `binary` provides information about the interpreter while `script`
    /// provides information about the script file provided to the
    /// interpreter.
    #[prost(message, optional, tag="5")]
    pub script: ::core::option::Option<File>,
    /// Process arguments as JSON encoded strings.
    #[prost(string, repeated, tag="6")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// True if `args` is incomplete.
    #[prost(bool, tag="7")]
    pub arguments_truncated: bool,
    /// Process environment variables.
    #[prost(message, repeated, tag="8")]
    pub env_variables: ::prost::alloc::vec::Vec<EnvironmentVariable>,
    /// True if `env_variables` is incomplete.
    #[prost(bool, tag="9")]
    pub env_variables_truncated: bool,
    /// The process id.
    #[prost(int64, tag="10")]
    pub pid: i64,
    /// The parent process id.
    #[prost(int64, tag="11")]
    pub parent_pid: i64,
}
/// EnvironmentVariable is a name-value pair to store environment variables for
/// Process.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvironmentVariable {
    /// Environment variable name as a JSON encoded string.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Environment variable value as a JSON encoded string.
    #[prost(string, tag="2")]
    pub val: ::prost::alloc::string::String,
}
/// Security Command Center finding source. A finding source
/// is an entity or a mechanism that can produce a finding. A source is like a
/// container of findings that come from the same scanner, logger, monitor, and
/// other tools.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Source {
    /// The relative resource name of this source. See:
    /// <https://cloud.google.com/apis/design/resource_names#relative_resource_name>
    /// Example:
    /// "organizations/{organization_id}/sources/{source_id}"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The source's display name.
    /// A source's display name must be unique amongst its siblings, for example,
    /// two sources with the same parent can't share the same display name.
    /// The display name must have a length between 1 and 64 characters
    /// (inclusive).
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    /// The description of the source (max of 1024 characters).
    /// Example:
    /// "Web Security Scanner is a web security scanner for common
    /// vulnerabilities in App Engine applications. It can automatically
    /// scan and detect four common vulnerabilities, including cross-site-scripting
    /// (XSS), Flash injection, mixed content (HTTP in HTTPS), and
    /// outdated or insecure libraries."
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    /// The canonical name of the finding. It's either
    /// "organizations/{organization_id}/sources/{source_id}",
    /// "folders/{folder_id}/sources/{source_id}" or
    /// "projects/{project_number}/sources/{source_id}",
    /// depending on the closest CRM ancestor of the resource associated with the
    /// finding.
    #[prost(string, tag="14")]
    pub canonical_name: ::prost::alloc::string::String,
}
/// Message that contains the resource name and display name of a folder
/// resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Folder {
    /// Full resource name of this folder. See:
    /// <https://cloud.google.com/apis/design/resource_names#full_resource_name>
    #[prost(string, tag="1")]
    pub resource_folder: ::prost::alloc::string::String,
    /// The user defined display name for this folder.
    #[prost(string, tag="2")]
    pub resource_folder_display_name: ::prost::alloc::string::String,
}
/// Information related to the Google Cloud resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    /// The full resource name of the resource. See:
    /// <https://cloud.google.com/apis/design/resource_names#full_resource_name>
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The human readable name of the resource.
    #[prost(string, tag="8")]
    pub display_name: ::prost::alloc::string::String,
    /// The full resource type of the resource.
    #[prost(string, tag="6")]
    pub r#type: ::prost::alloc::string::String,
    /// The full resource name of project that the resource belongs to.
    #[prost(string, tag="2")]
    pub project: ::prost::alloc::string::String,
    /// The project ID that the resource belongs to.
    #[prost(string, tag="3")]
    pub project_display_name: ::prost::alloc::string::String,
    /// The full resource name of resource's parent.
    #[prost(string, tag="4")]
    pub parent: ::prost::alloc::string::String,
    /// The human readable name of resource's parent.
    #[prost(string, tag="5")]
    pub parent_display_name: ::prost::alloc::string::String,
    /// Output only. Contains a Folder message for each folder in the assets ancestry.
    /// The first folder is the deepest nested folder, and the last folder is the
    /// folder directly under the Organization.
    #[prost(message, repeated, tag="7")]
    pub folders: ::prost::alloc::vec::Vec<Folder>,
}
/// A mute config is a Cloud SCC resource that contains the configuration
/// to mute create/update events of findings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MuteConfig {
    /// This field will be ignored if provided on config creation. Format
    /// "organizations/{organization}/muteConfigs/{mute_config}"
    /// "folders/{folder}/muteConfigs/{mute_config}"
    /// "projects/{project}/muteConfigs/{mute_config}"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The human readable name to be displayed for the mute config.
    #[deprecated]
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    /// A description of the mute config.
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    /// Required. An expression that defines the filter to apply across create/update events
    /// of findings. While creating a filter string, be mindful of the
    /// scope in which the mute configuration is being created. E.g., If a filter
    /// contains project = X but is created under the project = Y scope, it might
    /// not match any findings.
    ///
    /// The following field and operator combinations are supported:
    ///
    /// * severity: `=`, `:`
    /// * category: `=`, `:`
    /// * resource.name: `=`, `:`
    /// * resource.project_name: `=`, `:`
    /// * resource.project_display_name: `=`, `:`
    /// * resource.folders.resource_folder: `=`, `:`
    /// * resource.parent_name: `=`, `:`
    /// * resource.parent_display_name: `=`, `:`
    /// * resource.type: `=`, `:`
    /// * finding_class: `=`, `:`
    /// * indicator.ip_addresses: `=`, `:`
    /// * indicator.domains: `=`, `:`
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
    /// Output only. The time at which the mute config was created.
    /// This field is set by the server and will be ignored if provided on config
    /// creation.
    #[prost(message, optional, tag="5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The most recent time at which the mute config was updated.
    /// This field is set by the server and will be ignored if provided on config
    /// creation or update.
    #[prost(message, optional, tag="6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Email address of the user who last edited the mute config.
    /// This field is set by the server and will be ignored if provided on config
    /// creation or update.
    #[prost(string, tag="7")]
    pub most_recent_editor: ::prost::alloc::string::String,
}
/// Response of asset discovery run
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunAssetDiscoveryResponse {
    /// The state of an asset discovery run.
    #[prost(enumeration="run_asset_discovery_response::State", tag="1")]
    pub state: i32,
    /// The duration between asset discovery run start and end
    #[prost(message, optional, tag="2")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
}
/// Nested message and enum types in `RunAssetDiscoveryResponse`.
pub mod run_asset_discovery_response {
    /// The state of an asset discovery run.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Asset discovery run state was unspecified.
        Unspecified = 0,
        /// Asset discovery run completed successfully.
        Completed = 1,
        /// Asset discovery run was cancelled with tasks still pending, as another
        /// run for the same organization was started with a higher priority.
        Superseded = 2,
        /// Asset discovery run was killed and terminated.
        Terminated = 3,
    }
}
/// Representation of third party SIEM/SOAR fields within SCC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalSystem {
    /// External System Name e.g. jira, demisto, etc.
    ///  e.g.:
    ///  `organizations/1234/sources/5678/findings/123456/externalSystems/jira`
    /// `folders/1234/sources/5678/findings/123456/externalSystems/jira`
    /// `projects/1234/sources/5678/findings/123456/externalSystems/jira`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// References primary/secondary etc assignees in the external system.
    #[prost(string, repeated, tag="2")]
    pub assignees: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Identifier that's used to track the given finding in the external system.
    #[prost(string, tag="3")]
    pub external_uid: ::prost::alloc::string::String,
    /// Most recent status of the corresponding finding's ticket/tracker in the
    /// external system.
    #[prost(string, tag="4")]
    pub status: ::prost::alloc::string::String,
    /// The most recent time when the corresponding finding's ticket/tracker was
    /// updated in the external system.
    #[prost(message, optional, tag="5")]
    pub external_system_update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Represents a particular IAM binding, which captures a member's role addition,
/// removal, or state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IamBinding {
    /// The action that was performed on a Binding.
    #[prost(enumeration="iam_binding::Action", tag="1")]
    pub action: i32,
    /// Role that is assigned to "members".
    /// For example, "roles/viewer", "roles/editor", or "roles/owner".
    #[prost(string, tag="2")]
    pub role: ::prost::alloc::string::String,
    /// A single identity requesting access for a Cloud Platform resource,
    /// e.g. "foo@google.com".
    #[prost(string, tag="3")]
    pub member: ::prost::alloc::string::String,
}
/// Nested message and enum types in `IamBinding`.
pub mod iam_binding {
    /// The type of action performed on a Binding in a policy.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Action {
        /// Unspecified.
        Unspecified = 0,
        /// Addition of a Binding.
        Add = 1,
        /// Removal of a Binding.
        Remove = 2,
    }
}
/// Cloud Security Command Center (Cloud SCC) notification configs.
///
/// A notification config is a Cloud SCC resource that contains the configuration
/// to send notifications for create/update events of findings, assets and etc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationConfig {
    /// The relative resource name of this notification config. See:
    /// <https://cloud.google.com/apis/design/resource_names#relative_resource_name>
    /// Example:
    /// "organizations/{organization_id}/notificationConfigs/notify_public_bucket".
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The description of the notification config (max of 1024 characters).
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// The Pub/Sub topic to send notifications to. Its format is
    /// "projects/\[project_id]/topics/[topic\]".
    #[prost(string, tag="3")]
    pub pubsub_topic: ::prost::alloc::string::String,
    /// Output only. The service account that needs "pubsub.topics.publish"
    /// permission to publish to the Pub/Sub topic.
    #[prost(string, tag="4")]
    pub service_account: ::prost::alloc::string::String,
    /// The config for triggering notifications.
    #[prost(oneof="notification_config::NotifyConfig", tags="5")]
    pub notify_config: ::core::option::Option<notification_config::NotifyConfig>,
}
/// Nested message and enum types in `NotificationConfig`.
pub mod notification_config {
    /// The config for streaming-based notifications, which send each event as soon
    /// as it is detected.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StreamingConfig {
        /// Expression that defines the filter to apply across create/update events
        /// of assets or findings as specified by the event type. The expression is a
        /// list of zero or more restrictions combined via logical operators `AND`
        /// and `OR`. Parentheses are supported, and `OR` has higher precedence than
        /// `AND`.
        ///
        /// Restrictions have the form `<field> <operator> <value>` and may have a
        /// `-` character in front of them to indicate negation. The fields map to
        /// those defined in the corresponding resource.
        ///
        /// The supported operators are:
        ///
        /// * `=` for all value types.
        /// * `>`, `<`, `>=`, `<=` for integer values.
        /// * `:`, meaning substring matching, for strings.
        ///
        /// The supported value types are:
        ///
        /// * string literals in quotes.
        /// * integer literals without quotes.
        /// * boolean literals `true` and `false` without quotes.
        #[prost(string, tag="1")]
        pub filter: ::prost::alloc::string::String,
    }
    /// The config for triggering notifications.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum NotifyConfig {
        /// The config for triggering streaming-based notifications.
        #[prost(message, tag="5")]
        StreamingConfig(StreamingConfig),
    }
}
/// Exfiltration represents a data exfiltration attempt of one or more
/// sources to one or more targets.  Sources represent the source
/// of data that is exfiltrated, and Targets represents the destination the
/// data was copied to.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Exfiltration {
    /// If there are multiple sources, then the data is considered "joined" between
    /// them. For instance, BigQuery can join multiple tables, and each
    /// table would be considered a source.
    #[prost(message, repeated, tag="1")]
    pub sources: ::prost::alloc::vec::Vec<ExfilResource>,
    /// If there are multiple targets, each target would get a complete copy of the
    /// "joined" source data.
    #[prost(message, repeated, tag="2")]
    pub targets: ::prost::alloc::vec::Vec<ExfilResource>,
}
/// Resource that has been exfiltrated or exfiltrated_to.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExfilResource {
    /// Resource's URI (<https://google.aip.dev/122#full-resource-names>)
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Subcomponents of the asset that is exfiltrated - these could be
    /// URIs used during exfiltration, table names, databases, filenames, etc.
    /// For example, multiple tables may be exfiltrated from the same CloudSQL
    /// instance, or multiple files from the same Cloud Storage bucket.
    #[prost(string, repeated, tag="2")]
    pub components: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Configures how to deliver Findings to BigQuery Instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryExport {
    /// The relative resource name of this export. See:
    /// <https://cloud.google.com/apis/design/resource_names#relative_resource_name.>
    /// Example format:
    /// "organizations/{organization_id}/bigQueryExports/{export_id}" Example
    /// format: "folders/{folder_id}/bigQueryExports/{export_id}" Example format:
    /// "projects/{project_id}/bigQueryExports/{export_id}"
    /// This field is provided in responses, and is ignored when provided in create
    /// requests.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The description of the export (max of 1024 characters).
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// Expression that defines the filter to apply across create/update events
    /// of findings. The expression is a list of zero or more restrictions combined
    /// via logical operators `AND` and `OR`. Parentheses are supported, and `OR`
    /// has higher precedence than `AND`.
    ///
    /// Restrictions have the form `<field> <operator> <value>` and may have a
    /// `-` character in front of them to indicate negation. The fields map to
    /// those defined in the corresponding resource.
    ///
    /// The supported operators are:
    ///
    /// * `=` for all value types.
    /// * `>`, `<`, `>=`, `<=` for integer values.
    /// * `:`, meaning substring matching, for strings.
    ///
    /// The supported value types are:
    ///
    /// * string literals in quotes.
    /// * integer literals without quotes.
    /// * boolean literals `true` and `false` without quotes.
    #[prost(string, tag="3")]
    pub filter: ::prost::alloc::string::String,
    /// The dataset to write findings' updates to. Its format is
    /// "projects/\[project_id]/datasets/[bigquery_dataset_id\]".
    /// BigQuery Dataset unique ID  must contain only letters (a-z, A-Z), numbers
    /// (0-9), or underscores (_).
    #[prost(string, tag="4")]
    pub dataset: ::prost::alloc::string::String,
    /// Output only. The time at which the big query export was created.
    /// This field is set by the server and will be ignored if provided on export
    /// on creation.
    #[prost(message, optional, tag="5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The most recent time at which the big export was updated.
    /// This field is set by the server and will be ignored if provided on export
    /// creation or update.
    #[prost(message, optional, tag="6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Email address of the user who last edited the big query export.
    /// This field is set by the server and will be ignored if provided on export
    /// creation or update.
    #[prost(string, tag="7")]
    pub most_recent_editor: ::prost::alloc::string::String,
    /// Output only. The service account that needs permission to create table, upload data to
    /// the big query dataset.
    #[prost(string, tag="8")]
    pub principal: ::prost::alloc::string::String,
}
/// Contains information about the IP connection associated with the finding.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Connection {
    /// Destination IP address. Not present for sockets that are listening and not
    /// connected.
    #[prost(string, tag="1")]
    pub destination_ip: ::prost::alloc::string::String,
    /// Destination port. Not present for sockets that are listening and not
    /// connected.
    #[prost(int32, tag="2")]
    pub destination_port: i32,
    /// Source IP address.
    #[prost(string, tag="3")]
    pub source_ip: ::prost::alloc::string::String,
    /// Source port.
    #[prost(int32, tag="4")]
    pub source_port: i32,
    /// IANA Internet Protocol Number such as TCP(6) and UDP(17).
    #[prost(enumeration="connection::Protocol", tag="5")]
    pub protocol: i32,
}
/// Nested message and enum types in `Connection`.
pub mod connection {
    /// IANA Internet Protocol Number such as TCP(6) and UDP(17).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Protocol {
        /// Unspecified protocol (not HOPOPT).
        Unspecified = 0,
        /// Internet Control Message Protocol.
        Icmp = 1,
        /// Transmission Control Protocol.
        Tcp = 6,
        /// User Datagram Protocol.
        Udp = 17,
        /// Generic Routing Encapsulation.
        Gre = 47,
        /// Encap Security Payload.
        Esp = 50,
    }
}
/// The details pertaining to specific contacts
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContactDetails {
    /// A list of contacts
    #[prost(message, repeated, tag="1")]
    pub contacts: ::prost::alloc::vec::Vec<Contact>,
}
/// Representa a single contact's email address
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contact {
    /// An email address e.g. "person123@company.com"
    #[prost(string, tag="1")]
    pub email: ::prost::alloc::string::String,
}
/// Label represents a generic name=value label. Label has separate name and
/// value fields to support filtering with contains().
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Label {
    /// Label name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Label value.
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
/// Container associated with the finding.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Container {
    /// Container name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Container image URI provided when configuring a pod/container.
    /// May identify a container image version using mutable tags.
    #[prost(string, tag="2")]
    pub uri: ::prost::alloc::string::String,
    /// Optional container image id, when provided by the container runtime.
    /// Uniquely identifies the container image launched using a container image
    /// digest.
    #[prost(string, tag="3")]
    pub image_id: ::prost::alloc::string::String,
    /// Container labels, as provided by the container runtime.
    #[prost(message, repeated, tag="4")]
    pub labels: ::prost::alloc::vec::Vec<Label>,
}
/// Represents database access information, such as queries.
/// A database may be a sub-resource of an instance (as in the case of CloudSQL
/// instances or Cloud Spanner instances), or the database instance itself.
/// Some database resources may not have the full resource name populated
/// because these resource types are not yet supported by Cloud Asset Inventory
/// (e.g. CloudSQL databases).  In these cases only the display name will be
/// provided.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Database {
    /// The full resource name of the database the user connected to, if it is
    /// supported by CAI. (<https://google.aip.dev/122#full-resource-names>)
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The human readable name of the database the user connected to.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    /// The username used to connect to the DB. This may not necessarily be an IAM
    /// principal, and has no required format.
    #[prost(string, tag="3")]
    pub user_name: ::prost::alloc::string::String,
    /// The SQL statement associated with the relevant access.
    #[prost(string, tag="4")]
    pub query: ::prost::alloc::string::String,
    /// The target usernames/roles/groups of a SQL privilege grant (not an IAM
    /// policy change).
    #[prost(string, repeated, tag="5")]
    pub grantees: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Kubernetes related attributes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Kubernetes {
    /// Kubernetes Pods associated with the finding. This field will contain Pod
    /// records for each container that is owned by a Pod.
    #[prost(message, repeated, tag="1")]
    pub pods: ::prost::alloc::vec::Vec<kubernetes::Pod>,
    /// Provides Kubernetes Node information.
    #[prost(message, repeated, tag="2")]
    pub nodes: ::prost::alloc::vec::Vec<kubernetes::Node>,
    /// GKE Node Pools associated with the finding. This field will
    /// contain NodePool information for each Node, when it is available.
    #[prost(message, repeated, tag="3")]
    pub node_pools: ::prost::alloc::vec::Vec<kubernetes::NodePool>,
    /// Provides Kubernetes role information for findings that involve
    /// Roles or ClusterRoles.
    #[prost(message, repeated, tag="4")]
    pub roles: ::prost::alloc::vec::Vec<kubernetes::Role>,
    /// Provides Kubernetes role binding information for findings that involve
    /// RoleBindings or ClusterRoleBindings.
    #[prost(message, repeated, tag="5")]
    pub bindings: ::prost::alloc::vec::Vec<kubernetes::Binding>,
    /// Provides information on any Kubernetes access reviews (i.e. privilege
    /// checks) relevant to the finding.
    #[prost(message, repeated, tag="6")]
    pub access_reviews: ::prost::alloc::vec::Vec<kubernetes::AccessReview>,
}
/// Nested message and enum types in `Kubernetes`.
pub mod kubernetes {
    /// Kubernetes Pod.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Pod {
        /// Kubernetes Pod namespace.
        #[prost(string, tag="1")]
        pub ns: ::prost::alloc::string::String,
        /// Kubernetes Pod name.
        #[prost(string, tag="2")]
        pub name: ::prost::alloc::string::String,
        /// Pod labels.  For Kubernetes containers, these are applied to the
        /// container.
        #[prost(message, repeated, tag="3")]
        pub labels: ::prost::alloc::vec::Vec<super::Label>,
        /// Pod containers associated with this finding, if any.
        #[prost(message, repeated, tag="4")]
        pub containers: ::prost::alloc::vec::Vec<super::Container>,
    }
    /// Kubernetes Nodes associated with the finding.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Node {
        /// Full Resource name of the Compute Engine VM running the
        /// cluster node.
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
    }
    /// Provides GKE Node Pool information.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NodePool {
        /// Kubernetes Node pool name.
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        /// Nodes associated with the finding.
        #[prost(message, repeated, tag="2")]
        pub nodes: ::prost::alloc::vec::Vec<Node>,
    }
    /// Kubernetes Role or ClusterRole.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Role {
        /// Role type.
        #[prost(enumeration="role::Kind", tag="1")]
        pub kind: i32,
        /// Role namespace.
        #[prost(string, tag="2")]
        pub ns: ::prost::alloc::string::String,
        /// Role name.
        #[prost(string, tag="3")]
        pub name: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `Role`.
    pub mod role {
        /// Types of Kubernetes roles.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Kind {
            /// Role type is not specified.
            Unspecified = 0,
            /// Kubernetes Role.
            Role = 1,
            /// Kubernetes ClusterRole.
            ClusterRole = 2,
        }
    }
    /// Represents a Kubernetes RoleBinding or ClusterRoleBinding.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Binding {
        /// Namespace for binding.
        #[prost(string, tag="1")]
        pub ns: ::prost::alloc::string::String,
        /// Name for binding.
        #[prost(string, tag="2")]
        pub name: ::prost::alloc::string::String,
        /// The Role or ClusterRole referenced by the binding.
        #[prost(message, optional, tag="3")]
        pub role: ::core::option::Option<Role>,
        /// Represents the subjects(s) bound to the role. Not always available
        /// for PATCH requests.
        #[prost(message, repeated, tag="4")]
        pub subjects: ::prost::alloc::vec::Vec<Subject>,
    }
    /// Represents a Kubernetes Subject.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Subject {
        /// Authentication type for subject.
        #[prost(enumeration="subject::AuthType", tag="1")]
        pub kind: i32,
        /// Namespace for subject.
        #[prost(string, tag="2")]
        pub ns: ::prost::alloc::string::String,
        /// Name for subject.
        #[prost(string, tag="3")]
        pub name: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `Subject`.
    pub mod subject {
        /// Auth types that can be used for Subject's kind field.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum AuthType {
            /// Authentication is not specified.
            Unspecified = 0,
            /// User with valid certificate.
            User = 1,
            /// Users managed by Kubernetes API with credentials stored as Secrets.
            Serviceaccount = 2,
            /// Collection of users.
            Group = 3,
        }
    }
    /// Conveys information about a Kubernetes access review (e.g. kubectl auth
    /// can-i ...) that was involved in a finding.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AccessReview {
        /// Group is the API Group of the Resource. "*" means all.
        #[prost(string, tag="1")]
        pub group: ::prost::alloc::string::String,
        /// Namespace of the action being requested. Currently, there is no
        /// distinction between no namespace and all namespaces.  Both
        /// are represented by "" (empty).
        #[prost(string, tag="2")]
        pub ns: ::prost::alloc::string::String,
        /// Name is the name of the resource being requested. Empty means all.
        #[prost(string, tag="3")]
        pub name: ::prost::alloc::string::String,
        /// Resource is the optional resource type requested. "*" means all.
        #[prost(string, tag="4")]
        pub resource: ::prost::alloc::string::String,
        /// Subresource is the optional subresource type.
        #[prost(string, tag="5")]
        pub subresource: ::prost::alloc::string::String,
        /// Verb is a Kubernetes resource API verb, like: get, list, watch, create,
        /// update, delete, proxy. "*" means all.
        #[prost(string, tag="6")]
        pub verb: ::prost::alloc::string::String,
        /// Version is the API Version of the Resource. "*" means all.
        #[prost(string, tag="7")]
        pub version: ::prost::alloc::string::String,
    }
}
/// User specified security marks that are attached to the parent Security
/// Command Center resource. Security marks are scoped within a Security Command
/// Center organization -- they can be modified and viewed by all users who have
/// proper permissions on the organization.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityMarks {
    /// The relative resource name of the SecurityMarks. See:
    /// <https://cloud.google.com/apis/design/resource_names#relative_resource_name>
    /// Examples:
    /// "organizations/{organization_id}/assets/{asset_id}/securityMarks"
    /// "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}/securityMarks".
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Mutable user specified security marks belonging to the parent resource.
    /// Constraints are as follows:
    ///
    ///   * Keys and values are treated as case insensitive
    ///   * Keys must be between 1 - 256 characters (inclusive)
    ///   * Keys must be letters, numbers, underscores, or dashes
    ///   * Values have leading and trailing whitespace trimmed, remaining
    ///     characters must be between 1 - 4096 characters (inclusive)
    #[prost(btree_map="string, string", tag="2")]
    pub marks: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// The canonical name of the marks.
    /// Examples:
    /// "organizations/{organization_id}/assets/{asset_id}/securityMarks"
    /// "folders/{folder_id}/assets/{asset_id}/securityMarks"
    /// "projects/{project_number}/assets/{asset_id}/securityMarks"
    /// "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}/securityMarks"
    /// "folders/{folder_id}/sources/{source_id}/findings/{finding_id}/securityMarks"
    /// "projects/{project_number}/sources/{source_id}/findings/{finding_id}/securityMarks"
    #[prost(string, tag="3")]
    pub canonical_name: ::prost::alloc::string::String,
}
/// Security Command Center finding.
///
/// A finding is a record of assessment data like security, risk, health, or
/// privacy, that is ingested into Security Command Center for presentation,
/// notification, analysis, policy testing, and enforcement. For example, a
/// cross-site scripting (XSS) vulnerability in an App Engine application is a
/// finding.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Finding {
    /// The relative resource name of this finding. See:
    /// <https://cloud.google.com/apis/design/resource_names#relative_resource_name>
    /// Example:
    /// "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The relative resource name of the source the finding belongs to. See:
    /// <https://cloud.google.com/apis/design/resource_names#relative_resource_name>
    /// This field is immutable after creation time.
    /// For example:
    /// "organizations/{organization_id}/sources/{source_id}"
    #[prost(string, tag="2")]
    pub parent: ::prost::alloc::string::String,
    /// For findings on Google Cloud resources, the full resource
    /// name of the Google Cloud resource this finding is for. See:
    /// <https://cloud.google.com/apis/design/resource_names#full_resource_name>
    /// When the finding is for a non-Google Cloud resource, the resourceName can
    /// be a customer or partner defined string. This field is immutable after
    /// creation time.
    #[prost(string, tag="3")]
    pub resource_name: ::prost::alloc::string::String,
    /// The state of the finding.
    #[prost(enumeration="finding::State", tag="4")]
    pub state: i32,
    /// The additional taxonomy group within findings from a given source.
    /// This field is immutable after creation time.
    /// Example: "XSS_FLASH_INJECTION"
    #[prost(string, tag="5")]
    pub category: ::prost::alloc::string::String,
    /// The URI that, if available, points to a web page outside of Security
    /// Command Center where additional information about the finding can be found.
    /// This field is guaranteed to be either empty or a well formed URL.
    #[prost(string, tag="6")]
    pub external_uri: ::prost::alloc::string::String,
    /// Source specific properties. These properties are managed by the source
    /// that writes the finding. The key names in the source_properties map must be
    /// between 1 and 255 characters, and must start with a letter and contain
    /// alphanumeric characters or underscores only.
    #[prost(btree_map="string, message", tag="7")]
    pub source_properties: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost_types::Value>,
    /// Output only. User specified security marks. These marks are entirely
    /// managed by the user and come from the SecurityMarks resource that belongs
    /// to the finding.
    #[prost(message, optional, tag="8")]
    pub security_marks: ::core::option::Option<SecurityMarks>,
    /// The time the finding was first detected. If an existing finding is updated,
    /// then this is the time the update occurred.
    /// For example, if the finding represents an open firewall, this property
    /// captures the time the detector believes the firewall became open. The
    /// accuracy is determined by the detector. If the finding is later resolved,
    /// then this time reflects when the finding was resolved. This must not
    /// be set to a value greater than the current timestamp.
    #[prost(message, optional, tag="9")]
    pub event_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time at which the finding was created in Security Command Center.
    #[prost(message, optional, tag="10")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The severity of the finding. This field is managed by the source that
    /// writes the finding.
    #[prost(enumeration="finding::Severity", tag="12")]
    pub severity: i32,
    /// The canonical name of the finding. It's either
    /// "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}",
    /// "folders/{folder_id}/sources/{source_id}/findings/{finding_id}" or
    /// "projects/{project_number}/sources/{source_id}/findings/{finding_id}",
    /// depending on the closest CRM ancestor of the resource associated with the
    /// finding.
    #[prost(string, tag="14")]
    pub canonical_name: ::prost::alloc::string::String,
    /// Indicates the mute state of a finding (either muted, unmuted
    /// or undefined). Unlike other attributes of a finding, a finding provider
    /// shouldn't set the value of mute.
    #[prost(enumeration="finding::Mute", tag="15")]
    pub mute: i32,
    /// The class of the finding.
    #[prost(enumeration="finding::FindingClass", tag="17")]
    pub finding_class: i32,
    /// Represents what's commonly known as an Indicator of compromise (IoC) in
    /// computer forensics. This is an artifact observed on a network or in an
    /// operating system that, with high confidence, indicates a computer
    /// intrusion.
    /// Reference: <https://en.wikipedia.org/wiki/Indicator_of_compromise>
    #[prost(message, optional, tag="18")]
    pub indicator: ::core::option::Option<Indicator>,
    /// Represents vulnerability specific fields like cve, cvss scores etc.
    /// CVE stands for Common Vulnerabilities and Exposures
    /// (<https://cve.mitre.org/about/>)
    #[prost(message, optional, tag="20")]
    pub vulnerability: ::core::option::Option<Vulnerability>,
    /// Output only. The most recent time this finding was muted or unmuted.
    #[prost(message, optional, tag="21")]
    pub mute_update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Third party SIEM/SOAR fields within SCC, contains external
    /// system information and external system finding fields.
    #[prost(btree_map="string, message", tag="22")]
    pub external_systems: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ExternalSystem>,
    /// MITRE ATT&CK tactics and techniques related to this finding.
    /// See: <https://attack.mitre.org>
    #[prost(message, optional, tag="25")]
    pub mitre_attack: ::core::option::Option<MitreAttack>,
    /// Access details associated to the Finding, such as more information on the
    /// caller, which method was accessed, from where, etc.
    #[prost(message, optional, tag="26")]
    pub access: ::core::option::Option<Access>,
    /// Contains information about the IP connection associated with the finding.
    #[prost(message, repeated, tag="31")]
    pub connections: ::prost::alloc::vec::Vec<Connection>,
    /// First known as mute_annotation. Records additional information about the
    /// mute operation e.g. mute config that muted the finding, user who muted the
    /// finding, etc. Unlike other attributes of a finding, a finding provider
    /// shouldn't set the value of mute.
    #[prost(string, tag="28")]
    pub mute_initiator: ::prost::alloc::string::String,
    /// Represents operating system processes associated with the Finding.
    #[prost(message, repeated, tag="30")]
    pub processes: ::prost::alloc::vec::Vec<Process>,
    /// Output only. Map containing the point of contacts for the given finding.
    /// The key represents the type of contact, while the value contains a list of
    /// all the contacts that pertain. Please refer to:
    /// <https://cloud.google.com/resource-manager/docs/managing-notification-contacts#notification-categories>
    ///
    ///     {
    ///       "security": {
    ///         "contacts": [
    ///           {
    ///             "email": "person1@company.com"
    ///           },
    ///           {
    ///             "email": "person2@company.com"
    ///           }
    ///         ]
    ///       }
    ///     }
    #[prost(btree_map="string, message", tag="33")]
    pub contacts: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ContactDetails>,
    /// Contains compliance information for security standards associated to the
    /// finding.
    #[prost(message, repeated, tag="34")]
    pub compliances: ::prost::alloc::vec::Vec<Compliance>,
    /// Contains more detail about the finding.
    #[prost(string, tag="37")]
    pub description: ::prost::alloc::string::String,
    /// Represents exfiltration associated with the Finding.
    #[prost(message, optional, tag="38")]
    pub exfiltration: ::core::option::Option<Exfiltration>,
    /// Represents IAM bindings associated with the Finding.
    #[prost(message, repeated, tag="39")]
    pub iam_bindings: ::prost::alloc::vec::Vec<IamBinding>,
    /// Next steps associate to the finding.
    #[prost(string, tag="40")]
    pub next_steps: ::prost::alloc::string::String,
    /// Containers associated with the finding. containers provides information
    /// for both Kubernetes and non-Kubernetes containers.
    #[prost(message, repeated, tag="42")]
    pub containers: ::prost::alloc::vec::Vec<Container>,
    /// Kubernetes resources associated with the finding.
    #[prost(message, optional, tag="43")]
    pub kubernetes: ::core::option::Option<Kubernetes>,
    /// Database associated with the finding.
    #[prost(message, optional, tag="44")]
    pub database: ::core::option::Option<Database>,
}
/// Nested message and enum types in `Finding`.
pub mod finding {
    /// The state of the finding.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified state.
        Unspecified = 0,
        /// The finding requires attention and has not been addressed yet.
        Active = 1,
        /// The finding has been fixed, triaged as a non-issue or otherwise addressed
        /// and is no longer active.
        Inactive = 2,
    }
    /// The severity of the finding.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Severity {
        /// This value is used for findings when a source doesn't write a severity
        /// value.
        Unspecified = 0,
        /// Vulnerability:
        /// A critical vulnerability is easily discoverable by an external actor,
        /// exploitable, and results in the direct ability to execute arbitrary code,
        /// exfiltrate data, and otherwise gain additional access and privileges to
        /// cloud resources and workloads. Examples include publicly accessible
        /// unprotected user data, public SSH access with weak or no passwords, etc.
        ///
        /// Threat:
        /// Indicates a threat that is able to access, modify, or delete data or
        /// execute unauthorized code within existing resources.
        Critical = 1,
        /// Vulnerability:
        /// A high risk vulnerability can be easily discovered and exploited in
        /// combination with other vulnerabilities in order to gain direct access and
        /// the ability to execute arbitrary code, exfiltrate data, and otherwise
        /// gain additional access and privileges to cloud resources and workloads.
        /// An example is a database with weak or no passwords that is only
        /// accessible internally. This database could easily be compromised by an
        /// actor that had access to the internal network.
        ///
        /// Threat:
        /// Indicates a threat that is able to create new computational resources in
        /// an environment but not able to access data or execute code in existing
        /// resources.
        High = 2,
        /// Vulnerability:
        /// A medium risk vulnerability could be used by an actor to gain access to
        /// resources or privileges that enable them to eventually (through multiple
        /// steps or a complex exploit) gain access and the ability to execute
        /// arbitrary code or exfiltrate data. An example is a service account with
        /// access to more projects than it should have. If an actor gains access to
        /// the service account, they could potentially use that access to manipulate
        /// a project the service account was not intended to.
        ///
        /// Threat:
        /// Indicates a threat that is able to cause operational impact but may not
        /// access data or execute unauthorized code.
        Medium = 3,
        /// Vulnerability:
        /// A low risk vulnerability hampers a security organization's ability to
        /// detect vulnerabilities or active threats in their deployment, or prevents
        /// the root cause investigation of security issues. An example is monitoring
        /// and logs being disabled for resource configurations and access.
        ///
        /// Threat:
        /// Indicates a threat that has obtained minimal access to an environment but
        /// is not able to access data, execute code, or create resources.
        Low = 4,
    }
    /// Mute state a finding can be in.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Mute {
        /// Unspecified.
        Unspecified = 0,
        /// Finding has been muted.
        Muted = 1,
        /// Finding has been unmuted.
        Unmuted = 2,
        /// Finding has never been muted/unmuted.
        Undefined = 4,
    }
    /// Represents what kind of Finding it is.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FindingClass {
        /// Unspecified finding class.
        Unspecified = 0,
        /// Describes unwanted or malicious activity.
        Threat = 1,
        /// Describes a potential weakness in software that increases risk to
        /// Confidentiality & Integrity & Availability.
        Vulnerability = 2,
        /// Describes a potential weakness in cloud resource/asset configuration that
        /// increases risk.
        Misconfiguration = 3,
        /// Describes a security observation that is for informational purposes.
        Observation = 4,
        /// Describes an error that prevents some SCC functionality.
        SccError = 5,
    }
}
/// Cloud SCC's Notification
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationMessage {
    /// Name of the notification config that generated current notification.
    #[prost(string, tag="1")]
    pub notification_config_name: ::prost::alloc::string::String,
    /// The Cloud resource tied to this notification's Finding.
    #[prost(message, optional, tag="3")]
    pub resource: ::core::option::Option<Resource>,
    /// Notification Event.
    #[prost(oneof="notification_message::Event", tags="2")]
    pub event: ::core::option::Option<notification_message::Event>,
}
/// Nested message and enum types in `NotificationMessage`.
pub mod notification_message {
    /// Notification Event.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        /// If it's a Finding based notification config, this field will be
        /// populated.
        #[prost(message, tag="2")]
        Finding(super::Finding),
    }
}
/// Security Command Center representation of a Google Cloud
/// resource.
///
/// The Asset is a Security Command Center resource that captures information
/// about a single Google Cloud resource. All modifications to an Asset are only
/// within the context of Security Command Center and don't affect the referenced
/// Google Cloud resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    /// The relative resource name of this asset. See:
    /// <https://cloud.google.com/apis/design/resource_names#relative_resource_name>
    /// Example:
    /// "organizations/{organization_id}/assets/{asset_id}".
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Security Command Center managed properties. These properties are managed by
    /// Security Command Center and cannot be modified by the user.
    #[prost(message, optional, tag="2")]
    pub security_center_properties: ::core::option::Option<asset::SecurityCenterProperties>,
    /// Resource managed properties. These properties are managed and defined by
    /// the Google Cloud resource and cannot be modified by the user.
    #[prost(btree_map="string, message", tag="7")]
    pub resource_properties: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost_types::Value>,
    /// User specified security marks. These marks are entirely managed by the user
    /// and come from the SecurityMarks resource that belongs to the asset.
    #[prost(message, optional, tag="8")]
    pub security_marks: ::core::option::Option<SecurityMarks>,
    /// The time at which the asset was created in Security Command Center.
    #[prost(message, optional, tag="9")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time at which the asset was last updated or added in Cloud SCC.
    #[prost(message, optional, tag="10")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Cloud IAM Policy information associated with the Google Cloud resource
    /// described by the Security Command Center asset. This information is managed
    /// and defined by the Google Cloud resource and cannot be modified by the
    /// user.
    #[prost(message, optional, tag="11")]
    pub iam_policy: ::core::option::Option<asset::IamPolicy>,
    /// The canonical name of the resource. It's either
    /// "organizations/{organization_id}/assets/{asset_id}",
    /// "folders/{folder_id}/assets/{asset_id}" or
    /// "projects/{project_number}/assets/{asset_id}", depending on the closest CRM
    /// ancestor of the resource.
    #[prost(string, tag="13")]
    pub canonical_name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Asset`.
pub mod asset {
    /// Security Command Center managed properties. These properties are managed by
    /// Security Command Center and cannot be modified by the user.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SecurityCenterProperties {
        /// The full resource name of the Google Cloud resource this asset
        /// represents. This field is immutable after create time. See:
        /// <https://cloud.google.com/apis/design/resource_names#full_resource_name>
        #[prost(string, tag="1")]
        pub resource_name: ::prost::alloc::string::String,
        /// The type of the Google Cloud resource. Examples include: APPLICATION,
        /// PROJECT, and ORGANIZATION. This is a case insensitive field defined by
        /// Security Command Center and/or the producer of the resource and is
        /// immutable after create time.
        #[prost(string, tag="2")]
        pub resource_type: ::prost::alloc::string::String,
        /// The full resource name of the immediate parent of the resource. See:
        /// <https://cloud.google.com/apis/design/resource_names#full_resource_name>
        #[prost(string, tag="3")]
        pub resource_parent: ::prost::alloc::string::String,
        /// The full resource name of the project the resource belongs to. See:
        /// <https://cloud.google.com/apis/design/resource_names#full_resource_name>
        #[prost(string, tag="4")]
        pub resource_project: ::prost::alloc::string::String,
        /// Owners of the Google Cloud resource.
        #[prost(string, repeated, tag="5")]
        pub resource_owners: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The user defined display name for this resource.
        #[prost(string, tag="6")]
        pub resource_display_name: ::prost::alloc::string::String,
        /// The user defined display name for the parent of this resource.
        #[prost(string, tag="7")]
        pub resource_parent_display_name: ::prost::alloc::string::String,
        /// The user defined display name for the project of this resource.
        #[prost(string, tag="8")]
        pub resource_project_display_name: ::prost::alloc::string::String,
        /// Contains a Folder message for each folder in the assets ancestry.
        /// The first folder is the deepest nested folder, and the last folder is the
        /// folder directly under the Organization.
        #[prost(message, repeated, tag="10")]
        pub folders: ::prost::alloc::vec::Vec<super::Folder>,
    }
    /// Cloud IAM Policy information associated with the Google Cloud resource
    /// described by the Security Command Center asset. This information is managed
    /// and defined by the Google Cloud resource and cannot be modified by the
    /// user.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IamPolicy {
        /// The JSON representation of the Policy associated with the asset.
        /// See <https://cloud.google.com/iam/reference/rest/v1/Policy> for format
        /// details.
        #[prost(string, tag="1")]
        pub policy_blob: ::prost::alloc::string::String,
    }
}
/// User specified settings that are attached to the Security Command
/// Center organization.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrganizationSettings {
    /// The relative resource name of the settings. See:
    /// <https://cloud.google.com/apis/design/resource_names#relative_resource_name>
    /// Example:
    /// "organizations/{organization_id}/organizationSettings".
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// A flag that indicates if Asset Discovery should be enabled. If the flag is
    /// set to `true`, then discovery of assets will occur. If it is set to `false,
    /// all historical assets will remain, but discovery of future assets will not
    /// occur.
    #[prost(bool, tag="2")]
    pub enable_asset_discovery: bool,
    /// The configuration used for Asset Discovery runs.
    #[prost(message, optional, tag="3")]
    pub asset_discovery_config: ::core::option::Option<organization_settings::AssetDiscoveryConfig>,
}
/// Nested message and enum types in `OrganizationSettings`.
pub mod organization_settings {
    /// The configuration used for Asset Discovery runs.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AssetDiscoveryConfig {
        /// The project ids to use for filtering asset discovery.
        #[prost(string, repeated, tag="1")]
        pub project_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The mode to use for filtering asset discovery.
        #[prost(enumeration="asset_discovery_config::InclusionMode", tag="2")]
        pub inclusion_mode: i32,
        /// The folder ids to use for filtering asset discovery.
        /// It consists of only digits, e.g., 756619654966.
        #[prost(string, repeated, tag="3")]
        pub folder_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// Nested message and enum types in `AssetDiscoveryConfig`.
    pub mod asset_discovery_config {
        /// The mode of inclusion when running Asset Discovery.
        /// Asset discovery can be limited by explicitly identifying projects to be
        /// included or excluded. If INCLUDE_ONLY is set, then only those projects
        /// within the organization and their children are discovered during asset
        /// discovery. If EXCLUDE is set, then projects that don't match those
        /// projects are discovered during asset discovery. If neither are set, then
        /// all projects within the organization are discovered during asset
        /// discovery.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum InclusionMode {
            /// Unspecified. Setting the mode with this value will disable
            /// inclusion/exclusion filtering for Asset Discovery.
            Unspecified = 0,
            /// Asset Discovery will capture only the resources within the projects
            /// specified. All other resources will be ignored.
            IncludeOnly = 1,
            /// Asset Discovery will ignore all resources under the projects specified.
            /// All other resources will be retrieved.
            Exclude = 2,
        }
    }
}
/// Request message for bulk findings update.
///
/// Note:
/// 1. If multiple bulk update requests match the same resource, the order in
/// which they get executed is not defined.
/// 2. Once a bulk operation is started, there is no way to stop it.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BulkMuteFindingsRequest {
    /// Required. The parent, at which bulk action needs to be applied. Its format is
    /// "organizations/\[organization_id\]", "folders/\[folder_id\]",
    /// "projects/\[project_id\]".
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Expression that identifies findings that should be updated.
    /// The expression is a list of zero or more restrictions combined
    /// via logical operators `AND` and `OR`. Parentheses are supported, and `OR`
    /// has higher precedence than `AND`.
    ///
    /// Restrictions have the form `<field> <operator> <value>` and may have a
    /// `-` character in front of them to indicate negation. The fields map to
    /// those defined in the corresponding resource.
    ///
    /// The supported operators are:
    ///
    /// * `=` for all value types.
    /// * `>`, `<`, `>=`, `<=` for integer values.
    /// * `:`, meaning substring matching, for strings.
    ///
    /// The supported value types are:
    ///
    /// * string literals in quotes.
    /// * integer literals without quotes.
    /// * boolean literals `true` and `false` without quotes.
    #[prost(string, tag="2")]
    pub filter: ::prost::alloc::string::String,
    /// This can be a mute configuration name or any identifier for mute/unmute
    /// of findings based on the filter.
    #[deprecated]
    #[prost(string, tag="3")]
    pub mute_annotation: ::prost::alloc::string::String,
}
/// The response to a BulkMute request. Contains the LRO information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BulkMuteFindingsResponse {
}
/// Request message for creating a finding.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFindingRequest {
    /// Required. Resource name of the new finding's parent. Its format should be
    /// "organizations/\[organization_id]/sources/[source_id\]".
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Unique identifier provided by the client within the parent scope.
    /// It must be alphanumeric and less than or equal to 32 characters and
    /// greater than 0 characters in length.
    #[prost(string, tag="2")]
    pub finding_id: ::prost::alloc::string::String,
    /// Required. The Finding being created. The name and security_marks will be ignored as
    /// they are both output only fields on this resource.
    #[prost(message, optional, tag="3")]
    pub finding: ::core::option::Option<Finding>,
}
/// Request message for creating a mute config.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMuteConfigRequest {
    /// Required. Resource name of the new mute configs's parent. Its format is
    /// "organizations/\[organization_id\]", "folders/\[folder_id\]", or
    /// "projects/\[project_id\]".
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The mute config being created.
    #[prost(message, optional, tag="2")]
    pub mute_config: ::core::option::Option<MuteConfig>,
    /// Required. Unique identifier provided by the client within the parent scope.
    /// It must consist of lower case letters, numbers, and hyphen, with the first
    /// character a letter, the last a letter or a number, and a 63 character
    /// maximum.
    #[prost(string, tag="3")]
    pub mute_config_id: ::prost::alloc::string::String,
}
/// Request message for creating a notification config.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNotificationConfigRequest {
    /// Required. Resource name of the new notification config's parent. Its format is
    /// "organizations/\[organization_id\]".
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required.
    /// Unique identifier provided by the client within the parent scope.
    /// It must be between 1 and 128 characters, and contains alphanumeric
    /// characters, underscores or hyphens only.
    #[prost(string, tag="2")]
    pub config_id: ::prost::alloc::string::String,
    /// Required. The notification config being created. The name and the service account
    /// will be ignored as they are both output only fields on this resource.
    #[prost(message, optional, tag="3")]
    pub notification_config: ::core::option::Option<NotificationConfig>,
}
/// Request message for creating a source.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSourceRequest {
    /// Required. Resource name of the new source's parent. Its format should be
    /// "organizations/\[organization_id\]".
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Source being created, only the display_name and description will be
    /// used. All other fields will be ignored.
    #[prost(message, optional, tag="2")]
    pub source: ::core::option::Option<Source>,
}
/// Request message for deleting a mute config.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMuteConfigRequest {
    /// Required. Name of the mute config to delete. Its format is
    /// organizations/{organization}/muteConfigs/{config_id},
    /// folders/{folder}/muteConfigs/{config_id}, or
    /// projects/{project}/muteConfigs/{config_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for deleting a notification config.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteNotificationConfigRequest {
    /// Required. Name of the notification config to delete. Its format is
    /// "organizations/\[organization_id]/notificationConfigs/[config_id\]".
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for retrieving a big query export.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBigQueryExportRequest {
    /// Required. Name of the big query export to retrieve. Its format is
    /// organizations/{organization}/bigQueryExports/{export_id},
    /// folders/{folder}/bigQueryExports/{export_id}, or
    /// projects/{project}/bigQueryExports/{export_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for retrieving a mute config.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMuteConfigRequest {
    /// Required. Name of the mute config to retrieve. Its format is
    /// organizations/{organization}/muteConfigs/{config_id},
    /// folders/{folder}/muteConfigs/{config_id}, or
    /// projects/{project}/muteConfigs/{config_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for getting a notification config.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNotificationConfigRequest {
    /// Required. Name of the notification config to get. Its format is
    /// "organizations/\[organization_id]/notificationConfigs/[config_id\]".
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for getting organization settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrganizationSettingsRequest {
    /// Required. Name of the organization to get organization settings for. Its format is
    /// "organizations/\[organization_id\]/organizationSettings".
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for getting a source.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSourceRequest {
    /// Required. Relative resource name of the source. Its format is
    /// "organizations/\[organization_id]/source/[source_id\]".
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for grouping by assets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAssetsRequest {
    /// Required. Name of the organization to groupBy. Its format is
    /// "organizations/\[organization_id\], folders/\[folder_id\], or
    /// projects/\[project_id\]".
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Expression that defines the filter to apply across assets.
    /// The expression is a list of zero or more restrictions combined via logical
    /// operators `AND` and `OR`.
    /// Parentheses are supported, and `OR` has higher precedence than `AND`.
    ///
    /// Restrictions have the form `<field> <operator> <value>` and may have a `-`
    /// character in front of them to indicate negation. The fields map to those
    /// defined in the Asset resource. Examples include:
    ///
    /// * name
    /// * security_center_properties.resource_name
    /// * resource_properties.a_property
    /// * security_marks.marks.marka
    ///
    /// The supported operators are:
    ///
    /// * `=` for all value types.
    /// * `>`, `<`, `>=`, `<=` for integer values.
    /// * `:`, meaning substring matching, for strings.
    ///
    /// The supported value types are:
    ///
    /// * string literals in quotes.
    /// * integer literals without quotes.
    /// * boolean literals `true` and `false` without quotes.
    ///
    /// The following field and operator combinations are supported:
    ///
    /// * name: `=`
    /// * update_time: `=`, `>`, `<`, `>=`, `<=`
    ///
    ///   Usage: This should be milliseconds since epoch or an RFC3339 string.
    ///   Examples:
    ///     `update_time = "2019-06-10T16:07:18-07:00"`
    ///     `update_time = 1560208038000`
    ///
    /// * create_time: `=`, `>`, `<`, `>=`, `<=`
    ///
    ///   Usage: This should be milliseconds since epoch or an RFC3339 string.
    ///   Examples:
    ///     `create_time = "2019-06-10T16:07:18-07:00"`
    ///     `create_time = 1560208038000`
    ///
    /// * iam_policy.policy_blob: `=`, `:`
    /// * resource_properties: `=`, `:`, `>`, `<`, `>=`, `<=`
    /// * security_marks.marks: `=`, `:`
    /// * security_center_properties.resource_name: `=`, `:`
    /// * security_center_properties.resource_display_name: `=`, `:`
    /// * security_center_properties.resource_type: `=`, `:`
    /// * security_center_properties.resource_parent: `=`, `:`
    /// * security_center_properties.resource_parent_display_name: `=`, `:`
    /// * security_center_properties.resource_project: `=`, `:`
    /// * security_center_properties.resource_project_display_name: `=`, `:`
    /// * security_center_properties.resource_owners: `=`, `:`
    ///
    /// For example, `resource_properties.size = 100` is a valid filter string.
    ///
    /// Use a partial match on the empty string to filter based on a property
    /// existing: `resource_properties.my_property : ""`
    ///
    /// Use a negated partial match on the empty string to filter based on a
    /// property not existing: `-resource_properties.my_property : ""`
    #[prost(string, tag="2")]
    pub filter: ::prost::alloc::string::String,
    /// Required. Expression that defines what assets fields to use for grouping. The string
    /// value should follow SQL syntax: comma separated list of fields. For
    /// example:
    /// "security_center_properties.resource_project,security_center_properties.project".
    ///
    /// The following fields are supported when compare_duration is not set:
    ///
    /// * security_center_properties.resource_project
    /// * security_center_properties.resource_project_display_name
    /// * security_center_properties.resource_type
    /// * security_center_properties.resource_parent
    /// * security_center_properties.resource_parent_display_name
    ///
    /// The following fields are supported when compare_duration is set:
    ///
    /// * security_center_properties.resource_type
    /// * security_center_properties.resource_project_display_name
    /// * security_center_properties.resource_parent_display_name
    #[prost(string, tag="3")]
    pub group_by: ::prost::alloc::string::String,
    /// When compare_duration is set, the GroupResult's "state_change" property is
    /// updated to indicate whether the asset was added, removed, or remained
    /// present during the compare_duration period of time that precedes the
    /// read_time. This is the time between (read_time - compare_duration) and
    /// read_time.
    ///
    /// The state change value is derived based on the presence of the asset at the
    /// two points in time. Intermediate state changes between the two times don't
    /// affect the result. For example, the results aren't affected if the asset is
    /// removed and re-created again.
    ///
    /// Possible "state_change" values when compare_duration is specified:
    ///
    /// * "ADDED":   indicates that the asset was not present at the start of
    ///                compare_duration, but present at reference_time.
    /// * "REMOVED": indicates that the asset was present at the start of
    ///                compare_duration, but not present at reference_time.
    /// * "ACTIVE":  indicates that the asset was present at both the
    ///                start and the end of the time period defined by
    ///                compare_duration and reference_time.
    ///
    /// If compare_duration is not specified, then the only possible state_change
    /// is "UNUSED", which will be the state_change set for all assets present at
    /// read_time.
    ///
    /// If this field is set then `state_change` must be a specified field in
    /// `group_by`.
    #[prost(message, optional, tag="4")]
    pub compare_duration: ::core::option::Option<::prost_types::Duration>,
    /// Time used as a reference point when filtering assets. The filter is limited
    /// to assets existing at the supplied time and their values are those at that
    /// specific time. Absence of this field will default to the API's version of
    /// NOW.
    #[prost(message, optional, tag="5")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The value returned by the last `GroupAssetsResponse`; indicates
    /// that this is a continuation of a prior `GroupAssets` call, and that the
    /// system should return the next page of data.
    #[prost(string, tag="7")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single response. Default is
    /// 10, minimum is 1, maximum is 1000.
    #[prost(int32, tag="8")]
    pub page_size: i32,
}
/// Response message for grouping by assets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAssetsResponse {
    /// Group results. There exists an element for each existing unique
    /// combination of property/values. The element contains a count for the number
    /// of times those specific property/values appear.
    #[prost(message, repeated, tag="1")]
    pub group_by_results: ::prost::alloc::vec::Vec<GroupResult>,
    /// Time used for executing the groupBy request.
    #[prost(message, optional, tag="2")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results.
    #[prost(string, tag="3")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The total number of results matching the query.
    #[prost(int32, tag="4")]
    pub total_size: i32,
}
/// Request message for grouping by findings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupFindingsRequest {
    /// Required. Name of the source to groupBy. Its format is
    /// "organizations/\[organization_id]/sources/[source_id\]",
    /// folders/\[folder_id]/sources/[source_id\], or
    /// projects/\[project_id]/sources/[source_id\]. To groupBy across all sources
    /// provide a source_id of `-`. For example:
    /// organizations/{organization_id}/sources/-, folders/{folder_id}/sources/-,
    /// or projects/{project_id}/sources/-
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Expression that defines the filter to apply across findings.
    /// The expression is a list of one or more restrictions combined via logical
    /// operators `AND` and `OR`.
    /// Parentheses are supported, and `OR` has higher precedence than `AND`.
    ///
    /// Restrictions have the form `<field> <operator> <value>` and may have a `-`
    /// character in front of them to indicate negation. Examples include:
    ///
    ///  * name
    ///  * source_properties.a_property
    ///  * security_marks.marks.marka
    ///
    /// The supported operators are:
    ///
    /// * `=` for all value types.
    /// * `>`, `<`, `>=`, `<=` for integer values.
    /// * `:`, meaning substring matching, for strings.
    ///
    /// The supported value types are:
    ///
    /// * string literals in quotes.
    /// * integer literals without quotes.
    /// * boolean literals `true` and `false` without quotes.
    ///
    /// The following field and operator combinations are supported:
    ///
    /// * name: `=`
    /// * parent: `=`, `:`
    /// * resource_name: `=`, `:`
    /// * state: `=`, `:`
    /// * category: `=`, `:`
    /// * external_uri: `=`, `:`
    /// * event_time: `=`, `>`, `<`, `>=`, `<=`
    ///
    ///   Usage: This should be milliseconds since epoch or an RFC3339 string.
    ///   Examples:
    ///     `event_time = "2019-06-10T16:07:18-07:00"`
    ///     `event_time = 1560208038000`
    ///
    /// * severity: `=`, `:`
    /// * workflow_state: `=`, `:`
    /// * security_marks.marks: `=`, `:`
    /// * source_properties: `=`, `:`, `>`, `<`, `>=`, `<=`
    ///
    ///   For example, `source_properties.size = 100` is a valid filter string.
    ///
    ///   Use a partial match on the empty string to filter based on a property
    ///   existing: `source_properties.my_property : ""`
    ///
    ///   Use a negated partial match on the empty string to filter based on a
    ///   property not existing: `-source_properties.my_property : ""`
    ///
    /// * resource:
    ///   * resource.name: `=`, `:`
    ///   * resource.parent_name: `=`, `:`
    ///   * resource.parent_display_name: `=`, `:`
    ///   * resource.project_name: `=`, `:`
    ///   * resource.project_display_name: `=`, `:`
    ///   * resource.type: `=`, `:`
    #[prost(string, tag="2")]
    pub filter: ::prost::alloc::string::String,
    /// Required. Expression that defines what assets fields to use for grouping (including
    /// `state_change`). The string value should follow SQL syntax: comma separated
    /// list of fields. For example: "parent,resource_name".
    ///
    /// The following fields are supported:
    ///
    /// * resource_name
    /// * category
    /// * state
    /// * parent
    /// * severity
    ///
    /// The following fields are supported when compare_duration is set:
    ///
    /// * state_change
    #[prost(string, tag="3")]
    pub group_by: ::prost::alloc::string::String,
    /// Time used as a reference point when filtering findings. The filter is
    /// limited to findings existing at the supplied time and their values are
    /// those at that specific time. Absence of this field will default to the
    /// API's version of NOW.
    #[prost(message, optional, tag="4")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
    /// When compare_duration is set, the GroupResult's "state_change" attribute is
    /// updated to indicate whether the finding had its state changed, the
    /// finding's state remained unchanged, or if the finding was added during the
    /// compare_duration period of time that precedes the read_time. This is the
    /// time between (read_time - compare_duration) and read_time.
    ///
    /// The state_change value is derived based on the presence and state of the
    /// finding at the two points in time. Intermediate state changes between the
    /// two times don't affect the result. For example, the results aren't affected
    /// if the finding is made inactive and then active again.
    ///
    /// Possible "state_change" values when compare_duration is specified:
    ///
    /// * "CHANGED":   indicates that the finding was present and matched the given
    ///                  filter at the start of compare_duration, but changed its
    ///                  state at read_time.
    /// * "UNCHANGED": indicates that the finding was present and matched the given
    ///                  filter at the start of compare_duration and did not change
    ///                  state at read_time.
    /// * "ADDED":     indicates that the finding did not match the given filter or
    ///                  was not present at the start of compare_duration, but was
    ///                  present at read_time.
    /// * "REMOVED":   indicates that the finding was present and matched the
    ///                  filter at the start of compare_duration, but did not match
    ///                  the filter at read_time.
    ///
    /// If compare_duration is not specified, then the only possible state_change
    /// is "UNUSED",  which will be the state_change set for all findings present
    /// at read_time.
    ///
    /// If this field is set then `state_change` must be a specified field in
    /// `group_by`.
    #[prost(message, optional, tag="5")]
    pub compare_duration: ::core::option::Option<::prost_types::Duration>,
    /// The value returned by the last `GroupFindingsResponse`; indicates
    /// that this is a continuation of a prior `GroupFindings` call, and
    /// that the system should return the next page of data.
    #[prost(string, tag="7")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single response. Default is
    /// 10, minimum is 1, maximum is 1000.
    #[prost(int32, tag="8")]
    pub page_size: i32,
}
/// Response message for group by findings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupFindingsResponse {
    /// Group results. There exists an element for each existing unique
    /// combination of property/values. The element contains a count for the number
    /// of times those specific property/values appear.
    #[prost(message, repeated, tag="1")]
    pub group_by_results: ::prost::alloc::vec::Vec<GroupResult>,
    /// Time used for executing the groupBy request.
    #[prost(message, optional, tag="2")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results.
    #[prost(string, tag="3")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The total number of results matching the query.
    #[prost(int32, tag="4")]
    pub total_size: i32,
}
/// Result containing the properties and count of a groupBy request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupResult {
    /// Properties matching the groupBy fields in the request.
    #[prost(btree_map="string, message", tag="1")]
    pub properties: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost_types::Value>,
    /// Total count of resources for the given properties.
    #[prost(int64, tag="2")]
    pub count: i64,
}
/// Request message for listing  mute configs at a given scope e.g. organization,
/// folder or project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMuteConfigsRequest {
    /// Required. The parent, which owns the collection of mute configs. Its format is
    /// "organizations/\[organization_id\]", "folders/\[folder_id\]",
    /// "projects/\[project_id\]".
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of configs to return. The service may return fewer than
    /// this value.
    /// If unspecified, at most 10 configs will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListMuteConfigs` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListMuteConfigs` must
    /// match the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for listing mute configs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMuteConfigsResponse {
    /// The mute configs from the specified parent.
    #[prost(message, repeated, tag="1")]
    pub mute_configs: ::prost::alloc::vec::Vec<MuteConfig>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for listing notification configs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotificationConfigsRequest {
    /// Required. Name of the organization to list notification configs.
    /// Its format is "organizations/\[organization_id\]".
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The value returned by the last `ListNotificationConfigsResponse`; indicates
    /// that this is a continuation of a prior `ListNotificationConfigs` call, and
    /// that the system should return the next page of data.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single response. Default is
    /// 10, minimum is 1, maximum is 1000.
    #[prost(int32, tag="3")]
    pub page_size: i32,
}
/// Response message for listing notification configs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotificationConfigsResponse {
    /// Notification configs belonging to the requested parent.
    #[prost(message, repeated, tag="1")]
    pub notification_configs: ::prost::alloc::vec::Vec<NotificationConfig>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for listing sources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSourcesRequest {
    /// Required. Resource name of the parent of sources to list. Its format should be
    /// "organizations/\[organization_id\], folders/\[folder_id\], or
    /// projects/\[project_id\]".
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The value returned by the last `ListSourcesResponse`; indicates
    /// that this is a continuation of a prior `ListSources` call, and
    /// that the system should return the next page of data.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single response. Default is
    /// 10, minimum is 1, maximum is 1000.
    #[prost(int32, tag="7")]
    pub page_size: i32,
}
/// Response message for listing sources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSourcesResponse {
    /// Sources belonging to the requested parent.
    #[prost(message, repeated, tag="1")]
    pub sources: ::prost::alloc::vec::Vec<Source>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for listing assets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAssetsRequest {
    /// Required. Name of the organization assets should belong to. Its format is
    /// "organizations/\[organization_id\], folders/\[folder_id\], or
    /// projects/\[project_id\]".
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Expression that defines the filter to apply across assets.
    /// The expression is a list of zero or more restrictions combined via logical
    /// operators `AND` and `OR`.
    /// Parentheses are supported, and `OR` has higher precedence than `AND`.
    ///
    /// Restrictions have the form `<field> <operator> <value>` and may have a `-`
    /// character in front of them to indicate negation. The fields map to those
    /// defined in the Asset resource. Examples include:
    ///
    /// * name
    /// * security_center_properties.resource_name
    /// * resource_properties.a_property
    /// * security_marks.marks.marka
    ///
    /// The supported operators are:
    ///
    /// * `=` for all value types.
    /// * `>`, `<`, `>=`, `<=` for integer values.
    /// * `:`, meaning substring matching, for strings.
    ///
    /// The supported value types are:
    ///
    /// * string literals in quotes.
    /// * integer literals without quotes.
    /// * boolean literals `true` and `false` without quotes.
    ///
    /// The following are the allowed field and operator combinations:
    ///
    /// * name: `=`
    /// * update_time: `=`, `>`, `<`, `>=`, `<=`
    ///
    ///   Usage: This should be milliseconds since epoch or an RFC3339 string.
    ///   Examples:
    ///     `update_time = "2019-06-10T16:07:18-07:00"`
    ///     `update_time = 1560208038000`
    ///
    /// * create_time: `=`, `>`, `<`, `>=`, `<=`
    ///
    ///   Usage: This should be milliseconds since epoch or an RFC3339 string.
    ///   Examples:
    ///     `create_time = "2019-06-10T16:07:18-07:00"`
    ///     `create_time = 1560208038000`
    ///
    /// * iam_policy.policy_blob: `=`, `:`
    /// * resource_properties: `=`, `:`, `>`, `<`, `>=`, `<=`
    /// * security_marks.marks: `=`, `:`
    /// * security_center_properties.resource_name: `=`, `:`
    /// * security_center_properties.resource_display_name: `=`, `:`
    /// * security_center_properties.resource_type: `=`, `:`
    /// * security_center_properties.resource_parent: `=`, `:`
    /// * security_center_properties.resource_parent_display_name: `=`, `:`
    /// * security_center_properties.resource_project: `=`, `:`
    /// * security_center_properties.resource_project_display_name: `=`, `:`
    /// * security_center_properties.resource_owners: `=`, `:`
    ///
    /// For example, `resource_properties.size = 100` is a valid filter string.
    ///
    /// Use a partial match on the empty string to filter based on a property
    /// existing: `resource_properties.my_property : ""`
    ///
    /// Use a negated partial match on the empty string to filter based on a
    /// property not existing: `-resource_properties.my_property : ""`
    #[prost(string, tag="2")]
    pub filter: ::prost::alloc::string::String,
    /// Expression that defines what fields and order to use for sorting. The
    /// string value should follow SQL syntax: comma separated list of fields. For
    /// example: "name,resource_properties.a_property". The default sorting order
    /// is ascending. To specify descending order for a field, a suffix " desc"
    /// should be appended to the field name. For example: "name
    /// desc,resource_properties.a_property". Redundant space characters in the
    /// syntax are insignificant. "name desc,resource_properties.a_property" and "
    /// name     desc  ,   resource_properties.a_property  " are equivalent.
    ///
    /// The following fields are supported:
    /// name
    /// update_time
    /// resource_properties
    /// security_marks.marks
    /// security_center_properties.resource_name
    /// security_center_properties.resource_display_name
    /// security_center_properties.resource_parent
    /// security_center_properties.resource_parent_display_name
    /// security_center_properties.resource_project
    /// security_center_properties.resource_project_display_name
    /// security_center_properties.resource_type
    #[prost(string, tag="3")]
    pub order_by: ::prost::alloc::string::String,
    /// Time used as a reference point when filtering assets. The filter is limited
    /// to assets existing at the supplied time and their values are those at that
    /// specific time. Absence of this field will default to the API's version of
    /// NOW.
    #[prost(message, optional, tag="4")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
    /// When compare_duration is set, the ListAssetsResult's "state_change"
    /// attribute is updated to indicate whether the asset was added, removed, or
    /// remained present during the compare_duration period of time that precedes
    /// the read_time. This is the time between (read_time - compare_duration) and
    /// read_time.
    ///
    /// The state_change value is derived based on the presence of the asset at the
    /// two points in time. Intermediate state changes between the two times don't
    /// affect the result. For example, the results aren't affected if the asset is
    /// removed and re-created again.
    ///
    /// Possible "state_change" values when compare_duration is specified:
    ///
    /// * "ADDED":   indicates that the asset was not present at the start of
    ///                compare_duration, but present at read_time.
    /// * "REMOVED": indicates that the asset was present at the start of
    ///                compare_duration, but not present at read_time.
    /// * "ACTIVE":  indicates that the asset was present at both the
    ///                start and the end of the time period defined by
    ///                compare_duration and read_time.
    ///
    /// If compare_duration is not specified, then the only possible state_change
    /// is "UNUSED",  which will be the state_change set for all assets present at
    /// read_time.
    #[prost(message, optional, tag="5")]
    pub compare_duration: ::core::option::Option<::prost_types::Duration>,
    /// A field mask to specify the ListAssetsResult fields to be listed in the
    /// response.
    /// An empty field mask will list all fields.
    #[prost(message, optional, tag="7")]
    pub field_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The value returned by the last `ListAssetsResponse`; indicates
    /// that this is a continuation of a prior `ListAssets` call, and
    /// that the system should return the next page of data.
    #[prost(string, tag="8")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single response. Default is
    /// 10, minimum is 1, maximum is 1000.
    #[prost(int32, tag="9")]
    pub page_size: i32,
}
/// Response message for listing assets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAssetsResponse {
    /// Assets matching the list request.
    #[prost(message, repeated, tag="1")]
    pub list_assets_results: ::prost::alloc::vec::Vec<list_assets_response::ListAssetsResult>,
    /// Time used for executing the list request.
    #[prost(message, optional, tag="2")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results.
    #[prost(string, tag="3")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The total number of assets matching the query.
    #[prost(int32, tag="4")]
    pub total_size: i32,
}
/// Nested message and enum types in `ListAssetsResponse`.
pub mod list_assets_response {
    /// Result containing the Asset and its State.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ListAssetsResult {
        /// Asset matching the search request.
        #[prost(message, optional, tag="1")]
        pub asset: ::core::option::Option<super::Asset>,
        /// State change of the asset between the points in time.
        #[prost(enumeration="list_assets_result::StateChange", tag="2")]
        pub state_change: i32,
    }
    /// Nested message and enum types in `ListAssetsResult`.
    pub mod list_assets_result {
        /// The change in state of the asset.
        ///
        /// When querying across two points in time this describes
        /// the change between the two points: ADDED, REMOVED, or ACTIVE.
        /// If there was no compare_duration supplied in the request the state change
        /// will be: UNUSED
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum StateChange {
            /// State change is unused, this is the canonical default for this enum.
            Unused = 0,
            /// Asset was added between the points in time.
            Added = 1,
            /// Asset was removed between the points in time.
            Removed = 2,
            /// Asset was present at both point(s) in time.
            Active = 3,
        }
    }
}
/// Request message for listing findings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFindingsRequest {
    /// Required. Name of the source the findings belong to. Its format is
    /// "organizations/\[organization_id]/sources/[source_id\],
    /// folders/\[folder_id]/sources/[source_id\], or
    /// projects/\[project_id]/sources/[source_id\]". To list across all sources
    /// provide a source_id of `-`. For example:
    /// organizations/{organization_id}/sources/-, folders/{folder_id}/sources/- or
    /// projects/{projects_id}/sources/-
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Expression that defines the filter to apply across findings.
    /// The expression is a list of one or more restrictions combined via logical
    /// operators `AND` and `OR`.
    /// Parentheses are supported, and `OR` has higher precedence than `AND`.
    ///
    /// Restrictions have the form `<field> <operator> <value>` and may have a `-`
    /// character in front of them to indicate negation. Examples include:
    ///
    ///  * name
    ///  * source_properties.a_property
    ///  * security_marks.marks.marka
    ///
    /// The supported operators are:
    ///
    /// * `=` for all value types.
    /// * `>`, `<`, `>=`, `<=` for integer values.
    /// * `:`, meaning substring matching, for strings.
    ///
    /// The supported value types are:
    ///
    /// * string literals in quotes.
    /// * integer literals without quotes.
    /// * boolean literals `true` and `false` without quotes.
    ///
    /// The following field and operator combinations are supported:
    ///
    /// * name: `=`
    /// * parent: `=`, `:`
    /// * resource_name: `=`, `:`
    /// * state: `=`, `:`
    /// * category: `=`, `:`
    /// * external_uri: `=`, `:`
    /// * event_time: `=`, `>`, `<`, `>=`, `<=`
    ///
    ///   Usage: This should be milliseconds since epoch or an RFC3339 string.
    ///   Examples:
    ///     `event_time = "2019-06-10T16:07:18-07:00"`
    ///     `event_time = 1560208038000`
    ///
    /// * severity: `=`, `:`
    /// * workflow_state: `=`, `:`
    /// * security_marks.marks: `=`, `:`
    /// * source_properties: `=`, `:`, `>`, `<`, `>=`, `<=`
    ///
    ///   For example, `source_properties.size = 100` is a valid filter string.
    ///
    ///   Use a partial match on the empty string to filter based on a property
    ///   existing: `source_properties.my_property : ""`
    ///
    ///   Use a negated partial match on the empty string to filter based on a
    ///   property not existing: `-source_properties.my_property : ""`
    ///
    /// * resource:
    ///   * resource.name: `=`, `:`
    ///   * resource.parent_name: `=`, `:`
    ///   * resource.parent_display_name: `=`, `:`
    ///   * resource.project_name: `=`, `:`
    ///   * resource.project_display_name: `=`, `:`
    ///   * resource.type: `=`, `:`
    ///   * resource.folders.resource_folder: `=`, `:`
    ///   * resource.display_name: `=`, `:`
    #[prost(string, tag="2")]
    pub filter: ::prost::alloc::string::String,
    /// Expression that defines what fields and order to use for sorting. The
    /// string value should follow SQL syntax: comma separated list of fields. For
    /// example: "name,resource_properties.a_property". The default sorting order
    /// is ascending. To specify descending order for a field, a suffix " desc"
    /// should be appended to the field name. For example: "name
    /// desc,source_properties.a_property". Redundant space characters in the
    /// syntax are insignificant. "name desc,source_properties.a_property" and "
    /// name     desc  ,   source_properties.a_property  " are equivalent.
    ///
    /// The following fields are supported:
    /// name
    /// parent
    /// state
    /// category
    /// resource_name
    /// event_time
    /// source_properties
    /// security_marks.marks
    #[prost(string, tag="3")]
    pub order_by: ::prost::alloc::string::String,
    /// Time used as a reference point when filtering findings. The filter is
    /// limited to findings existing at the supplied time and their values are
    /// those at that specific time. Absence of this field will default to the
    /// API's version of NOW.
    #[prost(message, optional, tag="4")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
    /// When compare_duration is set, the ListFindingsResult's "state_change"
    /// attribute is updated to indicate whether the finding had its state changed,
    /// the finding's state remained unchanged, or if the finding was added in any
    /// state during the compare_duration period of time that precedes the
    /// read_time. This is the time between (read_time - compare_duration) and
    /// read_time.
    ///
    /// The state_change value is derived based on the presence and state of the
    /// finding at the two points in time. Intermediate state changes between the
    /// two times don't affect the result. For example, the results aren't affected
    /// if the finding is made inactive and then active again.
    ///
    /// Possible "state_change" values when compare_duration is specified:
    ///
    /// * "CHANGED":   indicates that the finding was present and matched the given
    ///                  filter at the start of compare_duration, but changed its
    ///                  state at read_time.
    /// * "UNCHANGED": indicates that the finding was present and matched the given
    ///                  filter at the start of compare_duration and did not change
    ///                  state at read_time.
    /// * "ADDED":     indicates that the finding did not match the given filter or
    ///                  was not present at the start of compare_duration, but was
    ///                  present at read_time.
    /// * "REMOVED":   indicates that the finding was present and matched the
    ///                  filter at the start of compare_duration, but did not match
    ///                  the filter at read_time.
    ///
    /// If compare_duration is not specified, then the only possible state_change
    /// is "UNUSED", which will be the state_change set for all findings present at
    /// read_time.
    #[prost(message, optional, tag="5")]
    pub compare_duration: ::core::option::Option<::prost_types::Duration>,
    /// A field mask to specify the Finding fields to be listed in the response.
    /// An empty field mask will list all fields.
    #[prost(message, optional, tag="7")]
    pub field_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The value returned by the last `ListFindingsResponse`; indicates
    /// that this is a continuation of a prior `ListFindings` call, and
    /// that the system should return the next page of data.
    #[prost(string, tag="8")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single response. Default is
    /// 10, minimum is 1, maximum is 1000.
    #[prost(int32, tag="9")]
    pub page_size: i32,
}
/// Response message for listing findings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFindingsResponse {
    /// Findings matching the list request.
    #[prost(message, repeated, tag="1")]
    pub list_findings_results: ::prost::alloc::vec::Vec<list_findings_response::ListFindingsResult>,
    /// Time used for executing the list request.
    #[prost(message, optional, tag="2")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results.
    #[prost(string, tag="3")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The total number of findings matching the query.
    #[prost(int32, tag="4")]
    pub total_size: i32,
}
/// Nested message and enum types in `ListFindingsResponse`.
pub mod list_findings_response {
    /// Result containing the Finding and its StateChange.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ListFindingsResult {
        /// Finding matching the search request.
        #[prost(message, optional, tag="1")]
        pub finding: ::core::option::Option<super::Finding>,
        /// State change of the finding between the points in time.
        #[prost(enumeration="list_findings_result::StateChange", tag="2")]
        pub state_change: i32,
        /// Output only. Resource that is associated with this finding.
        #[prost(message, optional, tag="3")]
        pub resource: ::core::option::Option<list_findings_result::Resource>,
    }
    /// Nested message and enum types in `ListFindingsResult`.
    pub mod list_findings_result {
        /// Information related to the Google Cloud resource that is
        /// associated with this finding.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Resource {
            /// The full resource name of the resource. See:
            /// <https://cloud.google.com/apis/design/resource_names#full_resource_name>
            #[prost(string, tag="1")]
            pub name: ::prost::alloc::string::String,
            /// The human readable name of the resource.
            #[prost(string, tag="8")]
            pub display_name: ::prost::alloc::string::String,
            /// The full resource type of the resource.
            #[prost(string, tag="6")]
            pub r#type: ::prost::alloc::string::String,
            /// The full resource name of project that the resource belongs to.
            #[prost(string, tag="2")]
            pub project_name: ::prost::alloc::string::String,
            /// The project ID that the resource belongs to.
            #[prost(string, tag="3")]
            pub project_display_name: ::prost::alloc::string::String,
            /// The full resource name of resource's parent.
            #[prost(string, tag="4")]
            pub parent_name: ::prost::alloc::string::String,
            /// The human readable name of resource's parent.
            #[prost(string, tag="5")]
            pub parent_display_name: ::prost::alloc::string::String,
            /// Contains a Folder message for each folder in the assets ancestry.
            /// The first folder is the deepest nested folder, and the last folder is
            /// the folder directly under the Organization.
            #[prost(message, repeated, tag="7")]
            pub folders: ::prost::alloc::vec::Vec<super::super::Folder>,
        }
        /// The change in state of the finding.
        ///
        /// When querying across two points in time this describes
        /// the change in the finding between the two points: CHANGED, UNCHANGED,
        /// ADDED, or REMOVED. Findings can not be deleted, so REMOVED implies that
        /// the finding at timestamp does not match the filter specified, but it did
        /// at timestamp - compare_duration. If there was no compare_duration
        /// supplied in the request the state change will be: UNUSED
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum StateChange {
            /// State change is unused, this is the canonical default for this enum.
            Unused = 0,
            /// The finding has changed state in some way between the points in time
            /// and existed at both points.
            Changed = 1,
            /// The finding has not changed state between the points in time and
            /// existed at both points.
            Unchanged = 2,
            /// The finding was created between the points in time.
            Added = 3,
            /// The finding at timestamp does not match the filter specified, but it
            /// did at timestamp - compare_duration.
            Removed = 4,
        }
    }
}
/// Request message for updating a finding's state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetFindingStateRequest {
    /// Required. The relative resource name of the finding. See:
    /// <https://cloud.google.com/apis/design/resource_names#relative_resource_name>
    /// Example:
    /// "organizations/{organization_id}/sources/{source_id}/finding/{finding_id}".
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The desired State of the finding.
    #[prost(enumeration="finding::State", tag="2")]
    pub state: i32,
    /// Required. The time at which the updated state takes effect.
    #[prost(message, optional, tag="3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request message for updating a finding's mute status.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMuteRequest {
    /// Required. The relative resource name of the finding. See:
    /// <https://cloud.google.com/apis/design/resource_names#relative_resource_name>
    /// Example:
    /// "organizations/{organization_id}/sources/{source_id}/finding/{finding_id}",
    /// "folders/{folder_id}/sources/{source_id}/finding/{finding_id}",
    /// "projects/{project_id}/sources/{source_id}/finding/{finding_id}".
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The desired state of the Mute.
    #[prost(enumeration="finding::Mute", tag="2")]
    pub mute: i32,
}
/// Request message for running asset discovery for an organization.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunAssetDiscoveryRequest {
    /// Required. Name of the organization to run asset discovery for. Its format is
    /// "organizations/\[organization_id\]".
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
}
/// Request message for updating a ExternalSystem resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateExternalSystemRequest {
    /// Required. The external system resource to update.
    #[prost(message, optional, tag="1")]
    pub external_system: ::core::option::Option<ExternalSystem>,
    /// The FieldMask to use when updating the external system resource.
    ///
    /// If empty all mutable fields will be updated.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for updating or creating a finding.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFindingRequest {
    /// Required. The finding resource to update or create if it does not already exist.
    /// parent, security_marks, and update_time will be ignored.
    ///
    /// In the case of creation, the finding id portion of the name must be
    /// alphanumeric and less than or equal to 32 characters and greater than 0
    /// characters in length.
    #[prost(message, optional, tag="1")]
    pub finding: ::core::option::Option<Finding>,
    /// The FieldMask to use when updating the finding resource. This field should
    /// not be specified when creating a finding.
    ///
    /// When updating a finding, an empty mask is treated as updating all mutable
    /// fields and replacing source_properties.  Individual source_properties can
    /// be added/updated by using "source_properties.<property key>" in the field
    /// mask.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for updating a mute config.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMuteConfigRequest {
    /// Required. The mute config being updated.
    #[prost(message, optional, tag="1")]
    pub mute_config: ::core::option::Option<MuteConfig>,
    /// The list of fields to be updated.
    /// If empty all mutable fields will be updated.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for updating a notification config.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNotificationConfigRequest {
    /// Required. The notification config to update.
    #[prost(message, optional, tag="1")]
    pub notification_config: ::core::option::Option<NotificationConfig>,
    /// The FieldMask to use when updating the notification config.
    ///
    /// If empty all mutable fields will be updated.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for updating an organization's settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOrganizationSettingsRequest {
    /// Required. The organization settings resource to update.
    #[prost(message, optional, tag="1")]
    pub organization_settings: ::core::option::Option<OrganizationSettings>,
    /// The FieldMask to use when updating the settings resource.
    ///
    /// If empty all mutable fields will be updated.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for updating a source.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSourceRequest {
    /// Required. The source resource to update.
    #[prost(message, optional, tag="1")]
    pub source: ::core::option::Option<Source>,
    /// The FieldMask to use when updating the source resource.
    ///
    /// If empty all mutable fields will be updated.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for updating a SecurityMarks resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSecurityMarksRequest {
    /// Required. The security marks resource to update.
    #[prost(message, optional, tag="1")]
    pub security_marks: ::core::option::Option<SecurityMarks>,
    /// The FieldMask to use when updating the security marks resource.
    ///
    /// The field mask must not contain duplicate fields.
    /// If empty or set to "marks", all marks will be replaced.  Individual
    /// marks can be updated using "marks.<mark_key>".
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The time at which the updated SecurityMarks take effect.
    /// If not set uses current server time.  Updates will be applied to the
    /// SecurityMarks that are active immediately preceding this time. Must be
    /// earlier or equal to the server time.
    #[prost(message, optional, tag="3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request message for creating a big query export.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBigQueryExportRequest {
    /// Required. Resource name of the new big query export's parent. Its format is
    /// "organizations/\[organization_id\]", "folders/\[folder_id\]", or
    /// "projects/\[project_id\]".
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The big query export being created.
    #[prost(message, optional, tag="2")]
    pub big_query_export: ::core::option::Option<BigQueryExport>,
    /// Required. Unique identifier provided by the client within the parent scope.
    /// It must consist of lower case letters, numbers, and hyphen, with the first
    /// character a letter, the last a letter or a number, and a 63 character
    /// maximum.
    #[prost(string, tag="3")]
    pub big_query_export_id: ::prost::alloc::string::String,
}
/// Request message for updating a BigQuery export.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBigQueryExportRequest {
    /// Required. The BigQuery export being updated.
    #[prost(message, optional, tag="1")]
    pub big_query_export: ::core::option::Option<BigQueryExport>,
    /// The list of fields to be updated.
    /// If empty all mutable fields will be updated.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for listing  BigQuery exports at a given scope e.g.
/// organization, folder or project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBigQueryExportsRequest {
    /// Required. The parent, which owns the collection of BigQuery exports. Its format is
    /// "organizations/\[organization_id\]", "folders/\[folder_id\]",
    /// "projects/\[project_id\]".
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of configs to return. The service may return fewer than
    /// this value.
    /// If unspecified, at most 10 configs will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListBigQueryExports` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListBigQueryExports`
    /// must match the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for listing BigQuery exports.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBigQueryExportsResponse {
    /// The BigQuery exports from the specified parent.
    #[prost(message, repeated, tag="1")]
    pub big_query_exports: ::prost::alloc::vec::Vec<BigQueryExport>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for deleting a big query export.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBigQueryExportRequest {
    /// Required. Name of the big query export to delete. Its format is
    /// organizations/{organization}/bigQueryExports/{export_id},
    /// folders/{folder}/bigQueryExports/{export_id}, or
    /// projects/{project}/bigQueryExports/{export_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod security_center_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// V1 APIs for Security Center service.
    #[derive(Debug, Clone)]
    pub struct SecurityCenterClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SecurityCenterClient<T>
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
        ) -> SecurityCenterClient<InterceptedService<T, F>>
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
            SecurityCenterClient::new(InterceptedService::new(inner, interceptor))
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
        /// Kicks off an LRO to bulk mute findings for a parent based on a filter. The
        /// parent can be either an organization, folder or project. The findings
        /// matched by the filter will be muted after the LRO is done.
        pub async fn bulk_mute_findings(
            &mut self,
            request: impl tonic::IntoRequest<super::BulkMuteFindingsRequest>,
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
                "/google.cloud.securitycenter.v1.SecurityCenter/BulkMuteFindings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a source.
        pub async fn create_source(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSourceRequest>,
        ) -> Result<tonic::Response<super::Source>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1.SecurityCenter/CreateSource",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a finding. The corresponding source must exist for finding creation
        /// to succeed.
        pub async fn create_finding(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFindingRequest>,
        ) -> Result<tonic::Response<super::Finding>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1.SecurityCenter/CreateFinding",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a mute config.
        pub async fn create_mute_config(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateMuteConfigRequest>,
        ) -> Result<tonic::Response<super::MuteConfig>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1.SecurityCenter/CreateMuteConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a notification config.
        pub async fn create_notification_config(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateNotificationConfigRequest>,
        ) -> Result<tonic::Response<super::NotificationConfig>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1.SecurityCenter/CreateNotificationConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes an existing mute config.
        pub async fn delete_mute_config(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteMuteConfigRequest>,
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
                "/google.cloud.securitycenter.v1.SecurityCenter/DeleteMuteConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a notification config.
        pub async fn delete_notification_config(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteNotificationConfigRequest>,
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
                "/google.cloud.securitycenter.v1.SecurityCenter/DeleteNotificationConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a big query export.
        pub async fn get_big_query_export(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBigQueryExportRequest>,
        ) -> Result<tonic::Response<super::BigQueryExport>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1.SecurityCenter/GetBigQueryExport",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the access control policy on the specified Source.
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.securitycenter.v1.SecurityCenter/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a mute config.
        pub async fn get_mute_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMuteConfigRequest>,
        ) -> Result<tonic::Response<super::MuteConfig>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1.SecurityCenter/GetMuteConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a notification config.
        pub async fn get_notification_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNotificationConfigRequest>,
        ) -> Result<tonic::Response<super::NotificationConfig>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1.SecurityCenter/GetNotificationConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the settings for an organization.
        pub async fn get_organization_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrganizationSettingsRequest>,
        ) -> Result<tonic::Response<super::OrganizationSettings>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1.SecurityCenter/GetOrganizationSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a source.
        pub async fn get_source(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSourceRequest>,
        ) -> Result<tonic::Response<super::Source>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1.SecurityCenter/GetSource",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Filters an organization's assets and  groups them by their specified
        /// properties.
        pub async fn group_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupAssetsRequest>,
        ) -> Result<tonic::Response<super::GroupAssetsResponse>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1.SecurityCenter/GroupAssets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Filters an organization or source's findings and  groups them by their
        /// specified properties.
        ///
        /// To group across all sources provide a `-` as the source id.
        /// Example: /v1/organizations/{organization_id}/sources/-/findings,
        /// /v1/folders/{folder_id}/sources/-/findings,
        /// /v1/projects/{project_id}/sources/-/findings
        pub async fn group_findings(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupFindingsRequest>,
        ) -> Result<tonic::Response<super::GroupFindingsResponse>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1.SecurityCenter/GroupFindings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists an organization's assets.
        pub async fn list_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAssetsRequest>,
        ) -> Result<tonic::Response<super::ListAssetsResponse>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1.SecurityCenter/ListAssets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists an organization or source's findings.
        ///
        /// To list across all sources provide a `-` as the source id.
        /// Example: /v1/organizations/{organization_id}/sources/-/findings
        pub async fn list_findings(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFindingsRequest>,
        ) -> Result<tonic::Response<super::ListFindingsResponse>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1.SecurityCenter/ListFindings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists mute configs.
        pub async fn list_mute_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMuteConfigsRequest>,
        ) -> Result<tonic::Response<super::ListMuteConfigsResponse>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1.SecurityCenter/ListMuteConfigs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists notification configs.
        pub async fn list_notification_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNotificationConfigsRequest>,
        ) -> Result<
            tonic::Response<super::ListNotificationConfigsResponse>,
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
                "/google.cloud.securitycenter.v1.SecurityCenter/ListNotificationConfigs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all sources belonging to an organization.
        pub async fn list_sources(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSourcesRequest>,
        ) -> Result<tonic::Response<super::ListSourcesResponse>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1.SecurityCenter/ListSources",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Runs asset discovery. The discovery is tracked with a long-running
        /// operation.
        ///
        /// This API can only be called with limited frequency for an organization. If
        /// it is called too frequently the caller will receive a TOO_MANY_REQUESTS
        /// error.
        pub async fn run_asset_discovery(
            &mut self,
            request: impl tonic::IntoRequest<super::RunAssetDiscoveryRequest>,
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
                "/google.cloud.securitycenter.v1.SecurityCenter/RunAssetDiscovery",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the state of a finding.
        pub async fn set_finding_state(
            &mut self,
            request: impl tonic::IntoRequest<super::SetFindingStateRequest>,
        ) -> Result<tonic::Response<super::Finding>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1.SecurityCenter/SetFindingState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the mute state of a finding.
        pub async fn set_mute(
            &mut self,
            request: impl tonic::IntoRequest<super::SetMuteRequest>,
        ) -> Result<tonic::Response<super::Finding>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1.SecurityCenter/SetMute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the access control policy on the specified Source.
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.securitycenter.v1.SecurityCenter/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the permissions that a caller has on the specified source.
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::iam::v1::TestIamPermissionsResponse,
            >,
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
                "/google.cloud.securitycenter.v1.SecurityCenter/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates external system. This is for a given finding.
        pub async fn update_external_system(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateExternalSystemRequest>,
        ) -> Result<tonic::Response<super::ExternalSystem>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1.SecurityCenter/UpdateExternalSystem",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates or updates a finding. The corresponding source must exist for a
        /// finding creation to succeed.
        pub async fn update_finding(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFindingRequest>,
        ) -> Result<tonic::Response<super::Finding>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1.SecurityCenter/UpdateFinding",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a mute config.
        pub async fn update_mute_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateMuteConfigRequest>,
        ) -> Result<tonic::Response<super::MuteConfig>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1.SecurityCenter/UpdateMuteConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        /// Updates a notification config. The following update
        /// fields are allowed: description, pubsub_topic, streaming_config.filter
        pub async fn update_notification_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateNotificationConfigRequest>,
        ) -> Result<tonic::Response<super::NotificationConfig>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1.SecurityCenter/UpdateNotificationConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an organization's settings.
        pub async fn update_organization_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateOrganizationSettingsRequest>,
        ) -> Result<tonic::Response<super::OrganizationSettings>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1.SecurityCenter/UpdateOrganizationSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a source.
        pub async fn update_source(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSourceRequest>,
        ) -> Result<tonic::Response<super::Source>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1.SecurityCenter/UpdateSource",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates security marks.
        pub async fn update_security_marks(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSecurityMarksRequest>,
        ) -> Result<tonic::Response<super::SecurityMarks>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1.SecurityCenter/UpdateSecurityMarks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a big query export.
        pub async fn create_big_query_export(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBigQueryExportRequest>,
        ) -> Result<tonic::Response<super::BigQueryExport>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1.SecurityCenter/CreateBigQueryExport",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes an existing big query export.
        pub async fn delete_big_query_export(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBigQueryExportRequest>,
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
                "/google.cloud.securitycenter.v1.SecurityCenter/DeleteBigQueryExport",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a BigQuery export.
        pub async fn update_big_query_export(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBigQueryExportRequest>,
        ) -> Result<tonic::Response<super::BigQueryExport>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1.SecurityCenter/UpdateBigQueryExport",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists BigQuery exports. Note that when requesting BigQuery exports at a
        /// given level all exports under that level are also returned e.g. if
        /// requesting BigQuery exports under a folder, then all BigQuery exports
        /// immediately under the folder plus the ones created under the projects
        /// within the folder are returned.
        pub async fn list_big_query_exports(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBigQueryExportsRequest>,
        ) -> Result<tonic::Response<super::ListBigQueryExportsResponse>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1.SecurityCenter/ListBigQueryExports",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
