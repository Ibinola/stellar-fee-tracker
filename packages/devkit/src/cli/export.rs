use crate::simulation::fee_model::FeePoint;
use std::fmt::Write as FmtWrite;

/// Exports devkit results to external formats.
pub struct Export;

impl Export {
    /// Serialize fee points to CSV string with columns: timestamp,fee,ledger,is_spike.
    pub fn to_csv(points: &[FeePoint]) -> String {
        let mut out = String::from("timestamp,fee,ledger,is_spike\n");
        for p in points {
            writeln!(out, "{},{},{},{}", p.timestamp, p.fee, p.ledger, p.is_spike).unwrap();
        }
        out
    }

    /// Write fee points to a CSV file.
    pub fn write_csv(points: &[FeePoint], path: &std::path::Path) -> std::io::Result<()> {
        std::fs::write(path, Self::to_csv(points))
    }
}
