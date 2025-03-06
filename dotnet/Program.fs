open FSharpx.Control
open System.Net.Http
open System.Threading.Tasks

let my_webpage = "http://127.0.0.1:8000/index.html"

let req = new HttpClient()
let count = 2000
let res =
    { 0 .. count }
    |> Seq.map (fun _ -> task {
        let! resp =  req.GetAsync(my_webpage)
        let! body = resp.Content.ReadAsStringAsync()
        assert (body.Length = 25)
        return resp
    })
    |> Task.WhenAll
    |> Async.AwaitTask
    |> Async.RunSynchronously
printfn "done"
