# API
## Students
A student record MAY consist of the following fields:
```javascript
{
    id: Number,
    name: String,
    class_letter: String,
    graduation_year: Number,
    lent_books: Array,
    base_sets: Array
}
```
which will always be included in a response. It MUST consist of the
following fields:
```javascript
{
    name: String,
    class_letter: String,
    graduation_year: Number
}
```
`lent_books` and `base_sets` will only be non-null in a response, if they are
specifically asked for (`include=…`). Both fields are arrays consisting of entries of
the following format:
```javascript
{
    id: Number,
    created_at: String,
    book: Object
}
```
where `created_at` is the RFC3339 representation of the date the book was lent on,
and `book` is a [Book](#books) record.

Note that a client-supplied `id` will always be ignored (aside from the route)
### Index
#### Without `include`
Request:
```
GET /students HTTP/1.1
Accept: application/json
```

Response:
```
HTTP/1.1 200 OK
Content-Type: application/json
```
```json
[
    {
        "id":2,
        "name":"P. S.",
        "class_letter":"a",
        "graduation_year":2016,
        "lent_books":null,
        "base_sets":null
    },
    {
        "id":3,
        "name":"PhiSchl",
        "class_letter":"c",
        "graduation_year":2016,
        "lent_books":null,
        "base_sets":null
    },
    {
        "id":5,
        "name":"Hannah Lange",
        "class_letter":"",
        "graduation_year":2015,
        "lent_books":null,
        "base_sets":null
    }
]
```

#### With `include`
Request:
```
GET /students?include=lendings.book,baseSets.book HTTP/1.1
Accept: application/json
```
(`lendings.book` is case-insensitive and may also be called `lendings`;
`baseSets.book` may alternatively be called `baseSets`).

Response:
```
HTTP/1.1 200 OK
Content-Type: application/json
```
```json
[
    {
        "id":2,
        "name":"P. S.",
        "class_letter":"a",
        "graduation_year":2016,
        "lent_books":[],
        "base_sets":[{
            "id":1,
            "created_at":"2017-01-01T09:55:37.123791+00:00",
            "book":{
                "id":1,
                "isbn":"3728374839234",
                "title":"isufghihdmstgkufh",
                "form":"10"
            }
        }]
    },
    {
        "id":3,
        "name":"PhiSchl",
        "class_letter":"c",
        "graduation_year":2016,
        "lent_books":[],
        "base_sets":[]
    },
    {
        "id":5,
        "name":"Hannah Lange",
        "class_letter":"",
        "graduation_year":2015,
        "lent_books":[{
            "id":1,
            "created_at":"2017-01-01T09:56:09.479132+00:00",
            "book":{
                "id":1,
                "isbn":"3728374839234",
                "title":"isufghihdmstgkufh",
                "form":"10"
            }
        }],
        "base_sets":[]
    }
]
```

### Show
#### Without `include`
Request:
```
GET /students/5 HTTP/1.1
Accept: application/json
```

Response:
```
HTTP/1.1 200 OK
```
```json
{
    "id":5,
    "name":"Hannah Lange",
    "class_letter":"",
    "graduation_year":2015,
    "lent_books":null,
    "base_sets":null
}
```

#### With `include`
Request:
```
GET /students/5?include=lendings.book,baseSets.book HTTP/1.1
Accept: application/json
```

Response:
```
HTTP/1.1 200 OK
Content-Type: application/json
```
```json
{
    "id":5,
    "name":"Hannah Lange",
    "class_letter":"",
    "graduation_year":2015,
    "lent_books":[{
        "id":1,
        "created_at":"2017-01-01T09:56:09.479132+00:00",
        "book":{
            "id":1,
            "isbn":"3728374839234",
            "title":"isufghihdmstgkufh",
            "form":"10"
            }
        }],
    "base_sets":[]
}
```

### Create
#### Single student
Request:
```
POST /students HTTP/1.1
Content-Type: application/json
```
```json
{
    "name":"Luz Karkoschka",
    "class_letter":"",
    "graduation_year":2015
}
```
(An `id` field may be specified, but will be ignored; the same goes for
`lent_books` and `base_sets`. Order matters!)

Response:
```
HTTP/1.1 201 Created
Content-Type: application/json
```
```json
{
    "id":6,
    "name":"Luz Karkoschka",
    "class_letter":"",
    "graduation_year":2015,
    "lent_books":null,
    "base_sets":null
}
```

#### Multiple students
Request:
```
POST /students HTTP/1.1
Content-Type: application/json
```
```json
[
    {
        "name":"Jael Veen",
        "class_letter":"",
        "graduation_year":2017
    },
    {
        "name":"Katharina Maier",
        "class_letter":"",
        "graduation_year":2017
    }
]
```

Response:
```
HTTP/1.1 201 Created
Content-Type: application/json
```
```json
[
    {
        "id":7,
        "name":"Jael Veen",
        "class_letter":"",
        "graduation_year":2017,
        "lent_books":null,
        "base_sets":null
    },
    {
        "id":8,
        "name":"Katharina Maier",
        "class_letter":"",
        "graduation_year":2017,
        "lent_books":null,
        "base_sets":null
    }
]
```

### Edit
Request:
```
PUT /students/6 HTTP/1.1
Content-Type: application/json
```
```json
{
    "name":"Luz Karkoschka",
    "class_letter":"b",
    "graduation_year":2015
}
```

Response:
```
HTTP/1.1 200 OK
Content-Type: application/json
```
```json
{
    "id":6,
    "name":"Luz Karkoschka",
    "class_letter":"b",
    "graduation_year":2015,
    "lent_books":null,
    "base_sets":null
}
```

### Delete
Request:
```
DELETE /students/8 HTTP/1.1
```

Response:
```
HTTP/1.1 204 No Content
```

## Books
A book record MAY consist of the following entries:
```javascript
{
    id: Number,
    isbn: String,
    title: String,
    form: String,
    aliases: Array
}
```
where `form` is a comma-separated list of forms (Jahrgangsstufen) to which the
book in question is usually distributed, and `aliases` is an array of alias records.
A server response will always look this way. The user usually only has to specify
the following fields:
```javascript
{
    isbn: String,
    title: String,
    form: String
}
```
the rest will be `None`'d automatically.

Note that a client-supplied `id` will always be ignored (aside from the route).

### Index
#### Without `include`
Request:
```
GET /books HTTP/1.1
Accept: application/json
```

Response:
```
HTTP/1.1 200 OK
Content-Type: application/json
```
```json
[
    {
        "id":1,
        "isbn":"3728374839234",
        "title":"isufghihdmstgkufh",
        "form":"10"
    },
    {
        "id":2,
        "isbn":"9781234567894",
        "title":"On The Origin Of Species",
        "form":"12"
    }
]
```

#### With `include`
Request:
```
GET /books?include=aliases HTTP/1.1
Accept: application/json
```

Response:
```
HTTP/1.1 200 OK
Content-Type: application/json
```
```json
[
    {
        "id":4,
        "isbn":"9781278945432",
        "title":"Quantisierung als Eigenwertproblem",
        "form":"14",
        "aliases":[{
            "id":1,
            "book_id":4,
            "name":"quant"
        }]
    },
    {
        "id":2,
        "isbn":"9781234567894",
        "title":"On The Origin Of Species",
        "form":"13",
        "aliases":[]
    }
]
```

### Show
#### Without `include`
Request:
```
GET /books/1 HTTP/1.1
Accept: application/json
```

Response:
```
HTTP/1.1 200 OK
Content-Type: application/json
```
```json
{
    "id":1,
    "isbn":"3728374839234",
    "title":"isufghihdmstgkufh",
    "form":"10"
}
```

#### With `include`
Request:
```
GET /books/4?include=aliases HTTP/1.1
Accept: application/json
```

Response:
```
HTTP/1.1 200 OK
Content-Type: application/json
```
```json
{
    "id":4,
    "isbn":"9781278945432",
    "title":"Quantisierung als Eigenwertproblem",
    "form":"14",
    "aliases":[{
        "id":1,
        "book_id":4,
        "name":"quant"
    }]
}
```

### Create
Request:
```
POST /books HTTP/1.1
Content-Type: application/json
```
```json
{
    "isbn":"9781278945432",
    "title":"Quantisierung als Eigenwertproblem",
    "form":"14"
}
```

Response:
```
HTTP/1.1 201 Created
Content-Type: application/json
```
```json
{
    "id":4,
    "isbn":"9781278945432",
    "title":"Quantisierung als Eigenwertproblem",
    "form":"14"
}
```

### Edit
Request:
```
PUT /books/2 HTTP/1.1
Content-Type: application/json
```
```json
{
    "isbn":"9781234567894",
    "title": "On The Origin Of Species",
    "form": "13"
}
```

Response:
```
HTTP/1.1 200 OK
Content-Type: application/json
```
```json
{
    "id":2,
    "isbn":"9781234567894",
    "title":"On The Origin Of Species",
    "form":"13"
}
```

### Delete
Request:
```
DELETE /books/1 HTTP/1.1
```

Response:
```
HTTP/1.1 204 No Content
```

## Aliases
An alias record consists of the following fields:
```javascript
{
    id: Number,
    book_id: Number,
    name: String
}
```

where `id` will always be ignored when specified by the client.

### Index
Request:
```
GET /aliases HTTP/1.1
Accept: application/json
```

Response:
```
HTTP/1.1 200 OK
Content-Type: application/json
```
```json
[
    {
        "id":1,
        "book_id":4,
        "name":"quant"
    },
    {
        "id":2,
        "book_id":4,
        "name":"schroed"
    }
]
```

### Create
Request:
```
POST /aliases HTTP/1.1
Content-Type: application/json
```
```json
{
    "book_id":2,
    "name":"spec"
}
```

Response:
```
HTTP/1.1 201 Created
Content-Type: application/json
```
```json
{
    "id":3,
    "book_id":2,
    "name":"spec"
}
```

### Edit
Request:
```
PUT /aliases/3 HTTP/1.1
Content-Type: application/json
```
```json
{
    "book_id":2,
    "name":"orig"
}
```

Response:
```
HTTP/1.1 200 OK
Content-Type: application/json
```
```json
{
    "id":3,
    "book_id":2,
    "name":"orig"
}
```

### Delete
Request:
```
DELETE /aliases/2 HTTP/1.1
```

Response:
```
HTTP/1.1 204 No Content
```

## Teachers
### Index
#### Without `include`
Request:
```
GET /teachers HTTP/1.1
Accept: application/json
```

Response:
```
HTTP/1.1 200 OK
Content-Type: application/json
```
```json
[
    {
        "id":1,
        "name":"Charles Darwin",
        "lent_books":null
    },
    {
        "id":2,
        "name":"Erwin Schroedinger",
        "lent_books":null
    }
]
```

#### With `include`
Request:
```
GET /teachers?include=lendings.book HTTP/1.1
Accept: application/json
```

Response:
```
HTTP/1.1 200 OK
Content-Type: application/json
```
```json
[
    {
        "id":1,
        "name":"Charles Darwin",
        "lent_books":[{
            "id":1,
            "created_at":"2017-01-02T13:04:59.241354+00:00",
            "book":{
                "id":2,
                "isbn":"9781234567894",
                "title":"On The Origin Of Species",
                "form":"13",
                "aliases":null
            }
        }]
    },
    {
        "id":2,
        "name":"Erwin Schroedinger",
        "lent_books":[]
    }
]
```

### Show
#### Without `include`
Request:
```
GET /teachers/1 HTTP/1.1
Accept: application/json
```

Response:
```
HTTP/1.1 200 OK
Content-Type: application/json
```
```json
{
    "id":1,
    "name":"Charles Darwin",
    "lent_books":null
}
```

#### With `include`
Request:
```
GET /teachers/1?include=lendings.book HTTP/1.1
Accept: application/json
```

Response:
```
HTTP/1.1 200 OK
Content-Type: application/json
```
```json
{
    "id":1,
    "name":"Charles Darwin",
    "lent_books":[
        {
            "id":1,
            "created_at":"2017-01-02T13:04:59.241354+00:00",
            "book":{
                "id":2,
                "isbn":"9781234567894",
                "title":"On The Origin Of Species",
                "form":"13",
                "aliases":null
            }
        },
        {
            "id":2,
            "created_at":"2017-01-02T13:14:23.142351+00:00",
            "book":{
                "id":4,
                "isbn":"9781278945432",
                "title":"Quantisierung als Eigenwertproblem",
                "form":"14",
                "aliases":null
            }
        }
    ]
}
```

### Create
Request:
```
POST /teachers HTTP/1.1
Content-Type: application/json
```
```json
{
    "name":"Max Planck"
}
```

Response:
```
HTTP/1.1 201 Created
Content-Type: application/json
```
```json
{
    "id":3,
    "name":"Max Planck",
    "lent_books":null
}
```

### Edit
Request:
```
PUT /teachers/3 HTTP/1.1
Content-Type: application/json
```
```json
{
    "name":"Werner Heisenberg"
}
```

Response:
```
HTTP/1.1 200 OK
Content-Type: application/json
```
```json
{
    "id":3,
    "name":"Werner Heisenberg",
    "lent_books":null
}
```

### Delete
Request:
```
DELETE /teachers/3 HTTP/1.1
```

Response:
```
HTTP/1.1 204 No Content
```

## Base Sets
### Create
#### Single Base Set
Request:
```
POST /base_sets HTTP/1.1
Content-Type: application/json`
```
```json
{
    "student_id":7,
    "book_id":4
}
```

Response:
```
HTTP/1.1 201 Created
Content-Type: application/json
```
```json
{
    "id":3,
    "student_id":7,
    "book_id":4,
    "created_at":"2017-01-03T09:45:21.661754557+00:00"
}
```
#### Multiple Base Sets
Request:
```
POST /base_sets HTTP/1.1
Content-Type: application/json
```
```json
[
    {
        "student_id":3,
        "book_id":4
    },
    {
        "student_id":3,
        "book_id":2
    }
]
```

Response:
```
HTTP/1.1 201 Created
Content-Type: application/json
```
```json
[
    {
        "id":4,
        "student_id":3,
        "book_id":4,
        "created_at":"2017-01-03T13:45:12.158573031+00:00"
    },
    {
        "id":5,
        "student_id":3,
        "book_id":2,
        "created_at":"2017-01-03T13:45:12.158595519+00:00"
    }
]
```

### Delete
Request:
```
DELETE /base_sets/3 HTTP/1.1
```

Response:
```
HTTP/1.1 204 No Content
```

## Lendings
Each lending record must contain the following entries:
```javascript
{
    person_type: String,
    person_id: Number,
    book_id: Number
}
```
where `person_type` is either `student` or `teacher`. A server response will always
look as follows:
```javascript
{
    id: Number,
    created_at: String,
    person_type: String,
    person_id: Number,
    book_id: Number
}
```
where `created_at` is the RFC3339 representation of the UTC-time, the record was
created.

### Create
#### Single Lending
Request:
```
POST /lendings HTTP/1.1
Content-Type: application/json
```
```json
{
    "person_type":"student",
    "person_id":6,
    "book_id":4
}
```

Response:
```
HTTP/1.1 201 Created
Content-Type: application/json
```
```json
{
    "id":4,
    "created_at":"2017-01-03T11:05:11.396771676+00:00",
    "person_type":"student",
    "person_id":6,
    "book_id":4
}
```

#### Multiple Lendings
Request:
```
POST /lendings HTTP/1.1
Content-Type: application/json
```
```json
[
    {
        "person_type":"student",
        "person_id":5,
        "book_id":4
    },
    {
        "person_type":"teacher",
        "person_id":2,
        "book_id":4
    }
]
```

Response:
```
HTTP/1.1 201 Created
Content-Type: application/json
```
```json
[
    {
        "id":5,
        "created_at":"2017-01-03T13:50:34.281133141+00:00",
        "person_type":"student",
        "person_id":5,
        "book_id":4
    },
    {
        "id":6,
        "created_at":"2017-01-03T13:50:34.281178885+00:00",
        "person_type":"teacher",
        "person_id":2,
        "book_id":4
    }
]
```

### Delete
Request:
```
DELETE /lendings/4 HTTP/1.1
```

Response:
```
HTTP/1.1 204 No Content
```

## Schools
### Create
Request:
```
POST /schools HTTP/1.1
Content-Type: application/json
```
```json
{
    "name":"Michaeli-Gymnasium",
    "password":"test1234"
}
```

Response:
```
HTTP/1.1 201 Created
Content-Type: application/json
```
```json
{
    "token_id":893958374535,
    "secret":"4yvOsSrYk6Bps1RBzIEAt4CE"
}
```

### Edit
#### Changing password
Request:
```
PUT /schools HTTP/1.1
Content-Type: application/json
```
```json
{
    "old_password":"test1234",
    "new_password":"password123"
}
```

Response:
```
HTTP/1.1 204 No Content
```

#### Changing the school's name
Request:
```
PUT /schools HTTP/1.1
Content-Type: application/json
```
```json
{
    "name":"MGM"
}
```

Response:
```
HTTP/1.1 204 No Content
```

### Delete
Request:
```
DELETE /schools HTTP/1.1
Content-Type: application/json
```
```json
{
    "password":"password123"
}
```

Response:
```
HTTP/1.1 204 No Content
```

## Sessions
### Create
Request:
```
POST /sessions HTTP/1.1
Content-Type: application/json
```
```json
{
    "name":"MGM",
    "password":"password123"
}
```

Response:
```
HTTP/1.1 201 Created
Content-Type: application
```
```json
{
    "token_id":2991965075,
    "secret":"4yvOsSrYk6Bps1RBzIEAt4CE"
}
```

### Delete
Request:
```
DELETE /sessions HTTP/1.1
```

Response:
```
HTTP/1.1 204 No Content
```

# Authentication
All routes except the following expect [Basic authentication](https://en.wikipedia.org/wiki/Basic_access_authentication):
- `/schools/new`
- `/sessions/new`

All other routes need to provide a name/password pair (or, as it should be called, a token/secret pair) that
is returned by `/schools/new` and `/sessions/new`. A correct request to one of those
routes yields a response that contains a `token_id` and a `secret`. In order to use
the other routes, please specify the `token_id` as username and the `secret` as password.
As is customary for Basic authentication, the username/password block is then base64-encoded
and appended to the string "`Basic `" and used as `Authorization` headers.

If no `Authorization` header is specified (but one is required), or the token is invalid or
malformed, the server will respond with

```
HTTP/1.1 403 Unauthorized
WWW-Authenticate: Basic: realm="Token and secret"
```

Once authorized, the user is perfectly ignorant of other users that might exist. They
have absolutely no access to any records belonging to other schools. The user is
also ignorant of their own school id because such knowledge is simply not necessary:
when logged in, there is only one school whose name or password one could possibly
change, and logging in happens via school name and password, so no school id required, either.
