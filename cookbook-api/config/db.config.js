module.exports = {
    HOST: "192.168.1.13",
    PORT: 5433,
    USER: "postgres",
    PASSWORD: "postgres",
    DB: "cookbook",
    dialect: "postgres",
    pool: {
      max: 5,
      min: 0,
      acquire: 30000,
      idle: 10000
    }
  };