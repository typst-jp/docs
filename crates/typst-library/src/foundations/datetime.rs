use std::cmp::Ordering;
use std::hash::Hash;
use std::ops::{Add, Sub};

<<<<<<< HEAD
use ecow::{eco_format, EcoString, EcoVec};
use time::error::{Format, InvalidFormatDescription};
use time::macros::format_description;
use time::{format_description, Month, PrimitiveDateTime};

use crate::diag::{bail, StrResult};
use crate::engine::Engine;
use crate::foundations::{
    cast, func, repr, scope, ty, Dict, Duration, Repr, Smart, Str, Value,
};
use crate::World;

/// 日付、時刻、またはその両方を表します。
///
/// この型のコンストラクタ関数を使ってカスタム日時を指定するか、[`datetime.today`]($datetime.today)を使って現在の日付を取得することで作成できます。
///
/// # 例
=======
use ecow::{EcoString, EcoVec, eco_format};
use time::error::{Format, InvalidFormatDescription};
use time::macros::format_description;
use time::{Month, PrimitiveDateTime, format_description};

use crate::World;
use crate::diag::{StrResult, bail};
use crate::engine::Engine;
use crate::foundations::{
    Dict, Duration, Repr, Smart, Str, Value, cast, func, repr, scope, ty,
};

/// Represents a date, a time, or a combination of both.
///
/// Can be created by either specifying a custom datetime using this type's
/// constructor function or getting the current date with [`datetime.today`].
///
/// # Example
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```example
/// #let date = datetime(
///   year: 2020,
///   month: 10,
///   day: 4,
/// )
///
/// #date.display() \
/// #date.display(
///   "y:[year repr:last_two]"
/// )
///
/// #let time = datetime(
///   hour: 18,
///   minute: 2,
///   second: 23,
/// )
///
/// #time.display() \
/// #time.display(
///   "h:[hour repr:12][period]"
/// )
/// ```
///
<<<<<<< HEAD
/// # DatetimeとDuration
/// 2つのdatetimeの差を取ることで、[duration]を取得できます。
=======
/// # Datetime and Duration
/// You can get a [duration] by subtracting two datetime:
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```example
/// #let first-of-march = datetime(day: 1, month: 3, year: 2024)
/// #let first-of-jan = datetime(day: 1, month: 1, year: 2024)
/// #let distance = first-of-march - first-of-jan
/// #distance.hours()
/// ```
///
<<<<<<< HEAD
/// datetimeとdurationを加減算することで、新しい日時（オフセットされたdatetime）を取得することもできます。
=======
/// You can also add/subtract a datetime and a duration to retrieve a new,
/// offset datetime:
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```example
/// #let date = datetime(day: 1, month: 3, year: 2024)
/// #let two-days = duration(days: 2)
/// #let two-days-earlier = date - two-days
/// #let two-days-later = date + two-days
///
/// #date.display() \
/// #two-days-earlier.display() \
/// #two-days-later.display()
/// ```
///
<<<<<<< HEAD
/// # フォーマット
/// [`display`]($datetime.display)メソッドを使うことで、日時をカスタマイズして表示するフォーマットを指定できます。日時のフォーマットは、_コンポーネント_ に _修飾子_ を組み合わせることで指定します。
/// コンポーネントは、日時の中の特定の部分（例えば年や月など）を表します。そして修飾子を使うことで、そのコンポーネントをどのように表示するかを細かく設定できます。
/// コンポーネントを表示するには、コンポーネントの名前を角かっこで囲みます（例：`[[year]]`は年を表示します）。修飾子を追加するには、コンポーネント名の後に半角スペースを入れ、修飾子名、コロン（:）、修飾子の値を記述します（例：`[[month repr:short]]`は月名を短縮形で表示します）。
///
/// 使用可能なコンポーネントと修飾子の組み合わせは以下の通りです。
///
/// - `year`: `datetime`の年を表示します。
///   - `padding`: 年表示のパディングは`zero`（ゼロ）、`space`（空白）、`none`（なし）が指定できます。
///   - `repr`: `full`（4桁表示）`last_two`（2桁表示）を指定できます。
///   - `sign`: 符号の表示を`automatic`（自動）または`mandatory`（常時）で指定できます。
/// - `month`: `datetime`の月を表示します
///   - `padding`: 月表示のパディングは`zero`（ゼロ）、`space`（空白）、`none`（なし）が指定できます。
///   - `repr`: 月を数値で表示するか（`numerical`）、英語の月名（`long`）、英語の略称（`short`）で表示するかを指定できます。
///     残念ながら、現在のところ、月名表示は英語のみです。将来的には多言語対応が予定されています。
/// - `day`: `datetime`の日を表示します。
///   - `padding`: 日表示のパディングは`zero`（ゼロ）、`space`（空白）、`none`（なし）が指定できます。
/// - `week_number`: datetimeの週番号を表示します。
///   - `padding`: 週番号のパディングは`zero`（ゼロ）、`space`（空白）、`none`（なし）が指定できます。
///   - `repr`: `ISO`、`sunday`（日曜開始）、`monday`（月曜開始）を指定できます。ISOの場合、数字は1〜53、それ以外では0〜53で表されます。
/// - `weekday`: `date`の曜日を表示します。
///   - `repr`: 曜日名（`long`）、曜日名の略称（`short`）、日曜（`sunday`）、月曜（`monday`）のいずれかを指定できます。
///     `long`と`short`では、対応する曜日名が英語で表示されます。
///     （月名と同様に、他言語での表示はまだサポートされていません）。
///     `sunday`と`monday`では、日曜または月曜から数えた日数が数値で表示されます。
///   - `one_indexed`: `true`または`false`を指定できます。これは、曜日の数値表示の際、開始日を0とするか1とするかを指定します。
/// - `hour`: `date`の時を表示します。
///   - `padding`: 時表示のパディングは`zero`（ゼロ）、`space`（空白）、`none`（なし）が指定できます。
///   - `repr`: 24時間表示（`24`）または12時間表示（`12`）を指定できます。
/// - `period`: AM/PM（午前・午後）の表示です。
///   - `case`: 小文字（`lower`）、大文字（`upper`）を指定できます。
/// - `minute`: `date`の分を表示します。
///   - `padding`: 分表示のパディングは`zero`（ゼロ）、`space`（空白）、`none`（なし）が指定できます。
/// - `second`: `date`の秒を表示します。
///   - `padding`: 秒表示のパディングは`zero`（ゼロ）、`space`（空白）、`none`（なし）が指定できます。
///
/// 全てのコンポーネントが常に使用できるとは限らない点には注意してください。例えば、`{datetime(year: 2023, month: 10, day: 13)}`のようにして新しい`datetime`を作成すると、内部的には日付のみが保持されるため、`hour`や`minute`のようなコンポーネントは使用できません。それらは特定の時刻が指定された`datetime`でのみ動作します。
#[ty(scope, cast)]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
=======
/// # Format
/// You can specify a customized formatting using the
/// [`display`]($datetime.display) method. The format of a datetime is
/// specified by providing _components_ with a specified number of _modifiers_.
/// A component represents a certain part of the datetime that you want to
/// display, and with the help of modifiers you can define how you want to
/// display that component. In order to display a component, you wrap the name
/// of the component in square brackets (e.g. `[[year]]` will display the year).
/// In order to add modifiers, you add a space after the component name followed
/// by the name of the modifier, a colon and the value of the modifier (e.g.
/// `[[month repr:short]]` will display the short representation of the month).
///
/// The possible combination of components and their respective modifiers is as
/// follows:
///
/// - `year`: Displays the year of the datetime.
///   - `padding`: Can be either `zero`, `space` or `none`. Specifies how the
///     year is padded.
///   - `repr` Can be either `full` in which case the full year is displayed or
///     `last_two` in which case only the last two digits are displayed.
///   - `sign`: Can be either `automatic` or `mandatory`. Specifies when the
///     sign should be displayed.
/// - `month`: Displays the month of the datetime.
///   - `padding`: Can be either `zero`, `space` or `none`. Specifies how the
///     month is padded.
///   - `repr`: Can be either `numerical`, `long` or `short`. Specifies if the
///     month should be displayed as a number or a word. Unfortunately, when
///     choosing the word representation, it can currently only display the
///     English version. In the future, it is planned to support localization.
/// - `day`: Displays the day of the datetime.
///   - `padding`: Can be either `zero`, `space` or `none`. Specifies how the
///     day is padded.
/// - `week_number`: Displays the week number of the datetime.
///   - `padding`: Can be either `zero`, `space` or `none`. Specifies how the
///     week number is padded.
///   - `repr`: Can be either `ISO`, `sunday` or `monday`. In the case of `ISO`,
///      week numbers are between 1 and 53, while the other ones are between 0
///      and 53.
/// - `weekday`: Displays the weekday of the date.
///   - `repr` Can be either `long`, `short`, `sunday` or `monday`. In the case
///     of `long` and `short`, the corresponding English name will be displayed
///     (same as for the month, other languages are currently not supported). In
///     the case of `sunday` and `monday`, the numerical value will be displayed
///     (assuming Sunday and Monday as the first day of the week, respectively).
///   - `one_indexed`: Can be either `true` or `false`. Defines whether the
///     numerical representation of the week starts with 0 or 1.
/// - `hour`: Displays the hour of the date.
///   - `padding`: Can be either `zero`, `space` or `none`. Specifies how the
///     hour is padded.
///   - `repr`: Can be either `24` or `12`. Changes whether the hour is
///     displayed in the 24-hour or 12-hour format.
/// - `period`: The AM/PM part of the hour
///   - `case`: Can be `lower` to display it in lower case and `upper` to
///     display it in upper case.
/// - `minute`: Displays the minute of the date.
///   - `padding`: Can be either `zero`, `space` or `none`. Specifies how the
///     minute is padded.
/// - `second`: Displays the second of the date.
///   - `padding`: Can be either `zero`, `space` or `none`. Specifies how the
///     second is padded.
///
/// Keep in mind that not always all components can be used. For example, if you
/// create a new datetime with `{datetime(year: 2023, month: 10, day: 13)}`, it
/// will be stored as a plain date internally, meaning that you cannot use
/// components such as `hour` or `minute`, which would only work on datetimes
/// that have a specified time.
#[ty(scope, cast)]
#[derive(Debug, Copy, Clone, PartialEq, Hash)]
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
pub enum Datetime {
    /// Representation as a date.
    Date(time::Date),
    /// Representation as a time.
    Time(time::Time),
    /// Representation as a combination of date and time.
    Datetime(time::PrimitiveDateTime),
}

impl Datetime {
    /// Create a datetime from year, month, and day.
    pub fn from_ymd(year: i32, month: u8, day: u8) -> Option<Self> {
        Some(Datetime::Date(
            time::Date::from_calendar_date(year, time::Month::try_from(month).ok()?, day)
                .ok()?,
        ))
    }

    /// Create a datetime from hour, minute, and second.
    pub fn from_hms(hour: u8, minute: u8, second: u8) -> Option<Self> {
        Some(Datetime::Time(time::Time::from_hms(hour, minute, second).ok()?))
    }

    /// Create a datetime from day and time.
    pub fn from_ymd_hms(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Option<Self> {
        let date =
            time::Date::from_calendar_date(year, time::Month::try_from(month).ok()?, day)
                .ok()?;
        let time = time::Time::from_hms(hour, minute, second).ok()?;
        Some(Datetime::Datetime(PrimitiveDateTime::new(date, time)))
    }

    /// Try to parse a dictionary as a TOML date.
    pub fn from_toml_dict(dict: &Dict) -> Option<Self> {
        if dict.len() != 1 {
            return None;
        }

        let Ok(Value::Str(string)) = dict.get("$__toml_private_datetime") else {
            return None;
        };

        if let Ok(d) = time::PrimitiveDateTime::parse(
            string,
            &format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]Z"),
        ) {
            Self::from_ymd_hms(
                d.year(),
                d.month() as u8,
                d.day(),
                d.hour(),
                d.minute(),
                d.second(),
            )
        } else if let Ok(d) = time::PrimitiveDateTime::parse(
            string,
            &format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]"),
        ) {
            Self::from_ymd_hms(
                d.year(),
                d.month() as u8,
                d.day(),
                d.hour(),
                d.minute(),
                d.second(),
            )
        } else if let Ok(d) =
            time::Date::parse(string, &format_description!("[year]-[month]-[day]"))
        {
            Self::from_ymd(d.year(), d.month() as u8, d.day())
        } else if let Ok(d) =
            time::Time::parse(string, &format_description!("[hour]:[minute]:[second]"))
        {
            Self::from_hms(d.hour(), d.minute(), d.second())
        } else {
            None
        }
    }

    /// Which kind of variant this datetime stores.
    pub fn kind(&self) -> &'static str {
        match self {
            Datetime::Datetime(_) => "datetime",
            Datetime::Date(_) => "date",
            Datetime::Time(_) => "time",
        }
    }
}

#[scope]
impl Datetime {
<<<<<<< HEAD
    /// 新しいdatetimeを作成。
    ///
    /// 年、月、日、時、分、秒を指定して[datetime]を作成します。
    ///
    /// _注_：指定する`datetime`の要素によって、Typstが保持する形式は次の3通りのいずれかになります。
    /// * 年、月、日だけを指定した場合、Typstは日付のみを保持します。
    /// * 時、分、秒だけを指定した場合、Typstは時刻のみを保持します。
    /// * 年、月、日、時、分、秒の全てを指定した場合、Typstは完全な日時を保持します。
    ///
    /// 保持形式に応じて、[`display`]($datetime.display)メソッドはデフォルトで異なるフォーマットを選択します。
=======
    /// Creates a new datetime.
    ///
    /// You can specify the [datetime] using a year, month, day, hour, minute,
    /// and second.
    ///
    /// _Note_: Depending on which components of the datetime you specify, Typst
    /// will store it in one of the following three ways:
    /// * If you specify year, month and day, Typst will store just a date.
    /// * If you specify hour, minute and second, Typst will store just a time.
    /// * If you specify all of year, month, day, hour, minute and second, Typst
    ///   will store a full datetime.
    ///
    /// Depending on how it is stored, the [`display`]($datetime.display) method
    /// will choose a different formatting by default.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #datetime(
    ///   year: 2012,
    ///   month: 8,
    ///   day: 3,
    /// ).display()
    /// ```
    #[func(constructor)]
    pub fn construct(
<<<<<<< HEAD
        /// `datetime`の年。
        #[named]
        year: Option<i32>,
        /// `datetime`の月。
        #[named]
        month: Option<Month>,
        /// `datetime`の日。
        #[named]
        day: Option<u8>,
        /// `datetime`の時。
        #[named]
        hour: Option<u8>,
        /// `datetime`の分。
        #[named]
        minute: Option<u8>,
        /// `datetime`の秒。
=======
        /// The year of the datetime.
        #[named]
        year: Option<i32>,
        /// The month of the datetime.
        #[named]
        month: Option<Month>,
        /// The day of the datetime.
        #[named]
        day: Option<u8>,
        /// The hour of the datetime.
        #[named]
        hour: Option<u8>,
        /// The minute of the datetime.
        #[named]
        minute: Option<u8>,
        /// The second of the datetime.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        #[named]
        second: Option<u8>,
    ) -> StrResult<Datetime> {
        let time = match (hour, minute, second) {
            (Some(hour), Some(minute), Some(second)) => {
                match time::Time::from_hms(hour, minute, second) {
                    Ok(time) => Some(time),
                    Err(_) => bail!("time is invalid"),
                }
            }
            (None, None, None) => None,
            _ => bail!("time is incomplete"),
        };

        let date = match (year, month, day) {
            (Some(year), Some(month), Some(day)) => {
                match time::Date::from_calendar_date(year, month, day) {
                    Ok(date) => Some(date),
                    Err(_) => bail!("date is invalid"),
                }
            }
            (None, None, None) => None,
            _ => bail!("date is incomplete"),
        };

        Ok(match (date, time) {
            (Some(date), Some(time)) => {
                Datetime::Datetime(PrimitiveDateTime::new(date, time))
            }
            (Some(date), None) => Datetime::Date(date),
            (None, Some(time)) => Datetime::Time(time),
            (None, None) => {
                bail!("at least one of date or time must be fully specified")
            }
        })
    }

<<<<<<< HEAD
    /// 現在の日付を取得。
=======
    /// Returns the current date.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// Today's date is
    /// #datetime.today().display().
    /// ```
    #[func]
    pub fn today(
        engine: &mut Engine,
<<<<<<< HEAD
        /// 現在のUTC日時に適用するオフセットです。`{auto}`に設定した場合は、ローカルのオフセットが適用されます。
=======
        /// An offset to apply to the current UTC date. If set to `{auto}`, the
        /// offset will be the local offset.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        #[named]
        #[default]
        offset: Smart<i64>,
    ) -> StrResult<Datetime> {
        Ok(engine
            .world
            .today(offset.custom())
            .ok_or("unable to get the current date")?)
    }

<<<<<<< HEAD
    /// 指定したフォーマットで`datetime`を表示します。
    /// 日付のみ、時刻のみ、または両方が指定されているかによって、デフォルトのフォーマットは異なります。
    /// 日付のみ指定されている場合は`[[year]-[month]-[day]]`になります。
    /// 時刻のみ指定されている場合は`[[hour]:[minute]:[second]]`になります。
    /// 日時の両方が指定されている場合は`[[year]-[month]-[day] [hour]:[minute]:[second]]`になります。
    ///
    /// 詳細は[フォーマット構文]($datetime/#format)を参照してください。
    #[func]
    pub fn display(
        &self,
        /// `datetime`を表示する際に使用するフォーマットです。
=======
    /// Displays the datetime in a specified format.
    ///
    /// Depending on whether you have defined just a date, a time or both, the
    /// default format will be different. If you specified a date, it will be
    /// `[[year]-[month]-[day]]`. If you specified a time, it will be
    /// `[[hour]:[minute]:[second]]`. In the case of a datetime, it will be
    /// `[[year]-[month]-[day] [hour]:[minute]:[second]]`.
    ///
    /// See the [format syntax]($datetime/#format) for more information.
    #[func]
    pub fn display(
        &self,
        /// The format used to display the datetime.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        #[default]
        pattern: Smart<DisplayPattern>,
    ) -> StrResult<EcoString> {
        let pat = |s| format_description::parse_borrowed::<2>(s).unwrap();
        let result = match pattern {
            Smart::Auto => match self {
                Self::Date(date) => date.format(&pat("[year]-[month]-[day]")),
                Self::Time(time) => time.format(&pat("[hour]:[minute]:[second]")),
                Self::Datetime(datetime) => {
                    datetime.format(&pat("[year]-[month]-[day] [hour]:[minute]:[second]"))
                }
            },

            Smart::Custom(DisplayPattern(_, format)) => match self {
                Self::Date(date) => date.format(&format),
                Self::Time(time) => time.format(&format),
                Self::Datetime(datetime) => datetime.format(&format),
            },
        };
        result.map(EcoString::from).map_err(format_time_format_error)
    }

<<<<<<< HEAD
    /// 年を返します。年が指定されていない場合や、日付を持たない時間の場合は`{none}`になります。
=======
    /// The year if it was specified, or `{none}` for times without a date.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[func]
    pub fn year(&self) -> Option<i32> {
        match self {
            Self::Date(date) => Some(date.year()),
            Self::Time(_) => None,
            Self::Datetime(datetime) => Some(datetime.year()),
        }
    }

<<<<<<< HEAD
    /// 月を返します。日付を持たない時間の場合は`{none}`になります。
=======
    /// The month if it was specified, or `{none}` for times without a date.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[func]
    pub fn month(&self) -> Option<u8> {
        match self {
            Self::Date(date) => Some(date.month().into()),
            Self::Time(_) => None,
            Self::Datetime(datetime) => Some(datetime.month().into()),
        }
    }

<<<<<<< HEAD
    /// （月曜日を1とする）曜日を返します。日付を持たない時間の場合は`{none}`になります。
=======
    /// The weekday (counting Monday as 1) or `{none}` for times without a date.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[func]
    pub fn weekday(&self) -> Option<u8> {
        match self {
            Self::Date(date) => Some(date.weekday().number_from_monday()),
            Self::Time(_) => None,
            Self::Datetime(datetime) => Some(datetime.weekday().number_from_monday()),
        }
    }

<<<<<<< HEAD
    /// 日を返します。日付を持たない時間の場合は`{none}`になります。
=======
    /// The day if it was specified, or `{none}` for times without a date.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[func]
    pub fn day(&self) -> Option<u8> {
        match self {
            Self::Date(date) => Some(date.day()),
            Self::Time(_) => None,
            Self::Datetime(datetime) => Some(datetime.day()),
        }
    }

<<<<<<< HEAD
    /// 時を返します。時刻を持たない日付の場合は`{none}`になります。
=======
    /// The hour if it was specified, or `{none}` for dates without a time.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[func]
    pub fn hour(&self) -> Option<u8> {
        match self {
            Self::Date(_) => None,
            Self::Time(time) => Some(time.hour()),
            Self::Datetime(datetime) => Some(datetime.hour()),
        }
    }

<<<<<<< HEAD
    /// 分を返します。時刻を持たない日付の場合は`{none}`になります。
=======
    /// The minute if it was specified, or `{none}` for dates without a time.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[func]
    pub fn minute(&self) -> Option<u8> {
        match self {
            Self::Date(_) => None,
            Self::Time(time) => Some(time.minute()),
            Self::Datetime(datetime) => Some(datetime.minute()),
        }
    }

<<<<<<< HEAD
    /// 秒を返します。時刻を持たない日付の場合は`{none}`になります。
=======
    /// The second if it was specified, or `{none}` for dates without a time.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[func]
    pub fn second(&self) -> Option<u8> {
        match self {
            Self::Date(_) => None,
            Self::Time(time) => Some(time.second()),
            Self::Datetime(datetime) => Some(datetime.second()),
        }
    }

<<<<<<< HEAD
    /// 年の通算日（1年の中での通し番号）を返します。
    /// 日付を持たない時刻の場合は `{none}` になります。
=======
    /// The ordinal (day of the year), or `{none}` for times without a date.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[func]
    pub fn ordinal(&self) -> Option<u16> {
        match self {
            Self::Datetime(datetime) => Some(datetime.ordinal()),
            Self::Date(date) => Some(date.ordinal()),
            Self::Time(_) => None,
        }
    }
}

impl Repr for Datetime {
    fn repr(&self) -> EcoString {
        let year = self.year().map(|y| eco_format!("year: {}", (y as i64).repr()));
        let month = self.month().map(|m| eco_format!("month: {}", (m as i64).repr()));
        let day = self.day().map(|d| eco_format!("day: {}", (d as i64).repr()));
        let hour = self.hour().map(|h| eco_format!("hour: {}", (h as i64).repr()));
        let minute = self.minute().map(|m| eco_format!("minute: {}", (m as i64).repr()));
        let second = self.second().map(|s| eco_format!("second: {}", (s as i64).repr()));
        let filtered = [year, month, day, hour, minute, second]
            .into_iter()
            .flatten()
            .collect::<EcoVec<_>>();

        eco_format!("datetime{}", &repr::pretty_array_like(&filtered, false))
    }
}

impl PartialOrd for Datetime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Datetime(a), Self::Datetime(b)) => a.partial_cmp(b),
            (Self::Date(a), Self::Date(b)) => a.partial_cmp(b),
            (Self::Time(a), Self::Time(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl Add<Duration> for Datetime {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        let rhs: time::Duration = rhs.into();
        match self {
            Self::Datetime(datetime) => Self::Datetime(datetime + rhs),
            Self::Date(date) => Self::Date(date + rhs),
            Self::Time(time) => Self::Time(time + rhs),
        }
    }
}

impl Sub<Duration> for Datetime {
    type Output = Self;

    fn sub(self, rhs: Duration) -> Self::Output {
        let rhs: time::Duration = rhs.into();
        match self {
            Self::Datetime(datetime) => Self::Datetime(datetime - rhs),
            Self::Date(date) => Self::Date(date - rhs),
            Self::Time(time) => Self::Time(time - rhs),
        }
    }
}

impl Sub for Datetime {
    type Output = StrResult<Duration>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Datetime(a), Self::Datetime(b)) => Ok((a - b).into()),
            (Self::Date(a), Self::Date(b)) => Ok((a - b).into()),
            (Self::Time(a), Self::Time(b)) => Ok((a - b).into()),
            (a, b) => bail!("cannot subtract {} from {}", b.kind(), a.kind()),
        }
    }
}

/// A format in which a datetime can be displayed.
pub struct DisplayPattern(Str, format_description::OwnedFormatItem);

cast! {
    DisplayPattern,
    self => self.0.into_value(),
    v: Str => {
        let item = format_description::parse_owned::<2>(&v)
            .map_err(format_time_invalid_format_description_error)?;
        Self(v, item)
    }
}

cast! {
    Month,
    v: u8 => Self::try_from(v).map_err(|_| "month is invalid")?
}

/// Format the `Format` error of the time crate in an appropriate way.
fn format_time_format_error(error: Format) -> EcoString {
    match error {
        Format::InvalidComponent(name) => eco_format!("invalid component '{}'", name),
        Format::InsufficientTypeInformation { .. } => {
            "failed to format datetime (insufficient information)".into()
        }
        err => eco_format!("failed to format datetime in the requested format ({err})"),
    }
}

/// Format the `InvalidFormatDescription` error of the time crate in an
/// appropriate way.
fn format_time_invalid_format_description_error(
    error: InvalidFormatDescription,
) -> EcoString {
    match error {
        InvalidFormatDescription::UnclosedOpeningBracket { index, .. } => {
            eco_format!("missing closing bracket for bracket at index {}", index)
        }
        InvalidFormatDescription::InvalidComponentName { name, index, .. } => {
            eco_format!("invalid component name '{}' at index {}", name, index)
        }
        InvalidFormatDescription::InvalidModifier { value, index, .. } => {
            eco_format!("invalid modifier '{}' at index {}", value, index)
        }
        InvalidFormatDescription::Expected { what, index, .. } => {
            eco_format!("expected {} at index {}", what, index)
        }
        InvalidFormatDescription::MissingComponentName { index, .. } => {
            eco_format!("expected component name at index {}", index)
        }
        InvalidFormatDescription::MissingRequiredModifier { name, index, .. } => {
            eco_format!(
                "missing required modifier {} for component at index {}",
                name,
                index
            )
        }
        InvalidFormatDescription::NotSupported { context, what, index, .. } => {
            eco_format!("{} is not supported in {} at index {}", what, context, index)
        }
        err => eco_format!("failed to parse datetime format ({err})"),
    }
}
