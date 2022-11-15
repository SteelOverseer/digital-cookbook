import { PrismaClient } from "@prisma/client"
import express from 'express'

const prisma = new PrismaClient()
const app = express()

app.use(express.json())

// app.get('/categories', async (req, res) => {

//     const categories = await prisma.categories.findMany()

//     res.json(categories)
// })

// const server = app.listen(3000, () =>
//   console.log(`🚀 Server ready at: http://localhost:3000`),
// )
require("./routes/category.routes")(app);

app.listen(3000, ()=> console.log(`Server is running on port 3000`))