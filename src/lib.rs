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
mod _06_convert;
mod _09_is_palindrome;
mod _100_is_same_tree;
mod _101_is_symmetric;
mod _102_level_order;
mod _1047_remove_duplicates;
mod _104_max_depth;
mod _105_build_tree;
mod _106_build_tree;
mod _107_level_order_bottom;
mod _108_sorted_array_to_bst;
mod _109_sorted_list_to_bst;
mod _10_is_match;
mod _110_is_balanced;
mod _111_min_depth;
mod _112_has_path_sum;
mod _113_path_sum;
mod _114_flatten;
mod _11_max_area;
mod _124_max_path_sum;
mod _129_sum_numbers;
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
mod _199_right_side_view;
mod _19_remove_nth_from_end;
mod _202_is_happy;
mod _209_min_sub_array_len;
mod _20_is_valid;
mod _222_count_nodes;
mod _224_calculate;
mod _226_invert_tree;
mod _229_majority_element;
mod _22_generate_parenthesis;
mod _230_kth_smallest;
mod _235_lowest_common_ancestor;
mod _236_lowest_common_ancestor;
mod _239_max_sliding_window;
mod _23_merge_k_lists;
mod _2413_smallest_even_multiple;
mod _242_is_anagram;
mod _24_swap_pairs;
mod _257_binary_tree_paths;
mod _25_reverse_k_group;
mod _26_remove_duplicates;
mod _27_remove_element;
mod _28_str;
mod _29_divide;
mod _32_longest_valid_parentheses;
mod _337_rob;
mod _344_reverse_string;
mod _347_top_k_frequent;
mod _349_intersection;
mod _34_search_range;
mod _35_search_insert;
mod _37_solve_sudoku;
mod _39_combination_sum;
mod _404_sum_of_left_leaves;
mod _40_combination_sum;
mod _41_first_missing_positive;
mod _42_trap;
mod _437_path_sum;
mod _43_multiply;
mod _454_four_sum_count;
mod _459_repeated_substring_pattern;
mod _46_permute;
mod _47_permute_unique;
mod _501_find_mode;
mod _513_find_bottom_left_value;
mod _51_solve_n_queens;
mod _52_total_n_queens;
mod _530_get_minimum_difference;
mod _541_reverse_str;
mod _54_spiral_order;
mod _563_find_tilt;
mod _59_generate_matrix;
mod _617_merge_trees;
mod _654_construct_maximum_binary_tree;
mod _700_search_bst;
mod _707_design_linked_list;
mod _75_sort_colors;
mod _887_super_egg_drop;
mod _94_inorder_traversal;
mod _958_is_complete_tree;
mod _95_generate_trees;
mod _965_is_unival_tree;
mod _96_num_trees;
mod _977_sorted_squares;
mod _98_is_valid_bst;
mod _998_insert_into_max_tree;
mod _99_recover_tree;
pub mod common;
mod lcr_024_reverse_list;
mod lcr_144_flip_tree;
mod lcr_175_calculate_depth;
mod lcr_21_remove_nth_from_end;
mod _993_is_cousins;
mod _543_diameter_of_binary_tree;
mod _31_next_permutation;
mod _538_convert_bst;
mod _77_combine;
mod _216_combination_sum3;
mod _78_subsets;
