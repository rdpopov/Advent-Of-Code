let input = "./input"
let test = "./test"
let emptyHead (lst: string list) =
    match lst with
    | [] -> ""
    | x -> List.hd lst
let emptyTail (lst: string list) =
    match lst with
    | [] -> []
    | x::[] -> []
    | x::xs -> xs

let parseToList fname =
    let ic = open_in fname in
    let try_read () =
        try Some(input_line ic) with End_of_file -> None in
    let rec loop lst =
        match try_read() with
        | None -> lst
        | Some("") -> loop (""::lst)
        | Some(x) -> loop ((emptyHead lst ^ " " ^ x)::(emptyTail lst))
    in loop [];;

let print lst =
    let rec loop l =
        match l with
        | [] -> Printf.printf ")\n"
        | x::xs -> Printf.printf "%s\n" x; loop xs;
    in loop lst;;

parseToList test |> print 
