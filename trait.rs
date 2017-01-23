
#[derive(Debug)]
struct Period {seconds: u64}

trait ToPeriod {
    fn minutes(self) -> Period;    
}

impl ToPeriod for u64 {
    fn minutes(self) -> Period {
        Period {seconds: self * 60}
    }
}

fn main() {
    let three_minutes = 3.minutes();
    println!("{:?}", three_minutes);
    // Period { seconds: 180 }
}

