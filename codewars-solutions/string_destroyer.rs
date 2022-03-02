// https://www.codewars.com/kata/5872637c2eefcb1216000081/rust

use std::collections::HashSet;

fn destroy(input_sets: Vec<HashSet<char>>) -> String {
    let mut string_king = "a b c d e f g h i j k l m n o p q r s t u v w x y z".to_owned();
    for hash in input_sets {
         for boi in hash {
             let good_boi = &boi.to_string();
             if good_boi != " " { 
                string_king = string_king.replace(good_boi, "_");
             };
         }
    };
    string_king
}