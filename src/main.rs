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
        let y = super::Variable::set(3.0);
        assert_eq!(x.data, 0.0);
        assert_eq!(y.data, 3.0);
    }
    #[test]
    fn square_test(){
        let x = super::Variable::set(3.0);
        let y = super::functions::square(x);
        assert_eq!(y.data, 9.0);
    }
    #[test]
    fn add_test() {
        let x = super::Variable::set(2.0);
        let y = super::Variable::set(5.0);
        let z = super::functions::add(x,y);
        assert_eq!(z.data, 7.0);
    }
    #[test]
    fn exp_test() {
        let x = super::Variable::set(2.0);
        let y = super::functions::exp(x, 3);
        assert_eq!(y.data, 8.0);
    }
}