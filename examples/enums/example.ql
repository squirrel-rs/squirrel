enum Dog {
  Poodle,  # 0
  Bulldog, # 1
  Beagle,
  Husky
}
let dog = Dog.Poodle;
println(dog == 0); # true
println(dog == Dog.Beagle); # false
