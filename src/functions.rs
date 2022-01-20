pub fn square(x: super::Variable) -> super::Variable{
    super::Variable{
        data: x.data * x.data,
    }
}

pub fn add(x: super::Variable, y: super::Variable) -> super::Variable{
    super::Variable{
        data: x.data + y.data,
    }
}

pub fn exp(x: super::Variable, y: i32) -> super::Variable{
    let mut z: f64;
    z = 1.0;
    for _n in 1..y {
        z *= x.data;
    }
    super::Variable{
        data: z,
    }
}