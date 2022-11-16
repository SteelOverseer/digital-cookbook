import { prisma } from '../db'

module.exports = app => {
  
    // Create a new Category
    app.post(`/createCategory`, async (req, res) => {
      const { name } = req.body
      const result = await prisma.categories.create({
        data: {
          name,
        },
      }) 
      res.json(result)
    })
  
    // Retrieve all Category
    app.get('/categories', async (req, res) => {

      const result = await prisma.categories.findMany()

      res.json(result)
    })
  
    // Update a Category with id
    app.put('/updateCategory/:id', async (req, res) => {
      const { id } = req.params
      const { name } = req.body
      const result = await prisma.categories.update({
        where: { categoryId: Number(id) },
        data: { name: name },
      })
      res.json(result)
    })
  
    // Delete a Category with id
    app.delete(`/deleteCategory/:id`, async (req, res) => {
      const { id } = req.params
      const result = await prisma.categories.delete({
        where: {
          categoryId: Number(id),
        },
      })
      res.json(result)
    })
    
  };