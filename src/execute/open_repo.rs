use crate::{git, misc::Counter, subshell};
use std::{error::Error, process::Command};

#[cfg(windows)]
const SYSTEM_INFO_CMD: (&str, [&str; 2]) = ("cmd", ["/c", "ver"]);
#[cfg(not(windows))]
const SYSTEM_INFO_CMD: (&str, [&str; 1]) = ("uname", ["-a"]);

pub fn open_repo(args: OpenRepoArgs) -> Result<OpenRepoResult, Box<dyn Error>> {
    if args.verbose {
        println!("Git Town {}", env!("CARGO_PKG_VERSION"));
        println!("OS: {}", std::env::consts::OS);
        _ = Command::new(SYSTEM_INFO_CMD.0)
            .args(SYSTEM_INFO_CMD.1)
            .status()
    }
    let commands_counter = Counter::new();
    let backend_runner = subshell::backend_runner::BackendRunner {
        commands_counter,
        dir: None,
        verbose: args.verbose,
    };
    todo!()
    // gitCommands := git.Commands{
    //     CurrentBranchCache: &cache.WithPrevious[gitdomain.LocalBranchName]{},
    //     RemotesCache:       &cache.Cache[gitdomain.Remotes]{},
    // }
    // gitVersion, err := gitCommands.GitVersion(backendRunner)
    // if err != nil {
    //     return emptyOpenRepoResult(), err
    // }
    // if !gitVersion.IsMinimumRequiredGitVersion() {
    //     return emptyOpenRepoResult(), errors.New(messages.GitVersionTooLow)
    // }
    // rootDir, hasRootDir := gitCommands.RootDirectory(backendRunner).Get()
    // if args.ValidateGitRepo {
    //     if !hasRootDir {
    //         err = errors.New(messages.RepoOutside)
    //         return emptyOpenRepoResult(), err
    //     }
    // }
    // configGitAccess := gitconfig.Access{Runner: backendRunner}
    // globalSnapshot, err := configGitAccess.Load(Some(configdomain.ConfigScopeGlobal), configdomain.UpdateOutdatedYes)
    // if err != nil {
    //     return emptyOpenRepoResult(), err
    // }
    // localSnapshot, err := configGitAccess.Load(Some(configdomain.ConfigScopeLocal), configdomain.UpdateOutdatedYes)
    // if err != nil {
    //     return emptyOpenRepoResult(), err
    // }
    // unscopedSnapshot, err := configGitAccess.Load(None[configdomain.ConfigScope](), configdomain.UpdateOutdatedNo)
    // if err != nil {
    //     return emptyOpenRepoResult(), err
    // }
    // unscopedConfig, err := configdomain.NewPartialConfigFromSnapshot(unscopedSnapshot, true, configGitAccess.RemoveLocalConfigValue)
    // if err != nil {
    //     return emptyOpenRepoResult(), err
    // }
    // configSnapshot := undoconfig.ConfigSnapshot{
    //     Global: globalSnapshot,
    //     Local:  localSnapshot,
    // }
    // finalMessages := stringslice.NewCollector()
    // configFile, err := configfile.Load(rootDir, configfile.FileName, finalMessages)
    // if err != nil {
    //     return emptyOpenRepoResult(), err
    // }
    // if configFile.IsNone() {
    //     configFile, err = configfile.Load(rootDir, configfile.AlternativeFileName, finalMessages)
    //     if err != nil {
    //         return emptyOpenRepoResult(), err
    //     }
    // }
    // unvalidatedConfig := config.NewUnvalidatedConfig(config.NewUnvalidatedConfigArgs{
    //     Access:        configGitAccess,
    //     ConfigFile:    configFile,
    //     DryRun:        args.DryRun,
    //     EnvConfig:     envconfig.Load(),
    //     FinalMessages: finalMessages,
    //     GitConfig:     unscopedConfig,
    //     GitVersion:    gitVersion,
    // })
    // frontEndRunner := newFrontendRunner(newFrontendRunnerArgs{
    //     backend:          backendRunner,
    //     counter:          commandsCounter,
    //     dryRun:           args.DryRun,
    //     getCurrentBranch: gitCommands.CurrentBranch,
    //     printBranchNames: args.PrintBranchNames,
    //     printCommands:    args.PrintCommands,
    // })
    // isOffline := unvalidatedConfig.NormalConfig.Offline
    // if args.ValidateIsOnline && isOffline.IsOffline() {
    //     err = errors.New(messages.OfflineNotAllowed)
    //     return emptyOpenRepoResult(), err
    // }
    // if args.ValidateGitRepo {
    //     var currentDirectory string
    //     currentDirectory, err = os.Getwd()
    //     if err != nil {
    //         err = errors.New(messages.DirCurrentProblem)
    //         return emptyOpenRepoResult(), err
    //     }
    //     if currentDirectory != rootDir.String() {
    //         err = gitCommands.ChangeDir(rootDir)
    //     }
    // }
    // return OpenRepoResult{
    //     Backend:           backendRunner,
    //     CommandsCounter:   commandsCounter,
    //     ConfigSnapshot:    configSnapshot,
    //     FinalMessages:     finalMessages,
    //     Frontend:          frontEndRunner,
    //     Git:               gitCommands,
    //     IsOffline:         isOffline,
    //     RootDir:           rootDir,
    //     UnvalidatedConfig: unvalidatedConfig,
    // }, err
}

#[derive(Debug)]
pub struct OpenRepoArgs {
    pub dry_run: bool,
    pub print_branch_names: bool,
    pub print_commands: bool,
    pub validate_git_repo: bool,
    pub validate_is_online: bool,
    pub verbose: bool,
}

#[derive(Debug)]
pub struct OpenRepoResult {
    // backend: git_domain::RunnerQuerier,
    // commands_counter: Mutable<gohacks::Counter>,
    // config_snapshot: undo_config::ConfigSnapshot,
    // final_messages: string_slice::Collector,
    // frontend: git_domain::Runner,
    // git: git::Commands,
    // is_offline: config_domain::Offline,
    // root_dir: git_domain::RepoRootDir,
    // unvalidated_config: config::UnvalidatedConfig,
}
