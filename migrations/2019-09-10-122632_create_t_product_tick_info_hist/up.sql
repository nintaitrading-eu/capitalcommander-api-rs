CREATE TABLE T_PRODUCT_TICK_INFO_HIST
(
    product_tick_info_hist_id bigserial not null,
    product_tick_info_id bigint not null,
    description varchar(4000) not null,
    tick decimal(18,6) not null,
    tick_value decimal(18,6) not null,
    order_min decimal(18,6) not null,
    order_max decimal(18,6) not null,
    margin_day_proc decimal(18,6) not null,
    margin_night_proc decimal(18,6) not null,
    is_deleted boolean not null,
    date_created timestamp with time zone not null,
    date_modified timestamp with time zone not null,
    date_hist_created timestamp with time zone not null,
    date_hist_modified timestamp with time zone not null default (now() at time zone 'UTC'),
    constraint pk_product_tick_info_hist_id primary key(product_tick_info_hist_id),
    unique(product_tick_info_hist_id),
    constraint fk_T_PRODUCT_TICK_INFO_HIST_product_tick_info_id foreign key(product_tick_info_id) references T_PRODUCT_TICK_INFO (product_tick_info_id),
    check(extract(timezone from date_created) = '0'),
    check(extract(timezone from date_modified) = '0'),
    check(extract(timezone from date_hist_created) = '0'),
    check(extract(timezone from date_hist_modified) = '0')
);
COMMENT ON TABLE T_PRODUCT_TICK_INFO_HIST IS 'History table for T_PRODUCT_TICK_INFO.';
