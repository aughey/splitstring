use rstest::rstest;
use rstest_reuse::{self, *};
use rustimpl::*;

#[template]
#[rstest]
#[case(split_string_try_op)]
#[case(split_string_3)]
#[case(split_string_like_cpp)]
#[case(split_string_iterator_combinator)]
#[case(split_string_3_iterparse)]
#[case(split_string_try_op_no_conditionals)]
#[case(split_string_try_op_one_line)]
#[case(split_string_like_cpp_no_unwrap)]
#[case(split_string_like_cpp_i_need_help)]
fn all_splits(#[case] split : impl Fn(&str, &str) -> Option<(u16, u16, u16)>) {}

#[apply(all_splits)]
fn happy_path(#[case] split : impl Fn(&str, &str) -> Option<(u16, u16, u16)>) {
    let res = split("1,2,3", ",");
    assert_eq!(res, Some((1, 2, 3)));
}

#[apply(all_splits)]
fn missing_one(#[case] split : impl Fn(&str, &str) -> Option<(u16, u16, u16)>) {
    let res = split("1,2", ",");
    assert_eq!(res, None);
}

#[apply(all_splits)]
fn hole(#[case] split : impl Fn(&str, &str) -> Option<(u16, u16, u16)>) {
    let res = split("1,,3", ",");
    assert_eq!(res, None);
}

#[apply(all_splits)]
fn not_number(#[case] split : impl Fn(&str, &str) -> Option<(u16, u16, u16)>) {
    let res = split("A,2,3", ",");
    assert_eq!(res, None);
}

#[apply(all_splits)]
fn out_of_range_3(#[case] split : impl Fn(&str, &str) -> Option<(u16, u16, u16)>) {
    let res = split("1,2,65536", ",");
    assert_eq!(res, None);
}

#[apply(all_splits)]
fn out_of_range_2(#[case] split : impl Fn(&str, &str) -> Option<(u16, u16, u16)>) {
    let res = split("1,65536,3", ",");
    assert_eq!(res, None);
}

#[apply(all_splits)]
fn out_of_range_1(#[case] split : impl Fn(&str, &str) -> Option<(u16, u16, u16)>) {
    let res = split("65536,2,3", ",");
    assert_eq!(res, None);
}

#[apply(all_splits)]
fn low_range_pass(#[case] split : impl Fn(&str, &str) -> Option<(u16, u16, u16)>) {
    let res = split("0,0,0", ",");
    assert_eq!(res, Some((0, 0, 0)));
}

#[apply(all_splits)]
fn low_range_fail_1(#[case] split : impl Fn(&str, &str) -> Option<(u16, u16, u16)>) {
    let res = split("-1,0,0", ",");
    assert_eq!(res, None);
}

#[apply(all_splits)]
fn low_range_fail_2(#[case] split : impl Fn(&str, &str) -> Option<(u16, u16, u16)>) {
    let res = split("0,-1,0", ",");
    assert_eq!(res, None);
}

#[apply(all_splits)]
fn low_range_fail_3(#[case] split : impl Fn(&str, &str) -> Option<(u16, u16, u16)>) {
    let res = split("0,0,-1", ",");
    assert_eq!(res, None);
}

#[apply(all_splits)]
fn high_range_pass(#[case] split : impl Fn(&str, &str) -> Option<(u16, u16, u16)>) {
    let res = split("65535,65535,65535", ",");
    assert_eq!(res, Some((65535, 65535, 65535)));
}

#[apply(all_splits)]
fn too_many_numbers(#[case] split : impl Fn(&str, &str) -> Option<(u16, u16, u16)>) {
    let res = split("1,2,3,4", ",");
    assert_eq!(res, None);
}


#[apply(all_splits)]
fn multi_char_delimiter(#[case] split : impl Fn(&str, &str) -> Option<(u16, u16, u16)>) {
    let res = split("1##2##3", "##");
    assert_eq!(res, Some((1, 2, 3)));   
}