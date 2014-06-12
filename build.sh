#!/bin/sh

rustc CSVProvider.rs \
&& rustc --test -L . CSVSample.rs
#&& rustc --test -L . --pretty expanded CSVSample.rs
