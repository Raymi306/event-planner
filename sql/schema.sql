CREATE TYPE person_role AS ENUM ('disabled', 'user', 'admin');

CREATE TYPE organization_person_role AS ENUM ('read', 'readwrite', 'admin');

CREATE TYPE organization_category AS ENUM (
    'activism',
    'bipoc',
    'lgbtq',
    'art',
    'music',
    'community',
    'labor',
    'mutual_aid'
);

CREATE TYPE organizer_role AS ENUM (
    'coordinator',
    'teammember'
);

CREATE TYPE event_organization_role AS ENUM (
    'vending',
    'performing',
    'tabling',
    'literature'
);

CREATE TABLE person (
    id serial
    full_name varchar(255),
    email varchar(254) unique,
    phone_number_prefix varchar(5),
    phone_number varchar(15),
    phone_number_extension varchar(11),
    password_digest bytes(16),
    role role
);

CREATE TABLE organization (
    id serial,
    name varchar(255),
    description text,
    website varchar(2083),
    phone_number varchar(15)
);

CREATE TABLE organization_categories (
    organization_id integer,
    category organization_category,
    primary key(organization_id, category),
    constraint fk_organization_id
        foreign key(organization_id)
            references organization(id)
            on delete cascade
);

CREATE TABLE organization_person_jct (
    organization_id integer,
    person_id integer,
    role organization_person_role,
    primary key(organization_id, person_id),
    constraint fk_organization_id
        foreign key(organization_id)
            references organization(id)
            on delete cascade,
    constraint fk_person_id
        foreign key(person_id)
            references person(id)
            on delete cascade
);

CREATE TABLE event (
    id serial,
    name varchar(255),
    theme varchar(255),
    description text
);

CREATE TABLE organizer (
    person_id integer,
    event_id integer,
    role organizer_role
);

CREATE TABLE event_organization_jct (
    id serial,
    event_id integer,
    organizer_id integer,
    role event_organization_role,
    contacted_on datetime,
    will_attend boolean
);

CREATE TABLE event_person_jct (
    id serial,
    event_id integer,
    organizer_id integer,
    contacted_on datetime,
    will_attend boolean
);

CREATE TABLE event_organization_annotations (
    id serial,
    author_id integer,
    event_organization_id integer,
    content text,
    is_deleted boolean,
    constraint fk_author_id
        foreign key(author_id)
            references person(id)
            on delete cascade,
    constraint fk_event_organization_id
        foreign key(event_organization_id)
            references event_organization_jct(id)
            on delete cascade
);

CREATE TABLE event_person_annotations (
    id serial,
    author_id integer,
    event_person_id integer,
    content text,
    is_deleted boolean,
    constraint fk_person_id
        foreign key(author_id)
            references person(id)
            on delete cascade,
    constraint fk_event_person_id
        foreign key(event_person_id)
            references event_person_jct(id)
            on delete cascade
);
