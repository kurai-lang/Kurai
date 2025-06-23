use core::fmt;
use std::{collections::HashMap, mem};

use inkwell::attributes::AttributeLoc;
use kurai_attr::attribute::Attribute;
use kurai_expr::expr::Expr;
use kurai_stmt::stmt::Stmt;
use kurai_typedArg::typedArg::TypedArg;
use kurai_types::{typ::Type, value::Value};

use crate::codegen::CodeGen;

// use kurai_codegen_traits::codegen_print::CodeGenPrint;
//
// pub struct CodeGenRegistry<'ctx> {
//     store: HashMap<String, Box<dyn CodeGenPrint<'ctx> + 'ctx>>,
// }

pub trait AttributeHandlerClone: Send + Sync {
    fn call(&self, attr_name: &str, stmt: &Stmt, codegen: &mut CodeGen);
    fn clone_box(&self) -> Box<dyn AttributeHandlerClone>;
}

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
}

impl AttributeRegistry {
    pub fn register<F>(&mut self, name: &str, handler: F)
    where
        F: Fn(&str, &Stmt, &mut CodeGen) + Send + Sync + Clone + 'static,
    {
        // just for the simplicity and convenience! XD
        self.handlers.insert(name.to_string(), AttributeHandler::new(handler));
    }

    pub fn register_all(&mut self) {
        self.register(
            "test", 
            move |attr_name, _, ctx| {
            ctx.import_printf().unwrap();
            ctx.printf(&vec![TypedArg {
                name: attr_name.to_string(),
                typ: Type::Str,
                value: Some(Expr::Literal(Value::Str("TEST ATTRIBUTE HAS BEEN CALLED".to_string())))
            }]).unwrap();
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
