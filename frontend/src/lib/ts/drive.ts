/*
/ Drive.ts
/
/ File containing functions and logic required for frontend handling of a drive
/ Will provide the communication with the backend and pass necessary information to API calls
/
/ Summary:
/
/
/
*/

// TODO Function to create a document
export async function create_document(document: Document): Promise<Boolean> {
	// Api URL
	const apiUrl = `http://localhost:3001/api/document/`;

	// attempt to call POST API
	try {
		const response = await fetch(apiUrl, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(login_payload),
			credentials: 'include'
		});

		// Check if the response is successful
		if (response.ok) {
			return true;
		} else {
			console.error('Login failed with status:', response.status);
			const errorText = await response.text();
			console.error('Error response:', errorText);
			return false;
		}
	} catch (error) {
		console.error('Login request error:', error);
		return false;
	}

	return true;
}

// TODO Function to get all documents
export async function get_all_documents(): Promise<Boolean> {
	// Api URL
	const apiUrl = `http://localhost:3001/api/users`;
	// create payload

	// Call API

	return true;
}

// TODO Function to delete document
export async function delete_document(): Promise<Boolean> {
	// Api URL
	const apiUrl = `http://localhost:3001/api/users`;
	// create payload

	// Call API

	return true;
}

// TODO Function to create a project
export async function create_project(): Promise<Boolean> {
	// Api URL
	const apiUrl = `http://localhost:3001/api/users`;
	// create payload

	// Call API

	return true;
}

// TODO Function to get a project
export async function get_project(): Promise<Boolean> {
	// Api URL
	const apiUrl = `http://localhost:3001/api/users`;
	// create payload

	// Call API

	return true;
}

// TODO Function to get a project
export async function get_all_projects(): Promise<Boolean> {
	// Api URL
	const apiUrl = `http://localhost:3001/api/users`;
	// create payload

	// Call API

	return true;
}

// TODO Function to update a project
export async function update_project(): Promise<Boolean> {
	// Api URL
	const apiUrl = `http://localhost:3001/api/users`;
	// create payload

	// Call API

	return true;
}

// TODO Function to delete a project
export async function delete_project(): Promise<Boolean> {
	// Api URL
	const apiUrl = `http://localhost:3001/api/users`;
	// create payload

	// Call API

	return true;
}

// TODO Function to delete a project and all its documents
export async function force_delete_project(): Promise<Boolean> {
	// Api URL
	const apiUrl = `http://localhost:3001/api/users`;
	// create payload

	// Call API

	return true;
}
