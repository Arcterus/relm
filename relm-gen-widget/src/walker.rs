/*
 * Copyright (c) 2017 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

use syn::{Expr, Ident};
use syn::ExprKind::{Field, Path};
use syn::visit::{Visitor, walk_expr};

pub struct ModelVariableVisitor {
    pub idents: Vec<Ident>,
}

impl ModelVariableVisitor {
    pub fn new() -> Self {
        ModelVariableVisitor {
            idents: vec![],
        }
    }
}

impl Visitor for ModelVariableVisitor {
    fn visit_expr(&mut self, expr: &Expr) {
        if let Field(ref obj, ref field) = expr.node {
            if let Path(_, ref path) = obj.node {
                if path.segments[0].ident == Ident::new("model") {
                    self.idents.push(field.clone());
                }
            }
        }
        walk_expr(self, expr);
    }
}
