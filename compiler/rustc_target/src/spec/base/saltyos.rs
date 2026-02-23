use crate::spec::{Cc, LinkerFlavor, Lld, Os, PanicStrategy, RelroLevel, TargetOptions};

pub(crate) fn opts() -> TargetOptions {
    let pre_link_args = TargetOptions::link_args(
        LinkerFlavor::Gnu(Cc::Yes, Lld::Yes),
        &["-z", "max-page-size=4096"],
    );

    TargetOptions {
        os: Os::SaltyOs,
        linker_flavor: LinkerFlavor::Gnu(Cc::Yes, Lld::Yes),
        linker: Some("clang".into()),
        dynamic_linking: true,
        position_independent_executables: true,
        relro_level: RelroLevel::Full,
        panic_strategy: PanicStrategy::Abort,
        has_thread_local: false,
        pre_link_args,
        ..Default::default()
    }
}
