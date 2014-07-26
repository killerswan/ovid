#![crate_name="CSVProvider"]
#![crate_type="dylib"]

// why doesn't "lib" work?

// rustc 0.11.0-pre (b47f2226a25654c5b781d27a91f2fa5274b3a347 2014-06-28 14:31:37 +0000)


#![feature(globs, macro_rules, quote, managed_boxes, plugin_registrar)]

#![allow(unused_imports)]
//#![allow(unused_variable)]
#![allow(uppercase_variables)]
#![allow(unnecessary_parens)]

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
use syntax::ext::quote::{
   //expand_parse_call
};
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

fn parse_csv_row(raw: &str, expected_columns: Option<uint>) -> str::StrSplits {
   let mut it = raw.split_str(",");

   // take a pulse
   let _whatever = it.map(|xx| {
      println!("iterating over row elements: {}", xx.to_string());
      return xx;
   });

   match expected_columns {
      Some(count) => {
         if (count == it.count()) {
            println!("parse_csv_row: matching length expected");
         } else {
            println!("parse_csv_row: wrong length!");
         }
      },
      None => (),
   }

   return it;
}

fn provide_csv_given_labels(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> Box<MacResult> {
   let mut entries = match parse_entries(cx, tts) {
      Some(entries) => entries,
      None => return DummyResult::expr(sp),
   };

   let icx : &ExtCtxt = cx;  // an immutable borrow

   let raw_name   : Entry = entries.remove(0).expect("should be given a type name");
   let raw_path   : Entry = entries.remove(0).expect("should be given a CSV file path");
   let raw_labels : Entry = entries.remove(0).expect("should be given column labels");

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

   fn interned_to_ident(xx: InternedString) -> Ident {
      let yy: String = format!("{}", xx);
      let zz: &str = yy.as_slice();
      return token::str_to_ident(zz);
   }

   fn interned_to_ident_with_suffix(xx: InternedString, suff: &str) -> Ident {
      let yy: String = format!("{}{}", xx, suff);
      let zz: &str = yy.as_slice();
      return token::str_to_ident(zz);
   }

   let MyCsv    = interned_to_ident(name.clone());
   let MyCsvRow = interned_to_ident_with_suffix(name, "Row");
   let col0     = interned_to_ident(labels.clone());

   println!("Parsing given labels...");

   let labels1 = format!("{}", labels);
   let labels_iter = parse_csv_row(labels1.as_slice(), None);

   println!("Defining items in the CSV provider...");

   // TODO: iterate through call columns
   let col = quote_item!(icx,
      pub $col0: String
   ).expect("column parsing");

   let item0 = quote_item!(icx,
      pub struct $MyCsvRow {
         $col,
      }
   );

   let item1: Option<Gc<syntax::ast::Item>> = quote_item!(icx,
      pub struct $MyCsv {
         pub data: Vec<$MyCsvRow>,
      }
   );

   let item2: Option<Gc<syntax::ast::Item>> = quote_item!(icx,
      impl $MyCsv {
         pub fn new() -> $MyCsv {
            println!("HMMMMMM.");
            return $MyCsv {
               data: (vec![
                  $MyCsvRow { $col0: "zero".to_string() },
                  $MyCsvRow { $col0:  "one".to_string() },
                  $MyCsvRow { $col0:  "two".to_string() },
               ]),
            };
         }
      }
   );


   let items = vec![item0.expect("Should be able to make struct MyCsvRow"),
                    item1.expect("Should be able to make struct MyCsv."),
                    item2.expect("Should be able to implement MyCsv.")];

   println!("OK");

   return MacItems::new(items);
}

