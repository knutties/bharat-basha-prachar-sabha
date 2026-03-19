// @generated automatically by Diesel CLI.

diesel::table! {
    student_profiles (user_id) {
        user_id -> Uuid,
        grade -> Int2,
        #[max_length = 100]
        mother_tongue -> Varchar,
        #[max_length = 100]
        state_of_residence -> Varchar,
        #[max_length = 50]
        board -> Nullable<Varchar>,
        school_id -> Nullable<Uuid>,
        #[max_length = 255]
        parent_email -> Nullable<Varchar>,
    }
}

diesel::table! {
    teacher_profiles (user_id) {
        user_id -> Uuid,
        languages -> Array<Nullable<Text>>,
        qualifications -> Text,
        bio -> Nullable<Text>,
        verified -> Bool,
        rating -> Nullable<Float8>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 20]
        phone -> Varchar,
        #[max_length = 50]
        role -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(student_profiles -> users (user_id));
diesel::joinable!(teacher_profiles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    student_profiles,
    teacher_profiles,
    users,
);
