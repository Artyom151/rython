#include <git2.h>
#include <cstring>
#include <string>
#include <vector>
#include <cstdio>
#include <cstdlib>

extern "C" {

int git_init_() {
    return git_libgit2_init() > 0 ? 0 : -1;
}

void git_shutdown_() {
    git_libgit2_shutdown();
}

int git_clone_(const char* url, const char* path) {
    git_repository* repo = nullptr;
    git_clone_options opts = GIT_CLONE_OPTIONS_INIT;
    int ret = git_clone(&repo, url, path, &opts);
    if (ret == 0 && repo) {
        git_repository_free(repo);
    }
    return ret;
}

void* git_open_(const char* path) {
    git_repository* repo = nullptr;
    if (git_repository_open(&repo, path) == 0) {
        return (void*)repo;
    }
    return nullptr;
}

void git_free_(void* repo) {
    if (repo) {
        git_repository_free((git_repository*)repo);
    }
}

char* git_commit_id_(void* repo, const char* branch) {
    if (!repo) return nullptr;
    git_oid oid;
    if (branch && branch[0] != '\0') {
        git_reference* ref = nullptr;
        if (git_reference_dwim(&ref, (git_repository*)repo, branch) != 0) return nullptr;
        const git_oid* target = git_reference_target(ref);
        if (!target) {
            git_object* obj = nullptr;
            if (git_reference_peel(&obj, ref, GIT_OBJECT_COMMIT) != 0) {
                git_reference_free(ref);
                return nullptr;
            }
            target = git_object_id(obj);
            git_oid_cpy(&oid, target);
            git_object_free(obj);
        } else {
            git_oid_cpy(&oid, target);
        }
        git_reference_free(ref);
    } else {
        if (git_reference_name_to_id(&oid, (git_repository*)repo, "HEAD") != 0) return nullptr;
    }
    char* out = (char*)malloc(GIT_OID_HEXSZ + 1);
    if (out) {
        git_oid_tostr(out, GIT_OID_HEXSZ + 1, &oid);
    }
    return out;
}

char** git_branch_list_(void* repo, int* out_count) {
    if (!repo) { *out_count = 0; return nullptr; }
    git_branch_iterator* iter = nullptr;
    if (git_branch_iterator_new(&iter, (git_repository*)repo, GIT_BRANCH_ALL) != 0) {
        *out_count = 0;
        return nullptr;
    }
    std::vector<char*> branches;
    git_reference* ref = nullptr;
    git_branch_t type;
    while (git_branch_next(&ref, &type, iter) == 0) {
        const char* name = nullptr;
        if (git_branch_name(&name, ref) == 0 && name) {
            branches.push_back(strdup(name));
        }
        git_reference_free(ref);
    }
    git_branch_iterator_free(iter);
    *out_count = (int)branches.size();
    if (branches.empty()) return nullptr;
    char** result = (char**)malloc(branches.size() * sizeof(char*));
    for (size_t i = 0; i < branches.size(); i++) {
        result[i] = branches[i];
    }
    return result;
}

int git_status_(void* repo, const char* filepath) {
    if (!repo) return -1;
    unsigned int flags = 0;
    git_status_file(&flags, (git_repository*)repo, filepath);
    return (int)flags;
}

int git_add_(void* repo, const char* filepath) {
    if (!repo) return -1;
    git_index* idx = nullptr;
    int ret = git_repository_index(&idx, (git_repository*)repo);
    if (ret != 0) return ret;
    ret = git_index_add_bypath(idx, filepath);
    if (ret == 0) {
        ret = git_index_write(idx);
    }
    git_index_free(idx);
    return ret;
}

int git_commit_(void* repo, const char* message, const char* name, const char* email) {
    if (!repo) return -1;
    git_index* idx = nullptr;
    int ret = git_repository_index(&idx, (git_repository*)repo);
    if (ret != 0) return ret;

    git_oid tree_oid, commit_oid;
    ret = git_index_write_tree(&tree_oid, idx);
    if (ret != 0) { git_index_free(idx); return ret; }

    git_tree* tree = nullptr;
    ret = git_tree_lookup(&tree, (git_repository*)repo, &tree_oid);
    if (ret != 0) { git_index_free(idx); return ret; }

    git_signature* sig = nullptr;
    ret = git_signature_now(&sig, name ? name : "Unknown", email ? email : "unknown@unknown");
    if (ret != 0) { git_tree_free(tree); git_index_free(idx); return ret; }

    git_commit* parent = nullptr;
    git_reference* head_ref = nullptr;
    ret = git_repository_head(&head_ref, (git_repository*)repo);
    if (ret == 0) {
        const git_oid* head_oid = git_reference_target(head_ref);
        if (head_oid) {
            git_commit_lookup(&parent, (git_repository*)repo, head_oid);
        }
    }

    ret = git_commit_create(
        &commit_oid,
        (git_repository*)repo,
        "HEAD",
        sig, sig,
        nullptr,
        message ? message : "commit",
        tree,
        parent ? 1 : 0,
        parent ? (const git_commit**)&parent : nullptr
    );

    if (parent) git_commit_free(parent);
    git_signature_free(sig);
    git_tree_free(tree);
    git_index_free(idx);
    if (head_ref) git_reference_free(head_ref);
    return ret;
}

int git_push_(void* repo, const char* remote_name, const char* refspec) {
    if (!repo) return -1;
    git_remote* remote = nullptr;
    int ret = git_remote_lookup(&remote, (git_repository*)repo, remote_name ? remote_name : "origin");
    if (ret != 0) return ret;

    char* spec = strdup(refspec ? refspec : "refs/heads/main:refs/heads/main");
    git_strarray refspecs = {&spec, 1};

    git_push_options opts = GIT_PUSH_OPTIONS_INIT;
    ret = git_remote_push(remote, &refspecs, &opts);
    free(spec);
    git_remote_free(remote);
    return ret;
}

int git_pull_(void* repo, const char* remote_name, const char* merge_branch) {
    if (!repo) return -1;
    const char* rname = remote_name ? remote_name : "origin";
    git_remote* remote = nullptr;
    int ret = git_remote_lookup(&remote, (git_repository*)repo, rname);
    if (ret != 0) return ret;

    git_fetch_options fetch_opts = GIT_FETCH_OPTIONS_INIT;
    ret = git_remote_fetch(remote, nullptr, &fetch_opts, nullptr);
    if (ret != 0) { git_remote_free(remote); return ret; }

    const char* mbranch = merge_branch ? merge_branch : "main";
    git_annotated_commit* fetch_head = nullptr;
    char fetch_ref[256];
    git_reference* ref = nullptr;
    snprintf(fetch_ref, sizeof(fetch_ref), "refs/remotes/%s/%s", rname, mbranch);
    ret = git_reference_lookup(&ref, (git_repository*)repo, fetch_ref);
    if (ret != 0) {
        snprintf(fetch_ref, sizeof(fetch_ref), "refs/heads/%s", mbranch);
        ret = git_reference_lookup(&ref, (git_repository*)repo, fetch_ref);
    }
    if (ret == 0) {
        ret = git_annotated_commit_from_ref(&fetch_head, (git_repository*)repo, ref);
        git_reference_free(ref);
    }
    if (ret != 0) { git_remote_free(remote); return ret; }

    git_merge_options merge_opts = GIT_MERGE_OPTIONS_INIT;
    git_checkout_options checkout_opts = GIT_CHECKOUT_OPTIONS_INIT;
    checkout_opts.checkout_strategy = GIT_CHECKOUT_SAFE;

    const git_annotated_commit* heads[] = {fetch_head};
    ret = git_merge((git_repository*)repo, heads, 1, &merge_opts, &checkout_opts);

    git_annotated_commit_free(fetch_head);
    git_remote_free(remote);
    return ret;
}

char* git_diff_stats_(void* repo) {
    if (!repo) return nullptr;
    git_reference* head = nullptr;
    if (git_repository_head(&head, (git_repository*)repo) != 0) return nullptr;

    git_tree* tree = nullptr;
    git_commit* commit = nullptr;
    const git_oid* oid = git_reference_target(head);
    if (!oid) { git_reference_free(head); return nullptr; }

    if (git_commit_lookup(&commit, (git_repository*)repo, oid) != 0) { git_reference_free(head); return nullptr; }
    if (git_commit_tree(&tree, commit) != 0) { git_commit_free(commit); git_reference_free(head); return nullptr; }

    git_diff* diff = nullptr;
    git_diff_tree_to_workdir(&diff, (git_repository*)repo, tree, nullptr);

    char* result = nullptr;
    if (diff) {
        git_diff_stats* stats = nullptr;
        if (git_diff_get_stats(&stats, diff) == 0) {
            git_buf buf = {0};
            if (git_diff_stats_to_buf(&buf, stats, GIT_DIFF_STATS_FULL, 80) == 0) {
                result = strdup(buf.ptr);
                git_buf_free(&buf);
            }
            git_diff_stats_free(stats);
        }
        git_diff_free(diff);
    }

    git_tree_free(tree);
    git_commit_free(commit);
    git_reference_free(head);
    return result;
}

char** git_log_(void* repo, int max_count, int* out_count) {
    if (!repo) { *out_count = 0; return nullptr; }
    git_revwalk* walker = nullptr;
    if (git_revwalk_new(&walker, (git_repository*)repo) != 0) {
        *out_count = 0;
        return nullptr;
    }
    git_revwalk_sorting(walker, GIT_SORT_TIME);
    git_revwalk_push_head(walker);

    std::vector<char*> commits;
    git_oid oid;
    int count = 0;
    while (git_revwalk_next(&oid, walker) == 0 && count < max_count) {
        git_commit* commit = nullptr;
        if (git_commit_lookup(&commit, (git_repository*)repo, &oid) != 0) continue;

        const char* msg = git_commit_summary(commit);
        const char* author = git_commit_author(commit) ? git_commit_author(commit)->name : "";
        git_time_t ctime = git_commit_time(commit);

        char oid_str[GIT_OID_HEXSZ + 1];
        git_oid_tostr(oid_str, sizeof(oid_str), &oid);

        char buf[512];
        snprintf(buf, sizeof(buf), "%s|%s|%lld|%s", oid_str, author, (long long)ctime, msg ? msg : "");

        commits.push_back(strdup(buf));
        git_commit_free(commit);
        count++;
    }

    git_revwalk_free(walker);

    *out_count = (int)commits.size();
    if (commits.empty()) return nullptr;

    char** result = (char**)malloc(commits.size() * sizeof(char*));
    for (size_t i = 0; i < commits.size(); i++) {
        result[i] = commits[i];
    }
    return result;
}

void git_free_str(char* s) {
    if (s) free(s);
}

void git_free_str_array(char** arr, int count) {
    if (!arr) return;
    for (int i = 0; i < count; i++) {
        if (arr[i]) free(arr[i]);
    }
    free(arr);
}

}
