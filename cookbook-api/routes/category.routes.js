const { categories } = require("../models/index.js");

module.exports = app => {
    const categories = require("../controllers/category.controller.js");
  
    var router = require("express").Router();
  
    // Create a new Tutorial
    // router.post("/", tutorials.create);
  
    // Retrieve all Tutorials
    router.get("/", categories.findAll);
  
    // Retrieve all published Tutorials
    // router.get("/published", tutorials.findAllPublished);
  
    // Retrieve a single Tutorial with id
    // router.get("/:id", tutorials.findOne);
  
    // Update a Tutorial with id
    // router.put("/:id", tutorials.update);
  
    // Delete a Tutorial with id
    // router.delete("/:id", tutorials.delete);
  
    // Delete all Tutorials
    // router.delete("/", tutorials.deleteAll);
  
    app.use('/api/categories', router);
  };