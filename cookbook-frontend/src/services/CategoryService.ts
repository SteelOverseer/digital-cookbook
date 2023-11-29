import http from '../http-common';

class CategoryService {
	getAll(): Promise<any> {
		return http.get("/categories");
	}

	createCategory(categoryName: string): Promise<any> {
		return http.post("/category", {name: categoryName})
	}
}

export default new CategoryService();