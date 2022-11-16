import express from 'express'

const app = express()

app.use(express.json())

require("./controllers/category.controller")(app);
require("./controllers/recipe.controller")(app);
require("./controllers/ingredient.controller")(app);
require("./controllers/step.controller")(app);

app.listen(3000, ()=> console.log(`Server is running on port 3000`))