CREATE TABLE T_TRADE_CALCULATED
(
        trade_calculated_id bigserial not null,
        amount_buy decimal(18,6) not null,
        amount_sell decimal(18,6) not null,
        amount_buy_total decimal(18,6) not null,
        amount_sell_total decimal(18,6) not null,
        risk_input decimal(18,6) not null,
        risk_initial decimal(18,6) not null,
        risk_initial_percent decimal(18,6) not null,
        risk_actual decimal(18,6) not null,
        risk_actual_percent decimal(18,6) not null,
        cost_total decimal(18,6) not null,
        cost_other decimal(18,6) not null,
        stoploss decimal(18,6) not null,
        stoploss_orig decimal(18,6) not null,
        profit_loss decimal(18,6) not null,
        profit_loss_orig decimal(18,6) not null,
        profit_loss_total decimal(18,6) not null,
        profit_loss_total_percent decimal(18,6) not null,
        r_multiple decimal(18,6) not null,
        is_deleted boolean not null default 'false',
        date_created timestamp with time zone not null,
        date_modified timestamp with time zone not null default (now() at time zone 'UTC'),
        constraint pk_trade_calculated_id primary key(trade_calculated_id),
        unique(trade_calculated_id),
        check(extract(timezone from date_created) = '0'),
        check(extract(timezone from date_modified) = '0')
);
COMMENT ON TABLE T_TRADE_CALCULATED IS 'Contains a trade records calculated values.';
COMMENT ON COLUMN T_TRADE_CALCULATED.amount_buy IS 'Amount at time of buying (start of long contract or stop of short contract).';
COMMENT ON COLUMN T_TRADE_CALCULATED.amount_sell IS 'Amount at time of selling (stop of long contract or start of short contract).';
COMMENT ON COLUMN T_TRADE_CALCULATED.risk_input IS 'Risk total, theoretical value.';
COMMENT ON COLUMN T_TRADE_CALCULATED.risk_initial IS 'Risk amount at start of trade.';
COMMENT ON COLUMN T_TRADE_CALCULATED.risk_initial_percent IS 'Risk amount at start of trade (in %).';
COMMENT ON COLUMN T_TRADE_CALCULATED.risk_actual IS 'Actual risk amount at close of trade.';
COMMENT ON COLUMN T_TRADE_CALCULATED.risk_initial_percent IS 'Actual risk amount at close of trade (in %).';
COMMENT ON COLUMN T_TRADE_CALCULATED.cost_total IS 'Total expenses incurred, at close of trade.';
COMMENT ON COLUMN T_TRADE_CALCULATED.cost_other IS 'Total expenses incurred, at close of trade, that are neither tax, nor commission. Mostly financing and negative dividends.';
COMMENT ON COLUMN T_TRADE_CALCULATED.stoploss IS 'Stoploss amount, in the base currency of the application.';
COMMENT ON COLUMN T_TRADE_CALCULATED.stoploss_orig IS 'Stoploss amount, in the base currency of the commodity.';
COMMENT ON COLUMN T_TRADE_CALCULATED.profit_loss IS 'Profit/loss amount, without tax and commission, in the base currency of the application.';
COMMENT ON COLUMN T_TRADE_CALCULATED.profit_loss_orig IS 'Profit/loss amount, without tax and commission, in the base currency of the commodity.';
COMMENT ON COLUMN T_TRADE_CALCULATED.profit_loss_total IS 'Profit/loss amount, with tax and commission, in the base currency of the application.';
COMMENT ON COLUMN T_TRADE_CALCULATED.profit_loss_total_percent IS 'Profit/loss amount, with tax and commission, in the base currency of the application (in %).';
COMMENT ON COLUMN T_TRADE_CALCULATED.is_deleted IS 'When a record is deleted. Nothing gets actually removed.';
