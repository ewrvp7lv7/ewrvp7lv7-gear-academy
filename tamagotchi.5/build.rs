// fn main() {
//     gear_wasm_builder::build();
// }

use tamagotchi_io::ProgramMetadata;

fn main() {
    gear_wasm_builder::build_with_metadata::<ProgramMetadata>();
}