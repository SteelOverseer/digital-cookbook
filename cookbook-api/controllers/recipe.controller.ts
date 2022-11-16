import { prisma } from '../db'

module.exports = app => {
  
    // Create a new Category
    app.post(`/createRecipe`, async (req, res) => {
      const { categoryId, name, tags, notes } = req.body
      const result = await prisma.recipes.create({
        data: {
          categoryId,
          name,
          tags,
          notes
        },
      }) 
      res.json(result)
    })
  
    // Retrieve all Category
    app.get('/recipes', async (req, res) => {

      const result = await prisma.recipes.findMany()

      res.json(result)
    })
  
    // Update a Category with id
    app.put('/updateRecipe/:id', async (req, res) => {
      const { id } = req.params
      const { categoryId, name, tags, notes } = req.body
      const result = await prisma.recipes.update({
        where: { recipeId: Number(id) },
        data: {
          categoryId,
          name,
          tags,
          notes
        },
      })
      res.json(result)
    })
  
    // Delete a Category with id
    app.delete(`/deleteRecipe/:id`, async (req, res) => {
      const { id } = req.params
      const result = await prisma.recipes.delete({
        where: {
          recipeId: Number(id),
        },
      })
      res.json(result)
    })
    
  };