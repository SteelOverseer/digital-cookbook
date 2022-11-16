import { prisma } from '../db'

module.exports = app => {
  
    // Create a new Category
    app.post(`/createStep`, async (req, res) => {
      const { recipeId, stepNumber, instructions } = req.body
      const result = await prisma.steps.create({
        data: {
          recipeId,
          stepNumber,
          instructions
        },
      }) 
      res.json(result)
    })
  
    // Retrieve all Category
    app.get('/steps', async (req, res) => {

      const result = await prisma.steps.findMany()

      res.json(result)
    })
  
    // Update a Category with id
    app.put('/updateStep/:id', async (req, res) => {
      const { id } = req.params
      const { recipeId, stepNumber, instructions } = req.body
      const result = await prisma.steps.update({
        where: { stepId: Number(id) },
        data: {
          recipeId,
          stepNumber,
          instructions
        },
      })
      res.json(result)
    })
  
    // Delete a Category with id
    app.delete(`/deleteStep/:id`, async (req, res) => {
      const { id } = req.params
      const result = await prisma.steps.delete({
        where: {
          stepId: Number(id),
        },
      })
      res.json(result)
    })
    
  };