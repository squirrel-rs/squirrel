use math for floor;

fn is_palindrome(n) {
    let original = n;
    let reversed_num = 0;

    while n > 0 {
        reversed_num = reversed_num * 10 + (n % 10);
        n = floor(n / 10.0);
    }

    return original == reversed_num;
}

let ans = 0;
let a = 100;
while a < 1000 {
    let b = 100;
    while b < 1000 {
        println("a: " + a + ", b: " + b);
        let product = a * b;

        if is_palindrome(product) {
            if product > ans {
                ans = product;
            }
        }

        b += 1;
    }

    a += 1;
}

println(ans);
