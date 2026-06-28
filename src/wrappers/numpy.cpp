#include <cstdint>
#include <cstring>
#include <vector>
#include <cmath>
#include <algorithm>
#include <string>
#include <sstream>
#include <cstdlib>
#include <map>

struct NDArray {
    std::vector<double> data;
    std::vector<int64_t> shape;
    std::vector<int64_t> strides;
    int ndim;
};

static int64_t num_elements(const int64_t* shape, int ndim) {
    int64_t total = 1;
    for (int i = 0; i < ndim; i++) total *= shape[i];
    return total;
}

static void compute_strides(const int64_t* shape, int ndim, int64_t* strides) {
    strides[ndim - 1] = 1;
    for (int i = ndim - 2; i >= 0; i--) {
        strides[i] = strides[i + 1] * shape[i + 1];
    }
}

static NDArray* create_array(const int64_t* shape, int ndim) {
    NDArray* arr = new NDArray();
    arr->shape.assign(shape, shape + ndim);
    arr->ndim = ndim;
    arr->strides.resize(ndim);
    compute_strides(shape, ndim, arr->strides.data());
    arr->data.resize(num_elements(shape, ndim), 0.0);
    return arr;
}

static int64_t flat_index(const NDArray* arr, const int64_t* indices, int num_indices) {
    int64_t idx = 0;
    for (int i = 0; i < num_indices; i++) {
        idx += indices[i] * arr->strides[i];
    }
    return idx;
}

extern "C" {

void* numpy_zeros(const int64_t* shape, int ndim) {
    return create_array(shape, ndim);
}

void* numpy_ones(const int64_t* shape, int ndim) {
    NDArray* arr = create_array(shape, ndim);
    for (auto& v : arr->data) v = 1.0;
    return arr;
}

void* numpy_eye(int n) {
    int64_t shape[2] = {n, n};
    NDArray* arr = create_array(shape, 2);
    for (int i = 0; i < n; i++) {
        arr->data[i * n + i] = 1.0;
    }
    return arr;
}

void* numpy_arange(double start, double stop, double step) {
    int64_t sz = (int64_t)std::ceil((stop - start) / step);
    if (sz < 0) sz = 0;
    int64_t shape[1] = {sz};
    NDArray* arr = create_array(shape, 1);
    for (int64_t i = 0; i < sz; i++) {
        arr->data[i] = start + i * step;
    }
    return arr;
}

void* numpy_linspace(double start, double stop, int64_t num) {
    if (num < 2) {
        int64_t shape[1] = {1};
        NDArray* arr = create_array(shape, 1);
        arr->data[0] = stop;
        return arr;
    }
    int64_t shape[1] = {num};
    NDArray* arr = create_array(shape, 1);
    double step = (stop - start) / (num - 1);
    for (int64_t i = 0; i < num; i++) {
        arr->data[i] = start + i * step;
    }
    return arr;
}

void* numpy_full(const int64_t* shape, int ndim, double value) {
    NDArray* arr = create_array(shape, ndim);
    for (auto& v : arr->data) v = value;
    return arr;
}

void* numpy_copy(void* arr_ptr) {
    NDArray* src = (NDArray*)arr_ptr;
    NDArray* dst = new NDArray(*src);
    return dst;
}

void* numpy_reshape(void* arr_ptr, const int64_t* new_shape, int new_ndim) {
    NDArray* src = (NDArray*)arr_ptr;
    NDArray* dst = new NDArray();
    dst->shape.assign(new_shape, new_shape + new_ndim);
    dst->ndim = new_ndim;
    dst->strides.resize(new_ndim);
    compute_strides(new_shape, new_ndim, dst->strides.data());
    dst->data = src->data;
    return dst;
}

void* numpy_transpose(void* arr_ptr) {
    NDArray* src = (NDArray*)arr_ptr;
    if (src->ndim != 2) {
        return numpy_copy(arr_ptr);
    }
    int64_t new_shape[2] = {src->shape[1], src->shape[0]};
    NDArray* dst = create_array(new_shape, 2);
    for (int64_t i = 0; i < src->shape[0]; i++) {
        for (int64_t j = 0; j < src->shape[1]; j++) {
            dst->data[j * src->shape[0] + i] = src->data[i * src->shape[1] + j];
        }
    }
    return dst;
}

void* numpy_concatenate(void** arrs, int num_arrs, int axis) {
    if (num_arrs == 0) return nullptr;
    NDArray** arrays = (NDArray**)arrs;
    NDArray* first = arrays[0];
    
    std::vector<int64_t> out_shape = first->shape;
    int64_t sum_dim = 0;
    for (int i = 0; i < num_arrs; i++) {
        sum_dim += arrays[i]->shape[axis];
    }
    out_shape[axis] = sum_dim;
    
    NDArray* result = create_array(out_shape.data(), first->ndim);
    int64_t offset = 0;
    for (int i = 0; i < num_arrs; i++) {
        NDArray* a = arrays[i];
        int64_t slice_size = 1;
        for (int d = axis; d < a->ndim; d++) slice_size *= a->shape[d];
        int64_t num_slices = a->data.size() / slice_size;
        int64_t copy_size = a->shape[axis] * (axis + 1 < a->ndim ? a->strides[axis] : 1);
        for (int64_t s = 0; s < num_slices; s++) {
            std::memcpy(&result->data[offset + s * result->strides[axis]], &a->data[s * a->strides[axis]], copy_size * sizeof(double));
        }
        offset += slice_size;
    }
    return result;
}

void* numpy_stack(void** arrs, int num_arrs, int axis) {
    if (num_arrs == 0) return nullptr;
    NDArray** arrays = (NDArray**)arrs;
    NDArray* first = arrays[0];
    
    int new_ndim = first->ndim + 1;
    std::vector<int64_t> out_shape(new_ndim);
    int out_idx = 0;
    for (int d = 0; d < new_ndim; d++) {
        if (d == axis) out_shape[d] = num_arrs;
        else out_shape[d] = first->shape[out_idx++];
    }
    
    NDArray* result = create_array(out_shape.data(), new_ndim);
    for (int i = 0; i < num_arrs; i++) {
        NDArray* a = arrays[i];
        int64_t elem_size = a->data.size();
        int64_t base = i * elem_size;
        for (int64_t j = 0; j < elem_size; j++) {
            result->data[base + j] = a->data[j];
        }
    }
    return result;
}

void* numpy_slice(void* arr_ptr, int64_t start, int64_t stop, int64_t step, int axis) {
    NDArray* src = (NDArray*)arr_ptr;
    if (step == 0) step = 1;
    if (start < 0) start += src->shape[axis];
    if (stop < 0) stop += src->shape[axis];
    if (start < 0) start = 0;
    if (stop > src->shape[axis]) stop = src->shape[axis];
    
    int64_t dim_size = (stop - start + step - 1) / step;
    if (dim_size < 0) dim_size = 0;
    
    std::vector<int64_t> out_shape = src->shape;
    out_shape[axis] = dim_size;
    
    NDArray* result = create_array(out_shape.data(), src->ndim);
    int64_t src_stride = src->strides[axis];
    int64_t dst_stride = result->strides[axis];
    
    if (src->ndim == 1) {
        for (int64_t i = 0; i < dim_size; i++) {
            result->data[i] = src->data[start + i * step];
        }
    } else {
        int64_t slice_size = 1;
        for (int d = axis + 1; d < src->ndim; d++) slice_size *= src->shape[d];
        int64_t num_slices = src->data.size() / (src->shape[axis] * slice_size);
        for (int64_t s = 0; s < num_slices; s++) {
            for (int64_t i = 0; i < dim_size; i++) {
                for (int64_t j = 0; j < slice_size; j++) {
                    int64_t src_idx = s * src->shape[axis] * slice_size + (start + i * step) * slice_size + j;
                    int64_t dst_idx = s * dim_size * slice_size + i * slice_size + j;
                    result->data[dst_idx] = src->data[src_idx];
                }
            }
        }
    }
    return result;
}

void* numpy_add(void* a_ptr, void* b_ptr) {
    NDArray* a = (NDArray*)a_ptr;
    NDArray* b = (NDArray*)b_ptr;
    NDArray* r;
    if (a->data.size() >= b->data.size()) {
        r = create_array(a->shape.data(), a->ndim);
        for (size_t i = 0; i < r->data.size(); i++) {
            r->data[i] = a->data[i] + b->data[i % b->data.size()];
        }
    } else {
        r = create_array(b->shape.data(), b->ndim);
        for (size_t i = 0; i < r->data.size(); i++) {
            r->data[i] = a->data[i % a->data.size()] + b->data[i];
        }
    }
    return r;
}

void* numpy_sub(void* a_ptr, void* b_ptr) {
    NDArray* a = (NDArray*)a_ptr;
    NDArray* b = (NDArray*)b_ptr;
    NDArray* r;
    if (a->data.size() >= b->data.size()) {
        r = create_array(a->shape.data(), a->ndim);
        for (size_t i = 0; i < r->data.size(); i++) {
            r->data[i] = a->data[i] - b->data[i % b->data.size()];
        }
    } else {
        r = create_array(b->shape.data(), b->ndim);
        for (size_t i = 0; i < r->data.size(); i++) {
            r->data[i] = a->data[i % a->data.size()] - b->data[i];
        }
    }
    return r;
}

void* numpy_mul(void* a_ptr, void* b_ptr) {
    NDArray* a = (NDArray*)a_ptr;
    NDArray* b = (NDArray*)b_ptr;
    NDArray* r;
    if (a->data.size() >= b->data.size()) {
        r = create_array(a->shape.data(), a->ndim);
        for (size_t i = 0; i < r->data.size(); i++) {
            r->data[i] = a->data[i] * b->data[i % b->data.size()];
        }
    } else {
        r = create_array(b->shape.data(), b->ndim);
        for (size_t i = 0; i < r->data.size(); i++) {
            r->data[i] = a->data[i % a->data.size()] * b->data[i];
        }
    }
    return r;
}

void* numpy_div(void* a_ptr, void* b_ptr) {
    NDArray* a = (NDArray*)a_ptr;
    NDArray* b = (NDArray*)b_ptr;
    NDArray* r;
    if (a->data.size() >= b->data.size()) {
        r = create_array(a->shape.data(), a->ndim);
        for (size_t i = 0; i < r->data.size(); i++) {
            r->data[i] = a->data[i] / b->data[i % b->data.size()];
        }
    } else {
        r = create_array(b->shape.data(), b->ndim);
        for (size_t i = 0; i < r->data.size(); i++) {
            r->data[i] = a->data[i % a->data.size()] / b->data[i];
        }
    }
    return r;
}

void* numpy_matmul(void* a_ptr, void* b_ptr);

void* numpy_dot(void* a_ptr, void* b_ptr) {
    NDArray* a = (NDArray*)a_ptr;
    NDArray* b = (NDArray*)b_ptr;
    if (a->ndim == 1 && b->ndim == 1) {
        int64_t shape[1] = {1};
        NDArray* r = create_array(shape, 1);
        r->data[0] = 0;
        for (size_t i = 0; i < a->data.size(); i++) {
            r->data[0] += a->data[i] * b->data[i];
        }
        return r;
    }
    if (a->ndim == 2 && b->ndim == 2) {
        return numpy_matmul(a_ptr, b_ptr);
    }
    return nullptr;
}

void* numpy_matmul(void* a_ptr, void* b_ptr) {
    NDArray* a = (NDArray*)a_ptr;
    NDArray* b = (NDArray*)b_ptr;
    if (a->ndim == 2 && b->ndim == 2) {
        int64_t m = a->shape[0], k = a->shape[1], n = b->shape[1];
        int64_t shape[2] = {m, n};
        NDArray* r = create_array(shape, 2);
        for (int64_t i = 0; i < m; i++) {
            for (int64_t j = 0; j < n; j++) {
                double sum = 0;
                for (int64_t t = 0; t < k; t++) {
                    sum += a->data[i * k + t] * b->data[t * n + j];
                }
                r->data[i * n + j] = sum;
            }
        }
        return r;
    }
    if (a->ndim == 1 && b->ndim == 2) {
        int64_t k = a->shape[0], n = b->shape[1];
        int64_t shape[1] = {n};
        NDArray* r = create_array(shape, 1);
        for (int64_t j = 0; j < n; j++) {
            double sum = 0;
            for (int64_t t = 0; t < k; t++) {
                sum += a->data[t] * b->data[t * n + j];
            }
            r->data[j] = sum;
        }
        return r;
    }
    if (a->ndim == 2 && b->ndim == 1) {
        int64_t m = a->shape[0], k = a->shape[1];
        int64_t shape[1] = {m};
        NDArray* r = create_array(shape, 1);
        for (int64_t i = 0; i < m; i++) {
            double sum = 0;
            for (int64_t t = 0; t < k; t++) {
                sum += a->data[i * k + t] * b->data[t];
            }
            r->data[i] = sum;
        }
        return r;
    }
    return nullptr;
}

void* numpy_sum(void* arr_ptr, int axis) {
    NDArray* a = (NDArray*)arr_ptr;
    if (axis == -1) {
        int64_t shape[1] = {1};
        NDArray* r = create_array(shape, 1);
        r->data[0] = 0;
        for (auto v : a->data) r->data[0] += v;
        return r;
    }
    std::vector<int64_t> out_shape = a->shape;
    out_shape.erase(out_shape.begin() + axis);
    if (out_shape.empty()) out_shape.push_back(1);
    NDArray* r = create_array(out_shape.data(), (int)out_shape.size());
    int64_t outer = 1;
    for (int d = 0; d < axis; d++) outer *= a->shape[d];
    int64_t inner = 1;
    for (int d = axis + 1; d < a->ndim; d++) inner *= a->shape[d];
    int64_t dim = a->shape[axis];
    int64_t r_stride = axis < r->ndim ? r->strides[axis] : 1;
    for (int64_t o = 0; o < outer; o++) {
        for (int64_t i = 0; i < inner; i++) {
            double s = 0;
            for (int64_t d = 0; d < dim; d++) {
                s += a->data[o * dim * inner + d * inner + i];
            }
            r->data[o * inner + i] = s;
        }
    }
    return r;
}

void* numpy_mean(void* arr_ptr, int axis) {
    NDArray* a = (NDArray*)arr_ptr;
    NDArray* s = (NDArray*)numpy_sum(arr_ptr, axis);
    double count = (axis == -1) ? (double)a->data.size() : (double)a->shape[axis];
    for (auto& v : s->data) v /= count;
    return s;
}

void* numpy_std(void* arr_ptr, int axis) {
    NDArray* a = (NDArray*)arr_ptr;
    NDArray* m = (NDArray*)numpy_mean(arr_ptr, axis);
    
    if (axis == -1) {
        double mean = m->data[0];
        double sum = 0;
        for (auto v : a->data) sum += (v - mean) * (v - mean);
        m->data[0] = std::sqrt(sum / a->data.size());
        return m;
    }
    
    int64_t dim = a->shape[axis];
    int64_t outer = 1;
    for (int d = 0; d < axis; d++) outer *= a->shape[d];
    int64_t inner = 1;
    for (int d = axis + 1; d < a->ndim; d++) inner *= a->shape[d];
    
    for (int64_t o = 0; o < outer; o++) {
        for (int64_t i = 0; i < inner; i++) {
            double mean = m->data[o * inner + i];
            double sum = 0;
            for (int64_t d = 0; d < dim; d++) {
                double diff = a->data[o * dim * inner + d * inner + i] - mean;
                sum += diff * diff;
            }
            m->data[o * inner + i] = std::sqrt(sum / dim);
        }
    }
    return m;
}

void* numpy_min(void* arr_ptr, int axis) {
    NDArray* a = (NDArray*)arr_ptr;
    if (axis == -1) {
        int64_t shape[1] = {1};
        NDArray* r = create_array(shape, 1);
        r->data[0] = *std::min_element(a->data.begin(), a->data.end());
        return r;
    }
    std::vector<int64_t> out_shape = a->shape;
    out_shape.erase(out_shape.begin() + axis);
    if (out_shape.empty()) out_shape.push_back(1);
    NDArray* r = create_array(out_shape.data(), (int)out_shape.size());
    int64_t outer = 1;
    for (int d = 0; d < axis; d++) outer *= a->shape[d];
    int64_t inner = 1;
    for (int d = axis + 1; d < a->ndim; d++) inner *= a->shape[d];
    int64_t dim = a->shape[axis];
    for (int64_t o = 0; o < outer; o++) {
        for (int64_t i = 0; i < inner; i++) {
            double v = a->data[o * dim * inner + i];
            for (int64_t d = 1; d < dim; d++) {
                v = std::min(v, a->data[o * dim * inner + d * inner + i]);
            }
            r->data[o * inner + i] = v;
        }
    }
    return r;
}

void* numpy_max(void* arr_ptr, int axis) {
    NDArray* a = (NDArray*)arr_ptr;
    if (axis == -1) {
        int64_t shape[1] = {1};
        NDArray* r = create_array(shape, 1);
        r->data[0] = *std::max_element(a->data.begin(), a->data.end());
        return r;
    }
    std::vector<int64_t> out_shape = a->shape;
    out_shape.erase(out_shape.begin() + axis);
    if (out_shape.empty()) out_shape.push_back(1);
    NDArray* r = create_array(out_shape.data(), (int)out_shape.size());
    int64_t outer = 1;
    for (int d = 0; d < axis; d++) outer *= a->shape[d];
    int64_t inner = 1;
    for (int d = axis + 1; d < a->ndim; d++) inner *= a->shape[d];
    int64_t dim = a->shape[axis];
    for (int64_t o = 0; o < outer; o++) {
        for (int64_t i = 0; i < inner; i++) {
            double v = a->data[o * dim * inner + i];
            for (int64_t d = 1; d < dim; d++) {
                v = std::max(v, a->data[o * dim * inner + d * inner + i]);
            }
            r->data[o * inner + i] = v;
        }
    }
    return r;
}

int64_t numpy_argmin(void* arr_ptr) {
    NDArray* a = (NDArray*)arr_ptr;
    auto it = std::min_element(a->data.begin(), a->data.end());
    return (int64_t)(it - a->data.begin());
}

int64_t numpy_argmax(void* arr_ptr) {
    NDArray* a = (NDArray*)arr_ptr;
    auto it = std::max_element(a->data.begin(), a->data.end());
    return (int64_t)(it - a->data.begin());
}

void* numpy_exp(void* arr_ptr) {
    NDArray* a = (NDArray*)arr_ptr;
    NDArray* r = create_array(a->shape.data(), a->ndim);
    for (size_t i = 0; i < r->data.size(); i++) {
        r->data[i] = std::exp(a->data[i]);
    }
    return r;
}

void* numpy_log(void* arr_ptr) {
    NDArray* a = (NDArray*)arr_ptr;
    NDArray* r = create_array(a->shape.data(), a->ndim);
    for (size_t i = 0; i < r->data.size(); i++) {
        r->data[i] = std::log(a->data[i]);
    }
    return r;
}

void* numpy_sqrt(void* arr_ptr) {
    NDArray* a = (NDArray*)arr_ptr;
    NDArray* r = create_array(a->shape.data(), a->ndim);
    for (size_t i = 0; i < r->data.size(); i++) {
        r->data[i] = std::sqrt(a->data[i]);
    }
    return r;
}

void* numpy_sin(void* arr_ptr) {
    NDArray* a = (NDArray*)arr_ptr;
    NDArray* r = create_array(a->shape.data(), a->ndim);
    for (size_t i = 0; i < r->data.size(); i++) {
        r->data[i] = std::sin(a->data[i]);
    }
    return r;
}

void* numpy_cos(void* arr_ptr) {
    NDArray* a = (NDArray*)arr_ptr;
    NDArray* r = create_array(a->shape.data(), a->ndim);
    for (size_t i = 0; i < r->data.size(); i++) {
        r->data[i] = std::cos(a->data[i]);
    }
    return r;
}

void* numpy_tan(void* arr_ptr) {
    NDArray* a = (NDArray*)arr_ptr;
    NDArray* r = create_array(a->shape.data(), a->ndim);
    for (size_t i = 0; i < r->data.size(); i++) {
        r->data[i] = std::tan(a->data[i]);
    }
    return r;
}

void* numpy_abs(void* arr_ptr) {
    NDArray* a = (NDArray*)arr_ptr;
    NDArray* r = create_array(a->shape.data(), a->ndim);
    for (size_t i = 0; i < r->data.size(); i++) {
        r->data[i] = std::abs(a->data[i]);
    }
    return r;
}

void* numpy_floor(void* arr_ptr) {
    NDArray* a = (NDArray*)arr_ptr;
    NDArray* r = create_array(a->shape.data(), a->ndim);
    for (size_t i = 0; i < r->data.size(); i++) {
        r->data[i] = std::floor(a->data[i]);
    }
    return r;
}

void* numpy_ceil(void* arr_ptr) {
    NDArray* a = (NDArray*)arr_ptr;
    NDArray* r = create_array(a->shape.data(), a->ndim);
    for (size_t i = 0; i < r->data.size(); i++) {
        r->data[i] = std::ceil(a->data[i]);
    }
    return r;
}

void* numpy_clip(void* arr_ptr, double min, double max) {
    NDArray* a = (NDArray*)arr_ptr;
    NDArray* r = create_array(a->shape.data(), a->ndim);
    for (size_t i = 0; i < r->data.size(); i++) {
        double v = a->data[i];
        if (v < min) v = min;
        if (v > max) v = max;
        r->data[i] = v;
    }
    return r;
}

void* numpy_where(void* cond_ptr, void* x_ptr, void* y_ptr) {
    NDArray* cond = (NDArray*)cond_ptr;
    NDArray* x = (NDArray*)x_ptr;
    NDArray* y = (NDArray*)y_ptr;
    NDArray* r = create_array(x->shape.data(), x->ndim);
    for (size_t i = 0; i < r->data.size(); i++) {
        r->data[i] = (cond->data[i % cond->data.size()] != 0.0) ? x->data[i] : y->data[i % y->data.size()];
    }
    return r;
}

void* numpy_sort(void* arr_ptr, int axis) {
    NDArray* a = (NDArray*)arr_ptr;
    NDArray* r = create_array(a->shape.data(), a->ndim);
    r->data = a->data;
    
    if (a->ndim == 1 || axis == -1) {
        std::sort(r->data.begin(), r->data.end());
    } else if (a->ndim == 2) {
        int64_t rows = a->shape[0], cols = a->shape[1];
        if (axis == 1) {
            for (int64_t i = 0; i < rows; i++) {
                std::sort(r->data.begin() + i * cols, r->data.begin() + (i + 1) * cols);
            }
        } else if (axis == 0) {
            for (int64_t j = 0; j < cols; j++) {
                std::vector<double> col(rows);
                for (int64_t i = 0; i < rows; i++) col[i] = r->data[i * cols + j];
                std::sort(col.begin(), col.end());
                for (int64_t i = 0; i < rows; i++) r->data[i * cols + j] = col[i];
            }
        }
    }
    return r;
}

void* numpy_unique(void* arr_ptr) {
    NDArray* a = (NDArray*)arr_ptr;
    std::vector<double> sorted = a->data;
    std::sort(sorted.begin(), sorted.end());
    auto last = std::unique(sorted.begin(), sorted.end());
    sorted.erase(last, sorted.end());
    int64_t shape[1] = {(int64_t)sorted.size()};
    NDArray* r = create_array(shape, 1);
    r->data = sorted;
    return r;
}

char* numpy_to_string(void* arr_ptr) {
    NDArray* a = (NDArray*)arr_ptr;
    std::ostringstream oss;
    oss << "ndarray([";
    if (a->ndim == 1) {
        for (size_t i = 0; i < a->data.size(); i++) {
            if (i > 0) oss << ", ";
            oss << a->data[i];
        }
    } else if (a->ndim == 2) {
        for (int64_t i = 0; i < a->shape[0]; i++) {
            if (i > 0) oss << ",\n         ";
            oss << "[";
            for (int64_t j = 0; j < a->shape[1]; j++) {
                if (j > 0) oss << ", ";
                oss << a->data[i * a->shape[1] + j];
            }
            oss << "]";
        }
    } else {
        for (size_t i = 0; i < a->data.size(); i++) {
            if (i > 0) oss << ", ";
            oss << a->data[i];
        }
    }
    oss << "])";
    std::string s = oss.str();
    char* cstr = (char*)std::malloc(s.size() + 1);
    std::memcpy(cstr, s.c_str(), s.size() + 1);
    return cstr;
}

void numpy_shape(void* arr_ptr, int64_t* out_dims) {
    NDArray* a = (NDArray*)arr_ptr;
    for (int i = 0; i < a->ndim; i++) {
        out_dims[i] = a->shape[i];
    }
}

int numpy_ndim(void* arr_ptr) {
    return ((NDArray*)arr_ptr)->ndim;
}

int64_t numpy_size(void* arr_ptr) {
    return (int64_t)((NDArray*)arr_ptr)->data.size();
}

double numpy_item(void* arr_ptr, const int64_t* indices, int num_indices) {
    NDArray* a = (NDArray*)arr_ptr;
    int64_t idx = flat_index(a, indices, num_indices);
    return a->data[idx];
}

void numpy_set_item(void* arr_ptr, const int64_t* indices, int num_indices, double value) {
    NDArray* a = (NDArray*)arr_ptr;
    int64_t idx = flat_index(a, indices, num_indices);
    a->data[idx] = value;
}

void numpy_to_float_array(void* arr_ptr, double* out_data) {
    NDArray* a = (NDArray*)arr_ptr;
    std::memcpy(out_data, a->data.data(), a->data.size() * sizeof(double));
}

void* numpy_from_float_array(const double* data, const int64_t* shape, int ndim) {
    NDArray* arr = create_array(shape, ndim);
    std::memcpy(arr->data.data(), data, arr->data.size() * sizeof(double));
    return arr;
}

void numpy_free(void* arr_ptr) {
    delete (NDArray*)arr_ptr;
}

void numpy_free_string(char* s) {
    std::free(s);
}

}
