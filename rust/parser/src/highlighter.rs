// Copyright 2025 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "hit_highlighter.pest"]
pub struct HighlightParser;
