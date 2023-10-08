use super::{
    data_block::DataBlock,
    errors::TimeZoneError,
    header::{Header, Version},
    transition_rule::TransitionRule,
};
use crate::{local::cursor::Cursor, util::constants::BUG_MSG};

/// TimeZone containing parsed TZif data
#[derive(Debug, PartialEq)]
pub(crate) struct TimeZone {
    /// Transition times of the time zone
    transitions: Vec<Transition>,
    /// Local time types of the time zone
    local_time_types: Vec<LocalTimeType>,
    /// Extra transition time
    extra_rule: Option<TransitionRule>,
}

impl TimeZone {
    /// Parses the TZif file at `/etc/localtime` to a TimeZone
    pub(crate) fn from_tzif(bytes: &[u8]) -> Result<Self, TimeZoneError> {
        let mut cursor = Cursor::new(bytes);
        let header = Header::parse(&mut cursor)?;

        let (header, data_block, footer) = match header.ver {
            Version::V1 => {
                let data_block = DataBlock::parse(&mut cursor, &header, Version::V1)?;
                (header, data_block, None)
            }
            Version::V2 | Version::V3 => {
                // Remove Version 1 data block
                DataBlock::parse(&mut cursor, &header, Version::V1)?;

                let header = Header::parse(&mut cursor)?;
                let data_block = DataBlock::parse(&mut cursor, &header, header.ver)?;
                let footer = cursor.remaining();
                (header, data_block, Some(footer))
            }
        };

        let mut transitions: Vec<Transition> = Vec::with_capacity(header.transition_count);

        for (transition_time, &transition_type) in data_block
            .transition_times
            .chunks_exact(data_block.time_size)
            .zip(data_block.transition_types)
        {
            let transition_time = match header.ver {
                Version::V1 => {
                    i32::from_be_bytes(transition_time.try_into().expect(BUG_MSG)).into()
                }
                Version::V2 | Version::V3 => {
                    i64::from_be_bytes(transition_time.try_into().expect(BUG_MSG))
                }
            };
            let time_type_index = transition_type as usize;
            transitions.push(Transition::new(transition_time, time_type_index));
        }

        let mut local_time_types = Vec::with_capacity(header.type_count);

        for local_time_type in data_block.local_time_types.chunks_exact(6) {
            let utoff = i32::from_be_bytes(local_time_type[0..4].try_into().expect(BUG_MSG));
            let dst = local_time_type[4] != 0;
            local_time_types.push(LocalTimeType::new(utoff, dst));
        }

        let extra_rule = if let Some(footer) = footer {
            TransitionRule::from_tz_string(footer, header.ver == Version::V3)?
        } else {
            None
        };

        Ok(Self {
            transitions,
            local_time_types,
            extra_rule,
        })
    }

    pub(crate) fn to_local_time_type(&self, timestamp: i64) -> LocalTimeType {
        match self.transitions[..] {
            [] => match &self.extra_rule {
                Some(rule) => match rule {
                    TransitionRule::Fixed(local_time_type) => local_time_type.clone(),
                    TransitionRule::Alternate(altt) => {
                        let std_end_timestamp = altt.local_std_end_timestamp(timestamp);
                        let dst_end_timestamp = altt.local_dst_end_timestamp(timestamp);

                        let std_end_unix = std_end_timestamp - altt.std.utoff as i64;
                        let dst_end_unix = dst_end_timestamp - altt.dst.utoff as i64;

                        match timestamp {
                            // std end is before dst end
                            // timestamp is after time changed to dst
                            timestamp
                                if std_end_unix < dst_end_unix
                                    && std_end_unix <= timestamp
                                    && timestamp < dst_end_unix =>
                            {
                                altt.dst.clone()
                            }
                            // std is before dst
                            // timestamp is in std range
                            _ if std_end_unix < dst_end_unix => altt.std.clone(),
                            // dst end is before std end
                            // timestamp is after time changed to std
                            timestamp
                                if dst_end_unix < std_end_unix
                                    && dst_end_unix <= timestamp
                                    && timestamp < std_end_unix =>
                            {
                                altt.std.clone()
                            }
                            _ => altt.dst.clone(),
                        }
                    }
                },
                None => self.local_time_types[0].clone(),
            },
            _ => {
                let mut local_time_type_index = 0;
                for transition in self.transitions.iter().rev() {
                    if transition.unix_leap_time < timestamp {
                        local_time_type_index = transition.local_time_type_index;
                        break;
                    }
                }
                self.local_time_types[local_time_type_index].clone()
            }
        }
    }
}

/// Transition of a TZif file
#[derive(Debug, Eq, PartialEq)]
struct Transition {
    /// Unix leap time
    unix_leap_time: i64,
    /// Index specifying the local time type of the transition
    local_time_type_index: usize,
}

impl Transition {
    fn new(unix_leap_time: i64, local_time_type_index: usize) -> Self {
        Self {
            unix_leap_time,
            local_time_type_index,
        }
    }
}

/// Local time type of a TZif file
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct LocalTimeType {
    /// UTC offset
    pub(crate) utoff: i32,
    /// If the local time type is considered DST
    _dst: bool,
}

impl LocalTimeType {
    pub(super) fn new(utoff: i32, dst: bool) -> Self {
        Self { utoff, _dst: dst }
    }
}

#[cfg(test)]
mod local_tests {
    use crate::local::{
        header::Version,
        timezone::{LocalTimeType, TimeZone, Transition},
        transition_rule::{AlternateLocalTimeType, RuleDay, TransitionRule},
    };

    #[test]
    fn test_v1_file() {
        let bytes = b"TZif\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x01\0\0\0\x1b\0\0\0\0\0\0\0\x01\0\0\0\x04\0\0\0\0\0\0UTC\0\x04\xb2\x58\0\0\0\0\x01\x05\xa4\xec\x01\0\0\0\x02\x07\x86\x1f\x82\0\0\0\x03\x09\x67\x53\x03\0\0\0\x04\x0b\x48\x86\x84\0\0\0\x05\x0d\x2b\x0b\x85\0\0\0\x06\x0f\x0c\x3f\x06\0\0\0\x07\x10\xed\x72\x87\0\0\0\x08\x12\xce\xa6\x08\0\0\0\x09\x15\x9f\xca\x89\0\0\0\x0a\x17\x80\xfe\x0a\0\0\0\x0b\x19\x62\x31\x8b\0\0\0\x0c\x1d\x25\xea\x0c\0\0\0\x0d\x21\xda\xe5\x0d\0\0\0\x0e\x25\x9e\x9d\x8e\0\0\0\x0f\x27\x7f\xd1\x0f\0\0\0\x10\x2a\x50\xf5\x90\0\0\0\x11\x2c\x32\x29\x11\0\0\0\x12\x2e\x13\x5c\x92\0\0\0\x13\x30\xe7\x24\x13\0\0\0\x14\x33\xb8\x48\x94\0\0\0\x15\x36\x8c\x10\x15\0\0\0\x16\x43\xb7\x1b\x96\0\0\0\x17\x49\x5c\x07\x97\0\0\0\x18\x4f\xef\x93\x18\0\0\0\x19\x55\x93\x2d\x99\0\0\0\x1a\x58\x68\x46\x9a\0\0\0\x1b\0\0";

        let time_zone = TimeZone::from_tzif(bytes).unwrap();

        let time_zone_result = TimeZone {
            transitions: vec![],
            local_time_types: vec![LocalTimeType::new(0, false)],
            extra_rule: None,
        };

        assert_eq!(time_zone, time_zone_result);
    }

    #[test]
    fn test_v2_file() {
        let bytes = b"TZif2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x06\0\0\0\x06\0\0\0\0\0\0\0\x07\0\0\0\x06\0\0\0\x14\x80\0\0\0\xbb\x05\x43\x48\xbb\x21\x71\x58\xcb\x89\x3d\xc8\xd2\x23\xf4\x70\xd2\x61\x49\x38\xd5\x8d\x73\x48\x01\x02\x01\x03\x04\x01\x05\xff\xff\x6c\x02\0\0\xff\xff\x6c\x58\0\x04\xff\xff\x7a\x68\x01\x08\xff\xff\x7a\x68\x01\x0c\xff\xff\x7a\x68\x01\x10\xff\xff\x73\x60\0\x04LMT\0HST\0HDT\0HWT\0HPT\0\0\0\0\0\x01\0\0\0\0\0\x01\0TZif2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x06\0\0\0\x06\0\0\0\0\0\0\0\x07\0\0\0\x06\0\0\0\x14\xff\xff\xff\xff\x74\xe0\x70\xbe\xff\xff\xff\xff\xbb\x05\x43\x48\xff\xff\xff\xff\xbb\x21\x71\x58\xff\xff\xff\xff\xcb\x89\x3d\xc8\xff\xff\xff\xff\xd2\x23\xf4\x70\xff\xff\xff\xff\xd2\x61\x49\x38\xff\xff\xff\xff\xd5\x8d\x73\x48\x01\x02\x01\x03\x04\x01\x05\xff\xff\x6c\x02\0\0\xff\xff\x6c\x58\0\x04\xff\xff\x7a\x68\x01\x08\xff\xff\x7a\x68\x01\x0c\xff\xff\x7a\x68\x01\x10\xff\xff\x73\x60\0\x04LMT\0HST\0HDT\0HWT\0HPT\0\0\0\0\0\x01\0\0\0\0\0\x01\0\x0aHST10\x0a";

        let time_zone = TimeZone::from_tzif(bytes).unwrap();

        let time_zone_result = TimeZone {
            transitions: vec![
                Transition::new(-2334101314, 1),
                Transition::new(-1157283000, 2),
                Transition::new(-1155436200, 1),
                Transition::new(-880198200, 3),
                Transition::new(-769395600, 4),
                Transition::new(-765376200, 1),
                Transition::new(-712150200, 5),
            ],
            local_time_types: vec![
                LocalTimeType::new(-37886, false),
                LocalTimeType::new(-37800, false),
                LocalTimeType::new(-34200, true),
                LocalTimeType::new(-34200, true),
                LocalTimeType::new(-34200, true),
                LocalTimeType::new(-36000, false),
            ],
            extra_rule: Some(TransitionRule::Fixed(LocalTimeType::new(-36000, false))),
        };

        assert_eq!(time_zone, time_zone_result);

        assert_eq!(
            time_zone.to_local_time_type(-1156939200),
            LocalTimeType::new(-34200, true)
        );
        assert_eq!(
            time_zone.to_local_time_type(1546300800),
            LocalTimeType::new(-36000, false)
        );
    }

    #[test]
    fn test_v3_file() {
        let bytes = b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x04\0\0\x1c\x20\0\0IST\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x01\0\0\0\0\0\0\0\x01\0\0\0\x01\0\0\0\x04\0\0\0\0\x7f\xe8\x17\x80\0\0\0\x1c\x20\0\0IST\0\x01\x01\x0aIST-2IDT,M3.4.4/26,M10.5.0\x0a";

        let time_zone = TimeZone::from_tzif(bytes).unwrap();

        let time_zone_result = TimeZone {
            transitions: vec![Transition::new(2145916800, 0)],
            local_time_types: vec![LocalTimeType::new(7200, false)],
            extra_rule: Some(TransitionRule::Alternate(AlternateLocalTimeType::new(
                LocalTimeType::new(7200, false),
                RuleDay::MonthWeekDay(3, 4, 4),
                93600,
                LocalTimeType::new(10800, true),
                RuleDay::MonthWeekDay(10, 5, 0),
                7200,
            ))),
        };

        assert_eq!(time_zone, time_zone_result);
    }

    #[test]
    fn footer_julian_day() {
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1CEST,J100,J200\x0a";

        let time_zone = TimeZone::from_tzif(bytes).unwrap();
        assert_eq!(
            Some(TransitionRule::Alternate(AlternateLocalTimeType::new(
                LocalTimeType::new(3600, false),
                RuleDay::JulianDayWithoutLeap(100),
                7200,
                LocalTimeType::new(7200, true),
                RuleDay::JulianDayWithoutLeap(200),
                7200,
            ))),
            time_zone.extra_rule
        );
        assert_eq!(
            LocalTimeType::new(3600, false),
            time_zone.to_local_time_type(1672531200)
        );
        assert_eq!(
            LocalTimeType::new(3600, false),
            time_zone.to_local_time_type(1681088400 - 1)
        );
        assert_eq!(
            LocalTimeType::new(7200, true),
            time_zone.to_local_time_type(1681088400)
        );
        assert_eq!(
            LocalTimeType::new(7200, true),
            time_zone.to_local_time_type(1689724800 - 1)
        );
        assert_eq!(
            LocalTimeType::new(3600, false),
            time_zone.to_local_time_type(1689724800)
        );
        assert_eq!(
            LocalTimeType::new(3600, false),
            time_zone.to_local_time_type(1704067199)
        );

        assert_eq!(
            LocalTimeType::new(3600, false),
            time_zone.to_local_time_type(1704067200)
        );
        assert_eq!(
            LocalTimeType::new(3600, false),
            time_zone.to_local_time_type(1712710800 - 1)
        );
        assert_eq!(
            LocalTimeType::new(7200, true),
            time_zone.to_local_time_type(1712710800)
        );
        assert_eq!(
            LocalTimeType::new(7200, true),
            time_zone.to_local_time_type(1721347200 - 1)
        );
        assert_eq!(
            LocalTimeType::new(3600, false),
            time_zone.to_local_time_type(1721347200)
        );
        assert_eq!(
            LocalTimeType::new(3600, false),
            time_zone.to_local_time_type(1735689599)
        );

        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1CEST,99,199\x0a";
        let time_zone = TimeZone::from_tzif(bytes).unwrap();
        assert_eq!(
            Some(TransitionRule::Alternate(AlternateLocalTimeType::new(
                LocalTimeType::new(3600, false),
                RuleDay::JulianDayWithLeap(99),
                7200,
                LocalTimeType::new(7200, true),
                RuleDay::JulianDayWithLeap(199),
                7200,
            ))),
            time_zone.extra_rule
        );
        assert_eq!(
            LocalTimeType::new(3600, false),
            time_zone.to_local_time_type(1672531200)
        );
        assert_eq!(
            LocalTimeType::new(3600, false),
            time_zone.to_local_time_type(1681088400 - 1)
        );
        assert_eq!(
            LocalTimeType::new(7200, true),
            time_zone.to_local_time_type(1681088400)
        );
        assert_eq!(
            LocalTimeType::new(7200, true),
            time_zone.to_local_time_type(1689724800 - 1)
        );
        assert_eq!(
            LocalTimeType::new(3600, false),
            time_zone.to_local_time_type(1689724800)
        );
        assert_eq!(
            LocalTimeType::new(3600, false),
            time_zone.to_local_time_type(1704067199)
        );

        assert_eq!(
            LocalTimeType::new(3600, false),
            time_zone.to_local_time_type(1704067200)
        );
        assert_eq!(
            LocalTimeType::new(3600, false),
            time_zone.to_local_time_type(1712624400 - 1)
        );
        assert_eq!(
            LocalTimeType::new(7200, true),
            time_zone.to_local_time_type(1712624400)
        );
        assert_eq!(
            LocalTimeType::new(7200, true),
            time_zone.to_local_time_type(1721260800 - 1)
        );
        assert_eq!(
            LocalTimeType::new(3600, false),
            time_zone.to_local_time_type(1721260800)
        );
        assert_eq!(
            LocalTimeType::new(3600, false),
            time_zone.to_local_time_type(1735689599)
        );
    }

    #[test]
    fn footer() {
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0CET-1CEST,J100,J200";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0a:character\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0a\0\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0a<CET>-1CEST+2,M3.3.0,M10.3.0\x0a";
        let time_zone = TimeZone::from_tzif(bytes).unwrap();
        assert_eq!(
            LocalTimeType::new(3600, false),
            time_zone.to_local_time_type(1672531200)
        );
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1:0:0CEST,M3.3.0,M10.3.0\x0a";
        let time_zone = TimeZone::from_tzif(bytes).unwrap();
        assert_eq!(
            LocalTimeType::new(3600, false),
            time_zone.to_local_time_type(1672531200)
        );
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET25:0:0CEST,M3.3.0,M10.3.0\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET1:60:0CEST,M3.3.0,M10.3.0\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET1:0:60CEST,M3.3.0,M10.3.0\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1CEST,M3.3.0/1:60,M10.3.0\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1CEST,M3.3.0/1:59:60,M10.3.0\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1CEST,M3.3.0/167,M10.3.0\x0a";
        assert!(TimeZone::from_tzif(bytes).is_ok());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1CEST,M3.3.0/168,M10.3.0\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1CEST,M3.3.0/-167,M10.3.0\x0a";
        assert!(TimeZone::from_tzif(bytes).is_ok());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1CEST,M3.3.0/-168,M10.3.0\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1CEST,Goob\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1CEST,M3.3.0/-167,M10.3.0\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0a\xFF\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1<\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1CEST\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1CEST-60\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1CEST,10\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1CEST,10,10/18000\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-a\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1:a\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1:0:a\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1CEST,10/A\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1CEST,\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1CEST,J\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1CEST,M\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1CEST,M3\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1CEST,M3.\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1CEST,M3.5\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1CEST,M3.5.\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1\x0a";
        assert!(TimeZone::from_tzif(bytes).is_err());
    }

    #[test]
    fn footer_month_week_day() {
        let bytes = b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1CEST,M3.5.0,M10.5.0\x0a";
        let time_zone = TimeZone::from_tzif(bytes).unwrap();
        assert_eq!(
            Some(TransitionRule::Alternate(AlternateLocalTimeType::new(
                LocalTimeType::new(3600, false),
                RuleDay::MonthWeekDay(3, 5, 0),
                7200,
                LocalTimeType::new(7200, true),
                RuleDay::MonthWeekDay(10, 5, 0),
                7200,
            ))),
            time_zone.extra_rule
        );
        assert_eq!(
            LocalTimeType::new(3600, false),
            time_zone.to_local_time_type(1672531200)
        );
        assert_eq!(
            LocalTimeType::new(3600, false),
            time_zone.to_local_time_type(1679792400 - 1)
        );
        assert_eq!(
            LocalTimeType::new(7200, true),
            time_zone.to_local_time_type(1679792400)
        );
        assert_eq!(
            LocalTimeType::new(7200, true),
            time_zone.to_local_time_type(1698537600 - 1)
        );
        assert_eq!(
            LocalTimeType::new(3600, false),
            time_zone.to_local_time_type(1698537600)
        );
        assert_eq!(
            LocalTimeType::new(3600, false),
            time_zone.to_local_time_type(1704067199)
        );

        let bytes = b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1CEST,M10.5.0,M3.5.0\x0a";
        let time_zone = TimeZone::from_tzif(bytes).unwrap();
        assert_eq!(
            Some(TransitionRule::Alternate(AlternateLocalTimeType::new(
                LocalTimeType::new(3600, false),
                RuleDay::MonthWeekDay(10, 5, 0),
                7200,
                LocalTimeType::new(7200, true),
                RuleDay::MonthWeekDay(3, 5, 0),
                7200,
            ))),
            time_zone.extra_rule
        );
        assert_eq!(
            LocalTimeType::new(7200, true),
            time_zone.to_local_time_type(1672531200)
        );
        assert_eq!(
            LocalTimeType::new(7200, true),
            time_zone.to_local_time_type(1679788800 - 1)
        );
        assert_eq!(
            LocalTimeType::new(3600, false),
            time_zone.to_local_time_type(1679788800)
        );
        assert_eq!(
            LocalTimeType::new(3600, false),
            time_zone.to_local_time_type(1698541200 - 1)
        );
        assert_eq!(
            LocalTimeType::new(7200, true),
            time_zone.to_local_time_type(1698541200)
        );
        assert_eq!(
            LocalTimeType::new(7200, true),
            time_zone.to_local_time_type(1704067199)
        );
        let bytes = b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0aCET-1\x0a";
        let time_zone = TimeZone::from_tzif(bytes).unwrap();
        assert_eq!(
            Some(TransitionRule::Fixed(LocalTimeType::new(3600, false))),
            time_zone.extra_rule
        );
        assert_eq!(
            LocalTimeType::new(3600, false),
            time_zone.to_local_time_type(1704067199)
        );
    }

    #[test]
    fn invalid_tzif_file() {
        let result = TimeZone::from_tzif(b"");
        assert!(result.is_err());
        println!("{}", result.unwrap_err());
        let result = TimeZone::from_tzif(b"InvalidMagic");
        assert!(result.is_err());
        println!("{}", result.unwrap_err());
        let result = TimeZone::from_tzif(b"TZif4");
        assert!(result.is_err());
        println!("{}", result.unwrap_err());
        assert!(TimeZone::from_tzif(b"TZif").is_err());
        assert!(TimeZone::from_tzif(b"TZif3").is_err());

        let header = b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
        assert!(TimeZone::from_tzif(header).is_err());
        let header = b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF";
        assert!(TimeZone::from_tzif(header).is_err());
        let header = b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF";
        assert!(TimeZone::from_tzif(header.as_slice()).is_err());
        let header =
            b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF";
        assert!(TimeZone::from_tzif(header.as_slice()).is_err());
        let header = b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF";
        assert!(TimeZone::from_tzif(header.as_slice()).is_err());
        let header = b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF";
        assert!(TimeZone::from_tzif(header.as_slice()).is_err());

        let bytes = b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFTZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x0a\x0f\x0a";
        let result = TimeZone::from_tzif(bytes);
        assert!(result.is_err());

        let bytes = b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFTZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x0a\xc0\x0a";
        let result = TimeZone::from_tzif(bytes);
        assert!(result.is_err());

        let mut header = b"TZif3".to_vec();
        header.append(&mut vec![b'\0'; 15]);
        header.append(&mut vec![b'\0'; 4 * 6]);

        let footer: Vec<u8> = b"\x0aCET-1CEST0M3.5.0,M10.5.0\x0a".to_vec();
        let bytes: Vec<u8> = [header.clone(), header.clone(), footer].concat();
        assert!(TimeZone::from_tzif(bytes.as_slice()).is_err());
        let footer: Vec<u8> = b"\x0aCET-1CEST0\x0a".to_vec();
        let bytes: Vec<u8> = [header.clone(), header.clone(), footer].concat();
        assert!(TimeZone::from_tzif(bytes.as_slice()).is_err());
        let footer: Vec<u8> = b"\x0a\x0a".to_vec();
        let bytes: Vec<u8> = [header.clone(), header.clone(), footer].concat();
        assert!(TimeZone::from_tzif(bytes.as_slice()).is_err());
    }

    #[test]
    fn data_block() {
        let bytes = b"TZif\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\0\xC8";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes = b"TZif\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\0\x65\x22\x9F\xAB";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes = b"TZif\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x01\0\0\0\0\x65\x22\x9F\xAB\x01";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes = b"TZif\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x01\0\0\0\x01\x65\x22\x9F\xAB\x01\x00\x00\x00\x00\x00\x00";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes = b"TZif\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x01\0\0\0\x01\0\0\0\x01\x65\x22\x9F\xAB\x01\x00\x00\x00\x00\x00\x00\x00";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes = b"TZif\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x01\0\0\0\x01\0\0\0\x01\0\0\0\x01\x65\x22\x9F\xAB\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes = b"TZif\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x01\0\0\0\x01\0\0\0\x01\0\0\0\x01\0\0\0\x01\x65\x22\x9F\xAB\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes =
            b"TZif\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x0E\x10\x01\0";
        assert!(TimeZone::from_tzif(bytes).is_ok());
        let bytes =
        b"TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x0E\x10\x01\0TZif3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x0E\x10\x01";
        assert!(TimeZone::from_tzif(bytes).is_err());
        let bytes = b"TZif\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\x0E\x10\0\0";
        let time_zone = TimeZone::from_tzif(bytes).unwrap();
        assert_eq!(
            LocalTimeType::new(3600, false),
            time_zone.to_local_time_type(1704067199)
        );
    }

    #[test]
    fn default_impl() {
        let mut header = b"TZif3".to_vec();
        header.append(&mut vec![b'\0'; 15]);
        header.append(&mut vec![b'\0'; 4 * 6]);

        let footer: Vec<u8> = b"\x0aCET-1CEST,M3.5.0,M10.5.0\x0a".to_vec();
        let bytes: Vec<u8> = [header.clone(), header.clone(), footer].concat();
        let time_zone = TimeZone::from_tzif(bytes.as_slice()).unwrap();

        assert_eq!(
            "TimeZone { transitions: [], local_time_types: [], extra_rule: Some(Alternate(AlternateLocalTimeType { std: LocalTimeType { utoff: 3600, _dst: false }, std_end: MonthWeekDay(3, 5, 0), std_end_time: 7200, dst: LocalTimeType { utoff: 7200, _dst: true }, dst_end: MonthWeekDay(10, 5, 0), dst_end_time: 7200 })) }",
            format!("{:?}", time_zone)
        );

        let bytes =
            b"TZif\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x0E\x10\x01\0";
        let time_zone = TimeZone::from_tzif(bytes).unwrap();

        assert_eq!(
            "Transition { unix_leap_time: 0, local_time_type_index: 0 }",
            format!("{:?}", time_zone.transitions[0])
        );

        let version = Version::V3;
        #[allow(clippy::clone_on_copy)]
        let _ = version.clone();
    }
}
