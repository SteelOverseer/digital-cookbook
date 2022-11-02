const dbConfig = require("../config/db.config.js");
const Sequelize = require('sequelize');

// const sequelize = new Sequelize('postgres://postgres:postgres@192.168.1.13:5433/cookbook') // Example for postgres
const sequelize = new Sequelize(dbConfig.DB, dbConfig.USER, dbConfig.PASSWORD, {
    host: dbConfig.HOST,
    dialect: dbConfig.dialect,
    port: dbConfig.PORT,
    pool: {
      max: dbConfig.pool.max,
      min: dbConfig.pool.min,
      acquire: dbConfig.pool.acquire,
      idle: dbConfig.pool.idle
    }
});
const db = {};

db.Sequelize = Sequelize;
db.sequelize = sequelize;

db.categories = require("./category.model.js")(sequelize, Sequelize);

module.exports = db;