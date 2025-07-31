QUERY GetUserFriends(user_id: U128) =>
    friends <- N<User>(user_id)::Out<Knows>
    RETURN friends