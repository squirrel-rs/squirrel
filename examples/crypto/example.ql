use crypto;

println(crypto.b64("Hello, world!"));
println(crypto.de_b64(crypto.b64("Hello, world!")));
println(crypto.sha224("Hello, world!"));
println(crypto.sha256("Hello, world!"));
println(crypto.sha384("Hello, world!"));
println(crypto.sha512("Hello, world!"));
println(crypto.md5("Hello, world!"));
