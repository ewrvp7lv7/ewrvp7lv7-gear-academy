// fn main() {
//     gear_wasm_builder::build();
// }

use hello_world_io::ProgramMetadata;

fn main() {
    gear_wasm_builder::build_with_metadata::<ProgramMetadata>();
}