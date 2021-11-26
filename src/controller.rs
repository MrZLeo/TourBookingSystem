use crate::bus::Bus;
use crate::customer::Customer;
use crate::flight::Flight;
use crate::hotels::Hotels;
use crate::reservation::Reservation;
use crate::view::View;
use crate::{read, MySQLConnection, Status};
use lazy_static::lazy_static;
use mysql::PooledConn;
use prettytable::Row;
use std::collections::{HashMap, HashSet};
use std::io::{stdin, stdout, Read, Stdin, Write};
use std::iter::FromIterator;
use std::process::exit;

lazy_static! {
    static ref FLIGHT_HINT: Row = row!["航班号", "价格", "出发城市", "到达城市"];
    static ref HOTEL_HINT: Row = row!["大巴号", "地点", "价格"];
    static ref BUS_HINT: Row = row!["酒店号", "地点", "价格"];
}

pub struct Controller {
    flights: Vec<Flight>,
    hotels: Vec<Hotels>,
    bus: Vec<Bus>,
    customers: Vec<Customer>,
    reservations: Vec<Reservation>,
    connection: MySQLConnection,
    current_user: u32,
}

impl Controller {
    pub fn new(sql: MySQLConnection) -> Controller {
        Controller {
            flights: vec![],
            hotels: vec![],
            bus: vec![],
            customers: vec![],
            reservations: vec![],
            connection: sql,
            current_user: 0,
        }
    }

    pub fn connection_mut(&mut self) -> &mut MySQLConnection {
        &mut self.connection
    }

    pub fn book_hotel(&mut self, res_id: String) -> bool {
        self.connection.book_hotel(self.current_user, res_id)
    }

    pub fn book_flight(&mut self, res_id: String) -> bool {
        self.connection.book_flight(self.current_user, res_id)
    }

    pub fn book_bus(&mut self, res_id: String) -> bool {
        self.connection.book_bus(self.current_user, res_id)
    }

    pub fn query_hotel(&mut self) {
        self.hotels = self.connection.query_hotel(self.current_user);
    }

    pub fn query_flight(&mut self) {
        self.flights = self.connection.query_flight(self.current_user);
    }

    pub fn query_bus(&mut self) {
        self.bus = self.connection.query_bus(self.current_user);
    }

    pub fn query_all_flight(&mut self) {
        self.flights = self.connection.query_all_flight();
    }

    pub fn query_all_hotel(&mut self) {
        self.hotels = self.connection.query_all_hotel();
    }

    pub fn query_all_bus(&mut self) {
        self.bus = self.connection.query_all_bus();
    }

    /// check consistency as soon as the system is opened or closed.
    /// error code: -2 stands for DBCC error
    pub fn check_consistency(&mut self) {
        View::init_check();
        if self.connection.check_consistency() == false {
            eprintln!("Fatal Error: Database Consistency Check(DBCC) failed");
            exit(-2);
        }
        View::success_hint();
    }

    pub fn login_view(&mut self) {
        View::login_menu();
        let mut login_cnt = 1;
        while self.login() == false && login_cnt < 3 {
            login_cnt += 1;
            View::login_menu();
        }
        if login_cnt == 3 {
            println!("三次输入账号错误, 程序自动退出...");
            exit(-1);
        }
    }

    fn sign_up(&mut self, id: u32) -> bool {
        View::sign_up(id);
        read!(comfirm as char);
        let mut cnt = 0;
        loop {
            if comfirm == 'y' || comfirm == 'Y' {
                self.add_user(id);
                self.current_user = id;
                return true;
            } else if comfirm == 'n' || comfirm == 'N' {
                return false;
            } else {
                println!("请输入正确的指令");
                cnt += 1;
                if cnt == 3 {
                    println!("三次输入错误，程序自动退出...");
                    break;
                }
            }
        }
        return false;
    }

    fn login(&mut self) -> bool {
        read!(user_id as u32);
        return if self.connection.user_exist(user_id) {
            self.current_user = user_id;
            println!("您好，{}!", self.connection.query_user_name(user_id));
            true
        } else {
            self.sign_up(user_id)
        };
    }
    fn add_user(&mut self, id: u32) {
        View::new_name();
        read!(name as String);
        self.connection.add_user(id, name);
    }

    fn user_menu(&mut self) -> Status {
        View::user_menu();
        read!(num as u32);
        match num {
            1 => self.booking(),
            2 => self.cancel_book(),
            3 => self.querying(),
            4 => self.travel_path(),
            5 => View::check_completeness(self.check_completeness()),
            0 => return Status::Login,
            _ => return Status::Quit,
        };
        return Status::Continue;
    }

    fn booking(&mut self) -> bool {
        View::booking_menu();
        read!(num as u32);
        match num {
            1 => {
                self.query_all_flight();
                View::booking(&FLIGHT_HINT, self.flights());
                read!(res_id as String);
                let success = self.book_flight(res_id);
                if success {
                    View::success_hint();
                }
            }
            2 => {
                self.query_all_bus();
                View::booking(&BUS_HINT, self.bus());
                read!(res_id as String);
                let success = self.book_bus(res_id);
                if success {
                    View::success_hint();
                }
            }
            3 => {
                self.query_all_hotel();
                View::booking(&HOTEL_HINT, self.hotels());
                read!(res_id as String);
                let success = self.book_hotel(res_id);
                if success {
                    View::success_hint();
                }
            }
            4 => {}
            _ => {
                return true;
            }
        };
        return false;
    }

    fn cancel_book(&mut self) -> bool {
        View::cancel_menu();
        read!(num as u32);
        match num {
            1 => {
                self.query_flight();
                View::querying(&FLIGHT_HINT, self.flights());
                View::cancel_hint();
                read!(res_id as String);
                let success = self.cancel_flight(res_id);
                if success {
                    View::success_hint();
                }
            }
            2 => {
                self.query_bus();
                View::querying(&BUS_HINT, self.bus());
                View::cancel_hint();
                read!(res_id as String);
                let success = self.cancel_bus(res_id);
                if success {
                    View::success_hint();
                }
            }
            3 => {
                self.query_hotel();
                View::querying(&HOTEL_HINT, self.hotels());
                View::cancel_hint();
                read!(res_id as String);
                let success = self.cancel_hotel(res_id);
                if success {
                    View::success_hint();
                }
            }
            _ => (),
        };

        return false;
    }

    fn querying(&mut self) -> bool {
        View::query_menu();
        read!(num as u32);
        match num {
            1 => {
                self.query_all_flight();
                View::querying(&FLIGHT_HINT, self.flights())
            }
            2 => {
                self.query_all_bus();
                View::querying(&BUS_HINT, self.bus())
            }
            3 => {
                self.query_all_hotel();
                View::querying(&HOTEL_HINT, self.hotels())
            }
            _ => (),
        }
        return false;
    }

    fn travel_path(&mut self) -> bool {
        self.query_flight();
        self.query_bus();
        self.query_hotel();

        View::travel_flight();
        View::querying(&FLIGHT_HINT, self.flights());

        View::travel_bus();
        View::querying(&BUS_HINT, self.bus());

        View::travel_hotel();
        View::querying(&HOTEL_HINT, self.hotels());
        return true;
    }

    /// # check the completeness:
    /// 1. hotel can't be outside of the region of all flights that user is booked.
    /// 2. all destination can be arrived, which means all flights can shape a graph
    ///    that calls euler path.
    ///
    /// In the function, we erect a `Map` to indicate relation **place <-> in/out**.
    ///
    /// If a flight go from a city, this city has one point deducted, and if a flight go to a city,
    /// this city has one point added.
    ///
    /// In the end, if all city is zero, or just one city is 1 while one city is -1, we have a euler
    /// graph. Therefore, completeness is ensured, otherwise, it is broken.
    fn check_completeness(&mut self) -> bool {
        self.query_flight();
        let mut all_city: Vec<String> = Vec::new();
        for flight in &self.flights {
            all_city.push(flight.from_city().to_string());
            all_city.push(flight.arrive_city().to_string());
        }

        let city: HashSet<String> = HashSet::from_iter(all_city.into_iter());
        let mut euler_graph: HashMap<String, i32> =
            HashMap::from_iter(city.iter().map(|x| (x.clone(), 0)));
        self.flights.iter().for_each(|x| {
            let from = euler_graph.get_mut(x.from_city()).unwrap();
            *from -= 1;
            let arrive = euler_graph.get_mut(x.arrive_city()).unwrap();
            *arrive += 1;
        });

        let mut positive = 0;
        let mut negative = 0;
        euler_graph.iter().for_each(|x| {
            if x.1 == &1 {
                positive += 1;
            } else if x.1 == &(-1) {
                negative += 1;
            }
        });

        if positive > 1 || negative > 1 {
            return false;
        } else if positive == 1 && negative != 1 {
            return false;
        } else if positive != 1 && negative == 1 {
            return false;
        }

        self.query_hotel();
        for hotel in &self.hotels {
            if !city.contains(hotel.location()) {
                return false;
            }
        }

        self.query_bus();
        for bus in &self.bus {
            if !city.contains(bus.location()) {
                return false;
            }
        }

        return true;
    }

    pub fn run(&mut self) -> Status {
        self.login_view();
        loop {
            let ret = self.user_menu();
            if !(ret == Status::Continue) {
                return ret;
            }
        }
    }

    pub fn flights(&self) -> &Vec<Flight> {
        &self.flights
    }
    pub fn hotels(&self) -> &Vec<Hotels> {
        &self.hotels
    }
    pub fn bus(&self) -> &Vec<Bus> {
        &self.bus
    }

    fn cancel_flight(&mut self, res_id: String) -> bool {
        self.connection.cancel_flight(self.current_user, res_id)
    }

    fn cancel_bus(&mut self, res_id: String) -> bool {
        self.connection.cancel_bus(self.current_user, res_id)
    }

    fn cancel_hotel(&mut self, res_id: String) -> bool {
        self.connection.cancel_hotel(self.current_user, res_id)
    }
}
