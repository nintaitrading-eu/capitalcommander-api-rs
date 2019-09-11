CREATE TABLE T_VERSION
(
    version_id int not null,
    database_version varchar(100) not null default '',
    database_version_info varchar(100) not null default '',
    application_version varchar(100) not null default '',
    application_version_info varchar(100) not null default '',
    date_created timestamp with time zone not null default '1900-01-01',
    date_modified timestamp with time zone not null default '1900-01-01',
    constraint pk_version_id primary key(version_id)
);
