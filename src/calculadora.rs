mod calc_result;
use calc_result::CalcResult;
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

        if self.syntax_checking() == CalcResult::SyntaxError{
            self.display = String::from("Syntax Error");
            return;
        }

        
        
        todo!();
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

    fn syntax_checking(&mut self) -> CalcResult{
        let mut number_before = false;
        let mut operator_before = false;
        for (i,opt) in self.operation.iter().enumerate(){
            match opt {
                InputOption::number(_) => {
                    if number_before {return CalcResult::SyntaxError;}
                    number_before = true;
                    operator_before = false;
                },
                InputOption::operator(o) => {
                    if operator_before {return CalcResult::SyntaxError;}

                    if i == self.operation.len()-1{
                        return CalcResult::SyntaxError;
                    }

                    if !number_before && !o.contains("-"){
                        return CalcResult::SyntaxError;
                    }

                    operator_before = true;
                    number_before = false;
                }
                
            }
        }
        CalcResult::Ok
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