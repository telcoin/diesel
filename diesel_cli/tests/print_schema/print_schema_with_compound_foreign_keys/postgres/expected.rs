diesel::table! {
    /// Representation of the `a` table.
    ///
    /// (Automatically generated by Diesel.)
    a (id) {
        /// The `id` column of the `a` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `a` column of the `a` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        a -> Int4,
        /// The `b` column of the `a` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        b -> Int4,
    }
}

diesel::table! {
    /// Representation of the `b` table.
    ///
    /// (Automatically generated by Diesel.)
    b (id) {
        /// The `id` column of the `b` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `a` column of the `b` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        a -> Nullable<Int4>,
        /// The `b` column of the `b` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        b -> Nullable<Int4>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    a,
    b,
);