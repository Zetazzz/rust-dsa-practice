
#[cfg(test)]
mod algorithms_bin_search_test {

    use algorithms::bin_search::bin_search;

    #[test]
    fn bin_search_found_first() {
        let a = [3, 6, 11, 24, 35, 44, 56, 67, 79, 88];

        let r = bin_search(&a, 3);

        assert_eq!(r, Some(0));
    }

    #[test]
    fn bin_search_found_last() {
        let a = [3, 6, 11, 24, 35, 44, 56, 67, 79, 88];

        let r = bin_search(&a, 88);

        assert_eq!(r, Some(a.len() - 1));
    }

    #[test]
    fn bin_search_found_odd_middle() {
        let a = [3, 6, 11, 24, 35, 39, 44, 56, 67, 79, 88];

        let r = bin_search(&a, 39);

        assert_eq!(r, Some(a.len() / 2));
    }

    #[test]
    fn bin_search_found_odd_smaller_than_middle() {
        let a = [3, 6, 11, 24, 35, 39, 44, 56, 67, 79, 88];

        let r = bin_search(&a, 6);

        assert_eq!(r, Some(1));
    }

    #[test]
    fn bin_search_found_odd_bigger_than_middle() {
        let a = [3, 6, 11, 24, 35, 39, 44, 56, 67, 79, 88];

        let r = bin_search(&a, 79);

        assert_eq!(r, Some(9));
    }

    #[test]
    fn bin_search_not_found_odd_smaller_than_smallest() {
        let a = [3, 6, 11, 24, 35, 39, 44, 56, 67, 79, 88];

        let r = bin_search(&a, 2);

        assert_eq!(r, None);
    }

    #[test]
    fn bin_search_not_found_odd_bigger_than_biggest() {
        let a = [3, 6, 11, 24, 35, 39, 44, 56, 67, 79, 88];

        let r = bin_search(&a, 100);

        assert_eq!(r, None);
    }


    #[test]
    fn bin_search_not_found_odd_smaller_than_middle() {
        let a = [3, 6, 11, 24, 35, 39, 44, 56, 67, 79, 88];

        let r = bin_search(&a, 5);

        assert_eq!(r, None);
    }

    #[test]
    fn bin_search_not_found_odd_bigger_than_middle() {
        let a = [3, 6, 11, 24, 35, 39, 44, 56, 67, 79, 88];

        let r = bin_search(&a, 87);

        assert_eq!(r, None);
    }
}
