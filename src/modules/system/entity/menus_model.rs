//!
//! Copyright (c) 2024-2999 北京心月狐科技有限公司 All rights reserved.
//!
//! https://www.mxxshop.com
//!
//! Licensed 并不是自由软件，未经许可不能去掉 MxxShop 相关版权
//!
//! 版权所有，侵权必究！
//!

use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

use crate::modules::system::entity::menus_entity::SystemMenu;
use crate::modules::system::entity::role_entity::SystemRole;
use crate::utils::string_utils::{deserialize_string_to_i8,
                                 serialize_option_i8_to_string,
                                 deserialize_string_to_u64,
                                 serialize_option_u64_to_string
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MenuSaveRequest {
    /// 菜单名称
    #[serde(rename = "menuName")]
    pub menu_name: Option<String>,
    /// 父菜单ID
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub parent_id: Option<u64>,
    /// 组件名称
    pub name: Option<String>,
    ///路由地址
    pub path: Option<String>,
    ///权限组ID
    pub roles: Vec<u64>,
    ///请求api地址
    pub api_url: Option<String>,
    /// 显示顺序
    pub sort: Option<i32>,
    ///组件路径
    pub component: Option<String>,
    ///是否为外链（0是 1否）
    pub is_link: Option<i8>,
    ///是否缓存（0缓存 1不缓存）
    pub is_keep_alive: Option<i8>,
    ///外链/内嵌时链接地址（http:xxx.com），开启外链条件，`1、isFrame:true 2、链接地址不为空`
    pub link_url: Option<String>,
    ///菜单高亮
    pub active: Option<String>,
    ///菜单类型（M目录 C菜单 F按钮）
    pub menu_type: Option<String>,
    ///是否隐藏（0显示 1隐藏）
    #[serde(deserialize_with = "deserialize_string_to_i8")]
    pub is_hide: Option<i8>,
    ///权限标识
    pub perms: Option<String>,
    ///菜单图标
    pub icon: Option<String>,
    ///是否固定，类似首页控制台在标签中是没有关闭按钮的
    pub is_affix: Option<i8>,
    ///颜色值
    pub color: Option<String>,
    ///是否内嵌，开启条件，`1、isIframe:true 2、链接地址不为空`
    pub is_iframe: Option<i8>,
    ///备注
    pub remark: Option<String>,
    ///状态（0正常 1停用）
    pub status: Option<i8>,
}

impl From<MenuSaveRequest> for SystemMenu {
    fn from(arg: MenuSaveRequest) -> Self {
        Self {
            id: None,
            menu_name: arg.menu_name,
            parent_id: arg.parent_id,
            name: arg.name,
            path: arg.path,
            api_url: arg.api_url,
            sort: arg.sort,
            component: arg.component,
            is_link: arg.is_link,
            is_keep_alive: arg.is_keep_alive,
            link_url: arg.link_url,
            active: arg.active,
            menu_type: arg.menu_type,
            is_hide: arg.is_hide,
            perms: arg.perms,
            icon: arg.icon,
            is_affix: arg.is_affix,
            color: arg.color,
            is_iframe: arg.is_iframe,
            create_by: None,
            create_time: Option::from(DateTime::now()),
            update_by: None,
            update_time: Option::from(DateTime::now()),
            remark: arg.remark,
            status: arg.status,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MenuUpdateRequest {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: Option<u64>,
    /// 菜单名称
    #[serde(rename = "menuName")]
    pub menu_name: Option<String>,
    /// 父菜单ID
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub parent_id: Option<u64>,
    /// 组件名称
    pub name: Option<String>,
    ///路由地址
    pub path: Option<String>,
    ///权限组ID
    pub roles: Vec<u64>,
    ///请求api地址
    pub api_url: Option<String>,
    /// 显示顺序
    pub sort: Option<i32>,
    ///组件路径
    pub component: Option<String>,
    ///是否为外链（0是 1否）
    #[serde(rename = "isLink")]
    pub is_link: Option<i8>,
    ///是否缓存（0缓存 1不缓存）
    pub is_keep_alive: Option<i8>,
    ///外链/内嵌时链接地址（http:xxx.com），开启外链条件，`1、isFrame:true 2、链接地址不为空`
    pub link_url: Option<String>,
    ///菜单高亮
    pub active: Option<String>,
    ///菜单类型（M目录 C菜单 F按钮）
    pub menu_type: Option<String>,
    ///是否隐藏（0显示 1隐藏）
    #[serde(deserialize_with = "deserialize_string_to_i8")]
    pub is_hide: Option<i8>,
    ///权限标识
    pub perms: Option<String>,
    ///菜单图标
    pub icon: Option<String>,
    ///是否固定，类似首页控制台在标签中是没有关闭按钮的
    pub is_affix: Option<i8>,
    ///颜色值
    pub color: Option<String>,
    ///是否内嵌，开启条件，`1、isIframe:true 2、链接地址不为空`
    pub is_iframe: Option<i8>,
    ///是否整页打开路由（脱离框架系），例如：fullpage: true
    pub fullpage: Option<i8>,
    ///更新者
    pub update_by: Option<String>,
    ///更新时间
    pub update_time: Option<DateTime>,
    ///备注
    pub remark: Option<String>,
    ///状态（0正常 1停用）
    pub status: Option<i8>,
}

impl From<MenuUpdateRequest> for SystemMenu {
    fn from(arg: MenuUpdateRequest) -> Self {
        Self {
            id: arg.id,
            menu_name: arg.menu_name,
            parent_id: arg.parent_id,
            name: arg.name,
            path: arg.path,
            api_url: arg.api_url,
            sort: arg.sort,
            component: arg.component,
            is_link: arg.is_link,
            is_keep_alive: arg.is_keep_alive,
            link_url: arg.link_url,
            active: arg.active,
            menu_type: arg.menu_type,
            is_hide: arg.is_hide,
            perms: arg.perms,
            icon: arg.icon,
            is_affix: arg.is_affix,
            color: arg.color,
            is_iframe: arg.is_iframe,
            create_by: None,
            create_time: None,
            update_by: arg.update_by,
            update_time: arg.update_time,
            remark: arg.remark,
            status: arg.status,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Router {
    #[serde(serialize_with = "serialize_option_u64_to_string")]
    pub id: Option<u64>,
    pub parent_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Router>>,
    pub component: Option<String>,
    pub meta: Meta,
    pub name: Option<String>,
    pub path: Option<String>,
    pub sort: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub title: Option<String>,
    ///是否固定，类似首页控制台在标签中是没有关闭按钮的
    pub is_affix: bool,
    ///是否内嵌，开启条件，`1、isIframe:true 2、链接地址不为空`
    pub is_iframe: bool,
    ///是否隐藏（0显示 1隐藏）
    pub is_hide: bool,
    ///是否缓存（0缓存 1不缓存）
    pub is_keep_alive: bool,
    ///菜单图标
    pub icon: Option<String>,
    ///是否为外链（0是 1否）
    pub is_link: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SystemMenuPage {
    pub id: Option<u64>,
    pub menu_name: Option<String>,
    pub parent_id: Option<u64>,
    pub order_num: Option<i8>,
    pub path: Option<String>,
    pub component: Option<String>,
    pub query: Option<String>,
    pub is_frame: Option<i8>,
    pub is_cache: Option<i8>,
    pub menu_type: Option<String>,
    pub visible: Option<String>,
    pub status: Option<String>,
    pub perms: Option<String>,
    pub icon: Option<String>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct MenuTree {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<MenuTree>>,
    pub id: Option<u64>,
    pub label: Option<String>,
}

/// 用户查询所在的角色和菜单路由
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct RoleMenuRoutes {
    pub permissions: Vec<String>,
    pub menu_list: Vec<Router>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SystemMenuResponse {
    #[serde(serialize_with = "serialize_option_u64_to_string")]
    pub id: Option<u64>,
    /// 菜单名称
    pub menu_name: Option<String>,
    /// 父菜单ID
    #[serde(serialize_with = "serialize_option_u64_to_string")]
    pub parent_id: Option<u64>,
    /// 组件名称
    pub name: Option<String>,
    ///路由地址
    pub path: Option<String>,
    ///请求api地址
    pub api_url: Option<String>,
    /// 显示顺序
    pub sort: Option<i32>,
    ///组件路径
    pub component: Option<String>,
    ///是否为外链（0是 1否）
    pub is_link: Option<i8>,
    ///是否缓存（0缓存 1不缓存）
    pub is_keep_alive: Option<i8>,
    ///外链/内嵌时链接地址（http:xxx.com），开启外链条件，`1、isFrame:true 2、链接地址不为空`
    pub link_url: Option<String>,
    ///菜单高亮
    pub active: Option<String>,
    ///菜单类型（M目录 C菜单 F按钮）
    pub menu_type: Option<String>,
    ///是否隐藏（0显示 1隐藏）
    #[serde(serialize_with = "serialize_option_i8_to_string")]
    pub is_hide: Option<i8>,
    ///权限标识
    pub perms: Option<String>,
    ///菜单图标
    pub icon: Option<String>,
    ///是否固定，类似首页控制台在标签中是没有关闭按钮的
    pub is_affix: Option<i8>,
    ///颜色值
    pub color: Option<String>,
    ///是否内嵌，开启条件，`1、isIframe:true 2、链接地址不为空`
    pub is_iframe: Option<i8>,
    ///备注
    pub remark: Option<String>,
    ///状态（0正常 1停用）
    pub status: Option<i8>,
}

impl From<SystemMenu> for SystemMenuResponse {
    fn from(arg: SystemMenu) -> Self {
        Self {
            id: arg.id,
            menu_name: arg.menu_name,
            parent_id: arg.parent_id,
            name: arg.name,
            path: arg.path,
            api_url: arg.api_url,
            sort: arg.sort,
            component: arg.component,
            is_link: arg.is_link,
            is_keep_alive: arg.is_keep_alive,
            link_url: arg.link_url,
            active: arg.active,
            menu_type: arg.menu_type,
            is_hide: arg.is_hide,
            perms: arg.perms,
            icon: arg.icon,
            is_affix: arg.is_affix,
            color: arg.color,
            is_iframe: arg.is_iframe,
            remark: arg.remark,
            status: arg.status,
        }
    }
}

///角色List和的菜单List
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct RoleMenuResponse {
    pub roles: Vec<SystemRole>,
    pub menus: Vec<SystemMenuResponse>,
}

/// 当前菜单和绑定的所有角色
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct MenuAndRoleResponse {
    pub role_ids: Vec<u64>,
    pub menu: SystemMenuResponse,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct MenuListData {
    #[serde(serialize_with = "serialize_option_u64_to_string")]
    pub id: Option<u64>,
    pub menu_name: Option<String>,
    #[serde(serialize_with = "serialize_option_u64_to_string")]
    pub parent_id: Option<u64>,
    pub name: Option<String>,
    pub path: Option<String>,
    pub sort: Option<i32>,
    pub component: Option<String>,
    pub is_link: Option<i8>,
    pub is_keep_alive: Option<i8>,
    pub active: Option<String>,
    pub menu_type: Option<String>,
    ///是否隐藏（0显示 1隐藏）
    pub is_hide: Option<i8>,
    pub link_url: Option<String>,
    pub perms: Option<String>,
    pub icon: Option<String>,
    pub is_affix: Option<i8>,
    pub color: Option<String>,
    ///是否内嵌，开启条件，`1、isIframe:true 2、链接地址不为空`
    pub is_iframe: Option<i8>,
    pub remark: Option<String>,
    pub status: Option<i8>,
    pub children: Option<Vec<MenuListData>>,
}

#[derive(Debug, Deserialize)]
pub struct MenuDeleteRequest {
    pub id: Option<u64>,
}

/// 更新菜单和角色关联信息
#[derive(Debug, Deserialize)]
pub struct UpdateRoleMenuRequest {
    pub menu_id: Option<u64>,
    pub role_ids: Vec<u64>,
}

