extern crate data_structures;
#[macro_use]
extern crate galvanic_test;

test_suite! {
    name bubble_sort;
    use data_structures::sort::*;

    test bubble_sort_1() {
        let mut array_input = [10,3,5,2,1,6,4,9,8,7].to_vec();
        let array_sorted = [1,2,3,4,5,6,7,8,9,10].to_vec();

        bubble_sort(&mut array_input);

        assert_eq!(array_input, array_sorted);
    }

    test bubble_sort_2() {
        let mut array_input = [96,	21,	29,	52,	54,10,	55,	66,	59,	18,58,	46,	31,	12,	40,53].to_vec();
        let array_sorted = [10, 12, 18, 21, 29, 31, 40, 46, 52, 53, 54, 55, 58, 59, 66, 96].to_vec();

        bubble_sort(&mut array_input);

        assert_eq!(array_input, array_sorted);
    }

    test bubble_sort_3() {
        let mut array_input = [100, 25, 65, 95, 24, -87, 14, 49, 99, 36, 8, 84, 43, 32, 93, 19, 21, 86, -4, 74].to_vec();
        let array_sorted = [-87, -4, 8, 14, 19, 21, 24, 25, 32, 36, 43, 49, 65, 74, 84, 86, 93, 95, 99, 100].to_vec();

        bubble_sort(&mut array_input);

        assert_eq!(array_input, array_sorted);
    }
}

test_suite! {
    name selection_sort;
    use data_structures::sort::*;

    test selection_sort_1() {
        let mut array_input = [10,3,5,2,1,6,4,9,8,7].to_vec();
        let array_sorted = [1,2,3,4,5,6,7,8,9,10].to_vec();
    
        selection_sort(&mut array_input);

        assert_eq!(array_input, array_sorted);
    }

    test selection_sort_2() {
        let mut array_input = [96,	21,	29,	52,	54,10,	55,	66,	59,	18,58,	46,	31,	12,	40,53].to_vec();
        let array_sorted = [10, 12, 18, 21, 29, 31, 40, 46, 52, 53, 54, 55, 58, 59, 66, 96].to_vec();

        selection_sort(&mut array_input);

        assert_eq!(array_input, array_sorted);
    }

    test selection_sort_3() {
        let mut array_input = [100, 25, 65, 95, 24, -87, 14, 49, 99, 36, 8, 84, 43, 32, 93, 19, 21, 86, -4, 74].to_vec();
        let array_sorted = [-87, -4, 8, 14, 19, 21, 24, 25, 32, 36, 43, 49, 65, 74, 84, 86, 93, 95, 99, 100].to_vec();

        selection_sort(&mut array_input);

        assert_eq!(array_input, array_sorted);
    }
}