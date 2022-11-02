module.exports = (sequelize, Sequelize) => {
	const Category = sequelize.define('category', {
		category_id: {
			allowNull: false,
			autoIncrement: true,
			primaryKey: true,
			type: Sequelize.INTEGER
		},
		name: {
			allowNull: false,
			type: Sequelize.STRING,
		},
	});

    return Category;
};