pub mod number;
pub mod text;
pub mod list;
pub mod baseclass;

use crate::{buildin::class::baseclass::BasicInnerClass, compiler::{BramaPrimative, GetType, function::NativeCall}};
use std::{sync::Arc, vec::Vec};
use lazy_static::lazy_static;

use super::ClassProperty;


pub fn get_empty_class() -> BasicInnerClass {
    let mut opcode = BasicInnerClass::default();
    opcode.set_name("__NO__CLASS__");
    opcode
}

lazy_static! {
    pub static ref PRIMATIVE_CLASSES: Vec<BasicInnerClass> = {
        let mut m = Vec::new();
        m.push(number::get_primative_class());
        m.push(text::get_primative_class());
        m.push(list::get_primative_class());
        m.push(get_empty_class());
        m.push(get_empty_class());
        m.push(get_empty_class());
        m.push(get_empty_class());
        m.push(get_empty_class());
        m.push(get_empty_class());
        m.push(get_empty_class());
        m
    };
}

#[macro_export]
macro_rules! nativecall_test {
    ($name:ident, $function_name:ident, $query:expr, $result:expr) => {
        #[test]
        fn $name () {
            use std::cell::RefCell;
            let stack: Vec<VmObject> = Vec::new();
            let stdout = Some(RefCell::new(String::new()));
            let stderr = Some(RefCell::new(String::new()));
            
            let parameter = FunctionParameter::new(&stack, Some(Arc::new($query)), 0, 0, &stdout, &stderr);
            let result = $function_name(parameter);
            assert!(result.is_ok());
            let object = result.unwrap().deref();
            assert_eq!(*object, $result);
        }
    };
}

#[macro_export]
macro_rules! primative_text {
    ($text:expr) => {
        BramaPrimative::Text(Arc::new($text.to_string()))
    };
}

#[macro_export]
macro_rules! nativecall_test_with_params {
    ($name:ident, $function_name:ident, $query:expr, $params:expr, $result:expr) => {
        #[test]
        fn $name () {
            use std::cell::RefCell;
            let stack: Vec<VmObject> = $params.to_vec();
            let stdout = Some(RefCell::new(String::new()));
            let stderr = Some(RefCell::new(String::new()));
            
            let parameter = FunctionParameter::new(&stack, Some(Arc::new($query)), stack.len() as usize, stack.len() as u8, &stdout, &stderr);
            let result = $function_name(parameter);
            assert!(result.is_ok());
            let object = result.unwrap().deref();
            assert_eq!(*object, $result);
        }
    };
}

#[macro_export]
macro_rules! n_parameter_check {
    ($function_name:expr, $parameter_size:expr) => {
        if parameter.length() > 1 {
            return n_parameter_expected!("tür_bilgisi", 1);
        }
    };
}

#[macro_export]
macro_rules! n_parameter_expected {
    ($function_name:expr, $parameter_size:expr) => { Err(format!("'{}' fonksiyonu {} parametre kabul ediyor", $function_name, $parameter_size)) };
    ($function_name:expr, $parameter_size:expr, $parameter_found:expr) => { Err(format!("'{}' fonksiyonu {} parametre kabul ediyor, fakat {} adet parametre bulundu", $function_name, $parameter_size, $parameter_found)) };
}

#[macro_export]
macro_rules! expected_parameter_type {
    ($function_name:expr, $expected_type:expr) => { Err((format!("'{}' sadece {} parametresini kabul ediyor", $function_name, $expected_type))) };
}



trait Class: GetType {
    fn get_type(&self) -> String;
    fn has_element(&self, field: Arc<BramaPrimative>) -> bool;
    fn element_count(&self) -> usize;
    fn add_method(&mut self, name: &String, function: NativeCall);
    fn add_property(&mut self, name: &String, property: Arc<BramaPrimative>);
    fn get_element(&self, field: Arc<BramaPrimative>) -> Option<&ClassProperty>;
}
