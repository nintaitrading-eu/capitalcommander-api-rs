insert into t_version values(1, '0.0.1', 'First try!', '0.0.1', 'First try!', (select now() at time zone 'UTC'), (select now() at time zone 'UTC'));
