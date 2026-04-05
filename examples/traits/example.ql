#[ Represents pet trait ]#
trait Pet {
    fn feed(self, amount)
}

#[ Represents a Cat, meow :3 ]#
class Cat {
    # Cat has method `feed`
    fn feed(self, amount) {
        self.food = amount;
    }
}

#[ Represents a Toad ]#
class Toad {
    # Toad doesn't have method `feed`, but `stroke`
    fn stroke(self) {
        println("Croak! Croak!");
    }
}

let cat = Cat();
if cat >: Pet {
    println("`Cat` impls `Pet`");
}

let toad = Toad();
if toad >! Pet {
    println("`Toad` doesn't impl `Pet`");
}
