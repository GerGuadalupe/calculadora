mod calc_result;
mod calc_tree;
use calc_result::CalcResult;
use calc_tree::CalcTree;

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
            CalcResult::Ok(ans) => ans
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
        let operaciones = &self.operation;
        let mut  vec_priority: Vec<CalcPriority> = Vec::new();

        for operacion in operaciones {
            match operacion {
                InputOption::number(_) => vec_priority.push(CalcPriority::Number),
                InputOption::operator(operator) => {
                    match operator.as_str(){
                        "+" => vec_priority.push(CalcPriority::Min),
                        "-" => vec_priority.push(CalcPriority::Min),
                        "*" => vec_priority.push(CalcPriority::Med),
                        "/" => vec_priority.push(CalcPriority::Med),
                        "^" => vec_priority.push(CalcPriority::Max),
                        _ => return CalcResult::MathError
                    }

                }
            }


            let Some(arbol) = CalcTree::create_from(&operaciones, &vec_priority)
            else {
                return CalcResult::SyntaxError;
            }; 
        };





        
        
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

#[derive(PartialEq)]
enum CalcPriority{
    Min,
    Med,
    Max,
    Number
}

impl CalcPriority {
    fn elevate_priority(&mut self) -> Self{
        match self {
            CalcPriority::Min => CalcPriority::Med,
            CalcPriority::Med => CalcPriority::Max,
            CalcPriority::Max => CalcPriority::Number,
            CalcPriority::Number => panic!("se ha intentado usar un numero como operador")
        }
    }
}