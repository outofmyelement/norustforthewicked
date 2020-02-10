fn main() {
	let user1 = User {
		email: String::from("someone@example.com"),
		username: String::from("John Doe"),
		sign_in_count: 1,
		active: true,
	};
	println!("{}", user1.email);
}

struct User {

	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}

