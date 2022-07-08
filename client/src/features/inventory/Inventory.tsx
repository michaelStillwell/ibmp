import useProductList from "../../hooks/useProductList";

export async function Inventory() {
    const { productList } = await useProductList();

    return (
        <>
            <div>
                <ul>
                    {productList.map((product) => (
                        <li>{product.name}</li>
                    ))}
                </ul>
            </div>
        </>
    );
}
