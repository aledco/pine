fn main() {
    let n1 = 10;
    let n2 = 5;
    let e = ast::Expression::new();
    
    println!("ast::test() = {}", ast::test());
    println!("ast::Expression.test = {}", e.test);
    println!("gen::add(n1, n2) = {}", gen::add(n1, n2));
    println!("pvm::add(n1, n2) = {}", pvm::add(n1, n2));
}
