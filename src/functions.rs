pub fn square(x: super::Variable) -> crate::Variable{
    let y = super::Variable::new();
    y.data = x.data * x.data;
    println!("{}",y.data);
    return y;
}