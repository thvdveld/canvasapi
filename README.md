# canvasapi

This is the (unofficial) Rust version of the library for accessing the Canvas LMS API.
The official documentation of the API can be found
[here](https://canvas.instructure.com/doc/api/).

## Quickstart

First, the information about the Canvas LMS server needs to be stored in a `CanvasInformation`
struct.
Using this information, the correct request URL's are created and, with a valid API token,
executed.

```rust
let base_url = std::env::var("CANVAS_BASE_URL").unwrap();
let canvas_token = std::env::var("CANVAS_ACCESS_TOKEN").unwrap();

let canvas = CanvasInformation::new(&base_url, &canvas_token);

let course = Canvas::get_course(13369).fetch(&canvas).await.unwrap().inner();
```

## Notes

This crate only supports GET requests, and not all of them are implemented.
Only the ones I use are implemented.
Feel free to add more.

To add support for new request, follow these steps:
1. Canvas will return data in the form of JSON. This data needs to be deserialized into
   a struct. Define this struct in this library, under `models`, when it is not yet defined.
   This information can be retrieved using the official Canvas API.
2. API request functions can are added in the implementation of the structs. These requests can
   be methods or functions. To define a new request, use the `api_get!` macro. The following
   example shows the usage of the macro.

```rust
impl Course {
    api_get! {
        /// Get all courses from the current user.
        courses():
            "courses" =>
            () -> () -> [Course]
    }

    api_get! {
        /// Return the assignment with the given id.
        get_assignment(self):
            "courses/{id}/assignments/{assignment_id}" =>
                (id: self.id) -> (assignment_id: usize) -> Assignment
    }

    api_get! {
        /// Get only the students from the course.
        get_students(self):
            "courses/{id}/search_users" =>
                (id: self.id) -> () -> [User]
                [EnrollmentType::Student]
    }
}
```

First, you write the (optional) documentation string, for use in this generated
documentation, followed by the function/method name.
If it is a method, the `self` keyword is used between the parentheses.
Next, you write the request url, without the base url and without `/api/v1/`.
The full request is generated using the `CanvasInformation` struct.
The request url can contain named parameters.
The `self` arguments are defined in the first group and the function arguments are
passed in the second group.
Finaly, the return type of the request is defined by passing the returned struct.
Square brackets are used when a `Vec` is returned by the API.
Optionaly, requests parameters can be added.

License: MIT OR Apache-2.0