CREATE TABLE T_PRODUCT_TICK_INFO
(
    product_tick_info_id bigserial not null,
    description varchar(4000) not null,
    tick decimal(18,6) not null,
    tick_value decimal(18,6) not null,
    order_min decimal(18,6) not null,
    order_max decimal(18,6) not null,
    margin_day_proc decimal(18,6) not null,
    margin_night_proc decimal(18,6) not null,
    is_deleted boolean not null default 'false',
    date_created timestamp with time zone not null,
    date_modified timestamp with time zone not null default (now() at time zone 'UTC'),
    constraint pk_product_tick_info_id primary key(product_tick_info_id),
    unique(product_tick_info_id),
    check(extract(timezone from date_created) = '0'),
    check(extract(timezone from date_modified) = '0')
);
COMMENT ON TABLE T_PRODUCT_TICK_INFO IS 'Table with tick value information on contracts. Only belongs to cfd products and futures contracts.';
