import api from "../api/api";
import Product from "../models/Product";

export default {
    async getProducts(): Promise<Product[]> {
        const response = await api.get("/products");
        return response.data as Product[];
    },

    async getProduct(id: number): Promise<Product> {
        const response = await api.get(`/product/${id}`);
        return response.data as Product;
    },

    async createProduct(product: Product): Promise<Product> {
        const response = await api.post("/products", product);
        return response.data as Product;
    },

    async updateProduct(product: Product): Promise<boolean> {
        const response = await api.put(`/product/${product.id}`, product);
        return response.status === 200;
    },

    async deleteProduct(id: number): Promise<boolean> {
        const response = await api.put(`/product/${id}`);
        return response.status === 200;
    },
};
