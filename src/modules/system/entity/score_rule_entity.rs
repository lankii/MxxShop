use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScoreRule {
    ///ID
    pub id: u64,
    ///规则名称
    pub name: String,
    ///变化积分
    pub score: i64,
    ///说明
    pub remark: String,
    ///奖励次数类型，day每天一次，week每周一次，month每月一次，year每年一次，one只有一次，unlimite不限次数
    pub r#type: String,
    ///规则状态
    pub status: i8,
    ///创建时间
    pub create_time: Option<DateTime>,
    ///更新时间
    pub update_time: Option<DateTime>,
}