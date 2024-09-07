use raydium_cp_swap::entry as entry_raydium_cp_swap;
use raydium_cp_swap::ID as PROGRAM_ID_RAYDIUM_CP_SWAP;
const PROGRAM_NAME_RAYDIUM_CP_SWAP: &str =  "raydium_cp_swap";
use fuzz_instructions::raydium_cp_swap_fuzz_instructions::FuzzInstruction as FuzzInstruction_raydium_cp_swap;
use trident_client::fuzzing::*;
mod accounts_snapshots;
mod fuzz_instructions;

pub type FuzzInstruction = todo!();

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {}

fn main() {
    loop {
        fuzz_trident!(fuzz_ix: FuzzInstruction, |fuzz_data: MyFuzzData| {

            // Specify programs you want to include in genesis
            // Programs without an `entry_fn`` will be searched for within `trident-genesis` folder.
            // `entry_fn`` example: processor!(convert_entry!(program_entry))
            let fuzzing_program1 = FuzzingProgram::new(todo!(),todo!(),processor!(convert_entry!(todo!())));

            let mut client =
                ProgramTestClientBlocking::new(&[todo!()])
                    .unwrap();

            // fill Program ID of program you are going to call
            let _ = fuzz_data.run_with_runtime(todo!(), &mut client);
        });
    }
}