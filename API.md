# API Documentation - UniversalPath.org

## Overview

The UniversalPath.org API provides programmatic access to content and functionality, enabling the development of mobile applications and integration with other systems. All responses are in JSON format.

## Base URL

```
https://universalpath.org/api
```

For development and testing:
```
http://localhost:8000/api
```

## Authentication

### API Key Authentication

Most endpoints require an API key, which should be included in all requests as a header:

```
x-api-key: your_api_key_here
```

### JWT Authentication for Admin Operations

Administrative operations require JWT authentication:

1. Obtain a token by calling `/api/auth/login`
2. Include the token in the `Authorization` header for subsequent requests:

```
Authorization: Bearer your_jwt_token_here
```

## Response Format

All API responses follow this structure:

### Success Response

```json
{
  "status": "success",
  "data": {}  // Response data varies by endpoint
}
```

### Error Response

```json
{
  "status": "error",
  "message": "Error description"
}
```

## Endpoints

### Authentication

#### Login

```
POST /auth/login
```

**Request Body:**
```json
{
  "username": "admin",
  "password": "your_password"
}
```

**Response:**
```json
{
  "status": "success",
  "data": "jwt_token_string"
}
```

#### Verify Token

```
GET /auth/verify
```

**Headers:**
- `Authorization: Bearer your_jwt_token`
- `x-api-key: your_api_key`

**Response:**
```json
{
  "status": "success",
  "data": true
}
```

### Articles

#### List Articles

```
GET /articles
```

**Headers:**
- `x-api-key: your_api_key`

**Query Parameters:**
- `limit` (optional): Maximum number of articles to return
- `offset` (optional): Number of articles to skip

**Response:**
```json
{
  "status": "success",
  "data": [
    {
      "article": {
        "id": 1,
        "title": "Название статьи",
        "publish_date": "2025-03-29",
        "resume": "Краткое описание статьи",
        ...
      },
      "author_name": "Имя автора",
      "category_name": "Название категории"
    },
    ...
  ]
}
```

#### Get Article

```
GET /articles/{id}
```

**Headers:**
- `x-api-key: your_api_key`

**Response:**
```json
{
  "status": "success",
  "data": {
    "article": {
      "id": 1,
      "title": "Название статьи",
      "publish_date": "2025-03-29",
      "resume": "Краткое описание",
      "txtfield": "Полный текст статьи",
      ...
    },
    "author_name": "Имя автора",
    "category_name": "Название категории"
  }
}
```

#### Get Articles by Category

```
GET /articles/category/{category_id}
```

**Headers:**
- `x-api-key: your_api_key`

**Query Parameters:**
- `limit` (optional): Maximum number of articles to return
- `offset` (optional): Number of articles to skip

**Response:**
```json
{
  "status": "success",
  "data": [
    {
      "article": {
        "id": 1,
        "title": "Название статьи",
        ...
      },
      "author_name": "Имя автора",
      "category_name": "Название категории"
    },
    ...
  ]
}
```

#### Search Articles

```
GET /articles/search?q={query}
```

**Headers:**
- `x-api-key: your_api_key`

**Response:**
```json
{
  "status": "success",
  "data": [
    {
      "article": {
        "id": 1,
        "title": "Название статьи",
        ...
      },
      "author_name": "Имя автора",
      "category_name": "Название категории"
    },
    ...
  ]
}
```

#### Create Article (Admin Only)

```
POST /articles
```

**Headers:**
- `Authorization: Bearer your_jwt_token`
- `x-api-key: your_api_key`
- `Content-Type: application/json`

**Request Body:**
```json
{
  "title": "Название новой статьи",
  "author_id": 1,
  "category_id": 5,
  "resume": "Краткое описание статьи",
  "txtfield": "Полный текст статьи",
  "publish_date": "2025-03-29",
  "available_on_site": true,
  "available_on_api": true,
  "keywords": "ключевые, слова",
  "description": "Мета-описание для SEO"
}
```

**Response:**
```json
{
  "status": "success",
  "data": 42  // ID созданной статьи
}
```

#### Update Article (Admin Only)

```
PUT /articles/{id}
```

**Headers:**
- `Authorization: Bearer your_jwt_token`
- `x-api-key: your_api_key`
- `Content-Type: application/json`

**Request Body:**
```json
{
  "title": "Обновленное название статьи",
  "resume": "Обновленное краткое описание",
  "txtfield": "Обновленный текст статьи",
  "available_on_site": true
}
```

**Response:**
```json
{
  "status": "success",
  "data": true
}
```

#### Delete Article (Admin Only)

```
DELETE /articles/{id}
```

**Headers:**
- `Authorization: Bearer your_jwt_token`
- `x-api-key: your_api_key`

**Response:**
```json
{
  "status": "success",
  "data": true
}
```

### Categories

#### List Root Categories

```
GET /categories
```

**Headers:**
- `x-api-key: your_api_key`

**Response:**
```json
{
  "status": "success",
  "data": [
    {
      "category": {
        "id": 1,
        "name": "Название категории",
        "title": "Заголовок категории",
        "description": "Описание категории",
        ...
      },
      "article_count": 15,
      "subcategory_count": 3
    },
    ...
  ]
}
```

#### Get Category

```
GET /categories/{id}
```

**Headers:**
- `x-api-key: your_api_key`

**Response:**
```json
{
  "status": "success",
  "data": {
    "category": {
      "id": 1,
      "name": "Название категории",
      "title": "Заголовок категории",
      "description": "Описание категории",
      ...
    },
    "article_count": 15,
    "subcategory_count": 3
  }
}
```

#### Get Subcategories

```
GET /categories/{id}/subcategories
```

**Headers:**
- `x-api-key: your_api_key`

**Response:**
```json
{
  "status": "success",
  "data": [
    {
      "category": {
        "id": 2,
        "name": "Название подкатегории",
        ...
      },
      "article_count": 8,
      "subcategory_count": 0
    },
    ...
  ]
}
```

#### Get Category Tree

```
GET /categories/tree
```

**Headers:**
- `x-api-key: your_api_key`

**Response:**
```json
{
  "status": "success",
  "data": [
    {
      "category": {
        "category": {
          "id": 1,
          "name": "Категория 1",
          ...
        },
        "article_count": 15,
        "subcategory_count": 2
      },
      "children": [
        {
          "category": {
            "category": {
              "id": 2,
              "name": "Подкатегория 1",
              ...
            },
            "article_count": 8,
            "subcategory_count": 0
          },
          "children": []
        },
        ...
      ]
    },
    ...
  ]
}
```

#### Get Category Path

```
GET /categories/{id}/path
```

**Headers:**
- `x-api-key: your_api_key`

**Response:**
```json
{
  "status": "success",
  "data": [
    {
      "id": 1,
      "name": "Родительская категория",
      ...
    },
    {
      "id": 5,
      "name": "Текущая категория",
      ...
    }
  ]
}
```

#### Create Category (Admin Only)

```
POST /categories
```

**Headers:**
- `Authorization: Bearer your_jwt_token`
- `x-api-key: your_api_key`
- `Content-Type: application/json`

**Request Body:**
```json
{
  "name": "Новая категория",
  "title": "Заголовок новой категории",
  "parent_id": 1,
  "description": "Описание новой категории",
  "available": true
}
```

**Response:**
```json
{
  "status": "success",
  "data": 42  // ID созданной категории
}
```

#### Update Category (Admin Only)

```
PUT /categories/{id}
```

**Headers:**
- `Authorization: Bearer your_jwt_token`
- `x-api-key: your_api_key`
- `Content-Type: application/json`

**Request Body:**
```json
{
  "name": "Обновленное имя категории",
  "description": "Обновленное описание категории"
}
```

**Response:**
```json
{
  "status": "success",
  "data": true
}
```

#### Delete Category (Admin Only)

```
DELETE /categories/{id}
```

**Headers:**
- `Authorization: Bearer your_jwt_token`
- `x-api-key: your_api_key`

**Response:**
```json
{
  "status": "success",
  "data": true
}
```

### Terms (Glossary)

#### List Terms

```
GET /terms
```

**Headers:**
- `x-api-key: your_api_key`

**Response:**
```json
{
  "status": "success",
  "data": [
    {
      "id": 1,
      "title": "Термин",
      "description": "Описание термина",
      "first_letter": "Т",
      "created": "2025-01-01T12:00:00"
    },
    ...
  ]
}
```

#### Get Term

```
GET /terms/{id}
```

**Headers:**
- `x-api-key: your_api_key`

**Response:**
```json
{
  "status": "success",
  "data": {
    "id": 1,
    "title": "Термин",
    "description": "Описание термина",
    "first_letter": "Т",
    "created": "2025-01-01T12:00:00"
  }
}
```

#### Get Terms by Letter

```
GET /terms/letter/{letter}
```

**Headers:**
- `x-api-key: your_api_key`

**Response:**
```json
{
  "status": "success",
  "data": [
    {
      "id": 1,
      "title": "Термин",
      "description": "Описание термина",
      "first_letter": "Т",
      "created": "2025-01-01T12:00:00"
    },
    ...
  ]
}
```

#### Get Available Letters

```
GET /terms/letters
```

**Headers:**
- `x-api-key: your_api_key`

**Response:**
```json
{
  "status": "success",
  "data": ["А", "Б", "В", "Г", ...]
}
```

#### Search Terms

```
GET /terms/search?q={query}
```

**Headers:**
- `x-api-key: your_api_key`

**Response:**
```json
{
  "status": "success",
  "data": [
    {
      "id": 1,
      "title": "Термин",
      "description": "Описание термина",
      "first_letter": "Т",
      "created": "2025-01-01T12:00:00"
    },
    ...
  ]
}
```

#### Create Term (Admin Only)

```
POST /terms
```

**Headers:**
- `Authorization: Bearer your_jwt_token`
- `x-api-key: your_api_key`
- `Content-Type: application/json`

**Request Body:**
```json
{
  "title": "Новый термин",
  "description": "Описание нового термина",
  "first_letter": "Н"
}
```

**Response:**
```json
{
  "status": "success",
  "data": 42  // ID созданного термина
}
```

#### Update Term (Admin Only)

```
PUT /terms/{id}
```

**Headers:**
- `Authorization: Bearer your_jwt_token`
- `x-api-key: your_api_key`
- `Content-Type: application/json`

**Request Body:**
```json
{
  "title": "Обновленный термин",
  "description": "Обновленное описание термина"
}
```

**Response:**
```json
{
  "status": "success",
  "data": true
}
```

#### Delete Term (Admin Only)

```
DELETE /terms/{id}
```

**Headers:**
- `Authorization: Bearer your_jwt_token`
- `x-api-key: your_api_key`

**Response:**
```json
{
  "status": "success",
  "data": true
}
```

## Error Codes

Common error responses include:

| Status Code | Description |
|-------------|-------------|
| 400 | Bad Request - Invalid parameters |
| 401 | Unauthorized - Authentication required |
| 403 | Forbidden - Insufficient permissions |
| 404 | Not Found - Resource does not exist |
| 422 | Unprocessable Entity - Request validation failed |
| 500 | Internal Server Error - Something went wrong on the server |

## Rate Limiting

API requests are limited to 100 requests per minute per API key. Exceeding this limit will result in a 429 Too Many Requests response.

## API Key Management

API keys can be created and managed through the admin interface at `/admin/apikeys`.

## Versioning

The current API version is v1. All endpoints are accessible without a version prefix. Future versions will use the prefix `/api/v2/`, etc.

## Support

For API support and queries, please contact:
- Email: api@universalpath.org
- Documentation updates: https://universalpath.org/api/docs