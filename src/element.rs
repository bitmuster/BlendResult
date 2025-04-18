pub enum ElementType {
    Suite,
    Test,
    Keyword,
}

pub enum ResultType {
    Pass,
    Fail,
    None,
}

pub struct Element {
    pub et: ElementType,
    pub children: Vec<Element>,
    pub result: ResultType,
}
