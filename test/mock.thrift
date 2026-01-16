namespace go test

service Service {
    Response MockTask(1: Request req) (api.post = "/test/mock")
}

struct Request {
    1: i64    ID (go.tag = "json:\"id\"")
    2: string Status (go.tag = "json:\"status\"")
    3: string Task (go.tag = "json:\"task\"")
    4: string Text (go.tag = "json:\"text\"")
}

struct Response {
    1: i64    Code (go.tag = "json:\"code\"")
    2: string Data (go.tag = "json:\"data\"")
    3: string Message (go.tag = "json:\"message\"")
}