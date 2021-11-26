mod bus;
mod controller;
mod customer;
mod flight;
mod hotels;
mod mysql_connection;
mod reservation;
mod view;

#[macro_use]
extern crate prettytable;
extern crate lazy_static;

use crate::controller::Controller;
use crate::mysql_connection::MySQLConnection;
use crate::view::*;
use prettytable::Row;
use std::io::{stdin, stdout, Stdin, Write};
use std::thread::sleep;
use std::{io, time};

pub fn run() {
    let url = "mysql://root:12345678@localhost:3306/BookingSystem";
    let mut controller = Controller::new(MySQLConnection::new(url));

    let time = time::Duration::from_secs(1);
    sleep(time);
    controller.check_consistency();
    while controller.run() == Status::Login {}
    println!("------ 程序结束 ------");
}

/// macro that use to read a specific type of value
#[macro_export]
#[allow(unused_macros)]
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().parse::<$type>().expect("Parsable");
    };
}

#[derive(Eq, PartialEq)]
pub enum Status {
    Quit = 0,
    Continue = 1,
    Login = 2,
}

trait ToRow {
    fn to_row(&self) -> Row;
}
