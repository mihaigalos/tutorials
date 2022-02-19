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
    user1 := User{"Foo", 55}
    users << user1
    user2 := User{"Bar", 22}
    users << user2
    
    for user in users{
        user.print()
    }
}
