mod calc_result;
mod calc_tree;
use calc_result::CalcResult;
use calc_tree::CalcTree;

#[derive(Clone)]
pub enum InputOption {
    Number(f64),
    Operator(String),
}

pub struct Calculadora {
    display: String,
    ans: f64,
    operation: Vec<InputOption>,
}

impl Calculadora {
    pub fn calculate(&mut self) {
        self.operation.clear();
        self.to_operation();
        self.ans = match self.operate() {
            CalcResult::MathError => {
                self.display = String::from("Math Error");
                return;
            }
            CalcResult::SyntaxError => {
                self.display = String::from("Syntax Error");
                return;
            }
            CalcResult::Ok(ans) => ans,
        };

        self.ans();
    }

    fn to_operation(&mut self) {
        for s in self.display.split_whitespace() {
            if let Ok(n) = s.parse() {
                self.operation.push(InputOption::Number(n));
            } else {
                self.operation.push(InputOption::Operator(s.to_string()));
            }
        }
    }

    fn operate(&mut self) -> CalcResult {
        let operaciones = &self.operation;
        let mut vec_priority: Vec<CalcPriority> = Vec::new();

        for operacion in operaciones {
            match operacion {
                InputOption::Number(_) => vec_priority.push(CalcPriority::Number),
                InputOption::Operator(operator) => match operator.as_str() {
                    "+" => vec_priority.push(CalcPriority::Min),
                    "-" => vec_priority.push(CalcPriority::Min),
                    "*" => vec_priority.push(CalcPriority::Med),
                    "/" => vec_priority.push(CalcPriority::Med),
                    "^" => vec_priority.push(CalcPriority::Max),
                    _ => return CalcResult::SyntaxError,
                },
            }
        }

        let Some(arbol) = CalcTree::create_from(&operaciones, &vec_priority) else {
            return CalcResult::SyntaxError;
        };
        arbol.operacion()
    }
}

impl Default for Calculadora {
    fn default() -> Self {
        Self {
            display: String::new(),
            ans: 0.0,
            operation: Vec::new(),
        }
    }
}

#[derive(PartialEq)]
enum CalcPriority {
    Min,
    Med,
    Max,
    Number,
}

impl CalcPriority {
    fn elevate_priority(&mut self) -> Self {
        match self {
            CalcPriority::Min => CalcPriority::Med,
            CalcPriority::Med => CalcPriority::Max,
            CalcPriority::Max => CalcPriority::Number,
            CalcPriority::Number => panic!("no se puede elevar la prioridad de un numero"),
        }
    }
}

impl Calculadora {
    pub fn ans(&mut self){
        if self.ans.to_string().len() >= 15{
            self.display = format!("{:e}", self.ans)
        }
        else {
            self.display = self.ans.to_string();
        }
    }

    pub fn display(&self) -> &str{
        &self.display
    }

    pub fn push_in_display(&mut self, string: &str){
        self.display.push_str(string);
    }

    pub fn delete(&mut self){
        match self.display.as_str() {
            "Math Error" => self.display.clear(),
            "Syntax Error" => self.display.clear(),

            _ => {
                if let Some(ch) = self.display.pop() {
                    if ch == ' ' {
                        self.display.pop();
                        self.display.pop();
                    }
                }
            }
        }
    }
}
