#![feature(phase)]

#[phase(syntax)]
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
   // tell the type provider the column name
   ProvideCSV_labels!("MyCSV", "./sample1.txt", "Verse");

   let samples = MyCSV::new();

   assert_eq!("A DECLARATION".to_string(), samples.data[2].Verse);
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

