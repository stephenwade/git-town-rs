pub const UNDO_CONTINUE_GUIDANCE: &str = "\n\nTo continue after having resolved conflicts, run \"git town continue\".\nTo go back to where you started, run \"git town undo\".\n";
pub const ALIASED_COMMANDS: &str = "Aliased commands: %s\n";
pub const ARGUMENT_UNKNOWN: &str = "unknown argument: %q";
pub const API_PARENT_BRANCH_LOOKUP_START: &str = "Looking for parent of %s ... ";
pub const API_PROPOSAL_LOOKUP_START: &str = "Looking for proposal online ... ";
pub const API_PROPOSAL_UPDATE_START: &str = "Updating proposal online ... ";
pub const API_UNEXPECTED_RESULT_DATA_STRUCTURE: &str = "unexpected result data structure";
pub const API_UPDATE_PROPOSAL_SOURCE: &str = "Updating source branch of proposal %s to %s ... ";
pub const API_UPDATE_PROPOSAL_TARGET: &str = "Updating target branch of proposal %s to %s ... ";
pub const BITBUCKET_APP_PASSWORD: &str = "Bitbucket app password: %s";
pub const BITBUCKET_USERNAME: &str = "Bitbucket username: %s";
pub const BRANCH_ALREADY_EXISTS_LOCALLY: &str = "there is already a branch %q";
pub const BRANCH_ALREADY_EXISTS_REMOTELY: &str =
    "there is already a branch %q at the \"origin\" remote";
pub const BRANCH_AUTHOR_MULTIPLE: &str = "\nMultiple people authored the %q branch.\n\n";
pub const BRANCH_CHECKOUT_PROBLEM: &str = "cannot check out branch %q: %w";
pub const BRANCH_CONTAINS_MERGE_COMMITS: &str =
    "branch %q contains merge commits, please compress and try again";
pub const BRANCH_CURRENT_PROBLEM: &str = "cannot determine current branch: %w";
pub const BRANCH_CURRENT_PROBLEM_NO_ERROR: &str = "cannot determine current branch";
pub const BRANCH_DELETED: &str = "deleted branch %q";
pub const BRANCH_DELETED_AT_REMOTE: &str = "branch %q was deleted at the remote";
pub const BRANCH_DELETED_HAS_UNMERGED_CHANGES: &str = "Branch %q was deleted at the remote but the local branch contains unshipped changes.\nI am therefore not removing this branch. You can see the unshipped changes by running \"git town diff-parent\".";
pub const BRANCH_DIFF_PROBLEM: &str = "cannot determine if branch %q has unmerged commits: %w";
pub const BRANCH_DOESNT_CONTAIN_COMMIT: &str =
    "branch %q does not contain commit %q. Found commits %s";
pub const BRANCH_DOESNT_EXIST: &str = "there is no branch %q";
pub const BRANCH_HAS_WRONG_SHA: &str = "cannot reset branch %q to %q because it received additional commits in the meantime. It should have SHA %q but has %q";
pub const BRANCH_INFO_NOT_FOUND: &str = "cannot find branch info for %q";
pub const BRANCH_INFOS_NOT_PROVIDED: &str = "BranchInfos not provided";
pub const BRANCH_IS_ALREADY_CONTRIBUTION: &str = "branch %q is already a contribution branch";
pub const BRANCH_IS_ALREADY_OBSERVED: &str = "branch %q is already observed";
pub const BRANCH_IS_ALREADY_PROTOTYPE: &str = "branch %q is already a prototype branch";
pub const BRANCH_IS_ALREADY_PARKED: &str = "branch %q is already parked";
pub const BRANCH_LOCAL_SHA_PROBLEM: &str = "cannot determine SHA of local branch %q: %w";
pub const BRANCH_LOCAL_PROBLEM: &str = "cannot determine whether the local branch %q exists: %w";
pub const BRANCH_NOT_IN_SYNC_WITH_PARENT: &str =
    r#"branch %q is not in sync with its parent, please run "git town sync" and try again"#;
pub const BRANCH_OTHER_WORKTREE: &str = r#"branch %q is active in another worktree"#;
pub const BRANCH_PARENT_CHANGED: &str = "branch %q is now a child of %q";
pub const BRANCH_TYPE_CANNOT_DETERMINE: &str = "cannot determine type of branch %q";
pub const BROWSER_OPEN: &str = "Please open in a browser: %s\n";
pub const CACHE_UNITIALIZED: &str = "using a cached value before initialization";
pub const CODEBERG_TOKEN: &str = "Codeberg token: %s\n";
pub const COMMANDS_RUN: &str = "Ran %d shell commands.";
pub const COMMIT_MESSAGE_PROBLEM: &str = "cannot determine last commit message: %w";
pub const COMMITS_SELECTED: &str = "%d commits selected";
pub const COMPRESS_UNSYNCED: &str = "please sync branch %q before compressing it";
pub const COMPRESS_IS_PERENNIAL: &str = "better not compress perennial branches";
pub const COMPRESS_ALREADY_ONE_COMMIT: &str = "branch %q has already just one commit";
pub const COMPRESS_BRANCH_NO_PARENT: &str = "cannot compress branch %q because it has no parent";
pub const COMPRESS_CONTRIBUTION_BRANCH: &str =
    "you are merely contributing to branch %q and should leave compressing it to the branch owner";
pub const COMPRESS_NO_BRANCH_INFO: &str = "no branch info for branch %q";
pub const COMPRESS_NO_COMMITS: &str = "branch %q has no commits";
pub const COMPRESS_OBSERVED_BRANCH: &str =
    "you are merely observing branch %q and should leave compressing it to the branch owner";
pub const COMPRESS_PARKED_BRANCH: &str = "branch %q and should not compress it";
pub const COMPLETION_TYPE_UNKNOWN: &str = "unknown completion type: %q";
pub const CONFIG_BRANCH_TYPE_OVERRIDE_EMPTY: &str = "removing empty branch type entry";
pub const CONFIG_FILE_CANNOT_READ: &str = "cannot read the configuration file %q: %w";
pub const CONFIG_FILE_INVALID_CONTENT: &str =
    "the configuration file %q does not contain TOML-formatted content: %w";
pub const CONFIG_LINEAGE_PARENT_IS_CHILD: &str =
    "removing lineage entry for %q because the parent is the child";
pub const CONFIG_LINEAGE_EMPTY_CHILD: &str = "removing empty lineage entry";
pub const CONFIG_MAINBRANCH_IN_CONFIG_FILE: &str =
    "please configure the main branch in the config file";
pub const CONFIG_NEEDED: &str = "Git Town needs to be configured\n\n";
pub const CONFIG_SCOPE_UNKNOWN: &str = "unknown configuration scope";
pub const CONFIG_STORAGE: &str = "Config storage: %s\n";
pub const CONFIG_SHIP_STRATEGY_UNKNOWN: &str = "unknown ship strategy: %q";
pub const CONFIG_SYNC_STRATEGY_UNKNOWN: &str = "unknown sync strategy: %q";
pub const CONFIG_REMOVE_ERROR: &str =
    "unexpected error while removing the 'git-town' section from the Git configuration: %w";
pub const CONFLICT_MERGE: &str = "git merge conflict";
pub const CONTINUE_MESSAGE: &str = r#"You can run "git town continue" to finish it."#;
pub const CONTINUE_SKIP_GUIDANCE: &str =
    "To continue by skipping the current branch, run \"git town skip\".";
pub const CONTRIBUTE_BRANCH_IS_NOW_CONTRIBUTION: &str = "branch %q is now a contribution branch\n";
pub const CONTRIBUTE_BRANCH_IS_LOCAL: &str = "branch %q is local only - branches you want to contribute to must have a remote branch because they are per definition other people's branches";
pub const CONTRIBUTION_BRANCH_CANNOT_PARK: &str = "cannot park contribution branches";
pub const CONTRIBUTION_BRANCH_CANNOT_PROPOSE: &str = "cannot propose contribution branches";
pub const CONTRIBUTION_BRANCH_CANNOT_SHIP: &str = "cannot ship contribution branches";
pub const CREATE_PROTOTYPE_BRANCHES: &str = "Create prototype branches:";
pub const CREATE_PROTOTYPE_BRANCHES_DEPRECATION: &str = r#"The Git Town configuration file contains the deprecated setting "create-prototype-branches".
Please upgrade to the new format: create.new-branch-type :&str= "prototype""#;
pub const CREDENTIALS_ACCESS: &str = "API token permits access to pull requests";
pub const CREDENTIALS_NO_ACCESS: &str = "Credentials don't provide access: %s\n";
pub const CREDENTIALS_FORGE_USER_NAME: &str = "Forge access as %s\n";
pub const DETACH_NEEDS_SYNC: &str = "please sync this stack before detaching branches from it";
pub const DETACH_NO_PARENT: &str = "cannot detach branches without parent";
pub const DETACH_OTHER_WORK_TREE: &str =
    "cannot detach because branch %q it is active in another worktree";
pub const DETACH_REMOTE_BRANCH: &str = "cannot detach a remote branch";
pub const DETACH_UNSUPPORTED_BRANCH_TYPE: &str =
    "cannot detach %s branches since you don't own them";
pub const DEV_REMOTE: &str = "Development remote: %s\n";
pub const DIFF_CONFLICT_WITH_MAIN: &str =
    "conflicts between your uncommmitted changes and the main branch";
pub const DRY_RUN: &str = "In dry run mode. No commands will be run. When run in normal mode, the command output will appear beneath the command. Some commands will only be run if necessary. For example: 'git push' will run if and only if there are local commits not on origin.";
pub const VALUE_INVALID: &str = "invalid value for %s: %q. Please provide either \"yes\" or \"no\"";
pub const CONFLICT_DETECTION_PROBLEM: &str = "cannot determine conflicts: %w";
pub const CONTINUE_NOTHING_TO_DO: &str = "nothing to continue";
pub const CONTINUE_UNRESOLVED_CONFLICTS: &str = "you must resolve the conflicts before continuing";
pub const CONTINUE_UNTRACKED_CHANGES: &str = "please stage or commit the untracked changes first";
pub const CURRENT_BRANCH_CANNOT_DETERMINE: &str = "cannot determine the current branch";
pub const DELETE_CANNOT_DELETE_MAIN_BRANCH: &str = "you cannot delete the main branch";
pub const DELETE_CANNOT_DELETE_PERENNIAL_BRANCHES: &str = "you cannot delete perennial branches";
pub const DIALOG_UNEXPECTED_RESPONSE: &str = "unexpected response: %s";
pub const DIFF_PARENT_NO_FEATURE_BRANCH: &str = "you can only diff-parent feature branches";
pub const DIFF_PROBLEM: &str = "cannot list diff of %q and %q: %w";
pub const DIR_CURRENT_PROBLEM: &str = "cannot determine the current directory";
pub const FEATURE_REGEX: &str = "Feature regex: %s\n";
pub const FILE_CONTENT_INVALID_JSON: &str = "cannot parse JSON content of file %q: %w";
pub const FILE_DELETE_PROBLEM: &str = "cannot delete file %q: %w";
pub const FILE_READ_PROBLEM: &str = "cannot read file %q: %w";
pub const FILE_STAT_PROBLEM: &str = "cannot check file %q: %w";
pub const FILE_WRITE_PROBLEM: &str = "cannot write file %q: %w";
pub const Forge: &str = "Forge: %s\n";
pub const FORGE_TOKEN_LOCATION: &str = "API token scope: %s";
pub const FORGE_BITBUCKET_NOT_IMPLEMENTED: &str = "shipping pull requests via the Bitbucket API is currently not supported. If you need this functionality, please vote for it by opening a ticket at https://github.com/git-town/git-town/issues";
pub const FORGE_BITBUCKET_MERGING_VIA_API: &str = "Bitbucket API: merging PR %s ... ";
pub const FORGE_CODEBERG_MERGING_VIA_API: &str = "Codeberg API: merging PR %s ... ";
pub const FORGE_GITLAB_MERGING_VIA_API: &str = "Merging MR !%d ... ";
pub const FORGE_GITLAB_UPDATE_MR_VIA_API: &str = "Updating target branch for MR !%d to %q ... ";
pub const FORGE_GITEA_NOT_IMPLEMENTED: &str = "shipping pull requests via the Gitea API is currently not supported. If you need this functionality, please vote for it by opening a ticket at https://github.com/git-town/git-town/issues";
pub const FORGE_GITEA_UPDATE_PR_VIA_API: &str = "Gitea API: Updating base branch for PR #%d to #%s";
pub const FORGE_GITHUB_MERGING_VIA_API: &str = "GitHub API: merging PR %s ... ";
pub const FORGE_TYPE_UNKNOWN: &str = "unknown forge type: %q";
pub const GITEA_TOKEN: &str = "Gitea token: %s\n";
pub const GIT_ANOTHER_PROCESS_IS_RUNNING_RETRY: &str =
    "another git process seems to be running in this repository, retrying in 1 sec ...";
pub const GIT_HUB_ENTERPRISE_INITIALIZE_ERROR: &str =
    "cannot initialize GitHub Enterprise client: %s";
pub const GIT_HUB_TOKEN: &str = "GitHub token: %s\n";
pub const GIT_LAB_TOKEN: &str = "GitLab token: %s\n";
pub const GIT_DIR_MISSING: &str = "cannot determine the '.git' directory: %w";
pub const GIT_USER_EMAIL_MISSING: &str =
    r#"please set the Git user email by running: git config --global user.email "<your email>""#;
pub const GIT_USER_NAME_MISSING: &str =
    r#"please set the Git user name by running: git config --global user.name "<your name>""#;
pub const GIT_URL_CANNOT_PARSE: &str = "cannot parse Git URL %q";
pub const GIT_VERSION_MAJOR_NOT_NUMBER: &str = "cannot convert major version %q to int: %w";
pub const GIT_VERSION_MINOR_NOT_NUMBER: &str = "cannot convert minor version %q to int: %w";
pub const GIT_VERSION_PROBLEM: &str = "cannot determine Git version: %w";
pub const GIT_VERSION_UNEXPECTED_OUTPUT: &str = "'git version' returned unexpected output: %q.\nPlease open an issue and supply the output of running 'git version'";
pub const GIT_VERSION_TOO_LOW: &str = "this app requires Git 2.30 or higher";
pub const HACK_TOO_MANY_ARGUMENTS: &str = "please provide only one branch to create";
pub const HACK_BRANCH_IS_ALREADY_FEATURE: &str = "branch %q is already a feature branch";
pub const HACK_BRANCH_IS_NOW_FEATURE: &str = "branch %q is now a feature branch\n";
pub const HACK_CANNOT_FEATURE_MAIN_BRANCH: &str = "you are trying to convert the main branch to a feature branch. That's not possible. If you want to create a feature branch, did you forget to add the branch name?";
pub const HACK_CANNOT_FEATURE_PERENNIAL_BRANCH: &str =
    "branch %q is a perennial branch and cannot be a feature branch";
pub const INPUT_ADD_OR_REMOVE: &str =
    r#"invalid argument %q. Please provide either "add" or "remove""#;
pub const INPUT_YES_OR_NO: &str = r#"invalid argument: %q. Please provide either "yes" or "no".\n"#;
pub const INVALID_STATUS_OUTPUT: &str = r#"invalid "git status -z" output: %q"#;
pub const KILL_DEPRECATION: &str = r#"DEPRECATION NOTICE

pub const This command has been renamed to "git town delete";
pub const and will be removed in future versions of Git Town."#;
pub const MAIN_BRANCH: &str = "Main branch: %s\n";
pub const MAIN_BRANCH_CANNOT_MAKE_CONTRIBUTION: &str =
    "cannot make the main branch a contribution branch";
pub const MAIN_BRANCH_CANNOT_OBSERVE: &str = "cannot observe the main branch";
pub const MAIN_BRANCH_CANNOT_PARK: &str = "cannot park the main branch";
pub const MAIN_BRANCH_CANNOT_PROPOSE: &str = "cannot propose the main branch";
pub const MAIN_BRANCH_CANNOT_PROTOTYPE: &str = "cannot prototype the main branch";
pub const MAIN_BRANCH_CANNOT_SHIP: &str = "cannot ship the main branch";
pub const MERGE_BRANCH_NOT_LOCAL: &str = "cannot merge: branch %q is not local";
pub const MERGE_OPEN_CHANGES: &str = "please commit or remove the open changes first";
pub const MERGE_NO_GRAND_PARENT: &str =
    "cannot merge branch %q because its parent branch (%s) has no parent";
pub const MERGE_NO_PARENT: &str = "cannot merge branch %q because it has no parent";
pub const MERGE_NOT_IN_SYNC_WITH_TRACKING: &str = r#"branch %q is not in sync with its tracking branch, please run "git town sync" and try again"#;
pub const MERGE_WRONG_BRANCH_TYPE: &str = "cannot merge %s branches";
pub const OBSERVED_BRANCH_CANNOT_PARK: &str = "cannot park observed branches";
pub const OBSERVED_BRANCH_CANNOT_PROPOSE: &str = "cannot propose observed branches";
pub const OBSERVED_BRANCH_CANNOT_SHIP: &str = "cannot ship observed branches";
pub const OBSERVE_BRANCH_IS_LOCAL: &str = "branch %q is local only - branches you want to observe must have a remote branch because they are per definition other people's branches";
pub const OBSERVED_BRANCH_IS_NOW_OBSERVED: &str = "branch %q is now an observed branch\n";
pub const OFFLINE_NOT_ALLOWED: &str = "this command requires an active internet connection";
pub const OPCODE_UNKNOWN: &str = "unknown opcode: %q, run \"git town status reset\" to reset it";
pub const OPEN_CHANGES_PROBLEM: &str = "cannot determine open changes: %w";
pub const ORIGIN_HOSTNAME: &str = "Origin hostname: %s\n";
pub const PARENT_DIALOG_SELECTED: &str = "Selected parent branch for %q: %s\n";
pub const PARKED_BRANCH_IS_NOW_PARKED: &str = "branch %q is now parked\n";
pub const PARKED_REMOVED: &str = "branch %q is no longer parked";
pub const PERENNIAL_BRANCH_CANNOT_MAKE_CONTRIBUTION: &str =
    "cannot make perennial branches contribution branches";
pub const PERENNIAL_BRANCH_CANNOT_OBSERVE: &str = "cannot observe perennial branches";
pub const PERENNIAL_BRANCH_CANNOT_PARK: &str = "cannot park perennial branches";
pub const PERENNIAL_BRANCH_CANNOT_PROPOSE: &str = "cannot propose perennial branches";
pub const PERENNIAL_BRANCH_CANNOT_PROTOTYPE: &str = "cannot prototype perennial branches";
pub const PERENNIAL_BRANCH_CANNOT_SHIP: &str = "cannot ship perennial branches";
pub const PERENNIAL_BRANCHES: &str = "Perennial branches: %s\n";
pub const PERENNIAL_BRANCH_REMOVED_PARENT_ENTRY: &str =
    "Removed parent entry for perennial branch %q\n";
pub const PERENNIAL_REGEX: &str = "Perennial regex: %s\n";
pub const PREVIOUS_COMMAND_FINISHED: &str =
    "The previous Git Town command (%s) finished successfully.\n";
pub const PREVIOUS_COMMAND_PROBLEM: &str = "The last Git Town command (%s) hit a problem %v ago.\n";
pub const PROPOSAL_MULTIPLE_FROM_TO_FOUND: &str = "found %d proposals from branch %q to branch %q";
pub const PROPOSAL_MULTIPLE_FROM_FOUND: &str = "found %d proposals for branch %q";
pub const PROPOSAL_NO_NUMBER_GIVEN: &str = "no proposal number given";
pub const PROPOSAL_NO_PARENT: &str = "branch %q has no parent and can therefore not be proposed";
pub const PROPOSAL_NOT_FOUND_FOR_BRANCH: &str = "cannot determine proposal for branch %q: %w";
pub const PROPOSAL_SOURCE_CANNOT_UPDATE: &str =
    "cannot update the proposal source branch on your forge";
pub const PROPOSAL_TARGET_BRANCH_UPDATE_PROBLEM: &str =
    "cannot update the target branch of proposal %d on your forge";
pub const PROPOSAL_URL_PROBLEM: &str = "cannot determine proposal URL from %q to %q: %w";
pub const PROTOTYPE_BRANCH_IS_NOW_PROTOTYPE: &str = "branch %q is now a prototype branch\n";
pub const PROTOTYPE_REMOVED: &str = "branch %q is no longer a prototype branch";
pub const PULL_REQUEST_DEPRECATION: &str = r#"DEPRECATION NOTICE

This command has been renamed to "git town propose";
and will be removed in future versions of Git Town."#;
pub const PUSH_HOOK: &str = "Push hook: %s\n";
pub const PUSH_NEW_BRANCHES_DEPRECATION: &str = r#"The Git Town configuration file contains the deprecated setting "push-new-branches".
Please upgrade to the new format: share-new-branches :&str= "push""#;
pub const REBASE_PROBLEM: &str = "cannot determine rebase in progress: %w";
pub const REMOTE_EXISTS_PROBLEM: &str = "cannot determine if remote %q exists: %w";
pub const REMOTES_PROBLEM: &str = "cannot determine remotes: %w";
pub const RENAME_BRANCH_DEPRECATION: &str = r#"DEPRECATION NOTICE

This command has been renamed to "git town rename";
and will be removed in future versions of Git Town."#;
pub const RENAME_MAIN_BRANCH: &str = "the main branch cannot be renamed";
pub const RENAME_PERENNIAL_BRANCH_WARNING: &str = "%q is a perennial branch. Renaming a perennial branch typically requires other updates. If you are sure you want to do this, use '--force'";
pub const RENAME_TO_SAME_NAME: &str = "cannot rename branch to current name";
pub const REPO_OUTSIDE: &str = "this is not a Git repository";
pub const RUN_AUTO_UNDO: &str = "%s\nAuto-undo... ";
pub const RUN_COMMAND_PROBLEM: &str = "error running command %q: %w";
pub const RUN_LOG_SERIALIZE_PROBLEM: &str = "cannot encode runlog: %w";
pub const RUN_LOG_CANNOT_OPEN: &str = "cannot open runlog file %q: %w";
pub const RUN_LOG_DELETED: &str = "Runlog deleted.";
pub const RUNLOG_DISPLAYING: &str = "Displaying runlog at %s\n";
pub const RUN_LOG_DOESNT_EXIST: &str = "Runlog doesn't exist.";
pub const RUN_LOG_CANNOT_READ: &str = "cannot read runlog file %q: %w";
pub const RUN_LOG_CANNOT_WRITE: &str = "cannot write to runlog file %q: %w";
pub const RUNSTATE_DELETED: &str = "Runstate file deleted.";
pub const RUNSTATE_DOESNT_EXIST: &str = "Runstate file doesn't exist.";
pub const RUNSTATE_DELETE_PROBLEM: &str = "cannot delete previous run state: %w";
pub const RUNSTATE_LOAD_PROBLEM: &str = "cannot load previous run state: %w";
pub const RUNSTATE_SERIALIZE_PROBLEM: &str = "cannot encode run-state: %w";
pub const RUNSTATE_CANNOT_DETERMINE_USER_DIR: &str =
    "cannot determine the user configuration directory: %w";
pub const RUNSTATE_SAVE_PROBLEM: &str = "cannot save run state: %w";
pub const SET_PARENT_NO_FEATURE_BRANCH: &str =
    "the branch %q is not a feature branch. Only feature branches can have parent branches";
pub const SETTING_DEPRECATED_MESSAGE: &str = "Upgrading deprecated %s setting %q to %q.";
pub const SETTING_DEPRECATED_VALUE_MESSAGE: &str =
    "Upgrading value of %s Git alias %q from %q to %q.";
pub const SETTING_CANNOT_REMOVE: &str = "ERROR: cannot remove %s Git setting %q: %v";
pub const SETTING_CANNOT_WRITE: &str = "ERROR: cannot write %s Git setting %q: %v";
pub const SETTING_IGNORE_INVALID: &str = "Notice: ignoring invalid dialog input setting %q\n";
pub const SETTING_SUNSET_DELETED: &str = "Deleting obsolete setting %q";
pub const SETTING_SUNSET_BRANCH_LIST: &str = "Inlining deprecated branch list %q";
pub const SHARE_NEW_BRANCHES: &str = "Share new branches: %s\n";
pub const SHIP_BRANCH_IS_IN_OTHER_WORKTREE: &str =
    "branch %q is checked out in another worktree, please ship from there";
pub const SHIP_ABORTED_MERGE_ERROR: &str = "aborted because merge exited with error";
pub const SHIP_API_CONNECTOR_REQUIRED: &str = "please configure API access to your forge, more info at https://www.git-town.com/configuration#access-tokens";
pub const SHIP_API_CONNECTOR_UNSUPPORTED: &str =
    "the Git Town driver for your forge does not support shipping via the API";
pub const SHIP_BRANCH_OTHER_WORKTREE: &str = "branch %q is active in another worktree";
pub const SHIP_BRANCH_HAS_NO_PARENT: &str = "branch %q has no parent to ship into";
pub const SHIP_BRANCH_NOTHING_TO_DO: &str = "the branch %q has no shippable changes";
pub const SHIP_CHILD_BRANCH: &str =
    "shipping this branch would ship %s as well,\nplease ship %q first";
pub const SHIP_DELETES_TRACKING_BRANCHES: &str = "Ship deletes tracking branches: %s\n";
pub const SHIP_API_NO_PROPOSAL: &str = "cannot ship branch %q via API because it has no proposal";
pub const SHIP_API_NO_REMOTE_BRANCH: &str =
    "cannot ship branch %q via API because it has no remote branch";
pub const SHIP_MESSAGE_WITH_FAST_FORWARD: &str =
    "shipping with the fast-forward strategy does not use the given commit message";
pub const SHIP_OPEN_CHANGES: &str =
    "you have uncommitted changes. Did you mean to commit them before shipping?";
pub const SHIP_STRATEGY_MISSING: &str = "no ship strategy provided";
pub const SKIP_BRANCH_HAS_CONFLICTS: &str = "cannot skip branch that resulted in conflicts";
pub const SKIP_MESSAGE: &str =
    r#"You can run "git town skip" to skip the currently failing operation."#;
pub const SKIP_NOTHING_TO_DO: &str = "nothing to skip";
pub const SKIP_NO_INITIAL_BRANCH_INFO: &str =
    "found no information about branch %q in the initial snapshot";
pub const SKIP_NO_FINAL_BRANCH_INFO: &str =
    "found no information about branch %q in the final snapshot";
pub const SKIP_NO_FINAL_SNAPSHOT: &str = "found no final snapshot";
pub const SQUASH_CANNOT_READ_FILE: &str = "cannot read squash message file %q: %w";
pub const SQUASH_COMMIT_AUTHOR_QUERY: &str = "Please choose an author for the squash commit:";
pub const SQUASH_COMMIT_AUTHOR_PROBLEM: &str = "error getting squash commit author: %w";
pub const SQUASH_COMMIT_AUTHOR_SELECTION: &str = "Selected squash commit author: %s\n";
pub const SQUASH_MESSAGE_PROBLEM: &str = "cannot comment out the squash commit message: %w";
pub const STATUS_FILE_NOT_FOUND: &str = "No status file found for this repository.";
pub const SWAP_NO_GRAND_PARENT: &str = "cannot swap a branch without grand-parent";
pub const SWAP_NO_PARENT: &str = "cannot swap a branch without parent";
pub const SWAP_NEEDS_COMPRESS: &str =
    "cannot swap because branch %q contains merge commits - please compress and try again";
pub const SWAP_NEEDS_SYNC: &str = "please sync your branches before swapping";
pub const SWAP_OTHER_WORK_TREE: &str =
    "cannot swap because branch %q it is active in another worktree";
pub const SWAP_PARENT_NOT_LOCAL: &str = "cannot swap: parent branch %q is not available locally";
pub const SWAP_PARENT_WRONG_BRANCH_TYPE: &str = "cannot swap: branch %q is a %s branch";
pub const SWAP_REMOTE_BRANCH: &str = "cannot swap: branch %q is remote";
pub const SWAP_UNSUPPORTED_BRANCH_TYPE: &str = "cannot swap: branch %q is a %s branch";
pub const SWITCH_NO_BRANCHES: &str = "no branches to switch to";
pub const SWITCH_UNCOMMITTED_CHANGES: &str = "uncommitted changes";
pub const SYNC_FEATURE_BRANCHES: &str = "Sync feature branches: %s\n";
pub const SYNC_PERENNIAL_BRANCH_HAS_UNPUSHED_COMMITS: &str =
    r#"cannot sync branch %q because it has unpushed local commits"#;
pub const SYNC_PERENNIAL_BRANCHES: &str = "Sync perennial branches: %s\n";
pub const SYNC_PROTOTYPE_BRANCHES: &str = "Sync prototype branches: %s\n";
pub const SYNC_STATUS_NOT_RECOGNIZED: &str =
    "cannot determine the sync status for Git remote %q and branch name %q";
pub const SYNC_TAGS: &str = "Sync tags: %s\n";
pub const SYNC_WITH_UPSTREAM: &str = "Sync with upstream: %s\n";
pub const UNDO_CANNOT_REVERT_COMMIT_ON_PERENNIAL_BRANCH: &str =
    "Cannot undo commit %q because it is on a perennial branch";
pub const UNDO_CREATE_OPCODE_PROBLEM: &str = "cannot create undo operations for %q: %w";
pub const UNDO_MESSAGE: &str = r#"You can run "git town undo" to go back to where you started."#;
pub const UNDO_NOTHING_TO_DO: &str = "nothing to undo";
pub const UNFINISHED_COMMAND_HANDLE: &str = "Handle unfinished command: %s\n";
pub const UNFINISHED_RUN_STATE_CONTINUE: &str =
    "Continue the \"%s\" command after having resolved conflicts";
pub const UNFINISHED_RUN_STATE_DISCARD: &str =
    "Discard the unfinished state and run the new command";
pub const UNFINISHED_RUN_STATE_QUIT: &str = "Quit without running anything";
pub const UNFINISHED_RUN_STATE_SKIP: &str =
    "Skip the current branch and continue the \"%s\" command on the next branch";
pub const UNFINISHED_RUN_STATE_UNDO: &str = "Undo the previous \"%s\" command";
pub const UNKNOWN_BRANCH_TYPE: &str = "Unknown branch type: %s\n";
pub const WALK_ALL_OR_STACK: &str = "please provide either --all or --stack";
pub const WALK_DONE: &str = "Branch walk done.";
pub const WALK_NO_DRY_RUN: &str = "there is no dry-run mode for walking through branches on your shell, please call with a command to run on each branch";
