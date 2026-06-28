#include <torch/torch.h>
#include <cstring>
#include <string>
#include <vector>
#include <sstream>
#include <cstdlib>

extern "C" {

int torch_init() {
    return 0;
}

static torch::TensorOptions dtype_opts(int dtype) {
    switch (dtype) {
        case 0: return torch::TensorOptions().dtype(torch::kFloat32);
        case 1: return torch::TensorOptions().dtype(torch::kFloat64);
        case 2: return torch::TensorOptions().dtype(torch::kInt64);
        case 3: return torch::TensorOptions().dtype(torch::kInt32);
        case 4: return torch::TensorOptions().dtype(torch::kBool);
        default: return torch::TensorOptions().dtype(torch::kFloat32);
    }
}

void* torch_tensor_create(const float* data, const int64_t* dims, int ndim, int dtype) {
    std::vector<int64_t> sizes(dims, dims + ndim);
    at::Tensor tensor;
    if (data) {
        tensor = torch::from_blob((void*)data, sizes, torch::kFloat32).clone();
        if (dtype != 0) {
            tensor = tensor.to(dtype_opts(dtype));
        }
    } else {
        tensor = torch::empty(sizes, dtype_opts(dtype));
    }
    return new at::Tensor(tensor);
}

void* torch_tensor_from_numpy(const float* data, const int64_t* shape, int ndim) {
    std::vector<int64_t> sizes(shape, shape + ndim);
    at::Tensor tensor = torch::from_blob((void*)data, sizes, torch::kFloat32).clone();
    return new at::Tensor(tensor);
}

void torch_tensor_fill(void* tensor, float value) {
    (*(at::Tensor*)tensor).fill_(value);
}

void* torch_tensor_zeros(const int64_t* shape, int ndim) {
    std::vector<int64_t> sizes(shape, shape + ndim);
    return new at::Tensor(torch::zeros(sizes));
}

void* torch_tensor_ones(const int64_t* shape, int ndim) {
    std::vector<int64_t> sizes(shape, shape + ndim);
    return new at::Tensor(torch::ones(sizes));
}

void* torch_tensor_rand(const int64_t* shape, int ndim) {
    std::vector<int64_t> sizes(shape, shape + ndim);
    return new at::Tensor(torch::rand(sizes));
}

void* torch_tensor_clone(void* tensor) {
    return new at::Tensor((*(at::Tensor*)tensor).clone());
}

void* torch_tensor_add(void* a, void* b) {
    return new at::Tensor((*(at::Tensor*)a) + (*(at::Tensor*)b));
}

void* torch_tensor_sub(void* a, void* b) {
    return new at::Tensor((*(at::Tensor*)a) - (*(at::Tensor*)b));
}

void* torch_tensor_mul(void* a, void* b) {
    return new at::Tensor((*(at::Tensor*)a) * (*(at::Tensor*)b));
}

void* torch_tensor_div(void* a, void* b) {
    return new at::Tensor((*(at::Tensor*)a) / (*(at::Tensor*)b));
}

void* torch_tensor_matmul(void* a, void* b) {
    return new at::Tensor((*(at::Tensor*)a).matmul(*(at::Tensor*)b));
}

void* torch_tensor_relu(void* tensor) {
    return new at::Tensor(torch::relu(*(at::Tensor*)tensor));
}

void* torch_tensor_sigmoid(void* tensor) {
    return new at::Tensor(torch::sigmoid(*(at::Tensor*)tensor));
}

void* torch_tensor_tanh(void* tensor) {
    return new at::Tensor(torch::tanh(*(at::Tensor*)tensor));
}

void* torch_tensor_softmax(void* tensor, int dim) {
    return new at::Tensor(torch::softmax(*(at::Tensor*)tensor, dim));
}

void* torch_tensor_sum(void* tensor, int dim) {
    if (dim == -1) {
        return new at::Tensor((*(at::Tensor*)tensor).sum());
    }
    return new at::Tensor((*(at::Tensor*)tensor).sum(dim));
}

void* torch_tensor_mean(void* tensor, int dim) {
    if (dim == -1) {
        return new at::Tensor((*(at::Tensor*)tensor).mean());
    }
    return new at::Tensor((*(at::Tensor*)tensor).mean(dim));
}

void* torch_tensor_reshape(void* tensor, const int64_t* shape, int ndim) {
    std::vector<int64_t> sizes(shape, shape + ndim);
    return new at::Tensor((*(at::Tensor*)tensor).reshape(sizes));
}

void* torch_tensor_view(void* tensor, const int64_t* shape, int ndim) {
    std::vector<int64_t> sizes(shape, shape + ndim);
    return new at::Tensor((*(at::Tensor*)tensor).view(sizes));
}

char* torch_tensor_to_string(void* tensor) {
    std::ostringstream oss;
    oss << (*(at::Tensor*)tensor);
    std::string s = oss.str();
    char* cstr = (char*)std::malloc(s.size() + 1);
    std::memcpy(cstr, s.c_str(), s.size() + 1);
    return cstr;
}

int torch_tensor_dim(void* tensor) {
    return (*(at::Tensor*)tensor).dim();
}

void torch_tensor_sizes(void* tensor, int64_t* out_dims) {
    auto& t = *(at::Tensor*)tensor;
    for (int64_t i = 0; i < t.dim(); i++) {
        out_dims[i] = t.size(i);
    }
}

double torch_tensor_item(void* tensor) {
    return (*(at::Tensor*)tensor).item<double>();
}

void torch_tensor_to_float_array(void* tensor, float* out_data, int64_t* out_len) {
    auto& t = *(at::Tensor*)tensor;
    auto flat = t.contiguous().flatten();
    if (flat.dtype() != torch::kFloat32) {
        flat = flat.to(torch::kFloat32);
    }
    *out_len = flat.numel();
    std::memcpy(out_data, flat.data_ptr<float>(), *out_len * sizeof(float));
}

void torch_tensor_free(void* tensor) {
    delete (at::Tensor*)tensor;
}

void torch_tensor_requires_grad(void* tensor, int req) {
    (*(at::Tensor*)tensor).set_requires_grad(req != 0);
}

void torch_tensor_backward(void* tensor) {
    (*(at::Tensor*)tensor).backward();
}

void* torch_tensor_grad(void* tensor) {
    auto grad = (*(at::Tensor*)tensor).grad();
    if (!grad.defined()) return nullptr;
    return new at::Tensor(grad);
}

struct LinearModule {
    torch::nn::Linear linear;
    LinearModule(int in, int out, bool bias) : linear(in, out, bias) {}
};

void* torch_linear_create(int in_features, int out_features, int bias) {
    return new LinearModule(in_features, out_features, bias != 0);
}

void* torch_linear_forward(void* module, void* input) {
    auto mod = (LinearModule*)module;
    return new at::Tensor(mod->linear->forward(*(at::Tensor*)input));
}

void torch_module_free(void* module) {
    delete (LinearModule*)module;
}

void torch_free_string(char* s) {
    std::free(s);
}

}
