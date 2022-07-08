import { useEffect, useState } from "react";
import api from "../api/api";
import Product from "../models/Product";

interface Data<T> {
    slug: string;
    results: T[];
}

const useProductList = async () => {
    let [productList, setProductList] = useState<Product[]>([]);

    let { data } = await api.get("/products");
    setProductList({ ...data });

    return { productList };
};

export default useProductList;
