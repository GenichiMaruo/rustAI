pub fn square(x: super::Variable) -> crate::Variable{
    let y = super::Variable::new();
    y.data = x.data * x.data;
    println!("{}",y.data);
    return y;
}

pub fn add(x: super::Variable, y: super::Variable) -> crate::Variable{
    let z = super::Variable::new();
    z.data = x.data + x.data;
    println!("{}",z.data);
    return y;
}

pub fn exp(x: super::Variable, y: i32) -> crate::Variable{
    let z = super::Variable::new();
    for n in 1..y {
        z.data = z.data * x.data;
    }
    println!("{}",z.data);
    return z;
}