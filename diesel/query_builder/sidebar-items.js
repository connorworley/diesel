initSidebarItems({"fn":[["debug_query","Takes a query `QueryFragment` expression as an argument and returns a type that implements `fmt::Display` and `fmt::Debug` to show the query."]],"mod":[["bind_collector","Types related to managing bind parameters during query construction."]],"struct":[["AstPass","The primary type used when walking a Diesel AST during query execution."],["DebugQuery","A struct that implements `fmt::Display` and `fmt::Debug` to show the SQL representation of a query."],["DeleteStatement","Represents a SQL `DELETE` statement."],["IncompleteInsertStatement","The structure returned by [`insert_into`]."],["IncompleteUpdateStatement","The type returned by `update`. The only thing you can do with this type is call `set` on it."],["InsertStatement","A fully constructed insert statement."],["SqlQuery","The return value of `sql_query`."],["UpdateStatement","Represents a complete `UPDATE` statement."]],"trait":[["AsChangeset","Types which can be passed to `update.set`."],["AsQuery","Types that can be converted into a complete, typed SQL query."],["IntoUpdateTarget","A type which can be passed to [`update`] or [`delete`]."],["Query","A complete SQL query with a return type."],["QueryBuilder","Constructs a SQL query from a Diesel AST."],["QueryFragment","An untyped fragment of SQL."],["QueryId","Uniquely identifies queries by their type for the purpose of prepared statement caching."],["UndecoratedInsertRecord","Marker trait to indicate that no additional operations have been added to a record for insert."]],"type":[["BuildQueryResult","A specialized Result type used with the query builder."]]});