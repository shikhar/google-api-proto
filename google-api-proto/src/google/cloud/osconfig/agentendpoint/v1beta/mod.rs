/// Patch configuration specifications. Contains details on how to
/// apply patches to a VM instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchConfig {
    /// Post-patch reboot settings.
    #[prost(enumeration="patch_config::RebootConfig", tag="1")]
    pub reboot_config: i32,
    /// Retry strategy can be defined to have the agent retry patching
    /// during the window if patching fails. If omitted, the agent will use its
    /// default retry strategy.
    #[prost(message, optional, tag="2")]
    pub retry_strategy: ::core::option::Option<RetryStrategy>,
    /// Apt update settings. Use this override the default apt patch rules.
    #[prost(message, optional, tag="3")]
    pub apt: ::core::option::Option<AptSettings>,
    /// Yum update settings. Use this override the default yum patch rules.
    #[prost(message, optional, tag="4")]
    pub yum: ::core::option::Option<YumSettings>,
    /// Goo update settings. Use this override the default goo patch rules.
    #[prost(message, optional, tag="5")]
    pub goo: ::core::option::Option<GooSettings>,
    /// Zypper update settings. Use this override the default zypper patch rules.
    #[prost(message, optional, tag="6")]
    pub zypper: ::core::option::Option<ZypperSettings>,
    /// Windows update settings. Use this override the default windows patch rules.
    #[prost(message, optional, tag="7")]
    pub windows_update: ::core::option::Option<WindowsUpdateSettings>,
    /// The ExecStep to run before the patch update.
    #[prost(message, optional, tag="8")]
    pub pre_step: ::core::option::Option<ExecStep>,
    /// The ExecStep to run after the patch update.
    #[prost(message, optional, tag="9")]
    pub post_step: ::core::option::Option<ExecStep>,
    /// Allows the patch job to run on Managed instance groups (MIGs).
    #[prost(bool, tag="10")]
    pub mig_instances_allowed: bool,
}
/// Nested message and enum types in `PatchConfig`.
pub mod patch_config {
    /// Post-patch reboot settings.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RebootConfig {
        /// The default behavior is DEFAULT.
        Unspecified = 0,
        /// The agent decides if a reboot is necessary by checking
        /// signals such as registry keys on Windows or `/var/run/reboot-required` on
        /// APT based systems. On RPM based systems, a set of core system package
        /// install times are compared with system boot time.
        Default = 1,
        /// Always reboot the machine after the update completes.
        Always = 2,
        /// Never reboot the machine after the update completes.
        Never = 3,
    }
}
/// Apt patching will be performed by executing `apt-get update && apt-get
/// upgrade`. Additional options can be set to control how this is executed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AptSettings {
    /// By changing the type to DIST, the patching will be performed
    /// using `apt-get dist-upgrade` instead.
    #[prost(enumeration="apt_settings::Type", tag="1")]
    pub r#type: i32,
    /// List of packages to exclude from update.
    #[prost(string, repeated, tag="2")]
    pub excludes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// An exclusive list of packages to be updated. These are the only packages
    /// that will be updated. If these packages are not installed, they will be
    /// ignored. This field cannot be specified with any other patch configuration
    /// fields.
    #[prost(string, repeated, tag="3")]
    pub exclusive_packages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `AptSettings`.
pub mod apt_settings {
    /// Apt patch type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// By default, upgrade will be performed.
        Unspecified = 0,
        /// Runs `apt-get dist-upgrade`.
        Dist = 1,
        /// Runs `apt-get upgrade`.
        Upgrade = 2,
    }
}
/// Yum patching will be performed by executing `yum update`. Additional options
/// can be set to control how this is executed.
///
/// Note that not all settings are supported on all platforms.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct YumSettings {
    /// Adds the `--security` flag to `yum update`. Not supported on
    /// all platforms.
    #[prost(bool, tag="1")]
    pub security: bool,
    /// Will cause patch to run `yum update-minimal` instead.
    #[prost(bool, tag="2")]
    pub minimal: bool,
    /// List of packages to exclude from update. These packages will be excluded by
    /// using the yum `--exclude` flag.
    #[prost(string, repeated, tag="3")]
    pub excludes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// An exclusive list of packages to be updated. These are the only packages
    /// that will be updated. If these packages are not installed, they will be
    /// ignored. This field must not be specified with any other patch
    /// configuration fields.
    #[prost(string, repeated, tag="4")]
    pub exclusive_packages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Googet patching is performed by running `googet update`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GooSettings {
}
/// Zypper patching is performed by running `zypper patch`.
/// See also <https://en.opensuse.org/SDB:Zypper_manual.>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZypperSettings {
    /// Adds the `--with-optional` flag to `zypper patch`.
    #[prost(bool, tag="1")]
    pub with_optional: bool,
    /// Adds the `--with-update` flag, to `zypper patch`.
    #[prost(bool, tag="2")]
    pub with_update: bool,
    /// Install only patches with these categories.
    /// Common categories include security, recommended, and feature.
    #[prost(string, repeated, tag="3")]
    pub categories: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Install only patches with these severities.
    /// Common severities include critical, important, moderate, and low.
    #[prost(string, repeated, tag="4")]
    pub severities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// List of patches to exclude from update.
    #[prost(string, repeated, tag="5")]
    pub excludes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// An exclusive list of patches to be updated. These are the only patches
    /// that will be installed using 'zypper patch patch:<patch_name>' command.
    /// This field must not be used with any other patch configuration fields.
    #[prost(string, repeated, tag="6")]
    pub exclusive_patches: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Windows patching is performed using the Windows Update Agent.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WindowsUpdateSettings {
    /// Only apply updates of these windows update classifications. If empty, all
    /// updates will be applied.
    #[prost(enumeration="windows_update_settings::Classification", repeated, tag="1")]
    pub classifications: ::prost::alloc::vec::Vec<i32>,
    /// List of KBs to exclude from update.
    #[prost(string, repeated, tag="2")]
    pub excludes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// An exclusive list of kbs to be updated. These are the only patches
    /// that will be updated. This field must not be used with other
    /// patch configurations.
    #[prost(string, repeated, tag="3")]
    pub exclusive_patches: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `WindowsUpdateSettings`.
pub mod windows_update_settings {
    /// Microsoft Windows update classifications as defined in
    /// \[1\]
    /// <https://support.microsoft.com/en-us/help/824684/description-of-the-standard-terminology-that-is-used-to-describe-micro>
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Classification {
        /// Invalid. If classifications are included, they must be specified.
        Unspecified = 0,
        /// "A widely released fix for a specific problem that addresses a critical,
        /// non-security-related bug." \[1\]
        Critical = 1,
        /// "A widely released fix for a product-specific, security-related
        /// vulnerability. Security vulnerabilities are rated by their severity. The
        /// severity rating is indicated in the Microsoft security bulletin as
        /// critical, important, moderate, or low." \[1\]
        Security = 2,
        /// "A widely released and frequent software update that contains additions
        /// to a product’s definition database. Definition databases are often used
        /// to detect objects that have specific attributes, such as malicious code,
        /// phishing websites, or junk mail." \[1\]
        Definition = 3,
        /// "Software that controls the input and output of a device." \[1\]
        Driver = 4,
        /// "New product functionality that is first distributed outside the context
        /// of a product release and that is typically included in the next full
        /// product release." \[1\]
        FeaturePack = 5,
        /// "A tested, cumulative set of all hotfixes, security updates, critical
        /// updates, and updates. Additionally, service packs may contain additional
        /// fixes for problems that are found internally since the release of the
        /// product. Service packs my also contain a limited number of
        /// customer-requested design changes or features." \[1\]
        ServicePack = 6,
        /// "A utility or feature that helps complete a task or set of tasks." \[1\]
        Tool = 7,
        /// "A tested, cumulative set of hotfixes, security updates, critical
        /// updates, and updates that are packaged together for easy deployment. A
        /// rollup generally targets a specific area, such as security, or a
        /// component of a product, such as Internet Information Services (IIS)." \[1\]
        UpdateRollup = 8,
        /// "A widely released fix for a specific problem. An update addresses a
        /// noncritical, non-security-related bug." \[1\]
        Update = 9,
    }
}
/// The strategy for retrying failed patches during the patch window.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetryStrategy {
    /// If true, the agent will continue to try and patch until the window has
    /// ended.
    #[prost(bool, tag="1")]
    pub enabled: bool,
}
/// A step that runs an executable for a PatchJob.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecStep {
    /// The ExecStepConfig for all Linux VMs targeted by the PatchJob.
    #[prost(message, optional, tag="1")]
    pub linux_exec_step_config: ::core::option::Option<ExecStepConfig>,
    /// The ExecStepConfig for all Windows VMs targeted by the PatchJob.
    #[prost(message, optional, tag="2")]
    pub windows_exec_step_config: ::core::option::Option<ExecStepConfig>,
}
/// Common configurations for an ExecStep.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecStepConfig {
    /// Defaults to \[0\]. A list of possible return values that the
    /// execution can return to indicate a success.
    #[prost(int32, repeated, tag="3")]
    pub allowed_success_codes: ::prost::alloc::vec::Vec<i32>,
    /// The script interpreter to use to run the script. If no interpreter is
    /// specified the script will be executed directly, which will likely
    /// only succeed for scripts with shebang lines.
    /// [Wikipedia shebang](<https://en.wikipedia.org/wiki/Shebang_(Unix>)).
    #[prost(enumeration="exec_step_config::Interpreter", tag="4")]
    pub interpreter: i32,
    /// Location of the executable.
    #[prost(oneof="exec_step_config::Executable", tags="1, 2")]
    pub executable: ::core::option::Option<exec_step_config::Executable>,
}
/// Nested message and enum types in `ExecStepConfig`.
pub mod exec_step_config {
    /// The interpreter used to execute the a file.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Interpreter {
        /// Deprecated, defaults to NONE for compatibility reasons.
        Unspecified = 0,
        /// Invalid for a Windows ExecStepConfig. For a Linux ExecStepConfig, the
        /// interpreter will be parsed from the shebang line of the script if
        /// unspecified.
        None = 3,
        /// Indicates that the script will be run with /bin/sh on Linux and cmd
        /// on windows.
        Shell = 1,
        /// Indicates that the file will be run with PowerShell.
        Powershell = 2,
    }
    /// Location of the executable.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Executable {
        /// An absolute path to the executable on the VM.
        #[prost(string, tag="1")]
        LocalPath(::prost::alloc::string::String),
        /// A GCS object containing the executable.
        #[prost(message, tag="2")]
        GcsObject(super::GcsObject),
    }
}
/// GCS object representation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsObject {
    /// Bucket of the GCS object.
    #[prost(string, tag="1")]
    pub bucket: ::prost::alloc::string::String,
    /// Name of the GCS object.
    #[prost(string, tag="2")]
    pub object: ::prost::alloc::string::String,
    /// Generation number of the GCS object. This is used to ensure that the
    /// ExecStep specified by this PatchJob does not change.
    #[prost(int64, tag="3")]
    pub generation_number: i64,
}
/// A unit of work to be performed by the agent.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Task {
    /// Unique task id.
    #[prost(string, tag="1")]
    pub task_id: ::prost::alloc::string::String,
    /// The type of task to perform.
    ///
    /// Task details must include the appropriate message based on this enum as
    /// specified below:
    /// APPLY_PATCHES = ApplyPatchesTask
    /// EXEC_STEP = ExecStepTask;
    #[prost(enumeration="TaskType", tag="2")]
    pub task_type: i32,
    /// Current directive to the agent.
    #[prost(enumeration="TaskDirective", tag="3")]
    pub task_directive: i32,
    /// Labels describing the task.  Used for logging by the agent.
    #[prost(btree_map="string, string", tag="6")]
    pub service_labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Specific details about the current task to perform.
    #[prost(oneof="task::TaskDetails", tags="4, 5")]
    pub task_details: ::core::option::Option<task::TaskDetails>,
}
/// Nested message and enum types in `Task`.
pub mod task {
    /// Specific details about the current task to perform.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TaskDetails {
        /// Details about the apply patches task to perform.
        #[prost(message, tag="4")]
        ApplyPatchesTask(super::ApplyPatchesTask),
        /// Details about the exec step task to perform.
        #[prost(message, tag="5")]
        ExecStepTask(super::ExecStepTask),
    }
}
/// Message which instructs agent to apply patches.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyPatchesTask {
    /// Specific information about how patches should be applied.
    #[prost(message, optional, tag="1")]
    pub patch_config: ::core::option::Option<PatchConfig>,
    /// If true, the agent will report its status as it goes through the motions
    /// but won't actually run any updates or perform any reboots.
    #[prost(bool, tag="3")]
    pub dry_run: bool,
}
/// Information reported from the agent about applying patches execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyPatchesTaskProgress {
    /// Required. The current state of this patch execution.
    #[prost(enumeration="apply_patches_task_progress::State", tag="1")]
    pub state: i32,
}
/// Nested message and enum types in `ApplyPatchesTaskProgress`.
pub mod apply_patches_task_progress {
    /// The intermediate states of applying patches.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified is invalid.
        Unspecified = 0,
        /// The agent has started the patch task.
        Started = 4,
        /// The agent is currently downloading patches.
        DownloadingPatches = 1,
        /// The agent is currently applying patches.
        ApplyingPatches = 2,
        /// The agent is currently rebooting the VM instance.
        Rebooting = 3,
    }
}
/// Information reported from the agent about applying patches execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyPatchesTaskOutput {
    /// Required. The final state of this task.
    #[prost(enumeration="apply_patches_task_output::State", tag="1")]
    pub state: i32,
}
/// Nested message and enum types in `ApplyPatchesTaskOutput`.
pub mod apply_patches_task_output {
    /// The final states of applying patches.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified is invalid.
        Unspecified = 0,
        /// Applying patches completed successfully.
        Succeeded = 1,
        /// Applying patches completed successfully, but a reboot is required.
        SucceededRebootRequired = 2,
        /// Applying patches failed.
        Failed = 3,
    }
}
/// Message which instructs agent to execute the following command.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecStepTask {
    /// Details of the exec step to run.
    #[prost(message, optional, tag="1")]
    pub exec_step: ::core::option::Option<ExecStep>,
}
/// Information reported from the agent about the exec step execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecStepTaskProgress {
    /// Required. The current state of this exec step.
    #[prost(enumeration="exec_step_task_progress::State", tag="1")]
    pub state: i32,
}
/// Nested message and enum types in `ExecStepTaskProgress`.
pub mod exec_step_task_progress {
    /// The intermediate states of exec steps.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified is invalid.
        Unspecified = 0,
        /// The agent has started the exec step task.
        Started = 1,
    }
}
/// Information reported from the agent about the exec step execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecStepTaskOutput {
    /// Required. The final state of the exec step.
    #[prost(enumeration="exec_step_task_output::State", tag="1")]
    pub state: i32,
    /// Required. The exit code received from the script which ran as part of the exec step.
    #[prost(int32, tag="2")]
    pub exit_code: i32,
}
/// Nested message and enum types in `ExecStepTaskOutput`.
pub mod exec_step_task_output {
    /// The final states of exec steps.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified is invalid.
        Unspecified = 0,
        /// The exec step completed normally.
        Completed = 1,
        /// The exec step was terminated because it took too long.
        TimedOut = 2,
        /// The exec step task was cancelled before it started.
        Cancelled = 3,
    }
}
/// Specifies the current agent behavior.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TaskDirective {
    /// Unspecified is invalid.
    Unspecified = 0,
    /// The task should continue to progress.
    Continue = 1,
    /// Task should not be started, or if already in progress, should stop
    /// at first safe stopping point.  Task should be considered done and will
    /// never repeat.
    Stop = 2,
}
/// Specifies the type of task to perform.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TaskType {
    /// Unspecified is invalid.
    Unspecified = 0,
    /// The apply patches task.
    ApplyPatches = 1,
    /// The exec step task.
    ExecStepTask = 2,
}
/// Package is a reference to the software package to be installed or removed.
/// The agent on the VM instance uses the system package manager to apply the
/// config.
///
///
/// These are the commands that the agent uses to install or remove
/// packages.
///
/// Apt
/// install: `apt-get update && apt-get -y install package1 package2 package3`
/// remove: `apt-get -y remove package1 package2 package3`
///
/// Yum
/// install: `yum -y install package1 package2 package3`
/// remove: `yum -y remove package1 package2 package3`
///
/// Zypper
/// install: `zypper install package1 package2 package3`
/// remove: `zypper rm package1 package2`
///
/// Googet
/// install: `googet -noconfirm install package1 package2 package3`
/// remove: `googet -noconfirm remove package1 package2 package3`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Package {
    /// The name of the package. A package is uniquely identified for conflict
    /// validation by checking the package name and the manager(s) that the
    /// package targets.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The desired_state the agent should maintain for this package. The
    /// default is to ensure the package is installed.
    #[prost(enumeration="DesiredState", tag="2")]
    pub desired_state: i32,
    /// Type of package manager that can be used to install this package.
    /// If a system does not have the package manager, the package is not
    /// installed or removed no error message is returned. By default,
    /// or if you specify `ANY`,
    /// the agent attempts to install and remove this package using the default
    /// package manager. This is useful when creating a policy that applies to
    /// different types of systems.
    ///
    /// The default behavior is ANY.
    #[prost(enumeration="package::Manager", tag="3")]
    pub manager: i32,
}
/// Nested message and enum types in `Package`.
pub mod package {
    /// Types of package managers that may be used to manage this package.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Manager {
        /// The default behavior is ANY.
        Unspecified = 0,
        /// Apply this package config using the default system package manager.
        Any = 1,
        /// Apply this package config only if Apt is available on the system.
        Apt = 2,
        /// Apply this package config only if Yum is available on the system.
        Yum = 3,
        /// Apply this package config only if Zypper is available on the system.
        Zypper = 4,
        /// Apply this package config only if GooGet is available on the system.
        Goo = 5,
    }
}
/// Represents a single Apt package repository. This repository is added to
/// a repo file that is stored at
/// `/etc/apt/sources.list.d/google_osconfig.list`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AptRepository {
    /// Type of archive files in this repository. The default behavior is DEB.
    #[prost(enumeration="apt_repository::ArchiveType", tag="1")]
    pub archive_type: i32,
    /// URI for this repository.
    #[prost(string, tag="2")]
    pub uri: ::prost::alloc::string::String,
    /// Distribution of this repository.
    #[prost(string, tag="3")]
    pub distribution: ::prost::alloc::string::String,
    /// List of components for this repository. Must contain at least one item.
    #[prost(string, repeated, tag="4")]
    pub components: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// URI of the key file for this repository. The agent maintains
    /// a keyring at `/etc/apt/trusted.gpg.d/osconfig_agent_managed.gpg` containing
    /// all the keys in any applied guest policy.
    #[prost(string, tag="5")]
    pub gpg_key: ::prost::alloc::string::String,
}
/// Nested message and enum types in `AptRepository`.
pub mod apt_repository {
    /// Type of archive.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ArchiveType {
        /// Unspecified.
        Unspecified = 0,
        /// DEB indicates that the archive contains binary files.
        Deb = 1,
        /// DEB_SRC indicates that the archive contains source files.
        DebSrc = 2,
    }
}
/// Represents a single Yum package repository. This repository is added to a
/// repo file that is stored at `/etc/yum.repos.d/google_osconfig.repo`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct YumRepository {
    /// A one word, unique name for this repository. This is
    /// the `repo id` in the Yum config file and also the `display_name` if
    /// `display_name` is omitted. This id is also used as the unique identifier
    /// when checking for guest policy conflicts.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The display name of the repository.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    /// The location of the repository directory.
    #[prost(string, tag="3")]
    pub base_url: ::prost::alloc::string::String,
    /// URIs of GPG keys.
    #[prost(string, repeated, tag="4")]
    pub gpg_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Represents a single Zypper package repository. This repository is added to a
/// repo file that is stored at `/etc/zypp/repos.d/google_osconfig.repo`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZypperRepository {
    /// A one word, unique name for this repository. This is
    /// the `repo id` in the zypper config file and also the `display_name` if
    /// `display_name` is omitted. This id is also used as the unique identifier
    /// when checking for guest policy conflicts.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The display name of the repository.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    /// The location of the repository directory.
    #[prost(string, tag="3")]
    pub base_url: ::prost::alloc::string::String,
    /// URIs of GPG keys.
    #[prost(string, repeated, tag="4")]
    pub gpg_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Represents a Goo package repository. These is added to a repo file
/// that is stored at C:/ProgramData/GooGet/repos/google_osconfig.repo.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GooRepository {
    /// The name of the repository.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The url of the repository.
    #[prost(string, tag="2")]
    pub url: ::prost::alloc::string::String,
}
/// A package repository.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PackageRepository {
    /// A specific type of repository.
    #[prost(oneof="package_repository::Repository", tags="1, 2, 3, 4")]
    pub repository: ::core::option::Option<package_repository::Repository>,
}
/// Nested message and enum types in `PackageRepository`.
pub mod package_repository {
    /// A specific type of repository.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Repository {
        /// An Apt Repository.
        #[prost(message, tag="1")]
        Apt(super::AptRepository),
        /// A Yum Repository.
        #[prost(message, tag="2")]
        Yum(super::YumRepository),
        /// A Zypper Repository.
        #[prost(message, tag="3")]
        Zypper(super::ZypperRepository),
        /// A Goo Repository.
        #[prost(message, tag="4")]
        Goo(super::GooRepository),
    }
}
/// A software recipe is a set of instructions for installing and configuring a
/// piece of software. It consists of a set of artifacts that are
/// downloaded, and a set of steps that install, configure, and/or update the
/// software.
///
/// Recipes support installing and updating software from artifacts in the
/// following formats:
/// Zip archive, Tar archive, Windows MSI, Debian package, and RPM package.
///
/// Additionally, recipes support executing a script (either defined in a file or
/// directly in this api) in bash, sh, cmd, and powershell.
///
/// Updating a software recipe
///
/// If a recipe is assigned to an instance and there is a recipe with the same
/// name but a lower version already installed and the assigned state
/// of the recipe is `INSTALLED_KEEP_UPDATED`, then the recipe is updated to
/// the new version.
///
/// Script Working Directories
///
/// Each script or execution step is run in its own temporary directory which
/// is deleted after completing the step.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SoftwareRecipe {
    /// Unique identifier for the recipe. Only one recipe with a given name is
    /// installed on an instance.
    ///
    /// Names are also used to identify resources which helps to determine whether
    /// guest policies have conflicts. This means that requests to create multiple
    /// recipes with the same name and version are rejected since they
    /// could potentially have conflicting assignments.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The version of this software recipe. Version can be up to 4 period
    /// separated numbers (e.g. 12.34.56.78).
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
    /// Resources available to be used in the steps in the recipe.
    #[prost(message, repeated, tag="3")]
    pub artifacts: ::prost::alloc::vec::Vec<software_recipe::Artifact>,
    /// Actions to be taken for installing this recipe. On failure it stops
    /// executing steps and does not attempt another installation. Any steps taken
    /// (including partially completed steps) are not rolled back.  Install steps
    /// must be specified and are used on first installation.
    #[prost(message, repeated, tag="4")]
    pub install_steps: ::prost::alloc::vec::Vec<software_recipe::Step>,
    /// Actions to be taken for updating this recipe. On failure it stops
    /// executing steps and  does not attempt another update for this recipe. Any
    /// steps taken (including partially completed steps) are not rolled back.
    /// Upgrade steps are not mandatory and are only used when upgrading.
    #[prost(message, repeated, tag="5")]
    pub update_steps: ::prost::alloc::vec::Vec<software_recipe::Step>,
    /// Default is INSTALLED. The desired state the agent should maintain for this
    /// recipe.
    ///
    /// INSTALLED: The software recipe is installed on the instance but won't be
    ///                         updated to new versions.
    /// UPDATED: The software recipe is installed on the instance. The recipe is
    ///                         updated to a higher version, if a higher version of
    ///                         the recipe is assigned to this instance.
    /// REMOVE: Remove is unsupported for software recipes and attempts to
    ///         create or update a recipe to the REMOVE state is rejected.
    #[prost(enumeration="DesiredState", tag="6")]
    pub desired_state: i32,
}
/// Nested message and enum types in `SoftwareRecipe`.
pub mod software_recipe {
    /// Specifies a resource to be used in the recipe.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Artifact {
        /// Id of the artifact, which the installation and update steps of this
        /// recipe can reference. Artifacts in a recipe cannot have the same id.
        #[prost(string, tag="1")]
        pub id: ::prost::alloc::string::String,
        /// Defaults to false. When false, recipes are subject to validations
        /// based on the artifact type:
        ///
        /// Remote: A checksum must be specified, and only protocols with
        ///         transport-layer security are permitted.
        /// GCS:    An object generation number must be specified.
        #[prost(bool, tag="4")]
        pub allow_insecure: bool,
        /// A specific type of artifact.
        #[prost(oneof="artifact::Artifact", tags="2, 3")]
        pub artifact: ::core::option::Option<artifact::Artifact>,
    }
    /// Nested message and enum types in `Artifact`.
    pub mod artifact {
        /// Specifies an artifact available via some URI.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Remote {
            /// URI from which to fetch the object. It should contain both the protocol
            /// and path following the format {protocol}://{location}.
            #[prost(string, tag="1")]
            pub uri: ::prost::alloc::string::String,
            /// Must be provided if `allow_insecure` is `false`.
            /// SHA256 checksum in hex format, to compare to the checksum of the
            /// artifact. If the checksum is not empty and it doesn't match the
            /// artifact then the recipe installation fails before running any of the
            /// steps.
            #[prost(string, tag="2")]
            pub checksum: ::prost::alloc::string::String,
        }
        /// Specifies an artifact available as a Cloud Storage object.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Gcs {
            /// Bucket of the Cloud Storage object.
            /// Given an example URL:
            /// `<https://storage.googleapis.com/my-bucket/foo/bar#1234567`>
            /// this value would be `my-bucket`.
            #[prost(string, tag="1")]
            pub bucket: ::prost::alloc::string::String,
            /// Name of the Cloud Storage object.
            /// As specified \[here\]
            /// (<https://cloud.google.com/storage/docs/naming#objectnames>)
            /// Given an example URL:
            /// `<https://storage.googleapis.com/my-bucket/foo/bar#1234567`>
            /// this value would be `foo/bar`.
            #[prost(string, tag="2")]
            pub object: ::prost::alloc::string::String,
            /// Must be provided if allow_insecure is false.
            /// Generation number of the Cloud Storage object.
            /// `<https://storage.googleapis.com/my-bucket/foo/bar#1234567`>
            /// this value would be `1234567`.
            #[prost(int64, tag="3")]
            pub generation: i64,
        }
        /// A specific type of artifact.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Artifact {
            /// A generic remote artifact.
            #[prost(message, tag="2")]
            Remote(Remote),
            /// A Cloud Storage artifact.
            #[prost(message, tag="3")]
            Gcs(Gcs),
        }
    }
    /// An action that can be taken as part of installing or updating a recipe.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Step {
        /// A specific type of step.
        #[prost(oneof="step::Step", tags="1, 2, 3, 4, 5, 6, 7")]
        pub step: ::core::option::Option<step::Step>,
    }
    /// Nested message and enum types in `Step`.
    pub mod step {
        /// Copies the artifact to the specified path on the instance.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CopyFile {
            /// The id of the relevant artifact in the recipe.
            #[prost(string, tag="1")]
            pub artifact_id: ::prost::alloc::string::String,
            /// The absolute path on the instance to put the file.
            #[prost(string, tag="2")]
            pub destination: ::prost::alloc::string::String,
            /// Whether to allow this step to overwrite existing files. If this is
            /// false and the file already exists the file is not overwritten
            /// and the step is considered a success. Defaults to false.
            #[prost(bool, tag="3")]
            pub overwrite: bool,
            /// Consists of three octal digits which represent, in
            /// order, the permissions of the owner, group, and other users for the
            /// file (similarly to the numeric mode used in the linux chmod utility).
            /// Each digit represents a three bit number with the 4 bit
            /// corresponding to the read permissions, the 2 bit corresponds to the
            /// write bit, and the one bit corresponds to the execute permission.
            /// Default behavior is 755.
            ///
            /// Below are some examples of permissions and their associated values:
            /// read, write, and execute: 7
            /// read and execute: 5
            /// read and write: 6
            /// read only: 4
            #[prost(string, tag="4")]
            pub permissions: ::prost::alloc::string::String,
        }
        /// Extracts an archive of the type specified in the specified directory.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ExtractArchive {
            /// The id of the relevant artifact in the recipe.
            #[prost(string, tag="1")]
            pub artifact_id: ::prost::alloc::string::String,
            /// Directory to extract archive to.
            /// Defaults to `/` on Linux or `C:\` on Windows.
            #[prost(string, tag="2")]
            pub destination: ::prost::alloc::string::String,
            /// The type of the archive to extract.
            #[prost(enumeration="extract_archive::ArchiveType", tag="3")]
            pub r#type: i32,
        }
        /// Nested message and enum types in `ExtractArchive`.
        pub mod extract_archive {
            /// Specifying the type of archive.
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
            #[repr(i32)]
            pub enum ArchiveType {
                /// Indicates that the archive type isn't specified.
                Unspecified = 0,
                /// Indicates that the archive is a tar archive with no encryption.
                Tar = 1,
                /// Indicates that the archive is a tar archive with gzip encryption.
                TarGzip = 2,
                /// Indicates that the archive is a tar archive with bzip encryption.
                TarBzip = 3,
                /// Indicates that the archive is a tar archive with lzma encryption.
                TarLzma = 4,
                /// Indicates that the archive is a tar archive with xz encryption.
                TarXz = 5,
                /// Indicates that the archive is a zip archive.
                Zip = 11,
            }
        }
        /// Installs an MSI file.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct InstallMsi {
            /// The id of the relevant artifact in the recipe.
            #[prost(string, tag="1")]
            pub artifact_id: ::prost::alloc::string::String,
            /// The flags to use when installing the MSI
            /// defaults to \["/i"\] (i.e. the install flag).
            #[prost(string, repeated, tag="2")]
            pub flags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// Return codes that indicate that the software installed or updated
            /// successfully. Behaviour defaults to \[0\]
            #[prost(int32, repeated, tag="3")]
            pub allowed_exit_codes: ::prost::alloc::vec::Vec<i32>,
        }
        /// Installs a deb via dpkg.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct InstallDpkg {
            /// The id of the relevant artifact in the recipe.
            #[prost(string, tag="1")]
            pub artifact_id: ::prost::alloc::string::String,
        }
        /// Installs an rpm file via the rpm utility.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct InstallRpm {
            /// The id of the relevant artifact in the recipe.
            #[prost(string, tag="1")]
            pub artifact_id: ::prost::alloc::string::String,
        }
        /// Executes an artifact or local file.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ExecFile {
            /// Arguments to be passed to the provided executable.
            #[prost(string, repeated, tag="3")]
            pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// Defaults to \[0\]. A list of possible return values that the program
            /// can return to indicate a success.
            #[prost(int32, repeated, tag="4")]
            pub allowed_exit_codes: ::prost::alloc::vec::Vec<i32>,
            /// Location of the file to execute.
            #[prost(oneof="exec_file::LocationType", tags="1, 2")]
            pub location_type: ::core::option::Option<exec_file::LocationType>,
        }
        /// Nested message and enum types in `ExecFile`.
        pub mod exec_file {
            /// Location of the file to execute.
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum LocationType {
                /// The id of the relevant artifact in the recipe.
                #[prost(string, tag="1")]
                ArtifactId(::prost::alloc::string::String),
                /// The absolute path of the file on the local filesystem.
                #[prost(string, tag="2")]
                LocalPath(::prost::alloc::string::String),
            }
        }
        /// Runs a script through an interpreter.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct RunScript {
            /// The shell script to be executed.
            #[prost(string, tag="1")]
            pub script: ::prost::alloc::string::String,
            /// Return codes that indicate that the software installed or updated
            /// successfully. Behaviour defaults to \[0\]
            #[prost(int32, repeated, tag="2")]
            pub allowed_exit_codes: ::prost::alloc::vec::Vec<i32>,
            /// The script interpreter to use to run the script. If no interpreter is
            /// specified the script is executed directly, which likely
            /// only succeed for scripts with
            /// [shebang lines](<https://en.wikipedia.org/wiki/Shebang_(Unix>)).
            #[prost(enumeration="run_script::Interpreter", tag="3")]
            pub interpreter: i32,
        }
        /// Nested message and enum types in `RunScript`.
        pub mod run_script {
            /// The interpreter used to execute a script.
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
            #[repr(i32)]
            pub enum Interpreter {
                /// Default value for ScriptType.
                Unspecified = 0,
                /// Indicates that the script is run with `/bin/sh` on Linux and `cmd`
                /// on windows.
                Shell = 1,
                /// Indicates that the script is run with powershell.
                Powershell = 3,
            }
        }
        /// A specific type of step.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Step {
            /// Copies a file onto the instance.
            #[prost(message, tag="1")]
            FileCopy(CopyFile),
            /// Extracts an archive into the specified directory.
            #[prost(message, tag="2")]
            ArchiveExtraction(ExtractArchive),
            /// Installs an MSI file.
            #[prost(message, tag="3")]
            MsiInstallation(InstallMsi),
            /// Installs a deb file via dpkg.
            #[prost(message, tag="4")]
            DpkgInstallation(InstallDpkg),
            /// Installs an rpm file via the rpm utility.
            #[prost(message, tag="5")]
            RpmInstallation(InstallRpm),
            /// Executes an artifact or local file.
            #[prost(message, tag="6")]
            FileExec(ExecFile),
            /// Runs commands in a shell.
            #[prost(message, tag="7")]
            ScriptRun(RunScript),
        }
    }
}
/// A request message for getting effective policy assigned to the instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupEffectiveGuestPolicyRequest {
    /// Required. This is the GCE instance identity token described in
    /// <https://cloud.google.com/compute/docs/instances/verifying-instance-identity>
    /// where the audience is 'osconfig.googleapis.com' and the format is 'full'.
    #[prost(string, tag="1")]
    pub instance_id_token: ::prost::alloc::string::String,
    /// Short name of the OS running on the instance. The OS Config agent only
    /// provideS this field for targeting if OS Inventory is enabled for that
    /// instance.
    #[prost(string, tag="2")]
    pub os_short_name: ::prost::alloc::string::String,
    /// Version of the OS running on the instance. The OS Config agent only
    /// provide this field for targeting if OS Inventory is enabled for that
    /// VM instance.
    #[prost(string, tag="3")]
    pub os_version: ::prost::alloc::string::String,
    /// Architecture of OS running on the instance. The OS Config agent only
    /// provide this field for targeting if OS Inventory is enabled for that
    /// instance.
    #[prost(string, tag="4")]
    pub os_architecture: ::prost::alloc::string::String,
}
/// The effective guest policy assigned to the instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EffectiveGuestPolicy {
    /// List of package configurations assigned to the VM instance.
    #[prost(message, repeated, tag="1")]
    pub packages: ::prost::alloc::vec::Vec<effective_guest_policy::SourcedPackage>,
    /// List of package repository configurations assigned to the VM instance.
    #[prost(message, repeated, tag="2")]
    pub package_repositories: ::prost::alloc::vec::Vec<effective_guest_policy::SourcedPackageRepository>,
    /// List of recipes assigned to the VM instance.
    #[prost(message, repeated, tag="3")]
    pub software_recipes: ::prost::alloc::vec::Vec<effective_guest_policy::SourcedSoftwareRecipe>,
}
/// Nested message and enum types in `EffectiveGuestPolicy`.
pub mod effective_guest_policy {
    /// A guest policy package including its source.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SourcedPackage {
        /// Name of the guest policy providing this config.
        #[prost(string, tag="1")]
        pub source: ::prost::alloc::string::String,
        /// A software package to configure on the VM instance.
        #[prost(message, optional, tag="2")]
        pub package: ::core::option::Option<super::Package>,
    }
    /// A guest policy package repository including its source.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SourcedPackageRepository {
        /// Name of the guest policy providing this config.
        #[prost(string, tag="1")]
        pub source: ::prost::alloc::string::String,
        /// A software package repository to configure on the VM instance.
        #[prost(message, optional, tag="2")]
        pub package_repository: ::core::option::Option<super::PackageRepository>,
    }
    /// A guest policy recipe including its source.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SourcedSoftwareRecipe {
        /// Name of the guest policy providing this config.
        #[prost(string, tag="1")]
        pub source: ::prost::alloc::string::String,
        /// A software recipe to configure on the VM instance.
        #[prost(message, optional, tag="2")]
        pub software_recipe: ::core::option::Option<super::SoftwareRecipe>,
    }
}
/// The desired state that the OS Config agent will maintain on the VM.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DesiredState {
    /// The default is to ensure the package is installed.
    Unspecified = 0,
    /// The agent ensures that the package is installed.
    Installed = 1,
    /// The agent ensures that the package is installed and
    /// periodically checks for and install any updates.
    Updated = 2,
    /// The agent ensures that the package is not installed and uninstall it
    /// if detected.
    Removed = 3,
}
/// A request message to receive task notifications.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceiveTaskNotificationRequest {
    /// Required. This is the Compute Engine instance identity token described in
    /// <https://cloud.google.com/compute/docs/instances/verifying-instance-identity>
    /// where the audience is 'osconfig.googleapis.com' and the format is 'full'.
    #[prost(string, tag="1")]
    pub instance_id_token: ::prost::alloc::string::String,
    /// Required. The version of the agent making the request.
    #[prost(string, tag="2")]
    pub agent_version: ::prost::alloc::string::String,
}
/// The streaming rpc message that notifies the agent when it has a task
/// that it needs to perform on the VM instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceiveTaskNotificationResponse {
}
/// A request message for signaling the start of a task execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartNextTaskRequest {
    /// Required. This is the Compute Engine instance identity token described in
    /// <https://cloud.google.com/compute/docs/instances/verifying-instance-identity>
    /// where the audience is 'osconfig.googleapis.com' and the format is 'full'.
    #[prost(string, tag="1")]
    pub instance_id_token: ::prost::alloc::string::String,
}
/// A response message that contains the details of the task to work on.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartNextTaskResponse {
    /// The details of the task that should be worked on.  Can be empty if there
    /// is no new task to work on.
    #[prost(message, optional, tag="1")]
    pub task: ::core::option::Option<Task>,
}
/// A request message for reporting the progress of current task.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportTaskProgressRequest {
    /// Required. This is the Compute Engine instance identity token described in
    /// <https://cloud.google.com/compute/docs/instances/verifying-instance-identity>
    /// where the audience is 'osconfig.googleapis.com' and the format is 'full'.
    #[prost(string, tag="1")]
    pub instance_id_token: ::prost::alloc::string::String,
    /// Required. Unique identifier of the task this applies to.
    #[prost(string, tag="2")]
    pub task_id: ::prost::alloc::string::String,
    /// Required. The type of task to report progress on.
    ///
    /// Progress must include the appropriate message based on this enum as
    /// specified below:
    /// APPLY_PATCHES = ApplyPatchesTaskProgress
    /// EXEC_STEP = Progress not supported for this type.
    /// APPLY_CONFIG_TASK = ApplyConfigTaskProgress
    #[prost(enumeration="TaskType", tag="3")]
    pub task_type: i32,
    /// Intermediate progress of the current task.
    #[prost(oneof="report_task_progress_request::Progress", tags="4, 5")]
    pub progress: ::core::option::Option<report_task_progress_request::Progress>,
}
/// Nested message and enum types in `ReportTaskProgressRequest`.
pub mod report_task_progress_request {
    /// Intermediate progress of the current task.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Progress {
        /// Details about the progress of the apply patches task.
        #[prost(message, tag="4")]
        ApplyPatchesTaskProgress(super::ApplyPatchesTaskProgress),
        /// Details about the progress of the exec step task.
        #[prost(message, tag="5")]
        ExecStepTaskProgress(super::ExecStepTaskProgress),
    }
}
/// The response message after the agent reported the current task progress.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportTaskProgressResponse {
    /// Instructs agent to continue or not.
    #[prost(enumeration="TaskDirective", tag="1")]
    pub task_directive: i32,
}
/// A request message for signaling the completion of a task execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportTaskCompleteRequest {
    /// Required. This is the Compute Engine instance identity token described in
    /// <https://cloud.google.com/compute/docs/instances/verifying-instance-identity>
    /// where the audience is 'osconfig.googleapis.com' and the format is 'full'.
    #[prost(string, tag="1")]
    pub instance_id_token: ::prost::alloc::string::String,
    /// Required. Unique identifier of the task this applies to.
    #[prost(string, tag="2")]
    pub task_id: ::prost::alloc::string::String,
    /// Required. The type of task to report completed.
    ///
    /// The output must include the appropriate message based on the following
    /// enum values:
    /// APPLY_PATCHES = ApplyPatchesTaskOutput
    /// EXEC_STEP = ExecStepTaskOutput
    /// APPLY_CONFIG_TASK = ApplyConfigTaskOutput
    #[prost(enumeration="TaskType", tag="3")]
    pub task_type: i32,
    /// Descriptive error message if the task execution ended in error.
    #[prost(string, tag="4")]
    pub error_message: ::prost::alloc::string::String,
    /// Final output details of the current task.
    #[prost(oneof="report_task_complete_request::Output", tags="5, 6")]
    pub output: ::core::option::Option<report_task_complete_request::Output>,
}
/// Nested message and enum types in `ReportTaskCompleteRequest`.
pub mod report_task_complete_request {
    /// Final output details of the current task.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Output {
        /// Final output details of the apply patches task;
        #[prost(message, tag="5")]
        ApplyPatchesTaskOutput(super::ApplyPatchesTaskOutput),
        /// Final output details of the exec step task;
        #[prost(message, tag="6")]
        ExecStepTaskOutput(super::ExecStepTaskOutput),
    }
}
/// The response message after the agent signaled the current task complete.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportTaskCompleteResponse {
}
/// The request message for registering the agent.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterAgentRequest {
    /// Required. This is the Compute Engine instance identity token described in
    /// <https://cloud.google.com/compute/docs/instances/verifying-instance-identity>
    /// where the audience is 'osconfig.googleapis.com' and the format is 'full'.
    #[prost(string, tag="1")]
    pub instance_id_token: ::prost::alloc::string::String,
    /// Required. The version of the agent.
    #[prost(string, tag="2")]
    pub agent_version: ::prost::alloc::string::String,
    /// Required. The capabilities supported by the agent. Supported values are:
    /// PATCH_GA
    /// GUEST_POLICY_BETA
    /// CONFIG_V1
    #[prost(string, repeated, tag="3")]
    pub supported_capabilities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The operating system long name.
    /// For example 'Debian GNU/Linux 9' or 'Microsoft Window Server 2019
    /// Datacenter'.
    #[prost(string, tag="4")]
    pub os_long_name: ::prost::alloc::string::String,
    /// The operating system short name.
    /// For example, 'windows' or 'debian'.
    #[prost(string, tag="5")]
    pub os_short_name: ::prost::alloc::string::String,
    /// The version of the operating system.
    #[prost(string, tag="6")]
    pub os_version: ::prost::alloc::string::String,
    /// The system architecture of the operating system.
    #[prost(string, tag="7")]
    pub os_architecture: ::prost::alloc::string::String,
}
/// The response message after the agent registered.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterAgentResponse {
}
/// Generated client implementations.
pub mod agent_endpoint_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// OS Config agent endpoint API.
    #[derive(Debug, Clone)]
    pub struct AgentEndpointServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AgentEndpointServiceClient<T>
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
        ) -> AgentEndpointServiceClient<InterceptedService<T, F>>
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
            AgentEndpointServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Stream established by client to receive Task notifications.
        pub async fn receive_task_notification(
            &mut self,
            request: impl tonic::IntoRequest<super::ReceiveTaskNotificationRequest>,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::ReceiveTaskNotificationResponse>,
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
                "/google.cloud.osconfig.agentendpoint.v1beta.AgentEndpointService/ReceiveTaskNotification",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Signals the start of a task execution and returns the task info.
        pub async fn start_next_task(
            &mut self,
            request: impl tonic::IntoRequest<super::StartNextTaskRequest>,
        ) -> Result<tonic::Response<super::StartNextTaskResponse>, tonic::Status> {
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
                "/google.cloud.osconfig.agentendpoint.v1beta.AgentEndpointService/StartNextTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Signals an intermediary progress checkpoint in task execution.
        pub async fn report_task_progress(
            &mut self,
            request: impl tonic::IntoRequest<super::ReportTaskProgressRequest>,
        ) -> Result<tonic::Response<super::ReportTaskProgressResponse>, tonic::Status> {
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
                "/google.cloud.osconfig.agentendpoint.v1beta.AgentEndpointService/ReportTaskProgress",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Signals that the task execution is complete and optionally returns the next
        /// task.
        pub async fn report_task_complete(
            &mut self,
            request: impl tonic::IntoRequest<super::ReportTaskCompleteRequest>,
        ) -> Result<tonic::Response<super::ReportTaskCompleteResponse>, tonic::Status> {
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
                "/google.cloud.osconfig.agentendpoint.v1beta.AgentEndpointService/ReportTaskComplete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lookup the effective guest policy that applies to a VM instance. This
        /// lookup merges all policies that are assigned to the instance ancestry.
        pub async fn lookup_effective_guest_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::LookupEffectiveGuestPolicyRequest>,
        ) -> Result<tonic::Response<super::EffectiveGuestPolicy>, tonic::Status> {
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
                "/google.cloud.osconfig.agentendpoint.v1beta.AgentEndpointService/LookupEffectiveGuestPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Registers the agent running on the VM.
        pub async fn register_agent(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterAgentRequest>,
        ) -> Result<tonic::Response<super::RegisterAgentResponse>, tonic::Status> {
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
                "/google.cloud.osconfig.agentendpoint.v1beta.AgentEndpointService/RegisterAgent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
