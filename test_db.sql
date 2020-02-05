create table color (
    id serial primary key,
    code varchar (10) not null,
    name varchar (20) not null,
    CONSTRAINT unq_name_color UNIQUE (name)
);

INSERT into color (id, code, name) values
 (1, 'FFFFFF', 'White'),
 (2, 'C0C0C0', 'Silver'),
 (3, '000000', 'Black'),
 (4, '0000A0', 'Dark Blue'),
 (5, 'FF0000', 'Red');
