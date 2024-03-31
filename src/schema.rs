use diesel::table;

// Cards related tables
table! {
    acts (id) {
        id -> Int4,
        back_flavor -> Nullable<Varchar>,
        back_name -> Nullable<Varchar>,
        back_text -> Nullable<Varchar>,
        backimagesrc -> Nullable<Varchar>,
        code -> Varchar,
        doom -> Nullable<Int4>,
        double_sided -> Bool,
        encounter_code -> Varchar,
        encounter_name -> Varchar,
        encounter_position -> Int4,
        exceptional -> Bool,
        faction_code -> Varchar,
        faction_name -> Varchar,
        flavor -> Nullable<Varchar>,
        health_per_investigator -> Bool,
        illustrator -> Nullable<Varchar>,
        imagesrc -> Nullable<Varchar>,
        is_unique -> Bool,
        myriad -> Bool,
        name -> Varchar,
        octgn_id -> Nullable<Varchar>,
        pack_code -> Varchar,
        pack_name -> Varchar,
        permanent -> Bool,
        position -> Int4,
        quantity -> Int4,
        real_name -> Varchar,
        real_slot -> Nullable<Varchar>,
        real_text -> Nullable<Varchar>,
        spoiler -> Int4,
        stage -> Int4,
        text -> Nullable<Varchar>,
        type_code -> Varchar,
        type_name -> Varchar,
        url -> Varchar,
        clues -> Nullable<Int4>,
    }
}

table! {
    agendas (id) {
        id -> Int4,
        back_flavor -> Nullable<Varchar>,
        back_name -> Nullable<Varchar>,
        back_text -> Nullable<Varchar>,
        backimagesrc -> Nullable<Varchar>,
        code -> Varchar,
        doom -> Nullable<Int4>,
        double_sided -> Bool,
        encounter_code -> Varchar,
        encounter_name -> Varchar,
        encounter_position -> Int4,
        exceptional -> Bool,
        faction_code -> Varchar,
        faction_name -> Varchar,
        flavor -> Nullable<Varchar>,
        health_per_investigator -> Bool,
        illustrator -> Nullable<Varchar>,
        imagesrc -> Nullable<Varchar>,
        is_unique -> Bool,
        myriad -> Bool,
        name -> Varchar,
        octgn_id -> Nullable<Varchar>,
        pack_code -> Varchar,
        pack_name -> Varchar,
        permanent -> Bool,
        position -> Int4,
        quantity -> Int4,
        real_name -> Varchar,
        real_slot -> Nullable<Varchar>,
        real_text -> Nullable<Varchar>,
        spoiler -> Int4,
        stage -> Nullable<Int4>,
    }
}

// Define other card-related tables similarly...

// Investigators related tables
table! {
    investigators (id) {
        id -> Int4,
        alternated_by -> Nullable<Array<Text>>,
        back_flavor -> Nullable<Text>,
        back_text -> Nullable<Text>,
        backimagesrc -> Nullable<Text>,
        code -> Text,
        deck_limit -> Nullable<Int4>,
        double_sided -> Bool,
        duplicated_by -> Nullable<Array<Text>>,
        exceptional -> Bool,
        faction_code -> Text,
        faction_name -> Text,
        flavor -> Nullable<Text>,
        health -> Int4,
        health_per_investigator -> Bool,
        illustrator -> Text,
        imagesrc -> Nullable<Text>,
        is_unique -> Bool,
        myriad -> Bool,
        name -> Text,
        octgn_id -> Nullable<Text>,
        pack_code -> Text,
        pack_name -> Text,
        permanent -> Bool,
        position -> Int4,
        quantity -> Int4,
        real_name -> Text,
        real_slot -> Nullable<Text>,
        real_text -> Text,
        real_traits -> Text,
        sanity -> Int4,
        skill_agility -> Int4,
        skill_combat -> Int4,
        skill_intellect -> Int4,
        skill_willpower -> Int4,
        subname -> Text,
        text -> Text,
        traits -> Text,
        type_code -> Text,
        type_name -> Text,
        url -> Text,
    }
}

// Define other investigator-related tables similarly...

// Errata related tables
table! {
    errata_dates (id) {
        id -> Int4,
        asset_id -> Int4,
        date -> Varchar,
        timezone_type -> Int4,
        timezone -> Varchar,
    }
}

// Define other errata-related tables similarly...

// Keys related tables
table! {
    keys (id) {
        id -> Int4,
        pack_code -> Varchar,
        pack_name -> Varchar,
        type_code -> Varchar,
        type_name -> Varchar,
        faction_code -> Varchar,
        faction_name -> Varchar,
        encounter_code -> Varchar,
        encounter_name -> Varchar,
        linked_to_code -> Nullable<Varchar>,
        linked_to_name -> Nullable<Varchar>,
        position -> Int4,
        exceptional -> Bool,
        myriad -> Bool,
        encounter_position -> Int4,
        code -> Varchar,
        name -> Varchar,
        real_name -> Varchar,
        subname -> Nullable<Varchar>,
        text -> Text,
        real_text -> Text,
        quantity -> Int4,
        health_per_investigator -> Bool,
        traits -> Text,
        real_traits -> Text,
        flavor -> Text,
        illustrator -> Text,
        is_unique -> Bool,
        permanent -> Bool,
        double_sided -> Bool,
        octgn_id -> Nullable<Varchar>,
        url -> Varchar,
        imagesrc -> Nullable<Varchar>,
        spoiler -> Int4,
        linked_card_id -> Nullable<Int4>,
    }
}

// Define other keys-related tables similarly...

// Define other tables for scenarios, skills, stories, locations, etc. similarly...
