module main

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