use nih_plug::prelude::*;
use wa_plugin::WalkAnalysis;

fn main() {
    nih_export_standalone::<WalkAnalysis>();
}
