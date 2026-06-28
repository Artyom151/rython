#include <sqlite3.h>
#include <cstring>
#include <string>
#include <vector>
#include <cstdint>

extern "C" {

void* sqlite3_open_db(const char* path) {
    sqlite3* db;
    if (sqlite3_open(path, &db) == SQLITE_OK) {
        return db;
    }
    return nullptr;
}

void sqlite3_close_db(void* db) {
    sqlite3_close((sqlite3*)db);
}

const char* sqlite3_last_error(void* db) {
    return sqlite3_errmsg((sqlite3*)db);
}

void* sqlite3_exec_query(void* db, const char* sql) {
    sqlite3_stmt* stmt;
    if (sqlite3_prepare_v2((sqlite3*)db, sql, -1, &stmt, nullptr) == SQLITE_OK) {
        return stmt;
    }
    return nullptr;
}

int sqlite3_stmt_step(void* stmt) {
    return sqlite3_step((sqlite3_stmt*)stmt);
}

void sqlite3_stmt_finalize(void* stmt) {
    sqlite3_finalize((sqlite3_stmt*)stmt);
}

int sqlite3_stmt_column_count(void* stmt) {
    return sqlite3_column_count((sqlite3_stmt*)stmt);
}

const char* sqlite3_stmt_column_name(void* stmt, int col) {
    return sqlite3_column_name((sqlite3_stmt*)stmt, col);
}

int sqlite3_stmt_column_type(void* stmt, int col) {
    return sqlite3_column_type((sqlite3_stmt*)stmt, col);
}

int64_t sqlite3_stmt_column_int64(void* stmt, int col) {
    return sqlite3_column_int64((sqlite3_stmt*)stmt, col);
}

double sqlite3_stmt_column_double(void* stmt, int col) {
    return sqlite3_column_double((sqlite3_stmt*)stmt, col);
}

const char* sqlite3_stmt_column_text(void* stmt, int col) {
    auto t = sqlite3_column_text((sqlite3_stmt*)stmt, col);
    return t ? (const char*)t : "";
}

int sqlite3_stmt_bind_int(void* stmt, int idx, int64_t val) {
    return sqlite3_bind_int64((sqlite3_stmt*)stmt, idx, val);
}

int sqlite3_stmt_bind_double(void* stmt, int idx, double val) {
    return sqlite3_bind_double((sqlite3_stmt*)stmt, idx, val);
}

int sqlite3_stmt_bind_text(void* stmt, int idx, const char* val) {
    return sqlite3_bind_text((sqlite3_stmt*)stmt, idx, val, -1, SQLITE_TRANSIENT);
}

int sqlite3_changes_count(void* db) {
    return sqlite3_changes((sqlite3*)db);
}

int64_t sqlite3_last_insert_rowid_(void* db) {
    return sqlite3_last_insert_rowid((sqlite3*)db);
}

void sqlite3_exec_direct(void* db, const char* sql) {
    sqlite3_exec((sqlite3*)db, sql, nullptr, nullptr, nullptr);
}

}
