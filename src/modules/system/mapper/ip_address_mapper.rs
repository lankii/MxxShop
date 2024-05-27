//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use rbatis::{crud, impl_select_page, py_sql, RBatis};

use crate::modules::system::entity::ip_address_entity::IpAddress;
use crate::modules::system::entity::ip_address_model::IpAddressPageDTO;


crud!(IpAddress {}, "mxx_ip_address");

// 查询ip
#[py_sql("
    `select count(*) from mxx_ip_address where start_ip >= #{start_ip} and start_ip <= #{end_ip} ORDER BY start_ip asc`
")]
pub async fn select_ip_address_list(rb: &RBatis, start_ip: &Option<u32>, end_ip: &Option<u32>) -> rbatis::Result<Vec<IpAddress>> {
    impled!()
}


#[py_sql("
    `select * from mxx_ip_address where start_ip <= #{ip} and start_ip >= #{ip} ORDER BY id asc limit 1`
")]
pub async fn find_ip_address_by_ip(rb: &RBatis, ip: &Option<u32>) -> rbatis::Result<Vec<IpAddress>> {
    impled!()
}

// 分页查询
impl_select_page!(IpAddress{select_page(dto: &IpAddressPageDTO) =>"
    trim end=' where ':
      ` where `
      trim ' and ':
        if dto.start_ip != null && dto.start_ip != '':
          ` and start_ip = #{dto.start_ip} `
        if dto.end_ip != null && dto.end_ip != '':
          ` and end_ip = #{dto.end_ip} `
    if !sql.contains('count'):
      ` order by id desc `"},"mxx_ip_address");










