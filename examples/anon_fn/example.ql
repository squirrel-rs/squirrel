let a = || 1;
let b = |a| a + 1;
let c = |a| {
    return a + 1;
};
println(a());
println(b(1));
println(c(2));
