use super::InputOption;
pub struct CalcTree{
    valor: InputOption,
    children: Box<(Option<CalcTree>, Option<CalcTree>)>
}

impl CalcTree{
    pub fn create_from(operaciones: Vec<InputOption>) -> Option<Self>{

    }

    fn new(valor: InputOption) -> Self{
        Self { valor, children: Box::new((None, None)) }
    }
    fn add_child(&mut self, child: CalcTree){

    }
}