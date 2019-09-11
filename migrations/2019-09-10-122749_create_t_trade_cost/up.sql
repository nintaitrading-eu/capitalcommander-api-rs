CREATE TABLE T_TRADE_COST
(
        trade_cost_id bigserial not null,
        commission decimal(18,6) not null,
        tax decimal(18,6) not null,
        is_manually_added boolean not null default 'false',
        is_deleted boolean not null default 'false',
        date_created timestamp with time zone not null,
        date_modified timestamp with time zone not null default (now() at time zone 'UTC'),
        constraint pk_trade_cost_id primary key(trade_cost_id),
        unique(trade_cost_id),
        check(extract(timezone from date_created) = '0'),
        check(extract(timezone from date_modified) = '0')
);
COMMENT ON TABLE T_TRADE_COST IS 'Table with costs associated to trading.';
