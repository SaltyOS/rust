use crate::spec::{Arch, StackProbeType, Target, TargetMetadata, base};

pub(crate) fn target() -> Target {
    let mut base = base::saltyos::opts();
    base.max_atomic_width = Some(128);
    base.stack_probes = StackProbeType::Inline;

    Target {
        llvm_target: "aarch64-unknown-saltyos".into(),
        metadata: TargetMetadata {
            description: Some("SaltyOS (AArch64)".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(true),
        },
        pointer_width: 64,
        data_layout: "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128-Fn32".into(),
        arch: Arch::AArch64,
        options: base,
    }
}
