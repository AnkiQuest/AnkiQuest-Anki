SELECT id,
  nid,
  due,
  cast(ivl AS integer),
  cast(mod AS integer),
  did,
  odid,
<<<<<<< HEAD
  reps
=======
  reps,
  DUE_PREDICATE
>>>>>>> 36c4f39d7a9880063656981b7d1f35dd23a38007
FROM cards
WHERE did IN (
    SELECT id
    FROM active_decks
  )
<<<<<<< HEAD
  AND queue = ?
=======
  AND queue = ?1
>>>>>>> 36c4f39d7a9880063656981b7d1f35dd23a38007
