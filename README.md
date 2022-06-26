# Inventory Based Meal Planning App

> Name is TBD


## Description
This app is to help meal in a budget-friendly way by prioritizing current
inventory before buying new food.


### MVP

#### Backend API

- [ ] cru products
- [ ] cru recipes
- [ ] cru ingredients
- [ ] search recipes by list of ingredients
  - [ ] prioritizing less ingredients needed

#### Frontend GUI

- [ ] input your current inventory of ingredients from the known products
- [ ] display all recipes that include the products
- [ ] choose recipes to add to your meal plan
  - [ ] update your current inventory's stock of products
- [ ] save recipes and shopping list of needed ingredients to file
  - [ ] subtracts the ingredients you have from the needed so you get an accurate
        vision of what you need to buy


### Data Models

Recipe
 - id
 - name
 - description
 - directions
 - picture (?)
 - ingredients

Product
 - id
 - name
 - picture (?)
 - TBD

Ingredient
 - id
 - product_id -> Product.id
 - recipe_id -> Recipe.id
 - measurement -> Measurement.id
 - amount

Measurement
 - id
 - type
 - conversion? -> TBD


### Project Structure

server - backend api for project

client - frontend ui for project


### Technologies

Rust, Actix, PostgresQL, (db-access), TBD
Vite, React, Typescript, CSS modules, TBD

