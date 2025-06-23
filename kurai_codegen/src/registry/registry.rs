use core::fmt;
use std::collections::HashMap;

use kurai_stmt::stmt::Stmt;

use crate::codegen::CodeGen;

// use kurai_codegen_traits::codegen_print::CodeGenPrint;
//
// pub struct CodeGenRegistry<'ctx> {
//     store: HashMap<String, Box<dyn CodeGenPrint<'ctx> + 'ctx>>,
// }

pub trait AttributeHandlerClone: Send + Sync {
    fn call(&self, stmt: &Stmt, codegen: &mut CodeGen);
    fn clone_box(&self) -> Box<dyn AttributeHandlerClone>;
    // fn fmt_debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl<T> AttributeHandlerClone for T
where
    T: Fn(&Stmt, &mut CodeGen) + Send + Sync + Clone + 'static,
{
    fn call(&self, stmt: &Stmt, codegen: &mut CodeGen) {
        (self)(stmt, codegen)
    }

    fn clone_box(&self) -> Box<dyn AttributeHandlerClone> {
        let cloned: T = self.clone();
        let boxed = Box::new(cloned);
        boxed
    }

    // fn fmt_debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //     fmt::Debug::fmt(self, f)
    // }
}

#[derive(Clone)]
pub struct AttributeRegistry {
    pub handlers: HashMap<String, AttributeHandler>,
}

impl AttributeRegistry {
    pub fn register<F>(&mut self, name: &str, handler: F)
    where
        F: Fn(&Stmt, &mut CodeGen) + Send + Sync + Clone + 'static,
    {
        // just for the simplicity and convenience! XD
        self.handlers.insert(name.to_string(), AttributeHandler::new(handler));
    }
}

pub struct AttributeHandler {
    inner: Box<dyn AttributeHandlerClone>,
}

impl AttributeHandler {
    pub fn new<T>(f: T) -> Self
    where 
        T: Fn(&Stmt, &mut CodeGen) + Send + Sync + Clone + 'static,
    {
        Self {
            inner: Box::new(f),
        }
    }

    pub fn call(&self, stmt: &Stmt, codegen: &mut CodeGen) {
        self.inner.call(stmt, codegen);
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
