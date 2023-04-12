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
    id serial unique,
    full_name varchar(255),
    email varchar(254) unique,
    phone_number_prefix varchar(5),
    phone_number varchar(15),
    phone_number_extension varchar(11),
    password_digest bytea,
    totp_secret bytea,
    role person_role not null default 'disabled'
);

CREATE TABLE organization (
    id serial unique,
    name varchar(255) not null,
    description text,
    website varchar(2083),
    phone_number_prefix varchar(5),
    phone_number varchar(15)
    phone_number_extension varchar(11),
);

CREATE TABLE organization_categories (
    organization_id integer,
    category organization_category unique,
    primary key(organization_id, category),
    constraint fk_organization_id
        foreign key(organization_id)
            references organization(id)
            on delete cascade
);

CREATE TABLE organization_person_jct (
    organization_id integer,
    person_id integer,
    role organization_person_role default 'read',
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
    id serial unique,
    name varchar(255) not null,
    theme varchar(255),
    description text
);

CREATE TABLE organizer (
    person_id integer,
    event_id integer,
    role organizer_role,
    primary key (person_id, event_id),
    constraint fk_person_id
        foreign key(person_id)
            references person(id)
            on delete cascade,
    constraint fk_event_id
        foreign key(event_id)
            references event(id)
            on delete cascade
);

CREATE TABLE event_organization_jct (
    id serial unique,
    event_id integer not null,
    organization_id integer not null,
    role event_organization_role,
    contacted_on timestamp,
    will_attend boolean default false,
    primary key(event_id, organization_id),
    constraint event_organization_unique unique (event_id, organization_id),
    constraint fk_event_id
        foreign key(event_id)
            references event(id)
            on delete cascade,
    constraint fk_organization_id
        foreign key(organization_id)
            references organization(id)
            on delete cascade
);

CREATE TABLE event_person_jct (
    id serial unique,
    event_id integer not null,
    person_id integer not null,
    contacted_on timestamp,
    will_attend boolean default false,
    primary key(event_id, person_id),
    constraint event_person_unique unique (event_id, person_id),
    constraint fk_event_id
        foreign key(event_id)
            references event(id)
            on delete cascade,
    constraint fk_person_id
        foreign key(person_id)
            references person(id)
            on delete cascade
);

CREATE TABLE event_organization_annotations (
    id serial unique,
    author_id integer not null,
    event_organization_id integer not null,
    content text not null,
    is_deleted boolean default false,
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
    id serial unique,
    author_id integer not null,
    event_person_id integer not null,
    content text not null,
    is_deleted boolean default false,
    constraint fk_person_id
        foreign key(author_id)
            references person(id)
            on delete cascade,
    constraint fk_event_person_id
        foreign key(event_person_id)
            references event_person_jct(id)
            on delete cascade
);
