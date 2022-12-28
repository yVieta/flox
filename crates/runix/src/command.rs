use derive_more::{Deref, From};

use crate::{
    arguments::{
        eval::EvaluationArgs, flake::FlakeArgs, source::SourceArgs, BundleArgs, DevelopArgs,
        EvalArgs, InstallableArg, InstallablesArgs,
    },
    command_line::{
        flag::{Flag, FlagType},
        Group, JsonCommand, NixCliCommand, TypedCommand,
    },
    installable::Installable,
};

#[derive(Debug, Default, Clone)]
pub struct Build {
    pub flake: FlakeArgs,
    pub eval: EvaluationArgs,
    pub source: SourceArgs,
    pub installables: InstallablesArgs,
}

impl NixCliCommand for Build {
    type Own = ();
    const SUBCOMMAND: &'static [&'static str] = &["build"];

    const INSTALLABLES: Group<Self, InstallablesArgs> = Some(|d| d.installables.clone());
    const FLAKE_ARGS: Group<Self, FlakeArgs> = Some(|d| d.flake.clone());
    const EVAL_ARGS: Group<Self, EvaluationArgs> = Some(|d| d.eval.clone());
    const SOURCE_ARGS: Group<Self, SourceArgs> = Some(|d| d.source.clone());
}
impl JsonCommand for Build {}
impl TypedCommand for Build {
    type Output = ();
}

/// `nix flake init` Command
#[derive(Debug, Default, Clone)]
pub struct FlakeInit {
    pub flake: FlakeArgs,
    pub eval: EvaluationArgs,
    pub installables: InstallablesArgs,

    pub template: Option<TemplateFlag>,
}

#[derive(Deref, Debug, Clone, From)]
#[from(forward)]
pub struct TemplateFlag(Installable);
impl Flag for TemplateFlag {
    const FLAG: &'static str = "--template";
    const FLAG_TYPE: FlagType<Self> = FlagType::arg();
}

impl NixCliCommand for FlakeInit {
    type Own = Option<TemplateFlag>;

    const SUBCOMMAND: &'static [&'static str] = &["flake", "init"];
    const INSTALLABLES: Group<Self, InstallablesArgs> = Some(|d| d.installables.clone());
    const FLAKE_ARGS: Group<Self, FlakeArgs> = Some(|d| d.flake.clone());
    const EVAL_ARGS: Group<Self, EvaluationArgs> = Some(|d| d.eval.clone());
    const OWN_ARGS: Group<Self, Option<TemplateFlag>> = Some(|d| d.template.clone());
}

/// `nix develop` Command
#[derive(Debug, Default, Clone)]
pub struct Develop {
    pub flake: FlakeArgs,
    pub eval: EvaluationArgs,
    pub source: SourceArgs,
    pub installable: InstallableArg,
    pub develop_args: DevelopArgs,
}

impl NixCliCommand for Develop {
    type Own = DevelopArgs;
    const SUBCOMMAND: &'static [&'static str] = &["develop"];
    const INSTALLABLE: Group<Self, InstallableArg> = Some(|d| d.installable.clone());
    const FLAKE_ARGS: Group<Self, FlakeArgs> = Some(|d| d.flake.clone());
    const EVAL_ARGS: Group<Self, EvaluationArgs> = Some(|d| d.eval.clone());
    const SOURCE_ARGS: Group<Self, SourceArgs> = Some(|d| d.source.clone());
    const OWN_ARGS: Group<Self, DevelopArgs> = Some(|d| d.develop_args.clone());
}

/// `nix eval` Command
#[derive(Debug, Default, Clone)]
pub struct Eval {
    pub flake: FlakeArgs,
    pub eval: EvaluationArgs,
    pub installable: InstallableArg,
    pub eval_args: EvalArgs,
}

impl NixCliCommand for Eval {
    type Own = EvalArgs;
    const SUBCOMMAND: &'static [&'static str] = &["eval"];
    const INSTALLABLE: Group<Self, InstallableArg> = Some(|d| d.installable.clone());
    const FLAKE_ARGS: Group<Self, FlakeArgs> = Some(|d| d.flake.clone());
    const EVAL_ARGS: Group<Self, EvaluationArgs> = Some(|d| d.eval.clone());
    const OWN_ARGS: Group<Self, EvalArgs> = Some(|d| d.eval_args.clone());
}
impl JsonCommand for Eval {}

/// `nix run` Command
#[derive(Debug, Default, Clone)]
pub struct Run {
    pub flake: FlakeArgs,
    pub eval: EvaluationArgs,
    pub source: SourceArgs,
    pub installable: InstallableArg,
}

impl NixCliCommand for Run {
    type Own = ();
    const SUBCOMMAND: &'static [&'static str] = &["run"];

    const INSTALLABLE: Group<Self, InstallableArg> = Some(|d| d.installable.clone());
    const FLAKE_ARGS: Group<Self, FlakeArgs> = Some(|d| d.flake.clone());
    const EVAL_ARGS: Group<Self, EvaluationArgs> = Some(|d| d.eval.clone());
    const SOURCE_ARGS: Group<Self, SourceArgs> = Some(|d| d.source.clone());
}
impl JsonCommand for Run {}
impl TypedCommand for Run {
    type Output = ();
}

/// `nix shell` Command
#[derive(Debug, Default, Clone)]
pub struct Shell {
    pub flake: FlakeArgs,
    pub eval: EvaluationArgs,
    pub source: SourceArgs,
    pub installables: InstallablesArgs,
}

impl NixCliCommand for Shell {
    type Own = ();
    const SUBCOMMAND: &'static [&'static str] = &["shell"];

    const INSTALLABLES: Group<Self, InstallablesArgs> = Some(|d| d.installables.clone());
    const FLAKE_ARGS: Group<Self, FlakeArgs> = Some(|d| d.flake.clone());
    const EVAL_ARGS: Group<Self, EvaluationArgs> = Some(|d| d.eval.clone());
    const SOURCE_ARGS: Group<Self, SourceArgs> = Some(|d| d.source.clone());
}
impl JsonCommand for Shell {}
impl TypedCommand for Shell {
    type Output = ();
}

/// `nix bundle` Command
#[derive(Debug, Default, Clone)]
pub struct Bundle {
    pub flake: FlakeArgs,
    pub eval: EvaluationArgs,
    pub source: SourceArgs,
    pub installable: InstallableArg,
    pub bundle_args: BundleArgs,
}

impl NixCliCommand for Bundle {
    type Own = BundleArgs;
    const SUBCOMMAND: &'static [&'static str] = &["bundle"];

    const INSTALLABLE: Group<Self, InstallableArg> = Some(|d| d.installable.clone());
    const FLAKE_ARGS: Group<Self, FlakeArgs> = Some(|d| d.flake.clone());
    const EVAL_ARGS: Group<Self, EvaluationArgs> = Some(|d| d.eval.clone());
    const SOURCE_ARGS: Group<Self, SourceArgs> = Some(|d| d.source.clone());
    const OWN_ARGS: Group<Self, BundleArgs> = Some(|d| d.bundle_args.clone());
}
impl JsonCommand for Bundle {}
impl TypedCommand for Bundle {
    type Output = ();
}
