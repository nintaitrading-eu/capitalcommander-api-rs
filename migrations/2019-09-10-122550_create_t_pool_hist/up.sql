CREATE TABLE T_POOL_HIST
(
    pool_hist_id bigserial not null,
    pool_id bigint not null,
    account_id bigint not null,
    total decimal(18,6) not null,
    invested decimal(18,6) not null,
    cash decimal(18,6) not null,
    is_manually_added boolean not null,
    is_deleted boolean not null,
    date_created timestamp with time zone not null,
    date_modified timestamp with time zone not null,
    date_hist_created timestamp with time zone not null,
    date_hist_modified timestamp with time zone not null default (now() at time zone 'UTC'),
    constraint pk_pool_hist_id primary key(pool_hist_id),
    unique(pool_hist_id),
    constraint fk_T_POOL_pool_id foreign key(pool_id) references T_POOL(pool_id),
    check(extract(timezone from date_created) = '0'),
    check(extract(timezone from date_modified) = '0'),
    check(extract(timezone from date_hist_created) = '0'),
    check(extract(timezone from date_hist_modified) = '0')
);
COMMENT ON TABLE T_POOL_HIST IS 'History table for T_POOL.';
