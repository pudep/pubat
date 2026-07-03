fn main(){
  let mut vec:Vec<char> = Vec::new();
  let mut i = 0;
  let j = 5;
  while i < j {
    vec.push('a');
    i += 1;
  }

  println!("{}", vec.len())
}
