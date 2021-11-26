use crate::bus::Bus;
use crate::flight::Flight;
use crate::hotels::Hotels;
use crate::reservation::Reservation;
use mysql::prelude::Queryable;
use mysql::{params, Error, Pool, PooledConn, Transaction};
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::task::Poll;

pub struct MySQLConnection {
    url: String,
    pool: Pool,
    conn: PooledConn,
}

impl MySQLConnection {
    pub fn new(url: &str) -> Self {
        let url = url;
        let pool = Pool::new(url).unwrap();
        let conn = pool.get_conn().unwrap();
        MySQLConnection {
            url: url.to_string(),
            pool,
            conn,
        }
    }

    // 安全检查:
    // 1. 检查操作时用户名是否存在
    // 2. 检查操作时的剩余数量和总数量之间的关系
    pub fn user_exist(&mut self, user_id: u32) -> bool {
        let res = self
            .conn
            .exec_first(
                "SELECT count(*) FROM Customer WHERE id = :user_id",
                params! {
                    "user_id" => user_id
                },
            )
            .map(|row| row.map(|num_of_user: u32| num_of_user));

        return if res.unwrap().unwrap() == 0 {
            // println!("Error: user doesn't exist");
            false
        } else {
            true
        };
    }

    // 预定操作
    pub fn book_hotel(&mut self, user_id: u32, res_id: String) -> bool {
        self.user_exist(user_id);

        let res = self.conn.exec_first(
            "SELECT count(*)
             FROM Hotels
             WHERE hotel_num = :res_id",
            params! {
                "res_id" => &res_id
            },
        );

        if res.unwrap() == Some(0) {
            eprintln!("错误：未找到想要预约的酒店");
            return false;
        }

        self.conn.exec_drop(
            "INSERT INTO Reservation \
            VALUES (:customer_id, 2, :res_id, null);",
            params! {
                "customer_id" => user_id,
                "res_id" => res_id,
            },
        );

        return true;
    }

    pub fn book_bus(&mut self, user_id: u32, res_id: String) -> bool {
        self.user_exist(user_id);

        let res = self.conn.exec_first(
            "SELECT count(*)
             FROM Bus
             WHERE bus_num = :res_id",
            params! {
                "res_id" => &res_id
            },
        );

        if res.unwrap() == Some(0) {
            eprintln!("错误：未找到想要预约的大巴");
            return false;
        }

        self.conn.exec_drop(
            "INSERT INTO Reservation \
            VALUES (:customer_id, 3, :res_id, null);",
            params! {
                "customer_id" => user_id,
                "res_id" => res_id,
            },
        );
        return true;
    }

    pub fn book_flight(&mut self, user_id: u32, res_id: String) -> bool {
        self.user_exist(user_id);

        let res = self.conn.exec_first(
            "SELECT count(*) FROM Flights WHERE flight_num = :res_id",
            params! {
                "res_id" => &res_id
            },
        );

        if res.unwrap() == Some(0) {
            eprintln!("错误：未找到想要预约的航班");
            return false;
        }

        self.conn.exec_drop(
            r"INSERT INTO Reservation
            VALUES (:customer_id, 1, :res_id, null);",
            params! {
                "customer_id" => user_id,
                "res_id" => res_id,
            },
        );
        return true;
    }

    // 查询操作
    pub fn query_flight(&mut self, user_id: u32) -> Vec<Flight> {
        self.user_exist(user_id);

        self.conn
            .query_map(
                format!(
                    "SELECT flight_num, price, from_city, arrive_city
                     FROM Flights, Reservation
                     WHERE customer_id = {}
                     AND res_type = 1
                     AND Flights.flight_num = Reservation.res_id",
                    user_id
                ),
                |(flight_num, price, from_city, arrive_city)| {
                    Flight::new(flight_num, price, from_city, arrive_city)
                },
            )
            .expect("Error: query flight failed")
    }

    pub fn query_hotel(&mut self, user_id: u32) -> Vec<Hotels> {
        self.user_exist(user_id);

        self.conn
            .query_map(
                format!(
                    "SELECT hotel_num, location, price
                     FROM Hotels, Reservation
                     WHERE customer_id = {}
                     AND res_type = 2
                     AND Hotels.hotel_num = Reservation.res_id",
                    user_id
                ),
                |(hotel_num, location, price)| Hotels::new(hotel_num, location, price),
            )
            .expect("Error: query hotel failed")
    }

    pub fn query_bus(&mut self, user_id: u32) -> Vec<Bus> {
        self.user_exist(user_id);

        self.conn
            .query_map(
                format!(
                    "SELECT bus_num, location, price
                     FROM Bus, Reservation
                     WHERE customer_id = {}
                     AND res_type = 3
                     AND Bus.bus_num = Reservation.res_id",
                    user_id
                ),
                |(bus_num, location, price)| Bus::new(bus_num, location, price),
            )
            .expect("Error: query bus failed")
    }

    pub fn query_all_flight(&mut self) -> Vec<Flight> {
        self.conn
            .query_map(
                "SELECT flight_num, price, from_city, arrive_city
                FROM Flights",
                |(flight_num, price, from_city, arrive_city)| {
                    Flight::new(flight_num, price, from_city, arrive_city)
                },
            )
            .expect("Error: query flight failed")
    }

    pub fn query_all_hotel(&mut self) -> Vec<Hotels> {
        self.conn
            .query_map(
                "SELECT hotel_num, location, price FROM Hotels",
                |(hotel_num, location, price)| Hotels::new(hotel_num, location, price),
            )
            .expect("Error: query hotel failed")
    }

    pub fn query_all_bus(&mut self) -> Vec<Bus> {
        self.conn
            .query_map(
                "SELECT bus_num, location, price FROM Bus",
                |(bus_num, location, price)| Bus::new(bus_num, location, price),
            )
            .expect("Error: query bus failed")
    }

    pub fn query_user_name(&mut self, user_id: u32) -> String {
        // 已经确定用户存在，所以直接解包
        self.conn
            .query_first(format!(
                "SELECT name \
                FROM customer \
                WHERE id = {}",
                user_id
            ))
            .unwrap()
            .unwrap()
    }

    pub fn check_consistency(&mut self) -> bool {
        let res = self
            .conn
            .query_map(
                "SELECT res_type, res_id, count(*) FROM Reservation GROUP BY res_id, res_type",
                |(res_type, res_id, num): (u32, String, u32)| (res_type, res_id, num),
            )
            .expect("user_id should be u32");

        for (res_type, res_id, num) in res.iter() {
            if match res_type {
                1 => self.flight_consistency(res_id, num),
                2 => self.hotel_consistency(res_id, num),
                3 => self.bus_consistency(res_id, num),
                _ => false,
            } == false
            {
                return false;
            }
        }

        return true;
    }

    fn flight_consistency(&mut self, res_id: &String, res_num: &u32) -> bool {
        let res: Option<(u32, u32)> = self
            .conn
            .query_first(format!(
                "SELECT num_seat, num_available \
                FROM Flights \
                WHERE flight_num = '{}'",
                res_id
            ))
            .unwrap();
        return match res {
            None => false,
            Some((sum, avail)) => sum - avail == res_num.clone(),
        };
    }

    fn bus_consistency(&mut self, res_id: &String, res_num: &u32) -> bool {
        let res: Option<(u32, u32)> = self
            .conn
            .query_first(format!(
                "SELECT num_bus, num_available \
                FROM Bus \
                WHERE bus_num = '{}'",
                res_id
            ))
            .unwrap();
        return match res {
            None => false,
            Some((sum, avail)) => sum - avail == res_num.clone(),
        };
    }

    fn hotel_consistency(&mut self, res_id: &String, res_num: &u32) -> bool {
        let res: Option<(u32, u32)> = self
            .conn
            .query_first(format!(
                "SELECT num_rooms, num_available \
                FROM Hotels \
                WHERE hotel_num = '{}'",
                res_id
            ))
            .unwrap();
        return match res {
            None => false,
            Some((sum, avail)) => sum - avail == res_num.clone(),
        };
    }

    pub fn add_user(&mut self, id: u32, name: String) {
        self.conn.exec_drop(
            "INSERT INTO Customer VALUES (:name, :id);",
            params! {
                "name" => name,
                "id" => id
            },
        );
    }

    pub fn cancel_flight(&mut self, user_id: u32, res_id: String) -> bool {
        self.user_exist(user_id);

        let res = self.conn.exec_first(
            "SELECT count(*) \
                FROM Reservation \
                WHERE customer_id = :user_id \
                  AND res_type = 1 \
                  AND res_id = :res_id",
            params! {
                "user_id" => user_id,
                "res_id" => &res_id
            },
        );

        if res.unwrap() == Some(0) {
            eprintln!("未预约对应的航班");
            return false;
        }

        self.conn.exec_drop(
            "DELETE FROM BookingSystem.Reservation \
                WHERE res_id = :res_id",
            params! {
                "res_id" => res_id
            },
        );

        return true;
    }

    pub fn cancel_bus(&mut self, user_id: u32, res_id: String) -> bool {
        self.user_exist(user_id);

        let res = self.conn.exec_first(
            "SELECT count(*) \
                FROM Reservation \
                WHERE customer_id = :user_id \
                  AND res_type = 3 \
                  AND res_id = :res_id",
            params! {
                "user_id" => user_id,
                "res_id" => &res_id
            },
        );

        if res.unwrap() == Some(0) {
            eprintln!("未预约对应的大巴");
            return false;
        }

        self.conn.exec_drop(
            "DELETE FROM BookingSystem.Reservation \
                WHERE res_id = :res_id",
            params! {
                "res_id" => res_id
            },
        );

        return true;
    }

    pub fn cancel_hotel(&mut self, user_id: u32, res_id: String) -> bool {
        self.user_exist(user_id);

        let res = self.conn.exec_first(
            "SELECT count(*) \
                FROM Reservation \
                WHERE customer_id = :user_id \
                  AND res_type = 2 \
                  AND res_id = :res_id",
            params! {
                "user_id" => user_id,
                "res_id" => &res_id
            },
        );

        if res.unwrap() == Some(0) {
            eprintln!("未预约对应的酒店");
            return false;
        }

        self.conn.exec_drop(
            "DELETE FROM BookingSystem.Reservation \
                WHERE hotel_num = ':res_id'",
            params! {
                "res_id" => res_id
            },
        );

        return true;
    }
}
