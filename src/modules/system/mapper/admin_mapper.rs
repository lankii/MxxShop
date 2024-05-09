//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use rbatis::{impl_select, impl_select_page};

use crate::modules::system::entity::admin_entity::SystemAdmin;
use crate::modules::system::entity::admin_model::UserListRequest;
use crate::modules::system::entity::admin_role_entity::SystemAdminRole;

rbatis::crud!(SystemAdmin {}, "fly_system_admin");



rbatis::pysql_select_page!(select_user_page(item: &UserListRequest) -> SystemAdmin =>
r#"`select `
    if do_count == true:
     ` count(1) as count `
    if do_count == false:
     ` u.* `
    ` from fly_system_admin u`
    ` left join fly_system_dept d on u.dept_id = d.id`
    ` where u.del_flag = 0`
    if item.admin_id != '' && item.admin_id != null:
     ` AND u.id = #{item.admin_id}`
    if item.user_name != '' && item.user_name != null:
     ` AND u.user_name like concat('%', #{item.user_name}, '%')`
    if item.status != '' && item.status != null:
     ` AND u.status = #{item.status}`
    if item.mobile != '' && item.mobile != null:
     ` AND u.mobile = #{item.mobile}`
    if item.begin_time != '' && item.begin_time != null:
     ` AND date_format(u.create_time,"%y%m%d") >= date_format(#{item.begin_time},"%y%m%d")`
    if item.end_time != '' && item.end_time != null:
     ` AND date_format(u.create_time,"%y%m%d") <= date_format(#{item.end_time},"%y%m%d")`
    if item.dept_id != '' && item.dept_id != null:
     ` AND (u.dept_id = #{item.dept_id} OR u.dept_id IN ( SELECT t.id FROM fly_system_dept t WHERE FIND_IN_SET (#{item.dept_id},ancestors) ))`
    if !sql.contains('count'):
     ` order by u.id desc`
    if do_count == false:
     ` limit #{page_no}, #{page_size}`
    "#);

//
impl_select_page!(SystemAdmin{select_page_by_name(user_name:&str,status:&str) =>"
      where 1=1
     if user_name != null && user_name != '':
       ` and user_name = #{user_name} `
     if status != null && status != '':
       ` and status = #{status} `
     if !sql.contains('count'):
        ` order by id desc `"},"fly_system_admin");


impl_select!(SystemAdmin{select_by_username(username: Option<String>) -> Option => "`where user_name = #{username} limit 1`"},"fly_system_admin");

rbatis::crud!(SystemAdminRole {}, "fly_system_admin_role");

impl_select_page!(SystemAdminRole{select_page() =>"
     if !sql.contains('count'):
       order by create_time desc"},"fly_system_admin_role");

impl_select_page!(SystemAdminRole{select_page_by_name(name:&str) =>"
     if name != null && name != '':
       where user_name != #{name}
     if name == '':
       where user_name != ''"},"fly_system_admin_role");


