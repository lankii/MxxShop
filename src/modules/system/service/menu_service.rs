//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use actix_web::{HttpResponse};
use rbatis::rbdc::{DateTime};

use crate::core::errors::error::Error;
use crate::core::errors::error::Result;
use crate::core::web::response::ResVO;
use crate::modules::system::entity::menu_entity::SystemMenu;
use crate::modules::system::entity::menu_model::{MenuListData,
                                                 MenuSaveRequest,
                                                 MenuUpdateRequest,
                                                 Meta,
                                                 Router,
                                                 UpdateRoleMenuRequest};
use crate::modules::system::entity::role_menu_entity::SystemRoleMenu;
use crate::modules::system::mapper::menu_mapper;
use crate::RB;
use crate::utils::snowflake_id::generate_snowflake_id;

pub async fn add_menu(payload: MenuSaveRequest) -> HttpResponse {
    let mut menu_entity: SystemMenu = payload.clone().into();
    menu_entity.id = generate_snowflake_id();
    let result_menu = SystemMenu::insert(&RB.clone(), &menu_entity).await;
    match result_menu {
        Ok(v) => {
            let role_menu = UpdateRoleMenuRequest {
                menu_id: u64::from(v.last_insert_id),
                role_ids: payload.roles.clone(),
            };
            insert_batch(role_menu).await;
            return HttpResponse::Ok().json(ResVO::ok_with_data(v.rows_affected))
        }
        Err(err) => {
            return HttpResponse::Ok().json(ResVO::<String>::error_msg(err.to_string()));
        }
    }
}


pub async fn insert_batch(item: UpdateRoleMenuRequest) -> HttpResponse {
    let menu_id = item.menu_id.clone();
    let role_menu_result = SystemRoleMenu::delete_by_column(&RB.clone(), "menu_id", &menu_id).await;

    return match role_menu_result {
        Ok(_) => {
            let mut menu_role: Vec<SystemRoleMenu> = Vec::new();

            for id in &item.role_ids {
                let role_id = id.clone();
                menu_role.push(SystemRoleMenu {
                    id: Some(generate_snowflake_id()),
                    menu_id: Option::from(menu_id.clone()),
                    role_id: Option::from(role_id.clone()),
                    create_time: Some(DateTime::now()),
                    update_time: Some(DateTime::now()),
                    status: Option::from(1),
                })
            }

            let result = SystemRoleMenu::insert_batch(&RB.clone(), &menu_role, item.role_ids.len() as u64).await;

            HttpResponse::Ok().json(ResVO::<u64>::handle_result(Ok(result.unwrap_or_default().rows_affected)))
        }
        Err(err) => {
            HttpResponse::Ok().json(ResVO::<String>::error_msg(err.to_string()))
        }
    }
}

pub async fn delete_in_column(ids: Vec<Option<String>>) -> HttpResponse {
    //有下级的时候 不能直接删除
    let menus = SystemMenu::select_in_column(&RB.clone(), "parent_id", &ids)
        .await
        .unwrap_or_default();

    if menus.len() > 0 {
        return HttpResponse::Ok().json(ResVO::<String>::error_msg("有下级菜单,不能直接删除".to_string()));
    }

    let result = SystemMenu::delete_in_column(&RB.clone(), "id", &ids).await;
    HttpResponse::Ok().json(ResVO::<u64>::handle_result(Ok(result.unwrap_or_default().rows_affected)))
}

pub async fn update_menu(payload: MenuUpdateRequest) -> HttpResponse {
    let menu_entity: SystemMenu = payload.clone().into();
    let role_menu_result = SystemMenu::update_by_column(&RB.clone(), &menu_entity, "id").await;
    match role_menu_result {
        Ok(v) => {
            let role_menu = UpdateRoleMenuRequest {
                menu_id: payload.id.clone(),
                role_ids: payload.roles.clone(),
            };
            insert_batch(role_menu).await;
            return HttpResponse::Ok().json(ResVO::ok_with_data(v.rows_affected))
        }
        Err(err) => {
            return HttpResponse::Ok().json(ResVO::<String>::error_msg(err.to_string()));
        }
    }
}


// 路由数组转树
pub fn router_arr_to_tree(re_list: &mut Vec<Router>, ori_arr: Vec<SystemMenu>, pid: u64) {
    for (_, it) in ori_arr.iter().enumerate() {
        if pid == it.parent_id && !it.menu_type.eq("F") {
            let temp_meta = Meta {
                title: it.menu_name.clone(),
                is_affix: if it.is_affix == Some(1) { true } else { false },
                is_iframe: if it.is_iframe == 1 { true } else { false },
                is_hide: if it.is_hide == 0 { true } else { false },
                icon: it.icon.clone(),
                is_keep_alive: if it.is_keep_alive == Some(1) { true } else { false },
                is_link: if it.is_link == Some(1) { true } else { false },
            };

            let mut children = Vec::<Router>::new();
            router_arr_to_tree(&mut children, ori_arr.clone(), it.id);

            let temp_router = Router {
                id: it.id.clone(),
                parent_id: it.parent_id.clone(),
                children: (|| -> Option<Vec<Router>> {
                    if children.len() > 0 {
                        Some(children)
                    } else {
                        None
                    }
                })(),
                component: it.component.clone(),
                name: it.name.clone(),
                path: it.path.clone(),
                sort: it.sort.clone(),
                redirect: (|| -> Option<String> {
                    if it.is_iframe == 1 && it.menu_type.eq("M") {
                        Some(String::from("noRedirect"))
                    } else {
                        None
                    }
                })(),
                meta: temp_meta,
            };
            re_list.push(temp_router)
        }
    }
}

// 菜单数组转树
pub fn menu_arr_to_tree(re_list: &mut Vec<MenuListData>, ori_arr: Vec<SystemMenu>, pid: u64) {
    for (_, it) in ori_arr.iter().enumerate() {
        if pid == it.parent_id {
            let mut children = Vec::<MenuListData>::new();
            menu_arr_to_tree(&mut children, ori_arr.clone(), it.id);

            let temp_router = MenuListData {
                children: (|| -> Option<Vec<MenuListData>> {
                    if children.len() > 0 {
                        Some(children)
                    } else {
                        None
                    }
                })(),
                id: it.id,
                menu_name: it.menu_name.clone(),
                parent_id: it.parent_id,
                name: it.name.clone(),
                path: it.path.clone(),
                sort: it.sort,
                component: it.component.clone(),
                is_link: it.is_link,
                is_keep_alive: it.is_keep_alive,
                active: it.active.clone(),
                menu_type: it.menu_type.clone(),
                is_hide: it.is_hide.clone(),
                link_url: it.link_url.clone(),
                perms: it.perms.clone(),
                icon: it.icon.clone(),
                is_affix: it.is_affix.clone(),
                color: it.color.clone(),
                is_iframe: it.is_iframe.clone(),
                remark: it.remark.clone(),
                status: it.status,
            };
            re_list.push(temp_router)
        }
    }
}

pub async fn query_btn_menu(is_admin: &bool, id: &Option<u64>) -> Result<Vec<String>> {
    // 判断是不是超级管理员
    let mut btn_menu: Vec<String> = Vec::new();
    if is_admin.clone() {
        let data = SystemMenu::select_all(&RB.clone()).await;
        match data {
            Ok(v) => {
                //btn_menu.clear();
                for x in v {
                    if let Some(ref perms) = x.perms {
                        if !perms.is_empty() {
                            btn_menu.push(x.perms.unwrap_or_default());
                        }
                    }
                }
                Ok(btn_menu)
            }
            Err(_) => {
                log::error!("获取菜单出错");
                return Ok(btn_menu);
            }
        }

    } else {
        let btn_menu_map = menu_mapper::select_menu_by_admin_id(&RB.clone(), is_admin, id).await;

        match btn_menu_map {
            Ok(x) => {
                for x in x {
                    if let Some(ref perms) = x.perms {
                        if !perms.is_empty() {
                            btn_menu.push(x.perms.unwrap_or_default());
                        }
                    }
                }
                Ok(btn_menu)
            }
            Err(_) => {
                log::error!("获取菜单出错");
                return Ok(btn_menu);
            }
        }
    }
}

pub async fn get_router_tree(is_admin: &bool, id: &Option<u64>) -> rbatis::Result<Vec<Router>> {
    let list: Vec<SystemMenu> =
        menu_mapper::select_menu_by_admin_id(&RB.clone(), &is_admin, id).await?;
    let mut router_list = Vec::<Router>::new();
    router_arr_to_tree(&mut router_list, list, 0);
    Ok(router_list)
}

pub async fn all_menu_list_tree() -> rbatis::Result<Vec<MenuListData>> {
    let list: Vec<SystemMenu> = menu_mapper::select_all_list(&RB.clone()).await?;
    let mut router_list = Vec::<MenuListData>::new();
    menu_arr_to_tree(&mut router_list, list, 0);
    Ok(router_list)
}

///按id查询
pub async fn select_by_column(ids: Vec<String>) -> rbatis::Result<Vec<SystemMenu>> {
    let result = SystemMenu::select_in_column(&RB.clone(), "parent_id", &ids).await;
    return result;
}

///按id查询
pub async fn find_by_id(id: &str) -> rbatis::Result<Option<SystemMenu>> {
    let result = SystemMenu::select_by_column(&RB.clone(), "id", &id)
        .await?
        .into_iter()
        .next();
    return Ok(result);
}


pub async fn select_all() -> rbatis::Result<Vec<SystemMenu>> {
    // 查询所有菜单
    let menu_list = SystemMenu::select_all(&RB.clone()).await;
    return menu_list;
}