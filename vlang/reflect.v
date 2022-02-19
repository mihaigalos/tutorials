struct User {
	name string
	age  int
}

struct Location {
    lat	f32
    lon	f32
}

fn get_generic_data<T>(data T) string {
    mut result := ""
	$for field in T.fields {
        result += data.$(field.name).str()
        result += ' '
	}
    return result
}

fn print_generic<T>(data T){
    println(get_generic_data<T>(data))
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
