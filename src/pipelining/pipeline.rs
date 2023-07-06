use crate::pipelining::staging::stage::Stage;

struct Pipeline {
    stages: Vec<Box<dyn Stage>>,
}

impl Pipeline {
    pub fn new(stages: Vec<Box<dyn Stage>>) -> Self {
        Pipeline {
            stages: stages
        }
    }

    fn process() {
        println!("processing");
    }
}