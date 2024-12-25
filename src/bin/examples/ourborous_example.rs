use ouroboros::self_referencing;

#[self_referencing]
pub struct OurborousStr {
    list: Vec<u32>,
    #[borrows(list)]
    iter: &'this Vec<u32>,
    field: u32,
}

impl OurborousStr {
    pub fn check_borrow_field(&self) -> &u32 {
        self.borrow_field()
    }
}

pub fn test_ouroborous_func() {
    let inst = OurborousStrBuilder {
        list: vec![1, 2, 3],
        iter_builder: |list| list,
        field: 135,
    }
    .build();
    let field = inst.check_borrow_field();
    println!("the value of field is {}", field);
}
