let read_whole_file filename =
  let ch = open_in_bin filename in
  let s = really_input_string ch (in_channel_length ch) in
  close_in ch;
  s

let str_to_i32 (data: string): int * int =
  match String.split_on_char '-' data with
  | [a; b] -> (int_of_string a, int_of_string b)
  | _ -> failwith "Could not convert to integer left"

let part2 (left: string) (right: string): int =
  let in_range k x y = k >= x && k < y in
  let first = str_to_i32 left in
  let second = str_to_i32 right in
  let f1, f2 = fst first, snd first in
  let s1, s2 = fst second, snd second in
  if (in_range f1 s1 (s2 + 1) || in_range f2 s1 (s2 + 1))
     || (in_range s1 f1 (f2 + 1) || in_range s2 f1 (f2 + 1)) then 1
  else 0

let part1 (left: string) (right: string): int =
  let first = str_to_i32 left in
  let second = str_to_i32 right in
  let f1, f2 = fst first, snd first in
  let s1, s2 = fst second, snd second in
  if (f1 <= s1 && f2 >= s2) || (s1 <= f1 && s2 >= f2) then 1
  else 0

let rec find_overlapping (lines: string list) (sum: int): int =
  match lines with
  | [] -> sum
  | hd :: tl ->
     if List.for_all (fun x -> x = "") lines then sum
     else
       let left, right =
         match String.split_on_char ',' hd with
         | hd' :: tl' :: [] -> hd', tl'
         | _ -> failwith "Could not parse line."
       in
       find_overlapping tl (sum + (part2 left right))

let filepath = "./input.txt"

let () =
  let data = read_whole_file filepath in
  let lines = String.split_on_char '\n' data in
  find_overlapping lines 0 |> Printf.printf "%d\n"

