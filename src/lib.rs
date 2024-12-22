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
mod _102_level_order;
mod _1047_remove_duplicates;
mod _104_max_depth;
mod _105_build_tree;
mod _106_build_tree;
mod _11_max_area;
mod _136_delete_node;
mod _13_roman_to_int;
mod _144_preorder_traversal;
mod _145_postorder_traversal;
mod _14_longest_common_prefix;
mod _150_eval_rpn;
mod _15_three_sum;
mod _1721_swap_nodes;
mod _17_letter_combinations;
mod _189_rotate;
mod _18_four_sum;
mod _19_remove_nth_from_end;
mod _202_is_happy;
mod _209_min_sub_array_len;
mod _20_is_valid;
mod _224_calculate;
mod _229_majority_element;
mod _22_generate_parenthesis;
mod _239_max_sliding_window;
mod _23_merge_k_lists;
mod _2413_smallest_even_multiple;
mod _242_is_anagram;
mod _24_swap_pairs;
mod _25_reverse_k_group;
mod _26_remove_duplicates;
mod _27_remove_element;
mod _28_str;
mod _32_longest_valid_parentheses;
mod _344_reverse_string;
mod _347_top_k_frequent;
mod _349_intersection;
mod _35_search_insert;
mod _37_solve_sudoku;
mod _39_combination_sum;
mod _41_first_missing_positive;
mod _42_trap;
mod _454_four_sum_count;
mod _459_repeated_substring_pattern;
mod _46_permute;
mod _51_solve_n_queens;
mod _52_total_n_queens;
mod _541_reverse_str;
mod _54_spiral_order;
mod _59_generate_matrix;
mod _707_design_linked_list;
mod _75_sort_colors;
mod _887_super_egg_drop;
mod _94_inorder_traversal;
mod _977_sorted_squares;
mod _98_is_valid_bst;
pub mod common;
mod lcr_024_reverse_list;
mod lcr_175_calculate_depth;
mod lcr_21_remove_nth_from_end;
mod _107_level_order_bottom;
mod lcr_144_flip_tree;
mod _100_is_same_tree;
