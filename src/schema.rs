table! {
  trash_days (trash_type, nth, day) {
      trash_type -> VarChar,
      nth -> Integer,
      day -> VarChar,
  }
}

table! {
  posts (id) {
      id -> Integer,
      title -> Text,
      body -> Text,
      published -> Bool,
  }
}