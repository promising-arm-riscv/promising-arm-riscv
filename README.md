# Promising-ARM/RISC-V

This is the artifact for the PLDI 2019 submission #368:
"Promising-ARM/RISC-V: a simpler and faster operational memory model
for ARMv8 and RISC-V", Christopher Pulte, Jean Pichon-Pharabod,
Jeehoon Kang, Sung-Hwan Lee, Chung-Kil Hur.

- The coq development for Promising-ARM/RISC-V can be found at
  `https://github.com/snu-sf/promising-arm/`. Please refer to
  `README.md` of that repository for detailed information.

- The `rmem` tool that contains the Promising executable model can be
  found at `https://github.com/rems-project/rmem/`, including the
  instructions for building it. Then, see `RUNNING_RMEM.md` for
  instructions on how to run the experiments of the paper. For using
  the scripts below, please checkout the copy from the repository into
  the top-level directory (the one containing this README file).

- `examples`: This contains the data-structure examples from Section 8
  of the paper, written as *.litmus files. This also contains some
  scripts for running the experiments: `run.sh` and `runfast.sh`,
  detailed in `RUNNING_RMEM.md`.

- `supplementary_text.pdf` contains the paper's appendix.
