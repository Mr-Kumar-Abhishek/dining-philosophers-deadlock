struct Philosopher {
	name: String,
}

impl Philosopher {
	fn new(name: &str) -> Philosopher {
		Philosopher {
			name: name.to_string(),
		}
	}
	
	fn eat(&self) {
		println!("{} has done eating.", self.name);
	}
}


fn main(){
	let philosophers = vec![
		Philosopher::new("Acharya Vamana"),
		Philosopher::new("Maha Kapphina"),
		Philosopher::new("Gargi Vachaknavi"),
		Philosopher::new("Vacaspati Misra"),
		Philosopher::new("Adi Shankra"),
	];

	for p in &philosophers {
		p.eat();
	}
}

