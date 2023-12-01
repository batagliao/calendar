use std::io::stdout;

use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveDateTime, TimeZone, Weekday};
use crossterm::{
    style::{SetAttribute, Stylize},
    ExecutableCommand,
};
use term_grid::{Alignment, Cell, Direction, Filling, Grid, GridOptions};
use unicode_width::UnicodeWidthStr;

fn main() {
    let mut grid = Grid::new(GridOptions {
        filling: Filling::Spaces(1),
        direction: Direction::LeftToRight,
    });

    let today = Local::now();
    let year = today.year();
    let month = today.month();
    let weekday = today.weekday().num_days_from_sunday();
    let days_in_month = days_in_month(year, month);

    // fist we need to add weekdays label
    grid.add(Cell {
        contents: " Dom".dark_green().to_string().into(),
        alignment: Alignment::Left,
        width: UnicodeWidthStr::width(" Dom"),
    });
    grid.add(Cell {
        contents: " Seg".dark_cyan().to_string().into(),
        alignment: Alignment::Left,
        width: UnicodeWidthStr::width(" Seg"),
    });
    grid.add(Cell {
        contents: " Ter".dark_cyan().to_string().into(),
        alignment: Alignment::Left,
        width: UnicodeWidthStr::width(" Ter"),
    });
    grid.add(Cell {
        contents: " Qua".dark_cyan().to_string().into(),
        alignment: Alignment::Left,
        width: UnicodeWidthStr::width(" Qua"),
    });
    grid.add(Cell {
        contents: " Qui".dark_cyan().to_string().into(),
        alignment: Alignment::Left,
        width: UnicodeWidthStr::width(" Qui"),
    });
    grid.add(Cell {
        contents: " Sex".dark_cyan().to_string().into(),
        alignment: Alignment::Left,
        width: UnicodeWidthStr::width(" Sex"),
    });
    grid.add(Cell {
        contents: " Sab".dark_cyan().to_string().into(),
        alignment: Alignment::Left,
        width: UnicodeWidthStr::width(" Sab"),
    });

    // filling week day space
    let space_to_add = (weekday) % 7;
    for _ in 0..space_to_add {
        grid.add(Cell::from(" "));
    }

    for day in 1..=days_in_month {
        let day_string = day.to_string() + " ";
        let daystr = day_string.as_str();
        let mut cell_content = day.to_string() + " ";

        // sunday (or holiday)
        let dt = NaiveDate::from_ymd_opt(year, month, day).unwrap();

        if Local
            .from_local_date(&dt)
            .unwrap()
            .weekday()
            .number_from_sunday()
            == 1
        {
            cell_content = cell_content.green().bold().to_string()
        }

        if today.day() == day {
            cell_content = cell_content.bold().negative().to_string()
        }

        let mut cell = Cell::from(cell_content);
        cell.alignment = Alignment::Right;
        cell.width = UnicodeWidthStr::width(daystr);
        grid.add(cell);
    }

    println!();
    println!("{}{} - {}", "\t", name_of_month(month), year);
    println!("{}", grid.fit_into_columns(7));
}

/// Takes in a year (e.g. 2019) and returns the number of days in that year.
fn days_in_year(year: i32) -> i32 {
    if year % 400 == 0 {
        366
    } else if year % 100 == 0 {
        365
    } else if year % 4 == 0 {
        366
    } else {
        365
    }
}

/// Takes in a year and month (e.g. 2020, February) and returns the number of days in that month.
fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 => 31,
        2 if days_in_year(year) == 366 => 29,
        2 => 28,
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        0_u32 | 13_u32..=u32::MAX => 0,
    }
}

fn name_of_month(month: u32) -> &'static str {
    match month {
        1 => "Janeiro",
        2 => "Fevereiro",
        3 => "Março",
        4 => "Abril",
        5 => "Maio",
        6 => "Junho",
        7 => "Julho",
        8 => "Agosto",
        9 => "Setembro",
        10 => "Outubro",
        11 => "Novembro",
        12 => "Dezembro",
        _ => "Mês Inválido",
    }
}
