CREATE TABLE T_PARAMETER_HIST
(
    parameter_hist_id bigserial not null,
    parameter_id bigint not null,
    name varchar(50) not null,
    value varchar(512) not null,
    value_default varchar(512) not null,
    description varchar(4000) not null,
    datatype varchar(50) not null,
    is_deleted boolean not null,
    date_created timestamp with time zone not null,
    date_modified timestamp with time zone not null,
    date_hist_created timestamp with time zone not null,
    date_hist_modified timestamp with time zone not null default (now() at time zone 'UTC'),
    constraint pk_parameter_hist_id primary key(parameter_hist_id),
    unique(parameter_id),
    constraint fk_T_PARAMETER_HIST_parameter_id foreign key (parameter_id) references T_PARAMETER (parameter_id),
    check(extract(timezone from date_created) = '0'),
    check(extract(timezone from date_modified) = '0'),
    check(extract(timezone from date_hist_created) = '0'),
    check(extract(timezone from date_hist_modified) = '0')
);
COMMENT ON TABLE T_PARAMETER_HIST IS 'History table for T_PARAMETER.';
