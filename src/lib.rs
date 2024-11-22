//  在调试模式下，禁用特定的编译器警告，允许未使用的代码、未使用的导入、未使用的变量以及未使用的mut关键字
//  cargo build --release会继续警告
#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
mod _01_two_sum;
mod _02_add_two_numbers;
mod _03_length_of_longest_substring;
mod _04_find_median_sorted_arrays;
mod _05_longest_palindrome;
mod _09_is_palindrome;
mod _105_build_tree;
mod _106_build_tree;
mod _11_max_area;
mod _136_delete_node;
mod _13_roman_to_int;
mod _14_longest_common_prefix;
mod _17_letter_combinations;
mod _189_rotate;
mod _18_four_sum;
mod _19_remove_nth_from_end;
mod _202_is_happy;
mod _209_min_sub_array_len;
mod _20_is_valid;
mod _21_remove_nth_from_end;
mod _229_majority_element;
mod _22_generate_parenthesis;
mod _23_merge_k_lists;
mod _24_reverse_list;
mod _24_swap_pairs;
mod _26_remove_duplicates;
mod _27_remove_element;
mod _28_str;
mod _32_longest_valid_parentheses;
mod _35_search_insert;
mod _37_solve_sudoku;
mod _39_combination_sum;
mod _41_first_missing_positive;
mod _42_trap;
mod _46_permute;
mod _51_solve_n_queens;
mod _54_spiral_order;
mod _59_generate_matrix;
mod _707_design_linked_list;
mod _887_super_egg_drop;
mod _977_sorted_squares;
mod common;
