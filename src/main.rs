//main rustAI code

mod functions;

pub struct Variable{
    data: f64,
}
impl Variable{
    pub fn new() -> Variable{
        Variable {
            data: 0.0
        }
    }
}

fn main(){
    let x = Variable::new();
    println!("{}",x.data);
    let y = functions::square(x);
    println!("{}",y.data);
}

#[cfg(test)]
mod tests {
    #[test]
    fn variable_test() {
        let x = super::Variable::new();
        assert_eq!(x.data, 0.0);
    }
}