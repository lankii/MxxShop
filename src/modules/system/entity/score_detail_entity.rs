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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScoreDetail {
    ///ID
    pub id: u64,
    ///类型，0是普通积分增加，1是奖励，2是撤销奖励
    pub r#type: String,
    ///用户ID
    pub user_id: u64,
    ///变化积分
    pub score: i64,
    ///账户剩余积分
    pub balance: i64,
    ///积分变动说明
    pub remark: String,
    ///外键ID
    pub foreign_id: u64,
    ///积分规则ID
    pub score_rule_id: u64,
    ///积分变动时间
    pub create_time: Option<DateTime>,
    ///审核状态
    pub status: i8,
}