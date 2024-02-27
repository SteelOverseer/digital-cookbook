import http from '../http-common';
import ResponseModel from '../models/ResponseModel';

class CategoryService {
	getAll(): Promise<ResponseModel> {
		return http.get("/categories");
	}

	createCategory(categoryName: string): Promise<ResponseModel> {
		return http.post("/category", {name: categoryName})
	}
}

export default new CategoryService();