// Copyright 2019 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

#[repr(C)]
#[derive(Parser)]
#[grammar = "twitter_text.pest"]
pub struct TwitterTextParser;
