import { prisma } from '../db'

module.exports = app => {
  
    // Create a new Category
    app.post(`/createIngredient`, async (req, res) => {
      const { recipeId, ingredientName, quantity } = req.body
      const result = await prisma.ingredients.create({
        data: {
          recipeId,
          quantity,
          ingredientName
        },
      }) 
      res.json(result)
    })
  
    // Retrieve all Category
    app.get('/ingredients', async (req, res) => {

      const result = await prisma.ingredients.findMany()

      res.json(result)
    })
  
    // Update a Category with id
    app.put('/updateIngredient/:id', async (req, res) => {
      const { id } = req.params
      const { recipeId, ingredientName, quantity } = req.body
      const result = await prisma.ingredients.update({
        where: { ingredientId: Number(id) },
        data: {
          recipeId,
          quantity,
          ingredientName
        },
      })
      res.json(result)
    })
  
    // Delete a Category with id
    app.delete(`/deleteIngredient/:id`, async (req, res) => {
      const { id } = req.params
      const result = await prisma.ingredients.delete({
        where: {
          ingredientId: Number(id),
        },
      })
      res.json(result)
    })
    
  };