table! {
    t_account (account_id) {
        account_id -> Int8,
        name -> Varchar,
        description -> Varchar,
        is_deleted -> Bool,
        date_created -> Timestamptz,
        date_modified -> Timestamptz,
    }
}

/*table! {
    t_account_hist (account_hist_id) {
        account_hist_id -> Int8,
        account_id -> Int8,
        name -> Varchar,
        description -> Varchar,
        is_deleted -> Bool,
        date_created -> Timestamptz,
        date_modified -> Timestamptz,
        date_hist_created -> Timestamptz,
        date_hist_modified -> Timestamptz,
    }
}

table! {
    t_currency (currency_id) {
        currency_id -> Int8,
        code -> Varchar,
        description -> Varchar,
        is_deleted -> Bool,
        date_created -> Timestamptz,
        date_modified -> Timestamptz,
    }
}

table! {
    t_currency_exchange (currency_exchange_id) {
        currency_exchange_id -> Int4,
        currency_from_id -> Int4,
        currency_to_id -> Int4,
        exchange_rate -> Numeric,
        date_created -> Timestamptz,
        date_modified -> Timestamptz,
    }
}

table! {
    t_currency_exchange_hist (currency_exchange_hist_id) {
        currency_exchange_hist_id -> Int8,
        currency_exchange_id -> Int8,
        currency_from_id -> Int8,
        currency_to_id -> Int8,
        exchange_rate -> Numeric,
        date_created -> Timestamptz,
        date_modified -> Timestamptz,
        date_hist_created -> Timestamptz,
        date_hist_modified -> Timestamptz,
    }
}

table! {
    t_market (market_id) {
        market_id -> Int8,
        code -> Varchar,
        name -> Varchar,
        country -> Bpchar,
        is_deleted -> Bool,
        date_created -> Timestamptz,
        date_modified -> Timestamptz,
    }
}

table! {
    t_market_hist (market_hist_id) {
        market_hist_id -> Int8,
        market_id -> Int8,
        code -> Varchar,
        name -> Varchar,
        country -> Bpchar,
        is_deleted -> Int4,
        date_created -> Timestamptz,
        date_modified -> Timestamptz,
        date_hist_created -> Timestamptz,
        date_hist_modified -> Timestamptz,
    }
}

table! {
    t_parameter (parameter_id) {
        parameter_id -> Int8,
        name -> Varchar,
        value -> Varchar,
        value_default -> Varchar,
        description -> Varchar,
        datatype -> Varchar,
        is_deleted -> Bool,
        date_created -> Timestamptz,
        date_modified -> Timestamptz,
    }
}

table! {
    t_parameter_datatype (parameter_datatype_id) {
        parameter_datatype_id -> Int8,
        datatype -> Varchar,
        is_deleted -> Bool,
        date_created -> Timestamptz,
        date_modified -> Timestamptz,
    }
}

table! {
    t_parameter_hist (parameter_hist_id) {
        parameter_hist_id -> Int8,
        parameter_id -> Int8,
        name -> Varchar,
        value -> Varchar,
        value_default -> Varchar,
        description -> Varchar,
        datatype -> Varchar,
        is_deleted -> Bool,
        date_created -> Timestamptz,
        date_modified -> Timestamptz,
        date_hist_created -> Timestamptz,
        date_hist_modified -> Timestamptz,
    }
}

table! {
    t_pool (pool_id) {
        pool_id -> Int8,
        account_id -> Int8,
        total -> Numeric,
        invested -> Numeric,
        cash -> Numeric,
        is_manually_added -> Bool,
        is_deleted -> Bool,
        date_created -> Timestamptz,
        date_modified -> Timestamptz,
    }
}

table! {
    t_pool_hist (pool_hist_id) {
        pool_hist_id -> Int8,
        pool_id -> Int8,
        account_id -> Int8,
        total -> Numeric,
        invested -> Numeric,
        cash -> Numeric,
        is_manually_added -> Bool,
        is_deleted -> Bool,
        date_created -> Timestamptz,
        date_modified -> Timestamptz,
        date_hist_created -> Timestamptz,
        date_hist_modified -> Timestamptz,
    }
}

table! {
    t_product (product_id) {
        product_id -> Int8,
        name -> Varchar,
        description -> Varchar,
        product_type_id -> Int8,
        currency_id -> Int8,
        market_id -> Int8,
        product_tick_info_id -> Nullable<Int8>,
        is_deleted -> Bool,
        date_created -> Timestamptz,
        date_modified -> Timestamptz,
    }
}

table! {
    t_product_hist (product_hist_id) {
        product_hist_id -> Int8,
        product_id -> Int8,
        name -> Varchar,
        description -> Varchar,
        product_type_id -> Int8,
        currency_id -> Int8,
        market_id -> Int8,
        product_tick_info_id -> Int8,
        is_deleted -> Bool,
        date_created -> Timestamptz,
        date_modified -> Timestamptz,
        date_hist_created -> Timestamptz,
        date_hist_modified -> Timestamptz,
    }
}

table! {
    t_product_tick_info (product_tick_info_id) {
        product_tick_info_id -> Int8,
        description -> Varchar,
        tick -> Numeric,
        tick_value -> Numeric,
        order_min -> Numeric,
        order_max -> Numeric,
        margin_day_proc -> Numeric,
        margin_night_proc -> Numeric,
        is_deleted -> Bool,
        date_created -> Timestamptz,
        date_modified -> Timestamptz,
    }
}

table! {
    t_product_tick_info_hist (product_tick_info_hist_id) {
        product_tick_info_hist_id -> Int8,
        product_tick_info_id -> Int8,
        description -> Varchar,
        tick -> Numeric,
        tick_value -> Numeric,
        order_min -> Numeric,
        order_max -> Numeric,
        margin_day_proc -> Numeric,
        margin_night_proc -> Numeric,
        is_deleted -> Bool,
        date_created -> Timestamptz,
        date_modified -> Timestamptz,
        date_hist_created -> Timestamptz,
        date_hist_modified -> Timestamptz,
    }
}

table! {
    t_product_type (product_type_id) {
        product_type_id -> Int8,
        name -> Varchar,
        description -> Varchar,
        is_deleted -> Bool,
        date_created -> Timestamptz,
        date_modified -> Timestamptz,
    }
}*/

table! {
    t_trade (trade_id) {
        trade_id -> Int8,
        trade_calculated_id -> Int8,
        product_id -> Int8,
        pool_id -> Int8,
        trade_cost_id -> Int8,
        trade_drawdown_id -> Int8,
        date_buy -> Nullable<Timestamptz>,
        year_buy -> Nullable<Int4>,
        month_buy -> Nullable<Int4>,
        day_buy -> Nullable<Int4>,
        date_sell -> Nullable<Timestamptz>,
        year_sell -> Nullable<Int4>,
        month_sell -> Nullable<Int4>,
        day_sell -> Nullable<Int4>,
        shares_buy -> Int4,
        shares_sell -> Int4,
        price_buy -> Numeric,
        price_sell -> Numeric,
        commission_buy -> Numeric,
        commission_sell -> Numeric,
        tax_buy -> Numeric,
        tax_sell -> Numeric,
        is_deleted -> Bool,
        date_created -> Timestamptz,
        date_modified -> Timestamptz,
    }
}

table! {
    t_trade_calculated (trade_calculated_id) {
        trade_calculated_id -> Int8,
        amount_buy -> Numeric,
        amount_sell -> Numeric,
        amount_buy_total -> Numeric,
        amount_sell_total -> Numeric,
        risk_input -> Numeric,
        risk_initial -> Numeric,
        risk_initial_percent -> Numeric,
        risk_actual -> Numeric,
        risk_actual_percent -> Numeric,
        cost_total -> Numeric,
        cost_other -> Numeric,
        stoploss -> Numeric,
        stoploss_orig -> Numeric,
        profit_loss -> Numeric,
        profit_loss_orig -> Numeric,
        profit_loss_total -> Numeric,
        profit_loss_total_percent -> Numeric,
        r_multiple -> Numeric,
        is_deleted -> Bool,
        date_created -> Timestamptz,
        date_modified -> Timestamptz,
    }
}

/*table! {
    t_trade_calculated_hist (trade_calculated_hist_id) {
        trade_calculated_hist_id -> Int8,
        trade_calculated_id -> Int8,
        amount_buy -> Numeric,
        amount_sell -> Numeric,
        amount_buy_total -> Numeric,
        amount_sell_total -> Numeric,
        risk_input -> Numeric,
        risk_initial -> Numeric,
        risk_initial_percent -> Numeric,
        risk_actual -> Numeric,
        risk_actual_percent -> Numeric,
        cost_total -> Numeric,
        cost_other -> Numeric,
        stoploss -> Numeric,
        stoploss_orig -> Numeric,
        profit_loss -> Numeric,
        profit_loss_orig -> Numeric,
        profit_loss_total -> Numeric,
        profit_loss_total_percent -> Numeric,
        r_multiple -> Numeric,
        is_deleted -> Bool,
        date_created -> Timestamptz,
        date_modified -> Timestamptz,
        date_hist_created -> Timestamptz,
        date_hist_modified -> Timestamptz,
    }
}

table! {
    t_trade_cost (trade_cost_id) {
        trade_cost_id -> Int8,
        commission -> Numeric,
        tax -> Numeric,
        is_manually_added -> Bool,
        is_deleted -> Bool,
        date_created -> Timestamptz,
        date_modified -> Timestamptz,
    }
}

table! {
    t_trade_drawdown (trade_drawdown_id) {
        trade_drawdown_id -> Int8,
        drawdown_current -> Int4,
        drawdown_max -> Int4,
        is_deleted -> Bool,
        date_created -> Timestamptz,
        date_modified -> Timestamptz,
    }
}

table! {
    t_trade_drawdown_hist (trade_drawdown_id) {
        trade_drawdown_hist_id -> Int8,
        trade_drawdown_id -> Int8,
        drawdown_current -> Int4,
        drawdown_max -> Int4,
        is_deleted -> Bool,
        date_created -> Timestamptz,
        date_modified -> Timestamptz,
        date_hist_created -> Timestamptz,
        date_hist_modified -> Timestamptz,
    }
}

table! {
    t_trade_hist (trade_hist_id) {
        trade_hist_id -> Int8,
        trade_id -> Int8,
        trade_calculated_id -> Int8,
        product_id -> Int8,
        pool_id -> Int8,
        trade_cost_id -> Int8,
        drawdown_id -> Int8,
        date_buy -> Nullable<Timestamptz>,
        year_buy -> Nullable<Int4>,
        month_buy -> Nullable<Int4>,
        day_buy -> Nullable<Int4>,
        date_sell -> Nullable<Timestamptz>,
        year_sell -> Nullable<Int4>,
        month_sell -> Nullable<Int4>,
        day_sell -> Nullable<Int4>,
        shares_buy -> Int4,
        shares_sell -> Int4,
        price_buy -> Numeric,
        price_sell -> Numeric,
        commission_buy -> Numeric,
        commission_sell -> Numeric,
        tax_buy -> Numeric,
        tax_sell -> Numeric,
        is_deleted -> Bool,
        date_created -> Timestamptz,
        date_modified -> Timestamptz,
        date_hist_created -> Timestamptz,
        date_hist_modified -> Timestamptz,
    }
}*/

table! {
    t_version (version_id) {
        version_id -> Int4,
        database_version -> Varchar,
        database_version_info -> Varchar,
        application_version -> Varchar,
        application_version_info -> Varchar,
        date_created -> Timestamptz,
        date_modified -> Timestamptz,
    }
}

/*joinable!(t_account_hist -> t_account (account_id));
joinable!(t_currency_exchange_hist -> t_currency_exchange (currency_exchange_id));
joinable!(t_market_hist -> t_market (market_id));
joinable!(t_parameter_hist -> t_parameter (parameter_id));
joinable!(t_product -> t_currency (currency_id));
joinable!(t_product -> t_market (market_id));
joinable!(t_product -> t_product_tick_info (product_tick_info_id));
joinable!(t_product -> t_product_type (product_type_id));
joinable!(t_product_hist -> t_product (product_id));
joinable!(t_product_tick_info_hist -> t_product_tick_info (product_tick_info_id));
joinable!(t_trade_calculated_hist -> t_trade_calculated (trade_calculated_id));
joinable!(t_trade_drawdown_hist -> t_trade_drawdown (trade_drawdown_id));
joinable!(t_trade_hist -> t_trade (trade_id));

allow_tables_to_appear_in_same_query!(
    t_account,
    t_account_hist,
    t_currency,
    t_currency_exchange,
    t_currency_exchange_hist,
    t_market,
    t_market_hist,
    t_parameter,
    t_parameter_datatype,
    t_parameter_hist,
    t_pool,
    t_pool_hist,
    t_product,
    t_product_hist,
    t_product_tick_info,
    t_product_tick_info_hist,
    t_product_type,
    t_trade,
    t_trade_calculated,
    t_trade_calculated_hist,
    t_trade_cost,
    t_trade_drawdown,
    t_trade_drawdown_hist,
    t_trade_hist,
    t_version,
);*/
