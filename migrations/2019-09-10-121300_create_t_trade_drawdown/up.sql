CREATE TABLE T_TRADE_DRAWDOWN
(
    trade_drawdown_id bigserial not null,
    drawdown_current int not null,
    drawdown_max int not null,
    is_deleted boolean not null default 'false',
    date_created timestamp with time zone not null,
    date_modified timestamp with time zone not null default (now() at time zone 'UTC'),
    constraint pk_trade_drawdown_id primary key(trade_drawdown_id),
    unique(trade_drawdown_id),
    check(extract(timezone from date_created) = '0'),
    check(extract(timezone from date_modified) = '0')
);
