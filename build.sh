#!/bin/sh

rustc CSVProvider.rs && \
rustc --test -L . CSVSample.rs
