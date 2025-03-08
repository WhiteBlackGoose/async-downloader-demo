open FSharpx.Control
open System.Net.Http
open System.Threading.Tasks
open System.Threading

let my_webpage = "http://127.0.0.1:8000/index.html"

let req = new HttpClient()
let count = 200000
let slim = new SemaphoreSlim 300
let res =
    { 0 .. count }
    |> Seq.map (fun _ -> Task.Run<HttpResponseMessage> (fun () -> task {
        do! slim.WaitAsync()
        let! resp =  req.GetAsync(my_webpage)
        assert (resp.StatusCode = System.Net.HttpStatusCode.OK)
        let! body = resp.Content.ReadAsStringAsync()
        let _ = slim.Release ()
        assert (body.Length = 6113 || body.Length = 25)
        return resp
    }))
    |> Task.WhenAll
    |> Async.AwaitTask
    |> Async.RunSynchronously
printfn "done"
