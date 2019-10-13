// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//#[macro_use]
//extern crate html5ever;

use std::default::Default;
use std::fs::{File};
use std::string::String;
use std::collections::HashMap;

use html5ever::parse_document;
use html5ever::rcdom::{Handle, NodeData, RcDom};
use html5ever::tendril::TendrilSink;

// This is not proper HTML serialization, of course.

fn walk(handle: &Handle, hm:&mut HashMap<String,f32>, print: bool, _name:&mut String, rating:bool){
    let node = handle;
    let mut b = print;
    let mut c = rating;
    match node.data {
        NodeData::Text { ref contents } => {
            if print && c{
                hm.insert(_name.to_string(), contents.borrow().to_string().parse::<f32>().unwrap());
            }
        },
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            assert!(name.ns == ns!(html));

            if name.local.to_string() == "tbody"{
                b = true;
            }else if name.local.to_string() == "a" && print{
                let n = node.children.borrow_mut().pop().unwrap();
                match n.data{
                    NodeData::Text{ ref contents } => {
                        *_name = format!("{}", contents.borrow());
                    },
                    _ => {
                        println!("Shouldnt happen!");
                    }
                }
            }else if name.local.to_string() == "td" && print{
                for attr in attrs.borrow().iter() {
                    assert!(attr.name.ns == ns!());
                    if attr.value.to_string() == "ratingCol"{
                        c = true;
                    }
                }
            }
        },
        _ =>{
            //do nothing
        }
    }

    for child in node.children.borrow().iter() {
        walk(child, hm, b, _name, c);
    }
}

pub fn parse(file:&mut File) -> HashMap<String,f32>{
    
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(file)
        .unwrap();

    let mut hm = HashMap::new();
    let mut s = "".to_string();

    walk(&dom.document, &mut hm, false, &mut s, false);
    
    return hm;

    //FIXME:ignore parse errors for now
    /*if !dom.errors.is_empty() {
        println!("\nParse errors:");
        for err in dom.errors.iter() {
            println!("    {}", err);
        }
    }*/
}
