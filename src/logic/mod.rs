use super::chrono::{UTC, DateTime, FixedOffset, NaiveTime};

const owm_api_key: String = "88faf93052c74046d35a1b902dc19fd7";

fn get_weather(long: f32, lat: f32) -> (String, u16, u16) {
    let query =
    format!("api.openweathermap.org/data/2.5/weather/daily?lat={lat}&lon={long}&APPID={apikey}&units=metric&cnt=3",
            lat=lat,
            long=long,
            apikey=owm_api_key);

    let client = Client::new();
    let res = client.get(&query).send().ok();
    let mut data = String::new();
    res.read_to_string(&data).ok();
    // TODO: Need to error check here. Can't panic whenever we get bad data.
    let weather: Value = serde_json::from_str(&data).unwrap(); 
    let weather_desc = weather.pointer("/list/0/weather/description").unwrap();
    let weather_max = weather.pointer("/list/0/temp/max").unwrap();
    let weather_min = weather.pointer("/list/0/temp/min").unwrap();
    (weather_desc, weather_max, weather_min)

}

fn timegreeting(dt: DateTime<FixedOffset>) -> String {
    let comp = dt.naive_local().time();
    let dawn = NaiveTime::from_hms(05, 00, 00);
    let noon = NaiveTime::from_hms(12, 00, 00);
    let dusk = NaiveTime::from_hms(17, 00, 00);
    let sleep = NaiveTime::from_hms(22, 00, 00);

    if comp >= dawn && comp < noon {
        "Good morning".to_string()
    } else if comp >= noon && comp < dusk {
        "Good afternoon".to_string()
    } else if comp >= dusk && comp < sleep {
        "Good evening".to_string()
    } else {
        "Hello".to_string()
    }
}

pub fn greeting (local_time: DateTime<FixedOffset>, lat: f32, long: f32) -> String {
    let weather = get_weather(lat, long); 
    format!("{timegreet}, {target_name}. Today is {date_string}. \
             Today will be {weather}, with a high of {high_temp} degrees.\
             And overnight lows of {low_temp} degrees.\
             You have {emails} new message{plural}, {importance_desc}.",
             timegreet = timegreeting(local_time),
             target_name = "Ross",
             date_string = local_time.format("%A, %B %e").to_string(),
             weather = weather.0,
             high_temp = weather.1,
             low_temp = weather.2
             emails = "no",
             plural = "s",
             importance_desc = "you poor bastard"
             )
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::*;

    #[test]
    fn morning_greeting_1 () {
        let local_time = FixedOffset::east(5 * 3600).ymd(2016, 7, 8).and_hms_milli(08, 00, 00, 00); 
        // 8am UTC-5 (EST)
        let greeting = greeting(local_time);
        let mut greet_split = greeting.split_whitespace();
        assert_eq!(greet_split.next(), Some("Good"));
        assert_eq!(greet_split.next().unwrap().trim_right_matches(","), "morning");
    }

    #[test]
    fn morning_greeting_2 () {
        let local_time = FixedOffset::east(5 * 3600).ymd(2016, 7, 8).and_hms_milli(11, 59, 00, 00); 
        // 11:59am UTC-5 (EST)
        let greeting = greeting(local_time);
        let mut greet_split = greeting.split_whitespace();
        assert_eq!(greet_split.next(), Some("Good"));
        assert_eq!(greet_split.next().unwrap().trim_right_matches(","), "morning");
    }

    #[test]
    fn morning_greeting_3 () {
        let local_time = FixedOffset::east(-1 * 3600).ymd(2016, 7, 8).and_hms_milli(11, 59, 00, 00); 
        // 11:59am UTC+1 (CET)
        let greeting = greeting(local_time);
        let mut greet_split = greeting.split_whitespace();
        assert_eq!(greet_split.next(), Some("Good"));
        assert_eq!(greet_split.next().unwrap().trim_right_matches(","), "morning");
    }

    #[test]
    fn afternoon_greeting_1 () {
        let local_time = FixedOffset::east(8 * 3600).ymd(2016, 7, 8).and_hms_milli(12, 00, 00, 00); 
        // 12pm UTC-8 (PST)
        let greeting = greeting(local_time);
        let mut greet_split = greeting.split_whitespace();
        assert_eq!(greet_split.next(), Some("Good"));
        assert_eq!(greet_split.next().unwrap().trim_right_matches(","), "afternoon");
    }
    
    #[test]
    fn afternoon_greeting_2 () {
        let local_time = FixedOffset::east(5 * 3600).ymd(2016, 7, 8).and_hms_milli(16, 59, 00, 00); 
        // 4:59pm UTC-8 (EST)
        let greeting = greeting(local_time);
        let mut greet_split = greeting.split_whitespace();
        assert_eq!(greet_split.next(), Some("Good"));
        assert_eq!(greet_split.next().unwrap().trim_right_matches(","), "afternoon");
    }

    #[test]
    fn afternoon_greeting_3 () {
        let local_time = FixedOffset::east(-8 * 3600).ymd(2016, 7, 8).and_hms_milli(16, 59, 00, 00); 
        // 4:59pm UTC+10 (HKT Hong Kong)
        let greeting = greeting(local_time);
        let mut greet_split = greeting.split_whitespace();
        assert_eq!(greet_split.next(), Some("Good"));
        assert_eq!(greet_split.next().unwrap().trim_right_matches(","), "afternoon");
    }

    #[test]
    fn evening_greeting_1 () {
        let local_time = FixedOffset::east(5 * 3600).ymd(2016, 7, 8).and_hms_milli(17, 00, 00, 00); 
        // 5pm UTC-5 (EST)
        let greeting = greeting(local_time);
        let mut greet_split = greeting.split_whitespace();
        assert_eq!(greet_split.next(), Some("Good"));
        assert_eq!(greet_split.next().unwrap().trim_right_matches(","), "evening");
    }

    #[test]
    fn evening_greeting_2 () {
        let local_time = FixedOffset::east(5 * 3600).ymd(2016, 7, 8).and_hms_milli(21, 59, 00, 00); 
        // 9:59pm UTC-5 (EST)
        let greeting = greeting(local_time);
        let mut greet_split = greeting.split_whitespace();
        assert_eq!(greet_split.next(), Some("Good"));
        assert_eq!(greet_split.next().unwrap().trim_right_matches(","), "evening");
    }

    #[test]
    fn other_greeting_1 () {
        let local_time = FixedOffset::east(5 * 3600).ymd(2016, 7, 8).and_hms_milli(03,39, 00, 00); 
        // 3:39am UTC-5 (EST)
        let greeting = greeting(local_time);
        let mut greet_split = greeting.split_whitespace();
        assert_eq!(greet_split.next().unwrap().trim_right_matches(","), "Hello");
    }


}
