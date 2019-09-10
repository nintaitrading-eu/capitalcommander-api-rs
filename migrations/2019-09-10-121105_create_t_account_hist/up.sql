CREATE TABLE T_ACCOUNT_HIST
(
    account_hist_id bigserial not null,
    account_id bigint not null,
    name varchar(4000) not null,
    description varchar(4000) not null,
    is_deleted boolean not null,
    date_created timestamp with time zone not null,
    date_modified timestamp with time zone not null,
    date_hist_created timestamp with time zone not null,
    date_hist_modified timestamp with time zone not null default (now() at time zone 'UTC'),
    constraint pk_account_hist_id primary key(account_hist_id),
    constraint fk_T_ACCOUNT_HIST_account_id foreign key (account_id) references T_ACCOUNT (account_id),
    check(extract(timezone from date_created) = '0'),
    check(extract(timezone from date_modified) = '0'),
    check(extract(timezone from date_hist_created) = '0'),
    check(extract(timezone from date_hist_modified) = '0')
);
COMMENT ON TABLE T_ACCOUNT_HIST IS 'History table for T_ACCOUNT.';
