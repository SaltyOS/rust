use crate::spec::{Cc, LinkerFlavor, Lld, Os, PanicStrategy, RelroLevel, TargetOptions, cvs};

pub(crate) fn opts() -> TargetOptions {
    let pre_link_args = TargetOptions::link_args(
        // `link_args` seeds both GNU variants internally; pass the non-LLD
        // flavor so rustc can mirror the args to the LLD twin without
        // tripping the add_link_args_iter assertion.
        LinkerFlavor::Gnu(Cc::Yes, Lld::No),
        &["-z", "max-page-size=4096"],
    );

    TargetOptions {
        os: Os::SaltyOs,
        linker_flavor: LinkerFlavor::Gnu(Cc::Yes, Lld::Yes),
        linker: Some("clang".into()),
        families: cvs!["unix"],
        dynamic_linking: true,
        position_independent_executables: true,
        relro_level: RelroLevel::Full,
        panic_strategy: PanicStrategy::Abort,
        has_thread_local: true,
        has_rpath: true,
        pre_link_args,
        ..Default::default()
    }
}
