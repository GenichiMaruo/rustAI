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
    pub fn set(x: f64) -> Variable{
        Variable {
            data: x,
        }
    }
}

fn main(){
    let x = Variable::set(3.0);
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
    fn square_test(){
        let x = super::Variable::new();
        let y = functions::square(x);
        assert_eq!(y.data, 0.0);
    }
    fn add_test() {
        let x = super::Variable::new();
        let y = super::Variable::new();
        let z = super::Variable::new();
        z = functions::add(x,y);
        assert_eq!(z.data, 0.0);
    }
    fn exp_test() {
        let x = super::Variable::new();
        let y = super::Variable::new();
        y = functions::exp(x, 2);
        assert_eq!(y.data, 0.0);
    }
}