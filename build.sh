#!/bin/sh

rustc CSVProvider.rs \
&& rustc --test -L . CSVSample.rs \
&& ./CSVSample \
#&& rustc --test -L . --pretty expanded CSVSample.rs
