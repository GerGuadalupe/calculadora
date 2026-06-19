use super::{CalcResult, InputOption, CalcPriority};
pub struct CalcTree{
    valor: InputOption,
    children: Box<(Option<CalcTree>, Option<CalcTree>)>
}
impl CalcTree{
    pub fn create_from(operaciones: &[InputOption], vec_priority: &[CalcPriority]) -> Option<Self>{
        let mut resultado = None;
        let mut i:usize = 0;
        let mut priority_check = CalcPriority::Min;
        loop {
            if operaciones.len() == 0 {break;}
            if vec_priority[i] == priority_check{
                let mut arbol = CalcTree::new(operaciones[i].clone());
                arbol.children.as_mut().0 = Self::create_from(&operaciones[..i], &vec_priority[..i]);
                arbol.children.as_mut().1 = Self::create_from(&operaciones[i+1..], &vec_priority[i+1..]);
                resultado = Some(arbol);
                break;
            }

            i += 1;

            if i == operaciones.len(){
                i = 0;
                priority_check = priority_check.elevate_priority();
            }
        }        
        resultado
    }

    fn new(valor: InputOption) -> Self{
        Self { valor, children: Box::new((None, None))}
    }


    pub fn operacion(&self)-> CalcResult{
        match &self.valor {
            InputOption::Number(n) => CalcResult::Ok(*n),
            InputOption::Operator(o) => {
                let Some(hijo_1) =  &self.children.as_ref().1
                else {
                    return CalcResult::SyntaxError;
                };
                match o.as_str() {
                    "+" => {
                        let Some(hijo_0) = &self.children.as_ref().0
                        else {
                            return CalcResult::SyntaxError;
                        };

                        hijo_0.operacion() + hijo_1.operacion()
                    }

                    "-" =>{
                        let Some(hijo_0) = &self.children.as_ref().0
                        else {
                            return CalcResult::Ok(0.0) - hijo_1.operacion()
                        };

                        hijo_0.operacion() + hijo_1.operacion()
                    }

                    "*" => {
                        let Some(hijo_0) = &self.children.as_ref().0
                        else {
                            return CalcResult::SyntaxError;
                        };

                        hijo_0.operacion() * hijo_1.operacion()
                    }

                    "/" => {
                        let Some(hijo_0) = &self.children.as_ref().0
                        else {
                            return CalcResult::SyntaxError;
                        };
                        let res = hijo_1.operacion();
                        if let CalcResult::Ok(n) = res {
                            if n == 0.0{
                                return CalcResult::MathError;
                            }
                        }
                        hijo_0.operacion()/res
                    }

                    "^" => {
                        let Some(hijo_0) = &self.children.as_ref().0
                        else {
                            return CalcResult::SyntaxError;
                        };
                        let res = hijo_1.operacion();
                        if let CalcResult::Ok(n) = res{
                            if n < 0.0 {return CalcResult::MathError;}
                        }

                        hijo_0.operacion().pow(hijo_1.operacion())
                    }

                    _ => CalcResult::SyntaxError

                }
            }
        }
    }
}