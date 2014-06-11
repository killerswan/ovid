#![crate_id="CSVProvider#0.1-pre"]
#![crate_type="dylib"]

// why doesn't "lib" work?
/*
CSVSample.rs:38:4: 38:21 error: macro undefined: 'ProvideCSV_labels'
CSVSample.rs:38    ProvideCSV_labels!("MyCSV", "./sample1.txt", "Verse");
                   ^~~~~~~~~~~~~~~~~
error: aborting due to previous error
 */

// rustc 0.11.0-pre (732e057 2014-06-06 01:21:54 -0700)
// host: x86_64-apple-darwin


#![feature(globs, macro_registrar, macro_rules, quote, managed_boxes)]

extern crate syntax;

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
                        ExtCtxt,
                        MacResult,
                        DummyResult,
                        MacExpr,
                        NormalTT,
                        BasicMacroExpander};
use syntax::parse;
use syntax::parse::token;
use syntax::parse::token::{InternedString, COMMA, EOF};

use std::io;
use std::io::fs;
use std::os;
use std::str;
use std::io::Process;

#[macro_registrar]
pub fn macro_registrar(register: |Name, SyntaxExtension|) {
    register(token::intern("ProvideCSV_labels"),
             NormalTT(box BasicMacroExpander {
                 expander: provide_labels,
                 span: None,
             },
             None));
}

#[deriving(Clone)]
struct Entry {
    str: InternedString,
    expr: @Expr
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


// FIXME: right now we only look at one column (CSV, haha)

fn provide_labels(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> Box<MacResult> {
   let mut entries = match parse_entries(cx, tts) {
      Some(entries) => entries,
      None => return DummyResult::expr(sp),
   };

/*
   println!("provide_labels: args: {}", entries);

   let name   = entries[0];
   let path   = entries[1];
   let labels = entries[2];

   println!("provide_labels: name:   {}", name);
   println!("provide_labels: path:   {}", path);
   println!("provide_labels: labels: {}", labels);
*/

   return DummyResult::expr(sp);
}

