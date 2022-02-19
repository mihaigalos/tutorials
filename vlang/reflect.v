struct User {
	name string
	age  int
}

struct Location {
    lat	f32
    lon	f32
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
    mut locations := []Location{cap: 100}
    locations << Location{3.14, 2.71}
    locations << Location{12.34, 56.78}
    
    for location in locations{
        print_generic(location)
    }
}
