mod calc_result;
mod calc_tree;
use calc_result::CalcResult;

#[derive(Clone)]
enum InputOption{
    number(f64),
    operator(String)
}

pub struct Calculadora{
    pub display: String,
    ans: f64,
    operation: Vec<InputOption>,
}

impl Calculadora{
    pub fn calculate(&mut self) {
        self.to_operation();
        self.ans = match self.operate() {
            CalcResult::MathError =>{
                self.display = String::from("Math Error");
                return;
            },
            CalcResult::SyntaxError => {
                self.display = String::from("Syntax Error");
                return;
            },
            CalcResult::Ok(result) => result
        };    
        

        self.display = format!("{}", self.ans);
    }

    fn to_operation(&mut self){
        for s in self.display.split_whitespace(){
            if let Ok(n) = s.parse(){
                self.operation.push(InputOption::number(n));
            }
            else {
                self.operation.push(InputOption::operator(s.to_string()));
            }
        }
    }

    

    fn operate(&mut self) -> CalcResult{

        
        todo!();
    }
}

impl Default for Calculadora {
    fn default() -> Self {
        Self { 
            display: String::new(),
            ans: 0.0,
            operation: Vec::new() 
        }
    }
}