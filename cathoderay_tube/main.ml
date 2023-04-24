let read_whole_file filename =
  let ch = open_in_bin filename in
  let s = really_input_string ch (in_channel_length ch) in
  close_in ch;
  s

let print_list lst =
  List.iter (fun x -> Printf.printf "d: %s\n" x) lst

let try_string_of_int (s: string): int option =
  try Some (int_of_string s)
  with Failure _ -> None

let draw (c: int) (x: int) =
  if (c mod 40) < x || (c mod 40) > x + 2 then
    Printf.printf "."
  else
    Printf.printf "#";
  if (c mod 40) = 0 then
    print_newline();;

let check (c: int) (x: int): int =
  match (c = 20, (c > 20 && (c - 20) mod 40 = 0)) with
  | (false, false) -> 0
  | _ -> x * c

let rec solve (data: string list) (c: int) (x: int) (sum: int): int =
  match data with
  | [] -> sum
  | "noop" :: tl ->
     draw (c + 1) x;
     solve tl (c + 1) x (sum + (check (c + 1) x))
  | hd :: tl ->
     (match try_string_of_int hd with
      | Some k ->
         draw (c + 1) x;
         draw (c + 2) x;
         solve tl (c + 2) (x + k) (sum + (check (c + 1) x) + (check (c + 2) x))
      | None -> sum)

let filepath = "input.txt"
let () =
  let data_vec = read_whole_file filepath
                 |> String.split_on_char '\n'
                 |> List.map (String.split_on_char ' ')
                 |> List.concat
                 |> List.filter (fun s -> s <> "addx") in
  let (c, x, sum) = (0, 1, 0) in
  let res = solve data_vec c x sum in
  Printf.printf "%d\n" res
