//! Protection Security Sandbox
//!
//! A security layer that enforces explicit authorization for all actions.
//! Only executes actions that are explicitly allowlisted, requires confirmation
//! for complex tasks, restricts access to sensitive directories, and logs
//! all actions for audit purposes.
//!
//! Designed specifically for Android/Termux environments with local LLMs
//! (TinyLlama/Ollama) where security and control are paramount.

use async_trait::async_trait;
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::security::traits::Sandbox;

/// Action type classification for permission requirements
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActionType {
    /// Simple read operations (cat, ls, echo, etc.)
    Read,
    /// Write operations that modify files
    Write,
    /// Network operations (curl, wget, etc.)
    Network,
    /// System operations that affect OS state
    System,
    /// Shell execution
    Shell,
    /// File system modifications
    Filesystem,
    /// Package management
    Package,
    /// External command execution
    External,
}

/// Permission level for actions
#[derive(Debug, Clone, PartialEq)]
pub enum Permission {
    /// Action is explicitly allowed
    Allowed,
    /// Action requires user confirmation
    RequiresConfirmation,
    /// Action is blocked
    Blocked,
}

/// Protection sandbox configuration
#[derive(Debug, Clone)]
pub struct ProtectionConfig {
    /// Whether the sandbox is enabled
    pub enabled: bool,
    /// Directory for action logs
    pub log_path: PathBuf,
    /// Sensitive directories that are blocked
    pub sensitive_paths: HashSet<PathBuf>,
    /// Commands that are explicitly allowed without confirmation
    pub allowed_commands: HashSet<String>,
    /// Commands that require confirmation before execution
    pub confirm_commands: HashSet<String>,
    /// Commands that are explicitly blocked
    pub blocked_commands: HashSet<String>,
    /// Whether to require confirmation for network operations
    pub confirm_network: bool,
    /// Whether to require confirmation for write operations outside workspace
    pub confirm_external_writes: bool,
    /// Workspace directory (allowed for operations)
    pub workspace_dir: PathBuf,
}

impl Default for ProtectionConfig {
    fn default() -> Self {
        let mut sensitive_paths = HashSet::new();
        sensitive_paths.insert(PathBuf::from("/system"));
        sensitive_paths.insert(PathBuf::from("/data"));
        sensitive_paths.insert(PathBuf::from("/sdcard"));
        sensitive_paths.insert(PathBuf::from("/root"));
        sensitive_paths.insert(PathBuf::from("/etc"));
        sensitive_paths.insert(PathBuf::from("/proc"));
        sensitive_paths.insert(PathBuf::from("/sys"));
        sensitive_paths.insert(PathBuf::from("/dev"));
        sensitive_paths.insert(PathBuf::from("/var"));
        sensitive_paths.insert(PathBuf::from("/usr"));
        sensitive_paths.insert(PathBuf::from("/bin"));
        sensitive_paths.insert(PathBuf::from("/lib"));
        sensitive_paths.insert(PathBuf::from("/lib64"));

        let mut allowed_commands = HashSet::new();
        // Basic read commands
        allowed_commands.insert("cat".to_string());
        allowed_commands.insert("ls".to_string());
        allowed_commands.insert("echo".to_string());
        allowed_commands.insert("pwd".to_string());
        allowed_commands.insert("whoami".to_string());
        allowed_commands.insert("date".to_string());
        allowed_commands.insert("head".to_string());
        allowed_commands.insert("tail".to_string());
        allowed_commands.insert("grep".to_string());
        allowed_commands.insert("find".to_string());
        // Git operations (common in development)
        allowed_commands.insert("git".to_string());
        allowed_commands.insert("git status".to_string());
        allowed_commands.insert("git log".to_string());
        allowed_commands.insert("git diff".to_string());
        allowed_commands.insert("git branch".to_string());
        // Cargo/Rust development
        allowed_commands.insert("cargo".to_string());
        allowed_commands.insert("cargo check".to_string());
        allowed_commands.insert("cargo build".to_string());
        allowed_commands.insert("cargo test".to_string());
        allowed_commands.insert("cargo fmt".to_string());
        allowed_commands.insert("cargo clippy".to_string());
        allowed_commands.insert("rustc".to_string());

        let mut confirm_commands = HashSet::new();
        // File operations that can be destructive
        confirm_commands.insert("rm".to_string());
        confirm_commands.insert("rm -rf".to_string());
        confirm_commands.insert("mv".to_string());
        confirm_commands.insert("cp".to_string());
        confirm_commands.insert("dd".to_string());
        confirm_commands.insert("chmod".to_string());
        confirm_commands.insert("chown".to_string());
        // Package management
        confirm_commands.insert("pkg".to_string());
        confirm_commands.insert("apt".to_string());
        confirm_commands.insert("apt-get".to_string());
        // Network
        confirm_commands.insert("curl".to_string());
        confirm_commands.insert("wget".to_string());
        confirm_commands.insert("ssh".to_string());
        confirm_commands.insert("scp".to_string());
        // Shell
        confirm_commands.insert("sh".to_string());
        confirm_commands.insert("bash".to_string());
        confirm_commands.insert("zsh".to_string());

        let mut blocked_commands = HashSet::new();
        // Dangerous system commands
        blocked_commands.insert("sudo".to_string());
        blocked_commands.insert("su".to_string());
        blocked_commands.insert("reboot".to_string());
        blocked_commands.insert("shutdown".to_string());
        blocked_commands.insert("poweroff".to_string());
        blocked_commands.insert("halt".to_string());
        blocked_commands.insert("init".to_string());
        blocked_commands.insert("systemctl".to_string());
        blocked_commands.insert("service".to_string());
        blocked_commands.insert("kill".to_string());
        blocked_commands.insert("pkill".to_string());
        blocked_commands.insert("killall".to_string());
        // Formatting/Partitioning
        blocked_commands.insert("mkfs".to_string());
        blocked_commands.insert("fdisk".to_string());
        blocked_commands.insert("parted".to_string());
        // Direct memory/hardware access
        blocked_commands.insert("dd".to_string());

        Self {
            enabled: true,
            log_path: PathBuf::from("logs/actions.log"),
            sensitive_paths,
            allowed_commands,
            confirm_commands,
            blocked_commands,
            confirm_network: true,
            confirm_external_writes: true,
            workspace_dir: PathBuf::from("."),
        }
    }
}

impl ProtectionConfig {
    /// Load configuration from file or create default
    pub fn load() -> Self {
        // Check for config file
        let config_paths = [
            PathBuf::from("protection.toml"),
            PathBuf::from("~/.config/protection/config.toml"),
            PathBuf::from("/etc/protection/config.toml"),
        ];

        for path in &config_paths {
            if let Ok(content) = std::fs::read_to_string(path) {
                return Self::from_toml(&content).unwrap_or_default();
            }
        }

        Self::default()
    }

    /// Parse TOML configuration
    fn from_toml(content: &str) -> Option<Self> {
        // Simple TOML parsing - in production use a proper TOML library
        let mut config = Self::default();

        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("enabled") && line.contains('=') {
                config.enabled = !line.contains("false");
            } else if line.starts_with("log_path") && line.contains('=') {
                if let Some(val) = line.split('=').nth(1) {
                    config.log_path = PathBuf::from(val.trim().trim_matches('"'));
                }
            } else if line.starts_with("workspace_dir") && line.contains('=') {
                if let Some(val) = line.split('=').nth(1) {
                    config.workspace_dir = PathBuf::from(val.trim().trim_matches('"'));
                }
            } else if line.starts_with("confirm_network") && line.contains('=') {
                config.confirm_network = !line.contains("false");
            } else if line.starts_with("confirm_external_writes") && line.contains('=') {
                config.confirm_external_writes = !line.contains("false");
            }
        }

        Some(config)
    }
}

/// Action log entry
#[derive(Debug, Clone)]
pub struct ActionLog {
    pub timestamp: u64,
    pub command: String,
    pub args: Vec<String>,
    pub action_type: ActionType,
    pub permission: Permission,
    pub executed: bool,
    pub reason: String,
}

/// Protection Security Sandbox implementation
pub struct ProtectionSandbox {
    config: Arc<ProtectionConfig>,
    action_log: Arc<Mutex<Vec<ActionLog>>>,
}

impl ProtectionSandbox {
    /// Create a new Protection sandbox with default configuration
    pub fn new() -> Self {
        let config = Arc::new(ProtectionConfig::load());
        Self {
            config,
            action_log: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Create a new Protection sandbox with custom configuration
    pub fn with_config(config: ProtectionConfig) -> Self {
        // Ensure log directory exists
        if let Some(parent) = config.log_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        Self {
            config: Arc::new(config),
            action_log: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Classify a command into action types
    fn classify_command(&self, program: &str, args: &[String]) -> Vec<ActionType> {
        let mut types = Vec::new();

        // Classify by program name
        match program {
            "cat" | "head" | "tail" | "grep" | "find" | "ls" | "pwd" | "echo" => {
                types.push(ActionType::Read);
            }
            "rm" | "mv" | "cp" | "touch" | "mkdir" | "rmdir" => {
                types.push(ActionType::Write);
                types.push(ActionType::Filesystem);
            }
            "curl" | "wget" | "ssh" | "scp" | "nc" | "netcat" | "telnet" => {
                types.push(ActionType::Network);
            }
            "sh" | "bash" | "zsh" | "fish" => {
                types.push(ActionType::Shell);
            }
            "sudo" | "su" | "reboot" | "shutdown" => {
                types.push(ActionType::System);
            }
            "pkg" | "apt" | "apt-get" | "pacman" | "yum" | "dnf" => {
                types.push(ActionType::Package);
                types.push(ActionType::System);
            }
            "cargo" | "rustc" | "make" | "cmake" => {
                types.push(ActionType::External);
            }
            _ => {
                types.push(ActionType::External);
            }
        }

        // Classify by arguments
        for arg in args {
            if arg.contains("/system") || arg.contains("/data") || arg.contains("/sdcard") {
                types.push(ActionType::System);
            }
            if arg.starts_with("http://") || arg.starts_with("https://") {
                types.push(ActionType::Network);
            }
        }

        if types.is_empty() {
            types.push(ActionType::External);
        }

        types
    }

    /// Check if a path is in a sensitive directory
    fn is_sensitive_path(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();

        for sensitive in &self.config.sensitive_paths {
            if path_str.starts_with(&sensitive.to_string_lossy().to_string()) {
                return true;
            }
        }
        false
    }

    /// Get the permission level for a command
    fn get_permission(&self, program: &str, args: &[String]) -> (Permission, String) {
        // Check if explicitly blocked
        if self.config.blocked_commands.contains(program) {
            return (
                Permission::Blocked,
                format!("Command '{}' is in blocked list", program),
            );
        }

        // Check full command string against blocked patterns
        let full_cmd = format!("{} {}", program, args.join(" "));
        for blocked in &self.config.blocked_commands {
            if full_cmd.starts_with(blocked) {
                return (
                    Permission::Blocked,
                    format!("Command pattern '{}' is blocked", blocked),
                );
            }
        }

        // Check for sensitive paths in arguments
        for arg in args {
            let arg_path = PathBuf::from(arg);
            if self.is_sensitive_path(&arg_path) {
                return (
                    Permission::Blocked,
                    format!("Access to sensitive path '{}' is blocked", arg),
                );
            }
        }

        // Check if in allowed list
        if self.config.allowed_commands.contains(program) {
            // But still check for confirmation patterns
            let full_cmd = format!("{} {}", program, args.join(" "));
            for confirm in &self.config.confirm_commands {
                if full_cmd.starts_with(confirm) || program == confirm {
                    return (
                        Permission::RequiresConfirmation,
                        format!(
                            "Command '{}' requires explicit confirmation",
                            program
                        ),
                    );
                }
            }
            return (Permission::Allowed, "Command is in allowed list".to_string());
        }

        // Check if requires confirmation
        if self.config.confirm_commands.contains(program) {
            return (
                Permission::RequiresConfirmation,
                format!("Command '{}' requires explicit confirmation", program),
            );
        }

        // Check action types for automatic classification
        let action_types = self.classify_command(program, args);

        // Network operations require confirmation if configured
        if action_types.contains(&ActionType::Network) && self.config.confirm_network {
            return (
                Permission::RequiresConfirmation,
                "Network operations require confirmation".to_string(),
            );
        }

        // External commands require confirmation
        if action_types.contains(&ActionType::External)
            || action_types.contains(&ActionType::Shell)
        {
            return (
                Permission::RequiresConfirmation,
                "External/shell commands require confirmation".to_string(),
            );
        }

        // Default: require confirmation for unknown commands
        (
            Permission::RequiresConfirmation,
            "Unknown command requires confirmation".to_string(),
        )
    }

    /// Log an action to file
    fn log_action(&self, entry: &ActionLog) {
        // Log to memory
        if let Ok(mut log) = self.action_log.lock() {
            log.push(ActionLog {
                timestamp: entry.timestamp,
                command: entry.command.clone(),
                args: entry.args.clone(),
                action_type: entry.action_type.clone(),
                permission: entry.permission.clone(),
                executed: entry.executed,
                reason: entry.reason.clone(),
            });
        }

        // Log to file
        let log_line = format!(
            "[{}] CMD={} ARGS={:?} TYPE={:?} PERM={:?} EXECUTED={} REASON={}\n",
            entry.timestamp,
            entry.command,
            entry.args,
            entry.action_type,
            entry.permission,
            entry.executed,
            entry.reason
        );

        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.config.log_path)
        {
            let _ = file.write_all(log_line.as_bytes());
        }
    }

    /// Get action log for audit purposes
    pub fn get_action_log(&self) -> Vec<ActionLog> {
        if let Ok(log) = self.action_log.lock() {
            log.clone()
        } else {
            Vec::new()
        }
    }

    /// Clear action log (for testing)
    pub fn clear_log(&self) {
        if let Ok(mut log) = self.action_log.lock() {
            log.clear();
        }
    }

    /// Check if sandbox is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    /// Evaluate a command without executing it
    pub fn evaluate(&self, program: &str, args: &[String]) -> (Permission, String) {
        self.get_permission(program, args)
    }
}

impl Sandbox for ProtectionSandbox {
    fn wrap_command(&self, cmd: &mut Command) -> std::io::Result<()> {
        if !self.config.enabled {
            return Ok(());
        }

        // Extract command information
        let program = cmd
            .get_program()
            .to_str()
            .unwrap_or("unknown")
            .to_string();
        let args: Vec<String> = cmd
            .get_args()
            .filter_map(|s| s.to_str().map(String::from))
            .collect();

        // Get permission
        let (permission, reason) = self.get_permission(&program, &args);

        // Get timestamp
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Classify action
        let action_types = self.classify_command(&program, &args);
        let primary_type = action_types.first().cloned().unwrap_or(ActionType::External);

        // Handle based on permission
        match permission {
            Permission::Allowed => {
                // Log the action
                self.log_action(&ActionLog {
                    timestamp,
                    command: program.clone(),
                    args: args.clone(),
                    action_type: primary_type,
                    permission: Permission::Allowed,
                    executed: true,
                    reason: reason.clone(),
                });

                // Command is allowed to proceed
                Ok(())
            }
            Permission::RequiresConfirmation => {
                // Log the request for confirmation
                self.log_action(&ActionLog {
                    timestamp,
                    command: program.clone(),
                    args: args.clone(),
                    action_type: primary_type,
                    permission: Permission::RequiresConfirmation,
                    executed: false,
                    reason: reason.clone(),
                });

                // Return error requiring confirmation
                // In a real implementation, this would trigger an interactive prompt
                // For now, we block and require explicit user action
                Err(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    format!(
                        "Protection: Command '{}' requires explicit confirmation. Reason: {}. \
                         Add '{}' to allowed_commands in protection.toml to allow automatically.",
                        program, reason, program
                    ),
                ))
            }
            Permission::Blocked => {
                // Log the blocked action
                self.log_action(&ActionLog {
                    timestamp,
                    command: program.clone(),
                    args: args.clone(),
                    action_type: primary_type,
                    permission: Permission::Blocked,
                    executed: false,
                    reason: reason.clone(),
                });

                // Return error
                Err(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    format!("Protection: Command '{}' is blocked. Reason: {}", program, reason),
                ))
            }
        }
    }

    fn is_available(&self) -> bool {
        // Protection is always available as it's a pure software control layer
        true
    }

    fn name(&self) -> &str {
        "protection"
    }

    fn description(&self) -> &str {
        "Protection security sandbox - explicit authorization required for all actions. \
         Blocks access to sensitive paths, logs all operations, requires confirmation for complex tasks."
    }
}

impl Default for ProtectionSandbox {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for ProtectionSandbox {
    fn clone(&self) -> Self {
        Self {
            config: Arc::clone(&self.config),
            action_log: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protection_name() {
        let sandbox = ProtectionSandbox::new();
        assert_eq!(sandbox.name(), "protection");
    }

    #[test]
    fn test_protection_is_always_available() {
        let sandbox = ProtectionSandbox::new();
        assert!(sandbox.is_available());
    }

    #[test]
    fn test_sensitive_paths_are_blocked() {
        let sandbox = ProtectionSandbox::new();

        assert!(sandbox.is_sensitive_path(Path::new("/system/etc/config")));
        assert!(sandbox.is_sensitive_path(Path::new("/data/data/com.termux")));
        assert!(sandbox.is_sensitive_path(Path::new("/sdcard/Downloads")));
        assert!(!sandbox.is_sensitive_path(Path::new("/home/user/project")));
    }

    #[test]
    fn test_allowed_commands_return_allowed() {
        let sandbox = ProtectionSandbox::new();
        let (perm, _) = sandbox.evaluate("echo", &["hello".to_string()]);
        assert_eq!(perm, Permission::Allowed);
    }

    #[test]
    fn test_blocked_commands_are_blocked() {
        let sandbox = ProtectionSandbox::new();
        let (perm, _) = sandbox.evaluate("sudo", &["ls".to_string()]);
        assert_eq!(perm, Permission::Blocked);
    }

    #[test]
    fn test_sensitive_path_in_args_is_blocked() {
        let sandbox = ProtectionSandbox::new();
        let (perm, _) = sandbox.evaluate("cat", &["/system/etc/passwd".to_string()]);
        assert_eq!(perm, Permission::Blocked);
    }

    #[test]
    fn test_network_commands_require_confirmation() {
        let sandbox = ProtectionSandbox::new();
        let (perm, _) = sandbox.evaluate("curl", &["https://example.com".to_string()]);
        assert_eq!(perm, Permission::RequiresConfirmation);
    }

    #[test]
    fn test_action_logging() {
        let sandbox = ProtectionSandbox::new();
        sandbox.clear_log();

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        sandbox.log_action(&ActionLog {
            timestamp,
            command: "echo".to_string(),
            args: vec!["test".to_string()],
            action_type: ActionType::Read,
            permission: Permission::Allowed,
            executed: true,
            reason: "Test".to_string(),
        });

        let log = sandbox.get_action_log();
        assert_eq!(log.len(), 1);
        assert_eq!(log[0].command, "echo");
    }

    #[test]
    fn test_config_default() {
        let config = ProtectionConfig::default();
        assert!(config.enabled);
        assert!(config.confirm_network);
        assert!(!config.sensitive_paths.is_empty());
        assert!(!config.allowed_commands.is_empty());
        assert!(!config.blocked_commands.is_empty());
    }

    #[test]
    fn test_command_classification() {
        let sandbox = ProtectionSandbox::new();

        let types = sandbox.classify_command("cat", &["file.txt".to_string()]);
        assert!(types.contains(&ActionType::Read));

        let types = sandbox.classify_command("rm", &["-rf".to_string(), "dir".to_string()]);
        assert!(types.contains(&ActionType::Write));
        assert!(types.contains(&ActionType::Filesystem));

        let types = sandbox.classify_command("curl", &["https://example.com".to_string()]);
        assert!(types.contains(&ActionType::Network));
    }
}
