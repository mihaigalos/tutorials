struct User {
	name string
	age  int
}

fn print_generic<T>(data T) {
	$for field in T.fields {
        print(data.$(field.name))
        print(' ')
	}
    println('')
}

fn main() {
    mut users := []User{cap: 100}
    users << User{"Foo", 55}
    users << User{"Bar", 22}
    
    for user in users{
        print_generic(user)
    }
}
