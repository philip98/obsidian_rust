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
```json
HTTP/1.1 200 OK
Content-Type: application/json

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
```json
HTTP/1.1 200 OK
Content-Type: application/json

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
```json
HTTP/1.1 200 OK
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
```json
POST /students HTTP/1.1
Content-Type: application/json

{
    "name":"Luz Karkoschka",
    "class_letter":"",
    "graduation_year":2015
}
```
(An `id` field may be specified, but will be ignored; the same goes for
`lent_books` and `base_sets`. Order matters!)

Response:
```json
HTTP/1.1 201 Created
Content-Type: application/json

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
```json
POST /students HTTP/1.1
Content-Type: application/json

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
```json
HTTP/1.1 201 Created
Content-Type: application/json

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
```json
PUT /students/6 HTTP/1.1
Content-Type: application/json

{
    "name":"Luz Karkoschka",
    "class_letter":"b",
    "graduation_year":2015
}
```

Response:
```json
HTTP/1.1 200 OK
Content-Type: application/json

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
