use rust_synth::io;

pub struct Audio {
}

impl Audio {
    pub fn new() -> Audio {
        let channels = io::start_manual();

        Audio {
            
        }
    }

}