#![crate_id="CSVSample#0.1-pre"]
#![crate_type="bin"]

#![allow(unused_imports)]
#![allow(unused_variable)]

#![feature(phase)]

#[phase(plugin)]
extern crate CSVProvider;

extern crate test;

use std::io;
use std::io::fs;
use std::os;
use std::str;
use std::io::Process;

#[cfg(not(test))]
#[main]
fn main () {
   ::std::io::println("eh, you wanna compile the tests, instead");
}

/*
#[test]
fn csv_basic() {
   // let the type provider grab first row as column name
   ProvideCSV!("MyCSV", "./sample1.txt");

   let samples = MyCSV::new();

   assert_eq!("A DECLARATION".to_string(), samples.data[2].text);
}
*/

#[test]
fn csv_labeled() {

   mod Demo {
      ProvideCSV_labels!("MyCSV", "./sample1.txt", "Verse")
   }

   // tell the type provider the column name

   assert_eq!(1u,1u);

   let samples = Demo::MyCSV::new();

   //assert_eq!("A DECLARATION".to_string(), samples.data.get(2).Verse);
   assert_eq!("two".to_string(), samples.data.get(2).clone());
}

/*
#[test]
fn csv_changed_at_runtime() {
   ProvideCSV_labels!("MyCSV", "./sample1.txt", "Number");

   // chose a different file at runtime
   let samples = MyCSV::new_with_path("./sample2.txt");
   
   assert_eq!("4351", samples.data[9].Number);
}

#[test]
fn csv_inferred() {
   ProvideCSV_labels_col_inferred!("MyCSV", "./sample2.txt", "Number");

   let samples = MyCSV::new()

   // infer the types of each column
   assert_eq!(4351, samples.data[8].Number);
}
*/

