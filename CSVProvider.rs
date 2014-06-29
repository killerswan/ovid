#![crate_id="CSVProvider#0.1-pre"]
#![crate_type="dylib"]

// why doesn't "lib" work?

// rustc 0.11.0-pre (b47f2226a25654c5b781d27a91f2fa5274b3a347 2014-06-28 14:31:37 +0000)


#![feature(globs, macro_rules, quote, managed_boxes, plugin_registrar)]

#![allow(unused_imports)]
#![allow(unused_variable)]

extern crate syntax;
extern crate rustc;

use syntax::ast::{Name,
                  TokenTree,
                  Expr,
                  ExprLit,
                  ExprVec,
                  LitStr,
                  DUMMY_NODE_ID,
                  MutImmutable};
use syntax::codemap::Span;
use syntax::ext::base::{SyntaxExtension,
                        BasicMacroExpander,
                        ExtCtxt,
                        MacResult,
                        MacPat,
                        MacItem,
                        DummyResult,
                        MacExpr,
                        NormalTT};
use std::gc::Gc;
use syntax::parse;
use syntax::parse::token;
use syntax::parse::token::{InternedString, COMMA, EOF};

use std::io;
use std::io::fs;
use std::os;
use std::str;
use std::io::Process;
use syntax::ast;
use syntax::ast::Name;

use syntax::util::small_vector::SmallVector;

use std::collections::HashMap;
use std::gc::{Gc, GC};

use rustc::plugin::Registry;

#[plugin_registrar]
pub fn macro_registrar(reg: &mut Registry) {
    reg.register_macro("ProvideCSV_labels", provide_csv_given_labels);
}

#[deriving(Clone)]
struct Entry {
    str: InternedString,
    expr: Gc<Expr>
}

// see https://github.com/sfackler/syntax-ext-talk/blob/gh-pages/simple-ext/lib.rs
fn parse_entries(cx: &mut ExtCtxt, tts: &[TokenTree]) -> Option<Vec<Entry>> {
    let mut parser = parse::new_parser_from_tts(
         cx.parse_sess(),
         cx.cfg(),
         tts.iter()
            .map(|x| (*x).clone())
            .collect()
    );

    let mut entries: Vec<Entry> = Vec::new();

    let mut error = false;
    while parser.token != EOF {
        let entry = parser.parse_expr();

        let entry_str = match entry.node {
            ExprLit(lit) => {
                match lit.node {
                    LitStr(ref s, _) => s.clone(),
                    _ => {
                        cx.span_err(entry.span, "expected string literal");
                        error = true;
                        InternedString::new("")
                    }
                }
            }
            _ => {
                cx.span_err(entry.span, "expected string literal");
                error = true;
                InternedString::new("")
            }
        };

        entries.push(Entry { str: entry_str, expr: entry });

        if !parser.eat(&COMMA) && parser.token != EOF {
            cx.span_err(parser.span, "expected `,`");
            return None;
        }
    }

    if error {
        return None;
    }

    Some(entries)
}

/// A convenience type for macros that return a single item.
pub struct MacItems {
    items: Vec<Gc<ast::Item>>
}

impl MacItems {
    pub fn new(items: Vec<Gc<ast::Item>>) -> Box<MacResult> {
        box MacItems { items: items } as Box<MacResult>
    }
}
impl MacResult for MacItems {
    fn make_items(&self) -> Option<SmallVector<Gc<ast::Item>>> {
        Some(SmallVector::many(self.items.clone()))
    }
}

fn provide_csv_given_labels(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> Box<MacResult> {
   let mut entries = match parse_entries(cx, tts) {
      Some(entries) => entries,
      None => return DummyResult::expr(sp),
   };

   let name   = entries.shift().expect("should be given a type name");
   let path   = entries.shift().expect("should be given a CSV file path");
   let labels = entries.shift().expect("should be given column labels");

   println!("provide_csv_given_labels: name:   {}", name.str);
   println!("provide_csv_given_labels: path:   {}", path.str);
   println!("provide_csv_given_labels: labels: {}", labels.str);

   // PENDING
   // read the CSV and try to get the type of data in it
   // create a type with
   //  * this name,
   //  * these labeled columns,
   //  * discovered type of data
   //  * a constructor which reads the whole file

/* OLD
   let banana = "YAY! BANANA!";
   return MacExpr::new(quote_expr!(cx, {let x: $banana = 44}));
 */

   fn define_my_csv(cx0: &mut ExtCtxt) -> Option<Gc<syntax::ast::Item>> {
      let item1: Option<Gc<syntax::ast::Item>>  = quote_item!(cx0,
         pub struct MyCSV {
            pub data: Vec<(String)>,
         }
      );
      return item1;
   }

   let item1 = define_my_csv(cx);  // Why is this necessary?

   let item2: Option<Gc<syntax::ast::Item>>  = quote_item!(cx,
      impl MyCSV {
         pub fn new() -> MyCSV {  // BUG: note removing the return time leads to goofy error messsages
            println!("HMMMMMM.");
            return MyCSV {
               data: (vec!["zero".to_string(), "one".to_string(), "two".to_string()]),
            };
         }
      }
   );

   //let mut items = Vec::new();
   //items.push(item1.expect("Should be able to construct MyCSV."));
   //items.push(item2.expect("Should be able to construct MyCSV."));
   let items = vec![item1.expect("Should be able to construct MyCSV."),
                    item2.expect("Should be able to construct MyCSV.")];
   return MacItems::new(items);
}

