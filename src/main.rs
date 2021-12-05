//main rustAI code

struct Variable{
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn variable_test() {
        let x = super::Variable::new();
        assert_eq!(x.data, 0.0);
    }
}