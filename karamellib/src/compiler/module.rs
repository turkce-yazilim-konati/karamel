use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;
use std::rc::Rc;

use crate::buildin::Class;
use crate::buildin::Module;
use crate::compiler::StaticStorage;
use crate::compiler::function::find_function_definition_type;
use crate::error::generate_error_message;
use crate::parser::Parser;
use crate::syntax::SyntaxParser;
use crate::types::CompilerResult;

use super::context::KaramelCompilerContext;
use super::ast::BramaAstType;
use super::function::FunctionReference;

pub struct OpcodeModule {
    pub name: String,
    pub storage_index: usize,
    pub file_path: String,
    pub main_ast: Rc<BramaAstType>,
    pub functions: RefCell<HashMap<String, Rc<FunctionReference>>>,
    pub modules: RefCell<HashMap<String, Rc<dyn Module>>>,
    pub path: Vec<String>
}

impl OpcodeModule {
    pub fn new(name: String, file_path: String, main_ast: Rc<BramaAstType>) -> OpcodeModule {
        OpcodeModule {
            name, 
            file_path, 
            main_ast,
            functions: RefCell::new(HashMap::new()),
            modules: RefCell::new(HashMap::new()),
            storage_index: 0,
            path: Vec::new()
        }
    }
}

impl Module for OpcodeModule {
    fn get_module_name(&self) -> String {
        self.name.to_string()
    }

    fn get_path(&self) -> &Vec<String> {
        &self.path
    }

    fn get_method(&self, name: &str) -> Option<Rc<FunctionReference>> {
        self.functions.borrow().get(name).map(|method| method.clone())
    }

    fn get_module(&self, _: &str) -> Option<Rc<dyn Module>> {
        None
    }

    fn get_methods(&self) -> Vec<Rc<FunctionReference>> {
        let mut response = Vec::new();
        self.functions.borrow().iter().for_each(|(_, reference)| response.push(reference.clone()));
        response
    }

    fn get_modules(&self) -> HashMap<String, Rc<dyn Module>> {
        HashMap::new()
    }

    fn get_classes(&self) -> Vec<Rc<dyn Class>> {
        Vec::new()
    }
}

fn get_module_path(options: &KaramelCompilerContext, module_path: &PathBuf) -> Vec<String> {
    let mut path = Vec::new();
    let script_path = PathBuf::from(&options.script_path[..]);
    let mut script_path_iter = script_path.iter();
    let mut module_path_iter = module_path.iter();

    while let Some(_) = script_path_iter.next() {
        module_path_iter.next();
    }
    
    while let Some(path_part) = module_path_iter.next() {
        path.push(path_part.to_str().unwrap().to_string());
    }
    path
}

pub fn load_module(params: &[String], modules: &mut Vec<Rc<OpcodeModule>>, options: &mut KaramelCompilerContext, upper_storage_index: usize) -> Result<Rc<OpcodeModule>, String> {
    let mut path = PathBuf::from(&options.script_path[..]);
    let module = params[(params.len() - 1)].to_string();

    for item in params.iter().take(params.len() - 1) {
        path.push(item);
    }

    path.push(module.clone());

    let content = match File::open(format!("{}.tpd", path.to_str().unwrap())) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            contents
        },
        Err(error) => return Err(format!("Dosya okuma hatası oldu. Hata : {:?}", error))
    };

    let mut parser = Parser::new(&content);
    match parser.parse() {
        Err(error) => return Err(generate_error_message(&content, &error)),
        _ => ()
    };

    let syntax = SyntaxParser::new(parser.tokens().to_vec());
    return match syntax.parse() {
        Ok(ast) => {
            let module_storage = options.storages.len();
            options.storages.push(StaticStorage::new(module_storage));
            options.storages[module_storage].set_parent_location(upper_storage_index);

            let mut module = OpcodeModule::new(module, path.to_str().unwrap().to_string(), ast.clone());
            module.path = get_module_path(options, &path);
            module.storage_index = module_storage;

            let module = Rc::new(module);
            find_load_type(module.main_ast.clone(), options, modules, module.storage_index)?;
            find_function_definition_type(module.clone(), ast.clone(), options, module_storage, true)?;
            Ok(module.clone())
        },
        Err(error) => return Err(generate_error_message(&content, &error))
    };
}

fn find_load_type(ast: Rc<BramaAstType>, options: &mut KaramelCompilerContext, modules: &mut Vec<Rc<OpcodeModule>>, upper_storage_index: usize) -> CompilerResult {
    match &*ast {
        BramaAstType::Load(module_name) => {
            if !options.has_module(&module_name) {
                let module = load_module(module_name, modules, options, upper_storage_index)?;
                options.add_module(module.clone());
                modules.push(module.clone());
            }
        },
        BramaAstType::Block(blocks) => {
            for block in blocks {
                find_load_type(block.clone(), options, modules, upper_storage_index)?;
            }
        },
        _ => ()
    }

    Ok(())
}

pub fn get_modules(main_ast: Rc<BramaAstType>, options: &mut KaramelCompilerContext) -> Result<Vec<Rc<OpcodeModule>>, String> {
    let mut modules: Vec<Rc<OpcodeModule>> = Vec::new();
    match find_load_type(main_ast, options, &mut modules, 0) {
        Ok(()) => Ok(modules),
        Err(error) => Err(error)
    }
}

#[cfg(test)]
mod tests {
    use std::panic;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    use crate::compiler::context::KaramelCompilerContext;
    use crate::compiler::module::load_module;

    fn setup() {
        println!("setup");
    }

    fn teardown(to_be_removed: Vec<String>) {
        for file in to_be_removed.iter() {
            std::fs::remove_file(file).unwrap();
        }
        println!("teardown");
    }

    fn run_test<T>(test: T, to_be_removed: Vec<String>) -> ()
        where T: FnOnce() -> () + panic::UnwindSafe
    {
        setup();

        let result = panic::catch_unwind(|| {
            test()
        });

        teardown(to_be_removed);

        assert!(result.is_ok())
    }

    fn write_to_file(content: &'static str, file_name: &'static str) -> String {
        let file_name = generate_file_name(file_name);
        let mut file = File::create(&file_name).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file_name
    }

    fn get_parent() -> String {
        match std::env::current_exe() {
            Ok(path) => match path.parent() {
                Some(parent_path) => parent_path.to_str().unwrap().to_string(),
                _ => String::from(".")
            },
            _ => String::from(".")
        }
    }

    fn generate_file_name(file_name: &'static str) -> String {
        match std::env::current_exe() {
            Ok(path) => match path.parent() {
                Some(parent_path) => parent_path.clone().join(file_name).to_str().unwrap().to_string(),
                _ => Path::new(".").join(file_name).to_str().unwrap().to_string()
            },
            _ => Path::new(".").join(file_name).to_str().unwrap().to_string()
        }
    }

    #[test]
    fn test_1() {
        let module_1 = r#"
fonk topla(bir, iki): dondur bir + iki"#;
        let topla_path = write_to_file(module_1, "topla.tpd");

        run_test(|| {
            let mut modules = Vec::new();
            let mut options = KaramelCompilerContext::new();
            options.script_path = get_parent();
            match load_module(&[String::from("topla")].to_vec(), &mut modules, &mut options, 0) {
                Ok(_) => (),
                Err(error) => assert!(false, "{}", error)
            };
        }, [topla_path].to_vec());
    }

    #[test]
    fn test_2() {
        let module_1 = r#"
fonk topla(bir, iki): dondur bir + iki"#;
        let module_2 = r#"
module_1 yükle
fonk topla2(bir, iki): dondur module_1::topla(bir + iki)"#;
        let module_1_path = write_to_file(module_1, "module_1.tpd");
        let module_2_path = write_to_file(module_2, "module_2.tpd");

        run_test(|| {
            let mut modules = Vec::new();
            let mut options = KaramelCompilerContext::new();
            options.script_path = get_parent();
            match load_module(&[String::from("module_1")].to_vec(), &mut modules, &mut options, 1) {
                Ok(_) => (),
                Err(error) => assert!(false, "{}", error)
            };
            match load_module(&[String::from("module_2")].to_vec(), &mut modules, &mut options, 0) {
                Ok(_) => (),
                Err(error) => assert!(false, "{}", error)
            };
        }, [module_1_path, module_2_path].to_vec());
    }
}