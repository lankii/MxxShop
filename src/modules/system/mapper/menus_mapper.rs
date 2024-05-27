//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use rbatis::{htmlsql, impl_select, impl_select_page, py_sql, RBatis};
use rbatis::rbatis_codegen::IntoSql;

use crate::modules::system::entity::menu_entity::SystemMenu;
use crate::modules::system::entity::role_menu_entity::RoleMenu;

impl_select!(RoleMenu{select_by_id(id:i32) -> Option => "`where id = #{id} limit 1`"},"mxx_system_menus");

//增删改查菜单
rbatis::crud!(SystemMenu {}, "mxx_system_menus");


impl_select_page!(SystemMenu{select_page_by_name(name:&str) =>"
     if name != null && name != '':
       where user_name != #{name}
     if !sql.contains('count'):
        where user_name != ''"});


/// 查询所有的菜单数据
#[py_sql("
    `select * from mxx_system_menus order by sort asc`
")]
pub async fn select_all_list(rb: &RBatis) -> rbatis::Result<Vec<SystemMenu>> {
    impled!()
}

impl_select!(SystemMenu{select_by_id(id:i32) -> Option => "`where id = #{id} limit 1`"});

impl_select!(SystemMenu{select_by_ids(ids:&[i32]) -> Vec => "`where status = 1 and id in ${ids.sql()} order by sort asc`"});

htmlsql!(select_menu_by_admin_id(rb: &RBatis, is_admin: &bool, admin_id: &Option<u64>) -> rbatis::Result<Vec<SystemMenu>> =>
r#"<mapper>
<select id="select_menu_by_admin_id">
        `select distinct m.id, m.parent_id, m.menu_name, m.name, m.path, m.component, m.active, m.menu_type, m.is_hide, ifnull(m.perms,'') as perms, m.icon, m.is_link, m.is_keep_alive, m.is_affix, m.color, m.is_iframe, m.sort, m.create_by, m.create_time, m.update_by, m.update_time, m.remark, m.status
        from mxx_system_menus m `
        `<if test="is_admin != true">
            left join mxx_system_role_menus rm on m.id = rm.menu_id
            left join mxx_system_admin_role ur on rm.role_id = ur.role_id
            left join mxx_system_role ro on ur.role_id = ro.id
            where ur.admin_id = #{admin_id}
        </if>`
       ` order by m.sort asc`
    </select>
</mapper>"#);
