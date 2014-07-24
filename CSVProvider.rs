#![crate_id="CSVProvider#0.1-pre"]
#![crate_type="dylib"]

// why doesn't "lib" work?

// rustc 0.11.0-pre (b47f2226a25654c5b781d27a91f2fa5274b3a347 2014-06-28 14:31:37 +0000)


#![feature(globs, macro_rules, quote, managed_boxes, plugin_registrar)]

#![allow(unused_imports)]
//#![allow(unused_variable)]
#![allow(uppercase_variables)]

extern crate syntax;
extern crate rustc;

use syntax::ast::{
   Name,
   TokenTree,
   Expr,
   ExprLit,
   ExprVec,
   LitStr,
   Ident,
   DUMMY_NODE_ID,
   MutImmutable
};
use syntax::codemap::Span;
use syntax::ext::base::{
   SyntaxExtension,
   BasicMacroExpander,
   ExtCtxt,
   MacResult,
   MacPat,
   MacItem,
   DummyResult,
   MacExpr,
   NormalTT
};
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
                  cx.span_err(entry.span, "expected string literal (1)");
                  error = true;
                  InternedString::new("")
               }
            }
         }
         _ => {
                cx.span_err(entry.span, "expected string literal (2)");
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

   let raw_name   : Entry = entries.shift().expect("should be given a type name");
   let raw_path   : Entry = entries.shift().expect("should be given a CSV file path");
   let raw_labels : Entry = entries.shift().expect("should be given column labels");

   let name   : InternedString = raw_name.str;
   let path   : InternedString = raw_path.str;
   let labels : InternedString = raw_labels.str;

   println!("provide_csv_given_labels: name:   {}", name);
   println!("provide_csv_given_labels: path:   {}", path);
   println!("provide_csv_given_labels: labels: {}", labels);

   // PENDING
   // read the CSV and try to get the type of data in it
   // create a type with
   //  * this name,
   //  * these labeled columns,
   //  * discovered type of data
   //  * a constructor which reads the whole file

   // try 1
   //let ns: &str = "MyCSV";
   //let MyCSV: Ident = token::str_to_ident(ns);

   // try 2
   //let ns: String = format!("{}", name);
   //let MyCSV: Ident = token::str_to_ident(ns);
   
   // try 3
   let nss: String = format!("{}", name);
   let ns: &str = nss.as_slice();
   let MyCSV: Ident = token::str_to_ident(ns);

   // FIXME: Why is this fn necessary?
   let define_my_csv = |cx0 : &mut ExtCtxt| {
      let item1: Option<Gc<syntax::ast::Item>> = quote_item!(cx0,
         pub struct $MyCSV {
            pub data: Vec<(String)>,
         }
      );
      return item1;
   };

   let item1: Option<Gc<syntax::ast::Item>> = define_my_csv(cx);

   let item2: Option<Gc<syntax::ast::Item>> = quote_item!(cx,
      impl $MyCSV {
         pub fn new() -> $MyCSV {
            println!("HMMMMMM.");
            return $MyCSV {
               data: (vec!["zero".to_string(), "one".to_string(), "two".to_string()]),
            };
         }
      }
   );

   let items = vec![item1.expect("Should be able to construct MyCSV."),
                    item2.expect("Should be able to construct MyCSV.")];
   return MacItems::new(items);
}

