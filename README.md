# MxxShop商城

#### 介绍
MxxShop Rust版，是基于actix-web和rbatis的商城管理系统，一款面向全球的多语言、多业态电子商务平台，系统提供管理、支付、物流、售后等众多应用功能，满足跨境市场的基本需 求，助力更多商户走出国门，大力开拓国外市场，为广大用户提供一站式的国际化电商运营解决方案

###### 后台管理UI
[https://gitee.com/rust-shop/MxxShop-Admin-UI.git](https://gitee.com/rust-shop/MxxShop-Admin-UI.git) 

###### 目标实现商城的功能说明
1. **统计**：产品销售统计、销售趋势、销售排行、销售分析、销售报表；
2. **商品**：商品列表、商品分类、商品品牌、商品属性、商品评论、商品SKU、商品库存、商品规格、商品规格值、商品规格图片、商品规格SKU、商品SKU库存、商品SKU规格值、商品SKU规格图片；
3. **订单**：订单列表、订单详情、订单发货、订单收货、订单退款、订单评价、订单统计、订单报表；
4. **用户**：用户列表、用户详情、用户收货地址、用户优惠券、用户积分、用户积分记录、用户积分明细、用户积分兑换、用户积分兑换记录、用户积分兑换明细、用户积分兑换记录、用户积分兑换明细
5. **内容**：文章列表、文章分类、文章标签、文章评论、文章评论回复、文章评论点赞、文章评论举报、文章评论审核、文章评论统计、文章评论报表；
6. **营销**：营销活动、优惠券、优惠券领取记录、优惠券使用记录、优惠券统计、优惠券报表；
7. **客服**：客服列表、客服详情、客服回复、客服统计、客服报表；
8. **财务**：财务账单、财务统计、财务报表；
9. **仓储**：仓储列表、仓储详情、仓储发货、仓储收货、仓储统计、仓储报表；
10. **物流**：物流公司列表、物流模板设置，物流详情、物流发货、物流收货、物流统计、物流报表；
11. **设置**：系统设置、管理员设置、角色设置、菜单设置、语言设置、系统日志、系统监控、系统定时任务、系统定时任务日志、系统定时任务日志详情；

### 软件架构
前台为了SEO采用tera模板引擎渲染，管理后台采用Vue+ElementUI

##### 后台技术选型

| 说明      | 框架        | 说明   |                  |
|---------|-----------|------|------------------|
| 基础框架    | actix_web | 模板引擎 | tera             |
| 持久框架    | Rbatis    | 程序构建 | Cargo            |
| 关系型数据库  | MySQL     | 权限验证 | actix-web-grants | 
| 缓存      | Redis     | 搜索查询 | Elasticsearch    |
| 负载均衡    | Nginx     | 定时任务 | job_scheduler    |
| 短信      | 阿里云短信     | 认证   | Jsonwebtoken     |
| 日志处理    | Log4rs    | 接口规范 | RESTful          |


#### 安装教程

1.  进入 docs/sql 目录 ， 根据 MySQL 版本选择对应的脚本；
先执行 mxx_shop_db.sql 完成数据库的创建；
2.  xxxx

#### 使用说明

1.  xxxx
2.  xxxx
3.  xxxx

##### 部分UI演示
1. 首页
![](/docs/img/index.jpg)
2. 后台菜单管理
![](/docs/img/menus.jpg)
3. 


#### 提交反馈

1. MxxShop官网：[https://www.mxxshop.com](https://www.mxxshop.com)


