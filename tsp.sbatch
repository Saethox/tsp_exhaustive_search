#!/usr/bin/env bash
#SBATCH --time=96:00:00
#SBATCH --partition=cpu
#SBATCH --output=output/output-%j.txt
#SBATCH --cpus-per-task=16
#SBATCH --mem-per-cpu=128
JOB_DIR=/data/oc-compute01/wurthjon/tsp_exhaustive_search
srun nix-shell $JOB_DIR/default.nix --command "cargo run --package tsp_exhaustive_search --bin tsp_exhaustive_search --release"
