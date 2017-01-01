# API
## Students
A student record MAY consist of the following fields:
```javascript
{
    "id": Number,
    "name": String,
    "class_letter": String,
    "graduation_year": Number,
    "lent_books": Array,
    "base_sets": Array
}
```
which will always be included in a response. It MUST consist of the
following fields:
```javascript
{
    "name": String,
    "class_letter": String,
    "graduation_year": Number
}
```
`lent_books` and `base_sets` will only be non-null in a response, if they are
specifically asked for (`include=…`). Both fields' items are arrays whose first
item is the date when the book was lent and whose second item is the respective
[Book](#books) record.
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
        "base_sets":[[
            "2017-01-01T09:55:37.123791+00:00", {
                "id":1,
                "isbn":"3728374839234",
                "title":"isufghihdmstgkufh",
                "form":"10"
            }
        ]]
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
        "lent_books":[[
            "2017-01-01T09:56:09.479132+00:00",
            {
                "id":1,
                "isbn":"3728374839234",
                "title":"isufghihdmstgkufh",
                "form":"10"
            }
        ]],
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
    "lent_books":[[
        "2017-01-01T09:56:09.479132+00:00",
        {
            "id":1,
            "isbn":"3728374839234",
            "title":"isufghihdmstgkufh",
            "form":"10"
            }
        ]],
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
A book record consists of the following entries:
```javascript
{
    isbn: String,
    title: String,
    form: String
}
```
where `form` is a comma-separated list of forms (Jahrgangsstufen) to which the
book in question is usually distributed.

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
//TODO

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
//TODO

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
