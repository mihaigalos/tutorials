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