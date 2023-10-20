// @generated automatically by Diesel CLI.

diesel::table! {
    durchsicht (id) {
        id -> Int4,
        datum -> Nullable<Date>,
        #[max_length = 30]
        volk -> Nullable<Varchar>,
        koenigin -> Nullable<Bool>,
        stifte -> Nullable<Bool>,
        offene -> Nullable<Bool>,
        verdeckelte -> Nullable<Bool>,
        weiselzelle -> Nullable<Bool>,
        spielnaepfe -> Nullable<Bool>,
        sanftmut -> Nullable<Int2>,
        volksstaerke -> Nullable<Int2>,
        anz_brutwaben -> Nullable<Int2>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(durchsicht,);
