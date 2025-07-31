N::User {
    Name: String,
    Label: String,
    Age: U8,
    IsAdmin: Boolean,
}

E::Knows {
    From: User,
    To: User,
    Properties: {
        Since: U64,
    }
}