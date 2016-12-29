use super::chrono;

fn timegreeting(dt: chrono::DateTime) -> String {

}
pub fn greeting (local_time: String) -> String {
    let dt = local_time.parse::<chrono::DateTime<FixedOffset>>();
    format!("{timegreet}, {target_name}. Today is {date_string}. \
             Today will be {weather}, with a high of {high temp}.\
             You have {emails} new message{plural}, {importance_desc}.",
             timegreet = timedgreeting(dt),
             target_name = "Ross",
             date_string = dt.format("%A, %B %e").to_string(),
             "no",
             "s",
             "you poor bastard"
             )
}

#[cfg(test)]
mod tests{
    #[test]
    fn greeting_morning() {
        let dt = chrono::UTC.ymd(2016, 12, 25).and_hms(8, 0, 0); // 8 am
        assert_eq!(timegreeting(dt), "Good morning");
    }

    #[test]
    fn greeting_end_morning() {
        let dt = chrono::UTC.ymd(2016, 12, 25).and_hms(11, 59, 59); // 8 am
        assert_eq!(timegreeting(dt), "Good morning");
    }
}
