module.exports = app => {
    const categories = require("../controllers/category.controller.ts");
  
    // var router = require("express").Router();
  
    // Create a new Category
    // router.post("/:id", categories.create);
  
    // Retrieve all Category
    app.get('/categories', async (req, res) => {

      const categories = await prisma.categories.findMany()

      res.json(categories)
    })
  
    // Update a Category with id
    // router.put("/:id", categories.update);
  
    // Delete a Category with id
    // router.delete("/:id", categories.delete);

  
    // app.use('/api/categories', router);
  };