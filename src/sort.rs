pub fn swap(array: &mut Vec<i32>,from: usize,to: usize) {
    let tmp = array[from];
    array[from] = array[to];
    array[to] = tmp;
}

pub fn bubble_sort(array: &mut Vec<i32>) {
    let mut walker: usize = 0;
    let mut number_of_sorted_elements: usize = 0;

    loop {
        if number_of_sorted_elements >= (array.len() - 1) {
            break;
        }

        loop {
            if walker >= (array.len() - number_of_sorted_elements - 1) {
                break;
            }

            if array[walker] > array[walker + 1] {
                swap(array, walker, walker + 1);
            }

            walker += 1;
        }

        walker = 0;
        number_of_sorted_elements += 1;
    }
}

pub fn selection_sort(array: &mut Vec<i32>) {
    let mut number_of_sorted_elements: usize = 0;
    let mut smallest: usize = 0;
    let mut walker: usize = number_of_sorted_elements + 1;

    loop {
        if number_of_sorted_elements == (array.len() - 1) {
            break;
        }

        loop {
            if walker == array.len() {
                break;
            }

            if  array[walker] < array[smallest] {
                smallest = walker
            }

            walker += 1
        }

        if smallest != number_of_sorted_elements {
            swap(array, smallest, number_of_sorted_elements);
        }

        number_of_sorted_elements += 1;
        
        walker = number_of_sorted_elements;
        smallest = walker;
    }
}