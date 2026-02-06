use std::path::PathBuf;

use eyre::{eyre, Result};
use openvm_build::TargetFilter;
use openvm_sdk::{config::SdkVmConfig, Sdk, StdIn};
use openvm_stark_sdk::config::setup_tracing;

const KECCAK_AIR_PREFIXES: &[&str] = &["Keccakf", "Xorin"];

fn main() -> Result<()> {
    setup_tracing();

    let guest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("programs");
    let openvm_toml_path = guest_dir.join("openvm.toml");
    let openvm_toml = std::fs::read_to_string(&openvm_toml_path)
        .map_err(|e| eyre!("Failed to read {openvm_toml_path:?}: {e}"))?;
    let app_config = SdkVmConfig::from_toml(&openvm_toml)
        .map_err(|e| eyre!("Failed to parse openvm.toml: {e}"))?;

    let sdk = Sdk::new(app_config)?;
    let target_filter = Some(TargetFilter {
        name: "keccak".to_string(),
        kind: "example".to_string(),
    });
    let elf = sdk.build(Default::default(), &guest_dir, &target_filter, None)?;
    let exe = sdk.convert_to_exe(elf)?;

    // Create app_prover to get access to the VM (for metered execution)
    // and the converted exe, without constructing them separately.
    let app_prover = sdk.app_prover(exe)?;
    let vm = app_prover.vm();
    let exe = app_prover.exe();

    let air_names: Vec<String> = vm.air_names().map(|s| s.to_string()).collect();

    // Identify keccak chip AIRs by matching name prefixes
    let keccak_airs: Vec<(usize, &str)> = air_names
        .iter()
        .enumerate()
        .filter(|(_, name)| KECCAK_AIR_PREFIXES.iter().any(|p| name.contains(p)))
        .map(|(idx, name)| (idx, name.as_str()))
        .collect();

    if keccak_airs.is_empty() {
        return Err(eyre!(
            "No keccak-related AIRs found. Is the keccak extension enabled in openvm.toml?"
        ));
    }

    // Run metered execution to collect per-AIR trace heights
    let ctx = vm.build_metered_ctx(&exe);
    let interpreter = vm
        .metered_interpreter(&exe)
        .map_err(|e| eyre!("Failed to create metered interpreter: {e}"))?;
    let (segments, _final_state) = interpreter
        .execute_metered(StdIn::default(), ctx)
        .map_err(|e| eyre!("Metered execution failed: {e}"))?;

    // Verify that at least one keccak AIR has a non-zero trace height,
    // which confirms the custom keccak opcodes were actually executed.
    let mut any_keccak_used = false;

    for (seg_idx, segment) in segments.iter().enumerate() {
        println!("Segment {seg_idx} (num_insns: {}):", segment.num_insns);
        for &(air_idx, air_name) in &keccak_airs {
            let height = segment.trace_heights.get(air_idx).copied().unwrap_or(0);
            println!("  {air_name} (idx {air_idx}): trace_height = {height}");
            if height > 0 {
                any_keccak_used = true;
            }
        }
    }

    if any_keccak_used {
        println!("PASS: Keccak chips have non-zero trace heights.");
        Ok(())
    } else {
        Err(eyre!(
            "FAIL: All keccak chip trace heights are zero.\n\
             The keccak chips are not being used; the patch may not be working correctly."
        ))
    }
}
