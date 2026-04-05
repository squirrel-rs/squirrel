#[ A kitten ]#
class Kitten {
    # Init called when a Kitten is born
    fn init(self, food) {
        # Kitten has age and food level
        self.age = 0.0;
        self.food = food;
    }

    # Increments cat age by 0.1
    fn grow(self) {
        self.age += 0.1;
    }

    # Increments food level by amount
    fn feed(self, amount) {
        self.food += amount;
    }
}

# Creating a kitten with food level = 10.0
let kitten = Kitten(10.0);
kitten.feed(15.0);
kitten.grow();
println("food: " + str_of(kitten.food) + ", age: " + str_of(kitten.age));
