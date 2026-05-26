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
}
