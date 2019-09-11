INSERT INTO T_CURRENCY (code, description, date_created)
     VALUES ('EUR', 'Euro', (select now() at time zone 'UTC')),
            ('USD', 'United States Dollar', (select now() at time zone 'UTC')),
            ('GBP', 'British Pound', (select now() at time zone 'UTC')),
            ('CHF', 'Swiss Frank', (select now() at time zone 'UTC')),
            ('CAD', 'Canadian Dollar', (select now() at time zone 'UTC')),
            ('JPY', 'Japanese Yen', (select now() at time zone 'UTC')),
            ('NZD', 'New Zealand Dollar', (select now() at time zone 'UTC')),
            ('AUD', 'Australian Dollar', (select now() at time zone 'UTC')),
            ('HKD', 'Hong Kong Dollar', (select now() at time zone 'UTC')),
            ('DKK', 'Danish Krone', (select now() at time zone 'UTC')),
            ('PLN', 'Polish Zloty', (select now() at time zone 'UTC')),
            ('MXN', 'Mexican Peso', (select now() at time zone 'UTC')),
            ('SEK', 'Swedish Krona', (select now() at time zone 'UTC')),
            ('RUB', 'Russian Ruble', (select now() at time zone 'UTC'));

