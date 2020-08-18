mod star_positions {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    pub struct Star {
        name: String, //"1Alp UMi"
        ra_deg: f32,  // in seconds
        dec_deg: f32,
        propermotion_ra: f32,
        propermotion_dec: f32,
        magnitude: f32,
    }

    pub fn load_bsc_stars(filename: &str) -> HashMap<u32, Star> {
        let mut map = HashMap::new();
        let r = File::open(filename);
        let r = match r {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };

        let br = BufReader::new(r);
        for line in br.lines() {
            let line = match line {
                Ok(line) => line,
                Err(error) => panic!("problem reading line: {:?}", error),
            };
            let number: u32 = match line[0..4].trim().parse() {
                Ok(l) => l,
                Err(error) => panic!("unable to parse number: {:?}", error),
            };
            let name = &line[4..14].trim();

            println!("{:?}", &line[75..77].trim());
            let ra_hour: u32 = match line[75..77].trim().parse() {
                Ok(l) => l,
                Err(error) => panic!("unable to parse ra_hour: {:?}", error),
            };
            let ra_minute: u32 = match line[77..79].trim().parse() {
                Ok(l) => l,
                Err(error) => panic!("unable to parse ra_minute: {:?}", error),
            };
            let ra_second: f32 = match line[79..83].trim().parse() {
                Ok(l) => l,
                Err(error) => panic!("unable to parse ra_second: {:?}", error),
            };
            let ra_deg = ra_hour as f32 * 3600. + ra_minute as f32 * 60. + ra_second;

            let dec_degree: u32 = match line[83..86].trim().parse() {
                Ok(l) => l,
                Err(error) => {
                    println!(
                        "unable to parse number: {:?} {:?}",
                        error,
                        line[83..86].trim()
                    );
                    0
                }
            };
            let dec_minute: u32 = match line[86..88].trim().parse() {
                Ok(l) => l,
                Err(error) => panic!("unable to parse number: {:?}", error),
            };
            let dec_second: u32 = match line[88..90].trim().parse() {
                Ok(l) => l,
                Err(error) => panic!("unable to parse number: {:?}", error),
            };
            let dec_deg =
                dec_degree as f32 + ((dec_minute as f32 * 60. + dec_second as f32) / 3600.);
            let propermotion_ra: f32 = match line[148..154].trim().parse() {
                Ok(l) => l,
                Err(error) => panic!("unable to parse propermotion_ra: {:?}", error),
            };
            let propermotion_dec: f32 = match line[154..160].trim().parse() {
                Ok(l) => l,
                Err(error) => panic!("unable to parse propermotion_dec: {:?}", error),
            };

            let magnitude: f32 = match line[102..107].trim().parse() {
                Ok(l) => l,
                Err(error) => panic!("unable to parse magnitude: {:?}", error),
            };

            map.insert(
                number,
                Star {
                    name: name.to_string(), //"1Alp UMi", //023148.7+891551
                    ra_deg: ra_deg,         // in seconds
                    dec_deg: dec_deg,
                    propermotion_ra: propermotion_ra,
                    propermotion_dec: propermotion_dec,
                    magnitude: magnitude,
                },
            );
        }
        return map;
    }

    pub fn date_to_jd(year: u32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> f64 {
        let (y, m): (f64, f64) = match month {
            month if month > 2 => (year as f64, month as f64),
            _ => (year as f64 - 1., month as f64 + 12.),
        };
        let d = day as f64
            + (((hour as f64 * 3600.) + (minute as f64 * 60.) + second as f64) / (24. * 60. * 60.));
        let b = 2. - (y / 100.).floor() + (y / 400.).floor();
        return (365.25 * (y + 4716.)).floor() + (30.6001 * (m + 1.)).floor() + d + b - 1524.5;
    }

    pub fn date_to_sideral(
        year: u32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        lambda: f64,
    ) -> (i32, i32, i32) {
        let jd = date_to_jd(year, month, day, 0, 0, 0);
        let t = (jd - 2451545.0) / 36525.0;

        let mut gmst =
            24110.54841 + (8640184.812866 * t) + (0.093104 * t * t) - (0.0000062 * (t * t * t));
        gmst = gmst + (1.00273790935 * (hour as f64 * 3600. + minute as f64 * 60. + second as f64));
        let mut lmst = gmst + (lambda / 15. * 3600.);

        let hours = (lmst / 3600.).floor();
        lmst = lmst - (hours * 3600.);
        let minutes = (lmst / 60.).floor();
        lmst = lmst - (minutes * 60.);

        return (hours as i32 % 24, minutes as i32, lmst as i32);
    }

    #[test]
    fn test_date_to_jd() {
        assert_eq!(date_to_jd(2020, 9, 17, 8, 14, 35), 2459109.8434606483);
        assert_eq!(date_to_jd(2000, 1, 1, 12, 0, 0), 2451545.0);
        assert_eq!(date_to_jd(1899, 12, 31, 19, 31, 28), 2415020.3135185186);
        assert_eq!(date_to_jd(2020, 9, 17, 19, 52, 52), 2459110.3283796296);
        assert_eq!(date_to_jd(2015, 9, 28, 18, 51, 00), 2457294.285416667);
        assert_eq!(date_to_jd(2017, 2, 6, 12, 11, 00), 2457791.007638889);
        assert_eq!(date_to_jd(2020, 08, 17, 20, 32, 48), 2459079.356111111);
    }

    #[test]
    fn test_date_to_sideral() {
        println!("hallo");
        assert_eq!(date_to_sideral(2007, 12, 25, 0, 0, 0, 0.0), (6, 12, 31));
        assert_eq!(date_to_sideral(2007, 12, 25, 20, 0, 0, 0.0), (2, 15, 48));
        assert_eq!(date_to_sideral(2007, 12, 25, 20, 0, 0, 13.5), (3, 9, 48));
        assert_eq!(
            date_to_sideral(2020, 08, 18, 14, 06, 05, 8.88),
            (12, 31, 12)
        );
    }

    #[test]
    fn test_bsc_loader() {
        let s = Star {
            name: "1Alp UMi".to_string(), //023148.7+891551
            ra_deg: 9108.7,               // in seconds
            dec_deg: 89.26417,
            propermotion_ra: 0.038,
            propermotion_dec: -0.015,
            magnitude: 2.02,
        };

        let stars = load_bsc_stars("bsc5.dat");
        let s2 = &stars[&424];
        assert_eq!(s.name, s2.name);
        assert_eq!(s.ra_deg, s2.ra_deg);
        assert_eq!(s.dec_deg, s2.dec_deg);
        assert_eq!(s.propermotion_dec, s2.propermotion_dec);
        assert_eq!(s.propermotion_ra, s2.propermotion_ra);
        assert_eq!(s.magnitude, s2.magnitude);
    }
}
