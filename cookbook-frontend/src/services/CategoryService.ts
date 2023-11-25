import http from '../http-common';

class CategoryService {
    getAll(): Promise<any> {
        return http.get("/categories");
      }
}

export default new CategoryService();