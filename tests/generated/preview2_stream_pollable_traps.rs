//! This file has been auto-generated, please do not modify manually
//! To regenerate this file re-run `cargo xtask generate tests` from the project root

use std::fs;
use xshell::{cmd, Shell};

#[test]
fn preview2_stream_pollable_traps() -> anyhow::Result<()> {
    let sh = Shell::new()?;
    let wasi_file = "./tests/rundir/preview2_stream_pollable_traps.component.wasm";
    let _ = fs::remove_dir_all("./tests/rundir/preview2_stream_pollable_traps");

    let cmd = cmd!(sh, "node ./src/jco.js run  --jco-dir ./tests/rundir/preview2_stream_pollable_traps --jco-import ./tests/virtualenvs/base.js {wasi_file} hello this '' 'is an argument' 'with 🚩 emoji'");

    cmd.run().expect_err("test should exit with code 1");
    Ok(())
}
