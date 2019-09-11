CREATE TABLE T_TRADE_DRAWDOWN_HIST
(
    trade_drawdown_hist_id bigserial not null,
    trade_drawdown_id bigint not null,
    drawdown_current int not null,
    drawdown_max int not null,
    is_deleted boolean not null,
    date_created timestamp with time zone not null,
    date_modified timestamp with time zone not null,
    date_hist_created timestamp with time zone not null,
    date_hist_modified timestamp with time zone not null default (now() at time zone 'UTC'),
    constraint pk_trade_drawdown_hist_id primary key(trade_drawdown_id),
    constraint fk_T_TRADE_DRAWDOWN_HIST_trade_drawdown_id foreign key (trade_drawdown_id) references T_TRADE_DRAWDOWN (trade_drawdown_id),
    check(extract(timezone from date_created) = '0'),
    check(extract(timezone from date_modified) = '0'),
    check(extract(timezone from date_hist_created) = '0'),
    check(extract(timezone from date_hist_modified) = '0')
);
COMMENT ON TABLE T_TRADE_DRAWDOWN_HIST IS 'History table for T_TRADE_DRAWDOWN.';
