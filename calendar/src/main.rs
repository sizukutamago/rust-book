use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

use chrono::NaiveDateTime;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Schedule {
    id: u64,
    subject: String,
    start: NaiveDateTime,
    end: NaiveDateTime,
}

impl Schedule {
    fn intersects(&self, other: &Schedule) -> bool {
        self.start < other.end && other.start < self.end
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Calendar {
    schedules: Vec<Schedule>,
}

const SCHEDULE_FILE: &str = "schedule.json";

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List,
    Add {
        subject: String,
        start: NaiveDateTime,
        end: NaiveDateTime,
    },
    Delete {
        id: u64,
    },
}

#[derive(thiserror::Error, Debug)]
enum MyError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

fn main() {
    let options = Cli::parse();

    match options.command {
        Commands::List => match read_calendar() {
            Ok(calendar) => {
                show_list(&calendar);
            }
            Err(error) => {
                println!("カレンダーの読み込みに失敗しました");
            }
        },
        Commands::Add {
            subject,
            start,
            end,
        } => {
            let mut calendar = read_calendar().unwrap();

            if add_schedule(&mut calendar, subject, start, end) {
                save_calendar(&mut calendar);
                println!("予定を追加しました");
            } else {
                println!("エラー: 予定が重複しています");
            }
        }
        Commands::Delete { id } => {
            let mut calendar = read_calendar().unwrap();

            if delete_schedule(&mut calendar, id) {
                save_calendar(&mut calendar);
                println!("予定を削除しました");
            } else {
                println!("エラー: IDが不正です");
            }
        }
    }
}

fn read_calendar() -> Result<Calendar, std::io::Error> {
    let file = File::open(SCHEDULE_FILE)?;
    let reader = BufReader::new(file);
    let calendar = serde_json::from_reader(reader).unwrap();
    Ok(calendar)
}

fn save_calendar(calendar: &mut Calendar) -> Result<(), MyError> {
    let file = File::create(SCHEDULE_FILE)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, calendar)?;
    Ok(())
}

fn show_list(calendar: &Calendar) {
    println!("ID\tSTART\tEND\tSUBJECT");

    for schedule in &calendar.schedules {
        println!(
            "{}\t{}\t{}\t{}",
            schedule.id, schedule.start, schedule.end, schedule.subject
        );
    }
}

fn add_schedule(
    calendar: &mut Calendar,
    subject: String,
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> bool {
    let id = calendar.schedules.len() as u64;
    let new_schedule = Schedule {
        id,
        subject,
        start,
        end,
    };

    for schedule in &calendar.schedules {
        if schedule.intersects(&new_schedule) {
            return false;
        }
    }

    calendar.schedules.push(new_schedule);
    true
}

fn delete_schedule(calendar: &mut Calendar, id: u64) -> bool {
    for i in 0..calendar.schedules.len() {
        if calendar.schedules[i].id == id {
            calendar.schedules.remove(i);
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use chrono::naive;
    use rstest::rstest;

    fn naive_date_time(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> NaiveDateTime {
        chrono::NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(hour, minute, second)
            .unwrap()
    }

    #[rstest]
    #[case(18, 15, 19, 15, true)]
    #[case(19, 45, 20, 45, true)]
    #[case(18, 30, 20, 15, true)]
    #[case(20, 15, 20, 45, false)]
    #[case(18, 15, 18, 45, false)]
    #[case(19, 15, 19, 45, true)]
    fn test_schedule_intersects(
        #[case] h0: u32,
        #[case] m0: u32,
        #[case] h1: u32,
        #[case] m1: u32,
        #[case] should_intersect: bool,
    ) {
        let schedule = Schedule {
            id: 0,
            subject: "既存予定".to_string(),
            start: naive_date_time(2024, 1, 1, h0, m0, 0),
            end: naive_date_time(2024, 1, 1, h1, m1, 0),
        };
        let new_schedule = Schedule {
            id: 999,
            subject: "新規予定".to_string(),
            start: naive_date_time(2024, 1, 1, 19, 0, 0),
            end: naive_date_time(2024, 1, 1, 20, 0, 0),
        };
        assert_eq!(should_intersect, schedule.intersects(&new_schedule));
    }

    #[test]
    fn test_delete_schedule() {
        let mut calendar = Calendar {
            schedules: vec![
                Schedule {
                    id: 0,
                    subject: "テスト予定".to_string(),
                    start: naive_date_time(2023, 11, 19, 11, 22, 33),
                    end: naive_date_time(2023, 11, 19, 22, 33, 44),
                },
                Schedule {
                    id: 1,
                    subject: "テスト予定2".to_string(),
                    start: naive_date_time(2023, 12, 8, 9, 0, 0),
                    end: naive_date_time(2023, 12, 8, 19, 30, 0),
                },
                Schedule {
                    id: 2,
                    subject: "追加できる予定".to_string(),
                    start: naive_date_time(2023, 12, 15, 10, 0, 0),
                    end: naive_date_time(2023, 12, 15, 11, 00, 0),
                },
            ],
        };
        assert!(delete_schedule(&mut calendar, 0));

        let expected = Calendar {
            schedules: vec![
                Schedule {
                    id: 1,
                    subject: "テスト予定2".to_string(),
                    start: naive_date_time(2023, 12, 8, 9, 0, 0),
                    end: naive_date_time(2023, 12, 8, 19, 30, 0),
                },
                Schedule {
                    id: 2,
                    subject: "追加できる予定".to_string(),
                    start: naive_date_time(2023, 12, 15, 10, 0, 0),
                    end: naive_date_time(2023, 12, 15, 11, 00, 0),
                },
            ],
        };

        assert_eq!(expected, calendar);

        assert!(delete_schedule(&mut calendar, 1));

        let expected = Calendar {
            schedules: vec![Schedule {
                id: 2,
                subject: "追加できる予定".to_string(),
                start: naive_date_time(2023, 12, 15, 10, 0, 0),
                end: naive_date_time(2023, 12, 15, 11, 00, 0),
            }],
        };
        assert_eq!(expected, calendar);

        assert!(delete_schedule(&mut calendar, 2));

        let expected = Calendar { schedules: vec![] };

        assert_eq!(expected, calendar);
    }
}
