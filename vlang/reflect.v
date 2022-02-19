struct User {
	name string
	age  int
}

fn (user User) print() {
	$for field in User.fields {
        print(user.$(field.name))
        print(' ')
	}
    println('')
}

fn main() {
    mut users := []User{cap: 100}
    users << User{"Foo", 55}
    users << User{"Bar", 22}
    
    for user in users{
        user.print()
    }
}
