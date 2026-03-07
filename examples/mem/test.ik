use mem;

println(mem.total());
println(mem.used());
println(mem.free());
println(mem.total_swap());
println(mem.used_swap());
println(mem.free_swap());

let a = [1,2,3,4,5,6];
println(mem.size_of(a));
println(mem.size_of("Hello, world!"));
println(mem.align_of(a));
