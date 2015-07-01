struct Philosopher {
	name: String,
}

impl Philosopher {
	fn new(name: &str) -> Philosopher {
		Philosopher {
			name: name.to_string(),
		}
	}
}


fn main(){
	let ph1 = Philosopher::new("Acharya Vamana");
	let ph2 = Philosopher::new("Maha Kapphina");
	let ph3 = Philosopher::new("Gargi Vachaknavi");
	let ph4 = Philosopher::new("Vacaspati Misra");
	let ph5 = Philosopher::new("Adi Shankra");

}

