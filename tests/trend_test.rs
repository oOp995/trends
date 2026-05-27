#[cfg(test)]
mod tests {
    use trends::{Trend, TrendExt};
    #[test]
    fn new_trend_test() {
        let trend = 10.to_trend(&20);
        assert!(trend.is_rising());
        assert_eq!(trend.start(), &10);
    }

    #[test]
    fn rising_test() {
        let start = "A";
        let end = "Z";
        let trend = start.to_trend(&end);

        assert!(trend.is_rising());
        assert_eq!(trend.direction(), 1);
        assert_eq!(trend.end(), &"Z");
    }

    #[test]
    fn falling_test() {
        let start = String::from("Z");
        let end = String::from("A");
        let trend = start.to_trend(&end);

        assert!(trend.is_falling());
        assert_eq!(trend.direction(), -1);
        assert_eq!(trend.end(), &"A");

        //negative asserts
        assert!(!trend.is_rising());
        assert!(!trend.is_stable());
    }

    #[test]
    fn stable_test() {
        let start = false;
        let end = false;
        let trend = start.to_trend(&end);
        assert!(trend.is_stable());
        assert_eq!(*trend.start(), false);
        assert_eq!(*trend.end(), false);
        //negative asserts
        assert!(!trend.is_rising());
        assert!(!trend.is_falling());
    }

    #[test]
    fn test_clone() {
        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
        struct NoCopy {
            st: String,
        }
        let start = NoCopy {
            st: String::from("A"),
        };
        let end = NoCopy {
            st: String::from("B"),
        };

        let trend = start.to_trend(&end);
        assert!(trend.is_rising());
        assert_eq!(trend.direction(), 1);
        //can not move out of &NoCopy because it does not
        //implement Copy
        //let tstart=*trend.start();
        //but it implememnt `Clone`
        //so if performance acceptable pay for it.
        let start_cloned = trend.clone_start();
        assert_eq!(start_cloned, start);

        let end_cloned = trend.clone_end();
        assert_eq!(end_cloned, end);
    }

    #[test]
    fn test_sign_of_change() {
        let start = false;
        let end = true;
        let trend = start.to_trend(&end);

        assert!(trend.is_rising());
        assert_eq!(trend.direction(), 1);

        let trend = end.to_trend(&start);
        assert!(trend.is_falling());
        assert_eq!(trend.direction(), -1);

        let trend = start.to_trend(&false);
        assert!(trend.is_stable());
        assert_eq!(trend.direction(), 0);
    }

    #[test]
    fn exact_variant() {
        let trend = 10.to_trend(&20);
        assert_eq!(
            trend,
            Trend::Rising {
                start: &10,
                end: &20
            }
        )
    }

    #[test]
    fn direction_matches_variant() {
        assert_eq!(1.to_trend(&2).direction(), 1);
        assert_eq!(2.to_trend(&1).direction(), -1);
        assert_eq!(1.to_trend(&1).direction(), 0);
    }

    #[test]
    fn min_test() {
        let trend = 1.to_trend(&5);
        let min = trend.min_point();
        assert_eq!(min, &1);

        let trend = 5.to_trend(&1);
        assert_eq!(trend.min_point(), &1);
    }

    #[test]
    fn max_test() {
        let start = String::from("A");
        let end = String::from("B");
        let trend = start.to_trend(&end);
        assert_eq!(trend.max_point(), &"B");

        let trend = end.to_trend(&start);
        assert_eq!(trend.max_point(), &"B")
    }

    #[test]
    fn min_cloned() {
        let start = String::from("A");
        let end = String::from("B");
        let trend = start.to_trend(&end);

        let min_cloned = trend.min_point_cloned();
        assert_eq!(min_cloned, "A");

        let min_cloned = end.to_trend(&start).min_point_cloned();
        assert_eq!(min_cloned, "A");
    }

    #[test]
    fn max_cloned() {
        let start = String::from("A");
        let end = String::from("B");
        let trend = start.to_trend(&end);

        let max_cloned = trend.max_point_cloned();
        assert_eq!(max_cloned, "B");

        let max_cloned = end.to_trend(&start).max_point_cloned();
        assert_eq!(max_cloned, "B");
    }
}
