/*
/ Project.ts
/
/ File containing functions and logic required for frontend handling of projects
/ Will provide the communication with the backend and pass necessary information to API calls
/
/ Summary:
/ Class Project: Mapper of a class to how we are storing projects in db
/ get_all_projects: Function to get all projects for the current user
/ get_project: Function to get a specific project by ID
/ create_project: Function to create a new project
/ update_project: Function to update an existing project
/ delete_project: Function to delete a project
/ force_delete_project: Function to force delete a project and all its documents
/ add_project_permissions: Function to add permissions for a user on a project
/ get_project_permissions: Function to get all users with permissions on a project
/ update_project_permission: Function to update a user's permission on a project
/ remove_project_permissions: Function to remove a user's permission from a project
/ get_project_documents: Function to get all documents in a project
/ add_document_to_project: Function to add a document to a project
/ remove_document_from_project: Function to remove a document from a project
/
*/

export class Project {
	id: number;
	name: string;
	user_id?: number;

	constructor(new_id: number, new_name: string, new_user_id?: number) {
		this.id = new_id;
		this.name = new_name;
		this.user_id = new_user_id;
	}
}

// Define a User type for project permissions
export class ProjectUser {
	id: number;
	name: string;
	email: string;
	role: string;

	constructor(new_id: number, new_name: string, new_email: string, new_role: string) {
		this.id = new_id;
		this.name = new_name;
		this.email = new_email;
		this.role = new_role;
	}
}

// TODO Function to get a project
export async function get_project(project_id: number): Promise<Project | null> {
	const apiUrl = `http://localhost:3001/api/project/${project_id}`;

	try {
		const response = await fetch(apiUrl, {
			method: 'GET',
			credentials: 'include'
		});

		if (!response.ok) {
			console.error('Get project failed with status:', response.status);
			return null;
		}

		const project = await response.json();

		return project;
	} catch (error) {
		console.error('Get project error:', error);
		return null;
	}
}

/**
 * Function to add permissions for a user on a project
 */
export async function add_project_permissions(projectId: number, userId: number, role: string): Promise<boolean> {
	try {
		const apiUrl = `http://localhost:3001/api/project/${projectId}/permissions`;
		
		const payload = {
			user_id: userId,
			role: role
		};
		
		const response = await fetch(apiUrl, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(payload),
			credentials: 'include'
		});
		
		return response.ok;
	} catch (error) {
		console.error('Error adding project permissions:', error);
		return false;
	}
}

/**
 * Function to get all users with permissions on a project
 */
export async function get_project_permissions(projectId: number): Promise<ProjectUser[] | null> {
	try {
		const apiUrl = `http://localhost:3001/api/project/${projectId}/permissions`;
		
		const response = await fetch(apiUrl, {
			credentials: 'include'
		});
		
		if (!response.ok) {
			console.error('Failed to fetch project permissions:', response.status);
			return null;
		}
		
		const data = await response.json();
		return data.users || null;
	} catch (error) {
		console.error('Error fetching project permissions:', error);
		return null;
	}
}

/**
 * Function to update a user's permission on a project
 */
export async function update_project_permission(projectId: number, userId: number, role: string): Promise<boolean> {
	try {
		const apiUrl = `http://localhost:3001/api/project/${projectId}/permissions`;
		
		const payload = {
			user_id: userId,
			role: role
		};
		
		const response = await fetch(apiUrl, {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(payload),
			credentials: 'include'
		});
		
		return response.ok;
	} catch (error) {
		console.error('Error updating project permission:', error);
		return false;
	}
}

/**
 * Function to remove a user's permission from a project
 */
export async function remove_project_permissions(projectId: number, userId: number): Promise<boolean> {
	try {
		const apiUrl = `http://localhost:3001/api/project/${projectId}/permissions/${userId}`;
		
		const response = await fetch(apiUrl, {
			method: 'DELETE',
			credentials: 'include'
		});
		
		return response.ok;
	} catch (error) {
		console.error('Error removing project permissions:', error);
		return false;
	}
}

/**
 * Function to get all documents in a project
 */
export async function get_project_documents(projectId: number): Promise<Document[] | null> {
	try {
		const apiUrl = `http://localhost:3001/api/project/${projectId}/documents`;
		
		const response = await fetch(apiUrl, {
			credentials: 'include'
		});
		
		if (!response.ok) {
			console.error('Failed to fetch project documents:', response.status);
			return null;
		}
		
		return await response.json();
	} catch (error) {
		console.error('Error fetching project documents:', error);
		return null;
	}
}

/**
 * Function to add a document to a project
 */
export async function add_document_to_project(projectId: number, documentId: number): Promise<boolean> {
	try {
		const apiUrl = `http://localhost:3001/api/project/${projectId}/documents/${documentId}`;
		
		const response = await fetch(apiUrl, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({}),
			credentials: 'include'
		});
		
		return response.ok;
	} catch (error) {
		console.error('Error adding document to project:', error);
		return false;
	}
}

/**
 * Function to remove a document from a project
 */
export async function remove_document_from_project(projectId: number, documentId: number): Promise<boolean> {
	try {
		const apiUrl = `http://localhost:3001/api/project/${projectId}/documents/${documentId}`;
		
		const response = await fetch(apiUrl, {
			method: 'DELETE',
			credentials: 'include'
		});
		
		return response.ok;
	} catch (error) {
		console.error('Error removing document from project:', error);
		return false;
	}
}
