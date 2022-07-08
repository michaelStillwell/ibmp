# Arch design

### Feature

An over-arching container that houses orchestrated sub-features, this is where dependencies are decided.

### Sub-feature

A _Sub-feature_ is a set of two parts:

-   a backend _Endpoint_
-   a ui _Component_

With these parts a piece of functionality is created.

### Endpoint

A _Endpoint_ is a backend implementation of a _Sub-feature_ that allows it to persist.

### Component

A _Component_ is a frontend part that holds the visualization for the _Sub-feature_.

===

## Examples

**Get product list**, _Sub-feature_:

-   _Endpoint_: /product/[all|{id}] -> list of products
-   _Component_: ProductList -> `ul` with styled `li` elements displaying product info
