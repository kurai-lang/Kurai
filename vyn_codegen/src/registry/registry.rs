use std::{cell::RefCell, collections::HashMap, mem, rc::Rc, sync::{Arc, RwLock}};

use inkwell::attributes::AttributeLoc;
use vyn_attr::attribute::Attribute;
use vyn_ast::expr::Expr;
use vyn_ast::stmt::Stmt;
use vyn_core::scope::Scope;

use vyn_parser::parse::Parser;
use vyn_types::{typ::Type, value::Value};

use crate::codegen::CodeGen;

// use vyn_codegen_traits::codegen_print::CodeGenPrint;
//
// pub struct CodeGenRegistry<'ctx> {
//     store: HashMap<String, Box<dyn CodeGenPrint<'ctx> + 'ctx>>,
// }

pub trait AttributeHandlerClone: Send + Sync {
    fn call(&self, attr_name: &str, stmt: &Stmt, codegen: &mut CodeGen);
    fn clone_box(&self) -> Box<dyn AttributeHandlerClone>;
}

// Clonin
impl<T> AttributeHandlerClone for T
where
    T: Fn(&str, &Stmt, &mut CodeGen) + Send + Sync + Clone + 'static,
{
    fn call(&self, attr_name: &str, stmt: &Stmt, codegen: &mut CodeGen) {
        (self)(attr_name, stmt, codegen)
    }

    fn clone_box(&self) -> Box<dyn AttributeHandlerClone> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn AttributeHandlerClone> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Default, Clone)]
pub struct AttributeRegistry {
    pub handlers: HashMap<String, AttributeHandler>,
    pub parser: Parser,
}

impl AttributeRegistry {
    pub fn register<F>(&mut self, name: &str, handler: F)
    where
        F: Fn(&str, &Stmt, &mut CodeGen) + Send + Sync + Clone + 'static,
    {
        // just for the simplicity and convenience! XD
        self.handlers.insert(name.to_string(), AttributeHandler::new(handler));
    }

    pub fn register_all(&mut self, expected_type: Option<&Type>, discovered_modules: &mut Vec<String>) {
        let scope = Arc::new(RwLock::new(Scope::new()));
        let scope = Arc::clone(&scope);
        let expected_type = expected_type.cloned(); // Option<Type>, not Option<&Type>

        let parser = Arc::new(RwLock::new(Parser::new()));
        let parser_ref = parser.clone();

        self.register(
            "test", 
            move |attr_name, _, ctx| {
            // let mut local_modules: Vec<String> = vec![];
            let mut scope_ref = scope.write().unwrap();
            // let mut discovered_modules = vec![]; // empty or cloned vec

            ctx.printf(&vec![Expr::Literal(Value::Str("TEST ATTRIBUTE HAS BEEN CALLED".to_string()))],
                expected_type.as_ref(),
                &mut parser_ref.write().unwrap()
            ).unwrap();
        });

        // #[inline] attribute 
        self.register(
            "inline",
            move |_, stmt, ctx| {
                if let Stmt::FnDecl { name, ..} = stmt {
                    ctx.inline_fns.insert(name.to_string());

                    // adds the tag to the function for inlining
                    if let Some(llvm_fn) = ctx.module.lock().unwrap().get_function(name.as_str()) {
                        llvm_fn.add_attribute(
                            AttributeLoc::Function,
                            ctx.context.create_enum_attribute(
                                inkwell::attributes::Attribute::get_named_enum_kind_id("alwaysinline"), 
                                0),
                        );
                    }
                }
            });

        // NOTE: PLANNING:
        // #[raw] - raw memory, no protection. fully disables all safety checks 
        // (pointer checks, null checks, casting checks, etc.)
        // ----------------
        // #[unchecked] - no runtime checks, for something like.. 
        // `let x = 10 / 0`, division by zero. (i know, very unsafe)
        // ----------------
        // #[do_not_try_this_at_home] - same as raw but renamed for funny purpose xDD
        // ----------------
        // #[no_cast] - disables all implicit type promotion/casting (such as, `as`)
        // ----------------
        // #[no_mangle] - naturally, llvm ir will generate strange names. this attribute prevents
        // strange naming and actually names the way the user wanted the specific function to be
        // ----------------
        // #[stack_only] - no heap, only stack memory. slow and predictable memory.
        // ----------------
    }

    pub fn _load_attributes(&self, attributes: &[Attribute], stmt: &Stmt, codegen: &mut CodeGen) {
        for attr in attributes {
            match attr {
                Attribute::Simple(name) | Attribute::WithArgs { name, .. } => {
                    let attr_registry = self.handlers.clone(); // needs Clone

                    // This checks if the attribute name is available or not
                    if let Some(handler) = attr_registry.get(name.as_str()) {
                        handler.call(name, stmt, codegen);
                    }
                }
                _ => ()
            }
        }
    }
}

impl<'ctx> CodeGen<'ctx> {
    pub fn load_attributes(&mut self, attributes: &[Attribute], stmt: &Stmt) {
        let attr_registry = mem::take(&mut self.attr_registry);
        let mut temp_registry = attr_registry;

        temp_registry._load_attributes(attributes, stmt, self);
        self.attr_registry = temp_registry;
    }
}

pub struct AttributeHandler {
    inner: Box<dyn AttributeHandlerClone>,
}

impl AttributeHandler {
    pub fn new<T>(f: T) -> Self
    where 
        T: Fn(&str, &Stmt, &mut CodeGen) + Send + Sync + Clone + 'static,
    {
        Self {
            inner: Box::new(f),
        }
    }

    pub fn call(&self, attr_name: &str, stmt: &Stmt, codegen: &mut CodeGen) {
        self.inner.call(attr_name, stmt, codegen);
    }
}

impl Clone for AttributeHandler {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone_box(),
        }
    }
}

// impl fmt::Debug for AttributeHandler {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         self.inner.fmt_debug(f)
//     }
// }
