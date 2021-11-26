use crate::{read, MySQLConnection, ToRow};
use prettytable::{format, table, Row, Table};
use std::io::{stdin, stdout, Stdout, Write};

pub(crate) struct View;

impl View {
    pub fn login_menu() {
        println!("------------- TouringBookingSystem -------------");
        print!("请输入您的ID(不存在的id将会进行注册):");
        stdout().flush();
    }

    pub fn sign_up(id: u32) {
        print!("账号[{}]不存在，是否要注册账号?(y/n)", id);
        stdout().flush();
    }

    pub fn new_name() {
        print!("请输入对应的账号名：");
        stdout().flush();
    }

    pub fn user_menu() {
        let mut table = table!(
            ["1. 预定航班/大巴车/宾馆房间"],
            ["2. 取消预定航班/大巴车/宾馆房间"],
            ["3. 查询航班/大巴车/宾馆房间/客户和预订信息"],
            ["4. 查询旅行线路"],
            ["5. 检查预定线路的完整性"],
            ["0. 退出当前用户"]
        );
        table.set_titles(row!["TouringBookingSystem"]);
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        table.printstd();
        print!("$> 选择服务：");
        stdout().flush();
    }

    pub fn booking_menu() {
        let mut table = table!(
            ["请输入您要预定的类型："],
            ["1. 预定航班"],
            ["2. 预定大巴车"],
            ["3. 预定宾馆房间"],
            ["4. 返回上一级"]
        );
        table.set_titles(row!["预定服务"]);
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        table.printstd();
        print!("$> 选择类型：");
        stdout().flush();
    }

    pub fn cancel_menu() {
        let mut table = table!(
            ["请输入您要取消预定的类型："],
            ["1. 取消预定航班"],
            ["2. 取消预定大巴车"],
            ["3. 取消预定宾馆房间"],
            ["4. 返回上一级"]
        );
        table.set_titles(row!["取消预订"]);
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        table.printstd();
        print!("$> 选择类型：");
        stdout().flush();
    }

    pub fn query_menu() {
        let mut table = table!(
            ["请输入您要查询的类型："],
            ["1. 查询航班"],
            ["2. 查询大巴车"],
            ["3. 查询宾馆房间"],
            ["4. 返回上一级"]
        );
        table.set_titles(row!["查询服务"]);
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        table.printstd();
        print!("$> 选择类型：");
        stdout().flush();
    }

    pub fn booking<T: ToRow>(hint: &Row, output: &Vec<T>) {
        let mut table = Table::new();
        table.set_titles(hint.clone());
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        output.iter().for_each(|x| {
            table.add_row(x.to_row());
        });
        table.printstd();
        print!("$> 请选择想要预约的号码/地址：");
        stdout().flush();
    }

    pub fn querying<T: ToRow>(hint: &Row, output: &Vec<T>) {
        if output.len() == 0 {
            println!("<!>没有相关预定");
            return;
        }
        let mut table = Table::new();
        table.set_titles(hint.clone());
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        output.iter().for_each(|x| {
            table.add_row(x.to_row());
        });
        table.printstd();
    }

    pub fn success_hint() {
        println!("成功！");
    }

    pub fn cancel_hint() {
        print!("请输入需要取消的航班/大巴/酒店号：");
        stdout().flush();
    }

    pub fn travel_flight() {
        println!("航班相关预定：");
    }

    pub fn travel_bus() {
        println!("大巴相关预定：");
    }

    pub fn travel_hotel() {
        println!("酒店相关预定");
    }

    pub fn check_completeness(success: bool) -> bool {
        return if success {
            println!("--- 路线完整 ---");
            false
        } else {
            println!("路线不完整，注意<!>：");
            println!("1. 航班是否覆盖酒店和大巴所在的城市");
            println!("2. 是否在旅行过程中无法连贯乘坐全部的航班");
            false
        };
    }
}
