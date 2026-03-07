use process;
let p = process.spawn("ls", ["-a"]);
println(p.output());
println(process.pid());
