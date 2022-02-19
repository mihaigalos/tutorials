struct User {
	name string
	age  int
}


//   fn is_metadata(field_name string) {
//       match field_name {
//           "is_pub" { true }
//           "is_mut" { true }
//           "is_shared" { true }
//           "typ" { true }
//           else { false }
//       }
//   }

//   fn print_fields(data FieldData) {
//
//       $for field in FieldData.fields {
//               println('user.${field} ')
//               println('+++++')
//
//       }
//   }

fn (user User) print() {
	$for field in User.fields {
        //if !is_metadata(user.'$(field)')
        {
            println('user.${field} ')
            println('----------')
        }
	}
}

fn main() {
    user := User{"Mihai", 36}
    user.print()

}
