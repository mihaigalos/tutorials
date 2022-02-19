module main

struct User {
	name string
	age  int
}

struct Location {
	lat f32
	lon f32
}

fn get_generic_data<T>(data T) string {
	mut result := ''
	$for field in T.fields {
		result += data.$(field.name).str()
		result += ' '
	}
	return result
}

fn print_generic<T>(data T) {
	println(get_generic_data<T>(data))
}

fn test_users_when_typical() {
	mut users := []User{cap: 100}
	users << User{'Foo', 55}
	users << User{'Bar', 22}

	assert get_generic_data(users[0]) == 'Foo 55 '
	assert get_generic_data(users[1]) == 'Bar 22 '
}


fn test_location_when_typical() {
	mut locations := []Location{cap: 100}
	locations << Location{3.14, 2.71}
	locations << Location{12.34, 56.78}

	assert get_generic_data(locations[0]) == '3.14 2.71 '
	assert get_generic_data(locations[1]) == '12.34 56.78 '
}